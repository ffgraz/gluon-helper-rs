#![feature(ip_in_core)]

use core::net::Ipv4Addr;

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use libgluonhelper::arp::ArpResolver;

mod arp;

/*

TODO:
cli with commands
- fetch_json <url>: <json>
- resolve_arp <ip>: mac

 */

fn main() {
    // let mut resolver = ArpResolver::new();
    // resolver.ip4_to_mac("127.0.0.1".to_string().parse().unwrap());
    println!("Hello, world!");

    let args = Cli::parse();

    match args.command {
        Commands::Resolve { ip_address } => {
            println!("Resolving {ip_address}");
        }
        Commands::Diff {
            mut base,
            mut head,
            mut path,
            color,
        } => {
            if path.is_none() {
                path = head;
                head = None;
                if path.is_none() {
                    path = base;
                    base = None;
                }
            }
            let base = base
                .as_deref()
                .map(|s| s.to_str().unwrap())
                .unwrap_or("stage");
            let head = head
                .as_deref()
                .map(|s| s.to_str().unwrap())
                .unwrap_or("worktree");
            let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
            println!(
                "Diffing {}..{} {} (color={})",
                base,
                head,
                path.to_string_lossy(),
                color
            );
        }
        Commands::FetchJson { url: remote } => {
            println!("Pushing to {remote}");
        }
        Commands::Add { path } => {
            println!("Adding {path:?}");
        }
        Commands::Stash(stash) => {
            let stash_cmd = stash.command.unwrap_or(StashCommands::Push(stash.push));
            match stash_cmd {
                StashCommands::Push(push) => {
                    println!("Pushing {push:?}");
                }
                StashCommands::Pop { stash } => {
                    println!("Popping {stash:?}");
                }
                StashCommands::Apply { stash } => {
                    println!("Applying {stash:?}");
                }
            }
        }
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }

    // Continued program logic goes here...
}

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Resolves an IPv4 address to a MAC address
    #[command(arg_required_else_help = true)]
    Resolve {
        /// The IPv4 address to resolve
        ip_address: String,
    },
    /// Compare two commits
    Diff {
        #[arg(value_name = "COMMIT")]
        base: Option<OsString>,
        #[arg(value_name = "COMMIT")]
        head: Option<OsString>,
        #[arg(last = true)]
        path: Option<OsString>,
        #[arg(
            long,
            require_equals = true,
            value_name = "WHEN",
            num_args = 0..=1,
            default_value_t = ColorWhen::Auto,
            default_missing_value = "always",
            value_enum
        )]
        color: ColorWhen,
    },
    /// Fetches JSON from a URL
    #[command(arg_required_else_help = true)]
    FetchJson {
        /// The URL to fetch
        url: String,
    },
    /// adds things
    #[command(arg_required_else_help = true)]
    Add {
        /// Stuff to add
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    Stash(StashArgs),
    #[command(external_subcommand)]
    External(Vec<OsString>),
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl std::fmt::Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct StashArgs {
    #[command(subcommand)]
    command: Option<StashCommands>,

    #[command(flatten)]
    push: StashPushArgs,
}

#[derive(Debug, Subcommand)]
enum StashCommands {
    Push(StashPushArgs),
    Pop { stash: Option<String> },
    Apply { stash: Option<String> },
}

#[derive(Debug, Args)]
struct StashPushArgs {
    #[arg(short, long)]
    message: Option<String>,
}
