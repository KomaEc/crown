# Why Pointer Analysis is Necessary in our Current Model

In our current ownership model, we must have pointer analysis. There are two reasons.

#### SSA is Incorrect without Pointer Analysis

SSA: track exact def-use chain for each variable

SSA without Pointer Analysis: missing def-use relation because of aliasing
```C
r = &a; // def of r
p = &r;
*p = &b; // def of r
use(r); // use of r
```



For function without explicit `&` or dereference of multi-level pointers, our algorithm is ok.



#### `p = q`

A statement like `p = q` should introduce two constraints:

1. ownership transfer,  $\rho_p = 0 \land \rho_q = \rho_{p^\prime} + \rho_{q^\prime}$
2. Alias constraints, any pointers reachable from `p` and `q` must behave the same onwards.

Tracking 2 is hard. 

It is possible to infer ownership values and ownership transfer relations among `*p`s (inner pointers). But problems occur when the base pointer (outter pointer) is used or defined.

For program like

```c
p = q; // p is a multi-level pointer
free(*p); // we must know that *q is freed as well!
```

we must emit an ownership transfer for outter pointers on line 1, where `p` and `q` are re-defined to `p'` and `q'`, respectively. A correct alias constraint will require that the two program expressions `*p'` and `*q'` behaves equivalently onwards. This means on line 2, where a pointer value `*p` is defined (note that in ownership analysis, using of pointer implies defining a pointer), we must also know that `*q` is defined as well. However this is impossible from syntax-level. We will have to use results from a seperate alias analysis.



### Stricter Ownership Model

Inspired by Rust's treatment to pointer dereferencing, we proposed a stricter ownership model.

#### Differences between C and Rust

1. `*p = ...` will drop the value pointed to by `p` in Rust (which incurs a lot of memory bugs in Rust [PLDI2020]). This is not the case in C.
2. `... = *p` it is not allowed to move value out of a reference in Rust, but is allowed in C.

If `p` is a `Box` pointer in Rust, rustc will be able to track ownership information of the path `*p`.

This means that the ownership of `*p` is _dominated_ by that of `p`.



#### New Model

We choose those C libs that have __ownership dominance__ property: that for a multi-level pointer `p`, `*p` is owning implies that `p` is owning.

Advantages of this new model:

1. It is closer to Rust's semantics.
2. Ownership of base pointers (outter pointers) provides __strong separation__, therefore the analyser needs not emit alias constraints



#### Constraints

We consider a language with six kinds of statements

```c
p = malloc(..);       // allocation
free(p);              // deallocation
p = &a;               // address taking
p = q;                // assignment
*p = q;               // store
p = *q;               // load
```

The constraints generated from those statements are showed below

| Statement Kind   | Constraints                                                  |
| ---------------- | ------------------------------------------------------------ |
| `p = &a`         | $\rho_p = 0$                                                 |
| `p = q`          | $\rho_p = 0 \land \rho_{p^\prime} + \rho_{q^\prime} = \rho_q$ |
| `*p = q`         | $\rho_q \le \rho_p \land \rho_{*p} = 0 \land \rho_{(*p)^\prime} + \rho_{q^\prime} = \rho_q$ |
| `p = *q`         | $\rho_{*q} \le \rho_q \land \rho_p = 0 \land \rho_{p^\prime} + \rho_{(*q)^\prime} = \rho_{*q}$ |
| `p = malloc(..)` | $\rho_p = 1$                                                 |
| `free(p)`        | $\rho_p = 1$                                                 |

The additional inequality constraints appear in load and store statements encode ownership dominance property, that if the pointer to be dereferenced is non-owning, everything should be non-owning.





#### Algorithm

Perform ownership analysis _outside in_. We first infer for top-level pointers to define an approximate ownership scheme. Then we track pointers behind one-level of dereference. If the base pointer is owning, then the inner pointer is allow to transfer ownership. Otherwise, the inner pointer can only be transient. Then two-level, etc. until a fixpoint.

As a really contrived example, the program

```c
p = q; // ownership transfer
free(*q); // q is non-owning, *q cannot be owning! ERROR
free(p); // p is owning
```

should be rejected by the analyser.

The first round of the analysis will be performed on the reduced program

```c
p = q;
free(p);
```

which identifies the ownership transfer relation.

The second round of the analysis will be performed on the original program, where an error is discovered.



#### Workaround with head references

It is a very common pattern in C libs that a reference to some owning objects is passed into a function. For example

```rust
#[repr(C)]#[derive(Copy, Clone)]
pub struct Node {
    pub data: i32,
    pub next: *mut Node,
}
/* Given a reference (pointer to pointer) to the head of a list and
   an int, inserts a new node on the front of the list. */
pub unsafe extern "C" fn push(mut head_ref: *mut *mut Node,
                              mut new_data: i32) {
    /* 1. allocate node */
    let mut new_node: *mut Node =
        malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
    /* 2. put in the data  */
    (*new_node).data = new_data;
    /* 3. Make next of new node as head */
    (*new_node).next = *head_ref;
    /* 4. move the head to point to the new node */
    *head_ref = new_node;
}
```

Since the new model requires ownership dominance property, those libs are directly rejected. 

My observation is that, in the above `push` function, the top-level reference `head_ref` is never re-defined. Therefore, it can be treated as the `this` pointer in PLDI03. 

To work around with this kind of pattern, we allow ownership transfer inside transient multi-level pointers, before such a pointer is redefined.

```c
*p = _; // *p is tracked, regardless of the ownership value of p
...
_ = *p; // *p is tracked, regardless of the ownership value of p
...
p = q; // p is redefined
...
*p = _; // *p is tracked only if p is owning
```

Note that before re-definition, base pointers have certain separation or uniqueness property, that at least it does not alias other pointers in a method.