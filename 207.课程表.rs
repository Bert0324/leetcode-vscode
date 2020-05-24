/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    // topological sorting
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut v: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); num_courses as usize];
        for p in prerequisites.iter() {
            v[p[0] as usize].1.push(p[1] as usize);
            v[p[1] as usize].0 += 1;
        }
        let mut stack: Vec<usize> = Vec::new();
        for (i, e) in (0..).zip(v.iter()) {
            if e.0 == 0 {
                stack.push(i);
            }
        }
        let mut count = 0;
        while let Some(last) = stack.pop() {
            count += 1;
            for i in v[last].1.clone() {
                v[i].0 -= 1;
                if v[i].0 == 0 {
                    stack.push(i);
                }
            }
        }
        count == num_courses
    }
}
// @lc code=end
