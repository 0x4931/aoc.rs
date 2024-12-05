use std::collections::{BTreeMap, HashSet};

struct Card {
    id: i32,
    winning_nums: Vec<i32>,
    nums: Vec<i32>,
}

fn count_matching_nums(card: &Card) -> i32 {
    let mut matching_nums = 0;
    let winning_nums: HashSet<i32> = HashSet::from_iter(card.winning_nums.iter().cloned());

    for num in card.nums.iter() {
        if winning_nums.contains(num) {
            matching_nums += 1;
        }
    }

    matching_nums
}

fn part_one(cards: &Vec<Card>) {
    let mut sum = 0;

    for card in cards.iter() {
        let matching_nums = count_matching_nums(card);
        let points = if matching_nums > 0 {
            1 << (matching_nums - 1)
        } else {
            0
        };

        sum += points;
    }

    println!("{}", sum);
}

fn part_two(cards: &Vec<Card>) {
    let mut sum = 0;

    let mut to_scratch = BTreeMap::new();
    for card_idx in 0..cards.len() {
        to_scratch.insert(card_idx, 1);
    }

    while !to_scratch.is_empty() {
        let (card_idx, card_count) = to_scratch.pop_first().unwrap();
        let card = &cards[card_idx];

        sum += card_count;

        let matching_nums = count_matching_nums(card) as usize;
        for next_card_idx in card_idx + 1..=card_idx + matching_nums {
            *to_scratch.entry(next_card_idx).or_insert(0) += card_count;
        }
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 4)?;

    let mut cards = Vec::new();
    for line in input.lines() {
        let (card_info, card_data) = line.split_once(": ").unwrap();
        let mut card_info = card_info.split_whitespace().skip(1);
        let card_id = card_info.next().unwrap();
        let (card_wins, card_nums) = card_data.split_once(" | ").unwrap();

        let id = card_id.parse().unwrap();
        let mut winning_nums = Vec::new();
        let mut nums = Vec::new();

        for winning_num in card_wins.split_whitespace() {
            winning_nums.push(winning_num.parse().unwrap());
        }

        for num in card_nums.split_whitespace() {
            nums.push(num.parse().unwrap());
        }

        cards.push(Card {
            id,
            winning_nums,
            nums,
        });
    }

    part_one(&cards);
    part_two(&cards);

    Ok(())
}
