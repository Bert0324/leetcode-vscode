/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-inorder-traversal/description/
 *
 * algorithms
 * Medium (69.07%)
 * Likes:    373
 * Dislikes: 0
 * Total Accepted:    95.8K
 * Total Submissions: 137.3K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给定一个二叉树，返回它的中序 遍历。
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
 * 输出: [1,3,2]
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // left -> root -> right (traverse balanced binary tree in the increasing sequence)
        let mut container = vec![];
        if root.is_none() {
            return container;
        }
        Self::fill_vec(root, &mut container)
    }

    fn fill_vec(node: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) -> Vec<i32> {
        match (node) {
            Some(n) => {
                let mut left_filled = Self::fill_vec(n.borrow().left.clone(), list);
                left_filled.push(n.borrow().val);
                Self::fill_vec(n.borrow().right.clone(), &mut left_filled)
            }
            None => list.to_vec(),
        }
    }
}
// @lc code=end
