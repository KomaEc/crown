/* See LICENSE file for copyright and license details. */
#include "internals.h"


int
zcmpi(z_t a, long long int b)
{
	if (!b)
		return zsignum(a);
	if (zzero(a))
		return b > 0 ? -1 : b < 0;
	zseti(libzahl_tmp_cmp, b);
	return zcmp(a, libzahl_tmp_cmp);
}
