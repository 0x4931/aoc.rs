use std::collections::{HashMap, HashSet};

struct Rule {
    before: i32,
    after: i32,
}

fn compute_adjacency_list(rules: &Vec<Rule>) -> HashMap<i32, Vec<i32>> {
    let mut comes_after = HashMap::new();
    for rule in rules.iter() {
        comes_after
            .entry(rule.before)
            .or_insert_with(|| Vec::new())
            .push(rule.after);
    }
    comes_after
}

fn is_valid_ordering(comes_after: &HashMap<i32, Vec<i32>>, ordering: &Vec<i32>) -> bool {
    let mut came_before = HashSet::new();

    for num in ordering.iter() {
        if let Some(comes_after) = comes_after.get(num) {
            for after in comes_after.iter() {
                if came_before.contains(after) {
                    return false;
                }
            }
        }
        came_before.insert(*num);
    }

    true
}

fn part_one(rules: &Vec<Rule>, orderings: &Vec<Vec<i32>>) {
    // technically a graph problem
    let mut sum = 0;

    let comes_after = compute_adjacency_list(rules);
    for ordering in orderings.iter() {
        if is_valid_ordering(&comes_after, ordering) {
            sum += ordering[ordering.len() / 2];
        }
    }

    println!("{}", sum);
}

fn part_two(rules: &Vec<Rule>, orderings: &Vec<Vec<i32>>) {
    let mut sum = 0;

    let comes_after = compute_adjacency_list(rules);
    for ordering in orderings.iter() {
        if is_valid_ordering(&comes_after, ordering) {
            continue;
        }

        let mut ordering = ordering.clone();
        ordering.sort_by(|a, b| {
            use std::cmp::Ordering;

            let empty_after = Vec::new();
            let after_a = comes_after.get(a).unwrap_or(&empty_after);
            let after_b = comes_after.get(b).unwrap_or(&empty_after);

            if after_a.contains(b) {
                return Ordering::Less; // a comes before b
            }

            if after_b.contains(a) {
                return Ordering::Greater; // b comes before a
            }

            Ordering::Equal // idc
        });

        sum += ordering[ordering.len() / 2];
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 5)?;

    let (rule_data, order_data) = input.split_once("\n\n").unwrap();
    let mut rules = Vec::new();
    let mut orderings = Vec::new();

    for rule in rule_data.lines() {
        let (before, after) = rule.split_once("|").unwrap();
        let before: i32 = before.parse().unwrap();
        let after: i32 = after.parse().unwrap();
        let rule = Rule { before, after };
        rules.push(rule);
    }

    for ordering in order_data.lines() {
        let mut nums = Vec::new();
        for num in ordering.split(",") {
            let num: i32 = num.parse().unwrap();
            nums.push(num);
        }
        orderings.push(nums);
    }

    part_one(&rules, &orderings);
    part_two(&rules, &orderings);

    Ok(())
}
