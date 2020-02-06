/*
 * @lc app=leetcode.cn id=96 lang=rust
 *
 * [96] 不同的二叉搜索树
 *
 * https://leetcode-cn.com/problems/unique-binary-search-trees/description/
 *
 * algorithms
 * Medium (63.60%)
 * Likes:    384
 * Dislikes: 0
 * Total Accepted:    27.9K
 * Total Submissions: 43.2K
 * Testcase Example:  '3'
 *
 * 给定一个整数 n，求以 1 ... n 为节点组成的二叉搜索树有多少种？
 *
 * 示例:
 *
 * 输入: 3
 * 输出: 5
 * 解释:
 * 给定 n = 3, 一共有 5 种不同结构的二叉搜索树:
 *
 * ⁠  1         3     3      2      1
 * ⁠   \       /     /      / \      \
 * ⁠    3     2     1      1   3      2
 * ⁠   /     /       \                 \
 * ⁠  2     1         2                 3
 *
 */

// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // catalan numbers: 1, 2, 5, 14, 42, 132, 429, 1430, 4862 ...
        // let mut c: i64 = 1;
        // for i in 1..(n as i64) {
        //     c = c * 2 * (2 * (i as i64) + 1) / ((i as i64) + 2);
        // }
        // c as i32

        // dp: https://leetcode-cn.com/problems/unique-binary-search-trees/solution/bu-tong-de-er-cha-sou-suo-shu-by-leetcode/
        let n = n as usize;
        let mut c = vec![0; n + 1];
        c[0] = 1;
        c[1] = 1;
        for i in 2..n + 1 {
            for j in 1..i + 1 {
                c[i] += c[j - 1] * c[i - j];
            }
        }
        c[n] as i32
    }
}
// @lc code=end
