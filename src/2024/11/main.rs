use std::collections::HashMap;

fn blink(num: i64) -> Vec<i64> {
    if num == 0 {
        return vec![1];
    }

    let num_str = num.to_string();
    let n = num_str.len();
    if n % 2 == 0 {
        let left: i64 = num_str[..n / 2].parse().unwrap();
        let right: i64 = num_str[n / 2..].parse().unwrap();
        return vec![left, right];
    }

    vec![num * 2024]
}

fn part_one(nums: &Vec<i64>) {
    let mut nums = nums.clone();

    for _ in 0..25 {
        nums = nums.into_iter().flat_map(blink).collect();
    }

    println!("{}", nums.len());
}

fn part_two(nums: &Vec<i64>) {
    let mut freqs: HashMap<i64, i64> = Default::default();
    for num in nums.iter() {
        *freqs.entry(*num).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut next_freqs = HashMap::new();

        for (num, freq) in freqs.iter() {
            for num in blink(*num) {
                *next_freqs.entry(num).or_insert(0) += freq;
            }
        }

        freqs = next_freqs;
    }

    let mut count = 0;
    for freq in freqs.values() {
        count += freq;
    }

    println!("{}", count);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 11)?;

    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    part_one(&nums);
    part_two(&nums);

    Ok(())
}
