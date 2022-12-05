use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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


fn task_1(matrix: &Vec<Vec<u32>>) -> u32 {
    let sum_vec: Vec<u32> = matrix.into_iter().map(| arr | arr.iter().sum()).collect();
    *sum_vec.iter().max().unwrap()
}

fn task_2(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut sum_vec: Vec<u32> = matrix.into_iter().map(| arr | arr.iter().sum()).collect();
    sum_vec.sort_unstable();
    let top_3: Vec<&u32> = sum_vec.iter().rev().take(3).collect();
    println!("Task 2 top 3: {:?}", top_3);
    top_3.into_iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read(path)
        .into_iter()
        .fold(vec![], | mut acc, val | {
            if acc.is_empty() || val.is_empty() {
                acc.push(vec![])
            }
            if !val.is_empty() {
                let last_vec = acc.last_mut().unwrap();
                last_vec.push(val.parse::<u32>().unwrap())
            }
            acc
        });
    println!("Task 1 result: {}", task_1(&input));
    println!("Task 2 result: {}", task_2(&input));
}
