//给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
//
// 有效字符串需满足：
//
//
// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
// 每个右括号都有一个对应的相同类型的左括号。
//
//
//
//
// 示例 1：
//
//
// 输入：s = "()"
//
//
// 输出：true
//
// 示例 2：
//
//
// 输入：s = "()[]{}"
//
//
// 输出：true
//
// 示例 3：
//
//
// 输入：s = "(]"
//
//
// 输出：false
//
// 示例 4：
//
//
// 输入：s = "([])"
//
//
// 输出：true
//
// 示例 5：
//
//
// 输入：s = "([)]"
//
//
// 输出：false
//
//
//
// 提示：
//
//
// 1 <= s.length <= 10⁴
// s 仅由括号 '()[]{}' 组成
//
//
// Related Topics 栈 字符串 👍 4877 👎 0

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec = vec![];
        for c in s.chars() {
            match c {
                ')' => {
                    if vec.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if vec.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if vec.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {
                    vec.push(c);
                }
            }
        }

        vec.is_empty()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let res = Solution::is_valid("()".to_string());
        assert_eq!(res, true);
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::is_valid("()[]{}".to_string());
        assert_eq!(res, true);
    }

    #[test]
    fn example_leetcode_3() {
        let res = Solution::is_valid("(]".to_string());
        assert_eq!(res, false);
    }

    #[test]
    fn example_leetcode_4() {
        let res = Solution::is_valid("([])".to_string());
        assert_eq!(res, true);
    }

    #[test]
    fn example_leetcode_5() {
        let res = Solution::is_valid("([)]".to_string());
        assert_eq!(res, false);
    }
}
