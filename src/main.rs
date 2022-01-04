/// Op is a enum that represents the different operations.
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
    /// Lsh x << y.
    Lsh,
    /// And x & y.
    And,
    /// Or x | y.
    Or,
    /// Xor x ^ y.
    Xor,
}

/// Op is a unary operation.
impl Op {
    /// new returns a new unary operation.
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

/// Token is a lexical token.
pub struct Token {
    /// op is the token's operation.
    pub op: Op,
    /// lhs is the token's left-hand side.
    pub lhs: i128,
    /// rhs is the token's right-hand side.
    pub rhs: Option<i128>,
}

impl Token {
    /// new creates a new token.
    pub fn new(input: &str) -> Token {
        let mut tokens = input.split_whitespace();
        let lhs = tokens.next().unwrap().parse::<i128>().unwrap();
        let op = tokens.next().unwrap();
        let rhs = tokens.next().map(|rhs| rhs.parse::<i128>().unwrap());
        Token {
            op: Op::new(op).unwrap(),
            lhs,
            rhs,
        }
    }

    /// eval evaluates the Token.
    pub fn eval(&self) -> i128 {
        match self.op {
            Op::Add => self.lhs + self.rhs.unwrap(),
            Op::Sub => self.lhs - self.rhs.unwrap(),
            Op::Mul => self.lhs * self.rhs.unwrap(),
            Op::Div => self.lhs / self.rhs.unwrap(),
            Op::Mod => self.lhs % self.rhs.unwrap(),
            Op::Eq => (self.lhs == self.rhs.unwrap()) as i128,
            Op::Rsh => self.lhs >> self.rhs.unwrap(),
            Op::Lsh => self.lhs << self.rhs.unwrap(),
            Op::And => self.lhs & self.rhs.unwrap(),
            Op::Or => self.lhs | self.rhs.unwrap(),
            Op::Xor => self.lhs ^ self.rhs.unwrap(),
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();

    let token = Token::new(&input);
    println!("{}", token.eval());
}
