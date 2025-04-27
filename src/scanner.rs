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
    Equal,
    Bang,
    Greater,
    Less,

    If,
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,

    Number(u64),
    StringLitteral(String),
    Identifier(String),
    Invalid(String),
}
pub struct Token {
    pub token_type: TokenType,
    pub raw: String,
    pub line: usize
}

pub fn scan(str: String) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();
    let mut buffer: String = String::new();
    for (line_number, line) in str.lines().enumerate() {
        let line_number = line_number + 1;
        let line: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < line.len() {
            match line[i] {
                ' ' | '\n' | '\t' => {},
                x if x.is_numeric() => {
                    loop {
                        if i == line.len() || !line[i].is_numeric() {
                            
                            let number = buffer.clone().parse::<u64>()?;
                            let token = Token {
                                token_type: TokenType::Number(number),
                                raw: buffer.clone(),
                                line: line_number
                            };
                            buffer.clear();
                            i -= 1;
                            out.push(token);
                            break;
                        }
                        buffer.push(line[i]);
                        i += 1;
                    }
                }
                '"' => {
                    i += 1;
                    loop {
                        if i == line.len() {
                            let token = Token {
                                token_type: TokenType::Invalid(format!("Unterminated string.")),
                                raw: buffer.clone(),
                                line: line_number
                            };
                            out.push(token);
                            break;
                        }
                        
                        //println!("{}/{}, {}", i, line.len(), line[i]);

                       
                        if line[i] == '"' {
                            let token = Token {
                                token_type: TokenType::StringLitteral(buffer.clone()),
                                raw: format!("\"{}\"", buffer.clone()),
                                line: line_number
                            };
                            buffer.clear();
                            out.push(token);
                            break;
                        }
                        buffer.push(line[i]);
                        i += 1;
                    }
                }
                '=' | '!' | '<' | '>' if i+1 < line.len() && line[i+1] == '=' => {
                    let token_type = match line[i] {
                        '=' => TokenType::EqualEqual,
                        '!' => TokenType::BangEqual,
                        '>' => TokenType::GreaterEqual,
                        '<' => TokenType::LessEqual,
                        _ => unreachable!()
                    };
                    let token = Token {
                        token_type,
                        raw: format!("{}{}", line[i], line[i+1]),
                        line: line_number
                    };
                    out.push(token);
                    i += 1;
                }
                '/' if i+1 < line.len() && line[i+1] == '/' => {
                    break;
                }
                c if c.is_alphabetic() => {
                    loop {
                        if i == line.len() || !line[i].is_alphabetic() {
                            let token = Token {
                                token_type: TokenType::from(buffer.clone()),
                                raw: buffer.clone(),
                                line: line_number
                            };
                            buffer.clear();
                            out.push(token);
                            i -= 1;
                            break;
                        }
                        buffer.push(line[i]);
                        i += 1;
                    }   
                }
                _ => {
                    let token = Token {
                        token_type: TokenType::from(line[i]),
                        raw: format!("{}", line[i]),
                        line: line_number
                    };
                    out.push(token);
                }
            }
            i += 1;
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
            Equal => "EQUAL",
            Bang => "BANG",
            Greater => "GREATER",
            Less => "LESS",

            If => "IF",

            EqualEqual => "EQUAL_EQUAL",
            BangEqual => "BANG_EQUAL",
            LessEqual => "LESS_EQUAL",
            GreaterEqual => "GREATER_EQUAL",

            Number(_) => "NUMBER",
            StringLitteral(_) => "STRING",
            Identifier(_) => "IDENTIFIER"
        };
        let inner = match self.token_type.clone() {
            Number(n) => format!("{}", n),
            StringLitteral(s) | Identifier(s) => format!("{}", s),
            _ => format!("null")
        };
        write!(f, "{} {} {}", str, self.raw, inner)?;
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
            '=' => Equal,
            '!' => Bang,
            '>' => Greater,
            '<' => Less,
            c => Invalid(format!("Unexpected character: {}", c)),         
        }
    }
}

impl From<String> for TokenType {
    fn from(value: String) -> Self {
        use TokenType::*;
        match value.as_str() {
            "if" => If,
            _ => Identifier(value)
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


