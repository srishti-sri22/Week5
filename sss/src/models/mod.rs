pub mod Point;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {

    #[arg(short, long)]
    pub secret: String,
    
    #[arg(short, long)]
    pub number: usize,

    #[arg(short, long)]
    pub k_items: u32,
}