use clap::{Args, Parser, Subcommand};
use os_info;
use std::env;

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

    /// Show the session type on Linux: wayland, x11, etc.
    SessionType,
}

#[derive(Args)]
struct Os {}

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

fn print_session_type() {
    match env::var("XDG_SESSION_TYPE") {
        Ok(val) => println!("{}", val),
        Err(_e) => println!("Couldn't read $XDG_SESSION_TYPE"),
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Os(_) => {
            print_os_release();
        }

        Commands::SessionType => {
            print_session_type();
        }
    }
}
