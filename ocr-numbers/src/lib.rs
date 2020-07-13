use array_tool::vec::Intersect;
use itertools::Itertools;
use std::str;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines_size = input.lines().count();
    if lines_size % 4 != 0 {
        return Err(Error::InvalidRowCount(lines_size));
    }

    let mut results = vec![];
    for chunk in &input.lines().chunks(4) {
        let mut possible_results = vec![];
        for (idx, line) in chunk.take(3).enumerate() {
            if line.len() % 3 != 0 {
                return Err(Error::InvalidColumnCount(line.len()));
            }

            line.as_bytes().chunks(3).enumerate().for_each(|(i, b)| {
                let c = str::from_utf8(b).unwrap();
                let possible = if idx == 0 {
                    match c {
                        " _ " => vec![2, 3, 5, 6, 7, 8, 9, 0],
                        "   " => vec![1, 4],
                        _ => vec![],
                    }
                } else if idx == 1 {
                    match c {
                        "  |" => vec![1, 7],
                        " _|" => vec![2, 3],
                        "|_|" => vec![4, 8, 9],
                        "|_ " => vec![5, 6],
                        "| |" => vec![0],
                        _ => vec![],
                    }
                } else {
                    match c {
                        "  |" => vec![1, 4, 7],
                        " _|" => vec![3, 5, 9],
                        "|_|" => vec![6, 8, 0],
                        "|_ " => vec![2],
                        _ => vec![],
                    }
                };

                let possible_result = possible_results.get(i);
                if possible_result.is_some() {
                    let item: &Vec<i32> = &possible_results[i];
                    possible_results[i] = item.intersect(possible);
                } else {
                    possible_results.push(possible);
                }
            });
        }
        results.push(possible_results);
    }

    let result = results
        .iter()
        .map(|r| {
            // let res = r.iter().flatten().join("");
            let res = r
                .iter()
                .map(|z| {
                    if let Some(n) = z.get(0) {
                        return n.to_string();
                    }
                    "?".to_string()
                })
                .collect::<String>();
            res
        })
        .collect::<Vec<String>>()
        .join(",");

    Ok(result)
}
