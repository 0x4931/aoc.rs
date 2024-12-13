fn predict_next(mut history: Vec<i32>) -> i32 {
    let mut prediction = 0;

    while history.iter().any(|&v| v != 0) {
        prediction += history.last().unwrap();
        history = history
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
    }

    prediction
}

fn part_one(histories: &Vec<Vec<i32>>) {
    let mut sum = 0;

    for history in histories.iter().cloned() {
        sum += predict_next(history);
    }

    println!("{}", sum);
}

fn part_two(histories: &Vec<Vec<i32>>) {
    let mut sum = 0;

    for mut history in histories.iter().cloned() {
        history.reverse();
        sum += predict_next(history);
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 9)?;

    let mut histories = Vec::new();
    for line in input.lines() {
        let history = line
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        histories.push(history);
    }

    part_one(&histories);
    part_two(&histories);

    Ok(())
}
