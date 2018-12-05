use std::collections::HashMap;
use super::claim::Cut;

pub struct Fabric {
    side: usize,
    cuts: HashMap<usize, u32>,
}

impl Fabric {
    pub fn new(side: usize) -> Self {
        Fabric {
            side: side,
            cuts: HashMap::new(),
        }
    }

    pub fn apply(&mut self, cut: &Cut) {
        let side = self.side;
        for key in cut.coordinates().map(|c| c.key(side)) {
            let v = self.cuts.get(&key).map_or(1, |&v| v+1);
            self.cuts.insert(key, v);
        };
    }

    pub fn count_overlap(&self) -> usize  {
        self.cuts.values()
        .filter(|&&v| v > 1)
        .count()
    }

    pub fn is_cut_overlapped(&self, cut: &Cut) -> bool  {
        cut.coordinates()
        .map(|c| c.key(self.side))
        .any(|key| self.cuts.get(&key).map(|&v| v > 1).unwrap())
    }
}
