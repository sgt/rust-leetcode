pub struct Solution;

impl Solution {
    /// O(n^2) solution
    pub fn length_of_lis_inefficient(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut lis_list: Vec<i32> = vec![0; nums.len()]; // stores LIS up to this element
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
        }
        result
    }

    /// O(n*log(n)) solution
    pub fn length_of_lis_efficient(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        // d[l] stores lowest element with which a LIS of length l is terminated
        let mut d: Vec<i32> = vec![i32::MAX; nums.len() + 1];
        d[0] = i32::MIN;
        for n in nums {
            if let Err(i) = d.binary_search(&n) {
                d[i] = n;
                result = result.max(i as i32)
            }
        }
        result
    }

    /// 300. Longest Increasing Subsequence
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::length_of_lis_efficient(nums)
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
            assert_eq!(0, f(vec![]));
            assert_eq!(4, f(vec![10, 9, 2, 5, 3, 7, 101, 18]));
            assert_eq!(4, f(vec![0, 1, 0, 3, 2, 3]));
            assert_eq!(4, f(vec![0, 1, 0, 3, 2, 2, 3]));
            assert_eq!(1, f(vec![7, 7, 7, 7, 7, 7, 7]));
        }
    }
}
