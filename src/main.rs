use clap::{Args, Parser, Subcommand};
use os_info;

/// A tool written in Rust
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show informtions about the OS
    Os(Os),
}

#[derive(Args)]
struct Os {
    // name: Option<String>,
}

fn print_os_release() {
    let os_release = os_info::get();

    let info = os_info::get();
    println!("OS information: {info}");

    println!("OS: {}", os_release.os_type());
    println!("Version: {}", os_release.version());

    match os_release.edition() {
        Some(edition) => {
            println!("Edition: {:?}", edition);
        }
        None => {}
    }

    println!("Codename: {:#?}", os_release.codename());
    println!("Bitness: {}", os_release.bitness());
    println!("Architecture: {:#?}", os_release.architecture());
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Os(_) => {
            print_os_release();
        }
    }
}
