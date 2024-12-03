use std::collections::HashMap;

fn part_one(left_list: &Vec<i32>, right_list: &Vec<i32>) {
    let mut left_list = left_list.to_owned();
    let mut right_list = right_list.to_owned();

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        total_distance += (left - right).abs();
    }

    println!("{}", total_distance);
}

fn part_two(left_list: &Vec<i32>, right_list: &Vec<i32>) {
    let mut right_freqs = HashMap::new();

    for loc_id in right_list {
        *right_freqs.entry(loc_id).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for loc_id in left_list {
        similarity_score += loc_id * *right_freqs.get(&loc_id).unwrap_or(&0);
    }

    println!("{}", similarity_score);
}

fn main() -> std::io::Result<()> {
    let input = aoc_rs::read_input(2024, 1)?;

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut line = line.split_whitespace();
        let left: i32 = line.next().unwrap().parse().unwrap();
        let right: i32 = line.next().unwrap().parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }

    part_one(&left_list, &right_list);
    part_two(&left_list, &right_list);

    Ok(())
}
