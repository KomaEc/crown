/* See LICENSE file for copyright and license details. */
#include "internals.h"


int
zcmp(z_t a, z_t b)
{
	if (zsignum(a) != zsignum(b))
		return zsignum(a) < zsignum(b) ? -1 : zsignum(a) > zsignum(b);
	return zsignum(a) * zcmpmag(a, b);
}
