const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "35");
    print_answer("one", &one(INPUT), "23028");
    // print_answer("two (example)", &two(EXAMPLE), "30");
    // print_answer("two", &two(INPUT), "9236992");
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

// fn two(input: &str) -> String {
//     String::new()
// }

fn parse(input: &str) -> World {
    let seeds = input.lines().next().expect("expect at least one line");
    let categories: Vec<Vec<&str>> = dbg!(input
        .split("\n\n")
        .skip(1)
        .map(|l| l.lines().collect())
        .collect());

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

    dbg!(World { seeds, categories })
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

#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
    size: u64,
}
