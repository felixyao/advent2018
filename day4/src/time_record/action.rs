
use std::convert::From;
use regex::RegexSet;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Action {
    Shift(u32),
    Sleep,
    Wakeup,
}

impl From<&str> for Action {
    fn from(input: &str) -> Self {
        let set = RegexSet::new(&[
            r"falls asleep",
            r"wakes up",
            r"Guard #(\d+) begins shift",
        ]).unwrap();
        let matches = set.matches(input);
        if matches.matched(0) {
            return Action::Sleep;
        }

        if matches.matched(1) {
            return Action::Wakeup;
        }

        if matches.matched(2) {
            let re = regex::Regex::new(&set.patterns()[2]).unwrap();
            let cap = re.captures(&input).unwrap();
            let id = cap[1].parse::<u32>().unwrap();
            return Action::Shift(id);
        }
     
        return Action::Sleep;
    }    
}





