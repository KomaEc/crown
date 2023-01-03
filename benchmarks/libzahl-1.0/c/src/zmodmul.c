/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zmodmul(z_t a, z_t b, z_t c, z_t d)
{
	/* TODO Montgomery modular multiplication */
	if (a == d) {
		zset(libzahl_tmp_modmul, d);
		zmul(a, b, c);
		zmod(a, a, libzahl_tmp_modmul);
	} else {
		zmul(a, b, c);
		zmod(a, a, d);
	}
}
