
use crate::{environment::Environment, parser::{BinaryOperator, Litteral, Node, Statement, UnaryOperator}, position::Position};
use anyhow::anyhow;

#[allow(unused)]
pub struct Interpreter {
    environment: Environment,
    global: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            global: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Statement>) -> anyhow::Result<()> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }
    pub fn execute(&mut self, statement: Statement) -> anyhow::Result<()> {
        match statement {
            Statement::Expression(expr) => {
                self.evaluate_expr(&expr)?;
            },
            Statement::Print(expr) => {
                let expr = self.evaluate_expr(&expr)?;
                println!("{}", expr);
            },
            Statement::VarDecl(ident, expr) => {
                let expr = self.evaluate_expr(&expr)?;
                if let Node::Litteral(lit, _) = expr {
                    self.environment.define(ident.clone(), lit);
                }
            },
            Statement::Block(statements) => {
                self.environment = Environment::with_parent(self.environment.clone());
                
                for stmnt in statements {
                    self.execute(stmnt)?;
                }

                if let Some(parent) = self.environment.parent.clone() {
                    self.environment = *parent;
                } else {
                    unreachable!();
                }
            },
            Statement::If(condition, then_stmt, else_stmt) => {
                let result = self.evaluate_expr(&condition)?;
                if let Node::Litteral(litteral, _) = result {
                    if litteral.is_truthy() {
                        self.execute(*then_stmt)?;
                    } else {
                        if let Some(stmt) = else_stmt {
                            self.execute(*stmt)?;
                        }
                    }
                } else {
                }
            }
        }
        Ok(())
    } 
    pub fn evaluate_expr(&mut self, expr: &Node) -> anyhow::Result<Node> {
        match expr {
            Node::Binary { 
                left, 
                right, 
                operator: BinaryOperator::Or,
                position
            } => {
                let left = self.evaluate_expr(left)?;
                if let Node::Litteral(l, _) = left.clone() {
                    if l.is_truthy() {
                        return Ok(left);
                    }
                    let right = self.evaluate_expr(right)?;
                    if let Node::Litteral(r, _) = right {
                        return Ok(
                            Node::Litteral(
                                BinaryOperator::Or.eval(l, r)?,
                                position.clone()
                            )
                        );
                    }
                }
                unreachable!()
            },
            Node::Binary { 
                left, 
                right, 
                operator: BinaryOperator::And,
                position
            } => {
                let left = self.evaluate_expr(left)?;
                if let Node::Litteral(l, _) = left.clone() {
                    if !l.is_truthy() {
                        return Ok(left);
                    }
                    let right = self.evaluate_expr(right)?;
                    if let Node::Litteral(r, _) = right {
                        return Ok(
                            Node::Litteral(
                                BinaryOperator::And.eval(l, r)?,
                                position.clone()
                            )
                        );
                    }
                }
                unreachable!()
            },
            Node::Binary { left, right, operator, position } => {
                let left = self.evaluate_expr(left)?;
                let right = self.evaluate_expr(right)?;

                if let (Node::Litteral(l, _), Node::Litteral(r, _)) = (left, right) {
                    match operator.eval(l, r) {
                        Ok(lit) => Ok(Node::Litteral(lit, position.clone())),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(70);
                        }
                    }
                } else {
                    unreachable!();
                }
            },
            Node::Parenthesis(node) => {
                self.evaluate_expr(node)
            },
            Node::Unary(op, node, pos) => {
                let node = self.evaluate_expr(node)?;
                if let Node::Litteral(l, _) = node {
                    match op.eval(l) {
                        Ok(lit) => Ok(Node::Litteral(lit, pos.clone())),
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(70);
                        }
                    }
                } else {
                    unreachable!();
                }
            },
            Node::Litteral(lit, pos) => {
                Ok(Node::Litteral(lit.clone(), pos.clone()))
            },
            Node::Identifier(i, pos) => {
                match self.environment.get(i) {
                    Ok(v) => Ok(Node::Litteral(v, pos.clone())),
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(70);
                    }
                }
            },
            Node::Assignment(i, value, _) => {
                let value = self.evaluate_expr(value)?;
                if let Node::Litteral(lit, _) = value.clone() {
                    self.environment.assign(i.clone(), lit)?;
                    Ok(value)
                } else {
                    eprintln!("Unknown variable type!");
                    std::process::exit(70);
                }
            }
        }
    }
}

impl BinaryOperator {
    fn eval(&self, left: Litteral, right: Litteral) -> anyhow::Result<Litteral> {
        use Litteral::*;
        use BinaryOperator::*;

        match (left.clone(), self, right.clone()) {
            (Number(l), Eq,  Number(r)) => Ok(Boolean(l == r)),
            (Number(l), NEq,  Number(r)) => Ok(Boolean(l != r)),
            (Number(l), LEq,  Number(r)) => Ok(Boolean(l <= r)),
            (Number(l), GEq,  Number(r)) => Ok(Boolean(l >= r)),
            (Number(l), L,  Number(r)) => Ok(Boolean(l < r)),
            (Number(l), G,  Number(r)) => Ok(Boolean(l > r)),

            (Number(l), Add,  Number(r)) => Ok(Number(l + r)),
            (Number(l), Sub,  Number(r)) => Ok(Number(l - r)),
            (Number(l), Mul,  Number(r)) => Ok(Number(l * r)),
            (Number(l), Div,  Number(r)) => Ok(Number(l / r)),
            (Number(l), Pow,  Number(r)) => Ok(Number(l.powf(r))),

            (String(l), Eq, String(r)) => Ok(Boolean(l == r)),
            (String(l), NEq, String(r)) => Ok(Boolean(l != r)),
            (String(l), Add, String(r)) => Ok(String(format!("{}{}", l, r))),

            (Boolean(l), Eq, Boolean(r)) => Ok(Boolean(l == r)),
            (Boolean(l), NEq, Boolean(r)) => Ok(Boolean(l != r)),

            (String(_), Eq, Number(_)) => Ok(Boolean(false)),
            (Number(_), Eq, String(_)) => Ok(Boolean(false)),

            (Boolean(true), Or, _) => Ok(Boolean(true)),
            (Number(l), Or, _) => Ok(Number(l)),
            (String(l), Or, _) => Ok(String(l)),
            (_, Or, Boolean(true)) => Ok(Boolean(true)),
            (_, Or, String(r)) => Ok(String(r)),
            (_, Or, Number(r)) => Ok(Number(r)),
            (_, Or, _) => Ok(Boolean(false)),

            (Boolean(false) | Nil, And, _) => Ok(Boolean(false)),
            (_, And, Boolean(false) | Nil) => Ok(Boolean(false)),
            (Boolean(true), And, Boolean(true)) => Ok(Boolean(true)),
            (l, And, r) if l.is_truthy() && r.is_truthy() => Ok(r),
            (_, And, _) => Ok(Boolean(false)),


            (String(_), Add, Number(_)) |
            (Number(_), Add, String(_)) => Err(anyhow!("Operands must be two numbers or two strings")),
            (_, Add | Sub | Mul | Div | Pow, _) => Err(anyhow!("Operands must be numbers")),
            (_, Eq | NEq | LEq | GEq | L | G, _) => Err(anyhow!("Operands must be numbers")),
        }
    }
}

impl Litteral {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(true) => true,
            Self::Boolean(false) => false,
            Self::Nil => false,
            Self::String(_) => true,
            Self::Number(_) => true,
        }
    }
}

impl UnaryOperator {
    fn eval(&self, lit: Litteral) -> anyhow::Result<Litteral> {
        use Litteral::*;
        use UnaryOperator::*;

        match (self, lit.clone()) {
            (Neg, Number(n)) => Ok(Number(-n)),
            (Not, Boolean(n)) => Ok(Boolean(!n)),
            (Not, Nil) => Ok(Boolean(true)),
            (Not, _) => Ok(Boolean(false)),
            (Neg, _) => Err(anyhow!("Operand must be a number")),
        }
    }
}
