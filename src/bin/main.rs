use clap::{ArgGroup, Parser};

use camel::*;

/// Program accepts either a raw program or a filename as input
#[derive(Parser, Debug)]
#[command(name = "camel")]
#[command(about = "")]
#[command(group = ArgGroup::new("input").required(true).args(&["path", "raw"]))]
struct Args {
  /// Path to the file
  #[arg(short, long, group = "input")]
  path: Option<String>,

  /// Raw string input
  #[arg(short, long, group = "input")]
  raw: Option<String>,
}

fn main() {
  let args = Args::parse();

  if let Some(path) = args.path {
    println!("Path: {}", path);
  } else if let Some(raw) = args.raw {
    println!("Raw: {}", raw);
  }
}
