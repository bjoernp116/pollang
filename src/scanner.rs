use std::fmt::{Debug, Display};

use anyhow::anyhow;

#[derive(Clone)]
pub enum Token {
    Left_Paren,
    Right_Paren,
}


pub fn scan(str: String) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();
    for c in str.chars() {
        let token: Token = match c {
            '(' => Token::Left_Paren,
            ')' => Token::Right_Paren,
            _ => return Err(anyhow!("Unknown symbol: {}", c))
        };
        out.push(token);
    }
    Ok(out)
}


impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let str: &str = match self.clone() {
            Left_Paren => "LEFT PAREN",
            Right_Paren => "RIGHT PAREN",
        };
        write!(f, "{}", str)?;
        Ok(())
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{}", self);
        let raw: char = self.clone().into();
        write!(f, "{} {} null", name, raw)?;
        Ok(())
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '(' => Token::Left_Paren,
            ')' => Token::Right_Paren,
            _ => todo!()
        }
    }
}

impl From<Token> for char {
    fn from(value: Token) -> Self {
        match value {
            Token::Left_Paren => '(',
            Token::Right_Paren => ')',
        }
    }
}
