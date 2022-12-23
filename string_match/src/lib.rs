// Time: O(nmc) | Space: O(1)
pub fn string_match(text: &str, pattern: &str) -> bool {
    let n = text.len();
    let m = pattern.len();
    for i in 0..=n - m {
        let mut j = 0;
        // chars().nth() is O(n), strings in rust aren't indexed
        while j < m
            && (text.chars().nth(i + j).unwrap() == pattern.chars().nth(j).unwrap()
                || pattern.chars().nth(j).unwrap() == '*')
        {
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
        assert_eq!(string_match("abada", "ab*"), true);
        assert_eq!(string_match("abada", "ad*"), true);
        assert_eq!(string_match("abada", "adaa*"), false);
        assert_eq!(string_match("abada", "*ada"), true);
    }
}
