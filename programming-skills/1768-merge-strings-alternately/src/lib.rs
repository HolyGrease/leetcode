pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());
        let mut i = 0;
        let mut j = 0;
        let mut word1_chars = word1.chars();
        let mut word2_chars = word2.chars();

        while i < word1.len() || j < word2.len() {
            if i < word1.len() {
                result.push(word1_chars.next().unwrap());
                i += 1;
            }
            if j < word2.len() {
                result.push(word2_chars.next().unwrap());
                j += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::merge_alternately("abc".to_string(), "pqr".to_string());
        assert_eq!(result, "apbqcr");
    }

    #[test]
    fn test_2() {
        let result = Solution::merge_alternately("ab".to_string(), "pqrs".to_string());
        assert_eq!(result, "apbqrs");
    }

    #[test]
    fn test_3() {
        let result = Solution::merge_alternately("abcd".to_string(), "pq".to_string());
        assert_eq!(result, "apbqcd");
    }
}
