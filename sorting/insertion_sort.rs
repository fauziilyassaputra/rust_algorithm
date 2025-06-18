mod sorting;
fn main() {
    println!("hello world")
}

pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]){
  // mulai dengan indeks ke-1 (karena indeks 0 dianggap sudah terurut
    for i in 1..arr.len(){
        let mut j = i;
        let cur = arr[i];
        // selama masih ada elemen sebelum j dan nilai sekarang lebih kecil dibanding elemen sebelum j, geser elemen ke kanan
        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = cur;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;
    
    #[test]
    fn basic(){
        let mut arr = ["d", "a", "c", "b","f","e"];
        let cloned = arr;
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        println!("{:?}", is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn already_sorted(){
        let mut arr: [&str; 3] = ["a","b","c"];
        let cloned = arr;
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        println!("{:?}", is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
    #[test]
    fn repeated_element(){
        let mut arr: Vec<usize> = vec![542,542,542];
        let cloned = arr.clone();
        insertion_sort(&mut arr);
        println!("{:?}", arr);
        println!("{:?}", is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
    
}

/* 
simulasi fungsi basic :
1. [d,a,c,b,f,e]
2. [a,d,c,b,f,e]
3. [a,c,d,b,f,e]
4. [a,c,b,d,f,e]
5. [a,b,c,d,f,e]
6. [a,b,c,d,e,f]

penjelasan :
    insertion sort adalah algoritma sorting sederhana seperti menyusun kartu ke tangan yaitu menyisipkan satu per satu elemen
  ke posisi yang tepat kebagian yang sudah terurut.
cara kerjanya :
    1. mulai dari elemen kedua dalam array
    2. bandingkan dengan elemen sebelumnya
    3. geser elemen yang lebih besar kesebelah kanan
    4. ulangi sampai semua data terurut

  algoritma ini mudah dipahami dan di implementasikan , gunakan insertion_sort untuk dataset kecil atau hampir terurut,  
*/
