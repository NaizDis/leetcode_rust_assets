use std::process::Output;

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
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut h = head;
    let mut p1 = h.as_mut().unwrap();
    while let Some(p2) = p1.next.as_mut() {
        if p1.val == p2.val {
            p1.next = p2.next.take();
        } else {
            p1 = p1.next.as_mut().unwrap();
        }
    }
    h
}
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);
    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}
pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(l1), Some(l2)) => {
            if l1.val >= l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                }))
            }
        }
    }
}
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur_node = head.as_ref();
    let mut mid_node = head.as_ref();

    while let Some(current) = cur_node {
        cur_node = current.next.as_ref().and_then(|next| {
            mid_node = mid_node.and_then(|node| node.next.as_ref());
            next.next.as_ref()
        });
    }
    mid_node.cloned()
}
