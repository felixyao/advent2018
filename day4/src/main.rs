extern crate regex;
extern crate input_parser;
mod time_record;


fn puzzle1(record: &time_record::TimeRecord) {
    let id =  record.get_max_sleeping_time_guard_id();
    let (most_sleeping_minute, _) = record.get_guard_most_sleeping_minute(id);
    println!("answer is {} x {} = {} ", id, most_sleeping_minute, id * most_sleeping_minute);

}

fn puzzle2(record: &time_record::TimeRecord) {
    let (id,  most_sleeping_minute) =  record.get_most_sleeping_minute();
    println!("answer is {} x {} = {} ", id, most_sleeping_minute, id * most_sleeping_minute);

}

fn main() {
    let record = time_record::TimeRecord::new("inputs/day4.txt");
    puzzle1(&record);
    puzzle2(&record);
}
