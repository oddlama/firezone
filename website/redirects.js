// Add all server-side redirects here. Will be loaded by next.config.mjs.

module.exports = [
  /*
   *
   * macOS Client
   *
   */
  {
    source: "/dl/firezone-client-macos/latest",
    destination:
      // mark:current-apple-version
      "https://www.github.com/firezone/firezone/releases/download/macos-client-1.4.1/firezone-macos-client-1.4.1.dmg",
    permanent: false,
  },
  /*
   *
   * Android Client
   *
   */
  {
    source: "/dl/firezone-client-android/latest",
    destination:
      // mark:current-android-version
      "https://www.github.com/firezone/firezone/releases/download/android-client-1.4.1/firezone-android-client-1.4.1.apk",
    permanent: false,
  },
  /*
   *
   * Windows GUI Client
   *
   */
  {
    source: "/dl/firezone-client-gui-windows/latest/x86_64",
    destination:
      // mark:current-gui-version
      "https://www.github.com/firezone/firezone/releases/download/gui-client-1.4.3/firezone-client-gui-windows_1.4.3_x86_64.msi",
    permanent: false,
  },
  /*
   *
   * Windows Headless Client
   *
   */
  {
    source: "/dl/firezone-client-headless-windows/latest/x86_64",
    destination:
      // mark:current-headless-version
      "https://www.github.com/firezone/firezone/releases/download/headless-client-1.4.1/firezone-client-headless-windows_1.4.1_x86_64.exe",
    permanent: false,
  },
  /*
   *
   * Linux Clients
   *
   */
  {
    source: "/dl/firezone-client-gui-linux/latest/x86_64",
    destination:
      // mark:current-gui-version
      "https://www.github.com/firezone/firezone/releases/download/gui-client-1.4.3/firezone-client-gui-linux_1.4.3_x86_64.deb",
    permanent: false,
  },
  {
    source: "/dl/firezone-client-gui-linux/latest/aarch64",
    destination:
      // mark:current-gui-version
      "https://www.github.com/firezone/firezone/releases/download/gui-client-1.4.3/firezone-client-gui-linux_1.4.3_aarch64.deb",
    permanent: false,
  },
  {
    source: "/dl/firezone-client-headless-linux/latest/x86_64",
    destination:
      // mark:current-headless-version
      "https://www.github.com/firezone/firezone/releases/download/headless-client-1.4.1/firezone-client-headless-linux_1.4.1_x86_64",
    permanent: false,
  },
  {
    source: "/dl/firezone-client-headless-linux/latest/aarch64",
    destination:
      // mark:current-headless-version
      "https://www.github.com/firezone/firezone/releases/download/headless-client-1.4.1/firezone-client-headless-linux_1.4.1_aarch64",
    permanent: false,
  },
  {
    source: "/dl/firezone-client-headless-linux/latest/armv7",
    destination:
      // mark:current-headless-version
      "https://www.github.com/firezone/firezone/releases/download/headless-client-1.4.1/firezone-client-headless-linux_1.4.1_armv7",
    permanent: false,
  },
  /*
   *
   * Gateway
   *
   */
  {
    source: "/dl/firezone-gateway/latest/x86_64",
    destination:
      // mark:current-gateway-version
      "https://www.github.com/firezone/firezone/releases/download/gateway-1.4.3/firezone-gateway_1.4.3_x86_64",
    permanent: false,
  },
  {
    source: "/dl/firezone-gateway/latest/aarch64",
    destination:
      // mark:current-gateway-version
      "https://www.github.com/firezone/firezone/releases/download/gateway-1.4.3/firezone-gateway_1.4.3_aarch64",
    permanent: false,
  },
  {
    source: "/dl/firezone-gateway/latest/armv7",
    destination:
      // mark:current-gateway-version
      "https://www.github.com/firezone/firezone/releases/download/gateway-1.4.3/firezone-gateway_1.4.3_armv7",
    permanent: false,
  },
  /*
   * Redirects for old KB URLs
   *
   */
  {
    // TODO: Remove on or after 2024-10-21 after crawlers have re-indexed
    source: "/kb/user-guides/:path",
    destination: "/kb/client-apps/:path",
    permanent: true,
  },
  {
    // TODO: Remove on or after 2024-10-21 after crawlers have re-indexed
    source: "/kb/user-guides",
    destination: "/kb/client-apps",
    permanent: true,
  },
];
