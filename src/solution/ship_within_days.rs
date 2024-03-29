use crate::Solution;

impl Solution {
    /// # [1011. Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/)
    ///
    /// A conveyor belt has packages that must be shipped from one port to another within `days` days.
    ///
    /// The `i^th` package on the conveyor belt has a weight of `weights[i]`. Each day, we load the ship with packages on the conveyor belt (in the order given by `weights`). We may not load more weight than the maximum weight capacity of the ship.
    ///
    /// Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within `days` days.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: weights = [1,2,3,4,5,6,7,8,9,10], days = 5
    /// Output: 15
    /// Explanation: A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
    /// 1st day: 1, 2, 3, 4, 5
    /// 2nd day: 6, 7
    /// 3rd day: 8
    /// 4th day: 9
    /// 5th day: 10
    ///
    /// Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and splitting the packages into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: weights = [3,2,2,4,1,4], days = 3
    /// Output: 6
    /// Explanation: A ship capacity of 6 is the minimum to ship all the packages in 3 days like this:
    /// 1st day: 3, 2
    /// 2nd day: 2, 4
    /// 3rd day: 1, 4
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: weights = [1,2,3,1,1], days = 4
    /// Output: 3
    /// Explanation:
    /// 1st day: 1
    /// 2nd day: 2
    /// 3rd day: 3
    /// 4th day: 1, 1
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= days <= weights.length <= 5 * 10^4`
    /// - `1 <= weights[i] <= 500`
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut low = weights.iter().copied().max().unwrap_or(0);
        let mut high: i32 = weights.iter().sum::<i32>() + 1;
        let feasible = |cap: i32| -> bool {
            let mut load = 0;
            let mut cnt = 1;
            for w in &weights {
                load += *w;
                if load > cap {
                    if cnt == days {
                        return false;
                    }
                    cnt += 1;
                    load = *w;
                }
            }
            true
        };
        while low != high {
            let mid = low + (high - low) / 2;
            if feasible(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let days = 5;
        assert_eq!(Solution::ship_within_days(weights, days), 15);
    }
    #[test]
    fn example2() {
        let weights = vec![3, 2, 2, 4, 1, 4];
        let days = 3;
        assert_eq!(Solution::ship_within_days(weights, days), 6);
    }
    #[test]
    fn example3() {
        let weights = vec![1, 2, 3, 1, 1];
        let days = 4;
        assert_eq!(Solution::ship_within_days(weights, days), 3);
    }
}
