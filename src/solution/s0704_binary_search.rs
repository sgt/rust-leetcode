use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    /// 704. Binary Search
    pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
        let mut low = 0;
        let mut high = nums.len();
        while low < high {
            let mid = (low + high) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => return Some(mid),
                Ordering::Greater => low = mid + 1,
                Ordering::Less => high = mid,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(None, &[0, 1, 2, 3], 4)]
    #[test_case(Some(2), &[0, 1, 2, 3], 2)]
    #[test_case(Some(0), &[0, 1, 2, 3], 0)]
    #[test_case(Some(0), &[3], 3 ; "single hit")]
    #[test_case(None, &[5], -5 ; "single miss")]
    #[test_case(None, &[], -5 ; "empty")]
    fn test_binary_search(result: Option<usize>, nums: &[i32], target: i32) {
        assert_eq!(result, Solution::binary_search(nums, target));
    }
}
