#[derive(Debug)]
pub struct ChessPosition {
    position: (i32, i32),
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition {
                position: (file, rank),
            }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (x, y) = self.0.position;
        let (other_x, other_y) = other.0.position;
        let is_same_line = x == other_x;
        let is_same_column = y == other_y;
        let is_diagonal = (y - other_y).abs() == (x - other_x).abs();

        is_same_line || is_same_column || is_diagonal
    }
}
