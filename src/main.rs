use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "gos-mirror", about = "Air-gapped GrapheneOS release mirror")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Sync releases from upstream
    Sync,
    /// Serve mirrored releases locally
    Serve,
}

/// Configuration for a release mirror.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MirrorConfig {
    pub upstream_url: String,
    pub local_dir: String,
    pub devices: Vec<String>,
    pub channel: String,
}

/// Sync status for a single device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MirrorStatus {
    pub device: String,
    pub local_version: Option<String>,
    pub upstream_version: Option<String>,
    pub needs_sync: bool,
}

/// Determine if a device needs syncing based on local and upstream versions.
///
/// Rules:
/// - Both `None` -> no sync needed (nothing to do)
/// - Upstream only -> needs sync (new release available)
/// - Both same -> no sync needed (already up to date)
/// - Both different -> needs sync (newer version upstream)
/// - Local only -> no sync needed (upstream removed or unavailable)
#[must_use]
pub fn check_needs_sync(local: Option<&str>, upstream: Option<&str>) -> bool {
    match (local, upstream) {
        (None, None) => false,
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (Some(l), Some(u)) => l != u,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Sync => {
            println!("gos-mirror: syncing from releases.grapheneos.org");
        }
        Command::Serve => {
            println!("gos-mirror: serving local mirror");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_none_no_sync() {
        assert!(!check_needs_sync(None, None));
    }

    #[test]
    fn upstream_only_needs_sync() {
        assert!(check_needs_sync(None, Some("2026030100")));
    }

    #[test]
    fn both_same_no_sync() {
        assert!(!check_needs_sync(Some("2026030100"), Some("2026030100")));
    }

    #[test]
    fn both_different_needs_sync() {
        assert!(check_needs_sync(Some("2026020100"), Some("2026030100")));
    }

    #[test]
    fn local_only_no_sync() {
        assert!(!check_needs_sync(Some("2026030100"), None));
    }
}
