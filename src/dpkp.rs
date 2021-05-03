

pub mod dpkp_algorithm {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fmt;

    pub struct KnapsackDP {
        pub filename: String, 
        pub path_to_results: String,
        pub n_items : i32,
        pub capacity: i32,
        pub profits : Vec<i32>,
        pub weights : Vec<i32>,
        pub table : Vec<Vec<i32>>,
        pub elapse_time : f64
    }

    impl Default for KnapsackDP {
        fn default() -> KnapsackDP {
            KnapsackDP {
                filename : "".to_string(),
                path_to_results: "".to_string(),
                n_items: -1,
                capacity: -1,
                profits: vec![],
                weights: vec![],
                table: vec![],
                elapse_time: -1.0,
            }
        }
    }

    impl fmt::Display for KnapsackDP {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "n_items {}, Q = {}\nw = {:?}\np = {:?}", self.n_items, 
            self.capacity, self.weights, self.profits);
            Ok(())
        }
    }

    impl KnapsackDP {
        pub fn load_instance(&mut self) -> std::io::Result<()>  {
            // Cargamos el fichero de instancia
            let file = File::open(self.filename.clone()).expect("Error opening File");
            let mut reader = BufReader::new(file);
            let mut tmp_string = String::new();

            reader.read_line(&mut tmp_string);
            let n_items_and_q : Vec<String> = tmp_string.split_whitespace().map(|s| s.to_string()).collect();
            self.n_items  = n_items_and_q[0].parse::<i32>().unwrap();
            self.capacity = n_items_and_q[1].parse::<i32>().unwrap();
            self.profits.reserve(self.n_items as usize);
            self.weights.reserve(self.n_items as usize);
            
            for(i, line) in reader.lines().enumerate() {
                let w_p : Vec<i32> = line.unwrap().split_whitespace().map(|s| s.to_string().parse().unwrap()).collect();
                if w_p.len() == 2 {
                self.weights.push(w_p[0]);   
                self.profits.push(w_p[1]); 
                }
            }
            Ok(())
        }
    }
}