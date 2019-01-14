
use std::convert::From;
use input_parser::for_each_line;
use std::collections::BTreeMap;
use std::collections::HashMap;

mod month_day;
mod timeline;
mod action;
mod datetime;

enum Status {
    Sleeping(i32),
    Awake(i32),
}


#[derive(Debug)]
pub struct TimeRecord {
    records: BTreeMap<month_day::MonthDay, BTreeMap<datetime::DateTime, action::Action>>,
}

impl TimeRecord {
    pub fn new(file: &str) -> Self {
        let mut records: BTreeMap<month_day::MonthDay, BTreeMap<datetime::DateTime, action::Action>> = BTreeMap::new();
        for_each_line(file, |entry: LogEntry| {
            if !records.contains_key(&entry.date) {
                records.insert(entry.date, BTreeMap::new());
            }
            let actions = records.get_mut(&entry.date).unwrap();
            actions.insert(entry.hour, entry.action);
            
        });
        /*
        for (k, d) in records.iter() {
            println!("{:?}  {:?}", k,  d.keys());
            println!("{:?}  {:?}", k, d.values());
        }
        */
        TimeRecord{
            records:records,
        }
    }

    pub fn get_max_sleeping_time_guard_id(&self) -> u32 {
        let mut gaurds_sleeping_time: HashMap<u32, i32> = HashMap::new();
        for day_time in self.records.values() {
            let mut id:u32 = 0;
            let mut last_status: Status  = Status::Awake(0);
            let mut sleep_time: i32 = 0;
            for (&minute, &action) in day_time.iter() {   
                match action {
                    action::Action::Shift(i) => {id = i},
                    action::Action::Sleep => {
                        last_status = Status::Sleeping(minute.minute)
                    },
                    action::Action::Wakeup => {
                        if let Status::Sleeping(v) = last_status {
                            sleep_time += minute.minute - v;
                            last_status = Status::Awake(minute.minute);
                        } 

                    }
                }

            }
            let v = gaurds_sleeping_time.get(&id).map_or(sleep_time, |&v| v + sleep_time);
            gaurds_sleeping_time.insert(id, v);
        }
        let mut max_sleeping_time: i32 = 0;
        let mut max_sleeping_gurad: u32 = 0;
        for (&id, &time) in gaurds_sleeping_time.iter() {
            if time > max_sleeping_time {
                max_sleeping_time = time;
                max_sleeping_gurad = id;
            }
        }
        //println!("guard {} sleep {}", max_sleeping_gurad, max_sleeping_time);
        max_sleeping_gurad
    }

    fn check_guard_id(&self, day: &month_day::MonthDay, id: u32)->bool {
        let day_time = self.records.get(day).unwrap();
        for &action in day_time.values() {
            match action {
                action::Action::Shift(i) => {
                    return i == id;
                },
                _ => { continue;}
            }
        }
        println!("{:?} missing shift action", &day);
        panic!();
    }
    fn check_guard_is_sleeping_at(&self, day: &month_day::MonthDay, minute: i32) -> u32 {
        let day_time = self.records.get(day).unwrap();
        let mut last_status: Status  = Status::Awake(0);
        for (&m, &action) in day_time.iter() {
            if m.minute > minute {
                break;
            }
            last_status = match action {
                action::Action::Sleep =>  Status::Sleeping(m.minute),
                action::Action::Wakeup => Status::Awake(m.minute),
                action::Action::Shift(_) => Status::Awake(m.minute),
            };
            if m.minute == minute {
                break;
            }
        }
        match last_status {
            Status::Sleeping(_) => 1,
            Status::Awake(_) => 0,
        }
    }

    fn get_gaurd_sleeping_minute_days(&self, id: u32, minute: i32) -> u32 {
        self.records.keys()
        .filter(|key| self.check_guard_id(key, id))
        .map(|key| self.check_guard_is_sleeping_at(key, minute))
        .sum()
    }

    pub fn get_guard_most_sleeping_minute(&self, id: u32) -> (u32, u32) {
        let mut max:u32 = 0;
        let mut mm:u32 = 0;
        for i in 0 .. 60 {
            let days = self.get_gaurd_sleeping_minute_days(id, i);
            if days > max {
                max = days;
                mm = i as u32;
            }
        }
        (mm, max)
    }

    pub fn get_most_sleeping_minute(&self) -> (u32, u32) {
        let mut gaurds_sleeping_time: HashMap<u32, (u32, u32)> = HashMap::new();
        for day_time in self.records.values() {
            let mut id:u32 = 0;
            for &action in day_time.values() {   
                match action {
                    action::Action::Shift(i) => {id = i},
                    action::Action::Sleep => {
                        break;
                    },
                    action::Action::Wakeup => {
                        break;
                    }
                }

            }
            if gaurds_sleeping_time.contains_key(&id) {
                continue;
            }
            let days_minute = self.get_guard_most_sleeping_minute(id);
            gaurds_sleeping_time.insert(id, days_minute);
        }
        let mut max_sleeping_days: u32 = 0;
        let mut max_sleeping_gurad: u32 = 0;
        let mut max_sleeping_min: u32 = 0;
        for (&id, &(minute, days)) in gaurds_sleeping_time.iter() {
            //println!("guard {} spend {} on minute {}", id, days, minute);
            if days > max_sleeping_days {
                max_sleeping_days = days;
                max_sleeping_gurad = id;
                max_sleeping_min = minute;
            }
        }
        (max_sleeping_gurad, max_sleeping_min)
    }
}


#[derive(Debug)]
struct LogEntry {
    date: month_day::MonthDay,
    hour: datetime::DateTime,
    action: action::Action,
}

impl From<String> for LogEntry {
    fn from(input: String) -> Self {
        let re = regex::Regex::new(r"\[\d+-(\d+)-(\d+) (\d+):(\d+)\] (.*)").unwrap();
        let cap = re.captures(&input).unwrap();
        let month = cap[1].parse::<u32>().unwrap();
        let day = cap[2].parse::<u32>().unwrap();
        let hour = cap[3].parse::<u32>().unwrap();
        let mut minute = cap[4].parse::<i32>().unwrap();
        let mut md =  month_day::MonthDay::new(month, day);
        if hour == 23 {
            minute -= 60; 
            md.add_one_day();
        }
        LogEntry {
            date: md,
            hour: datetime::DateTime::new(minute),
            action: action::Action::from(&cap[5]),
        }
    }
}

