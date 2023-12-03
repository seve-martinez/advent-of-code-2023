use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_digits(line: &str) -> Option<(u32, u32)> {
    let mut nums = line.chars().filter_map(|ch| ch.to_digit(10));
    dbg!(Some((nums.next()?, nums.rev().next().unwrap_or(0))))
}

fn get_more_digits(line: &str) -> (u32, u32) {
    let nums: Vec<u32> = line
        .chars()
        .filter(|ch| ch.is_digit(10))
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();
    (*nums.first().unwrap(), *nums.last().unwrap())
}

fn main() {
    let file = File::open("src/day_1/input.txt").unwrap();
    let buf = BufReader::new(file);
    // let mut output = 0;

    let output: u32 = buf
        .lines()
        .filter_map(|line| get_digits(&line.ok()?))
        .map(|(first, last)| first * 10 + last)
        .sum();

    /*for line in buf.lines() {
        let result = get_digits(&line.unwrap());
        output = result.and_then(|(a, b)| a.and_then(f))
        // output += first * 10 + last;
    }*/

    println!("{}", output);
}
