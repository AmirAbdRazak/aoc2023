use crate::utils::{get_data, split_helper};
use std::cmp::Ordering;
use std::collections::HashMap;

fn get_status(hand: &String, joker: bool) -> Option<usize> {
    let mut map = hand.chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    let mut joker_count = 0;
    let mut subtractor = 0;

    let filtered_map: Vec<&usize> = if joker {
        map.iter()
            .filter_map(|(key, val)| if key != &'J' { Some(val) } else { None })
            .collect()
    } else {
        map.values().collect()
    };

    let max_val = *filtered_map.iter().max().unwrap_or(&&0);
    let min_val = *filtered_map.iter().min().unwrap_or(&&1);

    if joker {
        joker_count = *map.get(&'J').unwrap_or(&0);
        subtractor = if joker_count > 0 { 1 } else { 0 };
    }

    if max_val + joker_count == 5 {
        return Some(6);
    } else if max_val + joker_count == 4 {
        return Some(5);
    } else if max_val + joker_count == 3 {
        if min_val == &2 {
            return Some(4);
        } else {
            return Some(3);
        }
    } else if max_val + joker_count == 2 {
        // if there's 3 values, it means its 2, 2, 1
        if map.values().count() - subtractor == 3 {
            return Some(2);
        } else {
            return Some(1);
        }
    } else {
        return Some(0);
    }
}

fn get_card_rank(rank: char, joker: bool) -> u32 {
    match rank {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if joker {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => rank.to_digit(10).unwrap(),
    }
}

fn compare_vectors(a: &&String, b: &&String, joker: bool) -> Ordering {
    // Iterate over the elements of both vectors
    for (ai, bi) in a.chars().zip(b.chars()) {
        let a_rank = get_card_rank(ai, joker);
        let b_rank = get_card_rank(bi, joker);
        match a_rank.cmp(&b_rank) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => continue,
        }
    }

    // If all elements are equal, return Ordering::Equal
    Ordering::Equal
}
pub fn problem(joker: bool) -> Option<usize> {
    let game_data: HashMap<String, usize> = get_data("src/day7/input.txt")
        .iter()
        .map(|hand| {
            (
                split_helper(hand, 0, ' ').to_string(),
                split_helper(hand, 1, ' ').parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut hands: Vec<(&String, usize)> = game_data
        .keys()
        .map(|a| (a, get_status(a, joker).unwrap()))
        .collect();

    hands.sort_by(|(a_hand, a), (b_hand, b)| {
        if a == b {
            compare_vectors(a_hand, b_hand, joker)
        } else {
            a.partial_cmp(b).unwrap()
        }
    });

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(idx, (hand, _))| (idx + 1) * game_data[hand.as_str()])
            .sum(),
    )
}
