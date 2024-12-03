use regex::Regex;

fn part_one(programs: &Vec<String>) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut results = 0;
    for program in programs.iter() {
        for (_, [a, b]) in re.captures_iter(program).map(|cap| cap.extract()) {
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();
            results += a * b;
        }
    }

    println!("{}", results);
}

fn part_two(programs: &Vec<String>) {
    let re = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let mut results = 0;
    let mut enabled = true;

    for program in programs.iter() {
        for capture in re.captures_iter(program) {
            if let Some(_) = capture.name("do") {
                enabled = true;
            }

            if let Some(_) = capture.name("dont") {
                enabled = false;
            }

            if let (Some(a), Some(b)) = (capture.name("a"), capture.name("b")) {
                if enabled {
                    let a: i32 = a.as_str().parse().unwrap();
                    let b: i32 = b.as_str().parse().unwrap();
                    results += a * b;
                }
            }
        }
    }

    println!("{}", results);
}

fn main() -> std::io::Result<()> {
    let input = aoc_rs::read_input(2024, 3)?;

    let mut programs = Vec::new();
    for line in input.lines() {
        let line = line.to_string();
        programs.push(line);
    }

    part_one(&programs);
    part_two(&programs);

    Ok(())
}
