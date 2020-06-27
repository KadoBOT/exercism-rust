#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result = vec![vec![0; size as usize]; size as usize];
    let mut direction = Direction::Right;
    let mut coords = (0, 0);
    for n in 0..size * size {
        result[coords.1][coords.0] = n + 1;
        let change_right = coords.0 == (size - 1) as usize || result[coords.1][coords.0 + 1] != 0;
        let change_down = coords.1 == (size - 1) as usize || result[coords.1 + 1][coords.0] != 0;
        let change_left = coords.0 == 0 || result[coords.1][coords.0 - 1] != 0;
        let change_up = coords.1 == 0 || result[coords.1 - 1][coords.0] != 0;
        direction = match direction {
            Direction::Right if change_right => Direction::Down,
            Direction::Down if change_down => Direction::Left,
            Direction::Left if change_left => Direction::Up,
            Direction::Up if change_up => Direction::Right,
            _ => direction,
        };
        match direction {
            Direction::Right => coords = (coords.0 + 1, coords.1),
            Direction::Down => coords = (coords.0, coords.1 + 1),
            Direction::Left => coords = (coords.0 - 1, coords.1),
            Direction::Up => coords = (coords.0, coords.1 - 1),
        }
    }

    result
}
