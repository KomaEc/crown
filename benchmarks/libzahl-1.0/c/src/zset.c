/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zset(z_t a, z_t b)
{
	if (zzero(b)) {
		SET_SIGNUM(a, 0);
	} else {
		ENSURE_SIZE(a, b->used);
		a->sign = b->sign;
		a->used = b->used;
		zmemcpy(a->chars, b->chars, b->used);
	}
}
