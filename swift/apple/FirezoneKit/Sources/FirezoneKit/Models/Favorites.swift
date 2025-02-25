import Foundation

public class Favorites: ObservableObject {
  private let key = "favoriteResourceIDs"
  @Published private(set) var ids: Set<String>

  public init () {
    self.ids = Set()
    Task { await self.ids = load() }
  }

  func contains(_ id: String) -> Bool {
    return ids.contains(id)
  }

  func load() async -> Set<String> {
    let ids = await withCheckedContinuation { continuation in
      continuation.resume(returning: UserDefaults.standard.stringArray(forKey: key))
    }

    if let ids {
      return Set(ids)
    }

    return Set()
  }

  func reset() {
    self.ids = Set()
    Task { await save() }
  }

  func add(_ id: String) {
    self.ids.insert(id)
    Task { await save() }
  }

  func remove(_ id: String) {
    self.ids.remove(id)
    Task { await save() }
  }

  private func save() async {
    // It's a run-time exception if we pass the `Set` directly here
    let ids = Array(ids)
    await withCheckedContinuation { continuation in
      UserDefaults.standard.set(ids, forKey: key)
      continuation.resume()
    }
  }
}
