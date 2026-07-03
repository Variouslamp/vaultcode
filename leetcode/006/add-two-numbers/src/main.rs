fn main() {
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

    // SOLUTION

    struct Solution {} // para que no me error
    impl Solution {
        pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut l1 = l1;
            let mut l2 = l2;
            let mut dummy: ListNode = ListNode::new(0);
            let mut new_node: &mut ListNode = &mut dummy;
            let mut carry: i32 = 0;
            while l1.is_some() || l2.is_some() || carry == 1 {
                let mut sum = carry;

                if let Some(nodo) = l1.take() {
                    sum += nodo.val;
                    l1 = nodo.next
                }
                if let Some(nodo) = l2.take() {
                    sum += nodo.val;
                    l2 = nodo.next
                }
                carry = sum / 10;
                sum = sum % 10;
                let siguiente = Some(Box::new(ListNode::new(sum)));
                new_node.next = siguiente;
                new_node = new_node.next.as_mut().unwrap();
            }
            return dummy.next
        }
    }
}
