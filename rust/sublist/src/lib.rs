use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_include<T: PartialEq + Clone>(small: Vec<T>, large: Vec<T>) -> bool {
    let first = small.first().unwrap();
    for (i, l) in large.iter().enumerate() {
        if l == first {
            let t = large.get(i..i + small.len()).unwrap_or(&[]).to_vec();
            if t == small {
                return true;
            }
        }
    }
    false
}

pub fn sublist<T: PartialEq + Clone>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if _first_list.len() < _second_list.len() {
        if _first_list.len() == 0 {
            return Comparison::Sublist;
        }
        if is_include(_first_list.to_vec(), _second_list.to_vec()) {
            return Comparison::Sublist;
        } else {
            return Comparison::Unequal;
        }
    }

    if _first_list.len() > _second_list.len() {
        if _second_list.len() == 0 {
            return Comparison::Superlist;
        }
        if is_include(_second_list.to_vec(), _first_list.to_vec()) {
            return Comparison::Superlist;
        } else {
            return Comparison::Unequal;
        }
    }

    return Comparison::Unequal;
}
