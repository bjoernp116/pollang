use std::collections::HashMap;
use anyhow::anyhow;

use crate::parser::{AstFactory, Litteral, Node, Statement};



#[derive(Clone)]
pub struct Enviornment {
    variables: HashMap<String, Litteral>,
    statements: Vec<Statement>
}

impl Enviornment {
    
    pub fn define(&mut self, ident: String, value: Litteral) {
        self.variables.insert(ident, value);
    }
    pub fn get(&self, ident: &String) -> anyhow::Result<Litteral> {
        if let Some(var) = self.variables.get(ident) {
            Ok(var.clone())
        } else {
            Err(anyhow!("Cant find variable"))
        }
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        for statement in self.statements.clone() {
            match statement {
                Statement::Expression(expr) => {
                    (&mut expr.clone()).evaluate(&mut self.clone())?;
                },
                Statement::Print(expr) => {
                    let expr = (&mut expr.clone()).evaluate(&mut self.clone())?;
                    println!("{}", expr);
                },
                Statement::VarDecl(ident, expr) => {
                    let expr = (&mut expr.clone()).evaluate(&mut self.clone())?;
                    if let Node::Litteral(lit, _) = expr {
                        self.define(ident.clone(), lit);
                    }
                }
            }
        }
        Ok(())
    }
}

impl TryFrom<&mut AstFactory> for Enviornment {
    type Error = anyhow::Error;
    fn try_from(ast: &mut AstFactory) -> Result<Self, Self::Error> {
        let statements: Vec<Statement> = match ast.parse_statements() {
            Ok(stmts) => stmts,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(65);
            }
        };
        let variables = HashMap::new();
        Ok(Self { statements, variables })
    }
}

impl Default for Litteral {
    fn default() -> Self {
        Litteral::Nil
    }
}
