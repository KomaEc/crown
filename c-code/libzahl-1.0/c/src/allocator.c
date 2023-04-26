/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
libzahl_realloc(z_t a, size_t need)
{
	size_t i, x;
	zahl_char_t *new;

	/* Find n such that n is a minimal power of 2 ≥ need. */
	if ((need & (~need + 1)) != need) {
		need |= need >> 1;
		need |= need >> 2;
		need |= need >> 4;
		for (i = sizeof(need), x = 8; i; i >>= 1, x <<= 1)
			need |= need >> x;
		need += 1;
	}

	for (i = 0, x = need; x; x >>= 1)
		i += 1;

	if (libzahl_pool_n[i]) {
		libzahl_pool_n[i]--;
		new = libzahl_pool[i][libzahl_pool_n[i]];
		zmemcpy(new, a->chars, a->alloced);
		zfree(a);
		a->chars = new;
	} else {
		a->chars = realloc(a->chars, need * sizeof(zahl_char_t));
		if (!a->chars) {
			if (!errno) /* sigh... */
				errno = ENOMEM;
			FAILURE(errno);
		}
	}
	a->alloced = need;
}
