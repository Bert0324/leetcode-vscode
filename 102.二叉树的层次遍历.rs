/*
 * @lc app=leetcode.cn id=102 lang=rust
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut ret = Vec::new();
        if root.is_none() {
            return ret;
        }
        let mut sub_vec = Vec::new();
        let mut queue = VecDeque::new();
        let mut cur_level = 0;
        queue.push_back((0, root.clone()));
        while !queue.is_empty() {
            if let Some((lev, Some(node))) = queue.pop_front() {
                if lev > cur_level {
                    ret.push(sub_vec.clone());
                    sub_vec.clear();
                    cur_level = lev;
                }
                sub_vec.push(node.borrow().val);
                queue.push_back((lev + 1, node.borrow().left.clone()));
                queue.push_back((lev + 1, node.borrow().right.clone()));
            }
        }
    
        if !sub_vec.is_empty() {
            ret.push(sub_vec);
        }
    
        ret
    }
}
// @lc code=end
