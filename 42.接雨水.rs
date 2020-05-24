/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        let (mut left, mut right): (usize, usize) = (0, height.len() - 1);
        let (mut ans, mut left_max, mut right_max): (i32, i32, i32) = (0, 0, 0);
        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    ans += (left_max - height[left]);
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    ans += (right_max - height[right]);
                }
                right -= 1;
            }
        }
        ans
    }
}
// @lc code=end
