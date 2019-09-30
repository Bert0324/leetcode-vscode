/*
 * @lc app=leetcode id=41 lang=javascript
 *
 * [41] First Missing Positive
 *
 * https://leetcode.com/problems/first-missing-positive/description/
 *
 * algorithms
 * Hard (29.82%)
 * Likes:    2113
 * Dislikes: 646
 * Total Accepted:    245.6K
 * Total Submissions: 823.7K
 * Testcase Example:  '[1,2,0]'
 *
 * Given an unsorted integer array, find the smallest missing positive
 * integer.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,2,0]
 * Output: 3
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [3,4,-1,1]
 * Output: 2
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: [7,8,9,11,12]
 * Output: 1
 * 
 * 
 * Note:
 * 
 * Your algorithm should run in O(n) time and uses constant extra space.
 * 
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var firstMissingPositive = function (nums) {
    nums = nums.filter((value, index, arr) => {
        return value > 0 && arr.indexOf(value) === index;
    });
    nums.sort((a, b) => {
        return a - b;
    });
    if (nums[0] > 1 || !nums[0]) {
        return 1;
    } else {
        for (let i = 0; i < nums.length - 1; i++) {
            if (nums[i] + 1 !== nums[i + 1]) {
                return nums[i] + 1;
            }
        }
    }
    return nums[nums.length - 1] + 1;
};
// @lc code=end