use crate::util::list_node::ListNode;

pub struct Solution;

impl Solution {
    /// 2. Add Two Numbers
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
    use crate::util::list_node::{linkedlist_to_vec, arr_to_linkedlist};

    use super::*;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            vec![0],
            linkedlist_to_vec(&Solution::add_two_numbers(
                arr_to_linkedlist(&[0]),
                arr_to_linkedlist(&[0])
            ))
        );
        assert_eq!(
            vec![7, 0, 8],
            linkedlist_to_vec(&Solution::add_two_numbers(
                arr_to_linkedlist(&[2, 4, 3]),
                arr_to_linkedlist(&[5, 6, 4])
            ))
        );
    }
}
