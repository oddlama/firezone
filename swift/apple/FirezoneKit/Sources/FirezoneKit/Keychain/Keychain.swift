//
//  Keychain.swift
//  (c) 2024 Firezone, Inc.
//  LICENSE: Apache-2.0
//

import Foundation

public enum KeychainError: Error {
  case securityError(KeychainStatus)
  case appleSecError(call: String, status: Keychain.SecStatus)
  case nilResultFromAppleSecCall(call: String)
  case resultFromAppleSecCallIsInvalid(call: String)
  case unableToFindSavedItem
  case unableToGetAppGroupIdFromInfoPlist
  case unableToFormExtensionPath
  case unableToGetPluginsPath
}

public actor Keychain {
  private let label = "Firezone token"
  private let description = "Firezone access token used to authenticate the client."
  private let account = "Firezone"
  private let service = Bundle.main.bundleIdentifier!

  private let workQueue = DispatchQueue(label: "FirezoneKeychainWorkQueue")

  public typealias Token = String
  public typealias PersistentRef = Data

  public enum SecStatus: Equatable {
    case status(KeychainStatus)
    case unknownStatus(OSStatus)

    init(_ osStatus: OSStatus) {
      if let status = KeychainStatus(rawValue: osStatus) {
        self = .status(status)
      } else {
        self = .unknownStatus(osStatus)
      }
    }

    var isSuccess: Bool {
      return self == .status(.success)
    }
  }

  public init() {}

  func add(token: Token) async throws -> PersistentRef {
    var query: [CFString: Any] = [
      // Common for both iOS and macOS:
      kSecClass: kSecClassGenericPassword,
      kSecAttrLabel: label,
      kSecAttrDescription: description,
      kSecAttrAccount: account,
      kSecAttrService: service,
      kSecValueData: token.data(using: .utf8) as Any,
      kSecReturnPersistentRef: true,
      kSecAttrAccessible: kSecAttrAccessibleAfterFirstUnlock,
    ]

    #if os(iOS)
    query.merge([
      kSecAttrAccessGroup: AppInfoPlistConstants.appGroupId as CFString as Any
    ], uniquingKeysWith: {_, _ in })
    #elseif os(macOS)
    query.merge([
      kSecAttrAccess: try secAccessForAppAndNetworkExtension()
    ], uniquingKeysWith: {_, _ in })
    #endif

    return try await withCheckedThrowingContinuation { [weak self] continuation in
      self?.workQueue.async {
        var ref: CFTypeRef?
        let ret = SecStatus(SecItemAdd(query as CFDictionary, &ref))
        guard ret.isSuccess else {
          continuation.resume(
            throwing: KeychainError.appleSecError(call: "SecItemAdd", status: ret))
          return
        }
        guard let savedPersistentRef = ref as? Data else {
          continuation.resume(throwing: KeychainError.nilResultFromAppleSecCall(call: "SecItemAdd"))
          return
        }
        continuation.resume(returning: savedPersistentRef)
        return
      }
    }
  }

  func update(token: Token) async throws {
    var query: [CFString: Any] = [
      kSecClass: kSecClassGenericPassword,
      kSecAttrAccount: account,
      kSecAttrService: service,
    ]

    let attributesToUpdate = [
      kSecValueData: token.data(using: .utf8) as Any,
    ]

    #if os(iOS)
    query.merge([
      kSecAttrAccessGroup: AppInfoPlistConstants.appGroupId as CFString as Any
    ], uniquingKeysWith: {_, _ in })
    #elseif os(macOS)
    query.merge([
      kSecAttrAccess: try secAccessForAppAndNetworkExtension()
    ], uniquingKeysWith: {_, _ in })
    #endif

    return try await withCheckedThrowingContinuation { [weak self] continuation in
      self?.workQueue.async {
        let ret = SecStatus(SecItemUpdate(query as CFDictionary, attributesToUpdate as CFDictionary))
        guard ret.isSuccess else {
          continuation.resume(
            throwing: KeychainError.appleSecError(call: "SecItemUpdate", status: ret))
          return
        }
        continuation.resume()
      }
    }
  }

  #if os(macOS)
    private func secAccessForAppAndNetworkExtension() throws -> SecAccess {
      // Creating a trusted-application-based SecAccess APIs are deprecated in favour of
      // data-protection keychain APIs. However, data-protection keychain doesn't support
      // accessing from non-userspace processes, like the tunnel process, so we can only
      // use the deprecated APIs for now.
      func secTrustedApplicationForPath(_ path: String?) throws -> SecTrustedApplication? {
        var trustedApp: SecTrustedApplication?
        let ret = SecStatus(SecTrustedApplicationCreateFromPath(path, &trustedApp))
        guard ret.isSuccess else {
          throw KeychainError.appleSecError(
            call: "SecTrustedApplicationCreateFromPath", status: ret)
        }
        if let trustedApp = trustedApp {
          return trustedApp
        } else {
          throw KeychainError.nilResultFromAppleSecCall(
            call: "SecTrustedApplicationCreateFromPath(\(path ?? "nil"))")
        }
      }
      guard let pluginsURL = Bundle.main.builtInPlugInsURL else {
        throw KeychainError.unableToGetPluginsPath
      }
      let extensionPath =
        pluginsURL
        .appendingPathComponent("FirezoneNetworkExtensionmacOS.appex", isDirectory: true)
        .path
      let trustedApps = [
        try secTrustedApplicationForPath(nil),
        try secTrustedApplicationForPath(extensionPath),
      ]
      var access: SecAccess?
      let ret = SecStatus(
        SecAccessCreate("Firezone Token" as CFString, trustedApps as CFArray, &access))
      guard ret.isSuccess else {
        throw KeychainError.appleSecError(call: "SecAccessCreate", status: ret)
      }
      if let access = access {
        return access
      } else {
        throw KeychainError.nilResultFromAppleSecCall(call: "SecAccessCreate")
      }
    }
  #endif

  // This function is public because the tunnel needs to call it to get the token
  public func load(persistentRef: PersistentRef) async -> Token? {
    return await withCheckedContinuation { [weak self] continuation in
      self?.workQueue.async {
        let query =
          [
            kSecValuePersistentRef: persistentRef,
            kSecReturnData: true,
          ] as [CFString: Any]
        var result: CFTypeRef?
        let ret = SecStatus(SecItemCopyMatching(query as CFDictionary, &result))
        if ret.isSuccess,
          let resultData = result as? Data,
          let resultString = String(data: resultData, encoding: .utf8)
        {
          continuation.resume(returning: resultString)
        } else {
          continuation.resume(returning: nil)
        }
      }
    }
  }

  func search() async -> PersistentRef? {
    return await withCheckedContinuation { [weak self] continuation in
      guard let self = self else { return }
      self.workQueue.async {
        let query =
          [
            kSecClass: kSecClassGenericPassword,
            kSecAttrAccount: self.account,
            kSecAttrDescription: self.description,
            kSecAttrService: self.service,
            kSecReturnPersistentRef: true,
          ] as [CFString: Any]
        var result: CFTypeRef?
        let ret = SecStatus(SecItemCopyMatching(query as CFDictionary, &result))
        if ret.isSuccess, let tokenRef = result as? Data {
          continuation.resume(returning: tokenRef)
        } else {
          continuation.resume(returning: nil)
        }
      }
    }
  }

  private func securityError(_ status: OSStatus) -> Error {
    KeychainError.securityError(KeychainStatus(rawValue: status)!)
  }
}
