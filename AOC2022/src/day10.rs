use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_file_reader_for(filename: &str) -> BufReader<File>
{
    // Lees bestand uit
    let current_dir = std::env::current_dir().unwrap()
                                             .to_str()
                                             .unwrap()
                                             .to_owned();

    let path_to_input = current_dir + filename;

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(path_to_input).unwrap();
    let reader = BufReader::new(file);

    return reader;
}


// lelijk, ik weet het
pub fn day10_part1() -> i32
{
    let reader = get_file_reader_for("\\AOC2022\\inputs\\day10.txt");

    let mut current_cycle = 1;
    let mut value_of_x = 1;
    let mut values_of_x_on_checkpoints = Vec::new();
    for line in reader.lines() // Ignore errors.
    {
        let unsplit_line =  line.unwrap();
        if unsplit_line == "noop"
        {
            if current_cycle == 20
            || current_cycle == 60
            || current_cycle == 100
            || current_cycle == 140
            || current_cycle == 180
            || current_cycle == 220
            {
                values_of_x_on_checkpoints.push(current_cycle * value_of_x);
                println!("Before noop cycle, added cycles: {}, times value of x now {}, which is {}", current_cycle, value_of_x, current_cycle * value_of_x);
            }
            current_cycle += 1;
        }
        else
        {
            if current_cycle == 20
            || current_cycle == 60
            || current_cycle == 100
            || current_cycle == 140
            || current_cycle == 180
            || current_cycle == 220
            {
                values_of_x_on_checkpoints.push(current_cycle * value_of_x);
                println!("Before addx cycle, added cycles: {}, times value of x now {}, which is {}", current_cycle, value_of_x, current_cycle * value_of_x);
            }
            current_cycle += 1;

            if current_cycle == 20
            || current_cycle == 60
            || current_cycle == 100
            || current_cycle == 140
            || current_cycle == 180
            || current_cycle == 220
            {
                values_of_x_on_checkpoints.push(current_cycle * value_of_x);
                println!("After 1 addx cycle, added cycles: {}, times value of x now {}, which is {}", current_cycle, value_of_x, current_cycle * value_of_x);
            }

            let split_line = unsplit_line.split_whitespace();
            for str in split_line
            {
                let number_to_add = str.trim().parse::<i32>();
                match number_to_add 
                {
                    Ok(number) => value_of_x += number,
                    Err(e) => () 
                }
            }
            current_cycle += 1;
        }
        println!("Operation complete: {} \n    now at cycle: {}\n    with value of x: {}", unsplit_line, current_cycle, value_of_x);
    }

    return values_of_x_on_checkpoints.into_iter().sum();
}
