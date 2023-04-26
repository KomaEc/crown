# Artifact for Crown
The artifact contains the source code our tool and all the benchmarks we looked into.
This document covers:
- File structure of the artifact
- Instructions to run our tool on a single benchmark
- Instructions to run out tool over all benchmarks and to produce the evaluation results

Note: due to the effect of some bug fixes, there are two tiny changes to the data for `buffer` in table 2: the #ptr goes from 38 to 37, and the #uses goes from 56 to 54. The percentage remains the same.

### File structure
We primarily work in the `/root/crown` folder. Inside the `crown` folder:
- `crates, src, build.rs, Cargo.toml`, etc. are source codes of the tool (as a cargo project)
- `benchmark` contains the set of benchmark in unsafe Rust code
- `comparison` contains the data necessary for comparing our tool to previous work. It includes the benchmarks in [14] and evaluation results of the tool in [14] (folder `laertes-laertes`); benchmarks provided by us and evaluation results we obtained by emailing authors of [14] (`laertes-crown`).
- `c-code` contains the original C code we started with. Our benchmarks are obtained by invoking `c2rust transpile --emit-modules --fail-on-error --reduce-type-annotations --emit-build-files PATH/TO/compile_commands.json`, where `compile_commands.json`s are obtained by `bear make` if applicable. We made some tiny changes to better work with them (like adding feature flags `#![feature(raw_ref_op)]`, removing wrappers of main entries, etc.). We omit these steps in this document. (The benchmark `tuplipindicator` can be found in [14]'s artifact).

### Instructions on a single benchmark
We use the `buffer` benchmark as an example.

First make a copy of `buffer`:
```shell
# in crown folder
cp -r benchmark/buffer .
```

__Run the preprocessing scripts__
```shell
# in crown folder
./preprocess.sh buffer
```
The preprocesing scripts will perform preprocessing steps as described in Sec7.1. Also it will provides some extra steps like adding null assertions:
```rust
if (p.is_null()) {
    assert!(p.is_null())
}
```
changing `&mut` to `&raw mut`, etc. (source code can be found in `crates/preprocess`), which facilitate our analysis.

__Analyse `buffer`__
```shell
# in crown folder
./analyse.sh buffer
```
You can see a trace of log information. At the end, the function sigatures with respect to ownership are printed out. Also, inside the `buffer` folder, there is an `analysis_results` folder which contain the ownership/fatness/mutability information for all local variables in json files.

__Perform rewrite__
```shell
# in crown folder
./rewrite.sh buffer
```
Similarly, a trace of log information will be printed out.

__Check compile__
```shell
# in crown folder
./check.sh buffer
```
`./check.sh` scripts will apply `cargo check` to check if `buffer` compiles (with all warnings suppressed). You can also go to the `buffer` folder and run `cargo check`.

__Run the test case__
C2rust transforms C programs to Rust library. To make the `buffer` library executable, we need to apply a patch that adds a main entry and changes `Cargo.toml`.
```shell
# in crown folder
mkdir test
cp -r buffer test
patch -s -p0 -f -r -< test.patch
```
Now `test/buffer` contains a copy of `buffer` that is testable. The following commands perform the test.
```shell
cd test/buffer
cargo run
```

__Evaluate__
```shell
# in crown folder
./evaluate.sh benchmark/buffer . buffer
```
You can find the evaluation results in `evaluation.csv`. The last columne corresponds to the pointer reduction rate.


### Instructions for all benchmark

#### Producing Table 1
```shell
# in crown folder
./benchmark_statistics.sh > size.csv
./sort.sh size.csv
```
The `size.csv` corresponds to table 1.

#### Run all benchmark
We provide a scripts that replicates the previous steps for all benchmarks. Before running it, make sure there is no `results` folder exists in `crown`.
```shell
# rm -r results # if already exists
# in crown folder
./run.sh
```
Now all transformed programs will be in the `results` folder.


#### Check all benchmark
Before checking all benchmark, we note that the benchmark 'heman' seems to trigger some Rust compiler internal bugs with our working version (nightly-2023-01-26). Here we need compile this program with another version. We already included a `rust-toolchain.toml` file in 'heman', so the following commands check that it compiles:
```shell
cd results/heman
cargo check # or cargo build
```

To check all results, run
```shell
./check.sh results
```
Since `./check.sh` resides in the 'crown' folder, our working version of the Rust compiler will be used. As a consequence, the 'heman' benchmark triggers internal errors.

#### Producing Table 2
First run all benchmarks,
```shell
# rm -r results # if already exists
# in crown folder
./run.sh
```
To produce Table2, run
```shell
./evaluate.sh benchmark results results
```
The table could be found in `evaluation.csv`, which reproduces the 'crown' column in Table 2.

We provide another script that evaluate the tool in [14]:
```shell
# in crown folder
./mkcomparison.sh
```
The table could be found in `comparison/laertes-laertes/evaluation.csv` and `comparison/laertes-crown/evaluation.csv`. Together they reproduce the `laertes` column in Table 2.

#### Test
As claimed in the paper, libtree, rgba, quadtree, urlparser, genann, buffer are associated with unit tests, and the translated versions pass all these tests.
```shell
# rm -r test # if test folder already exists
mkdir test
cp -r results/{libtree,rgba,quadtree,urlparser,genann,buffer} test
patch -s -p0 -f -r -< test.patch
```

Now for rgba, quadtree, urlparser, genann, buffer, go into the corresponding folder and run `cargo run`, the test results will be printed out.

For libtree, the tests are provided in `libtree/tests`. For example,
```shell
# in crown folder
cd test/libtree
cargo build
cp target/debug/libtree . # compile and copy the binary
cd tests/03_direct_and_absolute_rpath
make all
```
The behaviour is the same as specified in the `Makefile`, where the first call cannot find lib_f.so, and the second one should.
