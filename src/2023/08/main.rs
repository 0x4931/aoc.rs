use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct NodeRaw {
    label: String,
    left: Node,
    right: Node,
}

type Node = Option<Rc<RefCell<NodeRaw>>>;

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn part_one(instructions: &Vec<Instruction>, nodes: &HashMap<String, Node>) {
    let mut steps = 0;

    let instr_len = instructions.len();
    let mut instr_idx = 0;
    let mut node = nodes.get(&String::from("AAA")).unwrap().clone().unwrap();

    while node.borrow().label != "ZZZ" {
        node = match instructions[instr_idx] {
            Instruction::Left => node.borrow().left.clone().unwrap(),
            Instruction::Right => node.borrow().right.clone().unwrap(),
        };

        steps += 1;
        instr_idx = (instr_idx + 1) % instr_len;
    }

    println!("{}", steps);
}

fn part_two(instructions: &Vec<Instruction>, nodes: &HashMap<String, Node>) {
    let instr_len = instructions.len();

    // Amount of steps each XXA node takes to reach XXZ
    let mut node_steps: Vec<i64> = Vec::new();

    for (label, node) in nodes.iter() {
        if !label.ends_with('A') {
            continue;
        }

        let mut steps = 0;
        let mut instr_idx = 0;
        let mut node = node.clone().unwrap();

        while !node.borrow().label.ends_with('Z') {
            node = match instructions[instr_idx] {
                Instruction::Left => node.borrow().left.clone().unwrap(),
                Instruction::Right => node.borrow().right.clone().unwrap(),
            };

            steps += 1;
            instr_idx = (instr_idx + 1) % instr_len;
        }

        node_steps.push(steps);
    }

    let steps_lcm = node_steps
        .iter()
        .cloned()
        .reduce(|acc, steps| {
            let steps_product = acc * steps;
            let steps_gcd = gcd(acc, steps);
            let steps_lcm = steps_product / steps_gcd;
            steps_lcm
        })
        .unwrap();

    println!("{}", steps_lcm);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 8)?;

    let (instructions, nodes_data) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .bytes()
        .map(|b| match b {
            b'L' => Instruction::Left,
            b'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect();
    let mut nodes = HashMap::new();

    for line in nodes_data.lines() {
        let (label, _) = line.split_once(" = ").unwrap();
        let label = label.to_owned();

        let node = NodeRaw {
            label: label.clone(),
            left: None,
            right: None,
        };

        nodes.insert(label, Some(Rc::new(RefCell::new(node))));
    }

    for line in nodes_data.lines() {
        let (label, adjacents) = line.split_once(" = ").unwrap();
        let label = label.to_owned();
        let adjacents = adjacents.split(&['(', ')'][..]).skip(1).next().unwrap();
        let (left, right) = adjacents.split_once(", ").unwrap();
        let left = left.to_owned();
        let right = right.to_owned();

        let node = nodes.get(&label).unwrap().clone().unwrap();
        let left = nodes.get(&left).unwrap().clone();
        let right = nodes.get(&right).unwrap().clone();

        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
    }

    part_one(&instructions, &nodes);
    part_two(&instructions, &nodes);

    Ok(())
}
