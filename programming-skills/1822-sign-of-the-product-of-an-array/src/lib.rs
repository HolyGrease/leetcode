pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().map(|x| x.signum()).fold(1, |acc, x| acc * x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let result = Solution::array_sign(vec![1, 5, 0, 2, -3]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let result = Solution::array_sign(vec![-1, 1, -1, 1, -1]);
        assert_eq!(result, -1);
    }
}
