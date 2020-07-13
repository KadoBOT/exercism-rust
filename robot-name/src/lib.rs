use rand::prelude::*;

pub struct Robot(String);

impl Default for Robot {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let ch_one = rng.gen_range(b'A', b'Z' + 1);
        let ch_two = rng.gen_range(b'A', b'Z' + 1);
        let digits = rng.gen_range(100, 1000);
        let name = format!("{}{}{}", ch_one as char, ch_two as char, digits);

        Robot(name)
    }
}

impl Robot {
    pub fn new() -> Self {
        Robot::default()
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        *self = Robot::new();
    }
}
