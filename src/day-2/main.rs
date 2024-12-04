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

    let result_2 = solve_2(&level_reports);
    match result_2 {
        Ok(total) => println!("Result 2: {}", total),
        _ => println!("Something wrong in result 2"),
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

// Loop through level_reports
// If at first is not safe, create level report of pick one array out and check result
// If some is safe, total += 1
fn solve_2(level_reports: &Vec<Vec<i32>>) -> Result<i32, i32> {
    let mut total: i32 = 0;

    for i in 0..level_reports.len() {
        let level_report = &level_reports[i];
        let is_level_safe = check_is_level_safe(&level_report);

        let mut recheck_safe_level_from_pick: bool = false;
        let mut is_level_from_pick_safe: bool = false;

        match is_level_safe {
            Ok(result) => {
                if result {
                    total += 1;
                } else {
                    recheck_safe_level_from_pick = true;
                }
            }
            Err(_result) => println!("Error at line {}", i),
        }

        if recheck_safe_level_from_pick {
            let pick_level_reports = generate_combinations(&level_report);

            for i in 0..pick_level_reports.len() {
                let pick_level_report = &pick_level_reports[i];
                let is_picked_level_safe = check_is_level_safe(&pick_level_report);

                match is_picked_level_safe {
                    Ok(result) => {
                        if result {
                            is_level_from_pick_safe = true;
                        }
                    }
                    Err(_result) => println!("Error at line {}", i),
                }
            }

            if is_level_from_pick_safe {
                total += 1;
            }
        }
    }
    Ok(total)
}

// Utility: Check level safety
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

// Utility: Create combination
fn generate_combinations(numbers: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for i in 0..numbers.len() {
        let mut combination = Vec::new();
        for j in 0..numbers.len() {
            if i != j {
                combination.push(numbers[j]);
            }
        }
        result.push(combination);
    }

    result
}
