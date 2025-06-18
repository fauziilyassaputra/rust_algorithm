#[cfg(test)]
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] <= w[1])
}
use std::collections::HashSet;

#[cfg(test)]
use std::cmp;

#[cfg(test)]
pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
// T: cmp::PartialOrd,
// If HashSet is used
    T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
{


    if a.len() == b.len() {
        // This is O(n^2) but performs better on smaller data sizes
        //b.iter().all(|item| a.contains(item))

        // This is O(n), performs well on larger data sizes
        let set_a: HashSet<&T> = a.iter().collect();
        let set_b: HashSet<&T> = b.iter().collect();
        set_a == set_b
    } else {
        false
    }
}
