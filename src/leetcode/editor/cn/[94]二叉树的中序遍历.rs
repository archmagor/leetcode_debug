//给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。
//
//
//
// 示例 1：
//
//
//输入：root = [1,null,2,3]
//输出：[1,3,2]
//
//
// 示例 2：
//
//
//输入：root = []
//输出：[]
//
//
// 示例 3：
//
//
//输入：root = [1]
//输出：[1]
//
//
//
//
// 提示：
//
//
// 树中节点数目在范围 [0, 100] 内
// -100 <= Node.val <= 100
//
//
//
//
// 进阶: 递归算法很简单，你可以通过迭代算法完成吗？
//
// Related Topics 栈 树 深度优先搜索 二叉树 👍 2380 👎 0

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec = vec![];
        traversal(&mut vec, &root);
        vec
    }
}

pub fn traversal(vec: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
    if node.is_some() {
        let a = node.as_ref().unwrap().borrow();
        if a.left != None {
            traversal(vec, &a.left);
        }
        vec.push(a.val);
        if a.right != None {
            traversal(vec, &a.right);
        }
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
        let node_3 = TreeNode::new(3);
        let node_2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(node_3))),
            right: None,
        };
        let node_1 = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(node_2))),
        };
        let vec = Solution::inorder_traversal(Some(Rc::new(RefCell::new(node_1))));
        assert_eq!(vec, vec![1, 3, 2]);
    }

    #[test]
    fn example_leetcode_2() {
        let vec = Solution::inorder_traversal(None);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn example_leetcode_3() {
        let node_1 = TreeNode::new(1);
        let vec = Solution::inorder_traversal(Some(Rc::new(RefCell::new(node_1))));
        assert_eq!(vec, vec![1]);
    }
}
