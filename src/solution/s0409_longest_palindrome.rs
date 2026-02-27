use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// 409. Longest Palindrome
    // TODO simplify
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freq_map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            freq_map.insert(c, freq_map.get(&c).unwrap_or(&0) + 1);
        }
        let freqs: Vec<i32> = freq_map.into_values().collect();
        let odd_exists: bool = freqs.iter().any(|x| x % 2 == 1);
        let even_entries: i32 = freqs
            .iter()
            .filter_map(|x| {
                if *x < 2 {
                    None
                } else if x % 2 == 1 {
                    Some(x - 1)
                } else {
                    Some(*x)
                }
            })
            .sum();
        if odd_exists {
            even_entries + 1
        } else {
            even_entries
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(7, Solution::longest_palindrome("abccccdd".into()));
        assert_eq!(1, Solution::longest_palindrome("a".into()));
    }
}
