use js_sys::Math;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub text: String,
    pub value: f64,
}

impl Expression {
    pub fn new(level: u32) -> Self {
        let ops = ['+', '-', '*', '/'];
        
        let complexity = (level as f64 * 1.5).ceil() as i32;
        let a = (Math::random() * (complexity * 8) as f64).floor() as i32 + complexity;
        let b = (Math::random() * (complexity * 3) as f64).floor() as i32 + 1;
        let op = ops[(Math::random() * 4.0).floor() as usize];
        
        let (text, value) = match op {
            '+' => (format!("{} + {}", a, b), (a + b) as f64),
            '-' => (format!("{} - {}", a, b), (a - b) as f64), // a is always > b
            '*' => (format!("{} × {}", a, b), (a * b) as f64),
            '/' => {
                let product = a * b;
                (format!("{} ÷ {}", product, b), a as f64) // Division always results in a
            },
            _ => unreachable!(),
        };

        Expression { text, value }
    }
}