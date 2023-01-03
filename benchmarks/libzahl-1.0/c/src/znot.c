/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
znot(z_t a, z_t b)
{
	size_t bits, n;

	if (zzero(b)) {
		SET_SIGNUM(a, 0);
		return;
	}

	bits = zbits(b);
	SET(a, b);
	SET_SIGNUM(a, -zsignum(a));

	for (n = a->used; n--;)
		a->chars[n] = ~(a->chars[n]);
	bits = BITS_IN_LAST_CHAR(bits);
	if (bits)
		a->chars[a->used - 1] &= ((zahl_char_t)1 << bits) - 1;

	while (a->used && !a->chars[a->used - 1])
		 a->used--;
	if (!a->used)
		SET_SIGNUM(a, 0);
}
