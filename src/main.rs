use clap::{Parser, Subcommand};
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
    /// Show informations about the OS
    Os,

    /// Show the session type on Linux: wayland, x11, etc.
    #[clap(aliases = &["session", "s"])]
    SessionType,
}

fn print_os_release() {
    let os_release = os_info::get();

    let info = os_info::get();
    println!("OS information: {info}");

    println!("OS: {}", os_release.os_type());
    println!("Version: {}", os_release.version());

    match os_release.edition() {
        Some(edition) => {
            println!("Edition: {}", edition);
        }
        None => {}
    }

    if os_release.codename().is_some() {
        println!("Codename: {}", os_release.codename().unwrap());
    }

    println!("Bitness: {}", os_release.bitness());

    if os_release.architecture().is_some() {
        println!("Architecture: {}", os_release.architecture().unwrap());
    }
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
        Commands::Os => {
            print_os_release();
        }

        Commands::SessionType => {
            print_session_type();
        }
    }
}
