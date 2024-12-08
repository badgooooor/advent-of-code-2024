use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    // Get arguments
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    let mut content: Vec<String> = Vec::new();

    // Read line
    let file = File::open(file_location_arg)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => content.push(line),
            Err(_) => println!("Invalid line"),
        }
    }

    // Solve part 1
    let result_1 = solve_1(&content);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    Ok(())
}

fn solve_1(content: &Vec<String>) -> Result<i32, i32> {
    // Horizontal
    let horizontal_count = match_line_horizontal_count(content);
    let horizontal_reverse_count = match_line_horizontal_reverse_count(content);

    // Vertical
    let columns: Vec<String> = create_columns(&content);
    let vertical_count = match_line_horizontal_count(&columns);
    let vertical_reverse_count = match_line_horizontal_reverse_count(&columns);

    // Diagonal
    let diagonals: Vec<String> = create_diagonal_left_right(&content);
    let diagonals_count = match_line_horizontal_count(&diagonals);
    let diagonals_reverse_count = match_line_horizontal_reverse_count(&diagonals);

    Ok(horizontal_count?
        + horizontal_reverse_count?
        + vertical_count?
        + vertical_reverse_count?
        + diagonals_count?
        + diagonals_reverse_count?)
}

// Core: Match horizontal
fn match_line_horizontal_count(content: &Vec<String>) -> Result<i32, i32> {
    let mut count: i32 = 0;
    let re = Regex::new(r"XMAS").unwrap();

    for i in 0..content.len() {
        let captures_len = re.find_iter(&content[i]).count();
        count += captures_len as i32;
    }

    Ok(count)
}

fn match_line_horizontal_reverse_count(content: &Vec<String>) -> Result<i32, i32> {
    let mut count: i32 = 0;
    let re = Regex::new(r"SAMX").unwrap();

    for i in 0..content.len() {
        let captures_len = re.find_iter(&content[i]).count();
        count += captures_len as i32;
    }

    Ok(count)
}

// Core: Create vertical row
fn create_columns(content: &Vec<String>) -> Vec<String> {
    let mut columns: Vec<String> = Vec::new();
    let column_string = content.concat();
    let row_length: usize = content[0].len();

    for i in 0..row_length {
        let column: String = column_string.chars().skip(i).step_by(row_length).collect();
        columns.push(column);
    }

    columns
}

// Core: Create diagonal row
fn create_diagonal_left_right(content: &Vec<String>) -> Vec<String> {
    let mut diagonals: Vec<String> = Vec::new();

    let row_length: usize = content[0].len();
    let col_length: usize = content.len();

    // Left-to-right
    for start in 0..row_length {
        let mut item = String::new();
        for (i, j) in (start..row_length).zip(0..col_length) {
            item.push(content[i].chars().nth(j).unwrap());
        }
        diagonals.push(item);
    }

    for start in 1..col_length {
        let mut item = String::new();
        for (i, j) in (0..row_length).zip(start..col_length) {
            item.push(content[i].chars().nth(j).unwrap());
        }
        diagonals.push(item);
    }

    // Right-to-left
    for start in 0..row_length {
        let mut item = String::new();
        for (i, j) in (start..row_length).zip((0..col_length).rev()) {
            item.push(content[i].chars().nth(j).unwrap());
        }
        diagonals.push(item);
    }

    for start in (0..col_length - 1).rev() {
        let mut item = String::new();
        for (i, j) in (0..row_length).zip((0..=start).rev()) {
            item.push(content[i].chars().nth(j).unwrap());
        }
        diagonals.push(item);
    }

    diagonals
}
