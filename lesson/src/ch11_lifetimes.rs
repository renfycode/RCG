// fn longest(s1: &str, s2: &str) -> &str {
//     if s1.len() > s2.len() {
//         s1
//     } else {
//         s2
//     }
// }

// pub fn check_lifetimes() {
//     let string1 = String::from("long string");
//     let string2 = String::from("short");

//     // Проблема: функции ожидают ссылки, которые живут не менее 'a
//     let result = longest(&string1, &string2);
//     println!("Самая длинная строка: {}", result);
//     println!("{}, {}" , string1, string2);
// }

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn check_lifetimes() {
    let string1 = String::from("long string");
    let string2 = String::from("short");

    // Проблема: функции ожидают ссылки, которые живут не менее 'a
    let result = longest(&string1, &string2);
    println!("Самая длинная строка: {}", result);
    println!("{}, {}" , string1, string2);
}

