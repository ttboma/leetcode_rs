use crate::Solution;

impl Solution {
    /// # [1480. Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)
    ///
    /// Given an array `nums`. We define a running sum of an array as`runningSum[i] = sum(nums[0]…nums[i])`.
    ///
    /// Return the running sum of `nums`.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: nums = [1,2,3,4]
    /// Output: [1,3,6,10]
    /// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: nums = [1,1,1,1,1]
    /// Output: [1,2,3,4,5]
    /// Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: nums = [3,1,2,10,1]
    /// Output: [3,4,6,16,17]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= nums.length <= 1000`
    /// - `-10^6<= nums[i] <=10^6`
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::running_sum(nums), vec![1, 3, 6, 10])
    }
    #[test]
    fn example2() {
        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::running_sum(nums), vec![1, 2, 3, 4, 5])
    }
    #[test]
    fn example3() {
        let nums = vec![3, 1, 2, 10, 1];
        assert_eq!(Solution::running_sum(nums), vec![3, 4, 6, 16, 17])
    }
    #[test]
    fn example4() {
        let nums = vec![];
        assert_eq!(Solution::running_sum(nums), vec![])
    }
}
