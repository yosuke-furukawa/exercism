use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_include<T: PartialEq>(small: &[T], large: &[T]) -> bool {
    for v in large.windows(small.len()) {
        if small == v {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    match (_first_list, _second_list) {
        (_list1, _list2) if _list1.len() == 0 => Comparison::Sublist,
        (_list1, _list2) if is_include(_list1, _list2) => Comparison::Sublist,
        (_list1, _list2) if _list2.len() == 0 => Comparison::Superlist,
        (_list1, _list2) if is_include(_list2, _list1) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
