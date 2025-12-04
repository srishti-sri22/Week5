// cli.rs - Defines command-line interface structure
// This file specifies what commands and arguments the program accepts

use clap::{Parser, Subcommand};

/// Main arguments structure for the CLI
#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

/// Available commands for the secret sharing application
#[derive(Subcommand)]
pub enum Command {
    /// Split a secret into multiple shares
    Split {
        /// The secret to split
        #[arg(long)]
        secret: String,
        
        /// Total number of shares to create
        #[arg(long)]
        n: usize,
        
        /// Minimum number of shares needed to reconstruct
        #[arg(long)]
        k: usize,
    },
    
    /// Verify that a secret matches its commitment
    VerifySecret {
        /// The secret to verify
        #[arg(long)]
        secret: String,
        
        /// Comma-separated list of commitments
        #[arg(long)]
        commitments: String,
    },
    
    /// Verify that a share is valid
    VerifyShare {
        /// Semicolon-separated shares in format 'x1,y1;x2,y2;...'
        #[arg(long)]
        share: String,
        
        /// Comma-separated list of commitments
        #[arg(long)]
        commitments: String,
        
        /// Show detailed verification steps
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
    
    /// Reconstruct the original secret from shares
    Reconstruct {
        /// Semicolon-separated list of shares (format: 'x1,y1;x2,y2;...')
        #[arg(long)]
        shares: String,
    },
}