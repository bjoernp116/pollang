use std::{cell::{RefCell, RefMut}, collections::HashMap, fmt::Display, rc::{Rc, Weak}, sync::Arc};
use anyhow::anyhow;

use crate::parser::{AstFactory, Litteral, Node, Statement};


#[derive(Clone)]
pub struct Variables(
    pub HashMap<String, Litteral>, 
    pub Option<Weak<RefCell<Environment>>>
);

impl Variables {
    pub fn get(&self, ident: &String) -> anyhow::Result<Litteral> {
        if let Some(var) = self.0.get(ident) {
            Ok(var.clone())
        } else {
            if let Some(weak_parent) = self.1.clone() {
                let upgraded = weak_parent.upgrade();
                match upgraded {
                    Some(parent_rc) => {
                        return parent_rc.borrow().variables.get(ident);
                    },
                    None => ()
                }
            }
            Err(anyhow!("Undefined variable '{}'.", ident))
        }
    }
    pub fn contains_var(&self, name: &String) -> bool {
        self.0.contains_key(name)
    }
    pub fn assign(&mut self, ident: &String, value: Litteral) -> bool {
        if let Some(v) = self.0.get_mut(ident) {
            *v = value;
            true
        } else {
            if let Some(weak_parent) = self.1.clone() {
                let upgraded = weak_parent.upgrade(); 
                match upgraded {
                    Some(parent_rc) => {
                        let mut parent_ref_mut = parent_rc.borrow_mut();
                        parent_ref_mut.variables.assign(ident, value)
                    }, 
                    None => {
                        eprintln!("Parent has been dropped!");
                        false
                    }
                }
            } else {
                false
            }
        }
    }
    pub fn define(&mut self, ident: String, value: Litteral) {
        self.0.insert(ident, value);
    }
}

#[derive(Clone)]
pub struct Environment {
    pub variables: Variables,
    pub statements: Vec<Statement>,
}

impl Environment {
    pub fn run(self_rc: &Rc<RefCell<Environment>>) -> anyhow::Result<()> {
        let mut env_mut = self_rc.borrow_mut();
        let statements = env_mut.statements.clone();

        for statement in statements.iter() {
            match statement {
                Statement::Expression(expr) => {
                    let variables: &mut Variables = &mut env_mut.variables;
                    println!("Expression:\n\tvars_before:\n{}", variables);
                    let _evaluated: Node = expr.clone().evaluate(variables)?;
                    println!("\tevaluated:{}", _evaluated);
                    println!("\tvars_after:\n{}", variables);
                },
                Statement::Print(expr) => {
                    let variables: &mut Variables = &mut env_mut.variables;
                    println!("Print:\n\tvars_before:\n{}", variables);
                    let result: Node = expr.clone().evaluate(variables)?;
                    println!("\tevaluated:{}", result);
                    println!("\tvars_after:\n{}", variables);
                    println!("{}", result);
                },
                Statement::VarDecl(ident, expr) => {
                    let variables: &mut Variables = &mut env_mut.variables;
                    println!("VarDecl:\n\tvars_before:\n{}", variables);
                    let result: Node = expr.clone().evaluate(variables)?;

                    if let Node::Litteral(lit, _) = result.clone() {
                        variables.define(ident.clone(), lit);
                    }
                    println!("\tevaluated:{}", result);
                    println!("\tvars_after:\n{}", variables);
                },
                Statement::Block(block_env_rc) => {
                    Environment::run(block_env_rc)?;
                }
            }
        }
        Ok(())
    }
    pub fn connect_blocks(self_rc: &Rc<RefCell<Environment>>) -> () {
        let statements = self_rc.borrow().statements.clone();
        for stmt in statements {
            match stmt {
                Statement::Block(ref block) => {
                    let parent_weak: Weak<RefCell<Environment>> = Rc::downgrade(self_rc);
                    block.borrow_mut().variables.1 = Some(parent_weak);
                    Environment::connect_blocks(block);
                },
                _ => ()
            }
        }
    }
}

impl From<Vec<Statement>> for Environment {
    fn from(statements: Vec<Statement>) -> Self {
        Self {
            variables: Variables(HashMap::new(), None),
            statements,
        }
    }
}

impl TryFrom<&mut AstFactory> for Environment {
    type Error = anyhow::Error;
    fn try_from(ast: &mut AstFactory) -> Result<Self, Self::Error> {
        let statements: Vec<Statement> = match ast.parse_statements() {
            Ok(stmts) => stmts,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(65);
            }
        };
        let variables = Variables(HashMap::new(), None);
        Ok(Self { statements, variables })
    }
}

impl Default for Litteral {
    fn default() -> Self {
        Litteral::Nil
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Variables:")?; 
        for (name, value) in self.variables.0.clone() {
            writeln!(f, "{} = {}", name, value)?; 
        }
        writeln!(f, "Statements:")?;
        for statement in self.statements.clone() {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}





impl Display for Variables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.0.iter() {
            writeln!(f, "\t\t{}={}", k, v)?;
        }
        Ok(())
    } 
}
