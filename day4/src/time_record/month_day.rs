use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
pub  struct MonthDay{
    month: u32,
    day: u32,
}

static MONTH_MAX_DAYS: &'static [u32] = &[0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl MonthDay {
    pub fn new(month: u32, day: u32) -> Self {
        MonthDay {
            month: month,
            day: day,
        }
    }

    pub fn add_one_day(&mut self) {
        self.day += 1;
        if self.day > MONTH_MAX_DAYS[self.month as usize] {
            self.month += 1;
            self.day = 1;
        }
    }

    fn key(&self) -> u32 {
        self.month * 32 + self.day
    }
}

impl PartialOrd for MonthDay {
    fn partial_cmp(&self, other: &MonthDay) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MonthDay {
    fn eq(&self, other: &MonthDay) -> bool {
        self.key() == other.key()
    }
}

impl Eq for MonthDay {}


impl Ord for MonthDay {
    fn cmp(&self, other: &MonthDay) -> Ordering {
        self.key().cmp(&other.key())
    } 
}