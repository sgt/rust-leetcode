pub struct Solution;

impl Solution {
    /// 1480. Running Sum of 1d Array
    pub fn running_sum(nums: &[i32]) -> Vec<i32> {
        let (_, result) = nums.iter().fold((0, vec![]), |(sum, mut acc), i| {
            acc.push(sum + i);
            (sum + i, acc)
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(&[1, 2, 3, 4]));
        assert_eq!(vec![1, 2, 3, 4, 5], Solution::running_sum(&[1, 1, 1, 1, 1]));
        assert_eq!(
            vec![3, 4, 6, 16, 17],
            Solution::running_sum(&[3, 1, 2, 10, 1])
        );
    }
}
