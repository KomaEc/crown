#!/bin/sh
#
# Compiler wrapper script for cross-checks
# Pass to autogen.sh when running it:
# $ ./autogen.sh CC=./xchecks/cc_wrapper.sh [other autogen arguments...]

SCRIPT_DIR=`dirname $0`
C2RUST_DIR=`readlink -f $SCRIPT_DIR/../../../..`
MACHINE=`uname -n`
CC_WRAPPER="$C2RUST_DIR/cross-checks/c-checks/clang-plugin/cc_wrapper.sh"
#CC="$C2RUST_DIR/dependencies/llvm-6.0.1/build.$MACHINE/bin/clang"
CC="/usr/bin/clang"
XCHECK_PLUGIN="$C2RUST_DIR/dependencies/clang-xcheck-plugin.$MACHINE/plugin/CrossChecks.so"
XCHECK_RUNTIME="$C2RUST_DIR/dependencies/clang-xcheck-plugin.$MACHINE/runtime/libruntime.a"

#XCHECK_LIB=fakechecks
#XCHECK_LIB_PATH="$C2RUST_DIR/cross-checks/libfakechecks"
#XCHECK_LIB=fakechecks_zstd
#XCHECK_LIB_PATH="$C2RUST_DIR/cross-checks/rust-checks/target/release"
XCHECK_LIB=clevrbuf
XCHECK_LIB_PATH="$C2RUST_DIR/cross-checks/ReMon/libclevrbuf"

OUR_CFLAGS="-ffunction-sections"

# The C version calls stat64() and the Rust one calls stat()
# without this, which leads to divergence
OUR_CFLAGS+=" -DNO_LARGEFILE_SOURCE"

# We need `-fno-builtin` for testlimits.c, since it
# calls `printf` with constant strings, and we don't want
# clang to convert those to `puts`
OUR_CFLAGS+=" -fno-builtin"

exec "$CC_WRAPPER" "$CC" "$XCHECK_PLUGIN" "$XCHECK_RUNTIME" \
    $OUR_CFLAGS "$@" \
    -L$XCHECK_LIB_PATH -l$XCHECK_LIB -lpthread \
    -Xclang -plugin-arg-crosschecks -Xclang -C$SCRIPT_DIR/libxml2_c.yaml
