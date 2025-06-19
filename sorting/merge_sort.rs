mod sorting;
fn main() {
    println!("hello world")
}

fn merge<T: Ord + Copy>(arr: &mut[T], mid: usize){
    // membelah array menjadi dua
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    //inisialisasi petunjuk
    let mut l = 0;
    let mut r = 0;
    
    for v in arr {
      /* 
      - jika  r == right_half.len() yaitu semua elemen kanan sudah dihabis, maka masukkan ke array yang kiri
      - atau jika array kiri degan index[l] lebih kecil dari kanan[r], serta l < dari panjang array kiri maka masukkan
      ke array kiri  (kalau l sama dengan panjang array kiri maka masukkan ke array kanan) 
      
      */
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]){
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

pub fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        // cari nilai midnya 
        let mid = arr.len() / 2;
        // sort the laft recursively
        top_down_merge_sort(&mut arr[..mid]);
        // sort the right half recursively
        top_down_merge_sort(&mut arr[mid..]);
        // combine the two halves
        merge(arr, mid)
    }
}

pub fn bottom_up_merge_sort<T:Copy + Ord>(a : &mut [T]){
    if a.len() > 1 {
        let len: usize = a.len();
        // ukuran blok yang akan di merge, mulai dari 1
        let mut sub_array_size:usize = 1;
       
        while sub_array_size < len {
            let mut start_index:usize = 0;
            // selama masih ada blok kanan minimal seukuran sub_array_size
            while len - start_index > sub_array_size  {
                let end_idx:usize = if start_index + 2 * sub_array_size > len {
                    len
                } else { 
                    start_index + 2 * sub_array_size
                };
                // merge slice kanan dan kirinya dimana :
                // - kiri = [start_index .. start_index+sub_array_size]
                // - kanan = [start_index+sub_array_size .. end_idx]
                merge(&mut a[start_index..end_idx], sub_array_size);
                //update 'start_index' to merge the next sub_arrays
                start_index = end_idx;
            }
          // lompat ke blok berikutnya
            sub_array_size *= 2;
        }
    }
}

#[cfg(test)]      
mod test {
    #[cfg(test)]
    mod top_down_merge_sort {
        use super::super::*;
        use crate::sorting::is_sorted;
        use crate::sorting::have_same_elements;
        
        #[test]
        fn basic() {
            let mut res = vec![10,6,8,3,2,4,1,9,5,7];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            println!("{:?}",res);
            println!("{:?}", is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }
  
  #[cfg(test)]
    mod bottom_up_merge_sort {
        use super::super::*;
        use crate::sorting::is_sorted;
        use crate::sorting::have_same_elements;
        
        #[test]
        fn basic() {
            let mut res = vec![10,6,8,3,2,4,1,9,5,7];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            println!("{:?}",res);
            println!("{:?}", is_sorted(&res) && have_same_elements(&res, &cloned));
        }
        
    }
}

/* 
simulasi fungsi basic (top_down_merge_sort) :
- [10,6,8,3,2,4,1,9,5,7], mid : 5(2), left : [10,6,8,3,2] ,  right : [4,1,9,5,7]

- sort kiri :
- [10,6,8,3,2], mid = 2(8), left: [10,6] ,right: [3,2]
- [10,6], mid : 1 => merge([10,6], 1) => [6,10]
- [3,2], mid : 1 => merge ([3,2],1) => [2,3]
- merge[8] & [2,3] => [2,3,8] 
- merge([6,10,2,3,8],2) => [2,3,6,8,10]


-sort kanan :
- [4,1,9,5,7], mid: 2, left : [4,1] , right : [5,7]
- [4,1], mid: 1, merge([4,1],1) => [1,4]
- [5,7], mid: 1, merge([5,7,1) => [5,7]
- merge[9] & [5,7] => [5,7,9]
- merge([1,4,5,7,9],2) => [1,4,5,7,9]

- merge final :
- merge([2,3,6,8,10,1,4,5,7,9],5)   
-  [2,1] => [1
-  [2,4] => [1,2
-  [3,4] => [1,2,3
-  [6,4] => [1,2,3,4
-  [6,5] => [1,2,3,4,5
-  [6,7] => [1,2,3,4,5,6
-  [8,7] => [1,2,3,4,5,6,7
-  [8,9] => [1,2,3,4,5,6,7,8
-  [10,9] => [1,2,3,4,5,6,7,8,9
-  r yaitu 4 == mid_right yaitu 4 alias habis, maka jalankan yang kiri : [1,2,3,4,5,6,7,8,9,10]

simulasi fungsi basic (bottom_up_merge_sort) :
- [10,6,8,3,2,4,1,9,5,7] 

- sub_array_size = 1 :
- start_index = 0 , end_idx = 2, merge[10][6] => [6,10]
- start_index = 2 , end_idx = 4, merge[8][3] => [3,8]
- start_index = 4 , end_idx = 6, merge[2][4] => [2,4]
- start_index = 6 , end_idx = 8, merge[1][9] => [1,9]
- start_index = 8 , end_idx = 10, merge[5][7] => [5,7]

- sub_array_size = 2;
- start_index = 0, end_idx = 4,merge[6,10][3,8] = [3,6,8,10]
- start_index = 4, end_idx = 8,merge[2,4][1,9] = [1,2,4,9]
- start_index = 8, end_idx = -, sisa [5,7]

- sub_array_size = 4;
- start_index = 0, end_idx = 8, merge[3,6,8,10][1,2,4,9] = [1,2,3,4,6,8,9,10]
- start_index = 8, end_idx = -, sisa[5,7]

- sub_array_size = 8;
- start_index = 0, end_idx =  10, merge[1,2,3,4,6,8,9,10][5,7] = [1,2,3,4,5,6,7,8,9,10]

 
*/
