mod lib;

fn main() {
    println!("Hello, world!");
}

// di import dari math.rs 
use crate::lib::PCG32;

// mengambil waktu berbasis sekarang
use std::time::{SystemTime, UNIX_EPOCH};

//  angka default dari 2^32
const DEFAULT: u64 = 4294967296;

//  mengecek apakah slice sudah terurut
fn is_sorted<T: Ord>(arr: &[T], len: usize) -> bool {
    for i in 0..len - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }

    true
}

//  komplikasi fungsi yang sesuai dengan arsitektur cpu
#[cfg(target_pointer_width = "64")]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u64() as usize % range
}

#[cfg(not(target_pointer_width = "64"))]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u32() as usize % range
}

/**
 * Fisher–Yates shuffle for generating random permutation.
 */
fn permute_randomly<T>(arr: &mut [T], len: usize, generator: &mut PCG32) {
    for i in (1..len).rev() {
        let j = generate_index(i + 1, generator);
        arr.swap(i, j);
    }
}

pub fn bogo_sort<T: Ord>(arr: &mut [T]) {
    /* 
    -gunakan variable seed untuk memulai generator acak PCG32 agar memiliki state awal, jika tidak punya seed, maka
    tidak bisa menghasilkan deret bilangan acak
    - seed diambil dari waktu sekarang sejak 1 januari 1970 permilisekon lalu kira ubah menjadi u64
    -jikalau waktu yang sudah kita panggil mengalami error, maka kita gunakan default suapaya program tetap berjalan
    */
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => DEFAULT,
    };
  
    let mut random_generator = PCG32::new_default(seed);
    
    let arr_length = arr.len();
    // jika belum terurut, acak seluruh arraynya sampai terurut
    while !is_sorted(arr, arr_length) {
        permute_randomly(arr, arr_length, &mut random_generator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn random_array() {
        let mut arr = [1, 8, 3, 2, 7, 4, 6, 5];
        bogo_sort(&mut arr);
        println!("{:?}", arr);
        for i in 0..arr.len() - 1 {
            println!("{:?}",arr[i] <= arr[i + 1]);
        }
    }

    // pastikan array yang sudah terurut tetap berurut (sehingga membuat loop berakhir)
    #[test]
    fn sorted_array() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        bogo_sort(&mut arr);
        println!("{:?}", arr);
        
        for i in 0..arr.len() - 1 {
            println!("{:?}",arr[i] <= arr[i + 1]);
        }
    }
}
