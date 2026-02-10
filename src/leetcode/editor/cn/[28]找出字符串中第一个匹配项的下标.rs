//给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。
//如果 needle 不是 haystack 的一部分，则返回 -1 。
//
//
//
// 示例 1：
//
//
//输入：haystack = "sadbutsad", needle = "sad"
//输出：0
//解释："sad" 在下标 0 和 6 处匹配。
//第一个匹配项的下标是 0 ，所以返回 0 。
//
//
// 示例 2：
//
//
//输入：haystack = "leetcode", needle = "leeto"
//输出：-1
//解释："leeto" 没有在 "leetcode" 中出现，所以返回 -1 。
//
//
//
//
// 提示：
//
//
// 1 <= haystack.length, needle.length <= 10⁴
// haystack 和 needle 仅由小写英文字符组成
//
//
// Related Topics 双指针 字符串 字符串匹配 👍 2520 👎 0

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut i = 0;
        while i < haystack.len() && i + needle.len() <= haystack.len() {
            if Some(needle.trim()) == haystack.get(i..i + needle.len()) {
                return i as i32;
            }
            i += 1;
        }

        -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let res = Solution::str_str("sadbutsad".to_string(), "sad".to_string());
        assert_eq!(res, 0);
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::str_str("leetcode".to_string(), "leeto".to_string());
        assert_eq!(res, -1);
    }

    #[test]
    fn example_leetcode_3() {
        let res = Solution::str_str("hello".to_string(), "ll".to_string());
        assert_eq!(res, 2);
    }

    #[test]
    fn example_leetcode_4() {
        let res = Solution::str_str("a".to_string(), "a".to_string());
        assert_eq!(res, 0);
    }
}
