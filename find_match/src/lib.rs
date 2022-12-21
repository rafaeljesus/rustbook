// Time: O(nm) | Space: O(1)
pub fn string_match(text: &str, pattern: &str) -> bool {
    for i in 0..text.len() - pattern.len() + 1 {
        let mut j = 0;
        while j < pattern.len() && &text[..i + j] != &pattern[..j] {
            j += 1;
        }
        if j == pattern.len() {
            return true;
        }
    }
    false
}

pub fn string_match2(text: &str, pattern: &str) -> bool {
    for i in 0..text.len() - pattern.len() + 1 {
        let mut match_found = true;
        for j in 0..pattern.len() {
            if &pattern[..j] == "*" {
                continue;
            }
            if &text[..i + j] != &pattern[..j] {
                match_found = false;
                break;
            }
        }
        if match_found {
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

        assert_eq!(string_match2("abaababbaba", "abba"), true);
        assert_eq!(string_match2("abaababbaba", "abb*"), true);
        assert_eq!(string_match2("abaababbaba", "abaaa*"), false);
        assert_eq!(string_match2("abaababbaba", "aba*"), true);
        assert_eq!(string_match2("abada", "abade"), false);
    }
}
