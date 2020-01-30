/*
 * @lc app=leetcode.cn id=144 lang=rust
 *
 * [144] 二叉树的前序遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-preorder-traversal/description/
 *
 * algorithms
 * Medium (62.68%)
 * Likes:    195
 * Dislikes: 0
 * Total Accepted:    62.7K
 * Total Submissions: 98.5K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给定一个二叉树，返回它的 前序 遍历。
 *
 * 示例:
 *
 * 输入: [1,null,2,3]
 * ⁠  1
 * ⁠   \
 * ⁠    2
 * ⁠   /
 * ⁠  3
 *
 * 输出: [1,2,3]
 *
 *
 * 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
 *
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // root -> left -> right (find a node)
        let mut container = vec![];
        if root.is_none() {
            return container;
        }
        Self::fill_vec(root, &mut container)
    }

    fn fill_vec(node: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) -> Vec<i32> {
        match (node) {
            Some(n) => {
                list.push(n.borrow().val);
                Self::fill_vec(
                    n.borrow().right.clone(),
                    &mut Self::fill_vec(n.borrow().left.clone(), list),
                )
            }
            None => list.to_vec(),
        }
    }
}
// @lc code=end
