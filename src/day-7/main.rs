use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    let file = File::open(file_location_arg)?;

    let calibrations = get_calibrations(&file);

    let result_1 = solve_1(&calibrations);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    let result_2 = solve_2(&calibrations);
    match result_2 {
        Ok(total) => println!("Result 2: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    Ok(())
}

fn solve_1(calibrations: &Vec<(u64, Vec<u64>)>) -> Result<u64, u64> {
    let sum = Arc::new(AtomicU64::new(0));

    calibrations.into_par_iter().for_each(|calibration| {
        let sum = Arc::clone(&sum);
        let calibration_count = calibration.1.len() - 1;
        let max_operator = 2usize.pow(calibration_count as u32);

        for operator in 0..max_operator {
            let mut result = calibration.1[0];
            for i in 0..calibration_count {
                if (operator >> i) & 1 == 0 {
                    result += calibration.1[i + 1];
                } else {
                    result *= calibration.1[i + 1];
                }
            }
            if result == calibration.0 {
                sum.fetch_add(calibration.0, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    });

    let sum_value = sum.load(std::sync::atomic::Ordering::SeqCst);
    Ok(sum_value)
}

fn solve_2(calibrations: &Vec<(u64, Vec<u64>)>) -> Result<u64, u64> {
    let sum = Arc::new(AtomicU64::new(0));

    calibrations.into_par_iter().for_each(|calibration| {
        let sum = Arc::clone(&sum);
        let calibration_count = calibration.1.len() - 1;
        let max_operator = 3usize.pow(calibration_count as u32);

        for operator in 0..max_operator {
            let mut result = calibration.1[0];
            for i in 0..calibration_count {
                match operator / 3usize.pow(i as u32) % 3 {
                    0 => result += calibration.1[i + 1],
                    1 => result *= calibration.1[i + 1],
                    _ => {
                        result = format!("{}{}", result, calibration.1[i + 1])
                            .parse()
                            .unwrap()
                    }
                }
            }
            if result == calibration.0 {
                sum.fetch_add(calibration.0, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    });

    let sum_value = sum.load(std::sync::atomic::Ordering::SeqCst);
    Ok(sum_value)
}

fn get_calibrations(file: &File) -> Vec<(u64, Vec<u64>)> {
    let reader = io::BufReader::new(file);

    let mut calibrations: Vec<(u64, Vec<u64>)> = Vec::new();

    for line in reader.lines() {
        if let Ok(line_content) = line {
            let splited_value_calibration: Vec<String> =
                line_content.split(":").map(String::from).collect();

            let result: u64 = splited_value_calibration[0].parse::<u64>().unwrap();

            let item_calibrations: Vec<u64> = splited_value_calibration[1]
                .split_whitespace()
                .map(String::from)
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            calibrations.push((result, item_calibrations));
        }
    }

    calibrations
}
