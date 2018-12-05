extern crate regex;

use regex::Regex;
use std::fmt;
use std::iter::Iterator;
use myerror::MyResult;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Coordinate {
    x: u32,
    y: u32
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Self {
        Coordinate {
            x: x,
            y: y
        }
    }

    pub fn key(&self, side: usize) -> usize {
        let lines = self.x as usize;
        let columns = self.y as usize;
        lines * side + columns
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct CoordinateIterator<'a> {
    left_top : &'a Coordinate,
    right_bottom: &'a Coordinate,
    current: Coordinate,
}

impl<'a> Iterator for CoordinateIterator<'a> {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Coordinate> {
        if self.current.x == 0 && self.current.y == 0 {
            self.current = *self.left_top;
            return Some(self.current);
        }

        if self.current.x < self.right_bottom.x {
            if self.current.y < self.right_bottom.y {
                self.current.y += 1;    
            } else {
                self.current.x += 1;
                self.current.y = self.left_top.y;
            }
            return Some(self.current);
        }

        if self.current.y < self.right_bottom.y {
            self.current.y += 1;
            return Some(self.current);
        }
        return None;
    }
} 

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Cut{
    id: u32,
    left_top: Coordinate,
    right_bottum: Coordinate
}

impl Cut {
    pub fn new(input: &String) -> MyResult<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)\D(\d+)$").unwrap();
        }
        let cap = RE.captures(input).unwrap();
        let id = cap[1].parse::<u32>()?;
        let x = cap[2].parse::<u32>()?;
        let y = cap[3].parse::<u32>()?;
        let width = cap[4].parse::<u32>()?;
        let height = cap[5].parse::<u32>()?;
        Ok(Cut {
            id: id,
            left_top: Coordinate::new(x, y),
            right_bottum:Coordinate::new(x + width - 1, y + height -1)
        })
    }

    pub fn coordinates<'a>(&'a self) -> CoordinateIterator<'a> {
        CoordinateIterator {
            left_top: &self.left_top,
            right_bottom: &self.right_bottum,
            current: Coordinate::new(0, 0),
        }
    }
}

impl fmt::Display for Cut {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id:{} {} {}", self.id, self.left_top, self.right_bottum)
    }
}

