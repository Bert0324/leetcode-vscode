/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 *
 * https://leetcode-cn.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (27.68%)
 * Likes:    1681
 * Dislikes: 0
 * Total Accepted:    173K
 * Total Submissions: 612.1K
 * Testcase Example:  '"babad"'
 *
 * 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
 *
 * 示例 1：
 *
 * 输入: "babad"
 * 输出: "bab"
 * 注意: "aba" 也是一个有效答案。
 *
 *
 * 示例 2：
 *
 * 输入: "cbbd"
 * 输出: "bb"
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(sv: String) -> String {
        let s: Vec<char> = sv.chars().collect();
        let len = s.len();
        if len == 0 {
            return "".to_string();
        }
        let mut start = 0;
        let mut end = 0;
        for i in 0..s.len() {
            let mut left = i;
            let mut right = i;
            while right + 1 < len && s[right + 1] == s[left] {
                // same and continuous chars
                right += 1;
            }
            while right + 1 < len && left > 0 && s[right + 1] == s[left - 1] {
                right += 1;
                left -= 1;
            }

            if right - left > end - start {
                // save max result
                end = right;
                start = left;
            }
        }
        s[start..=end].iter().collect()
    }
}
// @lc code=end
