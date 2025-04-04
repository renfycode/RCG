#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

pub fn sum_something() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000;
    println!("one million = {}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0f32];

    println!("{:02}", forty_twos[0]);
}

pub fn print_same_bases() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 300;

    println!("base: 10 {}, {}, {}", three, thirty, three_hundred);
    println!("base: 2  {:b}, {:b}, {:b}", three, thirty, three_hundred);
    println!("base: 8  {:o}, {:o}, {:o}", three, thirty, three_hundred);
    println!("base: 16 {:x}, {:x}, {:x}", three, thirty, three_hundred);
}
pub fn check_examples() {
    let mut grains: Vec<Cereal> = Vec::new();
    grains.push(Cereal::Rye);
    grains.push(Cereal::Wheat);
    grains.push(Cereal::Rice);
    grains.push(Cereal::Barley);
    grains.push(Cereal::Spelt);
    grains.push(Cereal::Millet);
    println!("{:?}", grains);

    print_same_bases();
    sum_something();
}
