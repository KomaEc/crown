/* See LICENSE file for copyright and license details. */
#include "internals.h"

#define tb  libzahl_tmp_pow_b
#define tc  libzahl_tmp_pow_c
#define td  libzahl_tmp_pow_d


void
zmodpow(z_t a, z_t b, z_t c, z_t d)
{
	size_t i, j, n, bits;
	zahl_char_t x;

	if (zsignum(c) <= 0) {
		if (zzero(c)) {
			if (zzero(b))
				FAILURE(EDOM); /* Indeterminate form: 0:th power of 0 */
			else if (zzero(d))
				FAILURE(EDOM); /* Undefined form: Division by 0 */
			zsetu(a, 1);
		} else if (zzero(b) || zzero(d)) {
			FAILURE(EDOM); /* Undefined form: Division by 0 */
		} else {
			SET_SIGNUM(a, 0);
		}
		return;
	} else if (zzero(d)) {
		FAILURE(EDOM); /* Undefined form: Division by 0 */
	} else if (zzero(b)) {
		SET_SIGNUM(a, 0);
		return;
	}

	bits = zbits(c);
	n = FLOOR_BITS_TO_CHARS(bits);

	zmod(tb, b, d);
	zset(tc, c);
	zset(td, d);
	zsetu(a, 1);

	for (i = 0; i < n; i++) { /* Remember, n is floored. */
		x = tc->chars[i];
		for (j = BITS_PER_CHAR; j--; x >>= 1) {
			if (x & 1)
				zmodmul(a, a, tb, td);
			zmodsqr(tb, tb, td);
		}
	}
	x = tc->chars[i];
	for (; x; x >>= 1) {
		if (x & 1)
			zmodmul(a, a, tb, td);
		zmodsqr(tb, tb, td);
	}
}
