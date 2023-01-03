/* See LICENSE file for copyright and license details. */
#include "internals.h"


int
zcmpmag(z_t a, z_t b)
{
	size_t i, j;
	if (zzero(a))
		return -!zzero(b);
	if (zzero(b))
		return 1;
	i = a->used - 1;
	j = b->used - 1;
	for (; i > j; i--) {
		if (a->chars[i])
			return +1;
		a->used--;
	}
	for (; j > i; j--) {
		if (b->chars[j])
			return -1;
		b->used--;
	}
	for (; i; i--)
		if (a->chars[i] != b->chars[i])
			return (a->chars[i] > b->chars[i]) * 2 - 1;
	return a->chars[0] < b->chars[0] ? -1 : a->chars[0] > b->chars[0];
}
