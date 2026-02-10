//给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
//
// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
//
//
// 例如，121 是回文，而 123 不是。
//
//
//
//
// 示例 1：
//
//
//输入：x = 121
//输出：true
//
//
// 示例 2：
//
//
//输入：x = -121
//输出：false
//解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
//
//
// 示例 3：
//
//
//输入：x = 10
//输出：false
//解释：从右向左读, 为 01 。因此它不是一个回文数。
//
//
//
//
// 提示：
//
//
// -2³¹ <= x <= 2³¹ - 1
//
//
//
//
// 进阶：你能不将整数转为字符串来解决这个问题吗？
//
// Related Topics 数学 👍 3135 👎 0

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x = x.to_string();
        let y = x.as_bytes();
        let (mut a, mut b) = (0, y.len() - 1);
        while a < b {
            if y[a] != y[b] {
                return false;
            }

            a += 1;
            b -= 1;
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let res = Solution::is_palindrome(121);
        assert_eq!(res, true);
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::is_palindrome(-121);
        assert_eq!(res, false);
    }

    #[test]
    fn example_leetcode_3() {
        let res = Solution::is_palindrome(10);
        assert_eq!(res, false);
    }
}
