use anyhow::{bail, Result};
use clap::Parser;
use std::{env, error::Error, fs, path::PathBuf, process::ExitCode, str::FromStr};

#[derive(Parser)]
#[command(author = "Joey M <josephameadows@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Interprets or compiles the quantity code")]
struct Cli {
    /// Optional quantity file to execute
    name: Option<String>,
}

fn run(code: String) {
    println!("Running the following line:");
    println!("{}, ", code);
}

fn run_file(path: PathBuf) -> Result<()> {
    let file_text = fs::read_to_string(path)?;

    Ok(())
}

fn run_prompt() -> Result<()> {
    use std::io::stdin;
    println!("Welcome to the Qty REPL");
    loop {
        println!("> ");
        let mut buff = String::new();
        stdin().read_line(&mut buff)?;
        if let Some('\n') = buff.chars().next_back() {
            buff.pop();
        }
        if let Some('\r') = buff.chars().next_back() {
            buff.pop();
        }
        run(buff);
    }

    Ok(())
}

fn main() -> Result<()> {
    // let args: Vec<String> = env::args().collect();

    // Check for help flag
    let cli = Cli::parse();
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    // let thing = match args[..] {
    //     x if x == [] => "empty",
    //     _ => "Not empty",
    // }

    // let Ok(file_name) = args.get(1).ok_or("This is an error".to_owned());

    // let file_path = PathBuf::from(file_name);

    // if !file_path.exists() {
    //     println!("\"{}\" does not exist!", file_name);
    //     return ExitCode::from(2);
    // }

    // println!("Executing {}", file_name);
    // if let Err(error) = run_file(file_path) {
    //     println!("{}", error);
    //     return ExitCode::from(3);
    // }
    // println!("Execution complete");

    // Exit with a success
    Ok(())
}
