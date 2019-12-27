/*
 * @lc app=leetcode.cn id=2 lang=javascript
 *
 * [2] 两数相加
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
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function(l1, l2) {
  if (!l1) return l2;
  if (!l2) return l1;
  const listNode = new ListNode(null);
  const combineList = (node_one, node_two, carry, node) => {
    // if (node_one === null && node_two === null && !carry) {
    //   node = null; // why can't point to right object?
    //   return;
    // }
    const val1 = Number(node_one ? node_one.val : 0);
    const val2 = Number(node_two ? node_two.val : 0);
    const num = Array.from((val1 + val2 + carry).toString());
    const carry_next = num.length === 2;
    const val = carry_next ? num[1] : num[0];
    node.val = val;
    if (
      (node_one ? node_one.next : null) === null &&
      (node_two ? node_two.next : null) === null &&
      !carry_next
    ) {
      node.next = null;
      return;
    }
    node.next = new ListNode(null);
    combineList(
      node_one ? node_one.next : null,
      node_two ? node_two.next : null,
      carry_next,
      node.next
    );
  };
  combineList(l1, l2, false, listNode);
  return listNode;

  // use recursion to parse ListNode to number add plus them, but int is too long lead to SO
  // if (!l1) return l2;
  // if (!l2) return l1;
  // // (linkedList: ListNode) => number
  // const extractList = linkedList => {
  //   const list = [];
  //   const getList = node => {
  //     list.push(node.val);
  //     if (node.next) {
  //       getList(node.next);
  //     }
  //   };
  //   getList(linkedList);
  //   return Number(list.reverse().join(""));
  // };
  // // (num: number) => ListNode
  // // TODO: 1e+30 can't be parsed to number list
  // const createList = num => {
  //   const list = Array.from(num.toString());
  //   const getListNode = node => {
  //     const val = list.shift();
  //     if (val) {
  //       node.val = val;
  //       if (list.length > 0) {
  //         node.next = getListNode(new ListNode(undefined));
  //       } else {
  //         node.next = null;
  //       }
  //     } else {
  //       node.next = null;
  //     }
  //     return node;
  //   };
  //   return getListNode(new ListNode(undefined));
  // };
  // return createList(extractList(l1) + extractList(l2));
};
// @lc code=end
