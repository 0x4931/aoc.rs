struct Space {
    nonempty: i32,
    empty: i32,
}

fn transform_map(map: Vec<i32>) -> Vec<Space> {
    map.into_iter()
        .enumerate()
        .map(|(i, count)| match i % 2 {
            0 => Space {
                nonempty: count,
                empty: 0,
            },
            1 => Space {
                nonempty: 0,
                empty: count,
            },
            _ => unreachable!(),
        })
        .collect()
}

fn part_one(map: &Vec<i32>) {
    let mut map = map.clone();
    let n = map.len();

    let mut sum = 0;
    let mut idx = 0;
    let mut left = 0;
    let mut right = n - 1;

    // If right points to free space
    if right % 2 != 0 {
        right -= 1;
    }

    while left < n {
        if left % 2 == 0 {
            // Nonempty blocks
            if map[left] <= 0 {
                left += 1;
                continue;
            }

            let id = left / 2;
            sum += idx * id;
            idx += 1;
            map[left] -= 1;
        } else {
            // Empty blocks
            if left > right {
                left += 1;
                continue;
            }

            if map[left] <= 0 {
                left += 1;
                continue;
            }

            if map[right] <= 0 {
                right -= 2;
                continue;
            }

            let id = right / 2;
            sum += idx * id;
            idx += 1;
            map[left] -= 1;
            map[right] -= 1;
        }
    }

    println!("{}", sum);
}

fn part_two(map: &Vec<i32>) {
    let mut map = transform_map(map.clone());
    let n = map.len();

    let mut sum = 0;
    let mut idx = 0;
    let mut left = 0;

    while left < n {
        if left % 2 == 0 {
            // Nonempty blocks
            while map[left].nonempty > 0 {
                let id = left / 2;
                sum += idx * id;
                idx += 1;
                map[left].nonempty -= 1;
            }

            while map[left].empty > 0 {
                idx += 1;
                map[left].empty -= 1;
            }
        } else {
            // Empty blocks
            let mut right = n - 1;
            if right % 2 != 0 {
                right -= 1;
            }

            // This is O(n^2), although O(nlog(n)) is also possible using a heap
            while left <= right {
                if map[left].empty >= map[right].nonempty {
                    let id = right / 2;
                    while map[right].nonempty > 0 {
                        sum += idx * id;
                        idx += 1;
                        map[right].nonempty -= 1;
                        map[right].empty += 1;
                        map[left].empty -= 1;
                    }
                }

                right -= 2;
            }

            while map[left].empty > 0 {
                // sum += idx * 0
                idx += 1;
                map[left].empty -= 1;
            }
        }

        left += 1;
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 9)?;

    let map: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    part_one(&map);
    part_two(&map);

    Ok(())
}
