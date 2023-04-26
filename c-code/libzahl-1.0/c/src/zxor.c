/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zxor(z_t a, z_t b, z_t c)
{
	size_t n, m;

	if (zzero(b)) {
		if (zzero(c))
			SET_SIGNUM(a, 0);
		else
			SET(a, c);
		return;
	} else if (zzero(c)) {
		SET(a, b);
		return;
	}

	m = MAX(b->used, c->used);
	n = b->used + c->used - m;

	ENSURE_SIZE(a, m);

	if (a == b) {
		if (b->used < c->used)
			zmemcpy(a->chars + n, c->chars + n, m - n);
		while (n--)
			a->chars[n] ^= c->chars[n];
	} else if (a == c) {
		if (c->used < b->used)
			zmemcpy(a->chars + n, b->chars + n, m - n);
		while (n--)
			a->chars[n] ^= b->chars[n];
	} else if (m == b->used) {
		zmemcpy(a->chars, b->chars, m);
		while (n--)
			a->chars[n] ^= c->chars[n];
	} else {
		zmemcpy(a->chars, c->chars, m);
		while (n--)
			a->chars[n] ^= b->chars[n];
	}

	a->used = m;
	while (a->used && !a->chars[a->used - 1])
		a->used--;
	if (a->used)
		SET_SIGNUM(a, 1 - 2 * ((zsignum(b) ^ zsignum(c)) < 0));
	else
		SET_SIGNUM(a, 0);
}
