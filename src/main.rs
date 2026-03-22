use clap::{Parser, Subcommand};

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
