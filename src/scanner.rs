use std::fmt::Display;

#[derive(Clone, Debug)]
#[allow(unused)]
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
    Carrot,

    If,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,

    Number(f64),
    StringLitteral(String),
    Identifier(String),
    Invalid(String),
}
#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub raw: String,
    pub line: usize
}

pub fn scan(str: String) -> anyhow::Result<Vec<Token>> {
    let mut out = Vec::new();
    let mut buffer: String = String::new();
    let mut line_number = 1;
    let stream: Vec<char> = str.chars().collect();
    let mut i = 0;
    while i < stream.len() {
        match stream[i] {
            ' ' | '\t' => {},
            '\n' => line_number += 1,
            x if x.is_numeric() => {
                let mut float = false;
                loop {
                    if i != stream.len() && stream[i] == '.' && !float {
                        float = true;
                        buffer.push('.');
                        i+=1;
                        continue;
                    }
                    if i == stream.len() || !stream[i].is_numeric() {

                        let number = buffer.clone().parse::<f64>()?;
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
                    buffer.push(stream[i]);
                    i += 1;
                }
            }
            '"' => {
                i += 1;
                loop {
                    if i == stream.len() {
                        let token = Token {
                            token_type: TokenType::Invalid(format!("Unterminated string.")),
                            raw: buffer.clone(),
                            line: line_number
                        };
                        out.push(token);
                        break;
                    }
                    if stream[i] == '"' {
                        let token = Token {
                            token_type: TokenType::StringLitteral(buffer.clone()),
                            raw: format!("\"{}\"", buffer.clone()),
                            line: line_number
                        };
                        buffer.clear();
                        out.push(token);
                        break;
                    }
                    buffer.push(stream[i]);
                    i += 1;
                }
            }
            '=' | '!' | '<' | '>' if i+1 < stream.len() && stream[i+1] == '=' => {
                let token_type = match stream[i] {
                    '=' => TokenType::EqualEqual,
                    '!' => TokenType::BangEqual,
                    '>' => TokenType::GreaterEqual,
                    '<' => TokenType::LessEqual,
                    _ => unreachable!()
                };
                let token = Token {
                    token_type,
                    raw: format!("{}{}", stream[i], stream[i+1]),
                    line: line_number
                };
                out.push(token);
                i += 1;
            }
            '/' if i+1 < stream.len() && stream[i+1] == '/' => {
                break;
            }
            c if c.is_alphabetic() || c == '_' => {
                loop {
                    if i == stream.len()
                    || !(stream[i].is_alphanumeric() || stream[i] == '_') {
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
                    buffer.push(stream[i]);
                    i += 1;
                }
            }
            _ => {
                let token = Token {
                    token_type: TokenType::from(stream[i]),
                    raw: format!("{}", stream[i]),
                    line: line_number
                };
                out.push(token);
            }
        }
        i += 1;
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
            Carrot => "CARROT",

            If => "IF",
            And => "AND",
            Class => "CLASS",
            Else => "ELSE",
            False => "FALSE",
            For => "FOR",
            Fun => "FUN",
            Nil => "NIL",
            Or => "OR",
            Print => "PRINT",
            Return => "RETURN",
            Super => "SUPER",
            This => "THIS",
            True => "TRUE",
            Var => "VAR",
            While => "WHILE",

            EqualEqual => "EQUAL_EQUAL",
            BangEqual => "BANG_EQUAL",
            LessEqual => "LESS_EQUAL",
            GreaterEqual => "GREATER_EQUAL",

            Number(_) => "NUMBER",
            StringLitteral(_) => "STRING",
            Identifier(_) => "IDENTIFIER"
        };
        let inner = match self.token_type.clone() {
            Number(n) => format!("{:?}", n),
            StringLitteral(s) => format!("{}", s),
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
            '^' => Carrot,
            c => Invalid(format!("Unexpected character: {}", c)),
        }
    }
}

impl From<String> for TokenType {
    fn from(value: String) -> Self {
        use TokenType::*;
        match value.as_str() {
            "if" => If,
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
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


