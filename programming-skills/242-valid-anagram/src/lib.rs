pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut counts = [0; 26];

        for ch in s.chars() {
            // Convert character to 0-25 index
            counts[(ch as usize) - ('a' as usize)] += 1;
        }

        for ch in t.chars() {
            // Convert character to 0-25 index
            counts[(ch as usize) - ('a' as usize)] -= 1;
        }

        counts.iter().all(|&count| count == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::is_anagram(format!("anagram"), format!("nagaram"));
        assert!(result);
    }

    #[test]
    fn test_2() {
        let result = Solution::is_anagram(format!("rat"), format!("car"));
        assert!(!result);
    }

    #[test]
    fn test_3() {
        let result = Solution::is_anagram(format!("a"), format!("abb"));
        assert!(!result);
    }
}
