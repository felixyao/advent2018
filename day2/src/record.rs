use super::id::ID;
use std::vec::Vec;

pub struct Record {
    ids: Vec<String>,
    two_sum: u32,
    three_sum: u32,
}

impl Record {
    pub fn new() -> Self{
        Record{
            ids: Vec::new(),
            two_sum: 0,
            three_sum: 0,
        }
    }
    pub fn add_id(&mut self, id: ID){

        if id.check_two() {
            self.two_sum += 1;
        }

        if id.check_three() {
            self.three_sum += 1;
        }
        self.ids.push(id.into());
     }

     pub fn checksum(&self) -> u32 {
         self.two_sum * self.three_sum
     }

     pub fn find_one_difference_ids(&self)
     {
         let len = self.ids.len();
         for i in 0..(len - 1) {
             for j in (i + 1) .. len {
                 let id1 = self.ids.get(i).unwrap();
                 let id2 = self.ids.get(j).unwrap();
                 let difference = ID::compare(id1, id2);
                 if difference.len() == 1 {
                    let id:String = id1.char_indices()
                                       .filter_map(|(i, c)| {
                                           if i == difference[0] {
                                               return None;
                                           }
                                           return Some(c)
                                       })
                                       .collect();
                    println!("{} {} {}",id1, id2, id);
                    
                 }                
             }
         }

     }
}
