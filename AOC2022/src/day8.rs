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

pub fn is_visible_from_top(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> bool
{
    let current_value = grid[row_nr][col_nr];
    for i in 0..row_nr
    {
        if grid[i][col_nr] >= current_value
        {
            return false;
        }
    }
    return true;
}

pub fn is_visible_from_bottom(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> bool
{
    let current_value = grid[row_nr][col_nr];
    for i in row_nr + 1..grid.len()
    {
        if grid[i][col_nr] >= current_value
        {
            return false;
        }
    }
    return true;
}

pub fn is_visible_from_left(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> bool
{
    let current_value = grid[row_nr][col_nr];
    for i in 0..col_nr
    {
        if grid[row_nr][i] >= current_value
        {
            return false;
        }
    }
    return true;
}

pub fn is_visible_from_right(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> bool
{
    let current_value = grid[row_nr][col_nr];
    for i in col_nr + 1..grid[row_nr].len()
    {
        if grid[row_nr][i] >= current_value
        {
            return false;
        }
    }
    return true;
}

pub fn get_amount_visible_trees(grid: &Vec<Vec<u32>>) -> u32
{
    let mut amount_visible_trees = 0;
    for i in 0..grid.len()
    {
        for j in 0..grid[i].len()
        {
            if is_visible_from_left(i, j, &grid)
            || is_visible_from_right(i, j, &grid)
            || is_visible_from_top(i, j, &grid)
            || is_visible_from_bottom(i, j, &grid)
            {
                amount_visible_trees += 1;
            }
        }
    }
    return amount_visible_trees;
}

pub fn day8_part1() -> u32
{
    let reader = get_file_reader_for("\\inputs\\day8.txt");

    let mut grid = Vec::new();
    
    // get grid
    for line in reader.lines() // Ignore errors.
    {
        let mut row_from_grid = Vec::new();
        let row = line.unwrap().clone();
        let chars_from_line = row.chars();
        for c in chars_from_line
        {
            row_from_grid.push(c.to_digit(10).unwrap());
        }
        grid.push(row_from_grid);
    }

    let mut total_amount_trees_visible = 0;
    total_amount_trees_visible = get_amount_visible_trees(&grid);

    return total_amount_trees_visible;
}



// ================= part 2 ====================//

pub fn get_scenic_score_for_top(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> u32
{
    let current_value = grid[row_nr][col_nr];
    let mut blocked_at_x_trees = 1;
    let start_at_row = row_nr - 1; // TODO dit fixen, substract overflow bij 0
    for i in (0..start_at_row).rev()
    {
        if grid[i][col_nr] < current_value
        {
            blocked_at_x_trees += 1;
        }
        else 
        {
            return blocked_at_x_trees;
        }
    }
    return blocked_at_x_trees;
}

pub fn get_scenic_score_for_bottom(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> u32
{
    let current_value = grid[row_nr][col_nr];
    let mut blocked_at_x_trees = 1;
    let start_at_row = row_nr + 1;
    for i in start_at_row..grid.len()
    {
        if grid[i][col_nr] < current_value
        {
            blocked_at_x_trees += 1;
        }
        else 
        {
            return blocked_at_x_trees;
        }
    }
    return blocked_at_x_trees;
}

pub fn get_scenic_score_for_left(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> u32
{
    let current_value = grid[row_nr][col_nr];
    let mut blocked_at_x_trees = 1;
    let start_at_col = col_nr - 1;
    for i in (0..start_at_col).rev()
    {
        if grid[row_nr][i] < current_value
        {
            blocked_at_x_trees += 1;
        }
        else 
        {
            return blocked_at_x_trees;
        }
    }
    return blocked_at_x_trees;
}

pub fn get_scenic_score_for_right(row_nr: usize, col_nr: usize, grid: &Vec<Vec<u32>>) -> u32
{
    let current_value = grid[row_nr][col_nr];
    let mut blocked_at_x_trees = 1;
    let start_at_col = col_nr + 1;
    for i in start_at_col..grid[row_nr].len()
    {
        if grid[row_nr][i] < current_value
        {
            blocked_at_x_trees += 1;
        }
        else 
        {
            return blocked_at_x_trees;
        }
    }
    return blocked_at_x_trees;
}


pub fn get_best_scenic_score(grid: &Vec<Vec<u32>>) -> u32
{
    let mut biggest_scenic_score = 0;
    let mut total_scenic_score = 1;
    for i in 0..grid.len()
    {
        for j in 0..grid[i].len()
        {
            total_scenic_score = 1;
            total_scenic_score *= get_scenic_score_for_top(i, j, &grid);
            total_scenic_score *= get_scenic_score_for_bottom(i, j, &grid);
            total_scenic_score *= get_scenic_score_for_left(i, j, &grid);
            total_scenic_score *= get_scenic_score_for_right(i, j, &grid);
            if total_scenic_score > biggest_scenic_score
            {
                biggest_scenic_score = total_scenic_score;
            }
        }
    }
    return biggest_scenic_score;
}

pub fn day8_part2() -> u32
{
    let reader = get_file_reader_for("\\inputs\\day8.txt");

    let mut grid = Vec::new();
    
    // get grid
    for line in reader.lines() // Ignore errors.
    {
        let mut row_from_grid = Vec::new();
        let row = line.unwrap().clone();
        let chars_from_line = row.chars();
        for c in chars_from_line
        {
            row_from_grid.push(c.to_digit(10).unwrap());
        }
        grid.push(row_from_grid);
    }

    return get_best_scenic_score(&grid);;
}
