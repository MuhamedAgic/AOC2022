
pub mod day1_challenge1;
pub mod day2_challenge1;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub use day1_challenge1::*;
pub use day2_challenge1::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;
pub use day6::*;
pub use day7::*;
pub use day8::*;
pub use day9::*;





fn main() 
{
    let day8_part1 = day9::day9_part1();
    println!("{}", day8_part1);
}
