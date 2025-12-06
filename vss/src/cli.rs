use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Split {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        n: usize,
        #[arg(long)]
        k: usize,
    },

    VerifySecret {
        #[arg(long,short)]
        secret: String,
        #[arg(long,short)]
        commitments: String,
    },

    VerifyShare {
        #[arg(long,short)]
        share: String,       
        #[arg(long,short)]
        commitments: String,
        #[arg(long, default_value_t = false)]
        verbose: bool,
    },
    
    Reconstruct {
        #[arg(long,short)]
        shares: String,
    },

}