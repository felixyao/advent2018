extern crate input_parser;
use input_parser::for_each_line;
mod id;
mod record;

fn puzzle1(record: &mut record::Record) {
    for_each_line("inputs/day2.txt", |id: id::ID| {
        record.add_id(id);
    });
    println!("checksum {}", record.checksum());
}

fn puzzle2(record: &record::Record) {
    record.find_one_difference_ids();
}

fn main() {
   let mut record = record::Record::new();
   puzzle1(&mut record);
   puzzle2(&record);
}
