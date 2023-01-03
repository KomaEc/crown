/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zmod(z_t a, z_t b, z_t c)
{
	zdivmod(libzahl_tmp_mod, a, b, c);
}
