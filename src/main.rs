use std::error::Error;

use clap::Parser;
use rcalc::{Lexer, Token, convert_to_postfix};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    expr: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let tokens: Vec<Token> = match Lexer::new(&cli.expr).collect() {
        Ok(tokens) => tokens,
        Err(e) => return Err(e.into()),
    };

    println!("{:?}", convert_to_postfix(tokens));

    Ok(())
}
