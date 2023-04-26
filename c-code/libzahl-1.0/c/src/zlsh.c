/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zlsh(z_t a, z_t b, size_t bits)
{
	size_t i, chars, cbits;
	zahl_char_t carry[] = {0, 0};

	if (zzero(b)) {
		SET_SIGNUM(a, 0);
		return;
	}
	if (!bits) {
		SET(a, b);
		return;
	}

	chars = FLOOR_BITS_TO_CHARS(bits);
	bits = BITS_IN_LAST_CHAR(bits);
	cbits = BITS_PER_CHAR - bits;

	ENSURE_SIZE(a, b->used + chars);
	if (a == b)
		zmemmove(a->chars + chars, b->chars, b->used);
	else
		zmemcpy(a->chars + chars, b->chars, b->used);
	zmemset(a->chars, 0, chars);
	a->used = b->used + chars;

	if (bits) { /* This if statement is very important in C. */
		for (i = chars; i < a->used; i++) {
			carry[~i & 1] = a->chars[i] >> cbits;
			a->chars[i] <<= bits;
			a->chars[i] |= carry[i & 1];
		}
		if (carry[i & 1]) {
			ENSURE_SIZE(a, a->used + 1);
			a->chars[i] = carry[i & 1];
			a->used++;
		}
	}

	SET_SIGNUM(a, zsignum(b));
}
