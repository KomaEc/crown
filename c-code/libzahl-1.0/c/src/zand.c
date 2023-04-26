/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zand(z_t a, z_t b, z_t c)
{
	size_t n;

	if (zzero(b) || zzero(c)) {
		SET_SIGNUM(a, 0);
		return;
	}

	n = MIN(b->used, c->used);
	while (n--)
		if (b->chars[n] & c->chars[n])
			goto found_highest;
	SET_SIGNUM(a, 0);
	return;

found_highest:
	a->used = ++n;
	if (a == b) {
		while (n--)
			a->chars[n] &= c->chars[n];
	} else if (a == c) {
		while (n--)
			a->chars[n] &= b->chars[n];
	} else {
		ENSURE_SIZE(a, a->used);
		zmemcpy(a->chars, c->chars, a->used);
		while (n--)
			a->chars[n] &= b->chars[n];
	}
	SET_SIGNUM(a, (zsignum(b) > 0 || zsignum(c) > 0) * 2 - 1);
}
