/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子序和
 *
 * https://leetcode-cn.com/problems/maximum-subarray/description/
 *
 * algorithms
 * Easy (48.22%)
 * Likes:    1548
 * Dislikes: 0
 * Total Accepted:    149.8K
 * Total Submissions: 306.9K
 * Testcase Example:  '[-2,1,-3,4,-1,2,1,-5,4]'
 *
 * 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
 *
 * 示例:
 *
 * 输入: [-2,1,-3,4,-1,2,1,-5,4],
 * 输出: 6
 * 解释: 连续子数组 [4,-1,2,1] 的和最大，为 6。
 *
 *
 * 进阶:
 *
 * 如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的分治法求解。
 *
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // https://leetcode-cn.com/problems/maximum-subarray/solution/zui-da-zi-xu-he-cshi-xian-si-chong-jie-fa-bao-li-f/
        // greedy
        // let mut ret = nums[0];
        // let mut current_sum = 0;
        // for num in nums {
        //     if current_sum > 0 {
        //         current_sum += num;
        //     } else {                    // when this part's sum is less than 0, it means this part is meaningless for the whole part
        //         current_sum = num;
        //     }
        //     ret = max(ret, current_sum);
        // }
        // ret

        // dynamic programming
        let mut ret = nums[0];
        let mut last = nums[0];
        for (i, num) in nums.iter().enumerate() {
            if i != 0 {
                last = max(*num, last + num);
                ret = max(ret, last);
            }
        }
        ret
    }
}
// @lc code=end
