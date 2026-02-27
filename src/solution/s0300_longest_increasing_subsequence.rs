pub struct Solution;

impl Solution {
    fn length_of_lis_inefficient(nums: Vec<i32>) -> i32 {
        // stores LIS up to this element
        let mut result = 0;
        let mut lis_list: Vec<i32> = vec![0; nums.len()];
        for i in 0..nums.len() {
            // find previous LIS that this element might continue
            let mut current_longest = 0;
            for j in 0..i {
                if nums[j] < nums[i] {
                    current_longest = current_longest.max(lis_list[j]);
                }
            }
            current_longest += 1;
            lis_list[i] = current_longest;
            result = result.max(current_longest);
            println!("{:?}", lis_list);
        }
        result
    }

    fn length_of_lis_efficient(nums: Vec<i32>) -> i32 {
        todo!()
    }

    /// 300. Longest Increasing Subsequence
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::length_of_lis_inefficient(nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_increasing_subsequence() {
        for f in [
            Solution::length_of_lis_inefficient,
            Solution::length_of_lis_efficient,
        ] {
            assert_eq!(4, f(vec![10, 9, 2, 5, 3, 7, 101, 18]));
            assert_eq!(4, f(vec![0, 1, 0, 3, 2, 3]));
            assert_eq!(1, f(vec![7, 7, 7, 7, 7, 7, 7]));
            assert_eq!(0, f(vec![]));
        }
    }
}
