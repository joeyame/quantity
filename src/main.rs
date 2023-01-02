use anyhow::Result;
use clap::Parser;
use interface::*;
use std::path::PathBuf;

// Load in core modules
mod interface;
mod interpreting;
mod parsing;
mod scanning;

fn main() -> Result<()> {
    // Get command line arguments
    let cli = Cli::parse();

    match cli.filename {
        // If filename is provided, read and execute the file
        Some(name) => run_file(PathBuf::from(name))?,

        // Otherwise run the interactive REPL
        None => run_prompt()?,
    }

    // Exit with a success
    Ok(())
}

pub fn run(code: String) {
    println!("Scanning the following line:");
    println!("\"{}\"", code);

    // Turn source code into tokens
    let tokens = crate::scanning::scan_source(code);
    println!("Token list: {:#?}", tokens);

    // Turn tokens into an abstract syntax tree
    let tree = crate::parsing::Parser::parse_tokens(tokens);
    println!("AST {:#?}", tree);

    // Interpret the syntax tree
    crate::interpreting::interpret(tree).expect("Interpreter failed");
}
