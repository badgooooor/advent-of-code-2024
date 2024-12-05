use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    // Get arguments
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    // Read line
    let content = fs::read_to_string(file_location_arg)?;

    // Extract from line
    let multiples = extract_multiples(&content);

    // Solve part 1
    let result_1 = solve_1(&multiples);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    // Solve part 2
    let result_2 = solve_2(&content);
    match result_2 {
        Ok(total) => println!("Result 2: {}", total),
        _ => println!("Something wrong in result 2"),
    }

    Ok(())
}

fn solve_1(multiples: &Vec<(i32, i32, i32)>) -> Result<i32, i32> {
    let total: i32 = multiples.iter().map(|(x, y, _)| x * y).sum();

    Ok(total)
}

fn solve_2(content: &String) -> Result<i32, i32> {
    let re = Regex::new(r"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))")
        .unwrap();

    let mut res = 0;
    let mut valid = true;
    for m in re.captures_iter(content) {
        if let Some(_) = m.name("d") {
            valid = true;
        } else if let Some(_) = m.name("dt") {
            valid = false;
        }
        if valid {
            if let (Some(op1), Some(op2)) = (m.name("op1"), m.name("op2")) {
                res += op1.as_str().parse::<i32>().unwrap() * op2.as_str().parse::<i32>().unwrap();
            }
        }
    }

    Ok(res)
}

// Utility: Extract numbers from pattern
fn extract_multiples(text: &String) -> Vec<(i32, i32, i32)> {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    let mut multiples: Vec<(i32, i32, i32)> = Vec::new();
    // Find all matches
    for capture in re.find_iter(&text) {
        let line_str = capture.as_str();
        let numbers = &line_str[4..line_str.len() - 1];
        let mut parts = numbers.split(',');

        let first_number: i32 = parts.next().unwrap().parse().unwrap();
        let second_number: i32 = parts.next().unwrap().parse().unwrap();

        multiples.push((
            first_number,
            second_number,
            i32::try_from(capture.start()).unwrap(),
        ));
    }

    multiples
}
