use std::cmp::Ordering;


#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
pub  struct DateTime{
    pub minute: i32,
}

impl DateTime {
    pub fn new(minute: i32) -> Self {
        DateTime {
            minute: minute,
        }
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &DateTime) -> bool {
        self.minute == other.minute
    }
}

impl Eq for DateTime {}


impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> Ordering {
        self.minute.cmp(&other.minute)
    } 
}