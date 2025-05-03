use std::{collections::HashMap, fmt::Display};
use anyhow::anyhow;

use crate::parser::Litteral;


#[derive(Clone)]
pub struct Environment {
    pub variables: HashMap<String, Litteral>, 
    pub parent: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None
        }
    }
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent))
        }
    }
    pub fn get(&self, ident: &String) -> anyhow::Result<Litteral> {
        if let Some(var) = self.variables.get(ident) {
            Ok(var.clone())
        } else {
            if let Some(parent) = self.parent.clone() {
                return parent.get(ident);
            }
            Err(anyhow!("Undefined variable '{}'.", ident))
        }
    }
    pub fn assign(&mut self, ident: String, value: Litteral) -> anyhow::Result<()> {
        if self.variables.contains_key(&ident) {
            self.define(ident, value);
            Ok(())
        } else {
            match &mut self.parent {
                Some(parent) => parent.assign(ident, value),
                None => Err(anyhow!("attempt to assign to undefined variable: {}", ident))
            }
        }
    }
    pub fn define(&mut self, ident: String, value: Litteral) {
        self.variables.insert(ident, value);
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
        for (name, value) in self.variables.clone() {
            writeln!(f, "{} = {}", name, value)?; 
        }
        Ok(())
    }
}



