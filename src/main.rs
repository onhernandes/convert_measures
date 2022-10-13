use clap::Parser;

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
   value: u8,
}

fn main() {
   let args = Args::parse();
   print!("value: {:?}", args.value)
}
