use super::s0704_binary_search::Solution as S;

pub struct Solution;

impl Solution {
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
        S::binary_search(s, target).map_or(-1, |v| (v + offset) as i32)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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
}
