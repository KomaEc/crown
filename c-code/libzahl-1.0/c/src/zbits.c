/* See LICENSE file for copyright and license details. */
#include "internals.h"


size_t
zbits(z_t a)
{
	size_t i;
	zahl_char_t x;
	if (zzero(a))
		return 1; /* Deliver us from evil! */
	for (i = a->used - 1;; i--) {
		x = a->chars[i];
		if (x) {
			a->used = i + 1;
			for (i *= BITS_PER_CHAR; x; x >>= 1, i++);
			return i;
		}
	}
}
