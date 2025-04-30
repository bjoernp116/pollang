
use crate::parser::{BinaryOperator, Litteral, Node, UnaryOperator};
use anyhow::anyhow;



impl Node {
    pub fn evaluate(&mut self) -> anyhow::Result<Node> {
        match self {
            Self::Binary { left, right, operator } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                if let (Node::Litteral(l), Node::Litteral(r)) = (left, right) {
                    let lit = operator.eval(l, r)?;
                    Ok(Node::Litteral(lit))
                } else {
                    unreachable!();
                }
            },
            Self::Parenthesis(node) => {
                node.evaluate()
            },
            Self::Unary(op, node) => {
                let node = node.evaluate()?;
                if let Node::Litteral(l) = node {
                    let lit = op.eval(l)?;
                    Ok(Node::Litteral(lit))
                } else {
                    unreachable!();
                }
            },
            Self::Litteral(lit) => {
                Ok(Self::Litteral(lit.clone()))
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
            (String(_), Add, Number(_)) |
            (Number(_), Add, String(_)) => Err(anyhow!("Operands must be two numbers or two strings")),
            (_, Add | Sub | Mul | Div | Pow, _) => Err(anyhow!("Operands must be numbers")),
            (_, Eq | NEq | LEq | GEq | L | G, _) => Err(anyhow!("Operands must be numbers")),
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
            _ => Err(anyhow!("cant calculate {} {}", self, lit.clone()))
        }
    }
}
