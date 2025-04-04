pub fn icheckepsilon(){
    let result: f32 = 0.1 + 0.2;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    print!("absolute_difference: {}, f32::EPSILON: {}", absolute_difference, f32::EPSILON);
    // assert!(absolute_difference <= f32::EPSILON); // its a mistake example
}

pub fn check_examples() {
    icheckepsilon();
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);    //<1>
    // assert!(xyz.0 + xyz.1 == xyz.2);    // its a mistake example
}
