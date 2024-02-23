//! The Firezone GUI client for Windows

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> anyhow::Result<()> {
    if false {
        bogus().ok();
    }
    client::run()
}

#[cfg(target_family = "unix")]
mod client {
    pub(crate) fn run() -> anyhow::Result<()> {
        println!("The Windows client does not compile on non-Windows platforms yet");
        Ok(())
    }
}

/// Everything is hidden inside the `client` module so that we can exempt the
/// Windows client from static analysis on other platforms where it would throw
/// compile errors.
#[cfg(target_os = "windows")]
mod client;

/// Used to teset #3752 - mutation testing
fn bogus() -> anyhow::Result<()> {
    anyhow::bail!(
        "`cargo-mutants` should change this to `Ok(())` and then mark it as a missed mutant"
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn bogus() {
        super::bogus().ok();
    }
}
