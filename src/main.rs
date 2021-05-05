mod dpkp;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Error, expecting instance_path and results_path. Got {:?}",
            args
        );
    } else {
        println!("Dynamic Programming Knapsack Problem in Rust!");
        let instance_path = args[1].clone();
        let results_path = args[2].clone();
        let mut algorithm = dpkp::dpkp_algorithm::KnapsackDP {
            filename: instance_path,
            path_to_results: results_path,
            ..Default::default()
        };

        algorithm.load_instance().ok();
        algorithm.run();
    }
}
