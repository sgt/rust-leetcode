pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::util::test_util::*;

    // #[test_case(&["eat","tea","tan","ate","nat","bat"] => vec![vec!["bat"],vec!["nat","tan"],vec!["ate","eat","tea"]])]
    // fn test(strs: &[String]) -> Vec<Vec<String>> {
    //     Solution::group_anagrams(&strs)
    // }
    
    #[test_case(1 => using any_order(vec![1,2,3]))]
    fn test(n:i32) -> Vec<i32> {
        vec![3,2,1]
    }
}
