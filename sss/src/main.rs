use clap::Parser;
mod models;
use models::{Args};

fn main(){
    //here we will take the input as from the parsser for the args
    let args = Args::parse();
    println!("{}, {} , {}",args.secret, args.number, args.k_items);
}