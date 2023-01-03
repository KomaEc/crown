/* See LICENSE file for copyright and license details. */
#include "internals.h"


size_t
zlsb(z_t a)
{
	size_t i = 0;
	zahl_char_t x;
	if (zzero(a))
		return SIZE_MAX;
	for (;; i++) {
		x = a->chars[i];
		if (x) {
			x = ~x;
			for (i *= BITS_PER_CHAR; x & 1; x >>= 1, i++);
			return i;
		}
	}
}
