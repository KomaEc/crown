# Ownership and Lifetime guided C to Rust Refactoring

## Usage
Currently, this tool will do some meaningless stuff: given a path to a Cargo project (which is required to have a `Cargo.toml` file, with a 'lib' section indicating the path to the main lib entry), it will add to dependencies the 'crustr_ptr' library (which is a customised pointer wrapper type), and then it will rewrite `*mut T` in all struct definition into `Option<NonNullRawMut<T>>`.
To run:
```shell
cp -r benchmark/ezxml workspace
cargo run -- workspace/ezxml
```

## Motivating Examples
We show several refactoring examples guided by the ownership and lifetime _principles_ of Rust. I use the word _principle_, because Rust currently does not have a clear definition of lifetime. The borrow checker currently implemented in Rust view lifetimes as lifetimes of references. For example, for a reference of type `&'a T`, the lifetime `'a` refers to the set of program points where this reference is valid. However, there is a currently merging new formulation, called [polonius](http://smallcultfollowing.com/babysteps/blog/2018/04/27/an-alias-based-formulation-of-the-borrow-checker/), where such lifetime variable `'a` is treated as _a set of loans_. The new analysis is based on alias analysis and is more powerful.

## Ownership
Semantic differences between a C pointer of type `T*` and a Rust mutable reference of type `&mut T` or owned pointer of type `Box<T>`. A mutable reference or an owned pointer in Rust stands for _unique_ access to an object, while a pointer in C stands for an alias. In Rust, it is not allowed to copy those _unique_ references.

To transfer from _aliasing_ to _unique_, we have to synthesize new code/reoder statements/substitute variables, etc.

### BST min_value_node

For example, consider a binary search tree

```rust
pub struct node {
    pub key: i32,
    pub left: *mut node,
    pub right: *mut node,
}
```

with a safer definition as

```rust
pub struct Node {
    pub key: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
```

The original code snippet that traverses a node and return the leftmost child node:

```rust
pub unsafe extern "C" fn minValueNode(mut node: *mut node) 
                                      -> *mut node {
    let mut current: *mut node = node;
    while !current.is_null() && !(*current).left.is_null() {
        current = (*current).left
    }
    return current;
}
```

Since we cannot tell whether the returned pointer should be mutably or immutably used, we refactor it into two different versions:

```rust
/** Immutable version */
pub fn min_value_node(node: Option<&Node>) -> Option<&Node> {
    let mut current = node;
    while let Some(node_ref) = current {
        if let Some(_) = node_ref.left {
            current = node_ref.left.as_ref().map(|t| t.as_ref());
        } else {
            break;
        }
    }
    return current;
}
```

The immutable version `min_value_node` completely identical to the unsafe version. Note that the original short-circuit `&&` condition is refactored into two separate conditions. The `let Some(a) = b` in a condition behaves like testing whether an option `b` has the `Some` variant, and if so, taking the value. In this case, `current` has type `Option<&Node>`, where `&Node` can be copied out.

```rust
/** Mutable version */
pub fn min_value_node_mut(node: Option<&mut Node>) -> Option<&mut Node> {
    let mut current = node;
    while let Some(mut node_mut) = current {
        if let Some(_) = node_mut.left {
            current = node_mut.left.as_mut().map(|mut t| t.as_mut());
        } else {
          	/* Extra codes need to be synthesized */
            current = Some(node_mut);
            break;
        }
    }
    return current;
}
```
 The mutable version is also identical, except that in the _else_ branch, we have to insert new code `current = Some(node_mut)`.  

The reason for this change is that mutable reference in Rust represents unique access. Note that `current` has type `Option<&mut Node>`,  in the while loop condition `let Some(mut node_mut) = current`, the mutable reference `node_mut` inside `current` is _moved_ out instead of _copied_ out. Since `current` will be used after, we have to assign appropriate content back to it after this move.

### AVL right_rotate

Consider an AVL tree right rotation:

```rust
pub struct Node {
    pub key: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
  	pub height: i32,
}
```

The original code snippet:

```rust
pub unsafe extern "C" fn rightRotate(mut y: *mut Node) -> *mut Node {
    let mut x: *mut Node = (*y).left;
    let mut T2: *mut Node = (*x).right;
  
  
    /* Start of Problematic statments */
    (*x).right = y;
    (*y).left = T2;
    (*y).height =
        max(height((*y).left), height((*y).right)) + 1 as i32;
    (*x).height =
        max(height((*x).left), height((*x).right)) + 1 as i32;
  	/* End of Problematic statments */
  
  
    return x;
}
```

To refactor it into safe Rust, consider changing all `*mut Node` into `Option<Box<Node>>`. However, the highlighted four statements will make any attempt fail: in the first statement, `(*x).right` is set to be an alias of `y`, however, it is not allowed for _unique_ references like `Box`.

To refactor the code, we will have to do either of the following:

1. Reorder statements
2. Substitute `y` by `(*x).right`

The following is a result code snippet following the first approach:

```rust
pub fn rightRotate(mut y0: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut y = y0.unwrap();

    let mut x: Box<Node> = y.left.unwrap();
    let mut T2: Box<Node> = x.right.unwrap();
  
  
    /* Reordered */
    y.left = Some(T2);
    y.height = max(height(y.left.as_ref().map(|t| t.as_ref())),
                   height(y.right.as_ref().map(|t| t.as_ref()))) + 1;
    x.right = Some(y);
    x.height = max(height(x.left.as_ref().map(|t| t.as_ref())),
                   height(x.right.as_ref().map(|t| t.as_ref()))) + 1;
  
  
    return Some(x);
}
```

## Lifetime

TODO


## This Week

### Empirical Study
The pattern that I expect to find:
```rust
let q = &mut *p; // q mutably borrows p
mutating_use(p); // mutating use of p invalidates q
use(q);          // illegal
```
The corresponding transformation:
1. push the declaration down
2. `[q |-> p]`

Identify two such patterns in lib 'ezxml' (a C library of 1000 lines). But static analysis is imprecise (too many false positives).

Maybe we can use analysis tools on C?

### Ownership Analysis for C
Overall strategy?

Given: a set `S` of C-style structs, a set `F` of functions.

Originally: leave `S` as it is (argument: changing `*mut T` to `Box<T>` leads to huge semantics changes). Function signatures are naively inferred (1. analysis tool in C2Rust; 2. alias analysis), function bodies are transformed by reorder statements.

Now:
1. `*mut T` -> `Option<Unique<T>>`. `Unique<T>` is a wrapper type to `*mut T`, but it acts as if it has unique access right to the underlying `T`. This is done by exposing a method `as_mut()`. The type signature is `fn as_mut(&'a mut self) -> &'a mut T`.
Note: for a raw pointer `p`, `p.as_mut()` has unconstrained lifetime.
2. Analysis tool in C2Rust is informal and imprecise.
   

Ownership analysis paper:
1. [Automatic Generation of Library Bindings Using Static Analysis](https://dl.acm.org/doi/pdf/10.1145/1542476.1542516)
A simple static analysis to distinguish array pointer and non-array pointer
2. [A Practical Flow-Sensitive and Context-Sensitive C and C++ Memory Leak Detector](https://suif.stanford.edu/papers/pldi03d.pdf)
A static analysis to analyze whether a pointer variable `p` owns the underlying data. Pointers are labelled with an ownership indicator ρ ∈ {0, 1}.
For statement `p = q`, either ownership is transferred from `q` to `p`, or a transient pointer is created, which does not claim ownership. This kind of semantics is encoded as 0-1 linear constraints over ownership indicators (For instance, ρ_p + ρ_q = ρ_q_prev)

I expect: devise such constraint system, infer correct type for `S` and `F`, reorder statements along the way.
What's new in Rust: if `p = q` creates a transient pointer, then it's permission is somehow dependent on `q`. 

(Get more out of the constraint system)

### Fuzzer
The blog [Refactoring Rust Transpiled from C](https://immunant.com/blog/2020/09/transpiled_c_safety/)

```rust
// Transpiled Rust
#[repr(C)]
pub struct buffer_t {
   pub len: size_t,
   pub alloc: *mut libc::c_char,
   pub data: *mut libc::c_char,
}
```

Correct typing: `alloc` owns the data, `data` acts as if it's a cursor.

Suggestions from the blog: fuzzer to detect how `data` reference `alloc`, and therefore infer the correct typing.


### Case Study (EZXML)
1. Pattern in fuzzer (ezxml_decode)
2. Pointer assignment pattern
`p = q = r = NULL` is translated into
```rust
r = null;
q = r; // introduce extra dependency!
p = q; // introduce extra dependency!
```



More benchmark


## Analysis

$\rho \in \{0, 1\}$ indicates ownership. The original analysis correctly modelled ownerhip transfer relation:
$p = q; \Rightarrow \rho_{p^\prime} + \rho_{q^\prime} = \rho_q$

In our case, we may need to model two additional things:
1. Temporary ownership transfer (borrowing)
2. XOR principle (fractional ownership)

### Something More from the Analysis
1. OOPSLA: does not model ownership at all. This: correctly infer ownership types
2. C2Rust Analysis: Imprecise. Does not model ownership transfer
```c
// called when the parser finds a processing instruction
void ezxml_proc_inst(ezxml_root_t root, char *s, size_t len)
{
    int i = 0, j = 1;
    char *target = s;

    s[len] = '\0'; // null terminate instruction
    if (*(s += strcspn(s, EZXML_WS))) {
        *s = '\0'; // null terminate target
        s += strspn(s + 1, EZXML_WS) + 1; // skip whitespace after target
    }

    if (! strcmp(target, "xml")) { // <?xml ... ?>
        if ((s = strstr(s, "standalone")) && ! strncmp(s + strspn(s + 10,
            EZXML_WS "='\"") + 10, "yes", 3)) root->standalone = 1;
        return;
    }

    if (! root->pi[0]) *(root->pi = malloc(sizeof(char **))) = NULL; //first pi

    while (root->pi[i] && strcmp(target, root->pi[i][0])) i++; // find target
    if (! root->pi[i]) { // new target
        root->pi = realloc(root->pi, sizeof(char **) * (i + 2));
        root->pi[i] = malloc(sizeof(char *) * 3);
        root->pi[i][0] = target;
        root->pi[i][1] = (char *)(root->pi[i + 1] = NULL); // terminate pi list
        root->pi[i][2] = strdup(""); // empty document position list
    }

    while (root->pi[i][j]) j++; // find end of instruction list for this target
    root->pi[i] = realloc(root->pi[i], sizeof(char *) * (j + 3));
    root->pi[i][j + 2] = realloc(root->pi[i][j + 1], j + 1);
    strcpy(root->pi[i][j + 2] + j - 1, (root->xml.name) ? ">" : "<");
    root->pi[i][j + 1] = NULL; // null terminate pi list for this target
    root->pi[i][j] = s; // set instruction
}
```
In this example, a correct analysis will infer that the input `s` is an owned string. Upon declaration of the local `target`, there is an ownership transfer (`target` should have type `Box<..>`). After this transfer, `s` becomes an non-owning, transient pointer that acts like a cursor, and it should be modelled by a raw pointer. Additional program transformation:
```c
let target = s; // ownership transfer
let s: *mut _ = target; // new statement inserted
```

### Several attemps on Modeling Temporary ownership transfer
Expectation: constraint that is linear to the size of the program $\Rightarrow$ constraint variables must be in linear size.

However, to model 'temporary', we must know when the loan is clear. For instance,
```c
p = malloc(4); // rho_p = 1, tau_p = 1
q = p; // temporarily transferred to q
       // rho_p' + rho_q' = rho_p, tau_p' <= tau_p, tau_q' <= tau_p
mutating_use(q); // after, transferred back to p
                 // tau_q' = 1
r = p; // temporarily transferred to r
       // 
mutating_use(r); // transferred back
                 // tau_r'' = 0
free(p);
```
Note that in statement like `mutating_use(q)`, the variable `p` is invisible. How can we notify that after this mutating use, the ownership may be transferred back to `p`?

1. Link `q` to `p`? Control flow? quadratic solution size?
2. $\rho_{p, i}$, where $i$ represents the line number. Complicated?

#### Attempt: Seperating access right from ownership
$\tau_p \in \{0, 1\}$ is an access token. $\tau_p = 1$ means $p$ can be (mutably) used. Now mutably borrow does not mean temporary ownership transfer, but (possibly) duplication of access token:
$p = q; \Rightarrow \rho_{p^\prime} + \rho_{q^\prime} = \rho_q \land \tau_{q^\prime} \le \tau_{q} \land \tau_{p^\prime} \le \tau_{q} \land \rho_{p^\prime} \le \tau_q$
To control the 'return' of an access token, we formulate the constraint system as a minimisation problem:
$\min{\Sigma_{v}\tau_{v}}$ such that $C$.

However, it is not a very precise model.

### Idea?
1. __Prophecy variable__? It seems that recent verification systems on Rust tends to have constraints that _have access to the future_. For example in ESOP2019, a mutable reference is modelled by a pair $(m_0, m_1)$, where $m_0$ is the current target value, $m_1$ is the future target value. For example, prophecy variable in Iris.
Shall we apply the same idea here, that use a prophecy variable $\psi_p$ to indicate that the access right of $p$ must be returned in the future?
2. __Level-labelled__ The access right of a variable is labelled with a level $\ell_p \in \mathbb{Z}$. A resource (for instance, an object that is explicitly allocated) is labelled with a level as well. Creating mutable reference $\Rightarrow$ increasing level


### Ownership Analysis
1. Types
2. Assignment statements
   ```rust
   p = q;
   // ==> p = move q; // if ownership transfer happens
   // ==> p = &mut *q; // otherwise
   ```
3. Unsatisfiable core ==>
   1. Some pointers are inherently unsafe
   2. A set of transformation

### Feb 4
1. Ownership analysis
   1. SSA (ok)
   2. Constraint generation (partially ok)
   3. Z3
2. Array pointers are particularly annoying! (refactor `T` into `[T]`)
   1. occurs too frequently in C projects, introduces a lot of unsafe
   2. `offset()` complicates the MIR, making simple analysis hard to implement. Replacing `offset()` by array indexing also improves readability
   3. Design space for interesting transformation/refactoring patterns.
      1. thin pointer => fat pointer
      2. zero-terminated pointer (a la Cyclone)

    ==> `T` -> `[T]`; Lightweight loop summary (pattern detection?)

    Detection: `p` is an array pointer if
        1. `p.offset()`;
        2. `p = calloc()`;
        3. `_ = realloc(p)` occurs 

arr[i].f

*arr.offset(i).f
ptr = arr.offset(i)
*ptr = f


### Feb 11
1. SSA ~~> insert phi nodes, rename
2. slice analysis (value flow analysis)
Purpose of this analysis: 
   1. systematically remove `realloc` and `calloc`; 
   2. systematically remove `offset()`
    
    `L(p) = 1` ==> `p` is fat (with runtime length information)
    ```
        p = q
    ------------------
        L(p) <= L(q)


        p = offset(whatever, _)
    --------------------------------
        L(p) = 0


        p = calloc(..)
    --------------------------------
        L(p) = 1


        p = realloc(q, ...)
    --------------------------------
        L(q) = 1, L(p) <= L(q)
    ```
    Rewrite from solution: if `L(p) = 1`,
    1.  `p: *mut T` ==> `p: *mut [T]`, 
    2.  `p = calloc(..)` ==> `p = Box::leak(Vec::new(..))`
    3.  `free(p)` ==> `let _ = Box::from_raw(p) // runtime info is stored with p`
    4.  `p.offset(i)` ==> `let mut slice = p.as_mut().unwrap(); slice[i]`

    If there is no solution ==> a set of fixes
1. ownership analysis