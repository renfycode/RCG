

pub fn pointers() {
    let a = 42;
    let r = &a;
    let b = a + *r;
    println!("{} + {} = {}", a, r, b);
}

pub fn pointers_benefits(arr: &[i32; 10]) -> i32 {
    let mut _sum = 0;
    for &num in arr.iter() {
        _sum += num;
    }
    _sum
}


pub fn pointers_benefits_test() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum1 = pointers_benefits(&arr);
    println!("sum: {}", sum1);
}
