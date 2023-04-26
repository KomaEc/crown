/* See LICENSE file for copyright and license details. */
#include "internals.h"


int
zbtest(z_t a, size_t bit)
{
	size_t chars;
	if (zzero(a))
		return 0;

	chars = FLOOR_BITS_TO_CHARS(bit);
	if (chars >= a->used)
		return 0;

	bit = BITS_IN_LAST_CHAR(bit);
	return (a->chars[chars] >> bit) & 1;
}
