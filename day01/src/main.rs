const INPUT: &str = include_str!("input.txt");

const NUMBERS: [&str; 20] = [
    "this_will_never_match",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
];

fn main() {
    print_answer("one", &one(INPUT), "54450");
    print_answer("two", &two(INPUT), "54265");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    input
        .lines()
        .map(|line| -> u32 {
            let first = number(line);
            let second = number(&line.chars().rev().collect::<String>());

            first * 10 + second
        })
        .sum::<u32>()
        .to_string()
}

fn number(string: &str) -> u32 {
    string
        .chars()
        .find(|&c| c.is_ascii_digit())
        .expect("no digits in string")
        .to_digit(10)
        .expect("not an ascii digit")
}

fn two(input: &str) -> String {
    input
        .lines()
        .map(|line| -> u32 {
            let first = number_from_text(line, true);
            let second = number_from_text(line, false);

            first * 10 + second
        })
        .sum::<u32>()
        .to_string()
}

struct Answer {
    answer: u32,
    answer_index: usize,
}

fn number_from_text(string: &str, left_to_right: bool) -> u32 {
    let string = reverse_if_required(string, left_to_right);

    NUMBERS
        .iter()
        .enumerate()
        .fold(
            Answer {
                answer: 0,
                answer_index: string.len(),
            },
            |answer, (answer_index, &candidate)| {
                let candidate = reverse_if_required(candidate, left_to_right);

                let index_of_candidate = string.find(&candidate).unwrap_or(string.len());

                match index_of_candidate < answer.answer_index {
                    true => Answer {
                        answer: answer_index as u32 % 10,
                        answer_index: index_of_candidate,
                    },
                    false => answer,
                }
            },
        )
        .answer
}

fn reverse_if_required(string: &str, left_to_right: bool) -> String {
    match left_to_right {
        true => String::from(string),
        false => string.chars().rev().collect::<String>(),
    }
}
