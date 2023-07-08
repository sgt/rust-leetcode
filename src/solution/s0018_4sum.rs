use std::collections::{HashSet, BTreeSet};

pub struct Solution;

impl Solution {
    /// 18. 4Sum
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::k_sum(4, nums, target)
    }

    pub fn k_sum(k: usize, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        Self::k_sum_rec(k, &nums, target)
    }

    pub fn k_sum_rec(k: usize, nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        if nums.len() < k {
            return vec![];
        }
        match k {
            0 => vec![],
            1 => nums
                .iter()
                .filter_map(|&x| (x == target).then(|| vec![x]))
                .collect(),
            2 => {
                let mut result = BTreeSet::new();
                let mut s = HashSet::new();
                for n in nums {
                    let rest = target.saturating_sub(*n);
                    if s.contains(&rest) {
                        result.insert(vec![rest, *n]);
                    } else {
                        s.insert(n);
                    }
                }
                result.into_iter().collect()
            }
            _ => {
                let mut result = BTreeSet::new();
                for (i, n) in nums.iter().enumerate() {
                    let mut result_per_n =
                        Self::k_sum_rec(k - 1, &nums[i + 1..], target.saturating_sub(*n));
                    for v in result_per_n.iter_mut() {
                        v.push(*n);
                    }
                    result.append(&mut result_per_n.into_iter().collect());
                }
                result.into_iter().collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;
    use test_case::test_case;

    // #[test_case(vec!{1,0,-1,0,-2,2}, 0 => vec!{vec!{-2,-1,1,2},vec!{-2,0,0,2},vec!{-1,0,0,1}})]
    // #[test_case(vec!{2,2,2,2,2}, 8 => vec!{vec![2,2,2,2]})]
    // #[test_case(vec![1000000000,1000000000,1000000000,1000000000], -294967296 => vec![vec![1]])]
    fn test(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // TODO fix order for test
        Solution::four_sum(nums, target)
            .into_iter()
            .sorted()
            .collect_vec()
    }
}
