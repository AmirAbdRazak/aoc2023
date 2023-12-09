use crate::utils::{get_data, get_number_vec, split_helper};
use std::iter::zip;

pub fn part1() -> Option<usize> {
    let races = get_data("src/day6/input.txt");
    let time_vec: Vec<usize> = get_number_vec(split_helper(&races[0], 1, ':'));
    let distance_vec: Vec<usize> = get_number_vec(split_helper(&races[1], 1, ':'));

    let score_pairing: Vec<(usize, usize)> = zip(time_vec, distance_vec).collect();

    score_pairing
        .iter()
        .map(|(time, distance)| (0..(*time)).filter(|i| (*time - i) * i > *distance).count())
        .reduce(|acc, x| acc * x)
}
pub fn part2() -> Option<usize> {
    let races = get_data("src/day6/input.txt");
    let time = split_helper(&races[0], 1, ':')
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = split_helper(&races[1], 1, ':')
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    Some((0..(time)).filter(|i| (time - i) * i > distance).count())
}
