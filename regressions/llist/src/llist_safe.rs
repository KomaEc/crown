use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
// A linked list node

#[repr(C)]
pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>,
}
impl Default for Node {
    fn default() -> Self {
        Self {
            data: Default::default(),
            next: None,
        }
    }
}

/* Given a reference (pointer to pointer) to the head of a list and
an int, inserts a new node on the front of the list. */
pub fn push(mut head_ref: Option<&mut Option<Box<Node>>>, mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node = Some(Box::new(<Node as Default>::default()));
    /* 2. put in the data  */
    (*new_node.as_deref_mut().unwrap()).data = new_data;
    /* 3. Make next of new node as head */
    (*new_node.as_deref_mut().unwrap()).next = (*head_ref.as_deref_mut().unwrap()).take();
    /* 4. move the head to point to the new node */
    *head_ref.as_deref_mut().unwrap() = new_node;
}
/* Given a node prev_node, insert a new node after the given
prev_node */
pub fn insertAfter(mut prev_node: Option<&mut Node>, mut new_data: i32) {
    /*1. check if the given prev_node is NULL */
    if prev_node.as_deref().is_none() {
        unsafe {
            printf(
                b"the given previous node cannot be NULL\x00" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    /* 2. allocate new node */
    let mut new_node = Some(Box::new(<Node as Default>::default()));
    /* 3. put in the data  */
    (*new_node.as_deref_mut().unwrap()).data = new_data;
    /* 4. Make next of new node as next of prev_node */
    (*new_node.as_deref_mut().unwrap()).next = (*prev_node.as_deref_mut().unwrap()).next.take();
    /* 5. move the next of prev_node as new_node */
    (*prev_node.as_deref_mut().unwrap()).next = new_node;
}
/* Given a reference (pointer to pointer) to the head
of a list and an int, appends a new node at the end  */
pub fn append(mut head_ref: Option<&mut Option<Box<Node>>>, mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node = Some(Box::new(<Node as Default>::default())); /* used in step 5*/
    let mut last = unsafe {
        core::mem::transmute::<_, *mut Node>((*head_ref.as_deref_mut().unwrap()).as_deref_mut())
    };
    /* 2. put in the data  */
    (*new_node.as_deref_mut().unwrap()).data = new_data;
    /* 3. This new node is going to be the last node, so make next of
    it as NULL*/
    (*new_node.as_deref_mut().unwrap()).next = None;
    /* 4. If the Linked List is empty, then make the new node as head */
    if (*head_ref.as_deref().unwrap()).as_deref().is_none() {
        *head_ref.as_deref_mut().unwrap() = new_node;
        return;
    }
    // let mut last = *head_ref;
    // /* 5. Else traverse till the last node */
    unsafe {
        while !(*last).next.as_deref().is_none() {
            last = core::mem::transmute((*last).next.as_deref_mut())
        }
        /* 6. Change the next of last node */
        (*last).next = new_node;
    }
}

// unsafe fn ub() {
//     let owning = Box::new(1);
//     let raw: *mut i32 = &mut *owning;
//     let another_owning = owning;
//     *raw = 2;
// }

pub fn deleteFirst(mut head_ref: Option<&mut Option<Box<Node>>>) {
    if !(*head_ref.as_deref().unwrap()).as_deref().is_none() {
        // store the old value of pointer to head pointer
        let mut temp = (*head_ref.as_deref_mut().unwrap()).take();

        // Change head pointer to point to next node
        *head_ref.as_deref_mut().unwrap() = (*temp.as_deref_mut().unwrap()).next.take();
        // (**head_ref).next;

        // delete memory allocated for the previous head node
        //  free(temp as *mut libc::c_void);
        let _ = temp;
    } else {
    }
}


pub fn reverse(mut list: Option<&mut List>) {
    let mut current = (*list.as_deref_mut().unwrap()).head.take();
    let mut reversed = None;
    let mut temp = None;
    while !current.as_deref().is_none() {
        temp = current;
        current = (*temp.as_deref_mut().unwrap()).next.take();
        (*temp.as_deref_mut().unwrap()).next = reversed;
        reversed = temp;
    }
    
    (*list.as_deref_mut().unwrap()).head = reversed;
}

// pub unsafe fn test_list(mut list: *mut Node) -> *mut Node {
//     // let mut list = core::ptr::null_mut();
//     let mut list = list;
//     append(core::ptr::addr_of_mut!(list), 1);
//     append(core::ptr::addr_of_mut!(list), 2);
//     append(core::ptr::addr_of_mut!(list), 3);
//     append(core::ptr::addr_of_mut!(list), 4);

//     deleteFirst(core::ptr::addr_of_mut!(list));
//     deleteFirst(core::ptr::addr_of_mut!(list));
//     deleteFirst(core::ptr::addr_of_mut!(list));
//     deleteFirst(core::ptr::addr_of_mut!(list));

//     list
// }

// pub unsafe fn bad(a: *mut Node, y: *mut Node) -> *mut Node {
//     let mut x = a;
//     let mut u = (*y).next;
//     free(u as *mut _);
//     (*x).next = y;
//     let mut z = (*x).next;
//     let mut w = (*z).next;
//     free(w as *mut _);
//     free(z as *mut _);
//     return x;
// }

// pub unsafe fn delete_n(y: *mut Node, n: u32) -> *mut Node {
//     if n == 1 {
//         let u = (*y).next;
//         (*y).next = 0 as *mut _;
//         free(u as *mut _);
//         return y;
//     } else {
//         (*y).next = delete_n((*y).next, n - 1);
//         return y;
//     }
// }

// pub unsafe fn bad(mut y: *mut Node) {
//     y = delete_n(y, 1);
//     y = delete_n(y, 1);
//     free(y as *mut _);
// }

// pub unsafe fn delete_next(prev_node: *mut Node) {
//     let mut del = (*prev_node).next;
//     let mut next = (*del).next;
//     (*prev_node).next = next;
//     free(del as *mut _);
// }
