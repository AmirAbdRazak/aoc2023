use crate::utils::get_data;
use itertools::Itertools;

fn push(rockscape: &mut Vec<Vec<char>>, i: usize, j: usize, dir: &str) {
    let (mut dir_i, mut dir_j): (i32, i32) = (0, 0);
    let mut check = false;

    if dir == "up" {
        dir_i = -1;
        check = i > 0;
    } else if dir == "down" {
        dir_i = 1;
        check = i < rockscape.len() - 1;
    } else if dir == "left" {
        dir_j = -1;
        check = j > 0;
    } else if dir == "right" {
        dir_j = 1;
        check = j < rockscape[0].len() - 1;
    }
    if !(rockscape[i][j] == 'O') && !(rockscape[i][j] == '#') {
        rockscape[(i as i32 - dir_i) as usize][(j as i32 - dir_j) as usize] = '.';
        rockscape[i][j] = 'O';

        if check {
            push(
                rockscape,
                (i as i32 + dir_i) as usize,
                (j as i32 + dir_j) as usize,
                dir,
            );
        }
    }
}

fn tilt_north(rockscape: &mut Vec<Vec<char>>) {
    for i in 0..rockscape.len() {
        for j in 0..rockscape[0].len() {
            if rockscape[i][j] == 'O' && i > 0 {
                push(rockscape, i - 1, j, "up");
            }
        }
    }
}

fn tilt_south(rockscape: &mut Vec<Vec<char>>) {
    for i in (0..rockscape.len()).rev() {
        for j in (0..rockscape[0].len()).rev() {
            if rockscape[i][j] == 'O' && i < rockscape.len() - 1 {
                push(rockscape, i + 1, j, "down");
            }
        }
    }
}

fn tilt_west(rockscape: &mut Vec<Vec<char>>) {
    for j in 0..rockscape[0].len() {
        for i in 0..rockscape.len() {
            if rockscape[i][j] == 'O' && j > 0 {
                push(rockscape, i, j - 1, "left");
            }
        }
    }
}

fn tilt_east(rockscape: &mut Vec<Vec<char>>) {
    for j in (0..rockscape[0].len()).rev() {
        for i in (0..rockscape.len()).rev() {
            if rockscape[i][j] == 'O' && j < rockscape.len() - 1 {
                push(rockscape, i, j + 1, "right");
            }
        }
    }
}

fn tilt_a_bunch_of_times(rockscape: &mut Vec<Vec<char>>) {
    for i in 0..1000000000 {
        if i % 1000 == 0 {
            println!("Cycle: {}", i);
        }
        tilt_north(rockscape);
        tilt_west(rockscape);
        tilt_south(rockscape);
        tilt_east(rockscape);
    }
}
pub fn part1() -> Option<usize> {
    let mut rockscape = get_data("src/day14/input.txt")
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    tilt_north(&mut rockscape);
    for l in rockscape.clone() {
        println!("{:?}", l);
    }
    let limit = rockscape.len();
    Some(
        rockscape
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                line.iter()
                    .map(|ch| if ch == &'O' { limit - idx } else { 1 })
                    .sum::<usize>()
            })
            .sum(),
    )
}

pub fn part2() -> Option<usize> {
    let mut rockscape = get_data("src/day14/input.txt")
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    tilt_a_bunch_of_times(&mut rockscape);
    let limit = rockscape.len();
    Some(
        rockscape
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                line.iter()
                    .map(|ch| if ch == &'O' { limit - idx } else { 1 })
                    .sum::<usize>()
            })
            .sum(),
    )
}
