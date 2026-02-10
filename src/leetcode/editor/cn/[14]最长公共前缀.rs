//编写一个函数来查找字符串数组中的最长公共前缀。
//
// 如果不存在公共前缀，返回空字符串 ""。
//
//
//
// 示例 1：
//
//
//输入：strs = ["flower","flow","flight"]
//输出："fl"
//
//
// 示例 2：
//
//
//输入：strs = ["dog","racecar","car"]
//输出：""
//解释：输入不存在公共前缀。
//
//
//
// 提示：
//
//
// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] 如果非空，则仅由小写英文字母组成
//
//
// Related Topics 字典树 数组 字符串 👍 3450 👎 0

pub struct Solution;

// 1. 'outer: loop
// 2. break 'outer

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut index = 0;
        'outer: loop {
            let mut x = None;
            for (i, str) in strs.iter().enumerate() {
                let curr = str.get(index..index + 1);
                if curr == None {
                    break 'outer;
                }

                if i == 0 {
                    x = curr;
                } else {
                    if x != curr {
                        break 'outer;
                    }
                }
            }
            index += 1;
        }

        return strs[0][0..index].to_string();
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let res = Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!(res, "fl");
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ]);
        assert_eq!(res, "");
    }

    #[test]
    fn example_leetcode_3() {
        let res = Solution::longest_common_prefix(vec![
            "abc".to_string(),
            "abc".to_string(),
            "abc".to_string(),
        ]);
        assert_eq!(res, "abc");
    }
}
