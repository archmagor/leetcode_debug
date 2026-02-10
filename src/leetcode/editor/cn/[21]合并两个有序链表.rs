//将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
//
//
//
// 示例 1：
//
//
//输入：l1 = [1,2,4], l2 = [1,3,4]
//输出：[1,1,2,3,4,4]
//
//
// 示例 2：
//
//
//输入：l1 = [], l2 = []
//输出：[]
//
//
// 示例 3：
//
//
//输入：l1 = [], l2 = [0]
//输出：[0]
//
//
//
//
// 提示：
//
//
// 两个链表的节点数目范围是 [0, 50]
// -100 <= Node.val <= 100
// l1 和 l2 均按 非递减顺序 排列
//
//
// Related Topics 递归 链表 👍 3955 👎 0

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
// }
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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        while let (Some(x_1), Some(x_2)) = (&list1, &list2) {
            if x_1.val < x_2.val {
                curr.next = list1.take();
                curr = curr.next.as_mut()?;
                list1 = curr.next.take();
            } else {
                curr.next = list2.take();
                curr = curr.next.as_mut()?;
                list2 = curr.next.take();
            }
        }

        curr.next = list1.or(list2);

        dummy.next
    }
}

//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    #[test]
    fn example_leetcode() {
        let n_1_4 = Box::from(ListNode { val: 4, next: None });
        let n_1_2 = Box::from(ListNode {
            val: 2,
            next: Some(n_1_4),
        });
        let n_1_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_1_2),
        });
        let n_2_4 = Box::from(ListNode { val: 4, next: None });
        let n_2_2 = Box::from(ListNode {
            val: 3,
            next: Some(n_2_4),
        });
        let n_2_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_2_2),
        });
        let res = Solution::merge_two_lists(Some(n_1_1), Some(n_2_1));

        let n_3_4_1 = Box::from(ListNode { val: 4, next: None });
        let n_3_4 = Box::from(ListNode {
            val: 4,
            next: Some(n_3_4_1),
        });
        let n_3_3 = Box::from(ListNode {
            val: 3,
            next: Some(n_3_4),
        });
        let n_3_2 = Box::from(ListNode {
            val: 2,
            next: Some(n_3_3),
        });
        let n_3_1_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_3_2),
        });
        let n_3_1 = Box::from(ListNode {
            val: 1,
            next: Some(n_3_1_1),
        });
        assert_eq!(res, Some(n_3_1));
    }

    #[test]
    fn example_leetcode_2() {
        let res = Solution::merge_two_lists(None, None);
        assert_eq!(res, None);
    }

    #[test]
    fn example_leetcode_3() {
        let n_1 = Box::from(ListNode { val: 0, next: None });
        let res = Solution::merge_two_lists(None, Some(n_1));

        let n_2 = Box::from(ListNode { val: 0, next: None });
        assert_eq!(res, Some(n_2));
    }
}
