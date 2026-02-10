//给你一个字符串 s，由若干单词组成，单词前后用一些空格字符隔开。返回字符串中 最后一个 单词的长度。
//
// 单词 是指仅由字母组成、不包含任何空格字符的最大子字符串。
//
//
//
// 示例 1：
//
//
//输入：s = "Hello World"
//输出：5
//解释：最后一个单词是“World”，长度为 5。
//
//
// 示例 2：
//
//
//输入：s = "   fly me   to   the moon  "
//输出：4
//解释：最后一个单词是“moon”，长度为 4。
//
//
// 示例 3：
//
//
//输入：s = "luffy is still joyboy"
//输出：6
//解释：最后一个单词是长度为 6 的“joyboy”。
//
//
//
//
// 提示：
//
//
// 1 <= s.length <= 10⁴
// s 仅有英文字母和空格 ' ' 组成
// s 中至少存在一个单词
//
//
// Related Topics 字符串 👍 801 👎 0

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut i = 0;
        let mut b = false;
        for v in s.chars().rev() {
            if v != ' ' {
                b = true;
                i += 1;
            } else if b {
                return i;
            }
        }
        i
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let res = Solution::length_of_last_word("Hello World".to_string());
        assert_eq!(res, 5);
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::length_of_last_word("   fly me   to   the moon  ".to_string());
        assert_eq!(res, 4);
    }

    #[test]
    fn example_leetcode_3() {
        let res = Solution::length_of_last_word("luffy is still joyboy".to_string());
        assert_eq!(res, 6);
    }
}
