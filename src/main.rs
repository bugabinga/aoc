//! Advent of Code 2018
//! Oliver Jan Krylow
//! Rust learning exercise
//!
//! AoC poses a cute programming challenge each day of December until the 24th.
//! Each daily challenge gets solved in its own rust module.
//! The main program simply prints the results of all solved challenges so far.

mod day_one;
mod day_two;

/// Print the day of the challenge and the result per line of all solved challenges so far.
fn main() {
    println!("Day 1 Part 1: {}", day_one::part_one());
    println!("Day 1 Part 2: {}", day_one::part_two());

    println!("Day 2 Part 1: {}", day_two::part_one());
    println!("Day 2 Part 2: {}", day_two::part_two());
}
