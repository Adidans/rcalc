use clap::Parser;
use rcalc::Lexer;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    expr: String,
}

fn main() {
    let cli = Cli::parse();
    let lexer = Lexer::new(&cli.expr);
    for token in lexer {
        match token {
            Ok(token) => println!("{:?}", token),
            Err(err) => eprintln!("{err}"),
        }
    }
}
