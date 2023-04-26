/* See LICENSE file for copyright and license details. */
#include "internals.h"

#define X(x)  z_t x;
LIST_TEMPS
#undef X
#define X(x, f, v)  z_t x;
LIST_CONSTS
#undef X

z_t libzahl_tmp_divmod_ds[BITS_PER_CHAR];
jmp_buf libzahl_jmp_buf;
int libzahl_set_up = 0;
int libzahl_error;
zahl_char_t **libzahl_pool[sizeof(size_t) * 8];
size_t libzahl_pool_n[sizeof(size_t) * 8];
size_t libzahl_pool_alloc[sizeof(size_t) * 8];


void
zsetup(jmp_buf env)
{
	size_t i;
	*libzahl_jmp_buf = *env;

	if (!libzahl_set_up) {
		libzahl_set_up = 1;

		memset(libzahl_pool, 0, sizeof(libzahl_pool));
		memset(libzahl_pool_n, 0, sizeof(libzahl_pool_n));
		memset(libzahl_pool_alloc, 0, sizeof(libzahl_pool_alloc));

#define X(x)\
		zinit(x);
		LIST_TEMPS;
#undef X
#define X(x, f, v)\
		zinit(x), f(x, v);
		LIST_CONSTS;
#undef X
		for (i = BITS_PER_CHAR; i--;)
			zinit(libzahl_tmp_divmod_ds[i]);
	}
}
