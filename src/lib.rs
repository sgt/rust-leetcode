use std::cmp::Ordering;

use listnode::ListNode;

pub mod listnode;
pub mod median_sorted_arrays;

pub struct Solution;

impl Solution {
    /// Two Sum II - Input Array Is Sorted
    pub fn two_sum2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i1 = 0;
        let mut i2 = numbers.len() - 1;
        while i2 > i1 {
            let v1 = &numbers[i1];
            let v2 = &numbers[i2];
            let f = target - v1;
            match v2.cmp(&f) {
                Ordering::Equal => return vec![i1 as i32 + 1, i2 as i32 + 1],
                Ordering::Greater => i2 -= 1,
                Ordering::Less => i1 += 1,
            }
        }
        vec![]
    }

    /// Add Two Numbers
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_helper(l1, l2, 0)
    }

    fn add_two_numbers_helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        overhead: i32,
    ) -> Option<Box<ListNode>> {
        fn extract(l: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
            if let Some(v) = l {
                (v.val, v.next)
            } else {
                (0, None)
            }
        }

        if l1.is_none() && l2.is_none() && overhead == 0 {
            None
        } else {
            let (v1, next1) = extract(l1);
            let (v2, next2) = extract(l2);
            let sum = v1 + v2 + overhead;
            let newval = sum % 10;
            let overhead = sum / 10;
            Some(Box::new(ListNode {
                val: newval,
                next: Self::add_two_numbers_helper(next1, next2, overhead),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        listnode::{arr_to_linkedlist, linkedlist_to_vec},
        Solution,
    };

    #[test]
    fn test_two_sum2() {
        assert_eq!(vec![1, 2], Solution::two_sum2(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 3], Solution::two_sum2(vec![2, 3, 4], 6));
        assert_eq!(vec![1, 2], Solution::two_sum2(vec![-1, 0], -1));
    }

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            vec![0],
            linkedlist_to_vec(Solution::add_two_numbers(
                arr_to_linkedlist(&[0]),
                arr_to_linkedlist(&[0])
            ))
        );
        assert_eq!(
            vec![7, 0, 8],
            linkedlist_to_vec(Solution::add_two_numbers(
                arr_to_linkedlist(&[2, 4, 3]),
                arr_to_linkedlist(&[5, 6, 4])
            ))
        )
    }
}
