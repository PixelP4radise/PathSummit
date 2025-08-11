use rand::Rng;
use crate::solution::Cost::Invalid;
#[derive(Clone)]
enum Cost {
    Invalid,
    Valid(u16)
}
#[derive(Clone)]
struct Solution {
    selection_mask: Vec<bool>,
    cost: Cost,
}

enum SolutionType {
    V1,
    V2
}

impl Solution {
    fn new(size: usize) -> Self {
        Self{
            cost: Invalid,
            selection_mask: vec![false; size],
        }
    }

    fn init_solution(&mut self, subgroup_vert_num: u8) {
        let mut rng = rand::rng();
        let size = self.selection_mask.len();

        let mut count = 0;
        while count < subgroup_vert_num as usize {
            let pos = rng.random_range(0..size);
            if !self.selection_mask[pos] {
                self.selection_mask[pos] = true;
                count += 1;
            }
        }
    }

    fn new_solution(&self, solution_type: SolutionType) -> Self {
        match solution_type {
            SolutionType::V1 => {self.new_v1()}
            SolutionType::V2 => {self.new_v2()}
        }
    }

    fn new_v1(&self) -> Self {
        let mut rng = rand::rng();
        let size = self.selection_mask.len();

        // Pick a random position with true
        let mut pos_true = rng.random_range(0..size);
        while !self.selection_mask[pos_true] {
            pos_true = rng.random_range(0..size);
        }

        // Pick a random position with false
        let mut pos_false = rng.random_range(0..size);
        while self.selection_mask[pos_false] {
            pos_false = rng.random_range(0..size);
        }

        // Clone and modify
        let mut new_sol = self.clone();
        new_sol.selection_mask[pos_true] = false;
        new_sol.selection_mask[pos_false] = true;

        new_sol
    }

    fn new_v2(&self) -> Self {
        let mut rng = rand::rng();
        let size = self.selection_mask.len();

        let mut pos_true_1 = rng.random_range(0..size);
        while !self.selection_mask[pos_true_1] {
            pos_true_1 = rng.random_range(0..size);
        }

        let mut pos_true_2 = rng.random_range(0..size);
        while !self.selection_mask[pos_true_2] {
            pos_true_2 = rng.random_range(0..size);
        }

        let mut pos_false_1 = rng.random_range(0..size);
        while self.selection_mask[pos_false_1] {
            pos_false_1 = rng.random_range(0..size);
        }

        let mut pos_false_2 = rng.random_range(0..size);
        while self.selection_mask[pos_false_2] {
            pos_false_2 = rng.random_range(0..size);
        }

        let mut new_sol = self.clone();

        new_sol.selection_mask[pos_true_1] = false;
        new_sol.selection_mask[pos_true_2] = false;
        new_sol.selection_mask[pos_false_1] = true;
        new_sol.selection_mask[pos_false_2] = true;

        new_sol
    }
}