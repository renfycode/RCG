use std::convert::TryInto;



pub fn try_into_example() {
    let a: i32 = 10;
    let b: u16 = 100;
    let b_: i32  = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred");
    }
}
