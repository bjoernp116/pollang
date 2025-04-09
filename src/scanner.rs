use std::fmt::{Debug, Display};

use anyhow::anyhow;

#[derive(Clone)]
pub enum TokenType {
    Left_Paren,
    Right_Paren,
    Left_Brace,
    Right_Brace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Slash,
    SemiColon,
}
pub struct Token {
    token_type: TokenType,
    raw: char,
    line: usize
}


pub fn scan(str: String) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();
    let mut line = 1usize;
    for c in str.chars() {
        match c {
            '\n' => line += 1,
            ' ' => {},
            _ => {
                let token_type = match TokenType::try_from(c) {
                    Ok(x) => x,
                    Err(err) => {
                        println!("[line {}] Error: {}", line, err);
                        std::process::exit(65);
                    },
                };
                let token = Token {
                    token_type,
                    raw: c,
                    line
                };
                out.push(token)
            }
        }
    }
    Ok(out)
}


impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        let str: &str = match self.token_type.clone() {
            Left_Paren => "LEFT_PAREN",
            Right_Paren => "RIGHT_PAREN",
            Left_Brace => "LEFT_BRACE",
            Right_Brace => "RIGHT_BRACE",
            Star => "STAR",
            Dot => "DOT",
            Comma => "COMMA",
            Plus => "PLUS",
            Minus => "MINUS",
            Slash => "SLASH",
            SemiColon => "SEMICOLON",
        };
        write!(f, "{}", str)?;
        Ok(())
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{}", self);
        write!(f, "{} {} null", name, self.raw)?;
        Ok(())
    }
}

impl TryFrom<char> for TokenType {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use TokenType::*;
        let token_type = match value {
            '(' => Left_Paren,
            ')' => Right_Paren,
            '{' => Left_Brace,
            '}' => Right_Brace,
            '*' => Star,
            '.' => Dot,
            ',' => Comma,
            '+' => Plus,
            '-' => Minus,
            '/' => Slash,
            ';' => SemiColon,
            _ => return Err(anyhow!("Unexpected character: {}", value))
        };
        Ok(token_type)
    }
}

