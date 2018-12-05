use std::iter::Iterator;
use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Coordinate {
    x: u32,
    y: u32
}

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Self {
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
    pub left_top : &'a Coordinate,
    pub right_bottom: &'a Coordinate,
    pub current: Coordinate,
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