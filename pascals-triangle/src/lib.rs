pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0).fold(vec![], |mut acc: Vec<Vec<u32>>, num| {
            let prev_slice = num.checked_sub(1).and_then(|i| acc.get(i as usize));
            let mut new_vec: Vec<u32> = vec![1; (num + 1) as usize];
            if let Some(sl) = prev_slice {
                (0..=num).for_each(|n| {
                    let prev_num: &u32 = n
                        .checked_sub(1)
                        .map_or(&0u32, |j| sl.get(j as usize).unwrap());
                    let cur_num: &u32 = sl.get(n as usize).unwrap_or(&0);
                    new_vec[n as usize] = prev_num + cur_num;
                });
            }
            acc.push(new_vec);

            acc
        })
    }
}
