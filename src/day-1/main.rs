use std::collections::HashMap;
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

    // Solve part 1
    let result_1 = solve_1(&first_array, &second_array);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    let result_2 = solve_2(&first_array, &second_array);
    match result_2 {
        Ok(total) => println!("Result 2: {}", total),
        _ => println!("Something wrong in result 2"),
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

fn solve_2(first_array: &Vec<i32>, second_array: &Vec<i32>) -> Result<i32, i32> {
    let mut total: i32 = 0;

    if first_array.len() != second_array.len() {
        return Err(0);
    }

    let freq_map = count_frequencies(second_array);

    for i in 0..first_array.len() {
        match freq_map.get(&first_array[i]) {
            Some(item_frequency) => {
                let item_product = first_array[i] * item_frequency;
                total += item_product;
            }
            None => {
                total += 0;
            }
        }
    }

    return Ok(total);
}

// Utilities: Counting frequency
fn count_frequencies(numbers: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::new();

    for &num in numbers {
        *map.entry(num).or_insert(0) += 1;
    }

    map
}
