use std::fs;

fn main() {
    let file_path = "inputs/in-d3.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let rucksacks: Vec<&str> = input.split("\n").collect();

    let total_priority: i32 = rucksacks.iter().fold(0, |final_priority, rucksack| {
        let (compartment_1, compartment_2) = rucksack.split_at((rucksack.len()) / 2);
        let shared_item: char = compartment_1
            .chars()
            .reduce(|acum, c| if compartment_2.contains(c) { c } else { acum })
            .unwrap();

        final_priority + get_item_priority(shared_item)
    });

    let total_priority_2 = rucksacks
        .chunks(3)
        .fold(0, |final_priority, rucksack_group| {
            let shared_item: char = rucksack_group[0]
                .chars()
                .reduce(|acum, c| {
                    if rucksack_group[1].contains(c) && rucksack_group[2].contains(c) {
                        c
                    } else {
                        acum
                    }
                })
                .unwrap();

            final_priority + get_item_priority(shared_item)
        });

    println!("Total priority -> {}", { total_priority });
    println!("Total priority 2 -> {}", { total_priority_2 });
}

fn get_item_priority(item: char) -> i32 {
    let item_ascii_value = item as i32;

    if item.is_uppercase() {
        item_ascii_value - 38
    } else {
        item_ascii_value - 96
    }
}
