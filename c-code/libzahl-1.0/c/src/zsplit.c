/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zsplit(z_t high, z_t low, z_t a, size_t delim)
{
	if (zzero(a)) {
		SET_SIGNUM(high, 0);
		SET_SIGNUM(low, 0);
		return;
	}

	if (high == a) {
		ztrunc(low, a, delim);
		zrsh(high, a, delim);
	} else {
		zrsh(high, a, delim);
		ztrunc(low, a, delim);
	}
}
