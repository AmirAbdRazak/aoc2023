use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn get_data(file_name: &'static str) -> Vec<String> {
    let input_file = File::open(file_name).unwrap();
    let reader = io::BufReader::new(input_file);
    reader.lines().filter_map(Result::ok).collect()
}

pub fn split_helper(data_line: &str, position: usize, splitter: char) -> &str {
    data_line.split(splitter).nth(position).unwrap()
}
pub fn get_number_vec<T: FromStr>(data_line: &str, splitter: Option<char>) -> Vec<T> {
    data_line
        .split(splitter.unwrap_or(' '))
        .map(T::from_str)
        .flatten()
        .collect()
}
