/*
 * @lc app=leetcode.cn id=128 lang=javascript
 *
 * [128] 最长连续序列
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function(nums) {
  class UnionFind {
    constructor(_nums) {
      this.mapper = {};
    }

    add(num) {
      this.mapper[num] = num;
    }

    contains(num) {
      return this.mapper[num] !== undefined;
    }

    find(num) {
      while (num !== this.mapper[num]) num = this.mapper[num];
      return num;
    }

    union(x, y) {
      const rootX = this.find(x);
      const rootY = this.find(y);
      if (rootX !== rootY) {
        this.mapper[rootX] = rootY;
      }
    }

    getMaxRoot() {
      const sizeMapper = Object.keys(this.mapper).reduce((acc, crr) => {
        const root = this.find(this.mapper[crr]);
        acc[root] = acc[root] ? acc[root] + 1 : 1;
        return acc;
      }, {});
      return Object.keys(sizeMapper).reduce(
        (acc, crr) => {
          const size = sizeMapper[crr];
          return Math.max(acc, size);
        },
        nums.length ? 1 : 0
      );
    }
  }

  const uf = new UnionFind(nums);
  nums.forEach(num => {
    if (uf.contains(num)) return true;
    uf.add(num);
    if (uf.contains(num - 1)) uf.union(num, num - 1);
    if (uf.contains(num + 1)) uf.union(num, num + 1);
  });
  return uf.getMaxRoot();
};
// @lc code=end
