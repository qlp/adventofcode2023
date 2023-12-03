use std::collections::HashMap;
use crate::Item::{NUMBER, SYMBOL};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "4361");
    print_answer("one", &one(INPUT), "537832");
    print_answer("two (example)", &two(EXAMPLE), "467835");
    print_answer("two", &two(INPUT), "81939900");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let world = parse(input);

    world
        .items
        .iter()
        .filter(|(coordinate, item)| {
            match item {
                NUMBER(number) => {
                    candidate_coordinates(coordinate, number)
                        .iter()
                        .any(|coordinate| {
                            match world.items.get(&coordinate) {
                                None => false,
                                Some(item) =>
                                    match item {
                                        SYMBOL(_) => true,
                                        _ => false,
                                    }
                            }
                        })
                }
                _ => false
            }
        })
        .map(|(_, item)| match item {
            NUMBER(number) => number.clone(),
            _ => 0
        })
        .sum::<u32>()
        .to_string()
}

const GEAR: char = '*';

fn two(input: &str) -> String {
    let world = parse(input);

    let number_to_gear: Vec<NumberToGear> = world
        .items
        .iter()
        .flat_map(|(coordinate, item)| {
            match item {
                NUMBER(number) => {
                    candidate_coordinates(coordinate, number).iter()
                        .filter(|coordinate| {
                            match world.items.get(&coordinate) {
                                None => false,
                                Some(item) =>
                                    match item {
                                        SYMBOL(GEAR) => true,
                                        _ => false,
                                    }
                            }
                        })
                        .map(|gear| NumberToGear { number: number.clone(), gear: gear.clone() })
                        .collect()
                }
                _ => Vec::new()
            }
        })
        .collect();

    let mut gear_to_numbers: HashMap<Coordinate, Vec<u32>> = HashMap::new();

    for gtn in number_to_gear {
        let numbers = gear_to_numbers.entry(gtn.gear).or_insert_with(|| Vec::new());
        numbers.push(gtn.number)
    }

    gear_to_numbers
        .iter()
        .map(|(_, numbers)| if numbers.len() == 2 { numbers[0] * numbers[1] } else { 0 })
        .sum::<u32>()
        .to_string()
}

fn candidate_coordinates(coordinate: &Coordinate, number: &u32) -> Vec<Coordinate> {
    (coordinate.x.saturating_sub(1)..=(coordinate.x + number.ilog10() + 1))
        .map(|x| candidate_y_coordinates(&coordinate, x))
        .flatten()
        .collect()
}

fn candidate_y_coordinates(coordinate: &Coordinate, x: u32) -> Vec<Coordinate> {
    (coordinate.y.saturating_sub(1)..=(coordinate.y + 1)).map(|y|
        Coordinate { x, y }
    ).collect()
}

fn parse(input: &str) -> World {
    World {
        items: input
            .lines()
            .enumerate()
            .map(|(row_index, line)| {
                let mut items: Vec<(Coordinate, Item)> = Vec::new();
                let mut buffer = String::new();

                for (column_index, char) in line.chars().enumerate() {
                    match char {
                        '0'..='9' => buffer.push(char),
                        _ => {
                            if char != '.' {
                                items.push((Coordinate { x: column_index as u32, y: row_index as u32 }, SYMBOL(char)))
                            }

                            if !buffer.is_empty() {
                                items.push((Coordinate { x: (column_index - buffer.len()) as u32, y: row_index as u32 }, NUMBER(buffer.parse().expect("expect a number"))));
                                buffer.clear()
                            }
                        }
                    }
                }

                if !buffer.is_empty() {
                    items.push((Coordinate { x: (line.len() - buffer.len()) as u32, y: row_index as u32 }, NUMBER(buffer.parse().expect("expect a number"))));
                }

                items
            })
            .flatten()
            .collect()
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
struct NumberToGear {
    gear: Coordinate,
    number: u32,
}

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Item {
    NUMBER(u32),
    SYMBOL(char),
}


#[derive(Debug)]
struct World {
    items: HashMap<Coordinate, Item>,
}

