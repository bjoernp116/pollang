use std::env;
use std::fs;
use std::io::{self, Write};

use anyhow::anyhow;
mod scanner;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err(anyhow!("Usage: {} tokenize <filename>", args[0]));
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let (tokens, err_code) = scanner::scan(file_contents)?;
                for token in tokens {
                    println!("{:?}", token);
                }
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
                if err_code != 0 {
                    std::process::exit(err_code);
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            return Err(anyhow!("Unknown command: {}", command));
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_lexer() {
        let input = "(()".to_owned();
        let tokens = super::scanner::scan(input).unwrap();
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
