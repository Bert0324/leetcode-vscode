/*
 * @lc app=leetcode id=605 lang=javascript
 *
 * [605] Can Place Flowers
 *
 * https://leetcode.com/problems/can-place-flowers/description/
 *
 * algorithms
 * Easy (31.37%)
 * Likes:    551
 * Dislikes: 290
 * Total Accepted:    72.6K
 * Total Submissions: 231.4K
 * Testcase Example:  '[1,0,0,0,1]\n1'
 *
 * Suppose you have a long flowerbed in which some of the plots are planted and
 * some are not. However, flowers cannot be planted in adjacent plots - they
 * would compete for water and both would die.
 * 
 * Given a flowerbed (represented as an array containing 0 and 1, where 0 means
 * empty and 1 means not empty), and a number n, return if n new flowers can be
 * planted in it without violating the no-adjacent-flowers rule.
 * 
 * Example 1:
 * 
 * Input: flowerbed = [1,0,0,0,1], n = 1
 * Output: True
 * 
 * 
 * 
 * Example 2:
 * 
 * Input: flowerbed = [1,0,0,0,1], n = 2
 * Output: False
 * 
 * 
 * 
 * Note:
 * 
 * The input array won't violate no-adjacent-flowers rule.
 * The input array size is in the range of [1, 20000].
 * n is a non-negative integer which won't exceed the input array size.
 * 
 * 
 */

// @lc code=start
/**
 * @param {number[]} flowerbed
 * @param {number} n
 * @return {boolean}
 */
var canPlaceFlowers = function (flowerbed, n) {
    return flowerbed.concat([2]).reduce((acc, crr) => {
        if (acc.lastLast == null) {
            acc.lastLast = crr;
        } else if (acc.last == null) {
            if (acc.lastLast === 0 && crr === 0) {
                acc.lastLast = 1;
                acc.sumNum++;
            } else if (crr === 2 && acc.lastLast === 0) {
                acc.sumNum++;
            }
            acc.last = crr;
        } else {
            if (crr === 2) {
                if (acc.last === 0 && acc.lastLast === 0) {
                    acc.sumNum++;
                }
            } else if (acc.lastLast === 0 && acc.last === 0 && crr === 0) {
                acc.lastLast = 1;
                acc.sumNum++;
            } else {
                acc.lastLast = acc.last;
            }
            acc.last = crr;
        }
        return acc;
    }, {
        lastLast: null,
        last: null,
        sumNum: 0
    }).sumNum >= n;
};
// @lc code=end