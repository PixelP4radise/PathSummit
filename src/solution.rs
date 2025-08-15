use rand::Rng;
use crate::configuration::Graph;
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
    RandomV1,
    RandomV2
}

impl Solution {
    fn new(size: usize) -> Self {
        Self{
            cost: Invalid,
            selection_mask: vec![false; size],
        }
    }

    fn generate_solution(&mut self, subgroup_vert_num: u8) {
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
            SolutionType::RandomV1 => {self.random_v1()}
            SolutionType::RandomV2 => {self.random_v2()}
        }
    }

    fn random_v1(&self) -> Self {
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

    fn random_v2(&self) -> Self {
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

    fn is_valid(&self, graph: &Graph) -> bool {
            let size = self.selection_mask.len();

        for v in 0..size {
            if self.selection_mask[v] {
                let mut has_connection = false;

                for u in 0..size {
                    if v!=u && self.selection_mask[u]{
                        if let Some(_) = graph.get_cost(v as u16 + 1, u as u16 + 1){
                            has_connection = true;
                            break;
                        }
                    }
                }

                if !has_connection {
                    return false;
                }
            }
        }

        true
    }
}