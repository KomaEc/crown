VERSION = 0.0

PREFIX = /usr/local
EXECPREFIX = $(PREFIX)
MANPREFIX = $(PREFIX)/share/man

CC = cc
AR = ar
RANLIB = ranlib

# Unless /dev/urandom exists and is a non-blocking random number generator,
# you have to add -DFAST_RANDOM_PATHNAME=... to CPPFLAGS, and
# unless /dev/random exists and is a blocking secure random number generator
# you have to add -DSECURE_RANDOM_PATHNAME=... to CPPFLAGS.

CPPFLAGS = -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_XOPEN_SOURCE=700
CFLAGS   = -std=c99 -O3 -Wall -pedantic
LDFLAGS  = -s
