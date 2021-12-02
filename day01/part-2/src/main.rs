use std::collections::VecDeque;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong when reading a file");
    let lines_str = contents.lines();
    let lines = lines_str.map(|l| l.parse::<i32>().unwrap()).collect::<VecDeque<i32>>();
    let first_element = *lines.front().unwrap();
    let mut lines_shifted_by_one = lines.clone();
    lines_shifted_by_one.push_front(first_element);
    let mut lines_shifted_by_two = lines.clone();
    lines_shifted_by_two.push_front(first_element);
    lines_shifted_by_two.push_front(first_element);

    let triples = lines_shifted_by_two
        .iter()
        .zip(lines_shifted_by_one.iter())
        .zip(lines.iter())
        .map(|((a, b), c)| (a, b, c)).skip(2);

    let triples_sums = triples.map(|(a, b, c)| a + b + c).collect::<VecDeque<i32>>();

    let mut triples_sums_shifted = triples_sums.clone();
    triples_sums_shifted.push_front(*triples_sums_shifted.front().unwrap());
    let triples_zipped = triples_sums_shifted.iter().zip(triples_sums);

    let mut contains_increase = triples_zipped.map(|(x, y)| y > *x).collect::<VecDeque<bool>>();
    contains_increase.retain(|x| *x == true);
    let increases_count = contains_increase.len();
    println!("{}", increases_count);
}
