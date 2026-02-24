//给你一个二叉树的根节点 root ， 检查它是否轴对称。
//
//
//
// 示例 1：
//
//
//输入：root = [1,2,2,3,4,4,3]
//输出：true
//
//
// 示例 2：
//
//
//输入：root = [1,2,2,null,3,null,3]
//输出：false
//
//
//
//
// 提示：
//
//
// 树中节点数目在范围 [1, 1000] 内
// -100 <= Node.val <= 100
//
//
//
//
// 进阶：你可以运用递归和迭代两种方法解决这个问题吗？
//
// Related Topics 树 深度优先搜索 广度优先搜索 二叉树 👍 3078 👎 0

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (mut vec_1, mut vec_2) = (vec![], vec![]);

        traverse(&mut vec_1, &root, true);
        traverse(&mut vec_2, &root, false);

        vec_1.eq(&vec_2)
    }
}

pub fn traverse(
    vec: &mut Vec<i32>,
    root: &Option<Rc<RefCell<TreeNode>>>,
    is_left_before_right: bool,
) {
    if root.is_none() {
        vec.push(-999);
    } else {
        let node = root.as_ref().unwrap().borrow();
        vec.push(node.val);

        traverse(
            vec,
            if is_left_before_right {
                &node.left
            } else {
                &node.right
            },
            is_left_before_right,
        );
        traverse(
            vec,
            if is_left_before_right {
                &node.right
            } else {
                &node.left
            },
            is_left_before_right,
        );
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example_leetcode() {
        let node_3_4 = TreeNode::new(3);
        let node_3_3 = TreeNode::new(4);
        let node_3_2 = TreeNode::new(4);
        let node_3_1 = TreeNode::new(3);
        let node_2_2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(node_3_3))),
            right: Some(Rc::new(RefCell::new(node_3_4))),
        };
        let node_2_1 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(node_3_1))),
            right: Some(Rc::new(RefCell::new(node_3_2))),
        };
        let node_1_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_2_1))),
            right: Some(Rc::new(RefCell::new(node_2_2))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(node_1_1)))),
            true
        );
    }

    #[test]
    fn example_leetcode_2() {
        let node_3_4 = TreeNode::new(3);
        let node_3_2 = TreeNode::new(3);
        let node_2_2 = TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(node_3_4))),
        };
        let node_2_1 = TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(node_3_2))),
        };
        let node_1_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_2_1))),
            right: Some(Rc::new(RefCell::new(node_2_2))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(node_1_1)))),
            false
        );
    }

    #[test]
    fn example_leetcode_3() {
        let node_3_3 = TreeNode::new(2);
        let node_3_1 = TreeNode::new(2);
        let node_2_2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(node_3_3))),
            right: None,
        };
        let node_2_1 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(node_3_1))),
            right: None,
        };
        let node_1_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_2_1))),
            right: Some(Rc::new(RefCell::new(node_2_2))),
        };
        assert_eq!(
            Solution::is_symmetric(Some(Rc::new(RefCell::new(node_1_1)))),
            false
        );
    }
}
