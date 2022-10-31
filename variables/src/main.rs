fn main() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("{word}");

    let mut s1 = String::from("hello");
    let len = calc_lenght(&mut s1);
    println!("{s1} - {len}");
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn calc_lenght(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}
