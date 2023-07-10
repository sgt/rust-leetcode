use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    /// 18. 4Sum
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::k_sum(4, nums, target)
    }

    pub fn k_sum(k: usize, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        Self::k_sum_rec(k, &nums, target)
    }

    /// nums is always sorted
    fn k_sum_rec(k: usize, nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        if nums.len() < k {
            return vec![];
        }
        match k {
            2 => Self::two_sum(nums, target),
            _ if k > 2 => {
                let mut result = vec![];
                for (i, &n) in nums.iter().enumerate() {
                    if i > 0 && nums[i - 1] == n {
                        continue;
                    }
                    let mut result_per_n =
                        Self::k_sum_rec(k - 1, &nums[i + 1..], target.saturating_sub(n));
                    for v in result_per_n.iter_mut() {
                        v.push(n);
                    }
                    result.append(&mut result_per_n.into_iter().collect());
                }
                result.into_iter().collect()
            }
            _ => panic!("not interested in k < 2"),
        }
    }

    /// find two-sum in O(n)
    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut result = vec![];
        while left < right {
            if left > 0 && nums[left] == nums[left - 1] {
                left += 1;
                continue;
            }
            if right < nums.len() - 1 && nums[right] == nums[right + 1] {
                right -= 1;
                continue;
            }
            let sum = nums[left].saturating_add(nums[right]);
            match sum.cmp(&target) {
                Ordering::Equal => {
                    result.push(vec![nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                }
                Ordering::Greater => right -= 1,
                Ordering::Less => left += 1,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::Itertools;
    use test_case::test_case;

    #[test_case(vec![1,0,-1,0,-2,2], 0 => vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]] ; "case 1")]
    #[test_case(vec![2,2,2,2,2], 8 => vec![vec![2,2,2,2]] ; "case 2")]
    #[test_case(vec![1000000000,1000000000,1000000000,1000000000], -294967296 => with |v:Vec<Vec<i32>>| assert!(v.is_empty()) ; "overflow")]
    fn test_four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // make the result and the items sorted so order doesn't matter
        Solution::four_sum(nums, target)
            .iter()
            .map(|v| v.iter().copied().sorted().collect())
            .sorted()
            .collect_vec()
    }

    #[test_case(&[1, 2, 3, 4], 5 => vec![vec![1,4], vec![2,3]] ; "simple")]
    fn test_two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        Solution::two_sum(nums, target)
    }
}
