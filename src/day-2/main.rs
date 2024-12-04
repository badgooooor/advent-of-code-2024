use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    // Get arguments
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    let mut level_reports: Vec<Vec<i32>> = Vec::new();

    // Read line
    let file = File::open(file_location_arg)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let numbers: Result<Vec<i32>, _> = line?.split_whitespace().map(str::parse).collect();

        match numbers {
            Ok(nums) => {
                level_reports.push(nums);
            }
            _ => println!("Invalid line format"),
        }
    }

    // Solve part 1
    let result_1 = solve_1(&level_reports);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    Ok(())
}

fn solve_1(level_reports: &Vec<Vec<i32>>) -> Result<i32, i32> {
    let mut total: i32 = 0;

    for i in 0..level_reports.len() {
        let level_report = &level_reports[i];
        let is_level_safe = check_is_level_safe(&level_report);

        match is_level_safe {
            Ok(result) => {
                if result {
                    total += 1;
                }
            }
            Err(_result) => println!("Error at line {}", i),
        }
    }

    Ok(total)
}

fn check_is_level_safe(level_report: &Vec<i32>) -> Result<bool, bool> {
    let mut is_increase: bool = false;
    let mut is_decrease: bool = false;
    let mut is_stable: bool = false;
    let mut is_diff_above_threshold: bool = false;

    for i in 1..level_report.len() {
        if level_report[i - 1] < level_report[i] {
            is_increase = true;
        }

        if level_report[i - 1] > level_report[i] {
            is_decrease = true;
        }

        let diff = level_report[i - 1] - level_report[i];
        if i32::abs(diff) > 3 {
            is_diff_above_threshold = true;
        }

        if i32::abs(diff) == 0 {
            is_stable = true;
        }
    }

    let is_level_move_one_direction = (is_increase || is_decrease) && !(is_increase && is_decrease);
    Ok(!is_diff_above_threshold && is_level_move_one_direction && !is_stable)
}
