
fn main() {
    println!("hello world")
}

use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    let is_asc = is_asc_arr(arr);

    while left < right {
        if match_compare(item, arr, &mut left, &mut right, is_asc){
            return Some(left);
        }
    }
    None
}

fn match_compare<T: Ord>(
    item: &T,
    arr: &[T],
    left: &mut usize,
    right:  &mut usize,
    is_asc: bool
) -> bool{
    let mid = *left + (*right -*left) /  2;
    let cmp_result = item.cmp(&arr[mid]);
    
    match(is_asc, cmp_result) {
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            *right = mid;
        }
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            *left  = mid + 1;
        }
        (_, Ordering::Equal) => {
            *left = mid;
            return true;
        }
    }
    
    false
}

fn is_asc_arr<T: Ord>(arr: &[T]) -> bool {
    arr.len() > 1 && arr[0] < arr[arr.len() - 1]
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_binary_search(){
        let data_kosong = [];
        let data_awal = [5,4,3,2,1];
        let data_tengah = [1,2,3,4,5,6,7,8,9,10];
        let data_akhir =  [1,2,3,4,6,7,8,9,5];
        match binary_search(&5,&data_kosong) { 
            Some(idx) => println!("angka 5 berada pada index ke-{}", idx),
            None      => println!("angka 5 tidak ditemukan")
        }
    }
    
}

/*

*/
