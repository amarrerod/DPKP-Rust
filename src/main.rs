
mod dpkp;

fn main() {
    println!("Dynamic Programming Knapsack Problem in Rust!");
    let mut algorithm = dpkp::dpkp_algorithm::KnapsackDP {
        filename: "".to_string(),
        path_to_results: "/home/".to_string(),
        ..Default::default()
    };

    algorithm.load_instance();
    println!("{}", algorithm);
}
