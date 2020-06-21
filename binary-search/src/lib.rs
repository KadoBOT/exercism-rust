use std::cmp::Ordering;

pub fn find<R: AsRef<[T]>, T: Ord>(array: R, key: T) -> Option<usize> {
    let array = array.as_ref();

    if array.last()? < &key {
        return None;
    }
    let middle = array.len() / 2;
    let (left, right) = array.split_at(middle);
    match key.cmp(&array[middle]) {
        Ordering::Equal => Some(middle),
        Ordering::Less => find(left, key),
        Ordering::Greater => find(right, key).map(|i| middle + i),
    }
}
