use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    // Get arguments
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    // Read line
    let content = fs::read_to_string(file_location_arg)?;
    println!("Contents: {}", content);

    let multiples = extract_multiples(&content);

    // Solve part 1
    let result_1 = solve_1(&multiples);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    Ok(())
}

fn solve_1(multiples: &Vec<(i32, i32)>) -> Result<i32, i32> {
    let total: i32 = multiples.iter().map(|(x, y)| x * y).sum();

    Ok(total)
}

// Utility: Extract numbers from pattern
fn extract_multiples(text: &String) -> Vec<(i32, i32)> {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    let mut multiples: Vec<(i32, i32)> = Vec::new();
    // Find all matches
    for capture in re.find_iter(&text) {
        let line_str = capture.as_str();
        println!("Found match: {}", line_str);

        let numbers = &line_str[4..line_str.len() - 1];
        let mut parts = numbers.split(',');

        let first_number: i32 = parts.next().unwrap().parse().unwrap();
        let second_number: i32 = parts.next().unwrap().parse().unwrap();

        println!("Numbers: {} {}", first_number, second_number);

        multiples.push((first_number, second_number));
    }

    multiples
}
