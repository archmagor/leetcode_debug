//给你两个二进制字符串 a 和 b ，以二进制字符串的形式返回它们的和。
//
//
//
// 示例 1：
//
//
//输入:a = "11", b = "1"
//输出："100"
//
// 示例 2：
//
//
//输入：a = "1010", b = "1011"
//输出："10101"
//
//
//
// 提示：
//
//
// 1 <= a.length, b.length <= 10⁴
// a 和 b 仅由字符 '0' 或 '1' 组成
// 字符串如果不是 "0" ，就不含前导零
//
//
// Related Topics 位运算 数学 字符串 模拟 👍 1349 👎 0

pub struct Solution;

// 1. String::parse()
// 2. String::insert()

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut i = 0;
        let mut s = vec![];
        let mut forward = 0;
        while i < a.len() || i < b.len() {
            let a_i = if a.len() < i + 1 {
                0
            } else {
                a.get(a.len() - i - 1..a.len() - i)
                    .map(|v| v.parse().unwrap_or(0))
                    .unwrap_or(0)
            };

            let b_i = if b.len() < i + 1 {
                0
            } else {
                b.get(b.len() - i - 1..b.len() - i)
                    .map(|v| v.parse().unwrap_or(0))
                    .unwrap_or(0)
            };

            if a_i + b_i + forward == 3 {
                s.push('1');
                forward = 1;
            } else if a_i + b_i + forward == 2 {
                s.push('0');
                forward = 1;
            } else if a_i + b_i + forward == 1 {
                s.push('1');
                forward = 0;
            } else if a_i + b_i + forward == 0 {
                s.push('0');
                forward = 0;
            }
            i += 1
        }
        if forward == 1 {
            s.push('1');
        }
        s.into_iter().rev().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let res = Solution::add_binary("11".to_string(), "1".to_string());
        assert_eq!(res, "100".to_string());
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::add_binary("1010".to_string(), "1011".to_string());
        assert_eq!(res, "10101".to_string());
    }
}
