use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m = HashMap::new();
        for s in strs {
            m.entry(Self::make_key(&s)).or_insert(vec![]).push(s);
        }
        m.into_values().collect()
    }

    #[inline]
    fn make_key(s: &str) -> [i32; 26] {
        let mut result = [0; 26];
        for c in s.bytes() {
            let i = (c - b'a') as usize;
            result[i] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_util::*;
    use test_case::test_case;

    use itertools::Itertools;
    #[test_case(vec_string!["eat","tea","tan","ate","nat","bat"] => using any_order(vec![vec_string!["bat"],vec_string!["nat","tan"],vec_string!["ate","eat","tea"]]) ; "case1")]
    #[test_case(vec_string![""] => vec![vec_string![""]] ; "case2")]
    #[test_case(vec_string!["a"] => vec![vec_string!["a"]] ; "case3")]
    #[test_case(vec_string!["gggd", "dddg"] => using any_order(vec![vec_string!["gggd"], vec_string!["dddg"]]) ; "case4")]
    fn test(strs: Vec<String>) -> Vec<Vec<String>> {
        Solution::group_anagrams(strs)
            .iter()
            .map(|v| v.iter().cloned().sorted().collect())
            .collect()
    }
}
