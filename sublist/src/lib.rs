use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn b_contains_a<T: PartialEq>(list_a: &[T], list_b: &[T]) -> bool {
    list_a.is_empty() || list_b.windows(list_a.len()).any(|b| b.eq(list_a))
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.eq(second_list) {
        return Comparison::Equal;
    }

    if b_contains_a(first_list, second_list) {
        return Comparison::Sublist;
    }

    if b_contains_a(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
