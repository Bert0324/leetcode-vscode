/*
 * @lc app=leetcode.cn id=95 lang=rust
 *
 * [95] 不同的二叉搜索树 II
 *
 * https://leetcode-cn.com/problems/unique-binary-search-trees-ii/description/
 *
 * algorithms
 * Medium (60.19%)
 * Likes:    273
 * Dislikes: 0
 * Total Accepted:    18.6K
 * Total Submissions: 30.3K
 * Testcase Example:  '3'
 *
 * 给定一个整数 n，生成所有由 1 ... n 为节点所组成的二叉搜索树。
 *
 * 示例:
 *
 * 输入: 3
 * 输出:
 * [
 * [1,null,3,2],
 * [3,2,null,1],
 * [3,1,null,null,2],
 * [2,1,3],
 * [1,null,2,null,3]
 * ]
 * 解释:
 * 以上的输出对应以下 5 种不同结构的二叉搜索树：
 *
 * ⁠  1         3     3      2      1
 * ⁠   \       /     /      / \      \
 * ⁠    3     2     1      1   3      2
 * ⁠   /     /       \                 \
 * ⁠  2     1         2                 3
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            vec![]
        } else {
            Self::get_tree_from_root(1, n)
        }
    }

    fn get_tree_from_root(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            vec![None] // MARK: empty node
        } else {
            let mut ret = vec![];
            for i in start..end + 1 {
                let left_nodes = Self::get_tree_from_root(start, i - 1);
                let right_nodes = Self::get_tree_from_root(i + 1, end);
                for left_node in left_nodes.iter() {
                    for right_node in right_nodes.iter() {
                        ret.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: left_node.clone(),
                            right: right_node.clone(),
                        }))));
                    }
                }
            }
            ret
        }
    }
}
// @lc code=end
