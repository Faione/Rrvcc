use std::error::Error;

use clap::Parser;

use crate::util::strol;

mod util;

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

    let mut chars = args.args[0].chars().peekable();

    println!("  .globl main");
    println!("main:");

    println!("  li a0, {}", strol(&mut chars));

    while let Some(c) = chars.next() {
        match c {
            '+' => {
                println!("  addi a0, a0, {}", strol(&mut chars));
            }
            '-' => {
                println!("  addi a0, a0, -{}", strol(&mut chars));
            }
            _ => {
                eprintln!("unexpected character: {}", c);
                std::process::exit(1);
            }
        }
    }

    println!("  ret");

    Ok(())
}
