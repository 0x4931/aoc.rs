use core::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Record {
    time: i64,
    dist: i64,
}

/* Works, but the floats overflow (could switch to i128 but f128 is unstable)
fn solve(record: &Record) -> i64 {
    // Equation: dist = (time - hold_time) * hold_time => -h^2 + th - d = 0
    let Record { time, dist } = record;
    let discrim = time * time - 4 * dist;
    if discrim < 0 {
        return -1;
    }

    let discrim = (discrim as f64).sqrt();
    let time = *time as f64;

    let hold_min = (-time + discrim) / -2f64;
    let hold_max = (-time - discrim) / -2f64;

    let hold_min = hold_min.floor() as i64 + 1;
    let hold_max = hold_max.ceil() as i64 - 1;

    let hold_range = hold_max - hold_min + 1;

    hold_range
}
*/

fn solve(record: &Record) -> i64 {
    // Stupid number overflow made me calculate polynomial roots with binary search :(
    let time = record.time as f64;
    let dist = record.dist as f64;
    let vertex = time / 2f64;

    let hold_min = {
        let mut lower = 0f64;
        let mut upper = vertex;

        while upper - lower > 0.01f64 {
            let middle = (lower + upper) / 2f64;
            let middle_dist = (time - middle) * middle;
            if middle_dist > dist {
                upper = middle;
            } else {
                lower = middle;
            }
        }

        upper
    };

    let hold_max = {
        let mut lower = vertex;
        let mut upper = {
            let mut upper = vertex;
            while (time - upper) * upper > dist {
                upper *= 2f64;
            }
            upper
        };

        while upper - lower > 0.1f64 {
            let middle = (lower + upper) / 2f64;
            let middle_dist = (time - middle) * middle;
            if middle_dist < dist {
                upper = middle;
            } else {
                lower = middle;
            }
        }

        lower
    };

    let hold_min = hold_min.ceil() as i64;
    let hold_max = hold_max.floor() as i64;
    let hold_range = hold_max - hold_min + 1;

    hold_range
}

fn part_one(records: &Vec<Record>) {
    let mut product = 1;
    for record in records.iter() {
        let hold_range = solve(record);
        if hold_range > 0 {
            product *= hold_range;
        }
    }

    println!("{}", product);
}

fn part_two(records: &Vec<Record>) {
    // Fix record (no spaces wtf)
    let count_digits = |mut num: i64| -> u32 {
        let mut count = 0;
        while num > 0 {
            count += 1;
            num /= 10;
        }
        count.max(1)
    };

    let record = records
        .iter()
        .cloned()
        .reduce(|acc, record| Record {
            time: acc.time * 10i64.pow(count_digits(record.time)) + record.time,
            dist: acc.dist * 10i64.pow(count_digits(record.dist)) + record.dist,
        })
        .unwrap();

    let hold_range = solve(&record);

    println!("{}", hold_range);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 6)?;

    let mut records = Vec::new();
    let (time_info, dist_info) = input.split_once("\n").unwrap();
    let (_, time_info) = time_info.split_once(":").unwrap();
    let (_, dist_info) = dist_info.split_once(":").unwrap();
    let time_info = time_info.split_whitespace();
    let dist_info = dist_info.split_whitespace();

    for (time, dist) in time_info.zip(dist_info) {
        let time = time.parse().unwrap();
        let dist = dist.parse().unwrap();
        records.push(Record { time, dist });
    }

    part_one(&records);
    part_two(&records);

    Ok(())
}
