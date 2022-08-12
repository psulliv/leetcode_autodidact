#![allow(unused)]
struct Solution;

impl Solution {
    fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        // https://leetcode.com/problems/contains-duplicate/
        // Given an integer array nums, return true if any
        // value appears at least twice in the array.
        hashmap_contains_duplicate(nums)
    }
}

fn naive_contains_duplicate(mut nums: Vec<i32>) -> bool {
    // O(n^2)
    for i in 0..nums.len() - 1 {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }

    false
}

fn hashmap_contains_duplicate(mut nums: Vec<i32>) -> bool {
    // O(n), hashing is constant time, checking a hash membership
    // also, iterating each item in the vector is n
    use std::collections::HashMap;
    let mut counter: HashMap<i32, bool> = HashMap::new();
    for &num in nums.iter() {
        if counter.contains_key(&num) {
            return true;
        } else {
            counter.insert(num, true);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_element() {
        assert_eq!(false, Solution::contains_duplicate(vec![1]));
    }

    #[test]
    fn two_element() {
        assert_eq!(false, Solution::contains_duplicate(vec![1, 2]));
    }

    #[test]
    fn two_element_dup() {
        assert_eq!(true, Solution::contains_duplicate(vec![2, 2]));
    }
}
