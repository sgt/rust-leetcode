use std::cmp::Ordering;

use crate::Solution;

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

    /// 33. Search in Rotated Sorted Array
    pub fn search_in_rotated_sorted_array(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let pivot = Self::find_pivot(&nums);
        let (s, offset) = if target >= nums[pivot] && target <= nums[nums.len() - 1] {
            (&nums[pivot..], pivot)
        } else {
            (&nums[..pivot], 0)
        };
        Self::binary_search(s, target).map_or(-1, |v| (v + offset) as i32)
    }

    fn find_pivot(nums: &[i32]) -> usize {
        let (mut low, mut high) = (0, nums.len() - 1);

        while nums[low] > nums[high] {
            let mid = (low + high) / 2;
            if nums[mid] > nums[high] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }

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
        Self::binary_search(s, target).is_some()
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

        let mut high = bckwd(nums.len()-1, 0);
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
    use crate::Solution;
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

    #[test_case(0, vec![3, 1], 3)]
    #[test_case(4, vec![4, 5, 6, 7, 0, 1, 2], 0)]
    #[test_case(-1, vec![4, 5, 6, 7, 0, 1, 2], 3)]
    #[test_case(-1, vec![1], 3)]
    #[test_case(0, vec![0, 1, 2], 0)]
    #[test_case(-1, vec![], 0 ; "empty")]
    fn test_search_in_rotated_sorted_array(result: i32, nums: Vec<i32>, target: i32) {
        assert_eq!(
            result,
            Solution::search_in_rotated_sorted_array(nums, target)
        );
    }

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
