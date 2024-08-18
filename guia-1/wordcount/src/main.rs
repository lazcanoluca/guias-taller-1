use std::{
    cmp::Reverse,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("./words.txt").expect("Failed to open file");
    let reader = BufReader::new(input);

    let mut words = HashMap::new();

    for line in reader.lines() {
        for word in line.expect("Failed to read line").split_whitespace() {
            let mut word = word.to_owned();

            word.retain(|c| c.is_alphabetic());

            words
                .entry(word.to_lowercase())
                .and_modify(|w| *w += 1)
                .or_insert(1);
        }
    }

    let mut a: Vec<_> = words.into_iter().collect();
    a.sort_by_key(|(_, count)| Reverse(*count));

    for (word, count) in a {
        println!("{} -> {}", word, count);
    }
}
