use std::fs;

fn main() {
    let file_path = "inputs/in-d6.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let subroutine_value = find_subroutine_value(&input, 4);
    let subroutine_value_2 = find_subroutine_value(&input, 14);

    println!("Subroutine value -> {:?}", subroutine_value);
    println!("Subroutine value 2 -> {:?}", subroutine_value_2);
}

fn find_subroutine_value(input: &str, distinct_chars: usize) -> usize {
    let mut subroutine_value = 0;
    for (index, sub_chars) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(distinct_chars)
        .enumerate()
    {
        let is_marker = sub_chars.iter().all(|c| {
            let not_duplicate = sub_chars.iter().filter(|&n| *n == *c).count() < 2;
            not_duplicate
        });

        if is_marker {
            subroutine_value = index + distinct_chars;
            break;
        }
    }
    subroutine_value
}
