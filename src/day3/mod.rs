use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn problem() -> Option<usize> {
    let input_file = File::open("src/day3/input.txt").unwrap();
    let reader = io::BufReader::new(input_file);
    let schematic: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    println!("Part 1 answer: {}", part1(schematic.clone())?);
    Some(part2(schematic)?)
}

fn check_sym(schematic: Vec<String>, x: usize, y: usize, num_len: usize) -> Option<bool> {
    let y_start;
    let y_end;
    let x_start;
    let x_end;

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

    for b in y_start..y_end {
        for a in x_start..x_end {
            let cell = schematic.get(b)?.chars().nth(a)?;
            if !cell.is_numeric() && cell != '.' {
                return Some(true);
            }
        }
    }

    Some(false)
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
                    if let Some(_found_sym) =
                        check_sym(schematic.clone(), x + 1, y, num_found.len())
                            .filter(|&found| found)
                    {
                        part_list.push(num_found.parse::<usize>().unwrap());
                    }
                    num_found.clear()
                }
            }
        }
    }

    Some(part_list.iter().sum())
}

struct SymInfo {
    symbol: String,
    x: Option<usize>,
    y: Option<usize>,
}

fn check_sym_with_gear(
    schematic: Vec<String>,
    x: usize,
    y: usize,
    num_len: usize,
) -> Option<SymInfo> {
    let y_start;
    let y_end;
    let x_start;
    let x_end;

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

    for b in y_start..y_end {
        for a in x_start..x_end {
            let cell = schematic.get(b)?.chars().nth(a)?;
            if !cell.is_numeric() && cell != '.' {
                return Some(SymInfo {
                    symbol: cell.to_string(),
                    x: Some(a),
                    y: Some(b),
                });
            }
        }
    }

    Some(SymInfo {
        symbol: "".to_string(),
        x: None,
        y: None,
    })
}

fn part2(schematic: Vec<String>) -> Option<usize> {
    let x_limit = schematic[0].len() - 1;
    let mut num_found = "".to_string();
    let schema_clone = schematic.clone();
    let mut gear_hash: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for y in 0..schematic.len() {
        let row = schema_clone.get(y)?;
        for x in 0..row.len() {
            let cell = row.chars().nth(x)?;
            if cell.is_numeric() {
                num_found.push(cell);
                if !num_found.is_empty() && (x == x_limit || !row.chars().nth(x + 1)?.is_numeric())
                {
                    if let Some(found_sym) =
                        check_sym_with_gear(schematic.clone(), x + 1, y, num_found.len())
                    {
                        let parsed_num = num_found.parse::<u64>().unwrap();
                        if found_sym.symbol == "*" {
                            gear_hash
                                .entry((found_sym.x?, found_sym.y?))
                                .and_modify(|v| v.push(parsed_num))
                                .or_insert(vec![parsed_num]);
                        }
                    }
                    num_found.clear()
                }
            }
        }
    }

    let final_sum = gear_hash
        .values()
        .filter(|values| values.len() == 2)
        .map(|v| {
            let pow = v.iter().copied().reduce(|acc, e| acc * e).unwrap();
            return pow as usize;
        })
        .sum();

    Some(final_sum)
}
