use num::complex::Complex;

pub fn complex_example() {
    let a: Complex<f64> = Complex { re: 2.1, im: -1.2};
    let b: Complex<f64> = Complex::new(11.1, 22.2);
    let result: Complex<f64> = a + b;

    println!("{} + {}i", result.re, result.im);
}
