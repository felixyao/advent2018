#[macro_use] extern crate lazy_static;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate myerror;
use myerror::MyResult;

mod coordinate;
mod claim;
mod fabric;


const SIDE_LENGTH: usize = 1000;

fn puzzle1(fabric : &mut fabric::Fabric) -> MyResult<()>  {
    let file = File::open("inputs/day3.txt") ?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        fabric.apply(&line.unwrap())?;
    }
    println!("{} square inches are overlapped", fabric.count_overlap());
    Ok(())
}

fn puzzle2(fabric: &fabric::Fabric) -> MyResult<()>  {
    let file = File::open("inputs/day3.txt") ?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let cut = claim::Cut::new(&line.unwrap())?;
        if !fabric.is_cut_overlapped(&cut) {
            println!("{}", cut);
        }
    }
    Ok(())
}

fn main() -> MyResult<()> {
    let mut fabric: fabric::Fabric = fabric::Fabric::new(SIDE_LENGTH);
    puzzle1(&mut fabric)?;
    puzzle2(&fabric)?;
    Ok(())
}