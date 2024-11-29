use std::fs;
use std::str::Chars;

fn main() {
    let input: String = read_file();

    let answer_one = answer_one(input);

    println!("{}", answer_one);
}

fn answer_one(input: String) -> i64 {
    let mut output = 0;
    let input_iter = input.lines();

    for line in input_iter {
        let characters = line.chars();

        output += check_digits(characters);
    }

    output
}

fn read_file() -> String {
    fs::read_to_string("resources/input.txt").expect("This should be the input file")
}

// Used for Question 1
fn check_digits(input: Chars<'_>) -> i64 {
    let mut first = -1;
    let mut last = -1;

    for c in input {
        if c.is_ascii_digit() {
            let digit = i64::from(c.to_digit(10).unwrap());
            if first == -1 {
                first = digit;
            }
            last = digit;
        }
    }

    first * 10 + last
}
