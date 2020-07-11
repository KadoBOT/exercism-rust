#[derive(Copy, Clone)]
pub enum Direction {
    Down,
    Up,
}

#[derive(Copy, Clone)]
pub struct RailFence {
    rails: usize,
    direction: Direction,
    curr: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
            direction: Direction::Down,
            curr: 0,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![String::new(); self.rails];
        text.chars()
            .zip(*self)
            .for_each(|(ch, n)| rails[n].push(ch));
        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut indexes: Vec<_> = self.zip(1..).take(cipher.len()).collect();
        indexes.sort();

        let mut char_with_index: Vec<_> = cipher
            .chars()
            .zip(indexes)
            .map(|(c, (_, i))| (i, c))
            .collect();
        char_with_index.sort();
        char_with_index.iter().map(|(_, c)| c).collect()
    }
}

impl Iterator for RailFence {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match (self.curr, self.direction) {
            (x, Direction::Down) if x == self.rails - 1 => {
                self.direction = Direction::Up;
                self.curr = x - 1;
                Some(x)
            }
            (x, Direction::Up) if x == 0 => {
                self.direction = Direction::Down;
                self.curr = x + 1;
                Some(x)
            }
            (x, Direction::Down) => {
                self.curr = x + 1;
                Some(x)
            }
            (x, Direction::Up) => {
                self.curr = x - 1;
                Some(x)
            }
        }
    }
}
