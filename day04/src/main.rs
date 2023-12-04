const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "13");
    print_answer("one", &one(INPUT), "537832");
    print_answer("two (example)", &two(EXAMPLE), "30");
    print_answer("two", &two(INPUT), "9236992");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    parse(input)
        .iter()
        .map(|card| card.points())
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    let mut cards = parse(input);

    for index in 0..cards.len() {
        let card = &cards[index];

        let winning = card.winning() as usize;
        let start_index = index + 1;
        let copies = card.copies;

        if winning > 0 {
            for copy_index in start_index..(start_index + winning) {
                if let Some(card_to_copy) = cards.get_mut(copy_index) {
                    card_to_copy.add_copy(copies)
                }
            }
        }
    }

    cards
        .iter()
        .map(|card| card.copies)
        .sum::<u32>()
        .to_string()
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (card, contents) = line.split_once(':').expect("expect a ':");
            let (_, id) = card.split_once(' ').expect("expect a ' ");
            let id: u32 = id.trim().parse().expect("expect a number");
            let (numbers, winning) = contents.split_once(" | ").expect("expect a separator");
            let numbers: Vec<u32> = numbers
                .trim()
                .split(' ')
                .filter(|number| !number.is_empty())
                .map(|number| number.parse().expect("expect a number"))
                .collect();
            let winning: Vec<u32> = winning
                .trim()
                .split(' ')
                .filter(|winning| !winning.is_empty())
                .map(|winning| winning.parse().expect("expect a number"))
                .collect();

            Card {
                id,
                numbers,
                winning,
                copies: 1,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning: Vec<u32>,
    copies: u32,
}

impl Card {
    fn winning(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&number| self.winning.contains(number))
            .count() as u32
    }

    fn points(&self) -> u32 {
        let count = self
            .numbers
            .iter()
            .filter(|&number| self.winning.contains(number))
            .count() as u32;

        match count {
            0 => 0,
            n => 2u32.pow(n - 1),
        }
    }

    fn add_copy(&mut self, count: u32) {
        self.copies += count;
    }
}
