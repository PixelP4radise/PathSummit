use rand::Rng;
use crate::solution::Cost::Invalid;

enum Cost {
    Invalid,
    Valid(u16)
}

struct Solution {
    selection_cost: Vec<bool>,
    cost: Cost,
}

impl Solution {
    fn new(size: usize) -> Self {
        Self{
            cost: Invalid,
            selection_cost: vec![false; size],
        }
    }

    fn init_solution(&mut self, subgroup_vert_num: u8) {
        let mut rng = rand::rng();
        let size = self.selection_cost.len();

        let mut count = 0;
        while count < subgroup_vert_num as usize {
            let pos = rng.random_range(0..size);
            if !self.selection_cost[pos] {
                self.selection_cost[pos] = true;
                count += 1;
            }
        }
    }

    fn new_solution(&self, ) -> Self {

    }
}