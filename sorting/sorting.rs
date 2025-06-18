pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    for i in 0..arr.len().saturating_sub(1) {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

use std::collections::HashSet;

pub fn have_same_elements<T: Eq + std::hash::Hash>(a: &[T], b: &[T]) -> bool {
    let set_a: HashSet<_> = a.iter().collect();
    b.iter().any(|item| set_a.contains(item))
}
