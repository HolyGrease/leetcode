pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut result: Vec<i32> = digits
            .into_iter()
            .rev()
            .map(|digit| {
                let sum = digit + carry;
                carry = sum / 10;
                sum % 10
            })
            .collect();

        if carry > 0 {
            result.push(carry);
        }

        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::plus_one(vec![1, 2, 3]);
        assert_eq!(result, vec![1, 2, 4]);
    }

    #[test]
    fn test_2() {
        let result = Solution::plus_one(vec![4, 3, 2, 1]);
        assert_eq!(result, vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_3() {
        let result = Solution::plus_one(vec![9]);
        assert_eq!(result, vec![1, 0]);
    }
}
