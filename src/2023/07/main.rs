use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<i32>,
    kind: HandKind,
    bid: i32,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Error)]
#[error("failed to parse hand")]
struct ParseHandError;

impl std::str::FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.split_whitespace();
        let cards = data.next().ok_or_else(|| ParseHandError)?;
        let bid = data.next().ok_or_else(|| ParseHandError)?;

        let cards: Result<Vec<_>, _> = cards
            .bytes()
            .map(|b| match b {
                b'2' => Ok(2),
                b'3' => Ok(3),
                b'4' => Ok(4),
                b'5' => Ok(5),
                b'6' => Ok(6),
                b'7' => Ok(7),
                b'8' => Ok(8),
                b'9' => Ok(9),
                b'T' => Ok(10),
                b'J' => Ok(11),
                b'Q' => Ok(12),
                b'K' => Ok(13),
                b'A' => Ok(14),
                _ => Err(ParseHandError),
            })
            .collect();
        let cards = cards?;
        let bid = bid.parse()?;

        let mut freqs = HashMap::new();
        for card in cards.iter() {
            *freqs.entry(*card).or_insert(0) += 1;
        }
        let mut freqs: Vec<_> = freqs.values().cloned().collect();
        freqs.sort();

        let mut hand = Self {
            cards,
            kind: HandKind::HighCard,
            bid,
        };
        hand.recompute_kind(freqs);
        Ok(hand)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind != other.kind {
            Some(self.kind.cmp(&other.kind))
        } else {
            Some(self.cards.cmp(&other.cards))
        }
    }
}

impl Eq for Hand {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    // Thanks part two for making me create a whole new function!
    fn recompute_kind(&mut self, freqs: Vec<i32>) {
        self.kind = match &freqs[..] {
            [1, 1, 1, 1, 1] => HandKind::HighCard,
            [1, 1, 1, 2] => HandKind::OnePair,
            [1, 2, 2] => HandKind::TwoPair,
            [1, 1, 3] => HandKind::ThreeOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 4] => HandKind::FourOfAKind,
            [5] => HandKind::FiveOfAKind,
            _ => unreachable!(),
        };
    }

    fn apply_joker_rule(&mut self) {
        // Change jacks into jokers
        for card in self.cards.iter_mut() {
            if *card == 11 {
                *card = 1;
            }
        }

        // Recompute the hand kind
        let mut freqs = HashMap::new();
        let mut jokers = 0;
        for card in self.cards.iter() {
            match *card {
                1 => jokers += 1,
                c => *freqs.entry(c).or_insert(0) += 1,
            }
        }
        let mut freqs: Vec<_> = freqs.values().cloned().collect();
        freqs.sort();
        if freqs.is_empty() {
            freqs.push(0);
        }
        *freqs.last_mut().unwrap() += jokers;

        self.recompute_kind(freqs);
    }
}

fn part_one(hands: &Vec<Hand>) {
    let mut hands = hands.clone();
    hands.sort();

    let mut sum = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        let winnings = hand.bid * (idx + 1) as i32;
        sum += winnings;
    }

    println!("{}", sum);
}

fn part_two(hands: &Vec<Hand>) {
    let mut hands = hands.clone();
    for hand in hands.iter_mut() {
        hand.apply_joker_rule();
    }

    hands.sort();

    let mut sum = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        let winnings = hand.bid * (idx + 1) as i32;
        sum += winnings;
    }

    println!("{}", sum);
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2023, 7)?;

    let mut hands = Vec::new();
    for line in input.lines() {
        hands.push(line.parse().unwrap());
    }

    part_one(&hands);
    part_two(&hands);

    Ok(())
}
