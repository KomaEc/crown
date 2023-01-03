/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
ztrunc(z_t a, z_t b, size_t bits)
{
	zahl_char_t mask = 1;
	size_t chars, i;

	if (zzero(b)) {
		SET_SIGNUM(a, 0);
		return;
	}

	chars = CEILING_BITS_TO_CHARS(bits);
	a->sign = b->sign;
	a->used = MIN(chars, b->used);
	if (a->used < chars)
		bits = 0;
	if (a != b) {
		ENSURE_SIZE(a, a->used);
		zmemcpy(a->chars, b->chars, a->used);
	}
	bits = BITS_IN_LAST_CHAR(bits);
	if (bits) {
		mask <<= bits;
		mask -= 1;
		a->chars[a->used - 1] &= mask;
	}

	for (i = a->used; i--;)
		if (a->chars[i])
			return;
	SET_SIGNUM(a, 0);
}
