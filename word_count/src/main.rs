use std::collections::HashMap;

fn main() {
    let text = String::from("one two three one");
    let map = word_count(text);

    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    for (key, val) in sorted.iter() {
        println!("{key} => {val}");
    }
}

fn word_count(text: String) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for w in text.split(" ") {
        let count = map.entry(w.to_string()).or_insert(0);
        *count += 1;
    }
    map
}
