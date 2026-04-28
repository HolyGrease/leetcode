pub struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let diff = arr[1] - arr[0];
        arr.windows(2).all(|v| v[1] - v[0] == diff)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::can_make_arithmetic_progression(vec![3, 5, 1]);
        assert!(result);
    }

    #[test]
    fn test_2() {
        let result = Solution::can_make_arithmetic_progression(vec![1, 2, 4]);
        assert!(!result);
    }
}
