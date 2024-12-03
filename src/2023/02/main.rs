use std::collections::HashMap;

struct Game {
    id: i32,
    sets: Vec<Set>,
}

struct Set {
    cubes: Vec<Cubes>,
}

struct Cubes {
    count: i32,
    color: String,
}

fn part_one(games: &Vec<Game>) {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut sum = 0;
    for game in games.iter() {
        let mut max_shown = HashMap::new();
        for set in game.sets.iter() {
            for cubes in set.cubes.iter() {
                max_shown.insert(
                    cubes.color.clone(),
                    cubes.count.max(*max_shown.get(&cubes.color).unwrap_or(&0)),
                );
            }
        }

        if *max_shown.get(&String::from("red")).unwrap_or(&0) <= MAX_RED
            && *max_shown.get(&String::from("green")).unwrap_or(&0) <= MAX_GREEN
            && *max_shown.get(&String::from("blue")).unwrap_or(&0) <= MAX_BLUE
        {
            sum += game.id;
        }
    }

    println!("{}", sum);
}

fn part_two(games: &Vec<Game>) {
    let mut sum = 0;
    for game in games.iter() {
        let mut max_shown = HashMap::new();
        for set in game.sets.iter() {
            for cubes in set.cubes.iter() {
                max_shown.insert(
                    cubes.color.clone(),
                    cubes.count.max(*max_shown.get(&cubes.color).unwrap_or(&0)),
                );
            }
        }

        let red_cubes = *max_shown.get(&String::from("red")).unwrap_or(&0);
        let green_cubes = *max_shown.get(&String::from("green")).unwrap_or(&0);
        let blue_cubes = *max_shown.get(&String::from("blue")).unwrap_or(&0);

        sum += red_cubes * green_cubes * blue_cubes;
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 2)?;

    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let (game_info, game_data) = line.split_once(": ").unwrap();
        let (_, game_id) = game_info.split_once(" ").unwrap();

        let mut sets = Vec::new();
        for set_data in game_data.split("; ") {
            let mut cubes = Vec::new();
            for (cube_count, cube_color) in set_data
                .split(", ")
                .map(|cubes| cubes.split_once(" ").unwrap())
            {
                cubes.push(Cubes {
                    count: cube_count.parse().unwrap(),
                    color: cube_color.to_string(),
                });
            }
            sets.push(Set { cubes });
        }

        games.push(Game {
            id: game_id.parse().unwrap(),
            sets,
        });
    }

    part_one(&games);
    part_two(&games);

    Ok(())
}
