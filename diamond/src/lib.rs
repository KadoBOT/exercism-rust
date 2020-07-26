pub fn get_diamond(c: char) -> Vec<String> {
    let distance = ((c as u8) - b'A') as usize;
    let mut result = vec![" ".repeat(distance + distance + 1); distance + distance + 1];
    let size = result.len();

    let mut replace_char = |idx: usize, pos: usize, ch: char| {
        result[idx].replace_range(pos..=pos, &ch.to_string());
    };

    let mut replace_row = |idx: usize, n: usize| {
        replace_char(idx, distance - n, (b'A' + n as u8) as char);
        replace_char(idx, distance + n, (b'A' + n as u8) as char);
    };

    (0..=distance).for_each(|n| {
        let tail = (size - n) - 1;
        replace_row(n as usize, n);
        replace_row(tail as usize, n);
    });
    result
}
