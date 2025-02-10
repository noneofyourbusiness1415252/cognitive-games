use js_sys::Math;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub text: String,
    pub value: f64,
}

impl Expression {
    pub fn new(level: u32) -> Self {
        let ops = ['+', '-', '*', '/'];

        // More gradual complexity increase
        let complexity = (f64::from(level) * 1.2).ceil() as i32;
        let a = (Math::random() * f64::from(complexity * 5)).floor() as i32 + complexity;
        let b = (Math::random() * f64::from(complexity * 2)).floor() as i32 + 1;
        let op = ops[(Math::random() * 4.0).floor() as usize];

        let (text, value) = match op {
            '+' => (format!("{a} + {b}"), f64::from(a + b)),
            '-' => (format!("{a} - {b}"), f64::from(a - b)),
            '*' => (format!("{a} × {b}"), f64::from(a * b)),
            '/' => {
                let product = a * b;
                (format!("{product} ÷ {b}"), f64::from(a))
            }
            _ => unreachable!(),
        };

        Expression { text, value }
    }
}
