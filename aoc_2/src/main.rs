use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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

#[derive(Hash, Eq, PartialEq, Debug)]
struct Round {
    opponent_move: String,
    your_move: String,
}

fn score_1(round: &Round) -> u32 {
    let move_score = HashMap::from([
        ("X", 1u32),
        ("Y", 2u32),
        ("Z", 3u32),
    ]);
    let win_list = HashMap::from([
        ("A", "Y"),
        ("B", "Z"),
        ("C", "X")
    ]);
    let draw_list = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z"),
    ]);
    let m_score = move_score.get(round.your_move.as_str()).unwrap();
    let w_score = if *win_list.get(round.opponent_move.as_str()).unwrap() == round.your_move {
        6u32
    } else if *draw_list.get(round.opponent_move.as_str()).unwrap() == round.your_move {
        3u32
    } else {
        0u32
    };
    m_score + w_score
}

fn score_2(round: &Round) -> u32 {
    let move_score = HashMap::from([
        ("Stone", 1u32),
        ("Paper", 2u32),
        ("Scissors", 3u32),
    ]);
    let win_list = HashMap::from([
        ("A", "Paper"),
        ("B", "Scissors"),
        ("C", "Stone")
    ]);
    let draw_list = HashMap::from([
        ("A", "Stone"),
        ("B", "Paper"),
        ("C", "Scissors"),
    ]);
    let lose_list = HashMap::from([
        ("A", "Scissors"),
        ("B", "Stone"),
        ("C", "Paper"),
    ]);
    let (my_move, w_score) = if round.your_move == "X" {
        (*lose_list.get(round.opponent_move.as_str()).unwrap(), 0u32)
    } else if round.your_move == "Y" {
        (*draw_list.get(round.opponent_move.as_str()).unwrap(), 3u32)
    } else {
        (*win_list.get(round.opponent_move.as_str()).unwrap(), 6u32)
    };
    w_score + move_score.get(my_move).unwrap()
}


fn task_1(matrix: &Vec<Round>) -> u32 {
    let matrix = matrix.clone();
    matrix.into_iter().map(| round| score_1(round)).sum()
}

fn task_2(matrix: &Vec<Round>) -> u32 {
    let matrix = matrix.clone();
    matrix.into_iter().map(| round| score_2(round)).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read(path)
        .into_iter()
        .map(| line | {
            line.split_once(" ").map(| elem | Round { opponent_move: elem.0.to_string(), your_move: elem.1.to_string() }).unwrap()
        })
        .collect();
    println!("Task 1 result: {}", task_1(&input));
    println!("Task 2 result: {}", task_2(&input));
}
