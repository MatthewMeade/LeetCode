// 2. Add Two Numbers
// Solution: https://leetcode.com/submissions/detail/1293621649/

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut cur1 = l1;
    let mut cur2 = l2;

    let mut head = ListNode::new(0);
    let mut cur_out = &mut head;

    while cur1.is_some() || cur2.is_some() {
        let cur1_val = match &cur1 {
            Some(ref n) => n.val,
            _ => 0,
        };

        let cur2_val = match &cur2 {
            Some(ref n) => n.val,
            _ => 0,
        };

        let total = cur1_val + cur2_val + cur_out.val;
        let result = total % 10;
        let carry = (total - result) / 10;

        cur_out.val = result;

        cur1 = match cur1 {
            Some(n) => n.next,
            None => None,
        };

        cur2 = match cur2 {
            Some(n) => n.next,
            None => None,
        };

        if cur1.is_some() || cur2.is_some() || carry > 0 {
            cur_out.next = Some(Box::new(ListNode::new(carry)));
            if let Some(ref mut next_node) = cur_out.next {
                cur_out = next_node;
            }
        }
    }

    return Some(Box::new(head));
}

pub fn main() {
    let mut h1 = ListNode::new(1);
    h1.next = Some(Box::new(ListNode::new(1)));

    let mut h2 = ListNode::new(9);
    h2.next = Some(Box::new(ListNode::new(9)));

    let result = add_two_numbers(Some(Box::new(h1)), Some(Box::new(h2)));
    println!("The result is:");

    print_list(&result);
}

fn print_list(head: &Option<Box<ListNode>>) {
    let mut cur = head.as_ref().map(|boxed_node| &**boxed_node);
    while let Some(node) = cur {
        print!("{} ", node.val);
        cur = node.next.as_ref().map(|boxed_node| &**boxed_node);
    }
    println!();
}

// Definition for singly-linked list.
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
