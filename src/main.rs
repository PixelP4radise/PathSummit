use crate::algorithm::Algorithm;
use crate::configuration::get_configuration;
use crate::solution::{Cost, Solution};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;

mod algorithm;
mod configuration;
mod solution;

fn main() {
    let graph_settings = get_configuration("test_files/file5.txt");

    let (tx, rx) = mpsc::channel();

    let start = Instant::now();

    for _ in 0..10 {
        let thread_tx: Sender<Solution> = tx.clone();
        let edges_cost = graph_settings.edges_cost.clone();

        thread::spawn(move || {
            let mut initial_solution = Solution::new(graph_settings.num_vert as usize);

            let start_initial_solution = Instant::now();

            loop {
                initial_solution.generate_initial_solution(graph_settings.subgroup_vert_num);
                initial_solution.calculate_cost(&edges_cost);

                if initial_solution.is_valid(&edges_cost) {
                    break;
                }
            }

            let duration_initial_solution = start_initial_solution.elapsed();

            println!("Generated initial solution in {duration_initial_solution:?}");

            let best_solution: Solution =
                Algorithm::HillClimbing.run(&initial_solution, edges_cost, 50000, false);

            thread_tx.send(best_solution).unwrap();
        });
    }

    drop(tx);

    let mut best_solution = Solution::new(graph_settings.num_vert as usize);
    best_solution.generate_initial_solution(graph_settings.subgroup_vert_num);
    best_solution.calculate_cost(&graph_settings.edges_cost);

    for new_solution in rx {
        if new_solution.is_better_than(&best_solution) {
            best_solution = new_solution;
        }
    }

    let duration = start.elapsed();

    match best_solution.cost {
        Cost::Invalid => {
            println!("Bugou e a melhor solucao e invalida");
        }
        Cost::Valid(cost) => {
            println!("Best cost: {cost} in {duration:?}");
        }
    }
}
