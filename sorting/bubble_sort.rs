mod sorting;

fn main() {
    println!("Hello, world!");
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]){
  // mengembalikan langsung jika arraynya kosong
    if arr.is_empty(){
        return;
    }
    // membuat patokan awal
    let mut sorted = false;
    let mut n = arr.len();
    // selama sorted bernilai false, lakukan loop
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1]{
                arr.swap(i, i + 1);  //index i tukar dengan index i + 1
                sorted = false; // ubah sorted menjadi false agar bisa melakukan perulangan
            }
        }
        n -= 1;
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;
    
    #[test]
    fn descending(){
        let mut ve1 = vec![9, 6, 7, 3, 4, 1];
        let cloned = ve1.clone();
        bubble_sort(&mut ve1);
        println!("{:?}", ve1);
        println!("{:?}", is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }
    #[test]
    fn asscending(){
        let mut ve2 = vec![1,2,3,4,5,6];
        let cloned = ve2.clone();
        bubble_sort(&mut ve2);
        println!("{:?}", ve2);
        println!("{:?}", is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
    #[test]
    fn empty(){
        let mut ve3: Vec<usize> = vec![];
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        println!("{:?}", ve3);
        println!("{:?}", is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}

/*  
penjelasan :
bubble sort adalah algoritma penyortiran sederhana dengan cara "menggelebungkan" elemen-elemen terbesar (atau sebaliknya) ke
posisi akhirnya dengan cara membandingkan satu persatu dan menukarkan satu persatu. 
- cara membandingkan :
  yaitu ambil elemen ke-i dengan ke i + 1

  gunakan bubble sort jika ingin implementasi super simple dan data relatif kecil serta kecepatan bukan isu utama
  
*/
