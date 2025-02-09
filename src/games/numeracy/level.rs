use super::expression::Expression;
#[derive(Debug)]
pub struct Level {
    pub number: u32,
    expressions_per_round: usize,
}

impl Level {
    pub fn new(number: u32) -> Self {
        Self {
            number,
            expressions_per_round: 3,
        }
    }

    pub fn generate_expressions(&self) -> Vec<Expression> {
        (0..self.expressions_per_round)
            .map(|_| Expression::new(self.number))
            .collect()
    }

    pub fn check_order(expressions: &[Expression]) -> bool {
        let values: Vec<f64> = expressions.iter().map(|e| e.value).collect();
        values.windows(2).all(|w| w[0] <= w[1])
    }
}
