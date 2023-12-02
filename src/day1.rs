static NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_last_number(t: &str) -> usize {
    let mut value = 0;
    let mut last_index: usize = 0;
    for (i, el) in NUMBERS.iter().enumerate() {
        let found = t.rfind(el);
        if found.is_some() {
            if found.unwrap() > last_index {
                last_index = found.unwrap();
                value = i;
            }
        }
    }

    for i in last_index..t.len() {
        if t.chars().nth(i).unwrap().is_numeric() {
            value = t.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
        }
    }

    return value
}

fn find_first_number(t: &str) -> usize {
    let mut value = 0;
    let mut last_index: usize = 10000000;
    for (i, el) in NUMBERS.iter().enumerate() {
        let found = t.find(el);
        if found.is_some() {
            if found.unwrap() < last_index {
                last_index = found.unwrap();
                value = i;
            }
        }
    }

    for (i, char) in t.chars().enumerate() {
        if i >= last_index {
            break;
        }
        if char.is_numeric() {
            value = char.to_digit(10).unwrap() as usize;
            break;
        }
    }

    return value
}

pub(crate) fn run() {
    let input = include_str!("../inputs/day1.txt");

    let lines = input.split("\n");

    let mut sum = 0;

    for line in lines {
        let first = find_first_number(line);
        let last = find_last_number(line);

        sum += first * 10 + last;
    }

    println!("{sum}");
}