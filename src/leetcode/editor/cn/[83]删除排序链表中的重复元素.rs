//给定一个已排序的链表的头
// head ， 删除所有重复的元素，使每个元素只出现一次 。返回 已排序的链表 。
//
//
//
// 示例 1：
//
//
//输入：head = [1,1,2]
//输出：[1,2]
//
//
// 示例 2：
//
//
//输入：head = [1,1,2,3,3]
//输出：[1,2,3]
//
//
//
//
// 提示：
//
//
// 链表中节点数目在范围 [0, 300] 内
// -100 <= Node.val <= 100
// 题目数据保证链表已经按升序 排列
//
//
// Related Topics 链表 👍 1250 👎 0

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>

//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.as_mut()?;
        while let Some(mut tmp) = curr.next.take() {
            if tmp.val == curr.val {
                curr.next = tmp.next.take();
            } else {
                curr.next = Some(tmp);
                curr = curr.next.as_mut().unwrap();
            }
        }
        head
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let n_1_3 = Box::from(ListNode::new(2));
        let n_1_2 = Box::from(ListNode {
            val: 1,
            next: Some(n_1_3),
        });
        let n_1_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_1_2),
        });
        let res = Solution::delete_duplicates(Some(n_1_1));

        let n_2_2 = Box::from(ListNode::new(2));
        let n_2_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_2_2),
        });
        assert_eq!(res, Some(n_2_1));
    }

    #[test]
    fn example_leetcode_2() {
        let n_1_5 = Box::from(ListNode::new(3));
        let n_1_4 = Box::from(ListNode {
            val: 3,
            next: Some(n_1_5),
        });
        let n_1_3 = Box::from(ListNode {
            val: 2,
            next: Some(n_1_4),
        });
        let n_1_2 = Box::from(ListNode {
            val: 1,
            next: Some(n_1_3),
        });
        let n_1_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_1_2),
        });
        let res = Solution::delete_duplicates(Some(n_1_1));

        let n_2_3 = Box::from(ListNode::new(3));
        let n_2_2 = Box::from(ListNode {
            val: 2,
            next: Some(n_2_3),
        });
        let n_2_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_2_2),
        });
        assert_eq!(res, Some(n_2_1));
    }

    #[test]
    fn example_leetcode_3() {
        let res = Solution::delete_duplicates(None);
        assert_eq!(res, None);
    }
}
