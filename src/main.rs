

pub mod ast;

#[macro_use] 
extern crate lalrpop_util;

lalrpop_mod!(pub micron); // synthesized by LALRPOP


struct Vm {

    eval_stack: Vec<i32>
}

impl Vm {

    fn new() -> Self {
        Self {
            eval_stack: Vec::new()
        }
    }

    fn walk(&mut self, expr: ast::Expr) {
    
        match expr {
            ast::Expr::Number(n) => {
                println!("{}", n);

                self.eval_stack.push(n);
            }

            ast::Expr::Op(lhs, op, rhs) => {

                self.walk(*lhs);
                self.walk(*rhs);

                println!("{:?}", op);

                let l = self.eval_stack.pop().unwrap();
                let r = self.eval_stack.pop().unwrap();
                let mut result : i32 = 0;
                
                match op {
                    ast::Opcode::Mul => {
                        result = l * r;
                    }
                    ast::Opcode::Div => {
                        result = l / r;
                    }
                    ast::Opcode::Add => {
                        result = l + r;
                    }
                    ast::Opcode::Sub => {
                        result = l - r;
                    }
                }
                self.eval_stack.push(result);
            }
        }
    }

    fn eval(&mut self, expr: ast::Expr) {
        
        self.walk(expr);

        let result = self.eval_stack.pop().unwrap();

        println!("Result : {}", result);
    }
}

fn main() {
    println!("Hello, world!");

    let mut vm = Vm::new();

    let expr = micron::ExprParser::new()
        .parse("(22 * 44) * (3 + (66 * 2) )")
        .unwrap();

    vm.eval(*expr);
}
