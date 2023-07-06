use crate::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = Vec::len(&nums1) + Vec::len(&nums2);
        let mut ordered = Self::ordered_iter(&nums1, &nums2);
        if total_len % 2 == 0 {
            let result: Vec<&i32> = ordered.skip(total_len / 2 - 1).take(2).collect();
            ((result[0] + result[1]) as f64) / 2.0
        } else {
            *ordered.nth(total_len / 2).unwrap() as f64
        }
    }

    fn ordered_iter<'a, T: PartialOrd>(
        v1: &'a Vec<T>,
        v2: &'a Vec<T>,
    ) -> impl Iterator<Item = &'a T> {
        let mut i1 = 0;
        let mut i2 = 0;
        std::iter::from_fn(move || {
            if i1 == Vec::len(v1) && i2 == Vec::len(v2) {
                None
            } else if i1 == v1.len() {
                let v = &v2[i2];
                i2 += 1;
                Some(v)
            } else if i2 == v2.len() || v1[i1] < v2[i2] {
                let v = &v1[i1];
                i1 += 1;
                Some(v)
            } else {
                let v = &v2[i2];
                i2 += 1;
                Some(v)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::median_sorted_arrays::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
        assert_eq!(3.5, Solution::find_median_sorted_arrays(vec![], vec![3, 4]));
        assert_eq!(3.0, Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4, 5]));
    }
}
