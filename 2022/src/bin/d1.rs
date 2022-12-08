use std::fs;

fn main() {
    let file_path = "inputs/in-d1.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let total_elfs_calories: Vec<i32> = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|cal| cal.parse::<i32>().unwrap()).sum())
        .collect();

    fn get_max(arr: &Vec<i32>) -> i32 {
        *arr.iter().max().unwrap()
    }

    fn get_podium_sum(mut arr: Vec<i32>, podium_size: usize) -> i32 {
        arr.sort();
        arr[arr.len() - podium_size..].to_vec().iter().sum()
    }

    println!("{}", get_max(&total_elfs_calories));
    println!("{}", get_podium_sum(total_elfs_calories.clone(), 3));
}
