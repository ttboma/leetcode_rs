use crate::ListNode;
use crate::Solution;

impl Solution {
    /// # [21. Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)
    ///
    /// You are given the heads of two sorted linked lists `list1` and `list2`.
    ///
    /// Merge the two lists into one **sorted**  list. The list should be made by splicing together the nodes of the first two lists.
    ///
    /// Return the head of the merged linked list.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg" style="width: 662px; height: 302px;">
    ///
    /// ```txt
    /// Input: list1 = [1,2,4], list2 = [1,3,4]
    /// Output: [1,1,2,3,4,4]
    /// ```
    ///
    /// **Example 2:**
    ///
    /// ```txt
    /// Input: list1 = [], list2 = []
    /// Output: []
    /// ```
    ///
    /// **Example 3:**
    ///
    /// ```txt
    /// Input: list1 = [], list2 = [0]
    /// Output: [0]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - The number of nodes in both lists is in the range `[0, 50]`.
    /// - `-100 <= Node.val <= 100`
    /// - Both `list1` and `list2` are sorted in **non-decreasing**  order.
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let (mut list1, mut list2) = if list2.as_ref().unwrap().val < list1.as_ref().unwrap().val {
            (list2, list1)
        } else {
            (list1, list2)
        };

        let mut i: &mut ListNode = list1.as_mut().unwrap();

        while let Some(ref mut v) = list2 {
            while !(i.next.is_none() || i.next.as_mut().unwrap().val > v.val) {
                i = i.next.as_mut().unwrap();
            }
            std::mem::swap(&mut i.next, &mut list2);
        }

        list1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{singly_linked_list, SinglyLinkedList};

    #[test]
    fn example1() {
        let list1 = singly_linked_list![1, 2, 4].head;
        let list2 = singly_linked_list![1, 3, 4].head;
        let list3 = singly_linked_list![1, 1, 2, 3, 4, 4].head;
        assert_eq!(Solution::merge_two_lists(list1, list2), list3);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }
    #[test]
    fn example3() {
        let list2 = singly_linked_list![0].head;
        let list3 = singly_linked_list![0].head;
        assert_eq!(Solution::merge_two_lists(None, list2), list3);
    }
}
