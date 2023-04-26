/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zmul(z_t a, z_t b, z_t c)
{
	/*
	 * Karatsuba algorithm
	 */

	size_t m, m2;
	z_t z0, z1, z2, b_high, b_low, c_high, c_low;
	int b_sign, c_sign;

	b_sign = zsignum(b);
	c_sign = zsignum(c);

	if (!b_sign || !c_sign) {
		SET_SIGNUM(a, 0);
		return;
	}

	m = zbits(b);
	m2 = b == c ? m : zbits(c);

	if (m + m2 <= BITS_PER_CHAR) {
		/* zsetu(a, b->chars[0] * c->chars[0]); { */
		ENSURE_SIZE(a, 1);
		a->used = 1;
		a->chars[0] = b->chars[0] * c->chars[0];
		/* } */
		SET_SIGNUM(a, b_sign * c_sign);
		return;
	}

	SET_SIGNUM(b, 1);
	SET_SIGNUM(c, 1);

        m = MAX(m, m2);
	m2 = m >> 1;

	zinit(z0);
	zinit(z1);
	zinit(z2);
	zinit(b_high);
	zinit(b_low);
	zinit(c_high);
	zinit(c_low);

	zsplit(b_high, b_low, b, m2);
	zsplit(c_high, c_low, c, m2);

#if 1
	zmul(z0, b_low, c_low);
	zmul(z2, b_high, c_high);
	zadd(b_low, b_low, b_high);
	zadd(c_low, c_low, c_high);
	zmul(z1, b_low, c_low);

	zsub(z1, z1, z0);
	zsub(z1, z1, z2);

	zlsh(z1, z1, m2);
	m2 <<= 1;
	zlsh(z2, z2, m2);

	zadd(a, z2, z1);
	zadd(a, a, z0);
#else
	zmul(z0, b_low, c_low);
	zmul(z2, b_high, c_high);
	zsub(b_low, b_high, b_low);
	zsub(c_low, c_high, c_low);
	zmul(z1, b_low, c_low);

	zlsh(z0, z0, m2 + 1);
	zlsh(z1, z1, m2);
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
	zfree(b_high);
	zfree(b_low);
	zfree(c_high);
	zfree(c_low);

	SET_SIGNUM(b, b_sign);
	SET_SIGNUM(c, c_sign);
	SET_SIGNUM(a, b_sign * c_sign);
}
