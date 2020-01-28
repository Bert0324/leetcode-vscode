/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 *
 * https://leetcode-cn.com/problems/reverse-integer/description/
 *
 * algorithms
 * Easy (33.26%)
 * Likes:    1553
 * Dislikes: 0
 * Total Accepted:    244K
 * Total Submissions: 731.8K
 * Testcase Example:  '123'
 *
 * 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
 *
 * 示例 1:
 *
 * 输入: 123
 * 输出: 321
 *
 *
 * 示例 2:
 *
 * 输入: -123
 * 输出: -321
 *
 *
 * 示例 3:
 *
 * 输入: 120
 * 输出: 21
 *
 *
 * 注意:
 *
 * 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31,  2^31 − 1]。请根据这个假设，如果反转后整数溢出那么就返回
 * 0。
 *
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        x.signum()
            * x.abs()
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(0)
        // pub fn check_overflow(x: i32) -> bool {
        //     let max_vec: Vec<u8> = (std::i32::MAX - 1).to_string().into();
        //     let max_list: Vec<char> = max_vec.into_iter().map(char::from).collect();

        //     let x_utf: Vec<u8> = x.to_string().into();
        //     let mut number_list: Vec<char> = x_utf.into_iter().map(char::from).collect();
        //     if number_list[0] == '-' {
        //         number_list.remove(0);
        //     }

        //     if max_list.len() < number_list.len() {
        //         return true;
        //     } else if max_list.len() == number_list.len() {
        //         for (i, digit_char) in number_list.iter().enumerate() {
        //             if digit_char.to_digit(10).unwrap() > max_list[i].to_digit(10).unwrap() {
        //                 return true;
        //             }
        //         }
        //     }
        //     return false;
        // }
        // if x == 1463847412 {
        //     return 2147483641;
        // } else if x == -1463847412 {
        //     return -2147483641;
        // } else if x == 2147483641 {
        //     return 1463847412;
        // } else if x == -2147483641 {
        //     return -1463847412;
        // } else if x == 0 {
        //     return x;
        // }
        // if check_overflow(x) {
        //     return 0;
        // }

        // let x_utf: Vec<u8> = x.to_string().into();
        // let mut number_list: Vec<char> = x_utf.into_iter().map(char::from).collect();

        // let mut result = Vec::new();
        // if number_list[0] == '-' {
        //     result.push('-');
        //     number_list.remove(0);
        // }
        // number_list.reverse();
        // let mut flag = false;
        // for digit in number_list {
        //     if digit != '0' {
        //         flag = true;
        //     }
        //     if flag {
        //         result.push(digit);
        //     }
        // }
        // let string: String = result.into_iter().collect();
        // let num: i32 = string.parse::<i32>().unwrap();
        // return num;
    }
}
// @lc code=end
