use std::collections::VecDeque;
use std::fs;

fn main() {
    let filename = "example_input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong when reading a file");
    let lines_str = contents.lines();
    let lines = lines_str.map(|l| l.parse::<i32>().unwrap()).collect::<VecDeque<i32>>();
    let first_element = *lines.front().unwrap();
    let mut lines_shifted = lines.clone();
    lines_shifted.push_front(first_element);

    let pairs = lines_shifted.iter().zip(lines);

    let mut contains_increase = pairs.map(|(a, b)| b > *a).collect::<VecDeque<bool>>();
    contains_increase.retain(|x| *x == true);
    let increases_count = contains_increase.len();
    println!("{}", increases_count);
}
