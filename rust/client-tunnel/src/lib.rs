//! A library for the privileged tunnel process for a Linux Firezone Client
//!
//! This is built both standalone and as part of the GUI package. Building it
//! standalone is faster and skips all the GUI dependencies. We can use that build for
//! CLI use cases.
//!
//! Building it as a binary within the `gui-client` package allows the
//! Tauri deb bundler to pick it up easily.
//! Otherwise we would just make it a normal binary crate.

#[allow(clippy::print_stdout)]
pub fn run() {
    println!("Firezone Tunnel (library)");
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use serde::Serialize;
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

    const MESSAGE_ONE: &str = "message one";

    // Copied from <https://github.com/firezone/subzone>

    /// Reads a message from an async reader, with a 32-bit LE length prefix
    async fn read_ipc_msg<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf).await?;
        let len = u32::from_le_bytes(len_buf);
        let len = usize::try_from(len)?;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf).await?;
        Ok(buf)
    }

    /// Encodes a message as JSON and writes it to an async writer, with a 32-bit LE length prefix
    ///
    /// TODO: Why does this one take `T` and `read_ipc_msg` doesn't?
    async fn write_ipc_msg<W: AsyncWrite + Unpin, T: Serialize>(
        writer: &mut W,
        msg: &T,
    ) -> Result<()> {
        let buf = serde_json::to_string(msg)?;
        let len = u32::try_from(buf.len())?.to_le_bytes();
        writer.write_all(&len).await?;
        writer.write_all(buf.as_bytes()).await?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    mod linux {
        use std::time::Duration;
        use super::{MESSAGE_ONE, read_ipc_msg, write_ipc_msg};
        use tokio::net::{UnixListener, UnixStream};

        const MESSAGE_TWO: &str = "message two";

        #[tokio::test]
        async fn ipc_linux() {
            let sock_path = dirs::runtime_dir()
                .unwrap()
                .join("dev.firezone.client_ipc_test");

            // Remove the socket if a previous run left it there
            tokio::fs::remove_file(&sock_path).await.ok();
            let listener = UnixListener::bind(&sock_path).unwrap();

            let ipc_server_task = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let cred = stream.peer_cred().unwrap();
                // TODO: Check that the user is in the `firezone` group
                // For now, to make it work well in CI where that group isn't created,
                // just check if it matches our own UID.
                let actual_peer_uid = cred.uid();
                let expected_peer_uid = nix::unistd::Uid::current().as_raw();
                assert_eq!(actual_peer_uid, expected_peer_uid);

                let v = read_ipc_msg(&mut stream).await.unwrap();
                let s = String::from_utf8(v).unwrap();
                let decoded: String = serde_json::from_str(&s).unwrap();
                assert_eq!(MESSAGE_ONE, decoded);

                let v = read_ipc_msg(&mut stream).await.unwrap();
                let s = String::from_utf8(v).unwrap();
                let decoded: String = serde_json::from_str(&s).unwrap();
                assert_eq!(MESSAGE_TWO, decoded);
            });

            let mut stream = UnixStream::connect(&sock_path).await.unwrap();
            write_ipc_msg(&mut stream, &MESSAGE_ONE.to_string())
                .await
                .unwrap();

            write_ipc_msg(&mut stream, &MESSAGE_TWO.to_string())
                .await
                .unwrap();

            tokio::time::timeout(Duration::from_millis(2_000), ipc_server_task)
                .await
                .unwrap()
                .unwrap();
        }
    }

    #[cfg(target_os = "windows")]
    mod windows {
        use super::{MESSAGE_ONE, read_ipc_msg, write_ipc_msg};

        #[tokio::test]
        async fn ipc_windows() {
            // Round-trip a message to avoid dead code warnings
            let mut buffer = vec![];

            write_ipc_msg(&mut buffer, &MESSAGE_ONE.to_string()).await.unwrap();

            let mut cursor = std::io::Cursor::new(buffer);
            let v = read_ipc_msg(&mut cursor).await.unwrap();
            let s = String::from_utf8(v).unwrap();
            let decoded: String = serde_json::from_str(&s).unwrap();
            assert_eq!(decoded, MESSAGE_ONE);

            // TODO: Windows process splitting
            // <https://github.com/firezone/firezone/issues/3712>
        }
    }
}
