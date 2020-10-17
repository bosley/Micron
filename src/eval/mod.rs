use std::convert::TryFrom;

use crate::ast;
use crate::env::Environment;
use crate::env::Object;

pub mod error;
use error::EvalError;

pub struct Eval <'a> {

    env: &'a mut Environment,
    calculation_stack: Vec<Object>
}

impl <'a> Eval <'a> {

    /// Create a new evaluator
    pub fn new(env: &'a mut Environment) -> Self {
        Self {
            env: env,
            calculation_stack: Vec::new()
        }
    }

    /// Parse Statement
    pub fn evaluate_statement(&mut self, statement: ast::Statement) {

        match statement {

            //  Assignment of a variable
            //
            ast::Statement::Assignment(var, expr) => {

                //println!("Var: {:?} | Expr: {:?}", var, expr);

                // Evaluate the expression
                if let Err(e) = self.evaluate_expression(*expr) {
                    println!("Error: {:?}", e);
                }

                // Get the result off the stack
                let result = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { 
                        panic!("Error: {:?} ", EvalError::StackError); 
                    }
                };

                // println!("Result : {:?}", result);

                self.env.set_variable(&var, result);
            }

            //  Bare expression
            //
            ast::Statement::BareExpression(expr) => {

                // Evaluate the expression
                if let Err(e) = self.evaluate_expression(*expr) {
                    println!("Error: {:?}", e);
                }

                // Get the result off the stack
                let result = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { 
                        panic!("Error: {:?} ", EvalError::StackError); 
                    }
                };

                println!("{:?}", result);
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
                match self.env.get_variable(&var) {

                    Some(v) => {

                        // Place the item in the stack for computation
                        self.calculation_stack.push(
                            v.clone()
                        );
                    }

                    None => {

                        // No item by that variable name found, send the error
                        return Err(EvalError::UnknownVariable(var));
                    }
                }
            }

            //  Operation
            //
            ast::Expr::Op(lhs, op, rhs) => {

                // Evaluate the rhs
                if let Err(e) = self.evaluate_expression(*rhs) {
                    return Err(e);
                }

                // Evaluate the lhs
                if let Err(e) = self.evaluate_expression(*lhs) {
                    return Err(e);
                }

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

    /// Perform an operation on an integer type
    /// Panics on "pow" if rhs > u32::MAX
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
            ast::Opcode::Lte => {
                Object::Integer((lhs <= rhs) as i64)
            }

            ast::Opcode::Gte => {
                Object::Integer((lhs >= rhs) as i64)
            }

            ast::Opcode::Lt => {
                Object::Integer((lhs < rhs) as i64)
            }

            ast::Opcode::Gt => {
                Object::Integer((lhs > rhs) as i64)
            }

            ast::Opcode::Equal => {
                Object::Integer((lhs == rhs) as i64)
            }

            ast::Opcode::Ne => {
                Object::Integer((lhs != rhs) as i64)
            }

            ast::Opcode::Pow => {

                // Rust pow for i64 requires a u32 so we attempt to convert it to a u32
                // if it fails a PANIC!
                let rhs_converted = match u32::try_from(rhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert {} into u32 for \"pow\": {}", rhs, e);
                    }
                };

                Object::Integer(lhs.pow(rhs_converted))
            }

            ast::Opcode::Mod => {
                Object::Integer(lhs % rhs)
            }

            ast::Opcode::Lsh => {
                Object::Integer(lhs << rhs)
            }

            ast::Opcode::Rsh => {
                Object::Integer(lhs >> rhs)
            }

            ast::Opcode::BwXor => {
                Object::Integer(lhs ^ rhs)
            }

            ast::Opcode::BwOr => {
                Object::Integer(lhs << rhs)
            }

            ast::Opcode::BwAnd => {
                Object::Integer(lhs & rhs)
            }

            ast::Opcode::Or => {

                if lhs > 0 || rhs > 0{
                    return Object::Integer(1);
                }
                return Object::Integer(0);
            }

            ast::Opcode::And => {
                
                if lhs > 0 && rhs > 0{
                    return Object::Integer(1);
                }
                return Object::Integer(0);
            }

        }
    }
}