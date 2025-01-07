fn part_one(locks: &Vec<[i32; 5]>, keys: &Vec<[i32; 5]>) {
    let n = locks.len();
    let m = keys.len();

    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            let fits = (0..5).all(|k| locks[i][k] + keys[j][k] <= 5);
            if fits {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 25)?;

    let mut locks: Vec<[i32; 5]> = Default::default();
    let mut keys: Vec<[i32; 5]> = Default::default();

    for schematic in input.split("\n\n") {
        let schematic: Vec<Vec<_>> = schematic
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(schematic.len(), 7);
        assert_eq!(schematic[0].len(), 5);

        let bucket = if schematic[0][0] == '#' {
            &mut locks
        } else {
            &mut keys
        };

        let mut heights = [0; 5];
        for i in 0..5 {
            for j in 0..5 {
                if schematic[j + 1][i] == '#' {
                    heights[i] += 1;
                }
            }
        }

        bucket.push(heights);
    }

    part_one(&locks, &keys);

    Ok(())
}
