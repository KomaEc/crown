/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zrsh(z_t a, z_t b, size_t bits)
{
	size_t i, chars, cbits;

	if (!bits) {
		SET(a, b);
		return;
	}

	chars = FLOOR_BITS_TO_CHARS(bits);

	if (zzero(b) || chars >= b->used || zbits(b) <= bits) {
		SET_SIGNUM(a, 0);
		return;
	}

	bits = BITS_IN_LAST_CHAR(bits);
	cbits = BITS_PER_CHAR - bits;

	if (chars && a == b) {
		a->used -= chars;
		zmemmove(a->chars, a->chars + chars, a->used);
	} else if (a != b) {
		a->used = b->used - chars;
		ENSURE_SIZE(a, a->used);
		zmemcpy(a->chars, b->chars + chars, a->used);
	}

	if (bits) { /* This if statement is very important in C. */
		a->chars[0] >>= bits;
		for (i = 1; i < a->used; i++) {
			a->chars[i - 1] |= a->chars[i] << cbits;
			a->chars[i] >>= bits;
		}
		while (!a->chars[a->used - 1])
			a->used--;
	}

	SET_SIGNUM(a, zsignum(b));
}
