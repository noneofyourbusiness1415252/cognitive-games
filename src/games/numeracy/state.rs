use super::{Expression, Level};
use web_sys::{Performance, Storage};

#[derive(Debug)]
pub struct GameState {
    pub level: Level,
    pub expressions: Vec<Expression>,
    pub selected_indices: Vec<usize>,
    pub round_start: Option<f64>,
    pub level_start: Option<f64>,
    pub completed_rounds: u32,
    performance: Performance,
    storage: Storage,
}

impl GameState {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        let level_number = storage
            .get_item("numeracy_level")
            .unwrap()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        let level = Level::new(level_number);
        let expressions = level.generate_expressions();
        let performance = window.performance().unwrap();

        Self {
            level,
            expressions,
            selected_indices: Vec::new(),
            round_start: None,
            level_start: None,
            completed_rounds: 0,
            performance,
            storage,
        }
    }

    pub fn start_level(&mut self) {
        self.level_start = Some(self.performance.now());
        self.start_round();
    }

    pub fn start_round(&mut self) {
        self.expressions = self.level.generate_expressions();
        self.selected_indices.clear();
        self.round_start = Some(self.performance.now());
    }

    pub fn toggle_selection(&mut self, index: usize) -> bool {
        if let Some(pos) = self.selected_indices.iter().position(|&i| i == index) {
            self.selected_indices.remove(pos);
            true
        } else if self.selected_indices.len() < 3 {
            self.selected_indices.push(index);
            true
        } else {
            false
        }
    }

    pub fn check_current_round(&self) -> bool {
        if self.selected_indices.len() != 3 {
            return false;
        }

        let selected_expressions: Vec<Expression> = self
            .selected_indices
            .iter()
            .map(|&i| self.expressions[i].clone())
            .collect();

        Level::check_order(&selected_expressions)
    }

    pub fn get_round_time_remaining(&self) -> Option<f64> {
        self.round_start.map(|start| {
            let elapsed = self.performance.now() - start;
            if elapsed >= 15000.0 {
                0.0
            } else {
                15000.0 - elapsed
            }
        })
    }

    pub fn update_score(&mut self, round_success: bool) {
        let time_bonus = self
            .get_round_time_remaining()
            .map_or(0, |t| (t / 1000.0) as i32);

        if round_success {
            // Calculate level jumps based on time bonus using a mathematical formula
            let level_jump = if time_bonus > 0 {
                ((time_bonus as f64 * 0.2).floor() as u32).min(3)
            } else {
                0
            };

            if level_jump > 0 {
                let new_level = self.level.number + level_jump;
                self.level = Level::new(new_level);
                self.storage
                    .set_item("numeracy_level", &new_level.to_string())
                    .unwrap();
            }
        } else if self.level.number > 1 {
            let new_level = self.level.number - 1;
            self.level = Level::new(new_level);
            self.storage
                .set_item("numeracy_level", &new_level.to_string())
                .unwrap();
        }

        self.completed_rounds += 1;
    }
}
