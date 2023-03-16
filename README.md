# ORC
This is a wip research project on translating C to Rust. This is still under development and pre-release.

## Usage
Crown uses a specifc rust toolchain, which is specify in `rust-toolchain.toml`.

Crown uses rustc internals, so it is required that rustc libraries are added to system paths.
For linux,
`export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib`
For Mac,
`export DYLD_FALLBACK_LIBRARY_PATH=$(rustc --print sysroot)/lib`

### Compilation
`cargo build -r`
you can find the binary at `target/release/crown`

### Some Hiden Preprocessing Steps
Please compare `benchmark` and `benchmark2`.

In general, it is required that the following directives be added in the `lib.rs`:
```rust
#![feature(strict_provenance)]
#![feature(core_intrinsics)]
#![feature(raw_ref_op)]
```

Also, main function shall be commented out (since there might be some constructs like `Vec` that is not handled in crown).

### Preprocess
`target/release/crown PATH/TO/lib.rs preprocess in-place`
or
`cargo run -r --bin crown -- PATH/TO/lib.rs preprocess in-place`

then

`target/release/crown PATH/TO/lib.rs explicit-addr in-place`
or
`cargo run -r --bin crown -- PATH/TO/lib.rs explicit-addr in-place`

These two commands shall be merged into one in the future.

### Ownership Analysis
`target/release/crown PATH/TO/lib.rs ownership`
or 
`cargo run -r --bin crown -- PATH/TO/lib.rs ownership`
this will print out the analysis results

### Refactor
`target/release/crown PATH/TO/lib.rs rewrite in-place`


### Example
In the crown directory,
```shell
cp benchmark2/libtree .
target/release/crown libtree/lib.rs preprocess in-place
target/release/crown libtree/lib.rs explicit-addr in-place
target/release/crown libtree/lib.rs analyse # this has no effects on the source code, just for printing analysis results
target/release/crown libtree/lib.rs rewrite # perform refactor
```
