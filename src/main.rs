use crate::algorithm::Algorithm;
use crate::configuration::get_configuration;
use crate::solution::{Cost, Solution};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

mod algorithm;
mod configuration;
mod solution;

//lancar threads para cada run
//lancar threads para gerar solucoes para o trepa colinas
//meter uma best solution e po-la dentro de um rwLock para poder ser lida no final da execucao das iteracoes

fn main() {
    let graphSettings = get_configuration("test_files/test.txt");

    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let thread_tx: Sender<Solution> = tx.clone();
        let edges_cost = graphSettings.edges_cost.clone();

        thread::spawn(move || {
            let mut initial_solution = Solution::new(graphSettings.num_vert as usize);

            initial_solution.generate_initial_solution(graphSettings.subgroup_vert_num);
            initial_solution.calculate_cost(&edges_cost);

            let best_solution: Solution =
                Algorithm::HillClimbing.run(&initial_solution, edges_cost, 50000);

            thread_tx.send(best_solution).unwrap();
        });
    }

    drop(tx);

    let mut best_solution = Solution::new(graphSettings.num_vert as usize);
    best_solution.generate_initial_solution(graphSettings.subgroup_vert_num);
    best_solution.calculate_cost(&graphSettings.edges_cost);

    for solution in rx {
        if solution.is_better_than(&best_solution) {
            best_solution = solution;
        }
    }

    match best_solution.cost {
        Cost::Invalid => {
            println!("Bugou e a melhor solucao e invalida");
        }
        Cost::Valid(cost) => {
            println!("Best cost: {cost}");
        }
    }
}
