use std::collections::HashMap;

mod object;
use object::Object;

use crate::ast;

struct Scope {

    variables: HashMap<String, Object>
}

pub struct Environment {

    scopes: Vec<Scope>
}

impl Environment {

    /// Create a new environment
    pub fn new() -> Self {

        // Create the env
        let mut e = Environment {
            scopes: Vec::new()
        };

        e.scopes.push(Scope{
            variables: HashMap::new()
        });

        // Return env
        e
    }


    /// Parse Statement
    pub fn evaluate_statement(&mut self, statement: ast::Statement) {

        match statement {

            ast::Statement::Assignment(var, expr) => {

                println!("Var: {:?} | Expr: {:?}", var, expr);
                
                // Ensure var is unique

                // Evaluate expression

                // Store var with expression result in current scope 
                // variable map
            }
        }
    }
}