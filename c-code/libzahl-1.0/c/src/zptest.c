/* See LICENSE file for copyright and license details. */
#include "internals.h"

#define x   libzahl_tmp_ptest_x
#define a   libzahl_tmp_ptest_a
#define d   libzahl_tmp_ptest_d
#define n1  libzahl_tmp_ptest_n1
#define n4  libzahl_tmp_ptest_n4


enum zprimality
zptest(z_t witness, z_t n, int t)
{
	/*
	 * Miller–Rabin primarlity test.
	 */

	size_t i, r;

	if (zcmpu(n, 3) <= 0) {
		if (zcmpu(n, 1) <= 0) {
			if (witness)
				SET(witness, n);
			return NONPRIME;
		} else {
			return PRIME;
		}
	}
	if (zeven(n)) {
		if (witness)
			SET(witness, n);
		return NONPRIME;
	}

	zsub_unsigned(n1, n, libzahl_const_1);
	zsub_unsigned(n4, n, libzahl_const_4);

	r = zlsb(n1);
	zrsh(d, n1, r);

	while (t--) {
		zrand(a, FAST_RANDOM, UNIFORM, n4);
		zadd_unsigned(a, a, libzahl_const_2);
		zmodpow(x, a, d, n);

		if (!zcmp(x, libzahl_const_1) || !zcmp(x, n1))
			continue;

		for (i = 1; i < r; i++) {
			zsqr(x, x);
			zmod(x, x, n);
			if (!zcmp(x, libzahl_const_1)) {
				if (witness)
					zswap(witness, a);
				return NONPRIME;
			}
			if (!zcmp(x, n1))
				break;
		}
		if (i == r) {
			if (witness)
				zswap(witness, a);
			return NONPRIME;
		}
	}

	return PROBABLY_PRIME;
}
