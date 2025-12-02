use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands{

    //command ke hisaab se split ya reconstruct krenge hm us secret ko, jo user cli mei batayega uske hisaab se
    Split{
        #[arg(short, long)]
     secret: String,
    
    #[arg(short, long)]
     n: u32,

    #[arg(short, long)]
    k: u32,

    #[arg(short, long, default_value = "340282366920938463463374607431768211297")]
     p: u128,
    },
    Reconstruct{
        #[arg(short, long, default_value = "340282366920938463463374607431768211297")]
   prime: u128,

    #[arg(short, long)]
    shares: String,
    }
}

#[derive(Debug, Clone)]
pub struct Point{
  x: i32,
  y: i32
}