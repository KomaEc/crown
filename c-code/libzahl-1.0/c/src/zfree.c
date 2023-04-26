/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zfree(z_t a)
{
	size_t i = 0, x, j;
	zahl_char_t **new;

	if (!a->chars)
		return;

	for (x = a->alloced; x; x >>= 1)
		i += 1;

	j = libzahl_pool_n[i]++;

	if (j == libzahl_pool_alloc[i]) {
		x = j ? ((j * 3) >> 1) : 128;
		new = realloc(libzahl_pool[i], x * sizeof(zahl_char_t *));
		if (!new) {
			free(a->chars);
			free(libzahl_pool[i]);
			libzahl_pool_n[i] = 0;
			libzahl_pool[i] = 0;
			libzahl_pool_alloc[i] = 0;
			return;
		}
		libzahl_pool[i] = new;
		libzahl_pool_alloc[i] = x;
	}

	libzahl_pool[i][j] = a->chars;
}
