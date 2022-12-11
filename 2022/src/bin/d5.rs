use std::{collections::VecDeque, fs, str, vec};

struct CargoCrane {
    stacks: Vec<VecDeque<char>>,
}

impl CargoCrane {
    fn new(number_of_stacks: usize) -> Self {
        let initial_stacks = vec![VecDeque::new(); number_of_stacks];

        CargoCrane {
            stacks: initial_stacks,
        }
    }

    fn from_input(stacks_input: &str) -> Self {
        let stacks: Vec<Vec<&str>> = stacks_input
            .split("\n")
            .map(|line| {
                line.as_bytes()
                    .chunks(4)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap()
            })
            .collect();

        let mut cargo_crane = CargoCrane::new(stacks[0].len());

        // Set initial stacks
        stacks.iter().rev().skip(1).for_each(|crates_row| {
            crates_row
                .iter()
                .enumerate()
                .for_each(|(stack_index, crate_position)| {
                    let crate_name = match crate_position.find("[") {
                        Some(index) => crate_position.chars().collect::<Vec<char>>()[index + 1],
                        _ => ' ',
                    };

                    if crate_name != ' ' {
                        cargo_crane.add_crate(stack_index.clone(), crate_name.clone());
                    }
                });
        });

        cargo_crane
    }

    fn move_crate(&mut self, stack_from: usize, stack_to: usize) {
        let moving_crate = self.stacks[stack_from].pop_back().unwrap();
        self.stacks[stack_to].push_back(moving_crate);
    }

    fn move_crates_in_chuncks(&mut self, stack_from: usize, stack_to: usize, chunk_size: usize) {
        let crates_chunk_to_move = self.stacks[stack_from]
            .iter()
            .rev()
            .take(chunk_size)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();

        for moving_crate in crates_chunk_to_move.iter().rev() {
            self.stacks[stack_to].push_back(moving_crate.clone());
            self.stacks[stack_from].pop_back().unwrap();
        }
    }

    fn add_crate(&mut self, stack_to: usize, crate_name: char) {
        self.stacks[stack_to].push_back(crate_name);
    }
}

fn main() {
    let file_path = "inputs/in-d5.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    let (initial_stacks_input, rearrangement_procedures_input) =
        (splitted_input[0], splitted_input[1]);

    // Crane 1
    let mut cargo_crane = CargoCrane::from_input(initial_stacks_input);
    // Make rearrangements
    rearrangement_procedures_input
        .split("\n")
        .for_each(|procedure| {
            if let [_, number, _, from, _, to] = procedure.split(" ").collect::<Vec<&str>>()[..] {
                for _ in 0..number.parse().unwrap() {
                    cargo_crane.move_crate(
                        from.parse::<usize>().unwrap() - 1,
                        to.parse::<usize>().unwrap() - 1,
                    );
                }
            }
        });

    // Create final rearangements string
    let mut final_arrangement = String::new();
    for crates_names in cargo_crane.stacks {
        final_arrangement.push(crates_names.back().unwrap().clone());
        println!("{:?}", crates_names);
    }

    // Crane 2
    let mut cargo_crane_2 = CargoCrane::from_input(initial_stacks_input);
    // Make rearrangements in chunks
    rearrangement_procedures_input
        .split("\n")
        .for_each(|procedure| {
            if let [_, number, _, from, _, to] = procedure.split(" ").collect::<Vec<&str>>()[..] {
                cargo_crane_2.move_crates_in_chuncks(
                    from.parse::<usize>().unwrap() - 1,
                    to.parse::<usize>().unwrap() - 1,
                    number.parse().unwrap(),
                );
            }
        });

    // Create final rearangements string
    let mut final_arrangement_2 = String::new();
    for crates_names in cargo_crane_2.stacks {
        final_arrangement_2.push(crates_names.back().unwrap().clone());
    }

    println!("final arrangement -> {:?}", final_arrangement);
    println!("final arrangement 2 -> {:?}", final_arrangement_2);
}
