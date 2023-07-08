use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, usize> = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            let pair = target - n;
            match m.get(&pair) {
                Some(&j) => return vec![j as i32, i as i32],
                None => m.insert(n, i),
            };
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![2,7,11,15], 9 => vec![0,1])]
    #[test_case(vec![3,2,4], 6 => vec![1,2])]
    #[test_case(vec![3,3], 6 => vec![0,1])]
    pub fn test(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Solution::two_sum(nums, target)
    }
}
