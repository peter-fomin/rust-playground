use std::cmp::Ordering;

pub fn find<T: AsRef<[R]>, R: PartialOrd>(array: T, key: R) -> Option<usize> {
    let array = array.as_ref();
    let index = array.len() / 2;

    match array
        .get(index)?
        .partial_cmp(&key)
        .unwrap_or(Ordering::Equal)
    {
        Ordering::Equal => Some(index),
        Ordering::Greater => find(&array[..index], key),
        Ordering::Less => find(&array[(index + 1)..], key).map(|i| i + index + 1),
    }
}
