use std::ops::Range;

use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "35");
    print_answer("one", &one(INPUT), "579439039");
    print_answer("two (example)", &two(EXAMPLE), "46");
    print_answer("two", &two(INPUT), "7873084");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let world = parse(input);

    parse(input)
        .seeds
        .iter()
        .map(|seed| world.location(seed.clone()))
        .min()
        .expect("at least one")
        .to_string()
}

fn two(input: &str) -> String {
    let world = parse(input);

    (1..world.seeds.len())
        .step_by(2)
        .map(|i| (world.seeds[i - 1], world.seeds[i]))
        .map(|(start, length)| start..(start + length))
        .map(|range| {
            println!("range: {}-{}", range.start, range.end);
            range
                .into_par_iter()
                .map(|seed| world.location(seed))
                .min()
                .expect("at least one for {range}")
        })
        .min()
        .expect("at least one")
        .to_string()
}

fn parse(input: &str) -> World {
    let seeds = input.lines().next().expect("expect at least one line");
    let categories: Vec<Vec<&str>> = input
        .split("\n\n")
        .skip(1)
        .map(|l| l.lines().collect())
        .collect();

    let seeds = seeds
        .split_once(": ")
        .expect("expect a colon")
        .1
        .split(' ')
        .map(|number| number.parse().expect("should be a number"))
        .collect();

    let categories: Vec<Category> = categories
        .iter()
        .map(|c| {
            let (from, to) = c
                .first()
                .expect("at least one row")
                .split_once(' ')
                .expect("a space")
                .0
                .split_once("-to-")
                .expect("to separator");

            let ranges: Vec<MyRange> = c
                .iter()
                .skip(1)
                .map(|r| {
                    let numbers: Vec<&str> = r.split(' ').collect();

                    MyRange {
                        to: numbers
                            .first()
                            .expect("expect 1/3 number")
                            .parse()
                            .expect("number 1/3"),
                        from: numbers
                            .get(1)
                            .expect("expect 2/3 number")
                            .parse()
                            .expect("number 2/3"),
                        size: numbers
                            .get(2)
                            .expect("expect 3/3 number")
                            .parse()
                            .expect("number 3/3"),
                    }
                })
                .collect();

            Category {
                from: from.to_string(),
                to: to.to_string(),
                ranges: Ranges { ranges },
            }
        })
        .collect();

    World { seeds, categories }
}

#[derive(Debug)]
struct World {
    seeds: Vec<u64>,
    categories: Vec<Category>,
}

impl World {
    fn location(&self, seed: u64) -> u64 {
        let mut number = seed;
        let mut category = self
            .categories
            .iter()
            .find(|c| c.from.eq("seed"))
            .expect("expect a seed category");

        while category.to.ne("location") {
            number = category.next(number);
            category = self
                .categories
                .iter()
                .find(|c| c.from == category.to)
                .expect("category not found");
        }

        category.next(number)
    }
}

#[derive(Debug)]
struct Category {
    from: String,
    to: String,
    ranges: Ranges,
}

impl Category {
    fn next(&self, number: u64) -> u64 {
        self.ranges.next(number)
    }
}

#[derive(Debug, Clone)]
struct Ranges {
    ranges: Vec<MyRange>,
}

impl Ranges {
    fn next(&self, number: u64) -> u64 {
        match self.ranges.iter().find(|r| r.range().contains(&number)) {
            None => number,
            Some(range) => range.add(number),
        }
    }
    //
    // fn combine(&self, other: Self) -> Self {
    //     let mut result = self.ranges.clone();
    //
    //     other.ranges.iter().for_each(|o| {
    //         if !result.iter().any(|r| r.overlaps(o)) {
    //             result.push(*o);
    //         }
    //     });
    //
    //     Ranges { ranges: result }
    // }
}

#[derive(Debug, Copy, Clone)]
struct MyRange {
    from: u64,
    to: u64,
    size: u64,
}

impl MyRange {
    fn add(&self, number: u64) -> u64 {
        self.to + number - self.from
    }

    fn range(&self) -> Range<u64> {
        self.from..(self.from + self.size)
    }
}
