use std::collections::HashMap;

use crate::utils::get_data;
use combinations::Combinations;
use std::mem::swap;

type Coord = (usize, usize);
pub fn part1() -> Option<usize> {
    let universe = get_data("src/day11/input.txt");

    let mut galaxy_positions: Vec<Coord> = vec![];
    let mut y_seen: HashMap<usize, usize> = HashMap::from(
        (0..universe.len())
            .map(|idx| (idx, 0))
            .collect::<HashMap<usize, usize>>(),
    );
    let mut x_seen: HashMap<usize, usize> = HashMap::from(
        (0..universe[0].len())
            .map(|idx| (idx, 0))
            .collect::<HashMap<usize, usize>>(),
    );

    universe.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, cell)| {
            if cell == '#' {
                galaxy_positions.push((x, y));
                y_seen.entry(y).and_modify(|y| *y += 1);
                x_seen.entry(x).and_modify(|x| *x += 1);
            }
        })
    });

    let empty_x = x_seen
        .iter()
        .filter_map(|(x, count)| {
            if count == &0 {
                Some(x.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();
    let empty_y = y_seen
        .iter()
        .filter_map(|(y, count)| {
            if count == &0 {
                Some(y.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let total_distance = Combinations::new(galaxy_positions, 2)
        .map(|coords| {
            let (mut x1, mut y1) = coords[0];
            let (mut x2, mut y2) = coords[1];

            if y1 > y2 {
                swap(&mut y1, &mut y2);
            }
            if x1 > x2 {
                swap(&mut x1, &mut x2);
            }

            let y_expand = empty_y.iter().filter(|&&y| (y1..y2).contains(&y)).count();
            let x_expand = empty_x.iter().filter(|&&x| (x1..x2).contains(&x)).count();

            (x2 - x1) + (y2 - y1) + y_expand + x_expand
        })
        .sum();

    Some(total_distance)
}
pub fn part2() -> Option<usize> {
    let universe = get_data("src/day11/input.txt");

    let mut galaxy_positions: Vec<Coord> = vec![];
    let mut y_seen: HashMap<usize, usize> = HashMap::from(
        (0..universe.len())
            .map(|idx| (idx, 0))
            .collect::<HashMap<usize, usize>>(),
    );
    let mut x_seen: HashMap<usize, usize> = HashMap::from(
        (0..universe[0].len())
            .map(|idx| (idx, 0))
            .collect::<HashMap<usize, usize>>(),
    );

    universe.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, cell)| {
            if cell == '#' {
                galaxy_positions.push((x, y));
                y_seen.entry(y).and_modify(|y| *y += 1);
                x_seen.entry(x).and_modify(|x| *x += 1);
            }
        })
    });

    let empty_x = x_seen
        .iter()
        .filter_map(|(x, count)| {
            if count == &0 {
                Some(x.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();
    let empty_y = y_seen
        .iter()
        .filter_map(|(y, count)| {
            if count == &0 {
                Some(y.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let total_distance = Combinations::new(galaxy_positions, 2)
        .map(|coords| {
            let (mut x1, mut y1) = coords[0];
            let (mut x2, mut y2) = coords[1];

            if y1 > y2 {
                swap(&mut y1, &mut y2);
            }
            if x1 > x2 {
                swap(&mut x1, &mut x2);
            }

            let y_expand = empty_y.iter().filter(|&&y| (y1..y2).contains(&y)).count() * 999999;
            let x_expand = empty_x.iter().filter(|&&x| (x1..x2).contains(&x)).count() * 999999;

            (x2 - x1) + (y2 - y1) + y_expand + x_expand
        })
        .sum();

    Some(total_distance)
}
