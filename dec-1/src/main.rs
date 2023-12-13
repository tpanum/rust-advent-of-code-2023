use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

fn sum_with_only_real_digits(reader: std::io::BufReader<std::fs::File>) -> i32 {
    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let re = Regex::new(r"\d").unwrap();

        let mut first_value = "";
        let mut last_value = "";

        re.captures_iter(&line).for_each(|cap| {
            let v = cap.get(0).unwrap().as_str();
            if first_value == "" {
                first_value = v;
            }

            last_value = v;
        });

        let line_val = (first_value.to_owned() + last_value)
            .parse::<i32>()
            .unwrap();

        total += line_val;
    }

    total
}

fn sum_with_both_digit_types(reader: std::io::BufReader<std::fs::File>) -> i32 {
    let mut total = 0;
    let alphas_to_num = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let values_to_lookup = alphas_to_num
        .keys()
        .chain(alphas_to_num.values())
        .map(|s| s.to_owned())
        .collect::<Vec<&str>>();

    let re = Regex::new(&values_to_lookup.join("|")).unwrap();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut first_value = "";
        let mut last_value = "";

        for i in line.char_indices() {
            match re.find_at(&line, i.0) {
                Some(m) => {
                    let v = m.as_str();
                    if first_value == "" {
                        first_value = v;
                    }

                    last_value = v;
                }
                None => {}
            }
        }

        first_value = match alphas_to_num.get(first_value) {
            Some(new_v) => new_v,
            None => first_value,
        };

        last_value = match alphas_to_num.get(last_value) {
            Some(new_v) => new_v,
            None => last_value,
        };

        let line_val = (first_value.to_owned() + last_value)
            .parse::<i32>()
            .unwrap();

        total += line_val;
    }

    total
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let reader = std::io::BufReader::new(f);

    let total = sum_with_only_real_digits(reader);
    println!("Total (only digits): {}", total);

    let f = File::open("input.txt").expect("Unable to open file");
    let reader = std::io::BufReader::new(f);

    let total = sum_with_both_digit_types(reader);
    println!("Total (both digits): {}", total);
}
