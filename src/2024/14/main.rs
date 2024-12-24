#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

const SECONDS: i64 = 100;
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn part_one(robots: &Vec<Robot>) {
    let mut robots = robots.clone();

    for robot in robots.iter_mut() {
        robot.position.x += SECONDS * robot.velocity.x;
        robot.position.y += SECONDS * robot.velocity.y;

        robot.position.x = (robot.position.x % WIDTH + WIDTH) % WIDTH;
        robot.position.y = (robot.position.y % HEIGHT + HEIGHT) % HEIGHT;
    }

    let left_cutoff = WIDTH / 2;
    let right_cutoff = (WIDTH - 1) / 2;
    let top_cutoff = HEIGHT / 2;
    let bottom_cutoff = (HEIGHT - 1) / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in robots.iter() {
        let in_left = robot.position.x < left_cutoff;
        let in_right = robot.position.x > right_cutoff;
        let in_top = robot.position.y < top_cutoff;
        let in_bottom = robot.position.y > bottom_cutoff;

        if in_top && in_left {
            top_left += 1;
        }

        if in_top && in_right {
            top_right += 1;
        }

        if in_bottom && in_left {
            bottom_left += 1;
        }

        if in_bottom && in_right {
            bottom_right += 1;
        }
    }

    let safety_factor = top_left * top_right * bottom_left * bottom_right;
    println!("{}", safety_factor);
}

fn part_two(robots: &Vec<Robot>) {
    let mut robots = robots.clone();

    let mut time = 0;
    loop {
        time += 1;

        for robot in robots.iter_mut() {
            robot.position.x = ((robot.position.x + robot.velocity.x) % WIDTH + WIDTH) % WIDTH;
            robot.position.y = ((robot.position.y + robot.velocity.y) % HEIGHT + HEIGHT) % HEIGHT;
        }

        let rows = HEIGHT as usize;
        let cols = WIDTH as usize;
        let mut map = vec![vec![false; cols]; rows];

        for robot in robots.iter() {
            let row = robot.position.y as usize;
            let col = robot.position.x as usize;
            map[row][col] = true;
        }

        // Just check for a grid I guess lmao

        println!("{}", "\n".repeat(HEIGHT as usize));
        print!("\x1B[1;1H"); // positions the cursor at 1,1
        println!("After {} seconds:", time);
        for row in 0..rows {
            for col in 0..cols {
                let c = match map[row][col] {
                    true => '#',
                    false => '.',
                };
                print!("{}", c);
            }
            println!();
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 14)?;

    let mut robots: Vec<Robot> = Default::default();
    for line in input.lines() {
        let mut properties = line.split_whitespace().map(|property| {
            let coord = property.split_once('=').unwrap().1;
            let (x, y) = coord.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Coord { x, y }
        });

        let position = properties.next().unwrap();
        let velocity = properties.next().unwrap();
        let robot = Robot { position, velocity };

        robots.push(robot);
    }

    part_one(&robots);
    part_two(&robots);

    Ok(())
}
