#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn has_contain <T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    let _first_len = _first_list.len();
    let _second_len = _second_list.len();
    if _first_len <= _second_len {
        return false;
    }

    for i in 0.._first_len {
        if i + _second_len > _first_len {
            break;
        }
        if &_first_list[i..i + _second_len] == _second_list {
            return true;
        }
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if has_contain(_first_list, _second_list) {
        return Comparison::Superlist;
    } else if has_contain(_second_list, _first_list) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}
