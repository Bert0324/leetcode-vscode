/*
 * @lc app=leetcode.cn id=145 lang=rust
 *
 * [145] 二叉树的后序遍历
 *
 * https://leetcode-cn.com/problems/binary-tree-postorder-traversal/description/
 *
 * algorithms
 * Hard (68.89%)
 * Likes:    215
 * Dislikes: 0
 * Total Accepted:    46.8K
 * Total Submissions: 67.1K
 * Testcase Example:  '[1,null,2,3]'
 *
 * 给定一个二叉树，返回它的 后序 遍历。
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
 * 输出: [3,2,1]
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // left -> right -> root (delete a node)
        let mut container = vec![];
        if root.is_none() {
            return container;
        }
        fn fill_vec(node: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) -> Vec<i32> {
            match (node) {
                Some(n) => {
                    let mut children_filled = fill_vec(
                        n.borrow().right.clone(),
                        &mut fill_vec(n.borrow().left.clone(), list),
                    );
                    children_filled.push(n.borrow().val);
                    children_filled
                }
                None => list.to_vec(),
            }
        }
        fill_vec(root, &mut container)
    }
}
// @lc code=end
