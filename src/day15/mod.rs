use itertools::Itertools;

use crate::utils::get_data;

pub fn part1() -> Option<usize> {
    let data = get_data("src/day15/input.txt");
    let sequences = data[0].split(',').collect_vec();

    Some(
        sequences
            .iter()
            .map(|hash| {
                hash.chars()
                    .map(|ch| ch as usize)
                    .fold(0, |acc, val| (acc + val) * 17 % 256)
                    % 256
            })
            .sum::<usize>(),
    )
}
