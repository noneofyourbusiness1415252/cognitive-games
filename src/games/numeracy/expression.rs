use js_sys::Math;

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    pub text: String,
    pub value: f64,
}

impl Expression {
    pub fn new(level: u32) -> Self {
        let ops = ['+', '-', '*', '/'];
        let denominators = [2, 4, 5, 8];
        let use_decimals = level > 3 && Math::random() < 0.3;
        
        let complexity = (f64::from(level) * 1.2).ceil() as i32;
        let a = (Math::random() * f64::from(complexity * 5)).floor() as i32 + complexity;
        let b = (Math::random() * f64::from(complexity * 2)).floor() as i32 + 1;

        let make_decimal = |n: i32| 
            if use_decimals {
                let d = denominators[(Math::random() * 4.0) as usize];
                let val = (n * d) as f64 / d as f64;
                (val, val.to_string())
            } else {
                (n.into(), n.to_string())
            }
        ;

        let (a_val, a_text) = make_decimal(a);
        let (b_val, b_text) = make_decimal(b);
        let op = ops[(Math::random() * 4.0) as usize];

        let (text, value) = match op {
            '+' => (format!("{a_text} + {b_text}"), a_val + b_val),
            '-' => (format!("{a_text} - {b_text}"), a_val - b_val),
            '*' => (format!("{a_text} × {b_text}"), a_val * b_val),
            '/' => (format!("{} ÷ {b_text}", a_val * b_val), a_val),
            _ => unreachable!(),
        };

        Self { text, value }
    }
}
