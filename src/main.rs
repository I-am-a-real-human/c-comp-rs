use std::fs;
mod lexer;
mod parser;
use crate::lexer::Lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("fcc help TODO");
        return ();
    }

    let source = fs::read_to_string(&args[1]).expect("Couldn't read file '{}'");
    let mut lexer = Lexer::from_string(&source);

    let result = lexer.tokenise();

    match result {
        Ok(tokens) => {
            for token in tokens.iter() {
                println!("{} | ", token);
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("Syntax error: {:?}", error);
            }
            std::process::exit(1);
        }
    }

    for token in lexer.tokens() {
        print!("{} | ", token);
    }
    println!("");
}
