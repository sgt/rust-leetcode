use super::s1480_running_sum_of_1d_array::Solution as S;

pub struct Solution;

impl Solution {
    /// 724. Find Pivot Index
    pub fn pivot_index(nums: &[i32]) -> i32 {
        let mut nums_rev = nums.to_vec();
        nums_rev.reverse();
        let sums = S::running_sum(nums);
        let sums_rev = S::running_sum(&nums_rev);
        let l = nums.len();
        for i in 0..l {
            if sums[i] == sums_rev[l - i - 1] {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pivot_index() {
        assert_eq!(3, Solution::pivot_index(&[1, 7, 3, 6, 5, 6]));
        // 1,8,11,17,22,28   28,27,20,17,11,6
        assert_eq!(-1, Solution::pivot_index(&[1, 2, 3]));
        assert_eq!(0, Solution::pivot_index(&[2, 1, -1]));
    }
}
