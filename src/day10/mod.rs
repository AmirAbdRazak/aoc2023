use crate::utils::get_data;
use std::collections::{HashSet, VecDeque};

type Coord = (usize, usize);

fn traverse_adj(map: &Vec<Vec<String>>, coord: Coord) -> Vec<Coord> {
    let (x, y) = coord;
    let current_node = &map[y as usize][x as usize];
    let mut routes = vec![];

    if x > 0 && "-7JS".contains(current_node) {
        routes.push((x - 1, y));
    }
    if x + 1 < map[0].len() && "-LFS".contains(current_node) {
        routes.push((x + 1, y));
    }
    if y + 1 < map.len() && "|F7S".contains(current_node) {
        routes.push((x, y + 1));
    }
    if y > 0 && "|LJS".contains(current_node) {
        routes.push((x, y - 1));
    }

    return routes;
}
pub fn part1() -> Option<usize> {
    let mut start = (0, 0);
    let map = get_data("src/day10/input.txt")
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == 'S' {
                        start = (x, y);
                    }
                    ch.to_string()
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut queue: VecDeque<Coord> = traverse_adj(&map, start)
        .iter()
        .filter_map(|coord| {
            if traverse_adj(&map, *coord).contains(&start) {
                Some(coord.to_owned())
            } else {
                None
            }
        })
        .collect::<VecDeque<Coord>>();

    let mut cont = true;
    let mut seen = HashSet::from([start]);

    while cont {
        let coord = queue.pop_front().unwrap();
        let adj = traverse_adj(&map, coord);
        seen.insert(coord);

        queue.extend(adj.iter().filter(|coord| !seen.contains(coord)));

        if queue.is_empty() {
            cont = false;
        }
    }

    Some(seen.len() / 2)
}

fn raytrace(grid: &Vec<Vec<String>>, coord: Coord, seen: &HashSet<Coord>) -> usize {
    let (mut x, y) = coord;
    let mut intersections = 0;
    if seen.contains(&(x, y)) {
        return 0;
    }

    while x + 1 < grid[0].len() {
        x += 1;
        if seen.contains(&(x, y)) && "|JL".contains(&grid[y][x]) {
            intersections += 1
        }
    }

    intersections
}

pub fn part2() -> Option<usize> {
    let mut start = (0, 0);
    let map = get_data("src/day10/input.txt")
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == 'S' {
                        start = (x, y);
                    }
                    ch.to_string()
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut queue: VecDeque<Coord> = traverse_adj(&map, start)
        .iter()
        .filter_map(|coord| {
            if traverse_adj(&map, *coord).contains(&start) {
                Some(coord.to_owned())
            } else {
                None
            }
        })
        .collect::<VecDeque<Coord>>();

    let mut cont = true;
    let mut seen = HashSet::from([start]);

    while cont {
        let coord = queue.pop_front().unwrap();
        let adj = traverse_adj(&map, coord);
        seen.insert(coord);

        queue.extend(adj.iter().filter(|coord| !seen.contains(coord)));

        if queue.is_empty() {
            cont = false;
        }
    }

    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if raytrace(&map, (x, y), &seen) % 2 == 1 {
                count += 1;
            }
        }
    }

    Some(count)
}
