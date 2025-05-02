use std::{collections::HashMap, fmt::Display};
use anyhow::anyhow;

use crate::parser::{AstFactory, Litteral, Node, Statement};


#[derive(Clone)]
pub struct Variables(pub HashMap<String, Litteral>);

impl Variables {
    pub fn get(&self, ident: &String) -> anyhow::Result<Litteral> {
        if let Some(var) = self.0.get(ident) {
            Ok(var.clone())
        } else {
            Err(anyhow!("Undefined variable '{}'.", ident))
        }
    }
    pub fn contains_var(&self, name: &String) -> bool {
        self.0.contains_key(name)
    }
    pub fn assign(&mut self, ident: &String, value: Litteral) {
        if let Some(v) = self.0.get_mut(ident) {
            *v = value;
        }
    }
    pub fn define(&mut self, ident: String, value: Litteral) {
        self.0.insert(ident, value);
    }
}

#[derive(Clone)]
pub struct Enviornment {
    pub variables: Variables,
    pub statements: Vec<Statement>
}

impl Enviornment {
    
    pub fn run(&mut self) -> anyhow::Result<()> {
        for statement in self.statements.clone() {
            match statement {
                Statement::Expression(expr) => {
                    (&mut expr.clone()).evaluate(&mut self.variables)?;
                },
                Statement::Print(expr) => {
                    let expr = (&mut expr.clone()).evaluate(&mut self.variables)?;
                    println!("{}", expr);
                },
                Statement::VarDecl(ident, expr) => {
                    let expr = (&mut expr.clone()).evaluate(&mut self.variables)?;
                    if let Node::Litteral(lit, _) = expr {
                        self.variables.define(ident.clone(), lit);
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
        let variables = Variables(HashMap::new());
        Ok(Self { statements, variables })
    }
}

impl Default for Litteral {
    fn default() -> Self {
        Litteral::Nil
    }
}

impl Display for Enviornment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Variables:")?; 
        for (name, value) in self.variables.0.clone() {
            writeln!(f, "\t{} = {}", name, value)?; 
        }
        writeln!(f, "Statements:")?;
        for statement in self.statements.clone() {
            writeln!(f, "\n{}", statement)?;
        }
        Ok(())
    }
}




