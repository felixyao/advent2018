use std::convert::From;
use std::convert::Into;

pub struct ID {
    id: String,
    two: u32,
    three: u32,
}

impl ID {
    pub fn check_two(&self) -> bool {
        self.two > 0
    }

    pub fn check_three(&self) -> bool {
        self.three > 0
    }

    pub fn compare(id1: &str, id2: &str) -> Vec<usize> {
        let len1: usize = id1.len();
        let len2: usize = id2.len();
        let mut differences: Vec<usize> = Vec::new();
        assert_eq!(len1, len2);
        for i in 0..len1 {
            if id1.get(i .. i + 1) != id2.get(i .. i + 1) {
                differences.push(i);
            }
        }
        differences
    }
}

impl From<String> for ID {
    fn from(input: String) -> Self {
       
        let mut id:Vec<char> = input.chars().collect();
        id.sort();
        let mut two:u32 = 0;
        let mut three:u32 = 0;
        let mut index_char: char = ' ';
        let mut index_counter: u32 = 1;
        for c in id {
            if c != index_char {
                if index_counter == 2  {
                    two += 1;
                }
                if index_counter == 3 {
                    three += 1;
                }
                index_char = c;
                index_counter = 1;
            } else {
                index_counter += 1;
            }
        }
        if index_counter == 2  {
            two += 1;
        }
        if index_counter == 3 {
            three += 1;
        }
        ID {
            id: input,
            two: two,
            three: three,
        }
    }
}

impl Into<String> for ID {
    fn into(self) -> String {
        self.id
    }
} 