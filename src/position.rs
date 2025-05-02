use crate::parser::Node;



#[derive(Clone)]
pub struct Position {
    pub from: (usize, usize),
    pub to: (usize, usize)
}

impl Position {
    pub fn new(line1: usize, col1: usize, line2: usize, col2: usize) -> Position {
        Self {
            from: (line1, col1),
            to: (line2, col2)
        }
    }
    pub fn range(from: Self, to: Self) -> Self {
        Self {
            from: from.from,
            to: to.to
        }
    }
    pub fn line(&self) -> usize {
        self.to.0
    }
}

impl Node {
    pub fn position(&self) -> Position {
        #![allow(unused)]
        match self {
            Self::Binary { left, operator, right, position } => position.clone(),
            Self::Unary(_, _, position) => position.clone(),
            Self::Litteral(_, position) => position.clone(),
            Self::Parenthesis(child) => child.position()
        }
    }
}
