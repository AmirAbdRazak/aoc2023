use std::fs::File;
use std::io::{self, BufRead};

pub fn problem() -> u64 {
    let input_file = File::open("src/day2/input.txt").unwrap();
    let reader = io::BufReader::new(input_file);
    let game_list: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    println!("Part 1 answer: {}", part1(game_list.clone()));
    part2(game_list)
}

pub fn part1(game_list: Vec<String>) -> u64 {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    game_list
        .into_iter()
        .filter(|game| {
            let mut is_valid = true;
            game.split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()[1]
                .split(";")
                .for_each(|data| {
                    data.split(",")
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .for_each(|count_data| {
                            let c = count_data.split(" ").collect::<Vec<&str>>();
                            let color = c[2];
                            let num = c[1].parse::<i32>().unwrap();
                            if (color == "red" && num > red_limit)
                                || (color == "blue" && num > blue_limit)
                                || (color == "green" && num > green_limit)
                            {
                                is_valid = false;
                            }
                        })
                });

            is_valid
        })
        .map(|game| {
            let count = game
                .split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()[0]
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .unwrap();

            count
        })
        .sum()
}

pub fn part2(game_list: Vec<String>) -> u64 {
    game_list
        .into_iter()
        .map(|game| {
            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;
            game.split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()[1]
                .split(";")
                .for_each(|data| {
                    data.split(",")
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .for_each(|count_data| {
                            let c = count_data.split(" ").collect::<Vec<&str>>();
                            let color = c[2];
                            let num = c[1].parse::<i32>().unwrap();
                            if color == "red" && num > red_max {
                                red_max = num;
                            } else if color == "green" && num > green_max {
                                green_max = num;
                            } else if color == "blue" && num > blue_max {
                                blue_max = num;
                            }
                        })
                });

            red_max as u64 * green_max as u64 * blue_max as u64
        })
        .sum()
}
