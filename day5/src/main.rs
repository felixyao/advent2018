extern crate input_parser;
use input_parser::from_file;
use std::convert::From;

#[derive(Debug)]
enum Status {
    Init,
    New(u8),
    Swallowing(u8),
    Yield((u8, u8)),
}

fn expected_char(c1: u8 , c2:u8) -> bool {
    let distance = b'a' - b'A';
    if c1 > c2 {
        return c1 - c2 == distance;
    }
    return c2 - c1 == distance;
}

impl Status {
    fn change(&self, c: u8)-> Self {
        match *self {
            Status::Init => return Status::New(c),
            Status::New(c1) => {
                if expected_char(c1, c) {
                    return Status::Swallowing(c);
                }
                return Status::Yield((c1, c));
            },
            Status::Swallowing(_) => {
                return Status::New(c);
            }
            Status::Yield((_, c2)) => {
                if expected_char(c2, c) {
                    return Status::Swallowing(c);
                }
                return Status::Yield((c2, c));
            }
        }
    }
}

struct Parser {  
    input: String,
}

impl Parser {

    fn parse(&self, ins: &String) -> String {
        let mut out = ins.clone();
        loop {
            let mut status = Status::Init;
            let mut output:Vec<u8> = Vec::new();
            let mut swallowed = false;
            for c in out.bytes() {
                status = status.change(c);
                match status {
                    Status::Yield((c, _)) => {
                        output.push(c);
                    },
                    Status::Swallowing(_) => {
                        swallowed = true;
                    },
                    _ => {},
                }
            };
            let last = match status {
                Status::Init => 0,
                Status::New(c1) => c1,
                Status::Swallowing(_) => 0,
                Status::Yield((_, c2)) => c2,
            };
            if last != 0 {
                output.push(last);
            }
            out = String::from_utf8(output).unwrap();
            if !swallowed {
                break;
            }
        }
        out
    }

    fn remove_parse(&self, c: u8) -> String  {
        let distance = b'a' - b'A';
        let cc = c - distance;
        let ins : String = self.input.chars()
                           .filter(|&i| c as char != i && cc as char != i)
                           .collect();
        self.parse(&ins)
    }
}

impl From<String> for Parser {
    fn from(input: String) ->Self {
        Parser {
            input:input,
        }
    }
}


fn puzzle1(p: &Parser) {
    println!("{}", p.parse(&p.input).len());
}

fn puzzle2(p: &Parser) {
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        println!("{} {}", c, p.remove_parse(c as u8).len());
    }
}

fn main() {
    from_file("inputs/day5.txt", |p:Parser| {
        puzzle1(&p);
        puzzle2(&p);
    });
    /*
    let p = Parser{input: String::from("dabAcCaCBAcCcaDA")};
    puzzle1(&p);
    puzzle2(&p);
    */
}
