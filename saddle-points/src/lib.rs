pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input.iter().enumerate().fold(vec![], |mut acc, (j, row)| {
        row.iter().enumerate().for_each(|(i, num)| {
            let is_row_max = row.iter().all(|x| num >= x);
            let col = input.iter().filter_map(|n| n.get(i)).collect::<Vec<_>>();
            let is_col_min = col.iter().all(|&x| num <= x);
            if is_row_max && is_col_min {
                acc.push((j, i));
            }
        });
        acc
    })
}
