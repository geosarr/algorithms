#[cfg(test)]
mod unit_test;
mod stack;
mod queue;
mod deque;
pub use stack::{LinkedListStack};
pub use queue::{LinkedListQueue};
pub use deque::{LinkedListDeque};

// Implementation of the Djikstra two-stack algorithm
#[derive(Default, Debug)]
pub struct Evaluation {
    ops: LinkedListStack<String>,
    vals: LinkedListStack<usize>,
}

impl Evaluation {
    pub fn new() -> Self {
        Self {
            ops: LinkedListStack::new(),
            vals: LinkedListStack::new(),
        }
    }

    pub fn run(&mut self, mut expression: String) -> usize {
        // operations, parentheses and operands should be separated
        // by white spaces, e.g.  ( ( 1 * ( 2 + 3 ) ) + ( 4 * ( 5 + 6 ) ) )
        for elt in expression.split_whitespace(){
            let c = elt.to_string();
            println!("{:?}", self.vals);
            println!("{:?}", self.ops);
            
            if c == "+" || c == "*" {
                self.ops.push(c);
            } else if c == ")" {
                // Apply the last operation to the 2 last values
                let op = self.ops.pop().expect("Failed poping last op");
                let a = self.vals.pop().unwrap();
                let b = self.vals.pop().unwrap();
                if op == "+" {
                    println!("{}", a+b);
                    self.vals.push(a + b);   
                } else if op == "*" {
                    println!("{}", a * b);
                    self.vals.push(a * b);
                }
            } else if c == "(" {} 
            else {
                self.vals.push(c.parse::<usize>().unwrap());
            }
            println!("\n");
        }

        let res = self.vals.pop().expect("Failed poping result");
        println!("{}", res);
        res
    }
}