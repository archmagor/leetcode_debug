//给定一个表示 大整数 的整数数组 digits，其中 digits[i] 是整数的第 i 位数字。这些数字按从左到右，从最高位到最低位排列。这个大整数不包含
//任何前导 0。
//
// 将大整数加 1，并返回结果的数字数组。
//
//
//
// 示例 1：
//
//
//输入：digits = [1,2,3]
//输出：[1,2,4]
//解释：输入数组表示数字 123。
//加 1 后得到 123 + 1 = 124。
//因此，结果应该是 [1,2,4]。
//
//
// 示例 2：
//
//
//输入：digits = [4,3,2,1]
//输出：[4,3,2,2]
//解释：输入数组表示数字 4321。
//加 1 后得到 4321 + 1 = 4322。
//因此，结果应该是 [4,3,2,2]。
//
//
// 示例 3：
//
//
//输入：digits = [9]
//输出：[1,0]
//解释：输入数组表示数字 9。
//加 1 得到了 9 + 1 = 10。
//因此，结果应该是 [1,0]。
//
//
//
//
// 提示：
//
//
// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9
// digits 不包含任何前导 0。
//
//
// Related Topics 数组 数学 👍 1566 👎 0

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut vec = vec![];
        let mut forward = 1;
        for v in digits.iter().rev() {
            if v + forward >= 10 {
                vec.push(v + forward - 10);
                forward = 1;
            } else {
                vec.push(v + forward);
                forward = 0;
            }
        }
        if forward == 1 {
            vec.push(forward);
        }
        vec.reverse();
        vec
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let res = Solution::plus_one(vec![1, 2, 3]);
        assert_eq!(res, vec![1, 2, 4]);
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::plus_one(vec![4, 3, 2, 1]);
        assert_eq!(res, vec![4, 3, 2, 2]);
    }

    #[test]
    fn example_leetcode_3() {
        let res = Solution::plus_one(vec![9]);
        assert_eq!(res, vec![1, 0]);
    }
}
