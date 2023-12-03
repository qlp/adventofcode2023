use std::collections::HashMap;
use crate::Item::{NUMBER, SYMBOL};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "4361");
    print_answer("one", &one(INPUT), "2061");
    // print_answer("two (example)", &two(EXAMPLE), "2286");
    // print_answer("two", &two(INPUT), "72596");
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

            dbg!(coordinate);
            dbg!(item);
            dbg!(result);

            result
        })
        .map(|(_, item)| match item {
            NUMBER(number) => number.clone(),
            _ => 0
        })
        .sum::<u32>()
        .to_string()
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
