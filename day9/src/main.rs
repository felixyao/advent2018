extern crate linked_list;
use linked_list::{LinkedList, Cursor};
use std::cmp;

struct Input {
    players: usize,
    last: u32,
}
//const INPUT:Input = Input {players: 9, last:46};
//const INPUT:Input = Input {players: 10, last:1618};
//const INPUT:Input = Input {players: 13, last:7999};
//const INPUT:Input = Input {players: 17, last:1104};
//const INPUT:Input = Input {players: 465, last:71498};
const INPUT:Input = Input {players: 465, last:7149800};

#[inline]
fn _next(current: &mut Cursor<u32>)
{
    match current.next() {
        None => {current.next();},
        Some(_) => (),
    }
}

#[inline]
fn _prev(current: &mut Cursor<u32>)
{
    match current.prev() {
        None => {current.prev();},
        Some(_) => (),
    }
}

fn insert(number: u32, current: &mut Cursor<u32>){
    _next(current);
    current.insert(number);
    current.next();
}

fn go_back_steps(current: &mut Cursor<u32>, steps:usize)->u32 {
    for _ in 0..steps+1 {
       _prev(current);
    }
    let ret = current.remove().unwrap(); 
    _next(current);
    return ret;
}

#[inline]
fn get_player_id(id: u32) -> usize {
    (id as usize) % INPUT.players
}

#[inline]
fn get_marble_number(id: u32) -> u32 {
   (id + 1) as u32
}

fn count_score_puzzle(mut current: Cursor<u32>) -> u32 {
    let mut player_score:[u32; INPUT.players] = [0; INPUT.players];
    let mut max_score = 0;
    for i in 0..INPUT.last{
        let player = get_player_id(i);
        let marble_number = get_marble_number(i);
        match marble_number % 23 {
            0 => {
                let last_score = go_back_steps(&mut current, 7);
                player_score[player]+=last_score + marble_number;
                max_score = cmp::max(player_score[player], max_score);
            },
            _ => {
                insert(marble_number as u32, &mut current);
            }
        }
        
    }
    return max_score;
}

fn main() {
    let mut sequence:LinkedList<u32>  = LinkedList::new();
    sequence.insert(0,0);
    println!("{}", count_score_puzzle(sequence.cursor()));
}
