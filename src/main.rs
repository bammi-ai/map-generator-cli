extern crate bammi_ai_map_generator;

use std::env;
use std::process;

struct Config {
    width: usize,
    height: usize,
    is_repeated: bool,
    probability: f64,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, String> {
        if args.len() != 5 {
            return Err("Must pass 4 arguments".to_string());
        }
        let width = args[1].parse().unwrap();
        let height = args[2].parse().unwrap();
        let is_repeated = args[3].parse().unwrap();
        let probability = args[4].parse().unwrap();
        Ok(Config {
            width,
            height,
            is_repeated,
            probability,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_| {
        println!("usage: {} width height is_repeated probability", args[0]);
        process::exit(0);
    });

    let map = bammi_ai_map_generator::Map::new(
        config.width,
        config.height,
        config.is_repeated,
        config.probability,
    );
    let (width, height, _) = map.size();

    for i in 0..height {
        for j in 0..width {
            print!(" {}", map.map()[i][j]);
        }
        println!();
    }
}
