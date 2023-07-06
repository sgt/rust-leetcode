use super::s0704_binary_search::Solution as S;

pub struct Solution;

impl Solution {
    /// 81. Search in Rotated Sorted Array II
    pub fn search_in_rotated_non_distinct_array(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }

        let pivot = Self::find_pivot_non_distinct(&nums);
        let s = if target >= nums[pivot] && target <= nums[nums.len() - 1] {
            &nums[pivot..]
        } else {
            &nums[..pivot]
        };
        S::binary_search(s, target).is_some()
    }

    fn find_pivot_non_distinct(nums: &[i32]) -> usize {
        let fwd = |i: usize, max: usize| {
            let mut r = i;
            while r < max && nums[r] == nums[r + 1] {
                r += 1;
            }
            r
        };

        let bckwd = |i: usize, min: usize| {
            let mut r = i;
            while r > min && nums[r] == nums[r - 1] {
                r -= 1;
            }
            r
        };

        let mut high = bckwd(nums.len() - 1, 0);
        let mut low = fwd(0, high);

        while nums[low] >= nums[high] && low != high {
            let mid = (low + high) / 2;
            if nums[mid] > nums[high] {
                low = fwd(mid, high - 1) + 1;
            } else {
                high = bckwd(mid, low);
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(true, vec![2, 5, 6, 0, 0, 1, 2], 0 ; "1")]
    #[test_case(false, vec![2, 5, 6, 0, 0, 1, 2], 3 ; "2")]
    #[test_case(false, vec![], 3 ; "empty")]
    #[test_case(true, vec![0], 0 ; "single")]
    #[test_case(false, vec![5, 5, 5, 5], 0 ; "fives miss")]
    #[test_case(true, vec![5, 5, 5, 5], 5 ; "fives hit")]
    #[test_case(true, vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1], 2 ; "two surrounded by ones")]
    fn test_search_in_rotated_sorted_non_distinct_array(result: bool, nums: Vec<i32>, target: i32) {
        assert_eq!(
            result,
            Solution::search_in_rotated_non_distinct_array(nums, target)
        );
    }
}
