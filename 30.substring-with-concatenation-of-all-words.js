/*
 * @lc app=leetcode id=30 lang=javascript
 *
 * [30] Substring with Concatenation of All Words
 *
 * https://leetcode.com/problems/substring-with-concatenation-of-all-words/description/
 *
 * algorithms
 * Hard (24.18%)
 * Likes:    632
 * Dislikes: 1012
 * Total Accepted:    147.2K
 * Total Submissions: 608.9K
 * Testcase Example:  '"barfoothefoobarman"\n["foo","bar"]'
 *
 * You are given a string, s, and a list of words, words, that are all of the
 * same length. Find all starting indices of substring(s) in s that is a
 * concatenation of each word in words exactly once and without any intervening
 * characters.
 * 
 * Example 1:
 * 
 * 
 * Input:
 * ⁠ s = "barfoothefoobarman",
 * ⁠ words = ["foo","bar"]
 * Output: [0,9]
 * Explanation: Substrings starting at index 0 and 9 are "barfoor" and "foobar"
 * respectively.
 * The output order does not matter, returning [9,0] is fine too.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * ⁠ s = "wordgoodgoodgoodbestword",
 * ⁠ words = ["word","good","best","word"]
 * Output: []
 * 
 * 
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string[]} words
 * @return {number[]}
 */
var findSubstring = function (s, words) {
    // Sanity check
    if (!words || words.length === 0) return [];

    const m = words.length,
        n = words[0].length,
        len = m * n,
        result = [];

    // Build the word-count hash map
    const map = {};
    for (word of words) map[word] = ~~map[word] + 1;

    // Try every possible start position i
    for (let i = 0; i < s.length - len + 1; i++) {
        // Make a copy of the hash map
        const temp = Object.assign({}, map);

        for (let j = i; j < i + len; j += n) {
            const str = s.substr(j, n);
            // Cannot find the word in hash map (words list), try another position
            if (!(str in temp)) break;
            // All the same word str are found, remove it from the hash map
            if (--temp[str] === 0) delete temp[str];
        }

        // We have gone through the whole s and used all our words in the list
        if (Object.keys(temp).length === 0) result.push(i);
    }

    return result;
};
// @lc code=end