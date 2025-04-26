use std::fmt::Display;

#[derive(Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Slash,
    SemiColon,

    Invalid(String),
}
pub struct Token {
    pub token_type: TokenType,
    pub raw: char,
    pub line: usize
}


pub fn scan(str: String) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();
    let mut line = 1usize;
    for c in str.chars() {
        match c {
            '\n' => line += 1,
            ' ' => {},
            _ => {
                let token_type = TokenType::from(c);
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
            Invalid(err) => {
                eprint!("[line {}]: Error: {}", self.line, err);
                return Ok(());
            },
            LeftParen => "LEFT_PAREN",
            RightParen => "RIGHT_PAREN",
            LeftBrace => "LEFT_BRACE",
            RightBrace => "RIGHT_BRACE",
            Star => "STAR",
            Dot => "DOT",
            Comma => "COMMA",
            Plus => "PLUS",
            Minus => "MINUS",
            Slash => "SLASH",
            SemiColon => "SEMICOLON",
        };
        write!(f, "{} {} null", str, self.raw)?;
        Ok(())
    }
}

impl From<char> for TokenType {
    fn from(value: char) -> Self {
        use TokenType::*;
        match value {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '*' => Star,
            '.' => Dot,
            ',' => Comma,
            '+' => Plus,
            '-' => Minus,
            '/' => Slash,
            ';' => SemiColon,
            c => Invalid(format!("Unexpected character: {}", c)),         
        }
    }
}

impl Token {
    pub fn is_valid(&self) -> bool {
        match self.token_type {
            TokenType::Invalid(_) => false,
            _ => true
        }
    }
}
