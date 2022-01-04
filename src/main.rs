/// Op is a binary operation.
#[derive(Debug)]
pub enum Op {
    /// Add x + y.
    Add,
    /// Sub x - y.
    Sub,
    /// Mul x * y.
    Mul,
    /// Div x / y.
    Div,
    /// Mod x % y.
    Mod,
    /// Signed modulus x % y.
    Eq,
    /// Rsh x >> y.
    Rsh,
    Lsh,
    And,
    Or,
    Xor,
}

impl Op {
    // Create a new Op.
    pub fn new(op: &str) -> Option<Op> {
        match op {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mul),
            "/" => Some(Op::Div),
            "%" => Some(Op::Mod),
            "==" => Some(Op::Eq),
            ">>" => Some(Op::Rsh),
            "<<" => Some(Op::Lsh),
            "&" => Some(Op::And),
            "|" => Some(Op::Or),
            "^" => Some(Op::Xor),
            _ => None,
        }
    }
}


pub fn eval(op: Op, x: i32, y: i32) -> i32 {
    match op {
        Op::Add => x + y,
        Op::Sub => x - y,
        Op::Mul => x * y,
        Op::Div => x / y,
        Op::Mod => x % y,
        Op::Eq => (x == y) as i32,
        Op::Rsh => x >> y,
        Op::Lsh => x << y,
        Op::And => x&y,
        Op::Or => x|y,
        Op::Xor => x^y,
    }
}


pub fn parse_and_eval(expr: &str) -> i32 {
    let mut tokens = expr.split_whitespace();
    let x = tokens.next().unwrap().parse::<i32>().unwrap();
    let op = Op::new(tokens.next().unwrap()).unwrap();
    let y = tokens.next().unwrap().parse::<i32>().unwrap();

    eval(op, x, y)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();

    println!("{}", parse_and_eval(&input));
}
