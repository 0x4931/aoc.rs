#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn calculate_min_cost(machine: Machine) -> i64 {
    // Can be represented as a system of equations:
    // a * Ax + b * Bx = Cx
    // a * Ay + b * By = Cy
    let Machine {
        button_a,
        button_b,
        prize,
    } = machine;

    let Coord { x: ax, y: ay } = button_a;
    let Coord { x: bx, y: by } = button_b;
    let Coord { x: cx, y: cy } = prize;

    // This problem is stupid... never in the problem was it stated that the system is guaranteed to
    // have one solution. If anything, it implied that multiple solutions could exist by asking to
    // find the "cheapest" solution. The problem is MUCH, MUCH simpler when guaranteed to have one
    // solution, because you don't need to find Null(A). Thanks a lot AoC!
    let det = ax * by - ay * bx;
    assert_ne!(det, 0, "system has multiple solutions");

    let det_a = cx * by - cy * bx;
    let det_b = ax * cy - ay * cx;

    let a = det_a / det;
    let b = det_b / det;

    if a >= 0 && b >= 0 && a * ax + b * bx == cx && a * ay + b * by == cy {
        a * A_COST + b * B_COST
    } else {
        0
    }
}

fn part_one(machines: &Vec<Machine>) {
    let sum: i64 = machines.iter().cloned().map(calculate_min_cost).sum();
    println!("{}", sum);
}

fn part_two(machines: &Vec<Machine>) {
    let sum: i64 = machines
        .iter()
        .cloned()
        .map(|mut machine| {
            machine.prize.x += 10_000_000_000_000;
            machine.prize.y += 10_000_000_000_000;
            calculate_min_cost(machine)
        })
        .sum();
    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 13)?;

    let mut machines: Vec<Machine> = Default::default();

    for machine in input.split("\n\n") {
        let mut lines = machine.lines().map(|line| {
            let coords = line.split_once(": ").unwrap().1;
            let (x, y) = coords.split_once(", ").unwrap();
            let x = x.split_once(&['+', '='][..]).unwrap().1.parse().unwrap();
            let y = y.split_once(&['+', '='][..]).unwrap().1.parse().unwrap();
            Coord { x, y }
        });

        let button_a = lines.next().unwrap();
        let button_b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        let machine = Machine {
            button_a,
            button_b,
            prize,
        };

        machines.push(machine);
    }

    part_one(&machines);
    part_two(&machines);

    Ok(())
}
