
use crate::ast;
use crate::env::Environment;
use crate::env::Object;

pub mod error;
use error::EvalError;

pub struct Eval <'a> {

    env: &'a Environment,

    calculation_stack: Vec<Object>
}


impl <'a> Eval <'a> {

    pub fn new(env: &'a Environment) -> Self {
        Self {
            env: env,
            calculation_stack: Vec::new()
        }
    }

    /// Parse Statement
    pub fn evaluate_statement(&mut self, statement: ast::Statement) {

        match statement {

            ast::Statement::Assignment(var, expr) => {

                println!("Var: {:?} | Expr: {:?}", var, expr);

                if let Err(e) = self.evaluate_expression(*expr) {

                    println!("Error: {:?}", e);
                }

                let result = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { 
                        panic!("Error: {:?} ", EvalError::StackError); 
                    }
                };

                println!("Result : {:?}", result);
                // Store var with expression result in current scope 
                // variable map
            }
        }
    }

    /// Perform an operation on an integer type
    fn op_integer(&self, lhs: i64, rhs: i64, op: ast::Opcode) -> Object {

        return match op {
            ast::Opcode::Mul => {
                //println!("mul");
                Object::Integer(lhs * rhs)
            }
            ast::Opcode::Div => {
                //println!("div");
                Object::Integer(lhs / rhs)
            }
            ast::Opcode::Add => {
                //println!("add");
                Object::Integer(lhs + rhs)
            }
            ast::Opcode::Sub => {
                //println!("sub");
                Object::Integer(lhs - rhs)
            }
        }
    }

    /// Perform operation on env objects
    fn perform_operation(&self, lhs: Object, rhs: Object, op: ast::Opcode) -> Result<Object, EvalError> {

        match lhs {

            Object::Integer(i_lhs) => {

                match rhs {

                    Object::Integer(i_rhs) => {

                        return Ok(self.op_integer(i_lhs, i_rhs, op));
                    }
                }
            }

            // Float type

        }
    }

    /// Evaluate an expression
    fn evaluate_expression(&mut self, expr: ast::Expr) -> Result<(), EvalError> {

        match expr {

            //  Number found in expression
            //
            ast::Expr::Number(item) => {

                self.calculation_stack.push(
                    Object::Integer(item)
                );

                //println!(" > {}", item);
            }

            ast::Expr::Variable(var) => {

                // Load Var here

                // Place in env::Object::Integer()

                // Push to stack

                panic!("Variable loading not yet complete");

            }

            //  Operation
            //
            ast::Expr::Op(lhs, op, rhs) => {

                // Evaluate the rhs
                self.evaluate_expression(*rhs);

                // Evaluate the lhs
                self.evaluate_expression(*lhs);

                // Get the lhs operand
                let operation_lhs = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { return Err(EvalError::StackError); }
                };

                // Get the rhs operand
                let operation_rhs = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { return Err(EvalError::StackError); }
                };
                
                //  Perform the operation and push the result to the stack
                match self.perform_operation(operation_lhs, operation_rhs, op) {

                    Ok(v)  => { self.calculation_stack.push(v); }
                    Err(e) => { return Err(e); }
                }
            }
        }

        return Ok(())
    }
}