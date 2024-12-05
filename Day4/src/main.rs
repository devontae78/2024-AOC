use std::{
    fs::{File, read_to_string},
    io::{self, BufRead, BufReader},
};

type Input = (u32, u32);

fn main() {
    parse("./src/input.txt");
}

pub fn parse(input: &str) -> Input {
    let mut lindex = 0;
    let mut index = 0;
    let mut part_one = 0;
    let mut part_two = 0;

    let files: Vec<String> = read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut line = files[lindex].as_bytes();

    while index < files.len() {
        if line[index] != b'X' {
            index += 1;
            continue;
        }

        if line[index] == b'X' && (line[index + 1] == b'M' || line[index - 1] == b'M') {}
    }

    println!("{:?}", files.len());
    (part_one, part_two)
}

fn part1(input: &Input) -> u32 {
    input.0
}
