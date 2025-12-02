#![allow(dead_code)]

const BASE_DIR: &'static str = "C://Users/Mike_/Documents/Rust/advent_of_code/aoc2025/data";

mod day1;
mod day2;

fn main() {
    // let (r1,r2) = day1::run();
    // println!("Day 1 part 1: {}, Day 1 part 2: {}", r1, r2);
    let (r1,r2) = day2::run();
    println!("Day 2 part 1: {}, Day 2 part 2: {}", r1, r2);
}