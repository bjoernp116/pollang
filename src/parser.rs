use std::{collections::VecDeque, fmt::Display};

use crate::{position::Position, scanner::{Token, TokenType}};
use anyhow::anyhow;

#[derive(Clone)]
pub enum Node {
    Binary {
        left: Box<Node>,
        right: Box<Node>,
        operator: BinaryOperator,
        position: Position,
    },
    Parenthesis(Box<Node>),
    Unary(UnaryOperator, Box<Node>, Position),
    Litteral(Litteral, Position),
    Identifier(String, Position),
    Assignment(String, Box<Node>, Position),
}

#[derive(Clone)]
pub enum Litteral {
    Number(f64),
    Boolean(bool),
    Nil,
    String(String),
}

#[derive(Clone)]
pub enum Statement {
    Expression(Node),
    Print(Node),
    VarDecl(String, Node),
    Block(Vec<Statement>),
    If(Node, Box<Statement>, Option<Box<Statement>>),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Print(t) => write!(f, "print: {}", t)?,
            Statement::Expression(e) => write!(f, "expr: {}", e)?,
            Statement::VarDecl(i, e) => write!(f, "decl: {} = {}", i, e)?,
            Statement::Block(block) => {
                writeln!(f, "block: {{\n")?;
                for stmnt in block {
                    writeln!(f, "\t{}", stmnt)?;
                }
                writeln!(f, "}}\n")?;
            },
            Statement::If(condition, then, els) => {
                writeln!(f, "if {}", condition)?;
                writeln!(f, "then {}", then)?;
                if let Some(el) = els {
                    writeln!(f, "else {}", el)?;
                }
            }
        }
        Ok(())
    }
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
            Self::Neg => write!(f, "-"),
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

    Or,
    And
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
            BinaryOperator::Or => "||",
            BinaryOperator::And => "&&",

        };
        write!(f, "{}", op)
    }
}

pub struct AstFactory {
    //head: Node,
    current: usize,
    tokens: VecDeque<Token>,
}

impl AstFactory {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens: input.into(),
        }
    }
    pub fn parse_statements(&mut self) -> anyhow::Result<Vec<Statement>> {
        let mut out: Vec<Statement> = Vec::new();
        while self.current < self.tokens.len() {
            let node = match self.parse_statement() {
                Ok(expr) => expr,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(65);
                }
            };
            out.push(node);
        }
        Ok(out)
    }
    pub fn parse_statement(&mut self) -> anyhow::Result<Statement> {
        let out = match self.tokens[self.current].token_type {
            TokenType::Print => {
                self.current += 1;
                let value = self.parse_assignment()?;
                Ok(Statement::Print(value))
            },
            TokenType::Var => {
                self.current += 1;
                let identifier = self.parse_number()?;
                if let Node::Identifier(name, _) = identifier {
                    match self.tokens[self.current].token_type {
                        TokenType::SemiColon => {
                            let pos = self.tokens[self.current].position.clone();
                            let expr = Node::Litteral(Litteral::Nil, pos);
                            self.current += 1;
                            Ok(Statement::VarDecl(name, expr))
                        },
                        TokenType::Equal => {
                            self.current += 1;
                            let expr = self.parse_assignment()?;
                            Ok(Statement::VarDecl(name, expr)) 
                        },
                        _ => {
                            eprintln!("Expected = or ; after variable declearation!");
                            std::process::exit(70);
                        }
                    }
                } else {
                    Err(anyhow!("Expected identifier got {}", identifier))
                }
            }
            TokenType::LeftBrace => {
                let mut statements: Vec<Statement> = Vec::new();
                self.current += 1;
                while self.current < self.tokens.len() {
                    match self.tokens[self.current].token_type {
                        TokenType::RightBrace => {
                            self.current += 1;
                            break;
                        },
                        TokenType::SemiColon => self.current += 1,
                        _ => statements.push(self.parse_statement()?)
                    }
                    if self.current == self.tokens.len() {
                        eprintln!("[line {}] Error at end: Expect '}}'.", 
                            self.tokens[self.current - 1].position.line());
                        std::process::exit(65);
                    }
                }
                Ok(Statement::Block(statements))
            },
            TokenType::If => {
                self.current += 1;
                match self.tokens[self.current].token_type {
                    TokenType::LeftParen => {},
                    _ => {
                        eprintln!("Expected ( after if!");
                        std::process::exit(65);
                    }
                }
                self.current += 1;
                let condition = self.parse_assignment()?;
                self.current += 1;
                let statement = Box::new(self.parse_statement()?); 
                let else_stmnt = if self.current < self.tokens.len() {
                    match self.tokens[self.current].token_type {
                        TokenType::Else => {
                            self.current += 1;
                            Some(Box::new(self.parse_statement()?))
                        },
                        ref t => {
                            None
                        }
                    }
                } else { None };

                Ok(Statement::If(condition, statement, else_stmnt))
            }
            _ => {
                let value = self.parse_assignment()?;
                Ok(Statement::Expression(value))
            }
        };

        match self.tokens.get(self.current) {
            Some(Token {
                position: _,
                raw: _,
                token_type: TokenType::SemiColon
            })=> {
                self.current += 1;
            }
            _ => (),
        }
        
        out
    }
    fn parse_assignment(&mut self) -> anyhow::Result<Node> {
        let identifier: Node = self.parse_or()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::Equal => {
                    if let Node::Identifier(name, pos) = identifier {
                        self.current += 1;
                        let value = self.parse_assignment()?;
                        let position = Position::range(
                            pos,
                            value.position()
                        );
                        let node = Node::Assignment(name, Box::new(value), position); 
                        return Ok(node);
                    }
                },
                _ => break
            }
        }
        Ok(identifier)
    }

    pub fn parse_or(&mut self) -> anyhow::Result<Node> {
        let mut node = self.parse_and()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::Or => {
                    let op = self.tokens[self.current].clone();
                    self.current += 1;
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_and()?);
                    let position = Position::range(node.position(), right.position());
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: op.try_into()?,
                        position
                    };
                },
                _ => break 
            }
        }
        Ok(node)
    }

    pub fn parse_and(&mut self) -> anyhow::Result<Node> {
        let mut node = self.parse_equality()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::And => {
                    let op = self.tokens[self.current].clone();
                    self.current += 1;
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_equality()?);
                    let position = Position::range(node.position(), right.position());
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: op.try_into()?,
                        position
                    };
                },
                _ => break 
            }
        }
        Ok(node)
    }
    pub fn parse_equality(&mut self) -> anyhow::Result<Node> {
        let mut node: Node = self.parse_term()?;
        while self.current < self.tokens.len() {
            match self.tokens[self.current].token_type {
                TokenType::EqualEqual
                | TokenType::GreaterEqual
                | TokenType::LessEqual
                | TokenType::BangEqual
                | TokenType::Greater
                | TokenType::Less => {
                    let op = self.tokens[self.current].clone();
                    self.current += 1;
                    if self.current >= self.tokens.len() {
                        break;
                    }
                    let right = Box::new(self.parse_term()?);
                    let position = Position::range(node.position(), right.position());
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: op.try_into()?,
                        position
                    };
                }
                _ => break,
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
                    let position = Position::range(node.position(), right.position());
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: op.try_into()?,
                        position
                    };
                }
                _ => break,
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
                    let position = Position::range(node.position(), right.position());
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: op.try_into()?,
                        position,
                    };
                }
                _ => break,
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
                    let position = Position::range(node.position(), right.position());
                    node = Node::Binary {
                        left: Box::new(node),
                        right,
                        operator: op.try_into()?,
                        position
                    };
                }
                _ => break,
            }
        }
        Ok(node)
    }

    fn parse_primary(&mut self) -> anyhow::Result<Node> {
        if self.current >= self.tokens.len() {
            return Err(anyhow!("Out of bounds access in parse_primary"));
        }
        match self.tokens[self.current].token_type.clone() {
            TokenType::LeftParen => self.parse_paren(),
            TokenType::Bang | TokenType::Minus => self.parse_unary(),
            _ => self.parse_number(),
        }
    }

    fn parse_unary(&mut self) -> anyhow::Result<Node> {
        let op = self.tokens[self.current].clone();
        self.current += 1;
        let node: Node = self.parse_primary()?;
        let position = Position::range(op.clone().position, node.position());
        let unary = Node::Unary(op.try_into()?, Box::new(node), position);
        Ok(unary)
    }

    fn parse_paren(&mut self) -> anyhow::Result<Node> {
        let mut open_p = 0;
        let mut private_tokens: VecDeque<Token> = VecDeque::new();

        match self.tokens[self.current].token_type {
            TokenType::LeftParen => {}
            _ => {
                return self.parse_number();
            }
        };
        self.current += 1;
        open_p += 1;
        while self.current < self.tokens.len() && open_p != 0 {
            match self.tokens[self.current].token_type.clone() {
                TokenType::LeftParen => open_p += 1,
                TokenType::RightParen => open_p -= 1,
                _x => (), //println!("{:?}", x)
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
        let node = parser.parse_assignment()?;
        Ok(Node::Parenthesis(Box::new(node)))
    }

    fn parse_number(&mut self) -> anyhow::Result<Node> {
        if self.current >= self.tokens.len() {
            return Err(anyhow!("Out of bounds access in parse_number"));
        }
        let position = self.tokens[self.current].position.clone();
        match &self.tokens[self.current].token_type {
            TokenType::Number(x) => {
                let number = *x;
                self.current += 1;
                Ok(Node::Litteral(Litteral::Number(number as f64), position))
            }
            TokenType::True => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::Boolean(true), position))
            }
            TokenType::False => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::Boolean(false), position))
            }
            TokenType::Nil => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::Nil, position))
            }
            TokenType::StringLitteral(s) => {
                self.current += 1;
                Ok(Node::Litteral(Litteral::String(s.clone()), position))
            },
            TokenType::Identifier(i) => {
                self.current += 1;
                Ok(Node::Identifier(i.clone(), position)) 
            },
            _ => Err(anyhow!(
                "[line {}] Error at '{}': Expect expression.",
                &self.tokens[self.current].position.line(),
                &self.tokens[self.current].raw
            )),
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
            TokenType::Or => Ok(BinaryOperator::Or),
            TokenType::And => Ok(BinaryOperator::And),
            _ => Err(anyhow!("Cant convert Token {} to operator", token)),
        }
    }
}

impl TryFrom<Token> for UnaryOperator {
    type Error = anyhow::Error;
    fn try_from(token: Token) -> anyhow::Result<UnaryOperator> {
        match token.token_type {
            TokenType::Bang => Ok(UnaryOperator::Not),
            TokenType::Minus => Ok(UnaryOperator::Neg),
            _ => Err(anyhow!("Cant convert Token {} to operator", token)),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #![allow(unused)]
        match self {
            Node::Unary(op, e, _) => write!(f, "({} {})", op, e),
            Node::Litteral(l, _) => write!(f, "{}", l),
            Node::Binary {
                left,
                right,
                operator,
                position,
            } => write!(f, "({} {} {})", operator, left, right),
            Node::Parenthesis(e) => write!(f, "(group {})", e),
            Node::Identifier(i, _) => write!(f, "_{}", i),
            Node::Assignment(i, v, _) => write!(f, "{} = {}", i, v)
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #![allow(unused)]
        match self {
            Node::Unary(op, e, _) => write!(f, "({} {:?})", op, e),
            Node::Litteral(l, _) => write!(f, "{:?}", l),
            Node::Binary {
                left,
                right,
                operator,
                position,
            } => write!(f, "({} {:?} {:?})", operator, left, right),
            Node::Parenthesis(e) => write!(f, "(group {:?})", e),
            Node::Identifier(i, _) => write!(f, "_{}", i),
            Node::Assignment(i, v, _) => write!(f, "{} = {}", i, v)
        }
    }
}
