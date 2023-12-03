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
            let result = match item {
                NUMBER(number) => {
                    let coordinates: Vec<Coordinate> = candidate_coordinates(coordinate, number.clone());

                    coordinates.iter().any(|coordinate| {
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
            };

            result
        })
        .map(|(_, item)| match item {
            NUMBER(number) => number.clone(),
            _ => 0
        })
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    let world = parse(input);

    let gear_to_number: Vec<GearToNumber> = world
        .items
        .iter()
        .flat_map(|(coordinate, item)| {
            match item {
                NUMBER(number) => {
                    let coordinates: Vec<Coordinate> = candidate_coordinates(coordinate, number.clone());

                    coordinates.iter().filter(|coordinate| {
                        match world.items.get(&coordinate) {
                            None => false,
                            Some(item) =>
                                match item {
                                    SYMBOL('*') => true,
                                    _ => false,
                                }
                        }
                    })
                        .map(|&gear| GearToNumber { gear: gear, number: number.clone()})
                        .collect()
                }
                _ => Vec::new()
            }
        })
        .collect();

    let mut gear_to_numbers: HashMap<Coordinate, Vec<u32>> = HashMap::new();

    for gtn in gear_to_number {
        let numbers = gear_to_numbers.entry(gtn.gear).or_insert_with(|| Vec::new());
        numbers.push(gtn.number)
    }

    gear_to_numbers
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers.iter().fold(0, |result, &number| if result == 0 { number } else { result * number}))
        .sum::<u32>()
        .to_string()

    // String::new()
}

fn candidate_coordinates(coordinate: &Coordinate, number: u32) -> Vec<Coordinate> {
    (if coordinate.x == 0 { 0 } else { coordinate.x - 1 }..=(coordinate.x + number.to_string().len() as u32))
        .map(|x|candidate_y_coordinates(&coordinate, x))
        .flatten()
        .collect()
}

fn candidate_y_coordinates(coordinate: &Coordinate, x: u32) -> Vec<Coordinate> {
    (if coordinate.y == 0 { 0 } else { coordinate.y - 1 }..=(coordinate.y + 1)).map(|y|
        Coordinate { x: x.clone(), y: y.clone() }
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
                        '.' => if !buffer.is_empty() {
                            items.push((Coordinate { x: (column_index - buffer.len()) as u32, y: row_index as u32 }, NUMBER(buffer.parse().expect("expect a number"))));
                            buffer.clear()
                        },
                        '0'..='9' => buffer.push(char),
                        _ => {
                            if !buffer.is_empty() {
                                items.push((Coordinate { x: (column_index - buffer.len()) as u32, y: row_index as u32 }, NUMBER(buffer.parse().expect("expect a number"))));
                                buffer.clear()
                            }
                            items.push((Coordinate { x: column_index as u32, y: row_index as u32 }, SYMBOL(char)))
                        }
                    }
                }

                if !buffer.is_empty() {
                    items.push((Coordinate { x: (line.len() - buffer.len()) as u32, y: row_index as u32 }, NUMBER(buffer.parse().expect("expect a number"))));
                    buffer.clear()
                }

                items
            })
            .flatten()
            .collect()
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
struct GearToNumber {
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

