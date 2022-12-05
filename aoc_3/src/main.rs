use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Cannot open file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Cannot parse string").trim().to_string())
        .collect()
}

fn calculate_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        item as u32 - b'a' as u32 + 1
    } else if item.is_ascii_uppercase() {
        item as u32 - b'A' as u32 + 27
    } else {
        panic!("Invalid input")
    }
}


fn task_1(matrix: &Vec<String>) -> u32 {
    let mapping: Vec<(&str, &str)> = matrix.iter()
        .map(| line | {
            line.split_at(line.len() / 2)
        })
        .collect();
    mapping.into_iter().map(| (bag1_str, bag2_str)| {
        let bag_1: HashSet<char> = HashSet::from_iter(bag1_str.chars());
        let bag_2: HashSet<char> = HashSet::from_iter(bag2_str.chars());
        let overlapping_char = bag_1.intersection(&bag_2).next().unwrap().clone();
        calculate_priority(overlapping_char)
    }).sum()
}

fn task_2(matrix: &Vec<String>) -> u32 {
    let mapping: Vec<&[String]> = matrix
        .chunks(3)
        .collect();
    mapping.into_iter().map(| lines | {
        let bag_1: HashSet<char> = HashSet::from_iter(lines[0].chars());
        let bag_2: HashSet<char> = HashSet::from_iter(lines[1].chars());
        let bag_3: HashSet<char> = HashSet::from_iter(lines[2].chars());
        let item_set = &(&bag_1 & &bag_2) & &bag_3;
        let item = item_set.into_iter().next().unwrap();
        calculate_priority(item)
    }).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read(path);
    println!("Task 1 result: {}", task_1(&input));
    println!("Task 2 result: {}", task_2(&input));
}
