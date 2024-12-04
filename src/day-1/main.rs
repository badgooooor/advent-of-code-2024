use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    // Get arguments
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    let mut first_array: Vec<i32> = Vec::new();
    let mut second_array: Vec<i32> = Vec::new();

    // Read line
    let file = File::open(file_location_arg)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let numbers: Result<Vec<i32>, _> = line?.split_whitespace().map(str::parse).collect();

        match numbers {
            Ok(nums) if nums.len() == 2 => {
                first_array.push(nums[0]);
                second_array.push(nums[1]);
            }
            _ => println!("Invalid line format"),
        }
    }

    first_array.sort_by(|a, b| a.abs().cmp(&b.abs()));
    second_array.sort_by(|a, b| a.abs().cmp(&b.abs()));

    let result_1 = solve_1(&first_array, &second_array);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong"),
    }

    Ok(())
}

fn solve_1(first_array: &Vec<i32>, second_array: &Vec<i32>) -> Result<i32, i32> {
    let mut total: i32 = 0;

    if first_array.len() != second_array.len() {
        return Err(0);
    }

    for i in 0..first_array.len() {
        let diff = (first_array[i] - second_array[i]).abs();
        total += diff;
    }

    return Ok(total);
}
