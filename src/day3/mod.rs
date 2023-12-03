use std::fs::File;
use std::io::{self, BufRead};
use std::thread::ScopedJoinHandle;

pub fn problem() -> Option<usize> {
    let input_file = File::open("src/day3/input.txt").unwrap();
    let reader = io::BufReader::new(input_file);
    let schematic: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    Some(part1(schematic)?)
}

fn check_sym(schematic: Vec<String>, x: usize, y: usize, num_len: usize) -> Option<bool> {
    let y_start;
    let y_end;
    let x_start;
    let x_end;

    let mut check = false;
    let mut cell_found = "".to_string();

    if y == 0 {
        y_start = 0;
        y_end = y + 2;
    } else if y == schematic.len() - 1 {
        y_start = y - 1;
        y_end = schematic.len()
    } else {
        y_start = y - 1;
        y_end = y + 2;
    }

    if x - num_len == 0 {
        x_start = 0;
        x_end = x + 1;
    } else if x == schematic.len() {
        x_start = x - num_len - 1;
        x_end = schematic.len()
    } else {
        x_start = x - num_len - 1;
        x_end = x + 1;
    }

    println!(
        "Current scanning coords: x: {} - {}, y: {} - {}",
        x_start, x_end, y_start, y_end
    );

    for b in y_start..y_end {
        print!("\nRow: {} - ", b);
        for a in x_start..x_end {
            let cell = schematic.get(b)?.chars().nth(a)?;
            print!("{}", cell);
            if !cell.is_numeric() && cell != '.' {
                cell_found.push(cell);
                check = true;
            }
        }
    }

    if check {
        println!("\nCell Found: {}", cell_found);
    }
    Some(check)
}

fn part1(schematic: Vec<String>) -> Option<usize> {
    let x_limit = schematic[0].len() - 1;
    let mut part_list = vec![];
    let mut num_found = "".to_string();
    let schema_clone = schematic.clone();

    for y in 0..schematic.len() {
        let row = schema_clone.get(y)?;
        for x in 0..row.len() {
            let cell = row.chars().nth(x)?;
            if cell.is_numeric() {
                num_found.push(cell);
                if !num_found.is_empty() && (x == x_limit || !row.chars().nth(x + 1)?.is_numeric())
                {
                    println!("\n---------------------------------");
                    println!("\nChecking for: {}", num_found);
                    if let Some(_found_sym) =
                        check_sym(schematic.clone(), x + 1, y, num_found.len())
                            .filter(|&found| found)
                    {
                        println!("Check passed!");
                        part_list.push(num_found.parse::<usize>().unwrap());
                    }
                    num_found.clear()
                }
            }
        }
    }

    Some(part_list.iter().sum())
}
