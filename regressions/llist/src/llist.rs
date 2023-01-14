use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
// A linked list node

#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}
impl Default for Node {fn default() -> Self {Self {
data: Default::default(),
next: None,
}}}
impl Node {pub fn take(&mut self) -> Self {core::mem::take(self)}}


/* Given a reference (pointer to pointer) to the head of a list and
an int, inserts a new node on the front of the list. */
pub unsafe extern "C" fn push(mut head_ref: Option<&mut Option<Box<Node>>>, mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node = Some(Box::new(<crate::src::llist::Node as Default>::default()));
    /* 2. put in the data  */
    (*new_node.as_deref_mut().unwrap()).data= new_data;
    /* 3. Make next of new node as head */
    (*new_node.as_deref_mut().unwrap()).next= (*head_ref.as_deref_mut().unwrap()).take();
    /* 4. move the head to point to the new node */
    *head_ref.as_deref_mut().unwrap()= new_node;
}
/* Given a node prev_node, insert a new node after the given
prev_node */
pub unsafe extern "C" fn insertAfter(mut prev_node: Option<&mut Node>, mut new_data: i32) {
    /*1. check if the given prev_node is NULL */
    if prev_node.as_deref().is_none() {
        ();
        printf(b"the given previous node cannot be NULL\x00" as *const u8 as *const libc::c_char);
        return;
    }
    /* 2. allocate new node */
    let mut new_node = Some(Box::new(<crate::src::llist::Node as Default>::default()));
    /* 3. put in the data  */
    (*new_node.as_deref_mut().unwrap()).data= new_data;
    /* 4. Make next of new node as next of prev_node */
    (*new_node.as_deref_mut().unwrap()).next= (*prev_node.as_deref_mut().unwrap()).next.take();
    /* 5. move the next of prev_node as new_node */
    (*prev_node.as_deref_mut().unwrap()).next= new_node;
}
/* Given a reference (pointer to pointer) to the head
of a list and an int, appends a new node at the end  */
pub unsafe extern "C" fn append(mut head_ref: Option<&mut *mut Node>, mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node = Some(Box::new(<crate::src::llist::Node as Default>::default())); /* used in step 5*/
    let mut last = (*head_ref.as_deref().unwrap());
    /* 2. put in the data  */
    (*new_node.as_deref_mut().unwrap()).data= new_data;
    /* 3. This new node is going to be the last node, so make next of
    it as NULL*/
    (*new_node.as_deref_mut().unwrap()).next= None;
    /* 4. If the Linked List is empty, then make the new node as head */
    if (*head_ref.as_deref().unwrap()).is_null() {
        ();
        *head_ref.as_deref_mut().unwrap()= core::mem::transmute::<_, *mut crate::src::llist::Node>(new_node.as_deref_mut());
        return;
    }
    // let mut last = *head_ref;
    // /* 5. Else traverse till the last node */
    while !(*last).next.as_deref().is_none() {
        last= core::mem::transmute::<_, *mut crate::src::llist::Node>((*last).next.as_deref_mut())
    }
    // std::intrinsics::assume((*last).next as usize == 0);
    // /* 6. Change the next of last node */
    (*last).next= new_node;
}

pub unsafe extern "C" fn deleteFirst(mut head_ref: Option<&mut Option<Box<Node>>>) {
    if !(*head_ref.as_deref().unwrap()).as_deref().is_none() {
        // store the old value of pointer to head pointer
        let mut temp = (*head_ref.as_deref_mut().unwrap()).take();

        // Change head pointer to point to next node
        *head_ref.as_deref_mut().unwrap()= 
        (*temp.as_deref_mut().unwrap()).next.take();
        // (**head_ref).next;

        // delete memory allocated for the previous head node
        ();
    } else {
        ();
    }
}

pub unsafe fn test_list(mut list: Option<&mut Option<Box<Node>>>) {
    // let mut list = core::ptr::null_mut();
    append(Some(&mut (*list.as_deref_mut().unwrap())), 1);
    append(Some(&mut (*list.as_deref_mut().unwrap())), 2);
    append(Some(&mut (*list.as_deref_mut().unwrap())), 3);
    append(Some(&mut (*list.as_deref_mut().unwrap())), 4);

    deleteFirst(Some(&mut (*list.as_deref_mut().unwrap())));
    deleteFirst(Some(&mut (*list.as_deref_mut().unwrap())));
    deleteFirst(Some(&mut (*list.as_deref_mut().unwrap())));
    deleteFirst(Some(&mut (*list.as_deref_mut().unwrap())));

}

pub unsafe fn newNode2() -> Option<Box<Node>> {
    let mut new_node = Some(Box::new(<crate::src::llist::Node as Default>::default()));
    let mut next = Some(Box::new(<crate::src::llist::Node as Default>::default()));

    (*next.as_deref_mut().unwrap()).data= 3;
    (*next.as_deref_mut().unwrap()).next= None;

    (*new_node.as_deref_mut().unwrap()).data= 2;
    (*new_node.as_deref_mut().unwrap()).next= next;

    return new_node
}
