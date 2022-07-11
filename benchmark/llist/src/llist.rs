use ::libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
// A linked list node

#[repr(C)]
pub struct Node {
    pub data: i32,
    pub next: *mut Node,
}
/* Given a reference (pointer to pointer) to the head of a list and
   an int, inserts a new node on the front of the list. */
pub unsafe extern "C" fn push(mut head_ref: *mut *mut Node,
                              mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node =
        malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
    /* 2. put in the data  */
    (*new_node).data = new_data;
    /* 3. Make next of new node as head */
    (*new_node).next = *head_ref;
    /* 4. move the head to point to the new node */
    *head_ref = new_node;
}
/* Given a node prev_node, insert a new node after the given
   prev_node */
pub unsafe extern "C" fn insertAfter(mut prev_node: *mut Node,
                                     mut new_data: i32) {
    /*1. check if the given prev_node is NULL */
    if prev_node.is_null() {
        printf(b"the given previous node cannot be NULL\x00" as *const u8 as
                   *const libc::c_char);
        return
    }
    /* 2. allocate new node */
    let mut new_node =
        malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
    /* 3. put in the data  */
    (*new_node).data = new_data;
    /* 4. Make next of new node as next of prev_node */
    (*new_node).next = (*prev_node).next;
    /* 5. move the next of prev_node as new_node */
    (*prev_node).next = new_node;
}
/* Given a reference (pointer to pointer) to the head
   of a list and an int, appends a new node at the end  */
pub unsafe extern "C" fn append(mut head_ref: *mut *mut Node,
                                mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node =
        malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as
            *mut Node; /* used in step 5*/
    //let mut last = *head_ref;
    /* 2. put in the data  */
    (*new_node).data = new_data;
    /* 3. This new node is going to be the last node, so make next of
          it as NULL*/
    (*new_node).next = 0 as *mut Node;
    /* 4. If the Linked List is empty, then make the new node as head */
    if (*head_ref).is_null() { *head_ref = new_node; return }
    let mut last = *head_ref;
    /* 5. Else traverse till the last node */
    while !(*last).next.is_null() { last = (*last).next }
    /* 6. Change the next of last node */
    (*last).next = new_node;
}
