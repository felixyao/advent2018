extern crate regex;

use regex::Regex;
use std::fmt;
use myerror::MyResult;
use super::coordinate::{Coordinate, CoordinateIterator};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Cut{
    id: u32,
    left_top: Coordinate,
    right_bottom: Coordinate
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
            right_bottom:Coordinate::new(x + width - 1, y + height -1)
        })
    }

    pub fn coordinates<'a>(&'a self) -> CoordinateIterator<'a> {
        CoordinateIterator {
            left_top: &self.left_top,
            right_bottom: &self.right_bottom,
            current: Coordinate::new(0, 0),
        }
    }
}

impl fmt::Display for Cut {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id:{} {} {}", self.id, self.left_top, self.right_bottom)
    }
}

