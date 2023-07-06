use crate::util::list_node::{linkedlist_len, linkedlist_skip, ListNode};

pub struct Solution;

impl Solution {
    /// 876. Middle of the Linked List
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = linkedlist_len(&head);
        linkedlist_skip(head, len / 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::list_node::{arr_to_linkedlist, linkedlist_to_vec};

    use super::*;

    #[test]
    fn test_middle_node() {
        assert_eq!(
            vec![3, 4, 5],
            linkedlist_to_vec(&Solution::middle_node(arr_to_linkedlist(&[1, 2, 3, 4, 5])))
        );
        assert_eq!(
            vec![4, 5, 6],
            linkedlist_to_vec(&Solution::middle_node(arr_to_linkedlist(&[
                1, 2, 3, 4, 5, 6
            ])))
        );
    }
}
