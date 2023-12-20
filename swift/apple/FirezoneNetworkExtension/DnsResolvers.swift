//
//  DnsResolvers.swift
//  Firezone
//
//  Created by Jamil Bou Kheir on 12/12/23.
//
//  Wraps libresolv to return the system's default resolvers

struct DNSResolvers {
  public static func getDNSResolverAddresses() -> [String] {
    return Resolv().getservers().map(Resolv.getnameinfo)
  }
}

public class Resolv {
  var state = __res_9_state()

  public init() {
    res_9_ninit(&state)
  }

  deinit {
    res_9_ndestroy(&state)
  }

  public final func getservers() -> [res_9_sockaddr_union] {
    let maxServers = 10
    var servers = [res_9_sockaddr_union](repeating: res_9_sockaddr_union(), count: maxServers)
    let found = Int(res_9_getservers(&state, &servers, Int32(maxServers)))

    // filter is to remove the erroneous empty entry when there's no real servers
    return Array(servers[0..<found]).filter { $0.sin.sin_len > 0 }
  }
}

extension Resolv {
  public static func getnameinfo(_ s: res_9_sockaddr_union) -> String {
    var s = s
    var hostBuffer = [CChar](repeating: 0, count: Int(NI_MAXHOST))

    let sinlen = socklen_t(s.sin.sin_len)
    let _ = withUnsafePointer(to: &s) {
      $0.withMemoryRebound(to: sockaddr.self, capacity: 1) {
        Darwin.getnameinfo(
          $0, sinlen,
          &hostBuffer, socklen_t(hostBuffer.count),
          nil, 0,
          NI_NUMERICHOST)
      }
    }

    return String(cString: hostBuffer)
  }
}
