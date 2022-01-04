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
}

pub fn eval(op: Op, lhs: i32, rhs: i32) -> i32 {
    match op {
        Op::Add => lhs + rhs,
        Op::Sub => lhs - rhs,
        Op::Mul => lhs * rhs,
        Op::Div => lhs / rhs,
        Op::Mod => lhs % rhs,
        Op::Eq => (lhs == rhs) as i32,
    }
}

pub fn parse_op(op: &str) -> Op {
    match op {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        "%" => Op::Mod,
        "==" => Op::Eq,
        _ => panic!("unknown op: {}", op),
    }
}

pub fn parse_and_eval(expr: &str) -> i32 {
    let mut tokens = expr.split_whitespace();
    let lhs = tokens.next().unwrap().parse::<i32>().unwrap();
    let op = parse_op(tokens.next().unwrap());
    let rhs = tokens.next().unwrap().parse::<i32>().unwrap();

    eval(op, lhs, rhs)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();

    println!("{}", parse_and_eval(&input));
}
