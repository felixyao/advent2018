extern crate input_parser;
use input_parser::for_each_line;

use std::convert::From;
use std::convert::Into;

use std::collections::HashSet;

struct Number {
    v: i32,
}

impl From<String> for Number {
    fn from(input: String) -> Self
    {
        Number {
            v: input.parse::<i32>().unwrap(),
        }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.v
    }
}

fn puzzle1(all_frequencies: &mut Vec<i32>) {
    let mut frequency_final: i32 = 0;
    for_each_line("inputs/day1.txt", |frequency: Number| {
        let f:i32 = frequency.into();
        frequency_final += f;
        all_frequencies.push(f);
    });
    println!("frequency {}", frequency_final);
}

fn puzzle2(all_frequencies: &Vec<i32>) {
    let mut frequency_final: i32 = 0;
    let mut frequencies : HashSet<i32> = HashSet::new();
    frequencies.insert(frequency_final);
    loop {
        for &f in all_frequencies {
            frequency_final += f;
            if frequencies.contains(&frequency_final) {
                println!("first reach twice frequency {}", frequency_final);
                return;
            } else {
                frequencies.insert(frequency_final);
            }
        }
    }
}

fn main() {
    let mut all_frequencies: Vec<i32> = Vec::new();
    puzzle1(&mut all_frequencies);
    puzzle2(&all_frequencies);
}
