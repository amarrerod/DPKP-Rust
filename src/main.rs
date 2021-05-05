mod dpkp;
use std::env;
use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!(
            "Error, expecting instance_path output_filename. Got {:?}",
            args
        );
    } else {
        println!("Dynamic Programming Knapsack Problem in Rust!");
        let instance_path = args[1].clone();
        let results_path = args[2].clone();
        let mut algorithm = dpkp::dpkp_algorithm::KnapsackDP {
            filename: instance_path,
            ..Default::default()
        };

        algorithm.load_instance().ok();
        let (best, time) = algorithm.run();
        // Open a file in write-only mode, returns `io::Result<File>`
        let file = File::create(&results_path).unwrap();
        let mut writer = BufWriter::new(&file);

        write!(&mut writer, "{}\n{}", best, time);
    }
}
