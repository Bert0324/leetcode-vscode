/*
 * @lc app=leetcode.cn id=102 lang=javascript
 *
 * [102] 二叉树的层次遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-level-order-traversal/description/
 *
 * algorithms
 * Medium (59.88%)
 * Likes:    356
 * Dislikes: 0
 * Total Accepted:    72.4K
 * Total Submissions: 119.7K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给定一个二叉树，返回其按层次遍历的节点值。 （即逐层地，从左到右访问所有节点）。
 * 
 * 例如:
 * 给定二叉树: [3,9,20,null,null,15,7],
 * 
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 * 
 * 
 * 返回其层次遍历结果：
 * 
 * [
 * ⁠ [3],
 * ⁠ [9,20],
 * ⁠ [15,7]
 * ]
 * 
 * 
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
var levelOrder = function(root) {
    const ret = [];
    let currentLevel = 0;
    const queue = [{
        level: currentLevel,
        node: root
    }];
    let subArray = [];
    while (queue.length !== 0) {
        const ele = queue.shift();
        if (ele.level > currentLevel) {
            currentLevel = ele.level;
            ret.push([...subArray]);
            subArray = [];
        }
        if (ele.node) {
            subArray.push(ele.node.val);
            queue.push({
                level: currentLevel + 1,
                node: ele.node.left
            });
            queue.push({
                level: currentLevel + 1,
                node: ele.node.right
            });
        }
    }
    return ret;
};
// @lc code=end

