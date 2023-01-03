include config.mk

HDR =\
	zahl.h\
	src/internals.h

FUN =\
	zabs\
	zadd\
	zand\
	zbits\
	zbset\
	zbtest\
	zcmp\
	zcmpi\
	zcmpmag\
	zcmpu\
	zdiv\
	zdivmod\
	zerror\
	zfree\
	zgcd\
	zload\
	zlsb\
	zlsh\
	zmod\
	zmodmul\
	zmodpow\
	zmodpowu\
	zmodsqr\
	zmul\
	zneg\
	znot\
	zor\
	zperror\
	zpow\
	zpowu\
	zptest\
	zrand\
	zrsh\
	zsave\
	zset\
	zseti\
	zsets\
	zsetu\
	zsetup\
	zsplit\
	zsqr\
	zstr\
	zstr_length\
	zsub\
	zswap\
	ztrunc\
	zunsetup\
	zxor

INLINE_FUN =\
	zinit\
	zeven\
	zodd\
	zeven_nonzero\
	zodd_nonzero\
	zzero\
	zsignum

OBJ = $(FUN:=.o) allocator.o
MAN3 = $(FUN:=.3) $(INLINE_FUN:=.3)
MAN7 = libzahl.7

all: libzahl.a

%.o: src/%.c $(HDR) config.mk
	$(CC) $(CFLAGS) $(CPPFLAGS) -c -o $@ $<

libzahl.a: $(OBJ)
	$(AR) rc $@ $?
	$(RANLIB) $@

test-random.c: test-generate.py
	./test-generate.py > test-random.c

test: test.c libzahl.a test-random.c
	$(CC) $(LDFLAGS) $(CFLAGS) $(CPPFLAGS) -o $@ test.c libzahl.a

check: test
	./test

install: libzahl.a
	mkdir -p -- "$(DESTDIR)$(EXECPREFIX)/lib"
	mkdir -p -- "$(DESTDIR)$(PREFIX)/include"
	mkdir -p -- "$(DESTDIR)$(MANPREFIX)/man3"
	mkdir -p -- "$(DESTDIR)$(MANPREFIX)/man7"
	cp -- libzahl.a "$(DESTDIR)$(EXECPREFIX)/lib"
	cp -- zahl.h "$(DESTDIR)$(PREFIX)/include"
	cp -- $(foreach M,$(MAN3),man/$(M)) "$(DESTDIR)$(MANPREFIX)/man3"
	cp -- $(foreach M,$(MAN7),man/$(M)) "$(DESTDIR)$(MANPREFIX)/man7"

uninstall:
	rm -- "$(DESTDIR)$(EXECPREFIX)/lib/libzahl.a"
	rm -- "$(DESTDIR)$(PREFIX)/include/zahl.h"
	cd "$(DESTDIR)$(MANPREFIX)/man3" && rm $(MAN3)
	cd "$(DESTDIR)$(MANPREFIX)/man7" && rm $(MAN7)

clean:
	-rm -- *.o *.su *.a *.so test test-random.c 2>/dev/null

.PHONY: all check clean install uninstall
