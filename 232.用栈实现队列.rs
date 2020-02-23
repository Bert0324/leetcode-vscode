/*
 * @lc app=leetcode.cn id=232 lang=rust
 *
 * [232] 用栈实现队列
 *
 * https://leetcode-cn.com/problems/implement-queue-using-stacks/description/
 *
 * algorithms
 * Easy (61.83%)
 * Likes:    129
 * Dislikes: 0
 * Total Accepted:    30.7K
 * Total Submissions: 49.1K
 * Testcase Example:  '["MyQueue","push","push","peek","pop","empty"]\n[[],[1],[2],[],[],[]]'
 *
 * 使用栈实现队列的下列操作：
 * 
 * 
 * push(x) -- 将一个元素放入队列的尾部。
 * pop() -- 从队列首部移除元素。
 * peek() -- 返回队列首部的元素。
 * empty() -- 返回队列是否为空。
 * 
 * 
 * 示例:
 * 
 * MyQueue queue = new MyQueue();
 * 
 * queue.push(1);
 * queue.push(2);  
 * queue.peek();  // 返回 1
 * queue.pop();   // 返回 1
 * queue.empty(); // 返回 false
 * 
 * 说明:
 * 
 * 
 * 你只能使用标准的栈操作 -- 也就是只有 push to top, peek/pop from top, size, 和 is empty
 * 操作是合法的。
 * 你所使用的语言也许不支持栈。你可以使用 list 或者 deque（双端队列）来模拟一个栈，只要是标准的栈操作即可。
 * 假设所有操作都是有效的 （例如，一个空的队列不会调用 pop 或者 peek 操作）。
 * 
 * 
 */

// @lc code=start
#[derive(Default)]
struct MyQueue {
    stack: Vec<i32>,
    queue: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default() 
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        while let Some(q) = self.queue.pop() {
             self.stack.push(q); 
        }  
        self.stack.push(x);
        while let Some(s) = self.stack.pop() { 
            self.queue.push(s); 
        }
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.queue.pop().unwrap()
    }
    
    /** Get the front element. */
    fn peek(&self) -> i32 {
        *self.queue.last().unwrap()
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end

