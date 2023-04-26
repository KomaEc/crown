/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zsqr(z_t a, z_t b)
{
	/*
	 * Karatsuba algorithm, optimised for equal factors.
	 */

	size_t m2;
	z_t z0, z1, z2, high, low;
	int sign;

	if (zzero(b)) {
		SET_SIGNUM(a, 0);
		return;
	}

	m2 = zbits(b);

	if (m2 <= BITS_PER_CHAR / 2) {
		/* zsetu(a, b->chars[0] * b->chars[0]); { */
		ENSURE_SIZE(a, 1);
		a->used = 1;
		a->chars[0] = b->chars[0] * b->chars[0];
		/* } */
		SET_SIGNUM(a, 1);
		return;
	}

	sign = zsignum(b);
	SET_SIGNUM(b, 1);
	m2 >>= 1;

	zinit(z0);
	zinit(z1);
	zinit(z2);
	zinit(high);
	zinit(low);

	zsplit(high, low, b, m2);

#if 1
	zsqr(z0, low);
	zsqr(z2, high);
	zmul(z1, low, high);

	zlsh(z1, z1, m2 + 1);
	m2 <<= 1;
	zlsh(z2, z2, m2);

	zadd(a, z2, z1);
	zadd(a, a, z0);
#else
	zsqr(z0, low);
	zsqr(z2, high);
	zmul(z1, low, low);

	zlsh(z0, z0, m2 + 1);
	zlsh(z1, z1, m2 + 1);
	zlsh(a, z2, m2);
	m2 <<= 1;
	zlsh(z2, z2, m2);
	zadd(z2, z2, a);

	zsub(a, z2, z1);
	zadd(a, a, z0);
#endif

	zfree(z0);
	zfree(z1);
	zfree(z2);
	zfree(high);
	zfree(low);

	SET_SIGNUM(b, sign);
	SET_SIGNUM(a, 1);
}
