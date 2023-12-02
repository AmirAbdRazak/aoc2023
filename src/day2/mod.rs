use std::fs::File;
use std::io::{self, BufRead};

pub fn problem() -> u64 {
    let input_file = File::open("src/day2/input.txt").unwrap();
    let reader = io::BufReader::new(input_file);
    let game_list: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    println!("Part 1 answer: {}", part1(game_list.clone()));
    part1(game_list)
}

pub fn part1(game_list: Vec<String>) -> u64 {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    game_list
        .into_iter()
        .filter(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            game.split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()[1]
                .split(";")
                .for_each(|data| {
                    let parsed_data: Vec<&str> = data.split(" ").collect();
                    let color: &str = parsed_data[0];
                    let num = parsed_data[1].parse::<i32>().unwrap();

                    if color == "red" {
                        red += num;
                    } else if color == "blue" {
                        blue += num;
                    } else if color == "green" {
                        green += num;
                    }
                });

            red <= red_limit && green <= green_limit && blue <= blue_limit
        })
        .map(|game| {
            game.split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()[0]
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}
