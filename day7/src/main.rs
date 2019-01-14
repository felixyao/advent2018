extern crate input_parser;
extern crate regex;

use input_parser::for_each_line;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

const INPUT_FILE:&str = "inputs/day7.txt";

/*  example 
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
 */

////////////// puzzle1 ///////////////////////////////////////////////
type Task = u8;
type TaskMap =  HashMap<Task, HashSet<Task>>;
type WaitingQueue = BTreeSet<Task>;

#[derive(Debug)]
struct TaskManager {
    fllow_ups_map: TaskMap,
    dependences_map: TaskMap,
    waiting_queue: WaitingQueue,
}

impl TaskManager {

    fn new() -> Self {
        TaskManager{
            fllow_ups_map: HashMap::new(),
            dependences_map: HashMap::new(),
            waiting_queue: BTreeSet::new(),
        }
    } 

    fn build_tasks_map(&mut self, filename: &str)
    {
        for_each_line(filename, |input: String| {
            let re = regex::Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
            let cap = re.captures(&input).unwrap();
            let depend_task = cap[1].parse::<char>().unwrap() as Task;
            let task = cap[2].parse::<char>().unwrap() as Task;
            self.add_task(task, depend_task);
        });
    }
    
    fn add_task(&mut self, task: Task, depend_task: Task) {
        // depend_task's fllow_up is task
        self.fllow_ups_map.entry(depend_task).or_insert(HashSet::new()).insert(task);
        //task depend on depend_task;
        self.dependences_map.entry(task).or_insert(HashSet::new()).insert(depend_task);
        //task have depend so it shouldn't be in the waiting_queue
        self.waiting_queue.remove(&task);
        
        if !self.dependences_map.contains_key(&depend_task) {
            self.waiting_queue.insert(depend_task);
        }
    }

    fn have_dependences(&self, task: Task) -> bool {
        match self.dependences_map.get(&task) {
            Some(tasks) => !tasks.is_empty(),
            None=>false,
        }
    }

    fn remove_task(&mut self, task:Task) {
        for tasks in self.dependences_map.values_mut() {
            tasks.remove(&task);
        }
        if let Some(fllow_ups) = self.fllow_ups_map.get(&task) {
            for &fllow_up in fllow_ups {
                if !self.have_dependences(fllow_up) {
                    self.waiting_queue.insert(fllow_up);
                }
            }
        }
    }

    fn next_waiting_task(&mut self) -> Option<Task> {
        let mut ret = None;
        if !self.waiting_queue.is_empty() {
            ret = self.waiting_queue.iter().nth(0).map(|&v| v);
            self.waiting_queue.remove(&ret.unwrap());
        }
        return ret;
    }
    //// for puzzel2
    fn has_waiting_task(&self) -> bool {
        !self.waiting_queue.is_empty()
    }
    //// for puzzel2
}

/* puzzle 1 
after build up: 
    TaskManager {
        waiting_queue: {'C'},
        fllow_ups_map: {'D': {'E'}, 'B': {'E'}, 'C': {'A', 'F'}, 'F': {'E'}, 'A': {'B', 'D'}},
        dependences_map: {'A': {'C'}, 'F': {'C'}, 'D': {'A'}, 'B': {'A'}, 'E': {'B', 'F', 'D'}}
    }
after loop 1:
    task = 'C'
    TaskManager {
        waiting_queue: {'A', 'F'},
        fllow_ups_map: {'D': {'E'}, 'B': {'E'}, 'C': {'A', 'F'}, 'F': {'E'}, 'A': {'B', 'D'}},
        dependences_map: {'A': {}, 'F': {}, 'D': {'A'}, 'B': {'A'}, 'E': {'B', 'F', 'D'}}
    } 
after loop 2:
    task = 'A'
    TaskManager {
        waiting_queue: {'B', 'D', 'F'},
        fllow_ups_map: {'D': {'E'}, 'B': {'E'}, 'C': {'A', 'F'}, 'F': {'E'}, 'A': {'B', 'D'}},
        dependences_map: {'A': {}, 'F': {}, 'D': {}, 'B': {}, 'E': {'B', 'F', 'D'}}
    } 
after loop 3:
    task = 'B'
    TaskManager {
        waiting_queue: {'D', 'F'},
        fllow_ups_map: {'D': {'E'}, 'B': {'E'}, 'C': {'A', 'F'}, 'F': {'E'}, 'A': {'B', 'D'}},
        dependences_map: {'A': {}, 'F': {}, 'D': {}, 'B': {}, 'E': {'F', 'D'}}
    }
after loop 4:
    task = 'D'
    TaskManager {
        waiting_queue: {'F'},
        fllow_ups_map: {'D': {'E'}, 'B': {'E'}, 'C': {'A', 'F'}, 'F': {'E'}, 'A': {'B', 'D'}},
        dependences_map: {'A': {}, 'F': {}, 'D': {}, 'B': {}, 'E': {'F'}}
    }
after loop 5:
    task = 'F'
    TaskManager {
        waiting_queue: {'E'},
        fllow_ups_map: {'D': {'E'}, 'B': {'E'}, 'C': {'A', 'F'}, 'F': {'E'}, 'A': {'B', 'D'}},
        dependences_map: {'A': {}, 'F': {}, 'D': {}, 'B': {}, 'E': {}}
    }
after loop 6:
    task = 'E'
    TaskManager {
        waiting_queue: {'E'},
        fllow_ups_map: {'D': {'E'}, 'B': {'E'}, 'C': {'A', 'F'}, 'F': {'E'}, 'A': {'B', 'D'}},
        dependences_map: {'A': {}, 'F': {}, 'D': {}, 'B': {}, 'E': {}}
    } 
*/

fn puzzle1() {
    let mut tm = TaskManager::new();
    tm.build_tasks_map(INPUT_FILE);
    while let Some(task) = tm.next_waiting_task(){
        tm.remove_task(task);
        print!("{}", task as char);
    }
    println!("");
}

////////////// puzzle2 ///////////////////////////////////////////////
type Time = u8;
const MAX_WORKERS: usize = 5;
struct Workers {
    workers:[(Task, Time); MAX_WORKERS],
}

impl Workers {
    fn new()->Self {
        Workers {
            workers: [(0, 0); MAX_WORKERS],
        }
    }

    fn give(&mut self, task: Task) -> bool {
        let base_time = b'A' -1;
        for i in 0..MAX_WORKERS {
            if self.workers[i].1 == 0 {
                self.workers[i] = (task, task - base_time + 60);
                return true;
            }
        }
        return false;
    }

    fn wait_until_one_task_finish(&mut self, tm: &mut TaskManager)-> Option<u32> {
        let min = self.workers.iter().filter(|(_, t)| *t != 0).map(|(_, t)| *t).min()?;
        for t in self.workers.iter_mut().filter(|(_, t)| *t != 0) {
            t.1 -= min;
            if t.1 == 0  {
                tm.remove_task(t.0);
            }
        }
        return Some(min as u32);
    }

    fn has_task(&self) -> bool {
        self.workers.iter().any(|(_, t)| *t != 0)
    }
}

fn puzzle2() {
    let mut tm = TaskManager::new();
    tm.build_tasks_map(INPUT_FILE);
    let mut workers = Workers::new();
    let mut total: u32 = 0;
    while tm.has_waiting_task() || workers.has_task()  {
        while let Some(task) = tm.next_waiting_task(){
            if !workers.give(task) {
                // All works have a taks. waiting for some one free
                total += workers.wait_until_one_task_finish(&mut tm).unwrap();
                workers.give(task);
            }
        }
        // All free tasks have been in working queue. waiting for any one to free then we can process the fllow ups.
        if let Some(time) =  workers.wait_until_one_task_finish(&mut tm) {
            total += time;
        }
    }
    println!("{}", total);
}

fn main() {
   puzzle1();
   puzzle2();
}
