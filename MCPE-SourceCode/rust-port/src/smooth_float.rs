#[derive(Debug, Clone, Copy, Default)]
pub struct SmoothFloat {
    target_value: f32,
    remaining_value: f32,
    last_amount: f32,
}

impl SmoothFloat {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_new_delta_value(&mut self, delta_value: f32, acceleration_amount: f32) -> f32 {
        self.target_value += delta_value;

        let mut current_delta = (self.target_value - self.remaining_value) * acceleration_amount;
        self.last_amount = self.last_amount + (current_delta - self.last_amount) * 0.5;
        if (current_delta > 0.0 && current_delta > self.last_amount)
            || (current_delta < 0.0 && current_delta < self.last_amount)
        {
            current_delta = self.last_amount;
        }
        self.remaining_value += current_delta;
        current_delta
    }

    pub fn target_value(&self) -> f32 {
        self.target_value
    }
}

