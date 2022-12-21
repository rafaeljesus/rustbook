// Time: O(nm) | Space: O(1)
pub fn string_match(text: &str, pattern: &str) -> bool {
    for i in 0..text.len() - pattern.len() + 1 {
        let mut j = 0;
        while j < pattern.len() && (&pattern[..j] == "*" || &text[..i + j] != &pattern[..j]) {
            j += 1;
        }
        if j == pattern.len() {
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
        assert_eq!(string_match("abada", "ab*"), true);
        assert_eq!(string_match("abada", "ad*"), true);
        assert_eq!(string_match("abada", "adaa*"), false);
        assert_eq!(string_match("abada", "*ada"), true);
    }
}
