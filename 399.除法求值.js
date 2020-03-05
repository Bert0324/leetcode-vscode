/*
 * @lc app=leetcode.cn id=399 lang=javascript
 *
 * [399] 除法求值
 *
 * https://leetcode-cn.com/problems/evaluate-division/description/
 *
 * algorithms
 * Medium (52.90%)
 * Likes:    102
 * Dislikes: 0
 * Total Accepted:    5.2K
 * Total Submissions: 9.7K
 * Testcase Example:  '[["a","b"],["b","c"]]\n' +
  '[2.0,3.0]\n' +
  '[["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]'
 *
 * 给出方程式 A / B = k, 其中 A 和 B 均为代表字符串的变量， k
 * 是一个浮点型数字。根据已知方程式求解问题，并返回计算结果。如果结果不存在，则返回 -1.0。
 * 
 * 示例 :
 * 给定 a / b = 2.0, b / c = 3.0
 * 问题:  a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ? 
 * 返回 [6.0, 0.5, -1.0, 1.0, -1.0 ]
 * 
 * 输入为:  vector<pair<string, string>> equations, vector<double>& values,
 * vector<pair<string, string>> queries(方程式，方程式结果，问题方程式)， 其中 equations.size()
 * == values.size()，即方程式的长度与方程式结果长度相等（程式与结果一一对应），并且结果值均为正数。以上为方程式的描述。
 * 返回vector<double>类型。
 * 
 * 基于上述例子，输入如下：
 * 
 * 
 * equations(方程式) = [ ["a", "b"], ["b", "c"] ],
 * values(方程式结果) = [2.0, 3.0],
 * queries(问题方程式) = [ ["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x",
 * "x"] ]. 
 * 
 * 
 * 输入总是有效的。你可以假设除法运算中不会出现除数为0的情况，且不存在任何矛盾的结果。
 * 
 */

// @lc code=start
/**
 * @param {string[][]} equations
 * @param {number[]} values
 * @param {string[][]} queries
 * @return {number[]}
 */
var calcEquation = function(equations, values, queries) {
    // Graph Adjacency Matrix
    const matrix = {};
    equations.forEach((pair, i) => {
        if (!matrix[pair[0]]) matrix[pair[0]] = {};
        if (!matrix[pair[1]]) matrix[pair[1]] = {};
        matrix[pair[0]][pair[1]] = values[i];
        matrix[pair[1]][pair[0]] = 1 / values[i];
    });
    const keys = Object.keys(matrix);
    keys.forEach(x => {
        keys.forEach(y => {
            if (matrix[x][y]) {
                keys.forEach(xy => {
                    if (matrix[y][xy]) matrix[x][xy] = matrix[y][xy] * matrix[x][y];
                })
            }
        })
    })
    return queries.map(query => matrix[query[0]] && matrix[query[0]][query[1]] ? matrix[query[0]][query[1]] : -1);
};
// @lc code=end

