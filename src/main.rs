use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use anyhow::anyhow;
use scanner::{Token, TokenType};
mod scanner;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    command: Command,

    #[arg()]
    file_path: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    #[clap(name = "tokenize", alias = "t")]
    Tokenize,
}

enum ExitCode {
    Success,
    Error(i32),
}
impl From<i32> for ExitCode {
    fn from(value: i32) -> Self {
        match value {
            0 => ExitCode::Success,
            err => ExitCode::Error(err),
        }
    }
}

impl ExitCode {
    fn exit(self) {
        match self {
            ExitCode::Success => std::process::exit(0),
            ExitCode::Error(err) => std::process::exit(err),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let file_contents: String = if let Ok(fc) = fs::read_to_string(&args.file_path) {
        fc.to_owned()
    } else {
        return Err(anyhow!("Failed to read file {}", args.file_path.display()));
    };

    match args.command {
        Command::Tokenize => {
            let tokens: Vec<Token> = scanner::scan(file_contents)?;
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            //
            let exit_code = if tokens.iter().any(|t| !t.is_valid()) {
                ExitCode::Error(65)
            } else {
                ExitCode::Success
            };

            for token in tokens {
                if let TokenType::Invalid(e) = token.token_type {
                    eprintln!("[line {}] Error: {}", token.line, e);
                } else {
                    println!("{}", token);
                }
            }

            println!("EOF null"); // Placeholder, remove this line when implementing the scanner
            exit_code.exit();
        }
    }
    Ok(())
}


