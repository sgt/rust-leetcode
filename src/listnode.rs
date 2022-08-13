#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn linkedlist_to_vec(ll: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = &ll;
    while let Some(v) = current {
        result.push(v.val);
        current = &v.next;
    }
    result
}

pub fn linkedlist_to_vec_rec(ln: Option<Box<ListNode>>) -> Vec<i32> {
    // TODO wildly inefficient
    match ln {
        None => vec![],
        Some(v) => {
            let mut r = linkedlist_to_vec_rec(v.next);
            r.insert(0, v.val);
            r
        }
    }
}

pub fn arr_to_linkedlist(v: &[i32]) -> Option<Box<ListNode>> {
    let mut result = None;
    let mut current = &mut result;
    for i in v {
        current.replace(Box::new(ListNode::new(*i)));
        current = &mut current.as_mut().unwrap().next;
    }
    result
}

pub fn arr_to_linkedlist_rec(v: &[i32]) -> Option<Box<ListNode>> {
    if v.is_empty() {
        None
    } else {
        Some(Box::new(ListNode {
            val: v[0],
            next: arr_to_linkedlist(&v[1..]),
        }))
    }
}

#[cfg(test)]
mod test {
    use super::{
        arr_to_linkedlist, arr_to_linkedlist_rec, linkedlist_to_vec, linkedlist_to_vec_rec,
        ListNode,
    };

    fn l23() -> Option<Box<ListNode>> {
        let l3 = Some(Box::new(ListNode { val: 3, next: None }));
        Some(Box::new(ListNode { val: 2, next: l3 }))
    }

    fn assert_linkedlist_to_vec(f: fn(Option<Box<ListNode>>) -> Vec<i32>) {
        assert_eq!(vec![] as Vec<i32>, f(None));
        assert_eq!(vec![2, 3], f(l23()));
    }

    fn assert_arr_to_linkedlist(f: fn(&[i32]) -> Option<Box<ListNode>>) {
        assert_eq!(f(vec![].as_slice()), None);
        assert_eq!(f(vec![2, 3].as_slice()), l23());
    }

    #[test]
    fn test_linkedlist_to_vec() {
        assert_linkedlist_to_vec(linkedlist_to_vec);
        assert_linkedlist_to_vec(linkedlist_to_vec_rec);
    }

    #[test]
    fn test_arr_to_linkedlist() {
        assert_arr_to_linkedlist(arr_to_linkedlist);
        assert_arr_to_linkedlist(arr_to_linkedlist_rec);
    }
}
