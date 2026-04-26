pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut insert_position = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[insert_position] = nums[i];
                insert_position += 1;
            }
        }
        for i in insert_position..nums.len() {
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
