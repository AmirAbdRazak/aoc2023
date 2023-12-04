use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn problem() -> Option<usize> {
    let input_file = File::open("src/day4/input.txt").unwrap();
    let reader = io::BufReader::new(input_file);
    let card_games: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    println!("Part 1 answer: {}", part1(card_games.clone())?);
    Some(part2(card_games)?)
}

pub fn part1(card_games: Vec<String>) -> Option<usize> {
    let final_sum = card_games
        .iter()
        .map(|line| {
            let mut game_sum = 0;
            let game_data = line.split(":").nth(1)?;
            let winning_cards = &mut game_data
                .split("|")
                .nth(0)?
                .split(" ")
                .filter(|s| s != &&"".to_string())
                .map(|s| (s, false))
                .collect::<HashMap<&str, bool>>();
            game_data.split("|").nth(1)?.split(" ").for_each(|s| {
                if s != &"".to_string() {
                    winning_cards.entry(s).and_modify(|val| *val = true);
                }
            });

            winning_cards
                .iter()
                .filter(|(_, v)| **v)
                .map(|(k, _)| k.parse::<usize>().expect("{} is not"))
                .for_each(|_| game_sum = if game_sum == 0 { 1 } else { game_sum * 2 });

            Some(game_sum)
        })
        .sum::<Option<usize>>();

    final_sum
}

pub fn part2(card_games: Vec<String>) -> Option<usize> {
    let copies = card_games
        .iter()
        .filter_map(|line| {
            let game_data = line.split(":").nth(1)?;
            let winning_cards = &mut game_data
                .split("|")
                .nth(0)?
                .split(" ")
                .filter(|s| s != &&"".to_string())
                .map(|s| (s, false))
                .collect::<HashMap<&str, bool>>();
            game_data.split("|").nth(1)?.split(" ").for_each(|s| {
                if s != &"".to_string() {
                    winning_cards.entry(s).and_modify(|val| *val = true);
                }
            });

            Some(winning_cards.iter().filter(|(_, v)| **v).count())
        })
        .collect::<Vec<usize>>();

    let mut copies_map: HashMap<usize, usize> = copies
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, _)| (i, 1))
        .collect();

    Some(
        copies
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                let curr_copies = copies_map[&idx];
                (idx + 1..idx + val + 1).for_each(|i| {
                    copies_map.entry(i).and_modify(|v| *v += curr_copies);
                });

                curr_copies
            })
            .sum::<usize>(),
    )
}
