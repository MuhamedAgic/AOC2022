
pub mod day1_challenge1;

pub use day1_challenge1::*;



fn main() {
    println!("Hello, world!");
    let result_day_1_challenge1 = day1_challenge1::day1_challenge1();
    let result_day_1_challenge2 = day1_challenge1::day1_challenge2();
    println!("{}", result_day_1_challenge1);
    println!("{}", result_day_1_challenge2);
}
