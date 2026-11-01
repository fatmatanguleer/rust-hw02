#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    // Task 1: Convert char to Op
    fn from_char(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '-' => Some(Op::Sub),
            '*' => Some(Op::Mul),
            '/' => Some(Op::Div),
            _ => None,
        }
    }

    // Task 2: Convert Op to char
    fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    // Task 3: Get operator precedence (1 for +, -, 2 for *, /)
    fn precedence(self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }

    // Task 4: Apply operation with division-by-zero check
    fn apply(self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div => {
                if b == 0.0 {
                    None
                } else {
                    Some(a / b)
                }
            }
        }
    }
}

// Task 5: Evaluate simple expression in one line using ? operator
fn evaluate_simple(a: f64, op: char, b: f64) -> Option<f64> {
    Op::from_char(op)?.apply(a, b)
}

#[derive(Debug)]
struct Stack {
    items: Vec<f64>,
}

impl Stack {
    // Task 6: Implement new, push, and pop
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, v: f64) {
        self.items.push(v);
    }

    fn pop(&mut self) -> Option<f64> {
        self.items.pop()
    }

    // Task 7: Implement peek and len
    fn peek(&self) -> Option<&f64> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    // Task 8 & 9: Apply operation on stack (reverts stack if fails or not enough values)
    fn apply(&mut self, op: Op) -> Option<()> {
        if self.items.len() < 2 {
            return None;
        }

        let b = self.items.pop()?;
        let a = self.items.pop()?;

        if let Some(res) = op.apply(a, b) {
            self.items.push(res);
            Some(())
        } else {
            self.items.push(a);
            self.items.push(b);
            None
        }
    }
}

fn main() {
    // Task 10: Compute (2 + 3) * 4 - 1 using only push and apply (expected result is 19)
    let mut stack = Stack::new();

    stack.push(2.0);
    stack.push(3.0);
    stack.apply(Op::Add);

    stack.push(4.0);
    stack.apply(Op::Mul);

    stack.push(1.0);
    stack.apply(Op::Sub);

    println!("Result (expected 19): {:?}", stack.peek());

    // Task 10: Empty the stack using while let Some(v) = stack.pop()
    while let Some(v) = stack.pop() {
        println!("Popped value: {}", v);
    }
}