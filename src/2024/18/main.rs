use std::collections::{HashSet, VecDeque};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

const WIDTH: isize = 70;
const HEIGHT: isize = 70;

fn part_one(incoming: &Vec<Coord>) {
    let corrupted: HashSet<Coord> = incoming.iter().cloned().take(1024).collect();

    let start = Coord { x: 0, y: 0 };
    let end = Coord {
        x: WIDTH,
        y: HEIGHT,
    };

    // Simple BFS due to all steps having the same weight
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start, 0));

    while !to_visit.is_empty() {
        let (coord, steps) = to_visit.pop_front().unwrap();

        if coord == end {
            println!("{}", steps);
            break;
        }

        let directions = &[
            Coord { x: 1, y: 0 },
            Coord { x: -1, y: 0 },
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: -1 },
        ][..];
        for direction in directions {
            let next_coord = Coord {
                x: coord.x + direction.x,
                y: coord.y + direction.y,
            };

            if (0..=WIDTH).contains(&next_coord.x)
                && (0..=HEIGHT).contains(&next_coord.y)
                && !corrupted.contains(&next_coord)
                && visited.insert(next_coord)
            {
                to_visit.push_back((next_coord, steps + 1));
            }
        }
    }
}

fn part_two(incoming: &Vec<Coord>) {
    let mut corrupted = HashSet::new();

    let start = Coord { x: 0, y: 0 };
    let end = Coord {
        x: WIDTH,
        y: HEIGHT,
    };

    for to_corrupt in incoming.iter().cloned() {
        corrupted.insert(to_corrupt);

        let mut visited = HashSet::new();
        visited.insert(start);
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);

        while !to_visit.is_empty() {
            let coord = to_visit.pop_front().unwrap();

            let directions = &[
                Coord { x: 1, y: 0 },
                Coord { x: -1, y: 0 },
                Coord { x: 0, y: 1 },
                Coord { x: 0, y: -1 },
            ][..];
            for direction in directions {
                let next_coord = Coord {
                    x: coord.x + direction.x,
                    y: coord.y + direction.y,
                };

                if (0..=WIDTH).contains(&next_coord.x)
                    && (0..=HEIGHT).contains(&next_coord.y)
                    && !corrupted.contains(&next_coord)
                    && visited.insert(next_coord)
                {
                    to_visit.push_back(next_coord);
                }
            }
        }

        if !visited.contains(&end) {
            println!("{},{}", to_corrupt.x, to_corrupt.y);
            break;
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 18)?;

    let incoming: Vec<Coord> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Coord { x, y }
        })
        .collect();

    part_one(&incoming);
    part_two(&incoming);

    Ok(())
}
