//给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。
//
// 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
//
//
//
// 示例 1：
//
//
//输入：p = [1,2,3], q = [1,2,3]
//输出：true
//
//
// 示例 2：
//
//
//输入：p = [1,2], q = [1,null,2]
//输出：false
//
//
// 示例 3：
//
//
//输入：p = [1,2,1], q = [1,1,2]
//输出：false
//
//
//
//
// 提示：
//
//
// 两棵树上的节点数目都在范围 [0, 100] 内
// -10⁴ <= Node.val <= 10⁴
//
//
// Related Topics 树 深度优先搜索 广度优先搜索 二叉树 👍 1276 👎 0

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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let (mut vec_1, mut vec_2) = (vec![], vec![]);
        traversal(&mut vec_1, &p);
        traversal(&mut vec_2, &q);
        vec_1.eq(&vec_2)
    }
}

pub fn traversal(vec: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
    if node.is_some() {
        let a = node.as_ref().unwrap().borrow();
        vec.push(a.val);
        traversal(vec, &a.left);
        traversal(vec, &a.right);
    } else {
        vec.push(-999);
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
        let node_1_3 = TreeNode::new(3);
        let node_1_2 = TreeNode::new(2);
        let node_1_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_1_2))),
            right: Some(Rc::new(RefCell::new(node_1_3))),
        };
        let node_2_3 = TreeNode::new(3);
        let node_2_2 = TreeNode::new(2);
        let node_2_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_2_2))),
            right: Some(Rc::new(RefCell::new(node_2_3))),
        };
        let b = Solution::is_same_tree(
            Some(Rc::new(RefCell::new(node_1_1))),
            Some(Rc::new(RefCell::new(node_2_1))),
        );
        assert_eq!(b, true);
    }

    #[test]
    fn example_leetcode_2() {
        let node_1_2 = TreeNode::new(2);
        let node_1_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_1_2))),
            right: None,
        };
        let node_2_2 = TreeNode::new(2);
        let node_2_1 = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(node_2_2))),
        };
        let b = Solution::is_same_tree(
            Some(Rc::new(RefCell::new(node_1_1))),
            Some(Rc::new(RefCell::new(node_2_1))),
        );
        assert_eq!(b, false);
    }

    #[test]
    fn example_leetcode_3() {
        let node_1_3 = TreeNode::new(1);
        let node_1_2 = TreeNode::new(2);
        let node_1_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_1_2))),
            right: Some(Rc::new(RefCell::new(node_1_3))),
        };
        let node_2_3 = TreeNode::new(2);
        let node_2_2 = TreeNode::new(1);
        let node_2_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_2_2))),
            right: Some(Rc::new(RefCell::new(node_2_3))),
        };
        let b = Solution::is_same_tree(
            Some(Rc::new(RefCell::new(node_1_1))),
            Some(Rc::new(RefCell::new(node_2_1))),
        );
        assert_eq!(b, false);
    }

    #[test]
    fn example_leetcode_4() {
        let node_1_2 = TreeNode::new(0);
        let node_1_1 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(node_1_2))),
            right: None,
        };
        let node_2_2 = TreeNode::new(0);
        let node_2_1 = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(node_2_2))),
        };
        let b = Solution::is_same_tree(
            Some(Rc::new(RefCell::new(node_1_1))),
            Some(Rc::new(RefCell::new(node_2_1))),
        );
        assert_eq!(b, false);
    }
}
