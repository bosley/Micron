
extern crate micron_ast;

use std::{ cell::RefCell, rc::Rc };

use micron_ast::{ Statement, Expr, VariableType, DictAccessType};
use crate::types::{ Dictionary, RecordData };
use crate::error::ExecutionError;

/// The Micron Engine 
#[derive(Debug, Clone)]
pub struct Engine {

    /// Stored data
    scopes: Vec<Dictionary>,
    op_stack: Vec<Rc<RefCell<RecordData>>>
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
    fn get_record(&self, key: &String) -> Option<Rc<RefCell<RecordData>>> {
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

    fn get_record_by_var_type(&self, var_type: VariableType) -> Option<Rc<RefCell<RecordData>>> {

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
                                    
                                    /*
                                    
                                        Here we need to get each ['item'] block from the list and load that current 
                                        variable until we find the item we're looking for
                                    */

                                    panic!("Not done");
                                    
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


                match var_type {
                    VariableType::Singular(var_name) => {

                        match self.execute_expression(*expr) {

                            Some(e) => { return Some(e); }

                            None => {

                                let value = match self.op_stack.pop() {
                                    None => {
                                        return Some(ExecutionError::StackError);
                                    }

                                    Some(val) => { val }
                                };

                                self.set_record(&var_name, value.borrow().clone());
                            }
                        }
                    }

                    VariableType::Nested(var, accessors) => {

                        panic!("Nested variable assignment is not yet supported");
                    }
                }


                }

            Statement::BareExpression(expr) => {

                // Execute the expression
                match self.execute_expression(*expr) {

                    Some(e) => { return Some(e); }

                    None => {

                        // Get the resulting expression
                        let value = match self.op_stack.pop() {
                            None => {
                                return Some(ExecutionError::StackError);
                            }

                            Some(val) => { val.borrow().clone() }
                        };

                        // Print the value for now
                        println!("{:?}", value);
                    }
                }
            }
        }

        None
    }

    fn execute_expression(&mut self, expression: Expr) -> Option<ExecutionError> {

        match expression {


            Expr::Number(i) => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::Integer(i))));
                return None;
            }

            Expr::Real(f) => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::Float(f))));
                return None;
            }

            Expr::String(s) => {
                self.op_stack.push(Rc::new(RefCell::new(RecordData::String(s))));
                return None;
            }

            Expr::Variable(v) => {

                // Get the variable from memory
                match self.get_record_by_var_type(v) {
                    Some(var) => {

                        // If it exists stack it
                        self.op_stack.push(var);
                    }
                    None => {
                        
                        // Otherwise its an error
                        return Some(ExecutionError::UnknownVariable)
                    }
                };

                // Get on out!
                return None;
            }

            Expr::UnaryOp(op_expr, o) => {

                panic!("Unary op not yet complete!");
            }

            Expr::Op(lhs_expr, op, rhs_expr) => {

                panic!("Op not yet complete!");
            }

            Expr::Modifier(var, modifier) => {

                panic!("Modifier not yet complete!");
            }

            Expr::Dict(dict_entries) => {

                panic!("Dict Entries not yet complete!");
            }

            Expr::Access(access_expr, accessor, method) => {

                panic!("Access not yet complete!");
            }
        }


        None
    }

}