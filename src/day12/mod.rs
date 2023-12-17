use std::task::Wake;

use crate::utils::{get_data, get_number_vec};
use itertools::{self, Itertools};

pub fn calc_arrangements(
    mut springs: Vec<char>,
    pos: usize,
    expected_chart: Vec<usize>,
    mut expected_chart_index: usize,
    running_count: usize,
) -> Option<usize> {
    if pos >= springs.len() {
        if running_count > 0 {
            if expected_chart_index >= expected_chart.len()
                || running_count != expected_chart[expected_chart_index]
            {
                return Some(0);
            } else {
                expected_chart_index += 1;
            }
        }

        return Some(if expected_chart_index == expected_chart.len() {
            1
        } else {
            0
        });
    }

    if springs[pos] == '?' {
        let mut count = 0;
        springs[pos] = '#';
        count += calc_arrangements(
            springs.clone(),
            pos,
            expected_chart.clone(),
            expected_chart_index,
            running_count,
        )?;
        springs[pos] = '.';
        count += calc_arrangements(
            springs.clone(),
            pos,
            expected_chart.clone(),
            expected_chart_index,
            running_count,
        )?;

        return Some(count);
    } else if springs[pos] == '#' {
        return calc_arrangements(
            springs,
            pos + 1,
            expected_chart,
            expected_chart_index,
            running_count + 1,
        );
    } else if springs[pos] == '.' {
        if running_count > 0 {
            if expected_chart_index >= expected_chart.len()
                || expected_chart[expected_chart_index] != running_count
            {
                return Some(0);
            } else {
                expected_chart_index += 1;
            }
        }

        return calc_arrangements(springs, pos + 1, expected_chart, expected_chart_index, 0);
    }

    return Some(0);
}

pub fn part1() -> Option<usize> {
    let spring_data = get_data("src/day12/input.txt");
    let parsed_springs: Vec<(&str, Vec<usize>)> = spring_data
        .iter()
        .map(|line| {
            let mut splitted = line.split(" ");
            (
                splitted.next().unwrap(),
                get_number_vec::<usize>(splitted.next().unwrap(), Some(',')),
            )
        })
        .collect();

    Some(
        parsed_springs
            .iter()
            .filter_map(|(springs, chart)| {
                calc_arrangements(springs.chars().collect_vec(), 0, chart.to_owned(), 0, 0)
            })
            .sum(),
    )
}

pub fn part2() -> Option<usize> {
    let spring_data = get_data("src/day12/input.txt");
    let parsed_springs: Vec<(String, Vec<usize>)> = spring_data
        .iter()
        .map(|line| {
            let (spring, count) = line.split_once(" ").unwrap();
            (
                std::iter::once(spring).cycle().take(5).join("?"),
                get_number_vec::<usize>(count, Some(',')),
            )
        })
        .collect();

    Some(
        parsed_springs
            .iter()
            .filter_map(|(springs, chart)| {
                let n = chart.len();
                calc_arrangements(springs.chars().collect_vec(), 0, chart.iter(), 0, 0)
            })
            .sum(),
    )
}

pub fn _stupid_calculate_clusters(line: &str) -> Vec<usize> {
    let mut charts = vec![];
    let mut count = 0;

    for ch in line.chars() {
        if ch == '.' && count > 0 {
            charts.push(count);
            count = 0;
        } else if ch == '#' {
            count += 1
        }
    }

    if count > 0 {
        charts.push(count)
    }

    charts
}

pub fn _stupid_part1() -> Option<usize> {
    let spring_data = get_data("src/day12/input.txt");
    let parsed_springs: Vec<(&str, Vec<usize>)> = spring_data
        .iter()
        .map(|line| {
            let mut splitted = line.split(" ");
            (
                splitted.next().unwrap(),
                get_number_vec::<usize>(splitted.next().unwrap(), Some(',')),
            )
        })
        .collect();

    let brute_force = parsed_springs
        .iter()
        .map(|data| {
            let (springs, chart) = data;

            let unknown_count: usize = springs.chars().filter(|ch| ch == &'?').count();
            let c = (0..unknown_count)
                .map(|_| vec!['.', '#'])
                .multi_cartesian_product()
                .filter(|replacements| {
                    let mut sym_iter = replacements.iter();
                    let replaced_str = springs
                        .chars()
                        .map(|ch| {
                            if ch == '?' {
                                *sym_iter.next().unwrap()
                            } else {
                                ch
                            }
                        })
                        .collect::<String>();

                    _stupid_calculate_clusters(&replaced_str) == *chart
                })
                .count();

            c
        })
        .sum::<usize>();

    Some(brute_force)
}
