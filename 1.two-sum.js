/*
 * @lc app=leetcode id=1 lang=javascript
 *
 * [1] Two Sum
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
    for (let crr in nums) {
        for (let i in nums) {
            if (i != crr) {
                if (nums[crr] + nums[i] === target) {
                    return [crr, i].sort((a, b) => a - b);
                }
            }
        }
    }
};
// @lc code=end