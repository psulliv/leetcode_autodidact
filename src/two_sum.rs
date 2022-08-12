//! https://leetcode.com/problems/two-sum/
#![allow(unused)]

// Given an array of integers nums and target return the
// indices of the two numbers that add up to target

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Solution::two_sum_naive(nums, target)
    }

    fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (first_idx, &first_num) in nums.iter().enumerate() {
            for (sec_idx, &sec_num) in nums[(first_idx + 1)..].iter().enumerate() {
                if (first_num + sec_num) == target {
                    return vec![
                        i32::try_from(first_idx).unwrap(),
                        i32::try_from(sec_idx).unwrap(),
                    ];
                }
            }
        }
        vec![0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        assert_eq!(vec![1, 3], Solution::two_sum(vec![1, 2, 3, 4], 6));
    }
}
