use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let index = array.len() / 2;

    match array.get(index)?.cmp(&key) {
        Ordering::Equal => Some(index),
        Ordering::Greater => find(&array[..index], key),
        Ordering::Less => find(&array[(index + 1)..], key).map(|i| i + index + 1),
    }
}
