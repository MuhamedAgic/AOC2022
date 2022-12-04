
pub mod day1_challenge1;
pub mod day2_challenge1;
pub mod day3;
pub mod day4;

pub use day1_challenge1::*;
pub use day2_challenge1::*;
pub use day3::*;
pub use day4::*;





fn main() 
{
    let result_day_4_challenge2 = day4::day4_challenge2();
    println!("{}", result_day_4_challenge2);
}
