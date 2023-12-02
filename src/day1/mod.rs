use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn problem() -> u64 {
    let input_file = File::open("src/day1/input.txt").unwrap();
    let reader = io::BufReader::new(input_file);
    let calibration_values: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    println!("Part 1 answer: {}", part1(calibration_values.clone()));

    part2(calibration_values)
}

fn part1(calibration_values: Vec<String>) -> u64 {
    let mut parsed_values: Vec<u64> = Vec::new();
    for value in calibration_values {
        let mut parsed_value = String::new();
        for c in value.chars() {
            if c.is_numeric() {
                parsed_value.push(c);
                break;
            }
        }
        for c in value.chars().rev() {
            if c.is_numeric() {
                parsed_value.push(c);
                break;
            }
        }

        parsed_values.push(parsed_value.parse::<u64>().unwrap())
    }

    parsed_values.iter().sum()
}

fn part2(calibration_values: Vec<String>) -> u64 {
    let mut parsed_values: Vec<u64> = Vec::new();
    let mut dofs = HashMap::new();
    dofs.insert("nine", "nine9nine");
    dofs.insert("eight", "eight8eight");
    dofs.insert("seven", "seven7seven");
    dofs.insert("six", "six6six");
    dofs.insert("five", "five5five");
    dofs.insert("four", "four4four");
    dofs.insert("three", "three3three");
    dofs.insert("two", "two2two");
    dofs.insert("one", "one1one");

    for value in calibration_values {
        let mut parsed_value = String::new();

        let mut subbed = value.clone();
        for (dof, num) in &dofs {
            subbed = subbed.replace(dof, num);
        }

        for c in subbed.chars() {
            if c.is_numeric() {
                parsed_value.push(c);
                break;
            }
        }
        for c in subbed.chars().rev() {
            if c.is_numeric() {
                parsed_value.push(c);
                break;
            }
        }

        parsed_values.push(parsed_value.parse::<u64>().unwrap())
    }

    parsed_values.iter().sum()
}
