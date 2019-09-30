/*
 * @lc app=leetcode id=164 lang=javascript
 *
 * [164] Maximum Gap
 *
 * https://leetcode.com/problems/maximum-gap/description/
 *
 * algorithms
 * Hard (33.45%)
 * Likes:    593
 * Dislikes: 140
 * Total Accepted:    75.9K
 * Total Submissions: 226.9K
 * Testcase Example:  '[3,6,9,1]'
 *
 * Given an unsorted array, find the maximum difference between the successive
 * elements in its sorted form.
 * 
 * Return 0 if the array contains less than 2 elements.
 * 
 * Example 1:
 * 
 * 
 * Input: [3,6,9,1]
 * Output: 3
 * Explanation: The sorted form of the array is [1,3,6,9], either
 * (3,6) or (6,9) has the maximum difference 3.
 * 
 * Example 2:
 * 
 * 
 * Input: [10]
 * Output: 0
 * Explanation: The array contains less than 2 elements, therefore return 0.
 * 
 * Note:
 * 
 * 
 * You may assume all elements in the array are non-negative integers and fit
 * in the 32-bit signed integer range.
 * Try to solve it in linear time/space.
 * 
 * 
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var maximumGap = function (nums) {
    if (nums.length <= 1) return 0;
    //selection sorting
    for (let i = 0; i < nums.length; i++) {
        for (let j = i + 1; j < nums.length; j++) {
            if (nums[j] < nums[i]) {
                let temp = nums[j];
                nums[j] = nums[i];
                nums[i] = temp;
            }
        }
    }
    /*or
    nums.sort((a,b)=>{
        return a - b;
    });
    * */
    for (let i = 0; i < nums.length - 1; i++) {
        nums[i] = nums[i + 1] - nums[i];
    }
    nums.pop();
    return Math.max(...nums);
};
// @lc code=end