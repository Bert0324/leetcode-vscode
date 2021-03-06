/*
 * @lc app=leetcode.cn id=257 lang=rust
 *
 * [257] 二叉树的所有路径
 *
 * https://leetcode-cn.com/problems/binary-tree-paths/description/
 *
 * algorithms
 * Easy (60.96%)
 * Likes:    193
 * Dislikes: 0
 * Total Accepted:    22.8K
 * Total Submissions: 36.9K
 * Testcase Example:  '[1,2,3,null,5]'
 *
 * 给定一个二叉树，返回所有从根节点到叶子节点的路径。
 *
 * 说明: 叶子节点是指没有子节点的节点。
 *
 * 示例:
 *
 * 输入:
 *
 * ⁠  1
 * ⁠/   \
 * 2     3
 * ⁠\
 * ⁠ 5
 *
 * 输出: ["1->2->5", "1->3"]
 *
 * 解释: 所有根节点到叶子节点的路径为: 1->2->5, 1->3
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut container: Vec<String> = vec![];
        if let Some(r) = root {
            Self::get_all_path(r, &mut vec![], &mut container)
        } else {
            container
        }
    }

    fn get_all_path(
        node: Rc<RefCell<TreeNode>>,
        path: &mut Vec<String>,
        list: &mut Vec<String>,
    ) -> Vec<String> {
        path.push(node.borrow().val.to_string());
        match (node.borrow().left.clone(), node.borrow().right.clone()) {
            (Some(l), Some(r)) => Self::get_all_path(
                l,
                &mut path.clone(),
                &mut Self::get_all_path(r, &mut path.clone(), list),
            ),
            (Some(l), None) => Self::get_all_path(l, &mut path.clone(), list),
            (None, Some(r)) => Self::get_all_path(r, &mut path.clone(), list),
            _ => {
                list.push(path.join("->"));
                list.to_vec()
            }
        }
    }
}
// @lc code=end
