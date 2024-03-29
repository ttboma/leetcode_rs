use crate::Solution;

impl Solution {
    /// # [1646. Get Maximum in Generated Array](https://leetcode.com/problems/get-maximum-in-generated-array/description/)
    ///
    /// You are given an integer `n`. A **0-indexed**  integer array `nums` of length `n + 1` is generated in the following way:
    ///
    /// - `nums[0] = 0`
    /// - `nums[1] = 1`
    /// - `nums[2 * i] = nums[i]` when `2 <= 2 * i <= n`
    /// - `nums[2 * i + 1] = nums[i] + nums[i + 1]` when `2 <= 2 * i + 1 <= n`
    ///
    /// Return** ** the **maximum**  integer in the array `nums`​​​.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: n = 7
    /// Output: 3
    /// Explanation: According to the given rules:
    ///     nums[0] = 0
    ///     nums[1] = 1
    ///     nums[(1 * 2) = 2] = nums[1] = 1
    ///     nums[(1 * 2) + 1 = 3] = nums[1] + nums[2] = 1 + 1 = 2
    ///     nums[(2 * 2) = 4] = nums[2] = 1
    ///     nums[(2 * 2) + 1 = 5] = nums[2] + nums[3] = 1 + 2 = 3
    ///     nums[(3 * 2) = 6] = nums[3] = 2
    ///     nums[(3 * 2) + 1 = 7] = nums[3] + nums[4] = 2 + 1 = 3
    /// Hence, nums = [0,1,1,2,1,3,2,3], and the maximum is max(0,1,1,2,1,3,2,3) = 3.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: n = 2
    /// Output: 1
    /// Explanation: According to the given rules, nums = [0,1,1]. The maximum is max(0,1,1) = 1.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: n = 3
    /// Output: 2
    /// Explanation: According to the given rules, nums = [0,1,1,2]. The maximum is max(0,1,1,2) = 2.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `0 <= n <= 100`
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut res = vec![0, 1];
        for i in 1..(n as usize + 1) / 2 {
            res.push(res[i]);
            res.push(res[i] + res[i + 1]);
        }
        *res.iter().take(n as usize + 1).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::get_maximum_generated(2), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::get_maximum_generated(3), 2);
    }
}
