fn count_pattern_arrangements(design: &String, patterns: &Vec<String>) -> usize {
    let n = design.len();
    let mut dp = vec![0; n + 1];

    dp[0] = 1;
    for i in 0..n {
        if dp[i] == 0 {
            continue;
        }
        let design = &design[i..];
        for pattern in patterns.iter() {
            if design.starts_with(pattern) {
                let k = pattern.len();
                if i + k <= n {
                    dp[i + k] += dp[i];
                }
            }
        }
    }

    dp[n]
}

fn part_one(patterns: &Vec<String>, designs: &Vec<String>) {
    let count = designs
        .iter()
        .filter(|design| count_pattern_arrangements(design, patterns) > 0)
        .count();
    println!("{}", count);
}

fn part_two(patterns: &Vec<String>, designs: &Vec<String>) {
    let sum: usize = designs
        .iter()
        .map(|design| count_pattern_arrangements(design, patterns))
        .sum();
    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 19)?;

    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns: Vec<String> = patterns.split(", ").map(str::to_string).collect();
    let designs: Vec<String> = designs.lines().map(str::to_string).collect();

    part_one(&patterns, &designs);
    part_two(&patterns, &designs);

    Ok(())
}
