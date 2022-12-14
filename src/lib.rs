use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use listnode::{linkedlist_len, linkedlist_skip, ListNode};

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
    /// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
    pub fn running_sum(nums: &[i32]) -> Vec<i32> {
        let (_, result) = nums.iter().fold((0, vec![]), |(sum, mut acc), i| {
            acc.push(sum + i);
            (sum + i, acc)
        });
        result
    }

    pub fn pivot_index(nums: &[i32]) -> i32 {
        let mut nums_rev = nums.to_vec();
        nums_rev.reverse();
        let sums = Solution::running_sum(nums);
        let sums_rev = Solution::running_sum(&nums_rev);
        let l = nums.len();
        for i in 0..l {
            if sums[i] == sums_rev[l - i - 1] {
                return i as i32;
            }
        }
        -1
    }

    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut m: HashMap<u8, u8> = HashMap::new();
        let mut seen: HashSet<u8> = HashSet::new();
        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..s.len() {
            match m.get(&s[i]) {
                Some(lu) => {
                    if *lu != t[i] {
                        return false;
                    }
                }
                None => {
                    if seen.contains(&t[i]) {
                        return false;
                    } else {
                        m.insert(s[i], t[i]);
                        seen.insert(t[i]);
                    }
                }
            }
        }
        true
    }

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = linkedlist_len(&head);
        linkedlist_skip(head, len / 2)
    }

    pub fn longest_palindrome(s: String) -> i32 {
        let mut freq_map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            freq_map.insert(c, freq_map.get(&c).unwrap_or(&0) + 1);
        }
        let freqs: Vec<i32> = freq_map.into_values().collect();
        let odd_exists: bool = freqs.iter().find(|x| *x % 2 == 1).map_or(false, |_| true);
        let even_entries: i32 = freqs
            .iter()
            .filter_map(|x| {
                if *x < 2 {
                    None
                } else if x % 2 == 1 {
                    Some(x - 1)
                } else {
                    Some(*x)
                }
            })
            .sum();
        if odd_exists {
            even_entries + 1
        } else {
            even_entries
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

    #[test]
    fn test_running_sum() {
        assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(&[1, 2, 3, 4]));
        assert_eq!(vec![1, 2, 3, 4, 5], Solution::running_sum(&[1, 1, 1, 1, 1]));
        assert_eq!(
            vec![3, 4, 6, 16, 17],
            Solution::running_sum(&[3, 1, 2, 10, 1])
        );
    }
    #[test]
    fn test_pivot_index() {
        assert_eq!(3, Solution::pivot_index(&[1, 7, 3, 6, 5, 6]));
        // 1,8,11,17,22,28   28,27,20,17,11,6
        assert_eq!(-1, Solution::pivot_index(&[1, 2, 3]));
        assert_eq!(0, Solution::pivot_index(&[2, 1, -1]));
    }

    #[test]
    fn test_is_isomorphic() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "abc".to_string(),
            "ttt".to_string()
        ));
    }

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

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(7, Solution::longest_palindrome("abccccdd".into()));
        assert_eq!(1, Solution::longest_palindrome("a".into()));
    }
}
