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

fn count(matrix: &Vec<((u32, u32), (u32, u32))>, filter_pred: fn(&((u32, u32), (u32, u32))) -> bool) -> u32 {
    matrix.into_iter().filter(| item | {
        filter_pred(item)
    }).count() as u32

}


fn task_1(matrix: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    count(matrix, | ((elf_1_start, elf_1_end), (elf_2_start, elf_2_end)) | {
        elf_2_start >= elf_1_start && elf_2_end <= elf_1_end || elf_1_start >= elf_2_start && elf_1_end <= elf_2_end
    })
}

fn task_2(matrix: &Vec<((u32, u32), (u32, u32))>) -> u32 {
    count(matrix, | ((elf_1_start, elf_1_end), (elf_2_start, elf_2_end)) | {
        elf_1_start <= elf_2_end && elf_1_end >= elf_2_start || elf_2_start <= elf_1_end && elf_2_end >= elf_1_start
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read(path);
    let matrix: Vec<((u32, u32), (u32, u32))> = input.iter().map(| line | {
        let split = line.split_once(",").unwrap();
        let split_1 = split.0.split_once("-").unwrap();
        let split_2 = split.1.split_once("-").unwrap();
        ((split_1.0.parse::<u32>().unwrap(), split_1.1.parse::<u32>().unwrap()), (split_2.0.parse::<u32>().unwrap(), split_2.1.parse::<u32>().unwrap()))
    }).collect();
    println!("Task 1 result: {}", task_1(&matrix));
    println!("Task 2 result: {}", task_2(&matrix));
}
