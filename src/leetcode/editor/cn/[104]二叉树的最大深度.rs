//给定一个二叉树 root ，返回其最大深度。
//
// 二叉树的 最大深度 是指从根节点到最远叶子节点的最长路径上的节点数。
//
//
//
// 示例 1：
//
//
//
//
//
//
//输入：root = [3,9,20,null,null,15,7]
//输出：3
//
//
// 示例 2：
//
//
//输入：root = [1,null,2]
//输出：2
//
//
//
//
// 提示：
//
//
// 树中节点的数量在 [0, 10⁴] 区间内。
// -100 <= Node.val <= 100
//
//
// Related Topics 树 深度优先搜索 广度优先搜索 二叉树 👍 2074 👎 0

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        traverse(&root, 0, &mut max_depth);
        max_depth
    }
}

pub fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, curr_depth: i32, max_depth: &mut i32) {
    if root.is_none() {
        if *max_depth < curr_depth {
            *max_depth = curr_depth;
        }
    } else {
        let node = root.as_ref().unwrap().borrow();
        traverse(&node.left, curr_depth + 1, max_depth);
        traverse(&node.right, curr_depth + 1, max_depth);
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
        let node_3_4 = TreeNode::new(7);
        let node_3_3 = TreeNode::new(15);
        let node_2_2 = TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(node_3_3))),
            right: Some(Rc::new(RefCell::new(node_3_4))),
        };
        let node_2_1 = TreeNode {
            val: 9,
            left: None,
            right: None,
        };
        let node_1_1 = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(node_2_1))),
            right: Some(Rc::new(RefCell::new(node_2_2))),
        };
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(node_1_1)))),
            3
        );
    }

    #[test]
    fn example_leetcode_2() {
        let node_2_2 = TreeNode::new(2);
        let node_1_1 = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(node_2_2))),
        };
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(node_1_1)))),
            2
        );
    }
}
