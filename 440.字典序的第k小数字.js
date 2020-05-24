/*
 * @lc app=leetcode.cn id=440 lang=javascript
 *
 * [440] 字典序的第K小数字
 */

// @lc code=start
/**
 * @param {number} n
 * @param {number} k
 * @return {number}
 */
var findKthNumber = function(n, k) {
    // get total sub nodes count below a prefix
    const getCount = prefix => {
        const data = {
            current: prefix,
            next: prefix + 1,
            count: 0
        };
        while (data.current <= n) {
            data.count += Math.min(data.next, n + 1) - data.current;
            data.current *= 10;
            data.next *= 10;
        }
        return data.count;
    };
    
    const data = {
        rank: 1,        // rank
        num: 1          // current total number
    }

    while (data.rank < k) {
        const count = getCount(data.num);
        // check whether rank is over k
        if (data.rank + count > k) {
            data.num *= 10;
            data.rank += 1;
        } else {
            data.rank += count;
            data.num += 1;
        }
    }
    return data.num;
};
// @lc code=end

