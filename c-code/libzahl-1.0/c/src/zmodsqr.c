/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zmodsqr(z_t a, z_t b, z_t c)
{
	/* TODO What is the fastest way to do zmodsqr? */
	if (a == c) {
		zset(libzahl_tmp_modsqr, c);
		zsqr(a, b);
		zmod(a, a, libzahl_tmp_modsqr);
	} else {
		zsqr(a, b);
		zmod(a, a, c);
	}
}
