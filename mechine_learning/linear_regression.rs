fn main() {
    println!("hello world")
}

pub fn linear_regression(data_points: Vec<(f64, f64)>) -> Option<(f64, f64)> {
    if data_points.is_empty(){
        return None;
    }

    // hitung rata-rata  x dan y
    let count = data_points.len() as f64;
    let mean_x = data_points.iter().fold(0.0, |sum, y| sum + y.0) / count;
    let mean_y = data_points.iter().fold(0.0, |sum, y| sum + y.1) / count;
    
    let mut covariance = 0.0;
    let mut std_dev_sqr_x = 0.0;
    let mut std_dev_sqr_y = 0.0;
  
    // 3. Hitung covariance dan variance (sum of squared deviations)
    for data_point in data_points {
        covariance += (data_point.0 - mean_x) * (data_point.1 - mean_y);  // Σ[(x_i - μ_x)(y_i - μ_y)]
        std_dev_sqr_x += (data_point.0 - mean_x).powi(2); // Σ[(x_i - μ_x)²]
        std_dev_sqr_y += (data_point.1 - mean_y).powi(2);  // Σ[(y_i - μ_y)²]
    }
   
    let std_dev_x = std_dev_sqr_x.sqrt();
    let std_dev_y = std_dev_sqr_y.sqrt();
    let std_dev_prod = std_dev_x * std_dev_y;

    // koefisien korelasi, r = covariance / (a_x * a_y)
    let pcc = covariance / std_dev_prod;

    // slope (b) dan intersep  (a) untuk y = a + b.x
    // b = r * (a_y / a_x)
    let b = pcc * (std_dev_y / std_dev_x);
    // a = b_y - b . b_x
    let a = mean_y - b * mean_x;
    
    Some((a, b))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_linear_regression(){
        let test_regression =  linear_regression(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]);
        println!("{:?}", test_regression);
        // output : Some((2.220446049250313e-16, 0.9999999999999998))
    }
    #[test]
    fn test_empty_linear_regression(){
        let test_none_linear_regression = linear_regression(vec![]);
        println!("{:?}", test_none_linear_regression);
        // output : None
    }
}

/* 
  linear regression adalah metode paling sederhana dan mendasar untuk memodelkan hubungan antara variable input (fitur) dan 
variable output (target).
- y = a*x + b 
y = prediksi
x = input
a = kemiringan garis (koefisien)
b = intersep

rumus untuk menghitung alpha a dan beta b:
a = n sigma (xiyi) - (sigma ni) (sigma yi) / n sigma(xi^2) - (sigma xi)^2
b = rata-rata dari semua nilai yi (yaitu sigma yi /n) - a . rata-rata dari semua nilai xi( sigma yi / n)

*/
