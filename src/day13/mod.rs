use itertools::Itertools;

use crate::utils::get_data;

fn compare_lines_hr(landscape: Vec<String>, a: &str, b: &str, i: i32) -> usize {
    let h_len = landscape.len() as i32;
    if a == b {
        let a_len = i as i32 + 1;
        let b_len = h_len - (i as i32 + 1);
        let mut start = 0;
        let mut end = h_len;

        if a_len > b_len {
            start = a_len - b_len
        } else if b_len > a_len {
            end = h_len - (b_len - a_len);
        }
        let a = &mut landscape.clone()[start as usize..i as usize + 1];
        let b = &landscape[i as usize + 1..end as usize];

        a.reverse();
        if a == b {
            return i as usize + 1;
        }
    }

    return 0;
}

fn compare_smudge_lines_hr(landscape: Vec<String>, a: &str, b: &str, i: i32) -> usize {
    let h_len = landscape.len() as i32;
    if a == b {
        let a_len = i as i32 + 1;
        let b_len = h_len - (i as i32 + 1);
        let mut start = 0;
        let mut end = h_len;

        if a_len > b_len {
            start = a_len - b_len
        } else if b_len > a_len {
            end = h_len - (b_len - a_len);
        }
        let a = &mut landscape.clone()[start as usize..i as usize + 1];
        let b = &landscape[i as usize + 1..end as usize];

        a.reverse();
        if a == b {
            return i as usize + 1;
        }
    }

    return 0;
}
pub fn part1() -> Option<usize> {
    let landscapes = get_data("src/day13/input.txt")
        .join("\n")
        .split("\n\n")
        .map(|s| s.split("\n").map(|c| c.to_string()).collect_vec())
        .collect_vec();

    let final_sum = landscapes
        .iter()
        .map(|landscape| {
            let mut h_count = 0;

            for (i, window) in landscape.clone().windows(2).enumerate() {
                let check = compare_lines_hr(landscape.clone(), &window[0], &window[1], i as i32);
                if check > 0 {
                    h_count = check;
                    break;
                }
            }

            let mut v_count = 0;

            let transposed = (0..landscape[0].len())
                .map(|i| {
                    landscape
                        .iter()
                        .map(|row| row.chars().nth(i).unwrap())
                        .join("")
                })
                .collect_vec();

            for (i, window) in transposed.clone().windows(2).enumerate() {
                let check = compare_lines_hr(transposed.clone(), &window[0], &window[1], i as i32);
                if check > 0 {
                    v_count = check;
                    break;
                }
            }

            (h_count * 100) + v_count
        })
        .sum::<usize>();

    Some(final_sum)
}

pub fn mutate_smudge(landscape: Vec<String>) -> Vec<String> {
    landscape
}

pub fn part2() -> Option<usize> {
    let landscapes = get_data("src/day13/input.txt")
        .join("\n")
        .split("\n\n")
        .map(|s| s.split("\n").map(|c| c.to_string()).collect_vec())
        .collect_vec();

    let final_sum = landscapes
        .iter()
        .map(|landscape| {
            let mut h_count = 0;

            let new_landscape = mutate_smudge(landscape.to_owned());

            for (i, window) in landscape.clone().windows(2).enumerate() {
                let check = compare_lines_hr(landscape.clone(), &window[0], &window[1], i as i32);
                if check > 0 {
                    h_count = check;
                    break;
                }
            }

            let mut v_count = 0;

            let transposed = (0..landscape[0].len())
                .map(|i| {
                    landscape
                        .iter()
                        .map(|row| row.chars().nth(i).unwrap())
                        .join("")
                })
                .collect_vec();

            for (i, window) in transposed.clone().windows(2).enumerate() {
                let check = compare_lines_hr(transposed.clone(), &window[0], &window[1], i as i32);
                if check > 0 {
                    v_count = check;
                    break;
                }
            }

            (h_count * 100) + v_count
        })
        .sum::<usize>();

    Some(final_sum)
}
