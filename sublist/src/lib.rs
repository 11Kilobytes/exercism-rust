#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() <= _second_list.len() {
        sublist_helper(_first_list, _second_list)
    } else {
        match sublist_helper(_second_list, _first_list) {
            Comparison::Equal => Comparison::Equal,
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            Comparison::Unequal => Comparison::Unequal
        }
    }
}

/* Assumes that _first_list.len() <= _second_list.len() */
fn sublist_helper<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == 0 {
        if _second_list.len() == 0 {
            Comparison::Equal
        } else {
            Comparison::Sublist
        }
    } else {
        let startOfSublist = _second_list.iter()
            .enumerate()
            .filter(|&(i, elem)| {
                   *elem == _first_list[0] 
                && i <= _second_list.len() - _first_list.len()
                && &_second_list[i..(i+_first_list.len())] == _first_list
            }).next();
        match startOfSublist {
            Some(_) => 
                if _first_list.len() == _second_list.len() {
                    Comparison::Equal
                } else {
                    Comparison::Sublist
                },
            None => Comparison::Unequal,
        }
    }
}

