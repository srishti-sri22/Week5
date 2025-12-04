// main.rs - The entry point of the application
// This file handles command-line arguments and delegates work to other modules

mod cli;
mod crypto;
mod math;
mod commands;

use clap::Parser;
use cli::Args;

fn main() {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Execute the appropriate command
    commands::execute(args.cmd);
}