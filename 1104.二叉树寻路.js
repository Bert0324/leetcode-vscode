/*
 * @lc app=leetcode.cn id=1104 lang=javascript
 *
 * [1104] 二叉树寻路
 *
 * https://leetcode-cn.com/problems/path-in-zigzag-labelled-binary-tree/description/
 *
 * algorithms
 * Medium (66.13%)
 * Likes:    18
 * Dislikes: 0
 * Total Accepted:    2.5K
 * Total Submissions: 3.8K
 * Testcase Example:  '14'
 *
 * 在一棵无限的二叉树上，每个节点都有两个子节点，树中的节点 逐行 依次按 “之” 字形进行标记。
 * 
 * 如下图所示，在奇数行（即，第一行、第三行、第五行……）中，按从左到右的顺序进行标记；
 * 
 * 而偶数行（即，第二行、第四行、第六行……）中，按从右到左的顺序进行标记。
 * 
 * 
 * 
 * 给你树上某一个节点的标号 label，请你返回从根节点到该标号为 label 节点的路径，该路径是由途经的节点标号所组成的。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：label = 14
 * 输出：[1,3,4,14]
 * 
 * 
 * 示例 2：
 * 
 * 输入：label = 26
 * 输出：[1,2,6,10,26]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= label <= 10^6
 * 
 * 
 */

// @lc code=start
/**
 * @param {number} label
 * @return {number[]}
 */
var pathInZigZagTree = function(label) {

    /** 
     *  For example, 14 is 0b1110, shift 1 bit to the left to 0b111, 
     *  Inverts the bits except the first bit to 0b100, which is 14's parent value.
     */
    const ret = [label];
    while (label !== 1) {
        label = (label >>> 1) ^(1 << (label >>> 1).toString(2).length - 1) - 1;
        ret.unshift(label);
    }
    return ret;
};
// @lc code=end

