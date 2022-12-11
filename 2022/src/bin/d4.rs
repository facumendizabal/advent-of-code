use std::{fs, ops::Range};

fn main() {
    let file_path = "inputs/in-d4.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let pairs: Vec<Vec<(i32, i32)>> = input
        .split("\n")
        .map(|pair| {
            pair.split(",")
                .map(|sections_range| {
                    let sections_range_vec: Vec<i32> = sections_range
                        .split("-")
                        .map(|section| section.parse::<i32>().unwrap())
                        .collect();
                    (sections_range_vec[0], sections_range_vec[1])
                })
                .collect()
        })
        .collect();

    let total_intersections = pairs.iter().fold(0, |intersections, pair| {
        if sections_range_fully_contains(pair[0], pair[1]) {
            intersections + 1
        } else {
            intersections
        }
    });

    let total_overlaps = pairs.iter().fold(0, |intersections, pair| {
        if sections_range_overlaps(pair[0], pair[1]) {
            intersections + 1
        } else {
            intersections
        }
    });

    println!("Total fully intersections -> {:?}", total_intersections);
    println!("Total overlaps -> {:?}", total_overlaps);
}

fn sections_range_fully_contains(section_range_1: (i32, i32), section_range_2: (i32, i32)) -> bool {
    let (x, y) = section_range_2;
    match section_range_1 {
        (a, b) if a >= x && b <= y => true,
        (a, b) if x >= a && y <= b => true,
        _ => false,
    }
}

fn sections_range_overlaps(section_range_1: (i32, i32), section_range_2: (i32, i32)) -> bool {
    let (x, y) = section_range_2;
    match section_range_1 {
        (a, b) if (a >= x && a <= y) || (b <= y && b >= x) => true,
        (a, b) if (x >= a && x <= b) || (y <= b && y >= a) => true,
        _ => false,
    }
}
