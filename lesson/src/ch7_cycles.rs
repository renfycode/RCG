

pub fn i_try_cycles() {
    let mut counter = 0;
    for i in 0..10 {
        println!("i: {}", i);
        counter += 1;
        if counter == 5 {
            println!("counter: {}", counter);
    }
}
}


pub fn binary_search(target: i32, array: &[i32]) -> i32 {
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if array[mid] == target {
            return mid as i32;
        }
        if array[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return -1;
}

pub fn test_my_binary_search_algo() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(binary_search(5, &array), 4);
    assert_eq!(binary_search(1, &array), 0);
    assert_eq!(binary_search(10, &array), 9);
}


fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn check_is_even_number() {
    let n = 123456;
    let description = if is_even(n) {"even"} else {"odd"};
    println!("{} is {}", n, description);
}

fn check_is_even_number_match_version() {
    let n = 654321;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };
    println!("{} is {}", n, description);
}


fn more_match_example() {
    let item = 42;
    match item {
        0 => println!("zero"),
        1 => println!("one"),
        1..=10 => println!("1 to 10"),
        _ => println!("something else"),
    }
}
