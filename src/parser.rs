use std::{collections::VecDeque, fmt::Display};

use crate::scanner::{Token, TokenType};
use anyhow::anyhow;



pub enum Node {
    Number(f64),
    Binary {
        left: Box<Node>,
        right: Box<Node>,
        operator: Operator,
    },
    Boolean(bool),
    Nil,
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Mul => '*',
            Operator::Div => '/',
            Operator::Pow => '^',
        };
        write!(f, "{}", op)
    }
}

pub struct AstFactory {
    //head: Node,
    current: usize,
    tokens: VecDeque<Token>
}

impl AstFactory {

    pub fn new(input: Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens: input.into()
        }
    }
    pub fn parse(&mut self) -> anyhow::Result<Node> {
        Ok(self.parse_term()?)
    }

    fn parse_term(&mut self) -> anyhow::Result<Node> {
        let mut node: Node = self.parse_factor()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::Plus | TokenType::Minus => {
                    let op = self.tokens[self.current].clone();
                    self.current += 1;
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_factor()?);
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: to_operator(op)?
                    };
                },
                _ => break
            }
        }
        Ok(node)
    }

    fn parse_factor(&mut self) -> anyhow::Result<Node> {
        let mut node: Node = self.parse_exponent()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::Star | TokenType::Slash => {
                    let op = self.tokens[self.current].clone();
                    self.current += 1;
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_exponent()?);
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: to_operator(op)?
                    };
                },
                _ => break
            }
        }
        Ok(node)
    }

    fn parse_exponent(&mut self) -> anyhow::Result<Node> {
        let mut node: Node = self.parse_primary()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::Carrot => {
                    let op = self.tokens[self.current].clone();
                    self.current += 1;
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_primary()?);
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: to_operator(op)?
                    };
                },
                _ => break
            }
        }
        Ok(node)
    }

    fn parse_primary(&mut self) -> anyhow::Result<Node> {
        if self.current >= self.tokens.len() {
            return Err(anyhow!("Out of bounds access in parse_primary"));
        }
        match self.tokens[self.current].token_type {
            TokenType::LeftParen => self.parse_paren(),
            _ => self.parse_number()
        }
    }

    fn parse_paren(&mut self) -> anyhow::Result<Node> {
        let mut open_p = 0;
        let mut private_tokens: VecDeque<Token> = VecDeque::new();

        if self.tokens.len() == 1 {
            return Ok(Node::Number(0.));
        }
        match self.tokens[self.current].token_type {
            TokenType::LeftParen => {},
            _ => { return self.parse_number(); }
        };
        self.current += 1;
        open_p += 1;
        while self.current < self.tokens.len() && open_p != 0 {
            match self.tokens[self.current].token_type {
                TokenType::LeftParen => open_p += 1,
                TokenType::RightParen => open_p -= 1,
                _ => ()
            }
            private_tokens.push_back(self.tokens[self.current].clone());
            self.current += 1;
        }
        if open_p != 0 {
            return Err(anyhow!("Unescaped parenthesis"));
        }
        let mut parser = AstFactory {
            tokens: private_tokens,
            current: 0,
        };
        let node = parser.parse_term()?;
        Ok(node)
    }

    fn parse_number(&mut self) -> anyhow::Result<Node> {
        if self.current >= self.tokens.len() {
            return Err(anyhow!("Out of bounds access in parse_number"));
        }
        match &self.tokens[self.current].token_type {
            TokenType::Number(x) => {
                let number = *x;
                self.current += 1;
                Ok(Node::Number(number as f64))
            },
            TokenType::True => {
                self.current += 1;
                Ok(Node::Boolean(true))
            },
            TokenType::False => {
                self.current += 1;
                Ok(Node::Boolean(false))
            },
            TokenType::Nil => {
                self.current += 1;
                Ok(Node::Nil)
            }
            _ => Err(anyhow!("The token {} is not a number!", &self.tokens[self.current]))
        }
    }

}

fn to_operator(token: Token) -> anyhow::Result<Operator> {
    match token.token_type {
        TokenType::Plus => Ok(Operator::Add),
        TokenType::Minus => Ok(Operator::Sub),
        TokenType::Star => Ok(Operator::Mul),
        TokenType::Slash => Ok(Operator::Div),
        TokenType::Carrot => Ok(Operator::Pow),
        _ => Err(anyhow!("Cant convert Token {} to operator", token))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Number(n) => write!(f, "{:?}", n),
            Node::Binary { left, right, operator } => write!(f, "({} {} {})", operator, left, right),
            Node::Boolean(b) => write!(f, "{}", b),
            Node::Nil => write!(f, "nil")
        }
    }
}
