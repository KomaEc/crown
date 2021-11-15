// A linked list node

pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}
/* Given a reference (pointer to pointer) to the head of a list and
   an int, inserts a new node on the front of the list. */
pub fn push(mut head_ref: &mut Option<Box<Node>>,
            mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node = Node { data: new_data, next: std::mem::take(head_ref) };
    /* 2. put in the data  */
    /* 3. Make next of new node as head */
    /* 4. move the head to point to the new node */
    *head_ref = Some(Box::new(new_node));
}
/* Given a node prev_node, insert a new node after the given
   prev_node */
pub fn insertAfter(mut prev_node: &mut Option<Node>,
                   mut new_data: i32) {
    /*1. check if the given prev_node is NULL */
    match prev_node.as_mut() {
        None => {
            println!("the given revious node cannot be NULL");
        },
        Some(prev_node) => {
            let mut new_node = Node { data: new_data, next: std::mem::take(&mut prev_node.next) };
            prev_node.next = Some(Box::new(new_node));
        }
    }
    /* 2. allocate new node */
    /* 3. put in the data  */
    /* 4. Make next of new node as next of prev_node */
    /* 5. move the next of prev_node as new_node */
}
/* Given a reference (pointer to pointer) to the head
   of a list and an int, appends a new node at the end  */
pub fn append(mut head_ref: &mut Option<Box<Node>>,
              mut new_data: i32) {
    /* 1. allocate node */
    let new_node = Node { data: new_data, next: None };
    /* 2. put in the data  */
    /* 3. This new node is going to be the last node, so make next of
       it as NULL*/
    match head_ref.as_mut() {
        None => {
            *head_ref = Some(Box::new(new_node));
        }
        Some(mut last) => {
            loop {
                match last.next {
                    Some(ref mut node) => {
                        last = node;
                    },
                    None => {
                        last.next = Some(Box::new(new_node));
                        break;
                    }
                }
            }
        }
    }
    /* 4. If the Linked List is empty, then make the new node as head */
    /* 5. Else traverse till the last node */
    /* 6. Change the next of last node */
}
