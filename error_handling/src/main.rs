fn main() {
    let value = last_char_of_first_line("abc\n");
    println!("{:?}", value);
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    let line = match text.lines().next() {
        Some(line) => line,
        None => panic!(),
    };
    line.chars().last()
}
