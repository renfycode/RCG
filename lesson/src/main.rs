mod ch1_first_step;
mod ch2_bases_and_enums;
mod ch3_try_into;
mod ch4_floats;
mod ch5_nan;
mod ch6_complex_example;
mod ch7_cycles;
mod ch8_time_blazing;
mod ch9_pointers;
mod ch10_mandelbrot;
mod ch11_lifetimes;

fn print_separator(chapter: &str) {
    println!("\n{}", "=".repeat(50));
    println!("==={}===", chapter);
    println!("{}", "=".repeat(50));
    
}

fn main() {
    ch1_first_step::greet_world_enumerate();
    ch1_first_step::greet_world();
    ch1_first_step::make_csv_great();
    print_separator("_____________Examples from ch1_first_step______________");
    
    ch2_bases_and_enums::sum_something();
    ch2_bases_and_enums::print_same_bases();
    ch2_bases_and_enums::check_examples();
    print_separator("_____________Examples from ch2_bases_and_enums_________");

    ch3_try_into::try_into_example();
    print_separator("_____________Examples from ch3_try_into____________");

    ch4_floats::check_examples();
    print_separator("_____________Examples from ch4_floats_____________");

    ch5_nan::check();
    ch5_nan::is_nan_is_not_equal();
    print_separator("_____________Examples from ch5_nan________________");

    ch6_complex_example::complex_example();
    print_separator("_____________Examples from ch6_complex_example______");

    ch7_cycles::i_try_cycles();
    ch7_cycles::test_my_binary_search_algo();
    print_separator("_____________Examples from ch7_cycles_____________");

    ch8_time_blazing::check_blazing();
    print_separator("_____________Examples from ch8_time_blazing_______");

    ch9_pointers::pointers();
    ch9_pointers::pointers_benefits_test();
    print_separator("_____________Examples from ch9_pointers___________");
    
    print_separator("_____________EXAMPLE MANDELBROT___________________");
    let mandelbrot = ch10_mandelbrot::calculate_mandelbrot(1000, -2.5, 1.0, -1.5, 1.5, 100, 24);
    println!("Начало расчёта...");
    ch10_mandelbrot::render_mandelbrot(mandelbrot);

    print_separator("_____________EXAMPLE LIFETIMES___________________");
    ch11_lifetimes::check_lifetimes();

}   

