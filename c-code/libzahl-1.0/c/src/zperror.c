/* See LICENSE file for copyright and license details. */
#include "internals.h"

#include <stdio.h>


void
zperror(const char *prefix)
{
	if (libzahl_error >= 0) {
		errno = libzahl_error;
		perror(prefix);
	} else {
		/* Current, we should not be able to get here. */
		abort();
	}
}
