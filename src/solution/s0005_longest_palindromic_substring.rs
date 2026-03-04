pub struct Solution;

impl Solution {
    fn odd_palindrome(a: &[u8], i: usize) -> (usize, usize) {
        let mut i1 = i;
        let mut i2 = i;
        while i1 > 0 && i2 < a.len() - 1 && a[i1 - 1] == a[i2 + 1] {
            i1 -= 1;
            i2 += 1;
        }
        (i1, i2)
    }

    fn even_palindrome(a: &[u8], i: usize) -> Option<(usize, usize)> {
        let mut l1 = i;
        let mut l2 = i + 1;
        if l1 >= a.len() || l2 >= a.len() || a[l1] != a[l2] {
            return None;
        }
        while l1 > 0 && l2 < a.len() - 1 && a[l1 - 1] == a[l2 + 1] {
            l1 -= 1;
            l2 += 1;
        }
        Some((l1, l2))
    }

    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".into();
        }

        let bytes = s.as_bytes();

        let mut i1 = 0;
        let mut i2 = 0;
        let mut i = 0;

        // while there is still a chance to find a longer palindrome
        while i < (s.len() - (i2 - i1) / 2) {
            let (ci1, ci2) = Self::odd_palindrome(bytes, i);
            if (ci2 - ci1) > (i2 - i1) {
                (i1, i2) = (ci1, ci2);
            }

            if let Some((ci1, ci2)) = Self::even_palindrome(bytes, i)
                && (ci2 - ci1) > (i2 - i1)
            {
                (i1, i2) = (ci1, ci2);
            }

            i += 1;
        }

        s[i1..=i2].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(""=> "")]
    #[test_case("babad"=> "bab")]
    #[test_case("cbbd"=> "bb")]
    #[test_case("bb"=> "bb")]
    #[test_case("a"=> "a")]
    #[test_case("aaaa"=> "aaaa")]
    #[test_case("bbbbb"=> "bbbbb")]
    pub fn test_longest_palindrome(s: &str) -> String {
        Solution::longest_palindrome(s.into())
    }
}
