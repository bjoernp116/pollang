use std::{collections::VecDeque, fmt::{write, Display}};

use crate::scanner::{Token, TokenType};
use anyhow::anyhow;



#[derive(Clone)]
pub enum Node {
    Binary {
        left: Box<Node>,
        right: Box<Node>,
        operator: BinaryOperator,
    },
    /*Parenthesis {
        left: Box<Node>,
        right: Box<Node>,
    },*/
    Parenthesis(Box<Node>),
    Unary(UnaryOperator, Box<Node>),
    Litteral(Litteral)
}

#[derive(Clone)]
pub enum Litteral {
    Number(f64),
    Boolean(bool),
    Nil,
    String(String),
}
impl Display for Litteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Litteral::Number(n) => write!(f, "{}", n),
            Litteral::Boolean(b) => write!(f, "{}", b),
            Litteral::Nil => write!(f, "nil"),
            Litteral::String(s) => write!(f, "{}", s),
        }
    }
}

impl std::fmt::Debug for Litteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Litteral::Number(n) => write!(f, "{:?}", n),
            Litteral::Boolean(b) => write!(f, "{}", b),
            Litteral::Nil => write!(f, "nil"),
            Litteral::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone)]
pub enum UnaryOperator {
    Not,
    Neg,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not => write!(f, "!"),
            Self::Neg => write!(f, "-")
        }
    }
}

#[derive(Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,

    Eq,
    LEq,
    GEq,
    NEq,
    L,
    G,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
            BinaryOperator::Pow => "^",
            BinaryOperator::Eq => "==",
            BinaryOperator::LEq => "<=",
            BinaryOperator::GEq => ">=",
            BinaryOperator::NEq => "!=",
            BinaryOperator::L => "<",
            BinaryOperator::G => ">",
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
        Ok(self.parse_equality()?)
    }

    fn parse_equality(&mut self) -> anyhow::Result<Node> {
        let mut node: Node = self.parse_term()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::EqualEqual |
                TokenType::GreaterEqual |
                TokenType::LessEqual |
                TokenType::BangEqual |
                TokenType::Greater |
                TokenType::Less => {
                    let op = self.tokens[self.current].clone();
                    self.current += 1;
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_term()?);
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: op.try_into()?
                    };
                },
                _ => break
            }
        }
        Ok(node)
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
                        operator: op.try_into()?
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
                        operator: op.try_into()?
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
                        operator: op.try_into()?
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
            TokenType::Bang | TokenType::Minus => self.parse_unary(),
            _ => self.parse_number()
        }
    }

    fn parse_unary(&mut self) -> anyhow::Result<Node> {
        let op = self.tokens[self.current].clone();
        self.current += 1;
        let node: Node = self.parse_primary()?;
        let unary = Node::Unary(op.try_into()?, Box::new(node));
        Ok(unary)
    }

    fn parse_paren(&mut self) -> anyhow::Result<Node> {
        let mut open_p = 0;
        let mut private_tokens: VecDeque<Token> = VecDeque::new();

        if self.tokens.len() == 1 {
            return Ok(Node::Litteral(Litteral::Number(0.)));
        }
        match self.tokens[self.current].token_type {
            TokenType::LeftParen => {},
            _ => { return self.parse_number(); }
        };
        self.current += 1;
        open_p += 1;
        while self.current < self.tokens.len() && open_p != 0 {
            match self.tokens[self.current].token_type.clone() {
                TokenType::LeftParen => open_p += 1,
                TokenType::RightParen => open_p -= 1,
                _x => ()//println!("{:?}", x)
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
        let node = parser.parse_equality()?;
        Ok(Node::Parenthesis(Box::new(node)))
    }

    fn parse_number(&mut self) -> anyhow::Result<Node> {
        if self.current >= self.tokens.len() {
            return Err(anyhow!("Out of bounds access in parse_number"));
        }
        match &self.tokens[self.current].token_type {
            TokenType::Number(x) => {
                let number = *x;
                self.current += 1;
                Ok(Node::Litteral(Litteral::Number(number as f64)))
            },
            TokenType::True => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::Boolean(true)))
            },
            TokenType::False => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::Boolean(false)))
            },
            TokenType::Nil => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::Nil))
            },
            TokenType::StringLitteral(s) => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::String(s.clone())))
            }
            _ => {
                Err(anyhow!("[line {}] Error at '{}': Expect expression.", &self.tokens[self.current].line, &self.tokens[self.current].raw))
            }
        }
    }

}

impl TryFrom<Token> for BinaryOperator {
    type Error = anyhow::Error;
    fn try_from(token: Token) -> anyhow::Result<BinaryOperator> {
        match token.token_type {
            TokenType::Plus => Ok(BinaryOperator::Add),
            TokenType::Minus => Ok(BinaryOperator::Sub),
            TokenType::Star => Ok(BinaryOperator::Mul),
            TokenType::Slash => Ok(BinaryOperator::Div),
            TokenType::Carrot => Ok(BinaryOperator::Pow),
            TokenType::LessEqual => Ok(BinaryOperator::LEq),
            TokenType::GreaterEqual => Ok(BinaryOperator::GEq),
            TokenType::EqualEqual => Ok(BinaryOperator::Eq),
            TokenType::BangEqual => Ok(BinaryOperator::NEq),
            TokenType::Less => Ok(BinaryOperator::L),
            TokenType::Greater => Ok(BinaryOperator::G),
            _ => Err(anyhow!("Cant convert Token {} to operator", token))
        }
    }
}

impl TryFrom<Token> for UnaryOperator {
    type Error = anyhow::Error;
    fn try_from(token: Token) -> anyhow::Result<UnaryOperator> {
        match token.token_type {
            TokenType::Bang => Ok(UnaryOperator::Not),
            TokenType::Minus => Ok(UnaryOperator::Neg),
            _ => Err(anyhow!("Cant convert Token {} to operator", token))
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Unary(op, e) => write!(f, "({} {:?})", op, e),
            Node::Litteral(l) => write!(f, "{}", l),
            Node::Binary { left, right, operator } => write!(f, "({} {} {})", operator, left, right),
            Node::Parenthesis(e) => write!(f, "(group {})", e)
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Unary(op, e) => write!(f, "({} {})", op, e),
            Node::Litteral(l) => write!(f, "{:?}", l),
            Node::Binary { left, right, operator } => write!(f, "({} {:?} {:?})", operator, left, right),
            Node::Parenthesis(e) => write!(f, "(group {:?})", e)
        }
    }
}
