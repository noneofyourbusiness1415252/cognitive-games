use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub text: String,
    pub value: f64,
}

impl Expression {
    pub fn new(level: u32) -> Self {
        let mut rng = rand::thread_rng();
        let ops = ["+", "-", "*", "/"];
        
        let complexity = (level as f64 * 1.5).ceil() as i32;
        let a = rng.gen_range(1..=complexity * 5);
        let b = rng.gen_range(1..=complexity * 3);
        let op = ops[rng.gen_range(0..ops.len())];
        
        let (text, value) = match op {
            "+" => (format!("{} + {}", a, b), (a + b) as f64),
            "-" => (format!("{} - {}", a, b), (a - b) as f64),
            "*" => (format!("{} × {}", a, b), (a * b) as f64),
            "/" => {
                let product = a * b;
                (format!("{} ÷ {}", product, a), b as f64)
            }
            _ => unreachable!(),
        };

        Expression { text, value }
    }
}