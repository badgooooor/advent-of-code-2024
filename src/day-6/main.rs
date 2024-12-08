use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file_location_arg = std::env::args().nth(1).expect("no file argument");

    let file = File::open(file_location_arg)?;

    let (obstacle_locations, init_guard_location, size) = get_map_location(&file);

    // Direction: 0 Up / 1 Right / 2 Down / 3 Left
    let initial_guard_direction = 0;

    // Solve part 1
    let result_1 = traverse(
        &obstacle_locations,
        &init_guard_location,
        &initial_guard_direction,
        &size,
    );
    match result_1 {
        Ok(total) => println!("Result 1: {}", total),
        _ => println!("Something wrong in result 1"),
    }

    Ok(())
}

// Core: Traverse
fn traverse(
    obstacle_locations: &Vec<(i32, i32)>,
    init_guard_location: &(i32, i32),
    initial_guard_direction: &i32,
    size: &i32,
) -> Result<i32, i32> {
    let mut guard_location = init_guard_location.clone();
    let mut guard_direction = initial_guard_direction.clone();

    let mut is_not_dead_end = true;

    let mut passed_location: HashSet<(i32, i32)> = HashSet::new();

    while is_not_dead_end {
        let direction_obstacles: Vec<&(i32, i32)> = obstacle_locations
            .iter()
            .filter(|location| match guard_direction {
                0 => location.1 == guard_location.1 && location.0 < guard_location.0,
                1 => location.0 == guard_location.0 && location.1 > guard_location.1,
                2 => location.1 == guard_location.1 && location.0 > guard_location.0,
                3 => location.0 == guard_location.0 && location.1 < guard_location.1,
                _ => false,
            })
            .collect();
        let nearest_obstacle = match guard_direction {
            0 => direction_obstacles
                .iter()
                .max_by(|a: &&&(i32, i32), b| a.0.cmp(&b.0)),
            1 => direction_obstacles.iter().min_by(|a, b| a.1.cmp(&b.1)),
            2 => direction_obstacles.iter().min_by(|a, b| a.0.cmp(&b.0)),
            3 => direction_obstacles
                .iter()
                .max_by(|a: &&&(i32, i32), b| a.1.cmp(&b.1)),
            _ => None,
        };

        match nearest_obstacle {
            Some(location) => {
                // Push passed location
                match guard_direction {
                    0 => {
                        for i in 0..(guard_location.0 - location.0) {
                            passed_location.insert((guard_location.0 - i, guard_location.1));
                        }
                    }
                    1 => {
                        for i in 0..(location.1 - guard_location.1) {
                            passed_location.insert((guard_location.0, guard_location.1 + i));
                        }
                    }
                    2 => {
                        for i in 0..(location.0 - guard_location.0) {
                            passed_location.insert((guard_location.0 + i, guard_location.1));
                        }
                    }
                    3 => {
                        for i in 0..(guard_location.1 - location.1) {
                            passed_location.insert((guard_location.0, guard_location.1 - i));
                        }
                    }
                    _ => (),
                }
                // Update location.
                let guard_location_diff = get_next_location(&guard_direction);
                guard_location = (
                    location.0 + guard_location_diff.0,
                    location.1 + guard_location_diff.1,
                );
            }
            None => is_not_dead_end = false,
        }

        if !is_not_dead_end {
            match guard_direction {
                0 => {
                    for i in 0..(guard_location.0 - size) {
                        passed_location.insert((guard_location.0 - i, guard_location.1));
                    }
                }
                1 => {
                    for i in 0..(size - guard_location.1) {
                        passed_location.insert((guard_location.0, guard_location.1 + i));
                    }
                }
                2 => {
                    for i in 0..(size - guard_location.0) {
                        passed_location.insert((guard_location.0 + i, guard_location.1));
                    }
                }
                3 => {
                    for i in 0..(guard_location.1 - size) {
                        passed_location.insert((guard_location.0, guard_location.1 - i));
                    }
                }
                _ => (),
            }
        }

        // Change direction
        if guard_direction < 3 {
            guard_direction += 1;
        } else {
            guard_direction = 0;
        }
    }

    Ok(passed_location.len() as i32)
}

fn get_next_location(guard_direction: &i32) -> (i32, i32) {
    match guard_direction {
        0 => (1, 0),
        1 => (0, -1),
        2 => (-1, 0),
        3 => (0, 1),
        _ => (0, 0),
    }
}

// Core: Get map initial data
fn get_map_location(file: &File) -> (Vec<(i32, i32)>, (i32, i32), i32) {
    let reader = io::BufReader::new(file);

    let mut obstacle_locations: Vec<(i32, i32)> = Vec::new();
    let mut init_guard_location: (i32, i32) = (0, 0);

    let mut size = 0;

    for (line_idx, line) in reader.lines().into_iter().enumerate() {
        if let Ok(line_content) = line {
            for (char_idx, character) in line_content.chars().enumerate() {
                if character == '#' {
                    obstacle_locations.push((line_idx as i32, char_idx as i32));
                } else if character == '^' {
                    init_guard_location = (line_idx as i32, char_idx as i32)
                }
            }
        }
        size = line_idx as i32;
    }

    return (obstacle_locations, init_guard_location, size + 1);
}
