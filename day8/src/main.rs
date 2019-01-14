use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn read_input(filename: &str) -> Vec<u8> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.split(b' ').map(|token| -> u8 {
        String::from_utf8(token.unwrap()).unwrap().parse::<u8>().unwrap()
    }).collect()
}

fn caculate_tree_metadate_sum_puzzel1<'a, T>(iter: &mut T) -> u32 
where 
    T: Iterator<Item=&'a u8>
{
    let &children = iter.next().unwrap();
    let &metedata_nodes = iter.next().unwrap();
    let mut metadata:u32 = 0;
    for _ in 0..children {
        metadata += caculate_tree_metadate_sum_puzzel1(iter);
    }
    for _ in 0..metedata_nodes {
        metadata+=*iter.next().unwrap() as u32;
    }
    metadata
}

fn caculate_tree_metadate_sum_puzzel2<'a, T>(iter: &mut T) -> u32 
where 
    T: Iterator<Item=&'a u8>
{
    let &children = iter.next().unwrap();
    let &metedata_nodes = iter.next().unwrap();
    let mut metadata:u32 = 0;
    if children != 0 {
        let children_metadatas:Vec<u32> = (0..children)
                                        .map(|_| caculate_tree_metadate_sum_puzzel2(iter))
                                        .collect();
        for _ in 0..metedata_nodes {
            let index = *iter.next().unwrap() as usize;
            if index <= children as usize {
                metadata += children_metadatas[index-1];
            }
        }
    } else {
        for _ in 0..metedata_nodes {
             metadata+=*iter.next().unwrap() as u32;
        }
    }
    metadata
}

fn main() {
    let input = read_input("inputs/day8.txt");
    println!("{}", caculate_tree_metadate_sum_puzzel1(&mut input.iter()));
    println!("{}", caculate_tree_metadate_sum_puzzel2(&mut input.iter()));
}
