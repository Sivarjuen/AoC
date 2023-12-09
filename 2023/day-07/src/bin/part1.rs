use std::{collections::HashMap, cmp::Ordering};
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<u64>,
    bid: u64
}

impl Hand {
    fn new(input: &str) -> Hand {
        let line = input.split_whitespace().collect::<Vec<&str>>();
        let bid = line[1].parse::<u64>().unwrap();
        let cards: Vec<u64> = line[0].chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _default => panic!("Invalid card value")
            }
        }).collect();
        let mut type_map: HashMap<u64, u64> = HashMap::new();
        for card in cards.iter() {
            *type_map.entry(*card).or_insert(0) += 1;
        }
        let mut hand_type = HandType::HighCard;

        let type_counts: Vec<u64> = type_map.into_values().collect();
        if type_counts.contains(&5) {
            hand_type = HandType::FiveKind;
        } else if type_counts.contains(&4) {
            hand_type = HandType::FourKind;
        } else if type_counts.contains(&3) && type_counts.contains(&2) {
            hand_type = HandType::FullHouse;
        } else if type_counts.contains(&3) {
            hand_type = HandType::ThreeKind;
        } else if type_counts.iter().combinations(2).any(|v| *v[0] == 2 && *v[1] == 2) {
            hand_type = HandType::TwoPair;
        } else if type_counts.contains(&2) {
            hand_type = HandType::OnePair;
        }
        Hand { hand_type, cards, bid }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

fn part1(input: &str) -> u64 {
    let lines = input.lines();
    let mut hands: Vec<Hand> = lines.into_iter().map(Hand::new).collect();

    hands.sort();
    for hand in hands.iter() {
        println!("{:?}", hand);
    }
    hands.iter().enumerate().map(|(i, h)| {
        (i as u64+1) * h.bid
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part1("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(result, 6440);
    }
}
