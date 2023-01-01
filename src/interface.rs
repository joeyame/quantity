use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use read_input::prelude::*;

use crate::run;

#[derive(Parser)]
#[command(author = "Joey M <josephameadows@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Interprets or compiles the quantity code")]
pub struct Cli {
    /// Optional quantity file to execute
    pub filename: Option<String>,
}

pub fn run_file(path: PathBuf) -> Result<()> {
    let file_text = fs::read_to_string(path)?;
    run(file_text);
    Ok(())
}

pub fn run_prompt() -> Result<()> {
    println!("Welcome to the Qty REPL");
    loop {
        // Get user input
        let user_input: String = input().msg("> ").get();

        // Allow exit
        if user_input == "q" {
            break;
        }

        // Otherwise run code
        run(user_input);
    }

    Ok(())
}
