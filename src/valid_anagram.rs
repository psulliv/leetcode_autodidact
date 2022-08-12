//! https://leetcode.com/problems/valid-anagram/
#![allow(unused)]

struct Solution;
impl Solution {
    pub fn valid_anagram(s: String, t: String) -> bool {
        Solution::valid_anagram_naive(s, t)
    }

    pub fn valid_anagram_naive(mut s: String, mut t: String) -> bool {
        for letter in s.chars() {
            if let Some(letter_idx) = t.find(letter) {
                t.remove(letter_idx);
            } else {
                return false;
            }
        }
        if !t.is_empty() {
            return false;
        }
        true
    }

    pub fn valid_anagram_sorted(mut s: String, mut t: String) -> bool {
        let mut counter = 0;
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort();
        let mut t = t.chars().collect::<Vec<char>>();
        t.sort();

        for (s_char, t_char) in s.iter().zip(t.iter()) {
            if s_char != t_char {
                return false;
            } else {
                counter += 1;
            }
        }
        if (counter != s.len()) || (counter != t.len()) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_letter_anagrams_valid() {
        assert_eq!(
            true,
            Solution::valid_anagram(String::from("a"), String::from("a"))
        );
    }

    #[test]
    fn one_letter_anagrams_invalid() {
        assert_eq!(
            false,
            Solution::valid_anagram(String::from("a"), String::from("b"))
        );
    }

    #[test]
    fn two_letter_anagram_valid() {
        assert_eq!(
            true,
            Solution::valid_anagram(String::from("ab"), String::from("ba"))
        );
    }

    #[test]
    fn two_letter_anagram_invalid() {
        assert_eq!(
            false,
            Solution::valid_anagram(String::from("ab"), String::from("bb"))
        );
    }

    #[test]
    fn larger_anagram_valid() {
        assert_eq!(
            true,
            Solution::valid_anagram(String::from("banana"), String::from("baaann"))
        );
    }
}
