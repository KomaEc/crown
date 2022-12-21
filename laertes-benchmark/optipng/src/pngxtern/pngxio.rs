
/* PNG_LIBPNG_VER < 10405 */
/*
 * pngxio.c - libpng extension: I/O state query.
 *
 * Copyright (C) 2003-2017 Cosmin Truta.
 * This software is distributed under the same licensing and warranty terms
 * as libpng.
 *
 * NOTE:
 * The functionality provided in this module has "graduated", and is now
 * part of libpng.  The original code, retrofitted as a back-port, has
 * limitations: it is thread-unsafe, and it only allows one png_ptr object
 * for reading and one for writing.
 *
 * CAUTION:
 * libpng-1.4.5 is the earliest version whose I/O state implementation
 * can be used reliably.
 */
