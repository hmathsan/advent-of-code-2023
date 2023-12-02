use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod day_one;
mod day_two;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    println!("Day one solution: \n");
    day_one::run_day_one_challenge();

    println!("\nDay two solution: \n");
    day_two::run_day_two_challenge();
}

