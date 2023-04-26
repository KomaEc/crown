/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zdiv(z_t a, z_t b, z_t c)
{
	zdivmod(a, libzahl_tmp_div, b, c);
}
