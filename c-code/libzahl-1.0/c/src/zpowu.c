/* See LICENSE file for copyright and license details. */
#include "internals.h"

#define tb  libzahl_tmp_pow_b


void
zpowu(z_t a, z_t b, unsigned long long int c)
{
	if (!c) {
		if (zzero(b))
			FAILURE(EDOM); /* Indeterminate form: 0:th power of 0 */
		zsetu(a, 1);
		return;
	} else if (zzero(b)) {
		SET_SIGNUM(a, 0);
		return;
	}

	zset(tb, b);
	zsetu(a, 1);

	for (; c; c >>= 1) {
		if (c & 1)
			zmul(a, a, tb);
		zsqr(tb, tb);
	}
}
