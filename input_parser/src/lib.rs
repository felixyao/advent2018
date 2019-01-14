
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::convert::From;


pub fn for_each_line<F, T>(file: &str, mut f: F)
where
    T: From<String>,
    F: FnMut(T)
{
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let v: T = From::from(line.unwrap());
        f(v);
    }
}

pub fn from_file<F, T>(file: &str, mut f: F)
where
    T: From<String>,
    F: FnMut(T)
{
    let mut file = File::open(file).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    f(From::from(input));
}