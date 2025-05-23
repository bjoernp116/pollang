use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use anyhow::anyhow;
use interpreter::Interpreter;
use parser::{AstFactory, Statement};
use scanner::{Token, TokenType};
mod interpreter;
mod parser;
mod scanner;
mod position;
mod environment;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    command: Command,

    #[arg()]
    file_path: PathBuf,

    #[arg(short, long, default_value_t = false)]
    debug: bool
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    #[clap(name = "tokenize", alias = "t")]
    Tokenize,
    #[clap(name = "parse", alias = "p")]
    Parse,
    #[clap(name = "evaluate", alias = "e")]
    Evaluate,
    #[clap(name = "run", alias = "r")]
    Run,
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
                    eprintln!("[line {}] Error: {}", token.position.line(), e);
                } else {
                    println!("{}", token);
                }
            }

            println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            exit_code.exit();
        }
        Command::Parse => {
            let tokens: Vec<Token> = scanner::scan(file_contents)?;
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            //
            let mut exit_code = if tokens.iter().any(|t| !t.is_valid()) {
                ExitCode::Error(65)
            } else {
                ExitCode::Success
            };

            /*for token in tokens.clone() {
                if let TokenType::Invalid(e) = token.token_type {
                    eprintln!("[line {}] Error: {}", token.line, e);
                } else {
                    println!("{}", token);
                }
            }*/

            let mut ast = AstFactory::new(tokens);
            match ast.parse_equality() {
                Ok(h) => println!("{:?}", h),
                Err(e) => {
                    eprintln!("{}", e);
                    exit_code = ExitCode::Error(65);
                }
            };

            exit_code.exit();
        }
        Command::Evaluate => {
            let tokens: Vec<Token> = scanner::scan(file_contents)?;
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            //
            let mut _exit_code = if tokens.iter().any(|t| !t.is_valid()) {
                ExitCode::Error(65)
            } else {
                ExitCode::Success
            };

            /*for token in tokens.clone() {
                if let TokenType::Invalid(e) = token.token_type {
                    eprintln!("[line {}] Error: {}", token.line, e);
                } else {
                    println!("{}", token);
                }
            }*/

            let mut ast = AstFactory::new(tokens);
            let statement = ast.parse_equality()?;
            let statement = Statement::Print(statement);
            let mut interpreter = Interpreter::new();
            interpreter.execute(statement)?;
            
        }
        Command::Run => {
            let tokens: Vec<Token> = scanner::scan(file_contents)?;
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            //
            let mut _exit_code = if tokens.iter().any(|t| !t.is_valid()) {
                ExitCode::Error(65)
            } else {
                ExitCode::Success
            };

            // for token in tokens.clone() {
            //     if let TokenType::Invalid(e) = token.token_type {
            //         eprintln!("[line {}] Error: {}", token.line, e);
            //     } else {
            //         println!("{}", token);
            //     }
            // }

            let mut ast: AstFactory = AstFactory::new(tokens);
            let statements = ast.parse_statements()?;
            if args.debug {
                println!("DEBUG: {{");
                for stmt in statements.clone() {
                    println!("\t{}", stmt);
                }
                println!("}}");
            }
            let mut interpreter = Interpreter::new();
            interpreter.interpret(statements)?;
        }
    }
    Ok(())
}
