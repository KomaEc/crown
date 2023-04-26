/* See LICENSE file for copyright and license details. */
#include "internals.h"


enum zerror
zerror(const char **desc)
{
	if (libzahl_error >= 0) {
		if (desc)
			*desc = strerror(libzahl_error);
		errno = libzahl_error;
		return ZERROR_ERRNO_SET;
	} else {
		/* Current, we should not be able to get here. */
		if (desc)
			abort();
		return -libzahl_error;
	}
}
