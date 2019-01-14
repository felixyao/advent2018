
#[macro_use] extern crate lazy_static;

extern crate input_parser;
use input_parser::for_each_line;
mod coordinate;
mod claim;
mod fabric;

const SIDE_LENGTH: usize = 10000;

fn puzzle1(fabric : &mut fabric::Fabric) {
    for_each_line("inputs/day3.txt", |cut: claim::Cut| {
        fabric.apply(&cut);
    });
    println!("{} square inches are overlapped", fabric.count_overlap());
}

fn puzzle2(fabric: &fabric::Fabric) {
    for_each_line("inputs/day3.txt", |cut: claim::Cut| {
        if !fabric.is_cut_overlapped(&cut) {
            println!("{}", cut);
        }
    });
}

fn main() {
    let mut fabric: fabric::Fabric = fabric::Fabric::new(SIDE_LENGTH);
    puzzle1(&mut fabric);
    puzzle2(&fabric);
}