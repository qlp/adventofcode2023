use std::collections::HashMap;
use std::ops::Shl;
use std::panic::panic_any;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    // print_answer("one (example)", &one(EXAMPLE), "6440");
    print_answer("one", &one(INPUT), "1195150");
    // print_answer("two (example)", &two(EXAMPLE), "71503");
    // print_answer("two", &two(INPUT), "42550411");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let mut hands = parse(input).hands;
    hands.sort_by_key(|h| h.cards);

    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u64 + 1))
        .sum::<u64>()
        .to_string()
}

fn two(input: &str) -> String {
    String::new()
}

const NUMBER_OF_CARD_IN_HAND: usize = 5;

fn parse(input: &str) -> World {
    World {
        hands: input
            .lines()
            .map(|l| l.split_once(' ').expect("space"))
            .map(|(cards, bid)| Hand {
                cards: cards
                    .chars()
                    .enumerate()
                    .map(|(index, char)| {
                        let value = match char {
                            'A' => 14u32,
                            'K' => 13u32,
                            'Q' => 12u32,
                            'J' => 11u32,
                            'T' => 10u32,
                            _ => char.to_digit(10).expect("digit"),
                        };

                        value.shl((4 - index) * NUMBER_OF_CARD_IN_HAND)
                    })
                    .reduce(|acc, n| acc | n)
                    .expect("at least one")
                    | type_value(cards).shl(5 * NUMBER_OF_CARD_IN_HAND),
                bid: bid.parse().expect("number"),
            })
            .collect(),
    }
}

fn type_value(cards: &str) -> u32 {
    let map = cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let max = map
        .iter()
        .max_by_key(|(_, v)| v.clone())
        .expect("at least one")
        .1;

    match map.len() {
        1 => 7,
        2 => match max {
            4 => 6,
            3 => 5,
            _ => panic_any("expected 4 or 3"),
        },
        3 => match max {
            3 => 4,
            2 => 3,
            _ => panic_any("expected 3 or 2"),
        },
        4 => 2,
        5 => 1,
        _ => panic_any("expect max 5"),
    }
}

#[derive(Debug, Clone)]
struct World {
    hands: Vec<Hand>,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: u32,
    bid: u64,
}
