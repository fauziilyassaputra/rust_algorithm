
fn main() {
    println!("hello world")
}

pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    None
}

#[test]
fn test_searching(){
    let  kosong = ['_'];
    let cari_data_awal = ["rust","b","c"];
    let cari_data_tengah = ["a","b","c","rust","d","e"];
    let cari_data_akhir = ["a","b","c","d","e","rust"];
    let cari_data_tidak_tahu = ["a","b","c","d","e"];
    
    match linear_search(&"rust", &cari_data_tidak_tahu){
        Some(idx) => println!("kata rust ditemukan di indeks {}", idx),
        None      => println!("kata rust tidak ditemukan"),
    }
    
}

/* 
penjelasan :
  liear search adalah algoritma pencarian paling dasar. ia "menyisir" elemen satu persatu, dari posisi pertama sampai akhir,
untuk mencari nilai yang cocok.
gambaran umum :
  1.mulai dari index 0,
  2.bandingkan elemen pada indeks itu dengan nilai target
  3.kalau tidak cocok, lanjut ke index selanjutnya
  4.kalau cocok, hentikan dan laporkan lokasi 

keunggulan linear search adalah datanya tidak perlu terurut, implementasinya ringkas (hanya loop dan perbandingan), serta
cocok untuk koleksi yang berukuran kecil.

tidak efisien untuk dataset besar atau banyak pencarian

*/
