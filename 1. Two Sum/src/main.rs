// LeetCode Problem: Two Sum
// Problem Link: https://leetcode.com/problems/two-sum/
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = std::collections::HashMap::new();
        
        for (index, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&complement_index) = num_to_index.get(&complement) {
                return vec![complement_index as i32, index as i32];
            }
            num_to_index.insert(num, index);
        }
        
        vec![] // Return an empty vector if no solution is found
    }
}

// Create test for two_sum function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
        
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
        
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}