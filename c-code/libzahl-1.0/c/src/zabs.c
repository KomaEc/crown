/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zabs(z_t a, z_t b)
{
	SET(a, b);
	SET_SIGNUM(a, !zzero(a));
}
