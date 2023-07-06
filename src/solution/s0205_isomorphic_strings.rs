use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    /// 205. Isomorphic Strings
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut m: HashMap<u8, u8> = HashMap::new();
        let mut seen: HashSet<u8> = HashSet::new();
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            match m.get(&s[i]) {
                Some(lu) => {
                    if *lu != t[i] {
                        return false;
                    }
                }
                None => {
                    if seen.contains(&t[i]) {
                        return false;
                    } else {
                        m.insert(s[i], t[i]);
                        seen.insert(t[i]);
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "abc".to_string(),
            "ttt".to_string()
        ));
    }
}
