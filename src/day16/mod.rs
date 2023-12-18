use crate::utils::get_data;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);

pub fn traverse_h(
    tiles: &Vec<Vec<char>>,
    seen: &mut HashMap<Coord, HashSet<(char, i32)>>,
    x: usize,
    y: usize,
    dx: i32,
) {
    let mut check = false;
    seen.entry((x, y))
        .and_modify(|s| {
            if s.contains(&('h', dx)) {
                check = true;
            } else {
                s.insert(('h', dx));
            }
        })
        .or_insert(HashSet::from([('h', dx)]));

    if check {
        return;
    }

    let dx_is_positive = dx > 0;
    match tiles[y][x] {
        '.' => {
            if (dx_is_positive && x < tiles[0].len() - 1) || (!dx_is_positive && x > 0) {
                traverse_h(tiles, seen, (x as i32 + dx) as usize, y, dx)
            }
        }
        '-' => {
            if (dx_is_positive && x < tiles[0].len() - 1) || (!dx_is_positive && x > 0) {
                traverse_h(tiles, seen, (x as i32 + dx) as usize, y, dx)
            }
        }
        '/' => {
            if dx_is_positive {
                if y > 0 {
                    traverse_v(tiles, seen, x, y - 1, -1);
                }
            } else {
                if y < tiles.len() - 1 {
                    traverse_v(tiles, seen, x, y + 1, 1);
                }
            }
        }
        '\\' => {
            if dx_is_positive {
                if y < tiles.len() - 1 {
                    traverse_v(tiles, seen, x, y + 1, 1);
                }
            } else {
                if y > 0 {
                    traverse_v(tiles, seen, x, y - 1, -1);
                }
            }
        }
        '|' => {
            if y > 0 {
                traverse_v(tiles, seen, x, y - 1, -1);
            }
            if y < tiles.len() - 1 {
                traverse_v(tiles, seen, x, y + 1, 1);
            }
        }
        _ => unreachable!(),
    }
}
pub fn traverse_v(
    tiles: &Vec<Vec<char>>,
    seen: &mut HashMap<Coord, HashSet<(char, i32)>>,
    x: usize,
    y: usize,
    dy: i32,
) {
    let mut check = false;
    seen.entry((x, y))
        .and_modify(|s| {
            if s.contains(&('v', dy)) {
                check = true;
            } else {
                s.insert(('v', dy));
            }
        })
        .or_insert(HashSet::from([('v', dy)]));
    if check {
        return;
    }

    let dy_is_positive = dy > 0;
    match tiles[y][x] {
        '.' => {
            if (dy_is_positive && y < tiles.len() - 1) || (!dy_is_positive && y > 0) {
                traverse_v(tiles, seen, x, (y as i32 + dy) as usize, dy)
            }
        }
        '|' => {
            if (dy_is_positive && y < tiles.len() - 1) || (!dy_is_positive && y > 0) {
                traverse_v(tiles, seen, x, (y as i32 + dy) as usize, dy)
            }
        }
        '/' => {
            if dy_is_positive {
                if x > 0 {
                    traverse_h(tiles, seen, x - 1, y, -1);
                }
            } else {
                if x < tiles[0].len() - 1 {
                    traverse_h(tiles, seen, x + 1, y, 1);
                }
            }
        }
        '\\' => {
            if dy_is_positive {
                if x < tiles[0].len() - 1 {
                    traverse_h(tiles, seen, x + 1, y, 1);
                }
            } else {
                if x > 0 {
                    traverse_h(tiles, seen, x - 1, y, -1);
                }
            }
        }
        '-' => {
            if x < tiles[0].len() - 1 {
                traverse_h(tiles, seen, x + 1, y, 1);
            }
            if x > 0 {
                traverse_h(tiles, seen, x - 1, y, -1);
            }
        }
        _ => unreachable!(),
    }
}

pub fn debug(mut tiles: Vec<Vec<char>>, seen: HashMap<Coord, HashSet<(char, i32)>>) {
    tiles
        .iter_mut()
        .enumerate()
        .map(|(y, line)| {
            line.iter_mut()
                .enumerate()
                .map(|(x, ch)| {
                    if seen.contains_key(&(x, y)) {
                        *ch = 'X'
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    for t in tiles {
        println!("{:?}", t);
    }
}

pub fn part1() -> Option<usize> {
    let tiles = get_data("src/day16/input.txt")
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut seen: HashMap<Coord, HashSet<(char, i32)>> = HashMap::new();
    traverse_h(&tiles, &mut seen, 0, 0, 1);

    Some(seen.keys().count())
}

pub fn part2() -> Option<usize> {
    let tiles = get_data("src/day16/input.txt")
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut energized_tiles = HashSet::new();

    let max_x = tiles[0].len();
    let max_y = tiles.len();

    for i in 0..max_x {
        let mut seen_from_top = HashMap::new();
        traverse_v(&tiles, &mut seen_from_top, i, 0, 1);
        energized_tiles.insert(seen_from_top.keys().count());

        let mut seen_from_bottom = HashMap::new();
        traverse_v(&tiles, &mut seen_from_bottom, i, max_y - 1, -1);
        energized_tiles.insert(seen_from_bottom.keys().count());
    }
    for i in 0..max_y {
        let mut seen_from_left = HashMap::new();
        traverse_h(&tiles, &mut seen_from_left, 0, i, 1);

        energized_tiles.insert(seen_from_left.keys().count());
        let mut seen_from_right = HashMap::new();
        traverse_h(&tiles, &mut seen_from_right, max_x - 1, i, -1);
        energized_tiles.insert(seen_from_right.keys().count());
    }

    energized_tiles.iter().max().copied()
}
