// Time: O(nm) | Space: O(1)
pub fn string_match(text: &str, pattern: &str) -> bool {
    let n = text.len();
    let m = text.len();
    for i in 0..(n - m) + 1 {
        let mut j = 0;
        // rust indexing in string sucks :/
        while j < m && &text[..i + j] == &pattern[..j] {
            j += 1;
        }
        if j == m {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_match_test() {
        assert_eq!(string_match("abaababbaba", "abba"), true);
        assert_eq!(string_match("abada", "abade"), false);
        /*assert_eq!(string_match("abada", "ab*"), true);
        assert_eq!(string_match("abada", "ad*"), true);
        assert_eq!(string_match("abada", "adaa*"), false);
        assert_eq!(string_match("abada", "*ada"), true);*/
    }
}
