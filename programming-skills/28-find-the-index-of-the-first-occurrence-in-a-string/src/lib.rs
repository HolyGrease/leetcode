pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_bytes = needle.as_bytes();

        haystack
            .as_bytes()
            .windows(needle.len())
            .enumerate()
            .find(|(_index, slice)| *slice == needle_bytes)
            .map(|(index, _value)| index as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::str_str(format!("sadbutsad"), format!("sad"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_2() {
        let result = Solution::str_str(format!("leetcode"), format!("leeto"));
        assert_eq!(result, -1);
    }
}
