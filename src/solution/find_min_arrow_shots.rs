use super::Solution;

impl Solution {
    /// # [452. Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// There are some spherical balloons taped onto a flat wall that represents the XY-plane. The balloons are represented as a 2D integer array `points` where `points[i] = [x<sub>start</sub>, x<sub>end</sub>]` denotes a balloon whose **horizontal diameter**  stretches between `x<sub>start</sub>` and `x<sub>end</sub>`. You do not know the exact y-coordinates of the balloons.
    ///
    /// Arrows can be shot up **directly vertically**  (in the positive y-direction) from different points along the x-axis. A balloon with `x<sub>start</sub>` and `x<sub>end</sub>` is **burst**  by an arrow shot at `x` if `x<sub>start</sub> <= x <= x<sub>end</sub>`. There is **no limit**  to the number of arrows that can be shot. A shot arrow keeps traveling up infinitely, bursting any balloons in its path.
    ///
    /// Given the array `points`, return the **minimum**  number of arrows that must be shot to burst all balloons.
    ///
    /// **Example 1:**
    ///
    /// ```txt
    /// Input: points = [[10,16],[2,8],[1,6],[7,12]]
    /// Output: 2
    /// Explanation: The balloons can be burst by 2 arrows:
    /// - Shoot an arrow at x = 6, bursting the balloons [2,8] and [1,6].
    /// - Shoot an arrow at x = 11, bursting the balloons [10,16] and [7,12].
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: points = [[1,2],[3,4],[5,6],[7,8]]
    /// Output: 4
    /// Explanation: One arrow needs to be shot for each balloon for a total of 4 arrows.
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: points = [[1,2],[2,3],[3,4],[4,5]]
    /// Output: 2
    /// Explanation: The balloons can be burst by 2 arrows:
    /// - Shoot an arrow at x = 2, bursting the balloons [1,2] and [2,3].
    /// - Shoot an arrow at x = 4, bursting the balloons [3,4] and [4,5].
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `1 <= points.length <= 10^5`
    /// - `points[i].length == 2`
    /// - `-2^31 <= x<sub>start</sub> < x<sub>end</sub> <= 2^31 - 1`
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        points.sort_by_key(|p| p[1]);
        let mut count = 1;
        let mut i = 0;
        let mut j = 1;
        while j < points.len() {
            if points[i][1] < points[j][0] {
                count += 1;
                i = j;
                j += 1;
            } else {
                j += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        );
    }
}
