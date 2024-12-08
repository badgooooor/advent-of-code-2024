use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    // Ordering rule
    let mut order_rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut page_updates: Vec<Vec<i32>> = Vec::new();

    // Read line
    let file = File::open(file_location_arg)?;
    let reader = io::BufReader::new(file);

    let mut read_order_rules = true;

    for line in reader.lines() {
        if read_order_rules {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split('|').collect();
                let num_parts: Result<Vec<i32>, _> =
                    parts.iter().map(|s: &&str| s.parse::<i32>()).collect();

                match num_parts {
                    Ok(vec) => match order_rules_map.get(&vec[0]) {
                        Some(value) => {
                            let mut updated_map: Vec<i32> = value.clone();
                            updated_map.push(vec[1]);

                            order_rules_map.insert(vec[0], updated_map);
                        }
                        None => {
                            let mut updated_map: Vec<i32> = Vec::new();
                            updated_map.push(vec[1]);
                            order_rules_map.insert(vec[0], updated_map);
                        }
                    },
                    Err(_) => read_order_rules = false,
                }
            }
        } else {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(',').collect();
                let num_parts: Result<Vec<i32>, _> =
                    parts.iter().map(|s: &&str| s.parse::<i32>()).collect();
                match num_parts {
                    Ok(vec) => page_updates.push(vec),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }

    let (correct_ordered_pages, _) = group_ordered_pages(&order_rules_map, &page_updates);

    // Solve part 1
    let result_1 = solve_1(&correct_ordered_pages);
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    Ok(())
}

fn solve_1(correct_ordered_pages: &Vec<Vec<i32>>) -> Result<i32, i32> {
    return Ok(correct_ordered_pages
        .iter()
        .map(|vec| vec[vec.len() / 2])
        .sum());
}

fn group_ordered_pages(
    order_rules_map: &HashMap<i32, Vec<i32>>,
    page_updates: &Vec<Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut correct_ordered_pages: Vec<Vec<i32>> = Vec::new();
    let mut incorrect_ordered_pages: Vec<Vec<i32>> = Vec::new();

    for (_, page_update) in page_updates.iter().enumerate() {
        let mut has_correct_order: bool = true;

        for (i, page) in page_update.iter().enumerate() {
            let order_rules_map_page = order_rules_map.get(page);
            match order_rules_map_page {
                Some(rules) => {
                    for j in 0..page_update.len() {
                        if rules.contains(&page_update[j]) && i > j {
                            has_correct_order = false;
                        }
                    }
                }
                None => continue,
            }
        }

        if has_correct_order {
            correct_ordered_pages.push(page_update.to_vec());
        } else {
            incorrect_ordered_pages.push(page_update.to_vec());
        }
    }

    return (correct_ordered_pages, incorrect_ordered_pages);
}
