struct Minesweeper {
    field: Vec<Vec<i32>>,
}

impl Minesweeper {
    fn increase_row_values(&mut self, row_idx: usize, col_idx: usize, has_next_column: bool) {
        let row = self.field.get_mut(row_idx).unwrap();
        if col_idx != 0 {
            let left = row.get_mut(col_idx - 1).unwrap();
            if left >= &mut 0 {
                *left += 1;
            }
        }
        let mid = row.get_mut(col_idx).unwrap();
        if mid >= &mut 0 {
            *mid += 1;
        }
        if has_next_column {
            let right = row.get_mut(col_idx + 1).unwrap();
            if right >= &mut 0 {
                *right += 1;
            }
        }
    }

    fn print(&self) -> Vec<String> {
        let format_row = |row: &Vec<i32>| {
            row.iter()
                .map(|x| match x {
                    0 => " ".to_string(),
                    -1 => "*".to_string(),
                    _ => x.to_string(),
                })
                .collect::<String>()
        };

        self.field.iter().map(format_row).collect()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row_len = minefield.get(0).unwrap_or(&"").len();
    let mut minesweeper = Minesweeper {
        field: vec![vec![0; row_len]; minefield.len()],
    };

    minefield.iter().enumerate().for_each(|(row_idx, row)| {
        row.chars().enumerate().for_each(|(col_idx, field)| {
            let has_next_column = col_idx < row.len() - 1;
            let has_next_row = row_idx < minefield.len() - 1;
            if field == '*' {
                minesweeper.field[row_idx][col_idx] = -1;
                if row_idx != 0 {
                    minesweeper.increase_row_values(row_idx - 1, col_idx, has_next_column);
                }

                minesweeper.increase_row_values(row_idx, col_idx, has_next_column);

                if has_next_row {
                    minesweeper.increase_row_values(row_idx + 1, col_idx, has_next_column);
                }
            }
        })
    });

    minesweeper.print()
}
