use clap::{ArgAction, Args, Parser, Subcommand};
use windows::core::GUID;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Options {
    /// Logging verbosity (-v debug, -vv trace)
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count, global = true)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print a summary of all providers, countersets, counters, and instances.
    Summary,
    /// Print detailed information about a counterset and its counters and instances.
    Counterset(Counterset),
}

#[derive(Args, Debug)]
pub struct Counterset {
    /// The counterset's GUID, e.g. 811BBCE5-7327-4AD9-AB62-A8B955F61EEF
    pub guid: GUID,
}
