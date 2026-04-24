pub struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let string_len = s.len();

        // Iterate through all possible substring lengths.
        // A potential repeating substring's length (`n`) must be a divisor of the total string length (`string_len`).
        // We only need to check lengths up to half the string's total length.
        (1..=string_len / 2)
            .filter(|&n| string_len % n == 0)
            .any(|substring_len| {
                // Check if all subsequent segments of the string are identical to the `first_substring`.
                let s_bytes = s.as_bytes();
                let first_substring_bytes = &s.as_bytes()[..substring_len];

                // Start iterating from the second potential substring
                s_bytes[substring_len..]
                    .chunks(substring_len)
                    .all(|chunk| chunk == first_substring_bytes)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::repeated_substring_pattern(format!("abab"));
        assert!(result);
    }

    #[test]
    fn test_2() {
        let result = Solution::repeated_substring_pattern(format!("aba"));
        assert!(!result);
    }

    #[test]
    fn test_3() {
        let result = Solution::repeated_substring_pattern(format!("abcabcabcabc"));
        assert!(result);
    }
}
