use std::fs;

fn main() {
    let file_path = "inputs/in-d5.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");
}
