
pub mod day1_challenge1;
pub mod day2_challenge1;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub use day1_challenge1::*;
pub use day2_challenge1::*;
pub use day3::*;
pub use day4::*;
pub use day5::*;
pub use day6::*;





fn main() 
{
    let day6_part1 = day6::day6_part1();
    println!("{}", day6_part1);
}
