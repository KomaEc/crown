/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zunsetup(void)
{
	size_t i;
	if (libzahl_set_up) {
		libzahl_set_up = 0;
#define X(x)\
		free(x->chars);
		LIST_TEMPS;
#undef X
#define X(x, f, v)\
		free(x->chars);
		LIST_CONSTS;
#undef X
		for (i = BITS_PER_CHAR; i--;)
			free(libzahl_tmp_divmod_ds[i]->chars);

		for (i = sizeof(libzahl_pool) / sizeof(*libzahl_pool); i--;) {
			while (libzahl_pool_n[i]--)
				free(libzahl_pool[i][libzahl_pool_n[i]]);
			free(libzahl_pool[i]);
		}
	}
}
