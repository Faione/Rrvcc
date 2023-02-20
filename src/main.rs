use std::error::Error;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    args: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.args.len() < 1 {
        eprintln!("invalid number of arguments: {}", args.args.len());
        std::process::exit(1);
    }

    let num = args.args[0].parse::<i32>()?;

    println!("  .globl main");
    println!("main:");
    println!("  li a0, {}", num);
    println!("  ret");

    Ok(())
}
