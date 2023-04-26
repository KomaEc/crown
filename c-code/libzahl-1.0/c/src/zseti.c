/* See LICENSE file for copyright and license details. */
#include "internals.h"


void
zseti(z_t a, long long int b)
{
	if (b >= 0) {
		zsetu(a, (unsigned long long int)b);
	} else {
		zsetu(a, (unsigned long long int)-b);
		SET_SIGNUM(a, -1);
	}
}
