
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut current_index = 0;
        let mut current_list: Vec<i32> = vec![];
        let mut current = head;
        let mut result: Vec<i32> = vec![];
        while let Some(n) = current {
            current_list.push(n.val);
            if current_index == k - 1 {
                for v in current_list.into_iter().rev() {
                    result.push(v);
                }
                current_index = 0;
                current_list = vec![];
            } else {
                current_index += 1;
            }
            current = n.next;
        }
        for v in current_list.into_iter() {
            result.push(v);
        }
        Solution::create_list_node(result)
    }
    pub fn create_list_node(lists: Vec<i32>) -> Option<Box<ListNode>> {
        let mut target: Option<Box<ListNode>> = None;
        for val in lists.into_iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            if target.is_some() {
                node.next = target;
            }
            target = Some(node);
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::reverse_k_group(Solution::create_list_node(vec![1,2,3,4,5]), 2),
            Solution::create_list_node(vec![2,1,4,3,5])
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::reverse_k_group(Solution::create_list_node(vec![1,2,3,4,5]), 3),
            Solution::create_list_node(vec![3,2,1,4,5])
        )
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::reverse_k_group(Solution::create_list_node(vec![1,2]), 2),
            Solution::create_list_node(vec![2,1])
        )
    }
}
