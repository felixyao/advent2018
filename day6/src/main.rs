extern crate input_parser;
extern crate regex;

use input_parser::for_each_line;
use std::convert::From;
use std::iter::Iterator;
use std::iter::IntoIterator;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::{max, min};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
struct Coordinate {
    x:i32,
    y:i32,
}

impl Coordinate {
    fn new(x: i32, y:i32) -> Self {
        Coordinate {
            x:x,
            y:y,
        }
    }

    fn distance(&self, other: &Coordinate) -> usize {
        let mut d: i32 = 0;
        if self.x > other.x {
            d += self.x - other.x;
        } else {
            d += other.x - self.x;
        }

        if self.y > other.y {
            d += self.y - other.y;
        } else {
            d += other.y - self.y;
        }
        return d as usize;
    }
}

impl std::cmp::PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl std::cmp::Eq for Coordinate {}

impl From<String> for Coordinate {
    fn from(input: String) -> Self {
        let re = regex::Regex::new(r"(\d+), (\d+)").unwrap();
        let cap = re.captures(&input).unwrap();
        let x = cap[1].parse::<i32>().unwrap();
        let y = cap[2].parse::<i32>().unwrap();
        Coordinate {
            x:x,
            y:y,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Rectangle {
    top_left: Coordinate,
    bottom_right: Coordinate,
}

impl Rectangle {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Self {
        Rectangle {
            top_left: Coordinate::new(min_x, min_y),
            bottom_right: Coordinate::new(max_x, max_y),
        }
    }
}

struct RectangleIterator<'a> {
    top_left: &'a Coordinate,
    bottom_right: &'a Coordinate,
    current: Coordinate,
}

impl<'a> Iterator for RectangleIterator<'a> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Coordinate> {
        self.current.y += 1;
        if self.current.y > self.bottom_right.y {
            self.current.y = self.top_left.y;
            self.current.x += 1;
        }

        if self.current.x > self.bottom_right.x {
            return None;
        }
        return Some(self.current);
    }
}

impl<'a> IntoIterator for &'a Rectangle {
    type Item = Coordinate;
    type IntoIter = RectangleIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        RectangleIterator {
            top_left: &self.top_left,
            bottom_right: &self.bottom_right,
            current: Coordinate::new(self.top_left.x, self.top_left.y - 1),
        }
    }
}

#[derive(Debug)]
struct MapRecord {
    points: Vec<Coordinate>,
    rect: Rectangle,
}

impl MapRecord {
    fn new(file: &str) -> Self {
        let mut coordinates:Vec<Coordinate> = Vec::new();
        let mut min_x:i32 = std::i32::MAX;
        let mut max_x:i32 = std::i32::MIN;
        let mut min_y:i32 = std::i32::MAX;
        let mut max_y:i32 = std::i32::MIN;
        for_each_line(file, |c: Coordinate| {
            min_x = min(c.x, min_x);
            max_x = max(c.x, max_x);
            min_y = min(c.y, min_y);
            max_y = max(c.y, max_y);
            coordinates.push(c);
            
        });
        MapRecord {
            points: coordinates,
            rect: Rectangle::new(min_x, max_x, min_y, max_y),
        }
    }

    fn get_shortest_distance_point(&self, coordinate: &Coordinate) -> usize {
        let mut min_point_index : usize = 0;
        let mut min_distance: usize = std::usize::MAX;
        let mut index = 1;
        for c in &self.points {
            let d = coordinate.distance(c);
            if d == min_distance {
                min_point_index = 0;
            }
            if d < min_distance {
                min_distance = d;
                min_point_index = index;
            }
            index += 1;
        }
        return min_point_index;
    }

    fn get_total_distance(&self, coordinate: &Coordinate) -> usize {
        let mut total_distance: usize = 0;
        for c in &self.points {
            total_distance += coordinate.distance(c);
        }
        return total_distance;
    }

    fn is_edged_point(&self, coordinate: &Coordinate)->bool {
        return  self.rect.top_left.x == coordinate.x 
        || self.rect.top_left.y == coordinate.y
        || self.rect.bottom_right.x == coordinate.x
        || self.rect.bottom_right.y == coordinate.y;
    }

    fn is_corner_point(&self, coordinate: &Coordinate) -> bool {
        return self.rect.top_left == *coordinate
               || self.rect.top_left == *coordinate;
    }

 
    fn build_closet_map(&self) -> HashMap<Coordinate, usize> {
        let mut map:HashMap<Coordinate, usize> = HashMap::new();
        for c in &self.rect {
            map.insert(c, self.get_shortest_distance_point(&c));
        }
        return map;
    }

    fn get_total_distance_within_limit_coordinate_number(&self, limit_distance: usize) -> usize {
        let mut total_number:usize = 0;
        for c in &self.rect {
            let total_distance = self.get_total_distance(&c);
            if total_distance < limit_distance {
                total_number += 1;
                if self.is_edged_point(&c) {
                    let extra_points_number = (limit_distance - total_distance) / self.points.len();
                    total_number += extra_points_number;
                    if self.is_corner_point(&c) {
                        total_number += extra_points_number;
                        total_number += extra_points_number / 2;
                    }
                }
            }

        }
        return total_number;
    }
}

fn get_biggest_value(hash_map:&HashMap<usize, usize>) -> usize {
    return *hash_map.values().max().unwrap();
}

fn puzzle1(map: &MapRecord) {
    let closet_map = map.build_closet_map();
    let mut sum: HashMap<usize, usize> = HashMap::new();
    let mut infinite_points: HashSet<usize> = HashSet::new();
    for (coordinate, &index) in closet_map.iter() {
        if infinite_points.contains(&index) {
            continue;
        }
        if map.is_edged_point(coordinate) {
            infinite_points.insert(index);
            continue;
        }
        let closet_num = sum.entry(index).or_insert(0);
        *closet_num += 1;
    }
    println!("{}", get_biggest_value(&sum));
}

fn puzzle2(map: &MapRecord) {
    let total_distance_coordinate_number  = map.get_total_distance_within_limit_coordinate_number(10000);
    println!("{}", total_distance_coordinate_number);
}

fn main() {
    let map_record = MapRecord::new("inputs/day6.txt");
    puzzle1(&map_record);
    puzzle2(&map_record);
}
