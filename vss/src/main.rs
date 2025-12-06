mod cli;
mod crypto;
mod math;
mod commands;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    commands::execute(args.cmd);
}