extern crate bammi_ai_map_generator;

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter width:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let width: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter height:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let height: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter a bool value (true or false):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let is_repeated: bool = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter wall probability:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let p: f64 = input.trim().parse().expect("Invalid input");

    let map = bammi_ai_map_generator::Map::new(width, height, is_repeated, p);

    for i in 0..map.height {
        for j in 0..map.width {
            print!(" {}", map.map[i][j]);
        }
        println!();
    }
}
