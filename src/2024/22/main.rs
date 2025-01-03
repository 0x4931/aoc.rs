use std::collections::HashMap;

fn compute_next_secret(mut secret: i64) -> i64 {
    const M: i64 = 1 << 24;
    secret = (secret ^ (secret << 6)) % M; // step 1
    secret = (secret ^ (secret >> 5)) % M; // step 2
    secret = (secret ^ (secret << 11)) % M; // step 3
    secret
}

fn part_one(secrets: &Vec<i64>) {
    let mut sum = 0;
    for mut secret in secrets.iter().cloned() {
        for _ in 0..2000 {
            secret = compute_next_secret(secret);
        }
        sum += secret;
    }

    println!("{}", sum);
}

fn part_two(secrets: &Vec<i64>) {
    let mut sequence_sums = HashMap::new();

    // O(secrets * generations) shouldn't be super bad, but it takes a while to run for some reason?

    for mut secret in secrets.iter().cloned() {
        let mut prices = Vec::new();
        for _ in 0..=2000 {
            prices.push(secret % 10);
            secret = compute_next_secret(secret);
        }

        let mut sequences = HashMap::new();
        for group in prices.windows(5) {
            let a = group[1] - group[0];
            let b = group[2] - group[1];
            let c = group[3] - group[2];
            let d = group[4] - group[3];
            let sequence = (a, b, c, d);
            let price = group[4];

            if !sequences.contains_key(&sequence) {
                sequences.insert(sequence, price);
            }
        }

        for (&sequence, &price) in sequences.iter() {
            *sequence_sums.entry(sequence).or_insert(0) += price;
        }
    }

    let max_sum = sequence_sums.values().max().unwrap();
    println!("{}", max_sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 22)?;

    let secrets: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    part_one(&secrets);
    part_two(&secrets);

    Ok(())
}
