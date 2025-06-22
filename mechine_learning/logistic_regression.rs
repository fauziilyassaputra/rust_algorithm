mod optimization;

fn main() {
    println!("hello world")
}

/* 
rumus logistic_regression (fungsi sigmoid) : 
p(y = 1 | x)  = 1 / (1 + e^-(b + w1x1 + w2x2 + wnxn))

penjelasan:
x = data (seperti data genre / durasi pada film)
w = bobot (seberapa pentingkah genre / durasi ?)
b = titik awal bias
e = konstanta euler (2.17828)
*/

use optimization::gradient_descent;
use std::f64::consts::E;

pub fn logistic_regression (
    data_point: Vec<(Vec<f64>, f64)>,
    iterations: usize,
    learning_rate: f64,
)->  Option<Vec<f64>> {
    if data_point.is_empty(){
        return None;
    }
    //  hitung jumlah parameter , yaitu panjang vector fitur + 1 untuk bias(intersept)
    let num_features = data_point[0].0.len() +1;

    // inisialisasi params -> [bias, w1, w2, ...], semua dimulai dari nol
    let mut params = vec![0.0; num_features];

    // bungkus fungsi turunan (gradient) ke dalam closure :
    let derivative_fn = |params: &[f64]| derivative(params, &data_point);
    
    gradient_descent(derivative_fn, &mut params, learning_rate, iterations as i32);
    
    Some(params)
    
}


fn derivative (params: &[f64],data_points: &[(Vec<f64>, f64)])  -> Vec<f64> {
    let num_features = params.len();
    // siapkan gradients awalnya nol
    let mut gradients = vec![0.0; num_features];
    
    for (features, y_i) in data_points {
        // z = b + w.x
        let z = params[0]
        + params[1..]
            .iter()
            .zip(features)
            .map(|(p, x)| p * x)
            .sum::<f64>();
        // prediksi dengan sigmoid: a(z) = 1 / (1 + e^(-z))
        let prediction = 1.0 / (1.0 + E.powf(-z));
        
        gradients[0] = prediction - y_i;
        for (i, x_i) in features.iter().enumerate(){
            gradients[i + 1] += (prediction- y_i) * x_i;
        }
    }
    gradients
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_logistic_simple(){
        let data = vec![
            (vec![0.0], 0.0), // x = 0 -> y = 0
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 1.0),
            (vec![4.0], 1.0),
            (vec![5.0], 1.0),
        ];
        let result = logistic_regression(data, 10000, 0.05);
        println!("{:?}", result.is_some());
        let params = result.unwrap();
        println!("{:?}", (params[0] + 17.65).abs() < 1.0);
        println!("{:?}", (params[1] - 7.13).abs() < 1.0);
    }
    
    #[test]
    fn test_logistic_regression_no_data(){
        let result = logistic_regression(vec![], 5000, 0.1);
        println!("{:?}", result)
    }
}


/*
  logistic regression adalah salah satu algoritma paling sederhana dan populer dalam mechine learning. logistic
regression memprediksi probabilitas bahwa sebuah input termasuk dalam satu kelas tertentu. prosesnya melibatkan
dua langkah utama :
1. transformasi linear (untuk mendapatkan nilai zeta)
2. fungsi sigmoid (memetakan nilai ke rentang 0 dan 1)

  jika probabilitas melebihi ambang batas (biasanya 0.5), maka input diklasifikasikan sebagai kelas 1, dan sebaliknya
sebagai kelas 0

kelebihan :
- mudah di implementasikan dan efisien
- cocok untuk dataset yang tidak terlalu besar
keterbatasan :
-  hanya mampu memodelkan batas keputusan linier
-  mungkin underfiting pada data yang sangat kompleks

*/
