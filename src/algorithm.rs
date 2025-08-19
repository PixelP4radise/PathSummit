use crate::configuration::Graph;
use crate::solution::{Solution, SolutionType};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

pub enum Algorithm {
    HillClimbing,
}

impl Algorithm {
    pub fn run(
        &self,
        initial_solution: &Solution,
        edges_cost: Arc<Graph>,
        iterations: usize,
        accept_invalids: bool,
    ) -> Solution {
        match self {
            Algorithm::HillClimbing => {
                self.hill_climber(initial_solution, edges_cost, iterations, accept_invalids)
            }
        }
    }

    pub fn hill_climber(
        &self,
        initial_solution: &Solution,
        edges_cost: Arc<Graph>,
        iterations: usize,
        accept_invalids: bool,
    ) -> Solution {
        let (tx, rx) = mpsc::channel();

        let best_solution = Arc::new(RwLock::new(initial_solution.clone()));
        let iterations = Arc::new(Mutex::new(iterations));

        for _ in 0..2 {
            let iterations = iterations.clone();
            let best_solution = best_solution.clone();
            let thread_tx = tx.clone();
            let edges_cost = edges_cost.clone();

            thread::spawn(move || loop {
                let mut remaining_iterations = iterations.lock().unwrap();

                if *remaining_iterations == 0 {
                    break;
                }
                *remaining_iterations -= 1;

                drop(remaining_iterations);

                let best_solution = best_solution.read().unwrap();

                let mut neighbor = best_solution.new_solution(SolutionType::RandomV2);

                loop {
                    neighbor.calculate_cost(&edges_cost);

                    if !accept_invalids && !neighbor.is_valid(&edges_cost) {
                        neighbor = best_solution.new_solution(SolutionType::RandomV2);
                    } else {
                        break;
                    }
                }

                drop(best_solution);

                thread_tx.send(neighbor).unwrap();
            });
        }

        drop(tx);

        for neighbor in rx {
            let mut best_solution = best_solution.write().unwrap();

            if neighbor.is_better_than(&best_solution) {
                *best_solution = neighbor;
            }
            drop(best_solution);
        }

        best_solution.read().unwrap().clone()
    }
}
