
extern crate micron_ast;

use micron_ast::{ Statement, Expr, VariableType, DictAccessType};
use crate::types::{ Dictionary, Record, RecordData };
use crate::error::ExecutionError;

/// The Micron Engine 
#[derive(Debug, Clone)]
pub struct Engine {

    /// Stored data
    scopes: Vec<Dictionary>,
    op_stack: Vec<Record>
}

impl Engine {

    /// Create a new engine
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            op_stack: Vec::new()
        }
    }

    /// Add a new scope to the scope list
    fn new_scope(&mut self) {
        self.scopes.push(Dictionary::new());
    }

    /// Remove a scope from the scope list. 
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Get a record - Traverses scopes in reverse in an attempt
    /// to find the requested record. The first one found will be returned
    fn get_record(&self, key: &String) -> Option<Record> {
        for scope in self.scopes.iter().rev() {
            match scope.get(key) {
                Some(record) => { return Some(record); }
                None         => { return None; }
            }
        }
        None
    }

    /// Get the current scope
    fn current_scope (&mut self) -> &mut Dictionary {

        // If there is no scope for some reason
        if self.scopes.len() == 0 {

            // add  a new scope
            self.new_scope();
        }

        let n = self.scopes.len();

        // Return a mut reference to the top scope
        &mut self.scopes[n-1]
    }

    /// Set a record
    fn set_record(&mut self, key: &String, record: RecordData) {
        
        self.current_scope().set(key, record);
    }

    fn get_record_by_var_type(&self, var_type: VariableType) -> Option<Record> {

        match var_type {
            VariableType::Singular(var_name) => {

                return self.get_record(&var_name);
            }

            VariableType::Nested(var_name, accessor) => {
                for item in accessor.iter() {
                    match item {
                        DictAccessType::RawValue(string_key) => {
    
                            let value = match self.get_record(&var_name) {
    
                                Some(existing_variable) => {
                                    
                                    if accessor.len() == 0 {

                                        // We have the final item
                                    } 
                                    else 
                                    {
                                        // We aren't at the end, so we clone the accessors
                                        // pop the front, and recurse
                                        let mut next_search_vector = accessor.clone();
                                        next_search_vector.pop_front();
                                        return self.get_record_by_var_type(VariableType::Nested(var_name, next_search_vector));
                                    }
                                    
                                }
    
                                None => { return None; }
                            };

                            return None;
    
                        }
    
                        DictAccessType::Variable(var_key) => {
                            return None;
                        }
                    }
                }
            }
        }

        None
    }

    /// Execute an AST statement
    pub fn execute_statement(&mut self, statement: Statement) -> Option<ExecutionError> {

        /*

            Match the statement and do what it asks
        
        */
        match statement {

            Statement::Assignment(var_type, expr) => {

                panic!("{:?} | {:?}", var_type, expr);



                }

            Statement::BareExpression(expr) => {

                panic!("{:?}", expr);
            }
        }

        None
    }

    fn execute_expression(expresson: Expr) -> Option<ExecutionError> {

        /*
        
            Recurse through an expression
        
        */

        None
    }

}