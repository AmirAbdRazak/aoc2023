use crate::utils::{get_data, get_number_vec};

pub fn part1() -> Option<i64> {
    let data = get_data("src/day9/input.txt");
    let history_list: Vec<Vec<i64>> = data
        .iter()
        .map(|line| get_number_vec::<i64>(line))
        .collect();

    Some(
        history_list
            .iter()
            .map(|history| {
                let mut deltas = vec![];
                let mut line = history.to_owned();

                while !line.iter().all(|x| x == &0) {
                    deltas.push(line.clone());
                    let mut acc = vec![];
                    line.windows(2)
                        .for_each(|window| acc.push(window[1] - window[0]));
                    line = acc;
                }

                deltas.iter().map(|v| v.last().unwrap()).sum::<i64>()
            })
            .sum(),
    )
}

pub fn part2() -> Option<i64> {
    let data = get_data("src/day9/input.txt");
    let history_list: Vec<Vec<i64>> = data
        .iter()
        .map(|line| get_number_vec::<i64>(line))
        .collect();

    Some(
        history_list
            .iter()
            .filter_map(|history| {
                let mut deltas = vec![];
                let mut line = history.to_owned();

                while !line.iter().all(|x| x == &0) {
                    deltas.push(line.clone());
                    let mut acc = vec![];
                    line.windows(2)
                        .for_each(|window| acc.push(window[1] - window[0]));
                    line = acc;
                }

                deltas
                    .iter()
                    .map(|v| v.first().unwrap())
                    .copied()
                    .rev()
                    .reduce(|acc, x| x - acc)
            })
            .sum(),
    )
}
