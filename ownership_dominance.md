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

Scalability

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







#### 8.5

##### Iterative Algorithm ?

Iterative algorithm: expand pointers further in each round

```rust
// A utility function to right rotate subtree rooted with y
// See the diagram given above.
pub unsafe extern "C" fn rightRotate(mut y: *mut Node) -> *mut Node {
    let mut x: *mut Node = (*y).left;
    let mut T2: *mut Node = (*x).right;
    // Perform rotation
    (*y).left = T2;
    (*x).right = y;
    // Update heights
    (*y).height =
        max(height((*y).left), height((*y).right)) + 1 as i32;
    (*x).height =
        max(height((*x).left), height((*x).right)) + 1 as i32;
    // Return new root
    return x;
}
```

In the first round:

```rust
x = ...;
... = y;
return x;
```

In the second round, `*mut struct Node` is expanded into `*mut struct Node { left: *mut Node, right: *mut Node }`:

```rust
x = (*y).left;
T2 = (*x).right; // note that (*x).right has no connection to (*(*y).left).right in this round
(*y).left = T2
(*x).right = y;
```

In the third round, `*mut Node` is exapended further!

```rust
x = (*y).left; // && (*x).right = (*(*y).left).right && ...
T2 = (*x).right; // && ...
(*y).left = T2;
(*x).right = y;
```



```
*q = malloc(..); // q_1
p = q; // p_2 = q_1; p_2_1 = q_1_1;
free(*p); // p_2_1
```





###### Problem

State explosion??? Shall we use a separate shape analyser (say facebook Infer) for composite data structure (say singly linked list)



##### Data Structure

How to track the flow for `*p` ? There is no natural algorithm in the SSA framework to rename the whole expression `*p`, for instance,  `(*p_0)_1` to `(*p_0)_2`. 

CBMC: rename `p` when defining `*p`.

* Value Flow Graph
  Use an initial imprecise points-to analysis. Upon a store `*p = ...`, if `p` points to `{a, b}`, then issue two special statement $\chi(a)$ and $\chi(b)$. Rename both of them. Upon a load `... = *p`, if `p` points to `{a, b}`, then issue $\mu(a)$ and $\mu(b)$.
   This may introduce false def-use chains.
* Singleton Points to Set§
  Track the flow for dereference expression by line number. Top-level variables are renamed, `*p` are tracked by `*(p_k)[l]`







1. CBMC: SSA
2. Two routes, old analysis
3. Empirical study on level of dereferences
4. Limit scope to singly linked list and non-recursive data structures
5. Emprical study: singly linked list is one of the major data structure
6. Array









#### 8.26

1. CBMC: SSA
   Pointer dereferences are resolved to symbolic value.
2. Ownership Dominance property: treats `*p` as `x`! So `(*p).left` can be treated as `x.left`. Defining `*p` is treated as defining `p`!
3. Static single information form to deal with null tests
4. Algorithm 3.7 that prunes SSA


#### 9.4
1. To deal with head references, we may lift the requirement that a local must be non-owning upon exit for function arguments.


#### 9.13
Language:
```rust
p = q;
p = *q;
*p = q;
p = **q; ~ { tmp = deref_copy(*q); p = *tmp; }
**p = q; ~ { tmp = deref_copy(*p); *tmp = q; }
```


### 9.16
- [x] First stage analysis (runnable on large benchmark)
- [x] More empirical studies on struct definitions
- [x] More benchmarks (checked-c benchmark)
- [x] Better C2Rust results (need linux machine)
- [x] Support Union
- [ ] Second stage analysis
  - [x] Allow dereferencing once (works on bst, avl)
  - [x] Allow dereferencing more
  - [ ] Better iterative inference: infer `p` to be owning when `*p = q` and `q` changes its ownership state
- [ ] Initial rewrite
  - [ ] Rewrite BST/AVL
  - [ ] Rewrite Heap
  - [ ] Rewrite KDTREE
  - [ ] Rewrite List/Hashtbl
  - [ ] Rewrite Json parser (array ptr!!!)
- [ ] Smarter constraint solving?
    Relax $x + y = z$ by $x \le z \land y \le z \land x + y \le 1$, this may lead to memory leak, but what if the underying C lib is well-written? We may perform this relaxed analysis in second stage



### 9.23
Move is allowed inside owning pointer: `p: &move &move ..`. After moving, the type becomes `&move & ..`. The question is that, if this inner pointer is behind a struct definition, say `struct Wrapper(*mut _)`, does it need to reclaim ownership? If so, where? 

My thoughts: if it needs to, then it should reclaim ownership when the surrounding pointer is moved. However I don't think it needs to, and in this case, we need two kinds of rewrite when performing move of this kind:
1. `move((*p).0)`, this corresponds to a partial move, which requires explicit ownership re-claiming otherwise rustc fails;
2. `std::mem::take((*p).0)` if `Wrapper.0` is nullable.

Note that a struct has only two kinds of sink:
1. consumer functions like `free`;
2. move.

In the former case, we are happy about partially moved struct. In the latter case, the inferencer will check for ownership signatures, so we should be fine?

Strategy: fields are only checked at function boundaries: return position and local initialisation position.



### 10.7
#### The problem with ownership analysis:
1. The analysis does not infer for `&mut` even if `&mut` and `Box` share uniqueness property, so we have to make many 'random' decisions during rewrite;
2. Sometimes ownership info does not propagate well (especially when library uses its own memory 'manager', say stores customised allocation functions as fn_ptr).
3. Ownership dominance property prohibits `head_ref` style: input pointers are used as an ouput parameter


```rust
unsafe fn f(output_param: *mut *mut i32) {
  free(*output_param);
  *output_param = malloc(..);
  **output_param = 3;
}

unsafe fn add(head_ref: *mut *mut list) {
  let node = new_node();
  let next = *head_ref; // prohibited by ownership dominance property
  *head_ref = node; // prohibited by ownership dominance property
  node.next = next;
}
```

Some low hanging fruits: for function parameters, say `p` in `f(p)`, if the address of `p` is never exposed in `f` (`p` does not appear as `_ = p`, `_ = &p`), then `p` cannot be changed within `f`. We can trivially treat `p` as a unique pointer and grant it `&mut` type qualifier. I call those parameters `p` output parameters

#### Output parameters have state
```rust
unsafe fn free_f(s: *mut S) {
  free((*s).f) // ok if `s` is identified as output parameter
}

unsafe fn free_s(s: *mut S) {
  // requires (*s).f to be owning
  free_f(s);
  // requires (*s).f to be non-owning!
  // => output params have states!
  free(s);
}
```

States for an output parameter: current value and __future__ value. This is then related to RustHorn, Iris, etc.. In those framework, lifetime is not explicitly dealt with. Instead, mutable reference is formalized as a current/future value pair. Violation of lifetime rules reflects to inconsistency in future values.

Interesting future work: handle mutable reference and lifetime as well?


```text
    x -- pre value
    |
    |
   \/
    f(x) -- signature of f
    |
    |
   \/
    x -- post value

    associated constraint: x_post + x_sig = x_pre
```
The signature here acts like a filter. If `x_sig = 1`, then `f` consumes a value, `x_pre` hence needs to be owning and `x_post` becomes non-owning; if `x_sig = 0`, then `f` borrows a value, and `x_pre = x_post`. 
