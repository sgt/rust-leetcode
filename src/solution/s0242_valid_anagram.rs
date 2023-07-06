use std::{collections::HashMap};

pub struct Solution;

impl Solution {
    /// 242. Valid Anagram
    pub fn is_anagram(s: String, t: String) -> bool {
        Self::count_chars(&s) == Self::count_chars(&t)
    }

    fn count_chars(s: &str) -> HashMap<char, i32> {
        let mut m = HashMap::new();
        for c in s.chars() {
            m.entry(c).and_modify(|i| *i += 1).or_insert(1);
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(true, "anagram", "nagaram")]
    #[test_case(false, "rat", "car")]
    fn test(result: bool, s: &str, t: &str) {
        assert_eq!(result, Solution::is_anagram(s.into(), t.into()));
    }
}
