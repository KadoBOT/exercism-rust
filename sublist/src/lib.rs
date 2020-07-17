use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn b_contains_a<T: PartialEq>(list_a: &[T], list_b: &[T]) -> bool {
    if list_a.is_empty() {
        return true;
    }

    if list_a.len() < list_b.len() {
        for chunk in list_a.windows(list_a.len()) {
            for s_chunk in list_b.windows(list_a.len()) {
                if chunk == s_chunk {
                    return true;
                }
            }
        }
    }
    false
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.eq(second_list) {
        return Comparison::Equal;
    }

    if first_list.len() == second_list.len() {
        return Comparison::Unequal;
    }

    if b_contains_a(first_list, second_list) {
        return Comparison::Sublist;
    }

    if b_contains_a(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
