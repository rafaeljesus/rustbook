use std::collections::HashMap;

// problem description https://exercism.org/tracks/rust/exercises/word-count
// check this approach https://exercism.org/tracks/rust/exercises/word-count/solutions/jnaomi
fn main() {
    let text = "one two three one";
    let map = word_count_basic(text);

    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    for (key, val) in sorted.iter() {
        println!("{key}: {val}");
    }

    println!("\n");

    let text = "That's the password: 'PASSWORD 123'!\", cried the Special Agent.\nSo I fled.";
    let map = world_count(text);

    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    for (key, val) in sorted.iter() {
        println!("{key}: {val}");
    }
}

fn world_count(text: &str) -> HashMap<String, i32> {
    text.split(|c: char| !c.is_alphanumeric() && c != '\'')
        .filter(|w| !w.is_empty())
        .map(|w| w.trim_matches('\'').to_lowercase())
        .fold(HashMap::new(), |mut map, w| {
            let count = map.entry(w.to_string()).or_insert(0);
            *count += 1;
            map
        })
}

fn word_count_basic(text: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for w in text.split(" ") {
        let count = map.entry(w.to_string()).or_insert(0);
        *count += 1;
    }
    map
}
