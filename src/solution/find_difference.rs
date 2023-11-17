use crate::Solution;
use std::collections::HashSet;

impl Solution {
    /// # [2215. Find the Difference of Two Arrays](https://leetcode.com/problems/find-the-difference-of-two-arrays/)
    ///
    /// Given two **0-indexed**  integer arrays `nums1` and `nums2`, return a list `answer` of size `2` where:
    ///
    /// - `answer[0]` is a list of all **distinct**  integers in `nums1` which are **not**  present in `nums2`.
    /// - `answer[1]` is a list of all **distinct**  integers in `nums2` which are **not**  present in `nums1`.
    ///
    /// **Note**  that the integers in the lists may be returned in **any**  order.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums1 = [1,2,3], nums2 = [2,4,6]
    /// Output: [[1,3],[4,6]]
    /// **Explanation:
    /// ** For nums1, nums1[1] = 2 is present at index 0 of nums2, whereas nums1[0] = 1 and nums1[2] = 3 are not present in nums2. Therefore, answer[0] = [1,3].
    /// For nums2, nums2[0] = 2 is present at index 1 of nums1, whereas nums2[1] = 4 and nums2[2] = 6 are not present in nums2. Therefore, answer[1] = [4,6].
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums1 = [1,2,3,3], nums2 = [1,1,2,2]
    /// Output: [[3],[]]
    /// **Explanation:
    /// ** For nums1, nums1[2] and nums1[3] are not present in nums2. Since nums1[2] == nums1[3], their value is only included once and answer[0] = [3].
    /// Every integer in nums2 is present in nums1. Therefore, answer[1] = [].
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums1.length, nums2.length <= 1000`
    /// - `-1000 <= nums1[i], nums2[i] <= 1000`
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        vec![
            set1.difference(&set2).cloned().collect(),
            set2.difference(&set1).cloned().collect(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let mut answer = vec![vec![1, 3], vec![4, 6]];
        let mut output = Solution::find_difference(nums1, nums2);
        for item in &mut answer {
            item.sort();
        }
        for item in &mut output {
            item.sort();
        }
        assert_eq!(output, answer);
    }

    #[test]
    fn example2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let answer = vec![vec![3], vec![]];
        assert_eq!(Solution::find_difference(nums1, nums2), answer);
    }
}
