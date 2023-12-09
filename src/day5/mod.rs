use crate::utils::get_data;
pub fn part1() -> Option<usize> {
    let almanac = get_data("src/day5/input.txt");

    let mut source_map = almanac[0]
        .split(':')
        .nth(1)?
        .split(' ')
        .filter_map(|c| {
            if c == "" {
                return None;
            }
            Some(c.parse::<usize>().unwrap())
        })
        .collect::<Vec<usize>>();

    let mut destination_map = source_map.clone();

    let mut mapping = almanac
        .join("\n")
        .split("\n\n")
        .map(|c| {
            c.to_string()
                .split("\n")
                .enumerate()
                .filter_map(|(idx, ch)| {
                    if idx == 0 {
                        return None;
                    }
                    Some(
                        ch.to_string()
                            .split(' ')
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                    )
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    mapping.drain(0..1);

    mapping.iter().for_each(|to_mapper| {
        destination_map = source_map.clone();
        to_mapper.iter().for_each(|row| {
            source_map.iter().enumerate().for_each(|(idx, num)| {
                if (row[1]..row[1] + row[2]).contains(num) {
                    destination_map[idx] = num - row[1] + row[0];
                }
            })
        });
        source_map = destination_map.clone();
    });

    destination_map.iter().min().copied()
}

fn get_chunk_min(a: usize, b: usize, mapping: Vec<Vec<Vec<usize>>>) -> usize {
    let min_num_in_chunk = (a..a + b)
        .map(|idx| {
            let mut curr_num: usize = idx;
            mapping.iter().for_each(|to_mapper| {
                let r: Vec<&usize> = to_mapper
                    .iter()
                    .filter(|row| row[1] <= curr_num && row[1] + row[2] > curr_num)
                    .flatten()
                    .collect();

                if !r.is_empty() {
                    curr_num = curr_num - r[1] + r[0]
                }
            });
            curr_num
        })
        .min();

    println!("Min num in chunk: {}", min_num_in_chunk.unwrap());

    min_num_in_chunk.unwrap()
}

pub fn part2() -> Option<usize> {
    let almanac = get_data("src/day5/input.txt");

    let source_map = almanac[0]
        .split(':')
        .nth(1)?
        .split(' ')
        .filter_map(|c| {
            if c == "" {
                return None;
            }
            Some(c.parse::<usize>().unwrap())
        })
        .collect::<Vec<usize>>();

    let mut mapping = almanac
        .join("\n")
        .split("\n\n")
        .map(|c| {
            c.to_string()
                .split("\n")
                .enumerate()
                .filter_map(|(idx, ch)| {
                    if idx == 0 {
                        return None;
                    }
                    Some(
                        ch.to_string()
                            .split(' ')
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                    )
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    mapping.drain(0..1);
    println!(
        "There are {} chunks that are going to be calculated.",
        source_map.len() / 2
    );

    source_map
        .chunks(2)
        .map(|chunk| {
            let (a, b) = (chunk[0] as usize, chunk[1] as usize);
            let map_clone = mapping.clone();

            get_chunk_min(a, b, map_clone)
        })
        .collect::<Vec<usize>>()
        .iter()
        .min()
        .copied()
}
