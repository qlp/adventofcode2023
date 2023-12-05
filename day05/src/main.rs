use rayon::prelude::*;
use std::collections::HashSet;

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

    let significant_numbers: HashSet<u64> = world
        .categories
        .iter()
        .flat_map(|c| c.ranges.clone())
        .flat_map(|r| vec![r.from, r.from + r.size, r.to, r.to + r.size])
        .collect();

    // dbg!(&significant_numbers);

    let mut points: Vec<u64> = Vec::from_iter(significant_numbers.iter().copied());
    points.sort();

    let ranges: Vec<Range> = (1..points.len())
        .map(|i| (points[i - 1], points[i]))
        .map(|(from, to)| Range {
            from: from,
            to: world.location(from),
            size: to - from,
        })
        .collect();

    let seeds: Vec<u64> = (1..world.seeds.len())
        .step_by(2)
        .map(|i| (world.seeds[i - 1], world.seeds[i]))
        .map(|(start, length)| start..(start + length))
        .flat_map(|range| range.into_iter().collect::<Vec<u64>>())
        .collect();

    // let world = World {
    //     seeds: seeds.clone(),
    //     categories: vec![Category {
    //         from: String::from("seed"),
    //         to: String::from("location"),
    //         ranges,
    //     }],
    // };
    //
    // dbg!(&world);

    // seeds
    //     .iter()
    //     .map(|seed| {
    //         dbg!(&seed);
    //         dbg!(world.location(seed.clone()))
    //     })
    //     .min()
    //     .expect("at least one")
    //     .to_string()

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

            let ranges: Vec<Range> = c
                .iter()
                .skip(1)
                .map(|r| {
                    let numbers: Vec<&str> = r.split(' ').collect();

                    Range {
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
                ranges,
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
    ranges: Vec<Range>,
}

impl Category {
    fn next(&self, number: u64) -> u64 {
        match self
            .ranges
            .iter()
            .find(|r| number >= r.from && number < (r.from + r.size))
        {
            None => number,
            Some(range) => number + range.to - range.from,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    from: u64,
    to: u64,
    size: u64,
}
