use crate::utils::{get_data, split_helper};
use std::collections::HashMap;

pub fn part1() -> Option<usize> {
    let mapper = get_data("src/day8/input.txt");

    let mut curr = "AAA";

    let instruction = &mapper[0]
        .trim()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    let paths: HashMap<String, Vec<String>> = mapper[2..]
        .iter()
        .map(|line| {
            (
                split_helper(line, 0, '=').trim().to_string(),
                split_helper(line, 1, '=')
                    .split(',')
                    .map(|c| {
                        c.chars()
                            .filter(|n| n.is_alphabetic())
                            .collect::<String>()
                            .to_string()
                    })
                    .collect::<Vec<String>>(),
            )
        })
        .collect();

    let mut count = 0;

    loop {
        for direction in instruction {
            curr = &paths[curr][*direction];
            count += 1;
            if curr == "ZZZ" {
                break;
            }
        }
        if curr == "ZZZ" {
            break;
        }
    }

    Some(count)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn find_lcm(values: Vec<usize>) -> usize {
    values.iter().cloned().fold(1, lcm)
}

pub fn part2() -> Option<usize> {
    let mapper = get_data("src/day8/input.txt");

    let instruction = &mapper[0]
        .trim()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    let paths: HashMap<String, Vec<String>> = mapper[2..]
        .iter()
        .map(|line| {
            (
                split_helper(line, 0, '=').trim().to_string(),
                split_helper(line, 1, '=')
                    .split(',')
                    .map(|c| {
                        c.chars()
                            .filter(|n| n.is_alphanumeric())
                            .collect::<String>()
                            .to_string()
                    })
                    .collect::<Vec<String>>(),
            )
        })
        .collect();
    let mut path_iter: HashMap<String, usize> = paths
        .keys()
        .filter_map(|key| {
            if key.chars().nth(2).unwrap() == 'A' {
                Some((key.to_owned(), 0))
            } else {
                None
            }
        })
        .collect();

    let path_iter_clone = path_iter.clone();

    for start in path_iter_clone.keys() {
        let mut curr = start;
        let mut curr_count = 0;
        let mut count = 0;

        let mut seen = vec![];
        while curr != "ZZZ" && !seen.contains(&curr) {
            for direction in instruction {
                curr = &paths[curr][*direction];
                count += 1;
                if curr.chars().nth(2).unwrap() == 'Z' {
                    curr_count = count;
                    seen.push(curr);
                }

                if curr == "ZZZ" || seen.contains(&curr) {
                    break;
                }
            }
        }

        path_iter
            .entry(start.to_string())
            .and_modify(|v| *v = curr_count);
    }

    Some(find_lcm(
        path_iter.values().map(|val| val.to_owned()).collect(),
    ))
}
