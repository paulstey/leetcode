// Solution based on that of Minh Tuan Tran

use add_two_numbers::ListNode;


fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_lists(l1, l2, 0)
}

// Recursively step through nodes of linked lists.
fn add_lists(
    node1: Option<Box<ListNode>>,
    node2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {

    // Our "base case" when we have consumed all the nodes in both lists
    if node1.is_none() && node2.is_none() && carry == 0 {
        return None;
    }

    let (val1, next1) = unwrap_node(node1);
    let (val2, next2) = unwrap_node(node2);

    let sum = carry + val1 + val2;
    let next_node = add_lists(next1, next2, sum / 10);
    let sum_node = ListNode {
        val: sum % 10,
        next: next_node,
    };

    Some(Box::new(sum_node))
}

#[inline]
fn unwrap_node(node: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
    match node {
        Some(b) => (b.val, b.next),
        None => (0, None),
    }
}

fn main() {
    ()
}
