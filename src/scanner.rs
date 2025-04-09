use std::fmt::{Debug, Display};

use anyhow::anyhow;

#[derive(Clone)]
pub enum Token {
    Left_Paren,
    Right_Paren,
    Left_Brace,
    Right_Brace
}


pub fn scan(str: String) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();
    for c in str.chars() {
        match Token::try_from(c) {
            Ok(token) => out.push(token),
            Err(err) => {} // println!("{}", err)
        };
    }
    Ok(out)
}


impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let str: &str = match self.clone() {
            Left_Paren => "LEFT_PAREN",
            Right_Paren => "RIGHT_PAREN",
            Left_Brace => "LEFT_BRACE",
            Right_Brace => "RIGHT_BRACE",
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

impl TryFrom<char> for Token {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Token::Left_Paren),
            ')' => Ok(Token::Right_Paren),
            '{' => Ok(Token::Left_Brace),
            '}' => Ok(Token::Right_Brace),
            _ => Err(anyhow!("symbol not found: {}", value))
        }
    }
}

impl From<Token> for char {
    fn from(value: Token) -> Self {
        match value {
            Token::Left_Paren => '(',
            Token::Right_Paren => ')',
            Token::Left_Brace => '{',
            Token::Right_Brace => '}',
        }
    }
}
