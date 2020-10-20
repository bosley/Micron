

use std::convert::TryFrom;

use rug::{Integer, Float, ops::Pow};

use micron_ast::{ Expr, Opcode, UnaryOpcode, FLOAT_PRECISION, Accessors, MemberMethod};
use micron_environment::{ 
    MicronEnv, 
    EnvError,
    object::Object, 
    types::{
        MInteger,
        MFloat,
        MString,
        FromRug,
        AsMicronType
    },
    type_methods
};

use crate::error::InterpreterError;

pub struct ExpressionCalculator <'a> {

    env: &'a MicronEnv,
    calculation_stack: Vec<Object>
}

impl <'a> ExpressionCalculator <'a> {

    pub fn new(env: &'a MicronEnv) -> Self {

        Self {
            env: env,
            calculation_stack: Vec::new()
        }
    }

    /// Run an expression
    pub fn evaluate_expression(&mut self, expression: Expr) -> Result<Object, InterpreterError> {

        // Attempt to evaluate the expression
        if let Err(e) = self.run_expression(expression) {

            return Err(e);
        }

        // Get the result off the stack
        match self.calculation_stack.pop() {
            Some(n) => { return Ok(n); }
            None => { 
                return Err(InterpreterError::StackError);
            }
        };
    }

    //  Run the expression
    //
    fn run_expression(&mut self, expression: Expr) -> Result<(), InterpreterError> {

        match expression {

            //  Convert and load an integer
            //
            Expr::Number(item) => {

                self.calculation_stack.push(
                    Object::Integer(MInteger::from_rug_integer(item))
                );
            }

            // Convert and load a float
            //
            Expr::Real(item) => {

                self.calculation_stack.push(
                    Object::Float(MFloat::from_rug_float(item))
                );
            }

            // String
            //
            Expr::String(item) => {

                let actual_string = item.as_str().trim_matches('"');

                self.calculation_stack.push(
                    Object::String(MString::new(actual_string.to_string()))
                );
            }

            // Load a variable
            //
            Expr::Variable(var) => {

                match self.env.get_variable(MString::new(var), None) {
                    Ok(v) => {

                        self.calculation_stack.push(
                            v.clone()
                        );
                    }

                    Err(e) => {
                        return Err(InterpreterError::EnvironmentError(e));
                    }
                }
            }

            // Accessor
            //
            Expr::Access(lhs, accessor, method) => {

                // Evaluate the expression lhs
                if let Err(e) = self.run_expression(*lhs) {
                    return Err(e);
                }

                // Get the item to access
                let item = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { return Err(InterpreterError::StackError); }
                };

                //  Perform the operation and push the result to the stack
                match self.perform_access(item, accessor, *method) {
                    Ok(v)  => { self.calculation_stack.push(v); }
                    Err(e) => { return Err(e); }
                }
            }

            // Perform a unary operation
            //
            Expr::UnaryOp(unary_expression, unary_operation) => {

                // Evaluate the expression
                if let Err(e) = self.run_expression(*unary_expression) {
                    return Err(e);
                }

                // Get the operand
                let operand = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { return Err(InterpreterError::StackError); }
                };

                //  Perform the operation and push the result to the stack
                match self.perform_unary_operation(operand, unary_operation) {

                    Ok(v)  => { self.calculation_stack.push(v); }
                    Err(e) => { return Err(e); }
                }

            } 

            //  Perform a standard operation
            //
            Expr::Op(lhs, op, rhs) => {

                // Evaluate the rhs
                if let Err(e) = self.run_expression(*rhs) {
                    return Err(e);
                }

                // Evaluate the lhs
                if let Err(e) = self.run_expression(*lhs) {
                    return Err(e);
                }

                // Get the lhs operand
                let operation_lhs = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { return Err(InterpreterError::StackError); }
                };

                // Get the rhs operand
                let operation_rhs = match self.calculation_stack.pop() {
                    Some(n) => { n }
                    None => { return Err(InterpreterError::StackError); }
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

    //  Perform an access
    //
    fn perform_access(&mut self, item: Object, accessor: Accessors, method: MemberMethod) -> Result<Object, InterpreterError> {

        match accessor {

            Accessors::Dot => {

                match method.method.as_str() {

                    "as_string" => {
                        if method.params.len() > 0 { 
                            return Err(InterpreterError::EnvironmentError(
                                EnvError::InvalidNumberOfParameters(0, method.params.len() as i32)
                            ));
                        }

                        match type_methods::as_string(item) {
                            Ok(obj) => return Ok(obj),
                            Err(e)  => return Err(InterpreterError::EnvironmentError(e))
                        }
                    }

                    "as_int" => {
                        if method.params.len() > 0 { 
                            return Err(InterpreterError::EnvironmentError(
                                EnvError::InvalidNumberOfParameters(0, method.params.len() as i32)
                            ));
                        }
                        match type_methods::as_int(item) {
                            Ok(obj) => return Ok(obj),
                            Err(e)  => return Err(InterpreterError::EnvironmentError(e))
                        }
                    }

                    "as_float" => {
                        if method.params.len() > 0 { 
                            return Err(InterpreterError::EnvironmentError(
                                EnvError::InvalidNumberOfParameters(0, method.params.len() as i32)
                            ));
                        }
                        match type_methods::as_float(item) {
                            Ok(obj) => return Ok(obj),
                            Err(e)  => return Err(InterpreterError::EnvironmentError(e))
                        }
                    }

                    "at" => {

                        if method.params.len() != 1 { 
                            return Err(InterpreterError::EnvironmentError(
                                EnvError::InvalidNumberOfParameters(1, method.params.len() as i32)
                            ));
                        }

                        let exp = *method.params[0].clone();

                        // Evaluate precision
                        if let Err(e) = self.run_expression(exp) {
                            return Err(e);
                        }

                        // Get the lhs operand
                        let location = match self.calculation_stack.pop() {
                            Some(n) => { n }
                            None => { return Err(InterpreterError::StackError); }
                        };

                        match location {

                            Object::Integer(i) => {

                                match type_methods::at_string(item, i) {
                                    Ok(obj) => return Ok(obj),
                                    Err(e)  => return Err(InterpreterError::EnvironmentError(e))
                                }
                            }
                            _ => {
                                return Err(InterpreterError::EnvironmentError(
                                    EnvError::InvalidParameter("at expects parameter type: Integer")
                                ));
                            }
                        }


                    }

                    "with_precision" => {
                        if method.params.len() != 1 { 
                            return Err(InterpreterError::EnvironmentError(
                                EnvError::InvalidNumberOfParameters(1, method.params.len() as i32)
                            ));
                        }

                        let exp = *method.params[0].clone();

                        // Evaluate precision
                        if let Err(e) = self.run_expression(exp) {
                            return Err(e);
                        }

                        // Get the lhs operand
                        let precision = match self.calculation_stack.pop() {
                            Some(n) => { n }
                            None => { return Err(InterpreterError::StackError); }
                        };

                        match precision {

                            Object::Integer(i) => {

                                match type_methods::with_precision(item, i) {
                                    Ok(obj) => return Ok(obj),
                                    Err(e)  => return Err(InterpreterError::EnvironmentError(e))
                                }
                            }
                            _ => {
                                return Err(InterpreterError::EnvironmentError(
                                    EnvError::InvalidParameter("with_precision expects parameter type: Integer")
                                ));
                            }
                        }

                    }

                    _ => {
                        return Err(InterpreterError::EnvironmentError(EnvError::UnknownMethod(method.method)));
                    }
                }
            }
        }
    }

    //  Perform a string operation
    //
    fn perform_string_op(&mut self, lhs: Object, rhs: Object, op: Opcode) -> Result<Object, InterpreterError> {

        match op {

            Opcode::Add => {
                match lhs.clone() {

                    Object::String(s_lhs) => {

                        match rhs.clone() {

                            Object::String(s_rhs) => {

                                return Ok(Object::String(
                                    MString::new(s_lhs.get_value().as_str().to_owned() + s_rhs.get_value().as_str()))
                                );
                            }
                            _ => {
                                return Err(InterpreterError::InvalidStringExpression);
                            }
                        }
                    }
                    _ => {
                        return Err(InterpreterError::InvalidStringExpression);
                    }
                }
            }
            _ => {
                return Err(InterpreterError::InvalidStringExpression);
            }
        }
    }

    //  Perform a unary operation
    //
    fn perform_unary_operation(&self, operand: Object, op: UnaryOpcode) -> Result<Object, InterpreterError> {

        match operand {

            Object::Integer(i_op) => {
                return Ok(self.op_unary_integer(i_op, op));
            }

            Object::Float(f_op) => {

                let f = match f_op.as_micron_integer() {
                    Ok(v)  => { v }
                    Err(e) => { return Err(InterpreterError::EnvironmentError(e)); }
                };

                // We only do this to integers
                return Ok(self.op_unary_integer(f, op));
            }

            _ => {
                return Err(InterpreterError::InvalidUnaryOperation);
            }
        }
        
    }

    //  Perform an operation
    //
    fn perform_operation(&mut self, lhs: Object, rhs: Object, op: Opcode) -> Result<Object, InterpreterError> {

        match lhs.clone() {

            // Integer
            //
            Object::Integer(i_lhs) => {

                match rhs.clone() {

                    Object::Integer(i_rhs) => {

                        return Ok(self.op_integer(i_lhs, i_rhs, op));
                    },

                    Object::Float(f_rhs) => {

                        return Ok(self.op_float(MFloat::from_rug_integer(i_lhs.get_value()), f_rhs, op));
                    }

                    Object::String(_) => {

                        let s_lhs = match i_lhs.as_micron_string() {
                            Ok(s)   => { Object::String( s ) }
                            Err(e) =>  { return Err(InterpreterError::EnvironmentError(e)); }
                        };

                        return self.perform_string_op(s_lhs, rhs, op);
                    }

                    _ => {
                        return Err(InterpreterError::InvalidExpression);
                    }
                }
            }

            // Float type
            //
            Object::Float(f_lhs) => {

                match rhs.clone() {

                    Object::Integer(i_rhs) => {

                        //return Ok(self.op_integer(i_lhs, i_rhs, op));
                        return Ok(self.op_float(f_lhs, MFloat::from_rug_integer(i_rhs.get_value()), op));
                    },

                    Object::Float(f_rhs) => {

                        return Ok(self.op_float(f_lhs, f_rhs, op));
                    }

                    Object::String(_) => {

                        let s_lhs = match f_lhs.as_micron_string() {
                            Ok(s)   => { Object::String( s ) }
                            Err(e) =>  { return Err(InterpreterError::EnvironmentError(e)); }
                        };

                        return self.perform_string_op(s_lhs, rhs, op);
                    }

                    _ => {
                        return Err(InterpreterError::InvalidExpression);
                    }
                }
            }

            //  String Type
            //
            Object::String(_) => {

                match rhs.clone() {

                    Object::Integer(i_rhs) => {

                        let s_rhs = match i_rhs.as_micron_string() {
                            Ok(s)   => { Object::String( s ) }
                            Err(e) =>  { return Err(InterpreterError::EnvironmentError(e)); }
                        };

                        return self.perform_string_op(lhs, s_rhs, op);
                    },

                    Object::Float(f_rhs) => {

                        let s_rhs = match f_rhs.as_micron_string() {
                            Ok(s)   => { Object::String( s ) }
                            Err(e) =>  { return Err(InterpreterError::EnvironmentError(e)); }
                        };

                        return self.perform_string_op(lhs, s_rhs, op);
                    }

                    Object::String(_) => {

                        return self.perform_string_op(lhs, rhs, op);
                    }

                    _ => {
                        return Err(InterpreterError::InvalidExpression);
                    }

                }

            }

            //  Other Types
            //
            _ => {
                return Err(InterpreterError::InvalidExpression);
            }
        }
    }

    // Perform unary operation on an integer
    //
    fn op_unary_integer(&self, operand: MInteger, op: UnaryOpcode) -> Object {

        match op {

            // Negate (!)
            //
            UnaryOpcode::Negate => {

                if operand.get_value() > 0 {
                    return Object::Integer(MInteger::new(0));
                } else {
                    return Object::Integer(MInteger::new(1));
                }
            }

            // Not (~)
            //
            UnaryOpcode::BwNot => {
                return Object::Integer(MInteger::from_rug_integer(!operand.get_value()));
            }
        }
    }

    //  Perform op on integer
    //
    fn op_integer(&self, lhs: MInteger, rhs: MInteger, op: Opcode) -> Object {

        let lhs = lhs.get_value();
        let rhs = rhs.get_value();

        return match op {
            Opcode::Mul => {
                //println!("mul");
                Object::Integer(MInteger::from_rug_integer(lhs * rhs))
            }
            Opcode::Div => {
                //println!("div");
                Object::Integer(MInteger::from_rug_integer(lhs / rhs))
            }
            Opcode::Add => {
                //println!("add");
                Object::Integer(MInteger::from_rug_integer(lhs + rhs))
            }
            Opcode::Sub => {
                //println!("sub");
                Object::Integer(MInteger::from_rug_integer(lhs - rhs))
            }
            Opcode::Lte => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs <= rhs)))
            }

            Opcode::Gte => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs >= rhs)))
            }

            Opcode::Lt => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs < rhs)))
            }

            Opcode::Gt => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs > rhs)))
            }

            Opcode::Equal => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs == rhs)))
            }

            Opcode::Ne => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs != rhs)))
            }

            Opcode::Pow => {

                // Rust pow for i64 requires a u32 so we attempt to convert it to a u32
                // if it fails a PANIC!
                let rhs_converted = match u32::try_from(rhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u32 for \"pow\": {}", e);
                    }
                };

                Object::Integer(MInteger::from_rug_integer( 
                    Integer::from(lhs.pow(rhs_converted)))
                )
            }

            Opcode::Mod => {
                Object::Integer(MInteger::from_rug_integer(lhs % rhs))
            }

            Opcode::Lsh => {

                let rhs_converted = match u64::try_from(rhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs_converted << rhs_converted)))
            }

            Opcode::Rsh => {

                let rhs_converted = match u64::try_from(rhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };
                
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs_converted >> rhs_converted)))
            }

            Opcode::BwXor => {
                Object::Integer(MInteger::from_rug_integer(lhs ^ rhs))
            }

            Opcode::BwOr => {
                Object::Integer(MInteger::from_rug_integer(lhs | rhs))
            }

            Opcode::BwAnd => {
                Object::Integer(MInteger::from_rug_integer(lhs & rhs))
            }

            Opcode::Or => {

                if lhs > 0 || rhs > 0{
                    return Object::Integer(MInteger::from_rug_integer(Integer::from(1)));
                }
                return Object::Integer(MInteger::from_rug_integer(Integer::from(0)));
            }

            Opcode::And => {
                
                if lhs > 0 && rhs > 0{
                    return Object::Integer(MInteger::from_rug_integer(Integer::from(1)));
                }
                return Object::Integer(MInteger::from_rug_integer(Integer::from(0)));
            }

        }
    }

    // Perform op on float
    //
    fn op_float(&self, lhs: MFloat, rhs: MFloat, op: Opcode) -> Object {

        let lhs = lhs.get_value();
        let rhs = rhs.get_value();

        return match op {
            Opcode::Mul => {
                //println!("mul");
                Object::Float(MFloat::from_rug_float(lhs * rhs))
            }
            Opcode::Div => {
                //println!("div");
                Object::Float(MFloat::from_rug_float(lhs / rhs))
            }
            Opcode::Add => {
                //println!("add");
                Object::Float(MFloat::from_rug_float(lhs + rhs))
            }
            Opcode::Sub => {
                //println!("sub");
                Object::Float(MFloat::from_rug_float(lhs - rhs))
            }
            Opcode::Lte => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs <= rhs)))
            }

            Opcode::Gte => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs >= rhs)))
            }

            Opcode::Lt => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs < rhs)))
            }

            Opcode::Gt => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs > rhs)))
            }

            Opcode::Equal => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs == rhs)))
            }

            Opcode::Ne => {
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs != rhs)))
            }

            Opcode::Pow => {

                // Rust pow for i64 requires a u32 so we attempt to convert it to a u32
                // if it fails a PANIC!
                let rhs_converted = rhs.to_f64();
                
                Object::Float(MFloat::from_rug_float(Float::from(lhs.pow(rhs_converted))))
            }

            Opcode::Mod => {
                Object::Float(MFloat::from_rug_float(lhs % rhs))
            }

            Opcode::Lsh => {

                let rhs_converted = match u64::try_from(rhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"lsh\": {}", e);
                    }
                };

                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs_converted << rhs_converted)))
            }

            Opcode::Rsh => {

                let rhs_converted = match u64::try_from(rhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };

                let lhs_converted = match u64::try_from(lhs.to_integer().unwrap()) {
                    Ok(r) => { r }
                    Err(e) => {
                        panic!("Unable to convert value into u64 for \"rhs\": {}", e);
                    }
                };
                
                Object::Integer(MInteger::from_rug_integer(Integer::from(lhs_converted >> rhs_converted)))
            }

            Opcode::BwXor => {
                Object::Float( MFloat::from_rug_float(
                    Float::with_val(FLOAT_PRECISION, lhs.to_integer().unwrap() ^ rhs.to_integer().unwrap() ) )
                )
            }

            Opcode::BwOr => {
                Object::Float( MFloat::from_rug_float(
                    Float::with_val(FLOAT_PRECISION, lhs.to_integer().unwrap() | rhs.to_integer().unwrap() ) )
                )
            }

            Opcode::BwAnd => {
                Object::Float( MFloat::from_rug_float(
                    Float::with_val(FLOAT_PRECISION, lhs.to_integer().unwrap() & rhs.to_integer().unwrap() ) )
                )
            }

            Opcode::Or => {

                if lhs > 0 || rhs > 0{
                    return Object::Integer(MInteger::from_rug_integer(Integer::from(1)));
                }
                return Object::Integer(MInteger::from_rug_integer(Integer::from(0)));
            }

            Opcode::And => {
                
                if lhs > 0 && rhs > 0{
                    return Object::Integer(MInteger::from_rug_integer(Integer::from(1)));
                }
                return Object::Integer(MInteger::from_rug_integer(Integer::from(0)));
            }
        }
    }
}


