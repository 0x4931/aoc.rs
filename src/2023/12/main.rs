#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    conditions: Vec<Condition>,
    damage_groups: Vec<usize>,
}

fn count_arrangements_rec(
    record: &Record,
    cond_idx: usize,
    mut dmg_idx: usize,
    curr_group: usize,
    mut new_group: usize,
) -> i64 {
    let Record {
        conditions,
        damage_groups,
    } = record;

    let n = conditions.len();
    let m = damage_groups.len();

    if cond_idx == n && curr_group > 0 {
        new_group = curr_group;
    }

    if new_group > 0 {
        if dmg_idx == m {
            return 0;
        }

        if new_group == damage_groups[dmg_idx] {
            dmg_idx += 1;
        } else {
            return 0;
        }
    }

    if cond_idx == n {
        if dmg_idx == m {
            return 1;
        } else {
            return 0;
        }
    }

    match conditions[cond_idx] {
        Condition::Operational => {
            count_arrangements_rec(record, cond_idx + 1, dmg_idx, 0, curr_group)
        }
        Condition::Damaged => {
            count_arrangements_rec(record, cond_idx + 1, dmg_idx, curr_group + 1, 0)
        }
        Condition::Unknown => {
            count_arrangements_rec(record, cond_idx + 1, dmg_idx, 0, curr_group)
                + count_arrangements_rec(record, cond_idx + 1, dmg_idx, curr_group + 1, 0)
        }
    }
}

fn count_arrangements(record: &Record) -> i64 {
    count_arrangements_rec(record, 0, 0, 0, 0)
}

fn part_one(records: &Vec<Record>) {
    let sum = records
        .iter()
        .fold(0, |acc, record| acc + count_arrangements(record));
    println!("{}", sum);
}

fn count_arrangements_dp(record: &Record) -> i64 {
    let Record {
        conditions,
        damage_groups,
    } = record;
    let n = conditions.len();
    let k = damage_groups.len();

    // dp[i][j] = arrangements made from i damage groups and j conditions
    let mut dp = vec![vec![0; n + 1]; k + 1];

    // Fill i = 0
    dp[0][0] = 1;
    for j in 1..=n {
        if conditions[j - 1] == Condition::Damaged {
            dp[0][j] = 0;
        } else {
            dp[0][j] = dp[0][j - 1];
        }
    }

    // Fill j = 0 (default by initialization)
    /*for i in 1..=n {
        dp[i][0] = 0;
    }*/

    // At each dp[i][j]:

    // If condition = Operational: dp[i][j - 1] (same damage groups, previous index)
    // If condition = Damaged: dp[i - 1][j - group] if fillable
    // If condition = Unknown: dp[i - 1][j - group] if fillable + dp[i][j - 1]

    for i in 1..=k {
        let idx = i - 1;
        for j in 1..=n {
            let jdx = j - 1;

            if conditions[jdx] == Condition::Operational {
                dp[i][j] = dp[i][j - 1];
                continue;
            }

            let mut damaged_suffix = 0;
            for jdx in (0..=jdx).rev() {
                if conditions[jdx] == Condition::Operational {
                    break;
                }
                damaged_suffix += 1;
            }

            let dmgrp = damage_groups[idx] as usize;
            let can_fill_damage_group = damaged_suffix >= dmgrp
                && (dmgrp > jdx || conditions[jdx - dmgrp] != Condition::Damaged);

            let filled_damage_group = if can_fill_damage_group {
                let j_after_fill = j.saturating_sub(damage_groups[idx] as usize + 1);
                dp[i - 1][j_after_fill]
            } else {
                0
            };

            dp[i][j] = match conditions[jdx] {
                Condition::Operational => unreachable!(),
                Condition::Damaged => filled_damage_group,
                Condition::Unknown => {
                    let not_filled_damage_group = dp[i][j - 1];
                    filled_damage_group + not_filled_damage_group
                }
            };
        }
    }

    /*
    for i in 0..=k {
        for j in 0..=n {
            let record = Record {
                conditions: conditions[0..j].to_owned(),
                damage_groups: damage_groups[0..i].to_owned(),
            };
            let count = count_arrangements(&record);
            if dp[i][j] != count {
                println!("mismatch at {}, {}", i, j);
            }
        }
    }
    */

    dp[k][n]
}

fn part_two(records: &Vec<Record>) {
    let mut sum = 0;
    for record in records.iter() {
        let Record {
            conditions,
            damage_groups,
        } = record;

        let mut conditions_5x = conditions.clone();
        for _ in 0..4 {
            conditions_5x.push(Condition::Unknown);
            conditions_5x.extend_from_slice(&conditions[..]);
        }

        let damage_groups_5x = damage_groups.repeat(5);

        let record_5x = Record {
            conditions: conditions_5x,
            damage_groups: damage_groups_5x,
        };

        let count = count_arrangements_dp(&record_5x);
        sum += count;
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 12)?;

    let mut records: Vec<Record> = Default::default();
    for line in input.lines() {
        let (conditions, damage_groups) = line.split_once(' ').unwrap();
        let conditions = conditions
            .chars()
            .map(|c| match c {
                '.' => Condition::Operational,
                '#' => Condition::Damaged,
                '?' => Condition::Unknown,
                _ => unreachable!(),
            })
            .collect();
        let damage_groups = damage_groups
            .split(',')
            .map(|g| g.parse().unwrap())
            .collect();
        let record = Record {
            conditions,
            damage_groups,
        };

        records.push(record);
    }

    part_one(&records);
    part_two(&records);

    Ok(())
}
