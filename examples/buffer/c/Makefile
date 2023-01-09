
test: buffer.c test.c
	@$(CC) $^ -std=c99 -o $@
	@./test

C2RUST_HOME ?= "${HOME}/Work/c2rust"
HOSTNAME = $(shell hostname)
XC_PLUGIN = "${C2RUST_HOME}/dependencies/clang-xcheck-plugin.${HOSTNAME}/plugin/CrossChecks.so"
test_xcheck: CC = "${C2RUST_HOME}/dependencies/llvm-6.0.1/build.${HOSTNAME}/bin/clang-6.0"
test_xcheck: CFLAGS += -Xclang -load -Xclang ${XC_PLUGIN} -Xclang -add-plugin -Xclang crosschecks
test_xcheck: LDFLAGS += -L"${C2RUST_HOME}/dependencies/clang-xcheck-plugin.${HOSTNAME}/runtime/" -lruntime
test_xcheck: LDFLAGS += -L"${C2RUST_HOME}/cross-checks/libfakechecks/" -lfakechecks
test_xcheck: buffer.c test.c
	@$(CC) $(CFLAGS) $^ -std=c99 -o $@ $(LDFLAGS)
	@LD_LIBRARY_PATH="${C2RUST_HOME}/cross-checks/libfakechecks/" ./$@

.PHONY: test test_xcheck
