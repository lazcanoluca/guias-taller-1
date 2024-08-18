use std::collections::HashMap;

fn main() {
    let lines = vec!["La casa tiene una ventana.", "La ventana fue defenestrada."];

    let mut words = HashMap::new();

    for line in lines {
        for word in line.split_whitespace() {
            let mut word = word.to_owned();

            word.retain(|c| c.is_alphabetic());

            words
                .entry(word.to_lowercase())
                .and_modify(|w| *w += 1)
                .or_insert(1);
        }
    }

    let mut a: Vec<_> = words.into_iter().collect();
    a.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (word, count) in a {
        println!("{} -> {}", word, count);
    }
}
