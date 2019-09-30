/*
 * @lc app=leetcode id=93 lang=javascript
 *
 * [93] Restore IP Addresses
 *
 * https://leetcode.com/problems/restore-ip-addresses/description/
 *
 * algorithms
 * Medium (32.52%)
 * Likes:    804
 * Dislikes: 337
 * Total Accepted:    153.9K
 * Total Submissions: 473.1K
 * Testcase Example:  '"25525511135"'
 *
 * Given a string containing only digits, restore it by returning all possible
 * valid IP address combinations.
 * 
 * Example:
 * 
 * 
 * Input: "25525511135"
 * Output: ["255.255.11.135", "255.255.111.35"]
 * 
 * 
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string[]}
 */
var restoreIpAddresses = function (s) {

    const match = [];

    const select = (sub, ip) => {

        if (sub.length > 12) {
            return;
        }

        if (ip.length === 3) {
            ip.push(sub);
            if (ip.join('') === s && sub.length < 4 && sub.length > 0) {
                match.push(ip);
            }
            return;
        }

        for (let i = 1; i < Math.min(4, sub.length + 1); i++) {
            select(sub.substr(i), ip.concat([sub.substr(0, i)]));
        }
    };
    select(s, []);

    return match.map(ip => {
        let m = true;
        ip.forEach(num => {
            if (Number(num) > 255 ||
                (num[0] === '0' && num.length > 1)) {
                m = false;
            }
        });

        if (m) {
            return ip.join('.')
        }
        return null;
    }).filter(item => {
        if (item != null) {
            return item;
        }
    });
};
// @lc code=end