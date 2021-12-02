CURR: C-style Unsafe Rust Refactorer

### Roadmap
- [ ] Andersen's alias analysis
    - [x] Basic algorithm
        - [ ] Interprocedural, field-sensitive
        - [ ] Optimisation wrt the [paper](https://www.cs.utexas.edu/~lin/papers/pldi07.pdf)
    - [ ] Optimisation of work list algorithm wrt the [slides](https://homepages.dcc.ufmg.br/~fernando/classes/dcc888/ementa/slides/WorkList.pdf)
- [ ] Rustfix and source code rewriting
- [ ] Transformations that move statments downward/upward
