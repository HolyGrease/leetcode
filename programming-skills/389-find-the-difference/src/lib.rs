pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.chars()
            .chain(t.chars())
            .fold(0, |acc, ch| acc ^ (ch as u32))
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::find_the_difference(format!("abcd"), format!("abcde"));
        assert_eq!(result, 'e');
    }
    #[test]
    fn test_2() {
        let result = Solution::find_the_difference(format!(""), format!("y"));
        assert_eq!(result, 'y');
    }
    #[test]
    fn test_3() {
        let result = Solution::find_the_difference(format!("a"), format!("aa"));
        assert_eq!(result, 'a');
    }
}
