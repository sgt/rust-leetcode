use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    /// 167. Two Sum II - Input Array Is Sorted
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
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(vec![1, 2], Solution::two_sum2(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 3], Solution::two_sum2(vec![2, 3, 4], 6));
        assert_eq!(vec![1, 2], Solution::two_sum2(vec![-1, 0], -1));
    }
}
