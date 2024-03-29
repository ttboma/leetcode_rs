use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [876. Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)
    ///
    /// Given the `head` of a singly linked list, return the middle node of the linked list.
    ///
    /// If there are two middle nodes, return **the second middle**  node.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-midlist1.jpg" style="width: 544px; height: 65px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4,5]
    /// Output: [3,4,5]
    /// Explanation: The middle node of the list is node 3.
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-midlist2.jpg" style="width: 664px; height: 65px;">
    ///
    /// ```txt
    /// Input: head = [1,2,3,4,5,6]
    /// Output: [4,5,6]
    /// Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in the list is in the range `[1, 100]`.
    /// - `1 <= Node.val <= 100`
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow_p = &head;
        let mut fast_p = &head;
        while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
            slow_p = &slow_p.as_ref().unwrap().next;
            fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow_p.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let head = singly_linked_list![1, 2, 3, 4, 5].head;
        let expected = singly_linked_list![3, 4, 5].head;
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn example2() {
        let head = singly_linked_list![1, 2, 3, 4, 5, 6].head;
        let expected = singly_linked_list![4, 5, 6].head;
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn example3() {
        let head = singly_linked_list![1].head;
        let expected = singly_linked_list![1].head;
        assert_eq!(Solution::middle_node(head), expected);
    }

    #[test]
    fn example4() {
        let head = singly_linked_list![2, 1].head;
        let expected = singly_linked_list![1].head;
        assert_eq!(Solution::middle_node(head), expected);
    }
}
