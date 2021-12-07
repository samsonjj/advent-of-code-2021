#![feature(box_syntax)]
use aoc_util::{solve_and_print, AocResult};

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    println!("Hello, world!");
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    println!("{}", input);
    Ok(3)
}

fn part_2(input: &str) -> AocResult<i32> {
    Ok(5)
}
