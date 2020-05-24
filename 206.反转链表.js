/*
 * @lc app=leetcode.cn id=206 lang=javascript
 *
 * [206] 反转链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var reverseList = function(head) {
  if (!head || !head.next) return head;
  const list = [];
  const getList = item => {
    list.push(item.val);
    if (item.next) getList(item.next);
  };
  getList(head);
  const changeValue = item => {
    const val = list.pop();
    if (val !== undefined) {
      item.val = val;
      changeValue(item.next);
    }
  };
  changeValue(head);
  return head;
};
// @lc code=end
