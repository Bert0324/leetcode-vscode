/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] 二叉树的最大深度
 *
 * https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/description/
 *
 * algorithms
 * Easy (71.39%)
 * Likes:    433
 * Dislikes: 0
 * Total Accepted:    115.4K
 * Total Submissions: 160.4K
 * Testcase Example:  '[3,9,20,null,null,15,7]'
 *
 * 给定一个二叉树，找出其最大深度。
 *
 * 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
 *
 * 说明: 叶子节点是指没有子节点的节点。
 *
 * 示例：
 * 给定二叉树 [3,9,20,null,null,15,7]，
 *
 * ⁠   3
 * ⁠  / \
 * ⁠ 9  20
 * ⁠   /  \
 * ⁠  15   7
 *
 * 返回它的最大深度 3 。
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::cmp;
        if root.is_none() {
            return 0;
        }
        fn add_depth(
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
        ) -> i32 {
            match (left, right) {
                (Some(l), Some(r)) => cmp::max(
                    add_depth(l.borrow().left.clone(), l.borrow().right.clone(), depth + 1),
                    add_depth(r.borrow().left.clone(), r.borrow().right.clone(), depth + 1),
                ),
                (Some(l), None) => {
                    add_depth(l.borrow().left.clone(), l.borrow().right.clone(), depth + 1)
                }
                (None, Some(r)) => {
                    add_depth(r.borrow().left.clone(), r.borrow().right.clone(), depth + 1)
                }
                _ => depth,
            }
        }
        let left = root.as_ref().and_then(|v| v.borrow().left.clone());
        let right = root.as_ref().and_then(|v| v.borrow().right.clone());
        return add_depth(left, right, 1);
        // if root == None {
        //     return 0;
        // } else {
        //     let root = root.unwrap();
        //     let left_depth = Self::max_depth(root.borrow().left.clone());
        //     let right_depth = Self::max_depth(root.borrow().right.clone());
        //     if left_depth > right_depth {
        //         return left_depth + 1;
        //     } else {
        //         return right_depth + 1;
        //     }
        // }
    }
}
// @lc code=end
