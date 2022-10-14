use clap::Parser;
// use std::collections::HashMap;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Current measure
   #[arg(short, long)]
   from: String,

   /// Desired measure
   #[arg(short, long)]
   to: String,

   /// Value
   value: u128,
}

fn main() {
   let args = Args::parse();
   print!("value: {:?}", args.value)
}
