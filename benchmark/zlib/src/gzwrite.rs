use ::libc;
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    /* The application can compare zlibVersion and ZLIB_VERSION for consistency.
      If the first character differs, the library code actually used is not
      compatible with the zlib.h header file used by the application.  This check
      is automatically made by deflateInit and inflateInit.
    */
    /*
    ZEXTERN int ZEXPORT deflateInit OF((z_streamp strm, int level));

         Initializes the internal stream state for compression.  The fields
       zalloc, zfree and opaque must be initialized before by the caller.  If
       zalloc and zfree are set to Z_NULL, deflateInit updates them to use default
       allocation functions.

         The compression level must be Z_DEFAULT_COMPRESSION, or between 0 and 9:
       1 gives best speed, 9 gives best compression, 0 gives no compression at all
       (the input data is simply copied a block at a time).  Z_DEFAULT_COMPRESSION
       requests a default compromise between speed and compression (currently
       equivalent to level 6).

         deflateInit returns Z_OK if success, Z_MEM_ERROR if there was not enough
       memory, Z_STREAM_ERROR if level is not a valid compression level, or
       Z_VERSION_ERROR if the zlib library version (zlib_version) is incompatible
       with the version assumed by the caller (ZLIB_VERSION).  msg is set to null
       if there is no error message.  deflateInit does not perform any compression:
       this will be done by deflate().
    */
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    /*
        deflate compresses as much data as possible, and stops when the input
      buffer becomes empty or the output buffer becomes full.  It may introduce
      some output latency (reading input without producing any output) except when
      forced to flush.

        The detailed semantics are as follows.  deflate performs one or both of the
      following actions:

      - Compress more input starting at next_in and update next_in and avail_in
        accordingly.  If not all input can be processed (because there is not
        enough room in the output buffer), next_in and avail_in are updated and
        processing will resume at this point for the next call of deflate().

      - Generate more output starting at next_out and update next_out and avail_out
        accordingly.  This action is forced if the parameter flush is non zero.
        Forcing flush frequently degrades the compression ratio, so this parameter
        should be set only when necessary.  Some output may be provided even if
        flush is zero.

        Before the call of deflate(), the application should ensure that at least
      one of the actions is possible, by providing more input and/or consuming more
      output, and updating avail_in or avail_out accordingly; avail_out should
      never be zero before the call.  The application can consume the compressed
      output when it wants, for example when the output buffer is full (avail_out
      == 0), or after each call of deflate().  If deflate returns Z_OK and with
      zero avail_out, it must be called again after making room in the output
      buffer because there might be more output pending. See deflatePending(),
      which can be used if desired to determine whether or not there is more ouput
      in that case.

        Normally the parameter flush is set to Z_NO_FLUSH, which allows deflate to
      decide how much data to accumulate before producing output, in order to
      maximize compression.

        If the parameter flush is set to Z_SYNC_FLUSH, all pending output is
      flushed to the output buffer and the output is aligned on a byte boundary, so
      that the decompressor can get all input data available so far.  (In
      particular avail_in is zero after the call if enough output space has been
      provided before the call.) Flushing may degrade compression for some
      compression algorithms and so it should be used only when necessary.  This
      completes the current deflate block and follows it with an empty stored block
      that is three bits plus filler bits to the next byte, followed by four bytes
      (00 00 ff ff).

        If flush is set to Z_PARTIAL_FLUSH, all pending output is flushed to the
      output buffer, but the output is not aligned to a byte boundary.  All of the
      input data so far will be available to the decompressor, as for Z_SYNC_FLUSH.
      This completes the current deflate block and follows it with an empty fixed
      codes block that is 10 bits long.  This assures that enough bytes are output
      in order for the decompressor to finish the block before the empty fixed
      codes block.

        If flush is set to Z_BLOCK, a deflate block is completed and emitted, as
      for Z_SYNC_FLUSH, but the output is not aligned on a byte boundary, and up to
      seven bits of the current block are held to be written as the next byte after
      the next deflate block is completed.  In this case, the decompressor may not
      be provided enough bits at this point in order to complete decompression of
      the data provided so far to the compressor.  It may need to wait for the next
      block to be emitted.  This is for advanced applications that need to control
      the emission of deflate blocks.

        If flush is set to Z_FULL_FLUSH, all output is flushed as with
      Z_SYNC_FLUSH, and the compression state is reset so that decompression can
      restart from this point if previous compressed data has been damaged or if
      random access is desired.  Using Z_FULL_FLUSH too often can seriously degrade
      compression.

        If deflate returns with avail_out == 0, this function must be called again
      with the same value of the flush parameter and more output space (updated
      avail_out), until the flush is complete (deflate returns with non-zero
      avail_out).  In the case of a Z_FULL_FLUSH or Z_SYNC_FLUSH, make sure that
      avail_out is greater than six to avoid repeated flush markers due to
      avail_out == 0 on return.

        If the parameter flush is set to Z_FINISH, pending input is processed,
      pending output is flushed and deflate returns with Z_STREAM_END if there was
      enough output space.  If deflate returns with Z_OK or Z_BUF_ERROR, this
      function must be called again with Z_FINISH and more output space (updated
      avail_out) but no more input data, until it returns with Z_STREAM_END or an
      error.  After deflate has returned Z_STREAM_END, the only possible operations
      on the stream are deflateReset or deflateEnd.

        Z_FINISH can be used in the first deflate call after deflateInit if all the
      compression is to be done in a single step.  In order to complete in one
      call, avail_out must be at least the value returned by deflateBound (see
      below).  Then deflate is guaranteed to return Z_STREAM_END.  If not enough
      output space is provided, deflate will not return Z_STREAM_END, and it must
      be called again as described above.

        deflate() sets strm->adler to the Adler-32 checksum of all input read
      so far (that is, total_in bytes).  If a gzip stream is being generated, then
      strm->adler will be the CRC-32 checksum of the input read so far.  (See
      deflateInit2 below.)

        deflate() may update strm->data_type if it can make a good guess about
      the input data type (Z_BINARY or Z_TEXT).  If in doubt, the data is
      considered binary.  This field is only for information purposes and does not
      affect the compression algorithm in any manner.

        deflate() returns Z_OK if some progress has been made (more input
      processed or more output produced), Z_STREAM_END if all input has been
      consumed and all output has been produced (only when flush is set to
      Z_FINISH), Z_STREAM_ERROR if the stream state was inconsistent (for example
      if next_in or next_out was Z_NULL or the state was inadvertently written over
      by the application), or Z_BUF_ERROR if no progress is possible (for example
      avail_in or avail_out was zero).  Note that Z_BUF_ERROR is not fatal, and
      deflate() can be called again with more input and more output space to
      continue compressing.
    */
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    /*
         Sets the destination stream as a complete copy of the source stream.

         This function can be useful when several compression strategies will be
       tried, for example when there are several ways of pre-processing the input
       data with a filter.  The streams that will be discarded should then be freed
       by calling deflateEnd.  Note that deflateCopy duplicates the internal
       compression state which can be quite large, so this strategy is slow and can
       consume lots of memory.

         deflateCopy returns Z_OK if success, Z_MEM_ERROR if there was not
       enough memory, Z_STREAM_ERROR if the source stream state was inconsistent
       (such as zalloc being Z_NULL).  msg is left unchanged in both source and
       destination.
    */
    #[no_mangle]
    fn deflateReset(strm: z_streamp) -> libc::c_int;
    /*
         This function is equivalent to deflateEnd followed by deflateInit, but
       does not free and reallocate the internal compression state.  The stream
       will leave the compression level and any other attributes that may have been
       set unchanged.

         deflateReset returns Z_OK if success, or Z_STREAM_ERROR if the source
       stream state was inconsistent (such as zalloc or state being Z_NULL).
    */
    #[no_mangle]
    fn deflateParams(strm: z_streamp, level: libc::c_int, strategy: libc::c_int) -> libc::c_int;
    /* shared functions */
    #[no_mangle]
    fn gz_error(_: gz_statep, _: libc::c_int, _: *const libc::c_char);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn deflateInit2_(
        strm: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type va_list = __builtin_va_list;
pub type off64_t = __off64_t;
pub type ssize_t = __ssize_t;
/* zconf.h -- configuration of the zlib compression library
 * Copyright (C) 1995-2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* #undef Z_PREFIX */
/*
 * If you *really* need a unique prefix for all types and library functions,
 * compile with -DZ_PREFIX. The "standard" zlib should be compiled without it.
 * Even better than compiling with -DZ_PREFIX would be to use configure to set
 * this permanently in zconf.h using "./configure --zprefix".
 */
/* may be set to #if 1 by ./configure */
/*
 * Compile with -DMAXSEG_64K if the alloc function cannot allocate more
 * than 64k bytes at a time (needed on systems with 16-bit int).
 */
/* iSeries (formerly AS/400). */
pub type z_size_t = size_t;
/* Maximum value for memLevel in deflateInit2 */
/* Maximum value for windowBits in deflateInit2 and inflateInit2.
 * WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
 * created by gzip. (Files created by minigzip can still be extracted by
 * gzip.)
 */
/* 32K LZ77 window */
/* The memory requirements for deflate are (in bytes):
            (1 << (windowBits+2)) +  (1 << (memLevel+9))
 that is: 128K for windowBits=15  +  128K for memLevel = 8  (default values)
 plus a few kilobytes for small objects. For example, if you want to reduce
 the default memory requirements from 256K to 128K, compile with
     make CFLAGS="-O -DMAX_WBITS=14 -DMAX_MEM_LEVEL=7"
 Of course this will generally degrade compression (there's no free lunch).

   The memory requirements for inflate are (in bytes) 1 << windowBits
 that is, 32K for windowBits=15 (default value) plus about 7 kilobytes
 for small objects.
*/
/* Type declarations */
/* function prototypes */
/* function prototypes for stdarg */
/* The following definitions for FAR are needed only for MSDOS mixed
 * model programming (small or medium model with some far allocations).
 * This was tested only with MSC; for other MSDOS compilers you may have
 * to define NO_MEMCPY in zutil.h.  If you don't need the mixed model,
 * just define FAR to be empty.
 */
pub type Byte = libc::c_uchar;
/* 8 bits */
pub type uInt = libc::c_uint;
/* 16 bits or more */
pub type uLong = libc::c_ulong;
/* 32 bits or more */
pub type Bytef = Byte;
pub type voidpc = *const libc::c_void;
pub type voidpf = *mut libc::c_void;
/* zlib.h -- interface of the 'zlib' general purpose compression library
  version 1.2.11, January 15th, 2017

  Copyright (C) 1995-2017 Jean-loup Gailly and Mark Adler

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

  Jean-loup Gailly        Mark Adler
  jloup@gzip.org          madler@alumni.caltech.edu


  The data format used by the zlib library is described by RFCs (Request for
  Comments) 1950 to 1952 in the files http://tools.ietf.org/html/rfc1950
  (zlib format), rfc1951 (deflate format) and rfc1952 (gzip format).
*/
/*
    The 'zlib' compression library provides in-memory compression and
  decompression functions, including integrity checks of the uncompressed data.
  This version of the library supports only one compression method (deflation)
  but other algorithms will be added later and will have the same stream
  interface.

    Compression can be done in a single step if the buffers are large enough,
  or can be done by repeated calls of the compression function.  In the latter
  case, the application must provide more input and/or consume the output
  (providing more output space) before each call.

    The compressed data format used by default by the in-memory functions is
  the zlib format, which is a zlib wrapper documented in RFC 1950, wrapped
  around a deflate stream, which is itself documented in RFC 1951.

    The library also supports reading and writing files in gzip (.gz) format
  with an interface similar to that of stdio using the functions that start
  with "gz".  The gzip format is different from the zlib format.  gzip is a
  gzip wrapper, documented in RFC 1952, wrapped around a deflate stream.

    This library can optionally read and write gzip and raw deflate streams in
  memory as well.

    The zlib format was designed to be compact and fast for use in memory
  and on communications channels.  The gzip format was designed for single-
  file compression on file systems, has a larger header than zlib to maintain
  directory information, and uses a different, slower check method than zlib.

    The library does not install any signal handler.  The decoder checks
  the consistency of the compressed data, so the library should never crash
  even in the case of corrupted input.
*/
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
/* reserved for future use */
/*
     Same as uncompress, except that sourceLen is a pointer, where the
   length of the source is *sourceLen.  On return, *sourceLen is the number of
   source bytes consumed.
*/
/* gzip file access functions */
/*
     This library supports reading and writing files in gzip (.gz) format with
   an interface similar to that of stdio, using the functions that start with
   "gz".  The gzip format is different from the zlib format.  gzip is a gzip
   wrapper, documented in RFC 1952, wrapped around a deflate stream.
*/
/* semi-opaque gzip file descriptor */
/*
ZEXTERN gzFile ZEXPORT gzopen OF((const char *path, const char *mode));

     Opens a gzip (.gz) file for reading or writing.  The mode parameter is as
   in fopen ("rb" or "wb") but can also include a compression level ("wb9") or
   a strategy: 'f' for filtered data as in "wb6f", 'h' for Huffman-only
   compression as in "wb1h", 'R' for run-length encoding as in "wb1R", or 'F'
   for fixed code compression as in "wb9F".  (See the description of
   deflateInit2 for more information about the strategy parameter.)  'T' will
   request transparent writing or appending with no compression and not using
   the gzip format.

     "a" can be used instead of "w" to request that the gzip stream that will
   be written be appended to the file.  "+" will result in an error, since
   reading and writing to the same gzip file is not supported.  The addition of
   "x" when writing will create the file exclusively, which fails if the file
   already exists.  On systems that support it, the addition of "e" when
   reading or writing will set the flag to close the file on an execve() call.

     These functions, as well as gzip, will read and decode a sequence of gzip
   streams in a file.  The append function of gzopen() can be used to create
   such a file.  (Also see gzflush() for another way to do this.)  When
   appending, gzopen does not test whether the file begins with a gzip stream,
   nor does it look for the end of the gzip streams to begin appending.  gzopen
   will simply append a gzip stream to the existing file.

     gzopen can be used to read a file which is not in gzip format; in this
   case gzread will directly read from the file without decompression.  When
   reading, this will be detected automatically by looking for the magic two-
   byte gzip header.

     gzopen returns NULL if the file could not be opened, if there was
   insufficient memory to allocate the gzFile state, or if an invalid mode was
   specified (an 'r', 'w', or 'a' was not provided, or '+' was provided).
   errno can be checked to determine if the reason gzopen failed was that the
   file could not be opened.
*/
/*
     gzdopen associates a gzFile with the file descriptor fd.  File descriptors
   are obtained from calls like open, dup, creat, pipe or fileno (if the file
   has been previously opened with fopen).  The mode parameter is as in gzopen.

     The next call of gzclose on the returned gzFile will also close the file
   descriptor fd, just like fclose(fdopen(fd, mode)) closes the file descriptor
   fd.  If you want to keep fd open, use fd = dup(fd_keep); gz = gzdopen(fd,
   mode);.  The duplicated descriptor should be saved to avoid a leak, since
   gzdopen does not close fd if it fails.  If you are using fileno() to get the
   file descriptor from a FILE *, then you will have to use dup() to avoid
   double-close()ing the file descriptor.  Both gzclose() and fclose() will
   close the associated file descriptor, so they need to have different file
   descriptors.

     gzdopen returns NULL if there was insufficient memory to allocate the
   gzFile state, if an invalid mode was specified (an 'r', 'w', or 'a' was not
   provided, or '+' was provided), or if fd is -1.  The file descriptor is not
   used until the next gz* read, write, seek, or close operation, so gzdopen
   will not detect if fd is invalid (unless fd is -1).
*/
/*
     Set the internal buffer size used by this library's functions.  The
   default buffer size is 8192 bytes.  This function must be called after
   gzopen() or gzdopen(), and before any other calls that read or write the
   file.  The buffer memory allocation is always deferred to the first read or
   write.  Three times that size in buffer space is allocated.  A larger buffer
   size of, for example, 64K or 128K bytes will noticeably increase the speed
   of decompression (reading).

     The new buffer size also affects the maximum length for gzprintf().

     gzbuffer() returns 0 on success, or -1 on failure, such as being called
   too late.
*/
/*
     Dynamically update the compression level or strategy.  See the description
   of deflateInit2 for the meaning of these parameters.  Previously provided
   data is flushed before the parameter change.

     gzsetparams returns Z_OK if success, Z_STREAM_ERROR if the file was not
   opened for writing, Z_ERRNO if there is an error writing the flushed data,
   or Z_MEM_ERROR if there is a memory allocation error.
*/
/*
     Reads the given number of uncompressed bytes from the compressed file.  If
   the input file is not in gzip format, gzread copies the given number of
   bytes into the buffer directly from the file.

     After reaching the end of a gzip stream in the input, gzread will continue
   to read, looking for another gzip stream.  Any number of gzip streams may be
   concatenated in the input file, and will all be decompressed by gzread().
   If something other than a gzip stream is encountered after a gzip stream,
   that remaining trailing garbage is ignored (and no error is returned).

     gzread can be used to read a gzip file that is being concurrently written.
   Upon reaching the end of the input, gzread will return with the available
   data.  If the error code returned by gzerror is Z_OK or Z_BUF_ERROR, then
   gzclearerr can be used to clear the end of file indicator in order to permit
   gzread to be tried again.  Z_OK indicates that a gzip stream was completed
   on the last gzread.  Z_BUF_ERROR indicates that the input file ended in the
   middle of a gzip stream.  Note that gzread does not return -1 in the event
   of an incomplete gzip stream.  This error is deferred until gzclose(), which
   will return Z_BUF_ERROR if the last gzread ended in the middle of a gzip
   stream.  Alternatively, gzerror can be used before gzclose to detect this
   case.

     gzread returns the number of uncompressed bytes actually read, less than
   len for end of file, or -1 for error.  If len is too large to fit in an int,
   then nothing is read, -1 is returned, and the error state is set to
   Z_STREAM_ERROR.
*/
/*
     Read up to nitems items of size size from file to buf, otherwise operating
   as gzread() does.  This duplicates the interface of stdio's fread(), with
   size_t request and return types.  If the library defines size_t, then
   z_size_t is identical to size_t.  If not, then z_size_t is an unsigned
   integer type that can contain a pointer.

     gzfread() returns the number of full items read of size size, or zero if
   the end of the file was reached and a full item could not be read, or if
   there was an error.  gzerror() must be consulted if zero is returned in
   order to determine if there was an error.  If the multiplication of size and
   nitems overflows, i.e. the product does not fit in a z_size_t, then nothing
   is read, zero is returned, and the error state is set to Z_STREAM_ERROR.

     In the event that the end of file is reached and only a partial item is
   available at the end, i.e. the remaining uncompressed data length is not a
   multiple of size, then the final partial item is nevetheless read into buf
   and the end-of-file flag is set.  The length of the partial item read is not
   provided, but could be inferred from the result of gztell().  This behavior
   is the same as the behavior of fread() implementations in common libraries,
   but it prevents the direct use of gzfread() to read a concurrently written
   file, reseting and retrying on end-of-file, when size is not 1.
*/
/*
     Writes the given number of uncompressed bytes into the compressed file.
   gzwrite returns the number of uncompressed bytes written or 0 in case of
   error.
*/
/*
     gzfwrite() writes nitems items of size size from buf to file, duplicating
   the interface of stdio's fwrite(), with size_t request and return types.  If
   the library defines size_t, then z_size_t is identical to size_t.  If not,
   then z_size_t is an unsigned integer type that can contain a pointer.

     gzfwrite() returns the number of full items written of size size, or zero
   if there was an error.  If the multiplication of size and nitems overflows,
   i.e. the product does not fit in a z_size_t, then nothing is written, zero
   is returned, and the error state is set to Z_STREAM_ERROR.
*/
/*
     Converts, formats, and writes the arguments to the compressed file under
   control of the format string, as in fprintf.  gzprintf returns the number of
   uncompressed bytes actually written, or a negative zlib error code in case
   of error.  The number of uncompressed bytes written is limited to 8191, or
   one less than the buffer size given to gzbuffer().  The caller should assure
   that this limit is not exceeded.  If it is exceeded, then gzprintf() will
   return an error (0) with nothing written.  In this case, there may also be a
   buffer overflow with unpredictable consequences, which is possible only if
   zlib was compiled with the insecure functions sprintf() or vsprintf()
   because the secure snprintf() or vsnprintf() functions were not available.
   This can be determined using zlibCompileFlags().
*/
/*
     Writes the given null-terminated string to the compressed file, excluding
   the terminating null character.

     gzputs returns the number of characters written, or -1 in case of error.
*/
/*
     Reads bytes from the compressed file until len-1 characters are read, or a
   newline character is read and transferred to buf, or an end-of-file
   condition is encountered.  If any characters are read or if len == 1, the
   string is terminated with a null character.  If no characters are read due
   to an end-of-file or len < 1, then the buffer is left untouched.

     gzgets returns buf which is a null-terminated string, or it returns NULL
   for end-of-file or in case of error.  If there was an error, the contents at
   buf are indeterminate.
*/
/*
     Writes c, converted to an unsigned char, into the compressed file.  gzputc
   returns the value that was written, or -1 in case of error.
*/
/*
     Reads one byte from the compressed file.  gzgetc returns this byte or -1
   in case of end of file or error.  This is implemented as a macro for speed.
   As such, it does not do all of the checking the other functions do.  I.e.
   it does not check to see if file is NULL, nor whether the structure file
   points to has been clobbered or not.
*/
/*
     Push one character back onto the stream to be read as the first character
   on the next read.  At least one character of push-back is allowed.
   gzungetc() returns the character pushed, or -1 on failure.  gzungetc() will
   fail if c is -1, and may fail if a character has been pushed but not read
   yet.  If gzungetc is used immediately after gzopen or gzdopen, at least the
   output buffer size of pushed characters is allowed.  (See gzbuffer above.)
   The pushed character will be discarded if the stream is repositioned with
   gzseek() or gzrewind().
*/
/*
     Flushes all pending output into the compressed file.  The parameter flush
   is as in the deflate() function.  The return value is the zlib error number
   (see function gzerror below).  gzflush is only permitted when writing.

     If the flush parameter is Z_FINISH, the remaining data is written and the
   gzip stream is completed in the output.  If gzwrite() is called again, a new
   gzip stream will be started in the output.  gzread() is able to read such
   concatenated gzip streams.

     gzflush should be called only when strictly necessary because it will
   degrade compression if called too often.
*/
/*
ZEXTERN z_off_t ZEXPORT gzseek OF((gzFile file,
                                   z_off_t offset, int whence));

     Sets the starting position for the next gzread or gzwrite on the given
   compressed file.  The offset represents a number of bytes in the
   uncompressed data stream.  The whence parameter is defined as in lseek(2);
   the value SEEK_END is not supported.

     If the file is opened for reading, this function is emulated but can be
   extremely slow.  If the file is opened for writing, only forward seeks are
   supported; gzseek then compresses a sequence of zeroes up to the new
   starting position.

     gzseek returns the resulting offset location as measured in bytes from
   the beginning of the uncompressed stream, or -1 in case of error, in
   particular if the file is opened for writing and the new starting position
   would be before the current position.
*/
/*
     Rewinds the given file. This function is supported only for reading.

     gzrewind(file) is equivalent to (int)gzseek(file, 0L, SEEK_SET)
*/
/*
ZEXTERN z_off_t ZEXPORT    gztell OF((gzFile file));

     Returns the starting position for the next gzread or gzwrite on the given
   compressed file.  This position represents a number of bytes in the
   uncompressed data stream, and is zero when starting, even if appending or
   reading a gzip stream from the middle of a file using gzdopen().

     gztell(file) is equivalent to gzseek(file, 0L, SEEK_CUR)
*/
/*
ZEXTERN z_off_t ZEXPORT gzoffset OF((gzFile file));

     Returns the current offset in the file being read or written.  This offset
   includes the count of bytes that precede the gzip stream, for example when
   appending or when using gzdopen() for reading.  When reading, the offset
   does not include as yet unused buffered input.  This information can be used
   for a progress indicator.  On error, gzoffset() returns -1.
*/
/*
     Returns true (1) if the end-of-file indicator has been set while reading,
   false (0) otherwise.  Note that the end-of-file indicator is set only if the
   read tried to go past the end of the input, but came up short.  Therefore,
   just like feof(), gzeof() may return false even if there is no more data to
   read, in the event that the last read request was for the exact number of
   bytes remaining in the input file.  This will happen if the input file size
   is an exact multiple of the buffer size.

     If gzeof() returns true, then the read functions will return no more data,
   unless the end-of-file indicator is reset by gzclearerr() and the input file
   has grown since the previous end of file was detected.
*/
/*
     Returns true (1) if file is being copied directly while reading, or false
   (0) if file is a gzip stream being decompressed.

     If the input file is empty, gzdirect() will return true, since the input
   does not contain a gzip stream.

     If gzdirect() is used immediately after gzopen() or gzdopen() it will
   cause buffers to be allocated to allow reading the file to determine if it
   is a gzip file.  Therefore if gzbuffer() is used, it should be called before
   gzdirect().

     When writing, gzdirect() returns true (1) if transparent writing was
   requested ("wT" for the gzopen() mode), or false (0) otherwise.  (Note:
   gzdirect() is not needed when writing.  Transparent writing must be
   explicitly requested, so the application already knows the answer.  When
   linking statically, using gzdirect() will include all of the zlib code for
   gzip file reading and decompression, which may not be desired.)
*/
/*
     Flushes all pending output if necessary, closes the compressed file and
   deallocates the (de)compression state.  Note that once file is closed, you
   cannot call gzerror with file, since its structures have been deallocated.
   gzclose must not be called more than once on the same file, just as free
   must not be called more than once on the same allocation.

     gzclose will return Z_STREAM_ERROR if file is not valid, Z_ERRNO on a
   file operation error, Z_MEM_ERROR if out of memory, Z_BUF_ERROR if the
   last read ended in the middle of a gzip stream, or Z_OK on success.
*/
/*
     Same as gzclose(), but gzclose_r() is only for use when reading, and
   gzclose_w() is only for use when writing or appending.  The advantage to
   using these instead of gzclose() is that they avoid linking in zlib
   compression or decompression code that is not used when only reading or only
   writing respectively.  If gzclose() is used, then both compression and
   decompression code will be included the application when linking to a static
   zlib library.
*/
/*
     Returns the error message for the last error which occurred on the given
   compressed file.  errnum is set to zlib error number.  If an error occurred
   in the file system and not in the compression library, errnum is set to
   Z_ERRNO and the application may consult errno to get the exact error code.

     The application must not modify the returned string.  Future calls to
   this function may invalidate the previously returned string.  If file is
   closed, then the string previously returned by gzerror will no longer be
   available.

     gzerror() should be used to distinguish errors from end-of-file for those
   functions above that do not distinguish those cases in their return values.
*/
/*
     Clears the error and end-of-file flags for file.  This is analogous to the
   clearerr() function in stdio.  This is useful for continuing to read a gzip
   file that is being written concurrently.
*/
/* !Z_SOLO */
/* checksum functions */
/*
     These functions are not related to compression but are exported
   anyway because they might be useful in applications using the compression
   library.
*/
/*
     Update a running Adler-32 checksum with the bytes buf[0..len-1] and
   return the updated checksum.  If buf is Z_NULL, this function returns the
   required initial value for the checksum.

     An Adler-32 checksum is almost as reliable as a CRC-32 but can be computed
   much faster.

   Usage example:

     uLong adler = adler32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       adler = adler32(adler, buffer, length);
     }
     if (adler != original_adler) error();
*/
/*
     Same as adler32(), but with a size_t length.
*/
/*
ZEXTERN uLong ZEXPORT adler32_combine OF((uLong adler1, uLong adler2,
                                          z_off_t len2));

     Combine two Adler-32 checksums into one.  For two sequences of bytes, seq1
   and seq2 with lengths len1 and len2, Adler-32 checksums were calculated for
   each, adler1 and adler2.  adler32_combine() returns the Adler-32 checksum of
   seq1 and seq2 concatenated, requiring only adler1, adler2, and len2.  Note
   that the z_off_t type (like off_t) is a signed integer.  If len2 is
   negative, the result has no meaning or utility.
*/
/*
     Update a running CRC-32 with the bytes buf[0..len-1] and return the
   updated CRC-32.  If buf is Z_NULL, this function returns the required
   initial value for the crc.  Pre- and post-conditioning (one's complement) is
   performed within this function so it shouldn't be done by the application.

   Usage example:

     uLong crc = crc32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       crc = crc32(crc, buffer, length);
     }
     if (crc != original_crc) error();
*/
/*
     Same as crc32(), but with a size_t length.
*/
/*
ZEXTERN uLong ZEXPORT crc32_combine OF((uLong crc1, uLong crc2, z_off_t len2));

     Combine two CRC-32 check values into one.  For two sequences of bytes,
   seq1 and seq2 with lengths len1 and len2, CRC-32 check values were
   calculated for each, crc1 and crc2.  crc32_combine() returns the CRC-32
   check value of seq1 and seq2 concatenated, requiring only crc1, crc2, and
   len2.
*/
/* various hacks, don't look :) */
/* deflateInit and inflateInit are macros to allow checking the zlib version
 * and the compiler's view of z_stream:
 */
/* gzgetc() macro and its supporting function and exposed data structure.  Note
 * that the real internal state is much larger than the exposed structure.
 * This abbreviated structure exposes just enough for the gzgetc() macro.  The
 * user should not mess with these exposed elements, since their names or
 * behavior could change in the future, perhaps even capriciously.  They can
 * only be used by the gzgetc() macro.  You have been warned.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
/* gzguts.h -- zlib internal header definitions for gz* operations
 * Copyright (C) 2004, 2005, 2010, 2011, 2012, 2013, 2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* for compatibility with old definition */
/* unlike snprintf (which is required in C99), _snprintf does not guarantee
null termination of the result -- however this is only used in gzlib.c where
the result is assured to fit in the space provided */
/* since "static" is used to mean two completely different things in C, we
define "local" for the non-static meaning of "static", for readability
(compile with -Dlocal if your debugger can't find static symbols) */
/* gz* functions always use library allocation functions */
/* get errno and strerror definition */
/* provide prototypes for these when building zlib without LFS */
/* default memLevel */
/* default i/o buffer size -- double this for output when reading (this and
twice this must be able to fit in an unsigned type) */
/* gzip modes, also provide a little integrity check on the passed structure */
/* mode set to GZ_WRITE after the file is opened */
/* values for gz_state how */
/* look for a gzip header */
/* copy input directly */
/* decompress a gzip stream */
/* internal gzip file state data structure */
/* exposed contents for gzgetc() macro */
/* "x" for exposed */
/* x.have: number of bytes available at x.next */
/* x.next: next output data to deliver or write */
/* x.pos: current position in uncompressed data */
/* used for both reading and writing */
/* see gzip modes above */
/* file descriptor */
/* path or fd for error messages */
/* buffer size, zero if not allocated yet */
/* requested buffer size, default is GZBUFSIZE */
/* input buffer (double-sized when writing) */
/* output buffer (double-sized when reading) */
/* 0 if processing gzip, 1 if transparent */
/* just for reading */
/* 0: get header, 1: copy, 2: decompress */
/* where the gzip data started, for rewinding */
/* true if end of input file reached */
/* true if read requested past end */
/* just for writing */
/* compression level */
/* compression strategy */
/* seek request */
/* amount to skip (already rewound if backwards) */
/* true if seek request pending */
/* error information */
/* error code */
/* error message */
/* zlib inflate or deflate stream */
/* stream structure in-place (not a pointer) */
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: libc::c_int,
    pub fd: libc::c_int,
    pub path: *mut libc::c_char,
    pub size: libc::c_uint,
    pub want: libc::c_uint,
    pub in_0: *mut libc::c_uchar,
    pub out: *mut libc::c_uchar,
    pub direct: libc::c_int,
    pub how: libc::c_int,
    pub start: off64_t,
    pub eof: libc::c_int,
    pub past: libc::c_int,
    pub level: libc::c_int,
    pub strategy: libc::c_int,
    pub skip: off64_t,
    pub seek: libc::c_int,
    pub err: libc::c_int,
    pub msg: *mut libc::c_char,
    pub strm: z_stream,
}
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_BLOCK: libc::c_int = 5 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_STREAM_ERROR: libc::c_int = -(2 as libc::c_int);
pub const errno: libc::c_int = *__errno_location();
pub const Z_ERRNO: libc::c_int = -(1 as libc::c_int);
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_NULL: libc::c_int = 0 as libc::c_int;
pub const INT_MAX: libc::c_int = 2147483647 as libc::c_int;
pub const GZ_WRITE: libc::c_int = 31153 as libc::c_int;
pub const Z_DATA_ERROR: libc::c_int = -(3 as libc::c_int);
/* gzwrite.c -- zlib functions for writing gzip files
 * Copyright (C) 2004-2017 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* Local functions */
/* Initialize state for writing a gzip file.  Mark initialization by setting
state->size to non-zero.  Return -1 on a memory allocation failure, or 0 on
success. */
unsafe extern "C" fn gz_init(mut state: gz_statep) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    /* allocate input buffer (double size for gzprintf) */
    (*state).in_0 =
        malloc(((*state).want << 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_uchar;
    if (*state).in_0.is_null() {
        gz_error(
            state,
            Z_MEM_ERROR,
            b"out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    /* only need output buffer and deflate state if compressing */
    if (*state).direct == 0 {
        /* allocate output buffer */
        (*state).out = malloc((*state).want as libc::c_ulong) as *mut libc::c_uchar;
        if (*state).out.is_null() {
            free((*state).in_0 as *mut libc::c_void);
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        /* allocate deflate memory, set up for gzip compression */
        (*strm).zalloc =
            ::std::mem::transmute::<libc::intptr_t, alloc_func>(Z_NULL as libc::intptr_t);
        (*strm).zfree =
            ::std::mem::transmute::<libc::intptr_t, free_func>(Z_NULL as libc::intptr_t);
        (*strm).opaque = Z_NULL as voidpf;
        ret = deflateInit2_(
            strm,
            (*state).level,
            8 as libc::c_int,
            15 as libc::c_int + 16 as libc::c_int,
            8 as libc::c_int,
            (*state).strategy,
            ZLIB_VERSION.as_ptr(),
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        );
        if ret != Z_OK {
            free((*state).out as *mut libc::c_void);
            free((*state).in_0 as *mut libc::c_void);
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*strm).next_in = NULL as *mut Bytef
    }
    /* mark state as initialized */
    (*state).size = (*state).want;
    /* initialize write buffer if compressing */
    if (*state).direct == 0 {
        (*strm).avail_out = (*state).size;
        (*strm).next_out = (*state).out;
        (*state).x.next = (*strm).next_out
    }
    return 0 as libc::c_int;
}
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
/* Compress whatever is at avail_in and next_in and write to the output file.
Return -1 if there is an error writing to the output file or if gz_init()
fails to allocate memory, otherwise 0.  flush is assumed to be a valid
deflate() flush value.  If flush is Z_FINISH, then the deflate() state is
reset to start a new gzip stream.  If gz->direct is true, then simply write
to the output file without compressing, and ignore flush. */
unsafe extern "C" fn gz_comp(mut state: gz_statep, mut flush: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut writ: libc::c_int = 0;
    let mut have: libc::c_uint = 0;
    let mut put: libc::c_uint = 0;
    let mut max: libc::c_uint = (-(1 as libc::c_int) as libc::c_uint >> 2 as libc::c_int)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    let mut strm: z_streamp = &mut (*state).strm;
    /* allocate memory if this is the first time through */
    if (*state).size == 0 as libc::c_int as libc::c_uint && gz_init(state) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    /* write directly if requested */
    if (*state).direct != 0 {
        while (*strm).avail_in != 0 {
            put = if (*strm).avail_in > max {
                max
            } else {
                (*strm).avail_in
            };
            writ = write(
                (*state).fd,
                (*strm).next_in as *const libc::c_void,
                put as size_t,
            ) as libc::c_int;
            if writ < 0 as libc::c_int {
                gz_error(state, Z_ERRNO, strerror(errno));
                return -(1 as libc::c_int);
            }
            (*strm).avail_in = ((*strm).avail_in as libc::c_uint).wrapping_sub(writ as libc::c_uint)
                as uInt as uInt;
            (*strm).next_in = (*strm).next_in.offset(writ as isize)
        }
        return 0 as libc::c_int;
    }
    /* run deflate() on provided input until it produces no more output */
    ret = Z_OK;
    loop {
        /* write out current buffer contents if full, or if flushing, but if
        doing Z_FINISH then don't write until we get to Z_STREAM_END */
        if (*strm).avail_out == 0 as libc::c_int as libc::c_uint
            || flush != Z_NO_FLUSH && (flush != Z_FINISH || ret == Z_STREAM_END)
        {
            while (*strm).next_out > (*state).x.next {
                put = if (*strm).next_out.offset_from((*state).x.next) as libc::c_long
                    > max as libc::c_int as libc::c_long
                {
                    max
                } else {
                    (*strm).next_out.offset_from((*state).x.next) as libc::c_long
                        as libc::c_uint
                };
                writ = write(
                    (*state).fd,
                    (*state).x.next as *const libc::c_void,
                    put as size_t,
                ) as libc::c_int;
                if writ < 0 as libc::c_int {
                    gz_error(state, Z_ERRNO, strerror(errno));
                    return -(1 as libc::c_int);
                }
                (*state).x.next = (*state).x.next.offset(writ as isize)
            }
            if (*strm).avail_out == 0 as libc::c_int as libc::c_uint {
                (*strm).avail_out = (*state).size;
                (*strm).next_out = (*state).out;
                (*state).x.next = (*state).out
            }
        }
        /* compress */
        have = (*strm).avail_out;
        ret = deflate(strm, flush);
        if ret == Z_STREAM_ERROR {
            gz_error(
                state,
                Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\x00" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        have = have.wrapping_sub((*strm).avail_out);
        if !(have != 0) {
            break;
        }
    }
    /* if that completed a deflate stream, allow another to start */
    if flush == Z_FINISH {
        deflateReset(strm);
    }
    /* all done, no errors */
    return 0 as libc::c_int;
}
/* Compress len zeros to output.  Return -1 on a write error or memory
allocation failure by gz_comp(), or 0 on success. */
unsafe extern "C" fn gz_zero(mut state: gz_statep, mut len: off64_t) -> libc::c_int {
    let mut first: libc::c_int = 0;
    let mut n: libc::c_uint = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    /* consume whatever's left in the input buffer */
    if (*strm).avail_in != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    /* compress len zeros (len guaranteed > 0) */
    first = 1 as libc::c_int;
    while len != 0 {
        n = if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            == ::std::mem::size_of::<off64_t>() as libc::c_ulong
            && (*state).size > INT_MAX as libc::c_uint
            || (*state).size as off64_t > len
        {
            len as libc::c_uint
        } else {
            (*state).size
        };
        if first != 0 {
            memset(
                (*state).in_0 as *mut libc::c_void,
                0 as libc::c_int,
                n as libc::c_ulong,
            );
            first = 0 as libc::c_int
        }
        (*strm).avail_in = n;
        (*strm).next_in = (*state).in_0;
        (*state).x.pos += n as libc::c_long;
        if gz_comp(state, Z_NO_FLUSH) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        len -= n as libc::c_long
    }
    return 0 as libc::c_int;
}
/* Write len bytes from buf to file.  Return the number of bytes written.  If
the returned value is less than len, then there was an error. */
unsafe extern "C" fn gz_write(
    mut state: gz_statep,
    mut buf: voidpc,
    mut len: z_size_t,
) -> z_size_t {
    let mut put: z_size_t = len;
    /* if len is zero, avoid unnecessary operations */
    if len == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as z_size_t;
    }
    /* allocate memory if this is the first time through */
    if (*state).size == 0 as libc::c_int as libc::c_uint && gz_init(state) == -(1 as libc::c_int) {
        return 0 as libc::c_int as z_size_t;
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return 0 as libc::c_int as z_size_t;
        }
    }
    /* for small len, copy to input buffer, otherwise compress directly */
    if len < (*state).size as libc::c_ulong {
        loop
        /* copy to input buffer, compress when full */
        {
            let mut have: libc::c_uint = 0;
            let mut copy: libc::c_uint = 0;
            if (*state).strm.avail_in == 0 as libc::c_int as libc::c_uint {
                (*state).strm.next_in = (*state).in_0
            }
            have = (*state)
                .strm
                .next_in
                .offset((*state).strm.avail_in as isize)
                .offset_from((*state).in_0) as libc::c_long
                as libc::c_uint;
            copy = (*state).size.wrapping_sub(have);
            if copy as libc::c_ulong > len {
                copy = len as libc::c_uint
            }
            memcpy(
                (*state).in_0.offset(have as isize) as *mut libc::c_void,
                buf,
                copy as libc::c_ulong,
            );
            (*state).strm.avail_in =
                ((*state).strm.avail_in as libc::c_uint).wrapping_add(copy) as uInt as uInt;
            (*state).x.pos += copy as libc::c_long;
            buf = (buf as *const libc::c_char).offset(copy as isize) as voidpc;
            len =
                (len as libc::c_ulong).wrapping_sub(copy as libc::c_ulong) as z_size_t as z_size_t;
            if len != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as libc::c_int) {
                return 0 as libc::c_int as z_size_t;
            }
            if !(len != 0) {
                break;
            }
        }
    } else {
        /* consume whatever's left in the input buffer */
        if (*state).strm.avail_in != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as libc::c_int) {
            return 0 as libc::c_int as z_size_t;
        }
        /* directly compress user buffer to file */
        (*state).strm.next_in = buf as *mut Bytef;
        loop {
            let mut n: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
            if n as libc::c_ulong > len {
                n = len as libc::c_uint
            }
            (*state).strm.avail_in = n;
            (*state).x.pos += n as libc::c_long;
            if gz_comp(state, Z_NO_FLUSH) == -(1 as libc::c_int) {
                return 0 as libc::c_int as z_size_t;
            }
            len = (len as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as z_size_t as z_size_t;
            if !(len != 0) {
                break;
            }
        }
    }
    /* input was all buffered or compressed */
    return put;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzwrite(
    mut file: gzFile,
    mut buf: voidpc,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() {
        return 0 as libc::c_int;
    }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return 0 as libc::c_int;
    }
    /* since an int is returned, make sure len fits in one, otherwise return
    with an error (this avoids a flaw in the interface) */
    if (len as libc::c_int) < 0 as libc::c_int {
        gz_error(
            state,
            Z_DATA_ERROR,
            b"requested length does not fit in int\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    /* write len bytes from buf (the return value will fit in an int) */
    return gz_write(state, buf, len as z_size_t) as libc::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzfwrite(
    mut buf: voidpc,
    mut size: z_size_t,
    mut nitems: z_size_t,
    mut file: gzFile,
) -> z_size_t {
    let mut len: z_size_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() {
        return 0 as libc::c_int as z_size_t;
    }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return 0 as libc::c_int as z_size_t;
    }
    /* compute bytes to read -- error on overflow */
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        gz_error(
            state,
            Z_STREAM_ERROR,
            b"request does not fit in a size_t\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int as z_size_t;
    }
    /* write len bytes to buf, return the number of full items written */
    return if len != 0 {
        gz_write(state, buf, len).wrapping_div(size)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzputc(mut file: gzFile, mut c: libc::c_int) -> libc::c_int {
    let mut have: libc::c_uint = 0;
    let mut buf: [libc::c_uchar; 1] = [0; 1];
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    /* get internal structure */
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    /* check that we're writing and that there's no error */
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return -(1 as libc::c_int);
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    /* try writing to input buffer for speed (state->size == 0 if buffer not
    initialized) */
    if (*state).size != 0 {
        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
            (*strm).next_in = (*state).in_0
        }
        have = (*strm)
            .next_in
            .offset((*strm).avail_in as isize)
            .offset_from((*state).in_0) as libc::c_long as libc::c_uint;
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as libc::c_uchar;
            (*strm).avail_in = (*strm).avail_in.wrapping_add(1);
            (*state).x.pos += 1;
            return c & 0xff as libc::c_int;
        }
    }
    /* no room in buffer or not initialized, use gz_write() */
    buf[0 as libc::c_int as usize] = c as libc::c_uchar;
    if gz_write(
        state,
        buf.as_mut_ptr() as voidpc,
        1 as libc::c_int as z_size_t,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    return c & 0xff as libc::c_int;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzputs(mut file: gzFile, mut str: *const libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut len: z_size_t = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return -(1 as libc::c_int);
    }
    /* write string */
    len = strlen(str);
    ret = gz_write(state, str as voidpc, len) as libc::c_int;
    return if ret == 0 as libc::c_int && len != 0 as libc::c_int as libc::c_ulong {
        -(1 as libc::c_int)
    } else {
        ret
    };
}
/* backward compatibility */
/* provide 64-bit offset functions if _LARGEFILE64_SOURCE defined, and/or
 * change the regular functions to 64 bits if _FILE_OFFSET_BITS is 64 (if
 * both are true, the application gets the *64 functions, and the regular
 * functions are changed to 64 bits) -- in case these are set on systems
 * without large file support, _LFS64_LARGEFILE must also be true
 */
/* Z_SOLO */
/* !Z_SOLO */
/* undocumented functions */
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzvprintf(
    mut file: gzFile,
    mut format: *const libc::c_char,
    mut va: ::std::ffi::VaList,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut left: libc::c_uint = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    /* get internal structure */
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    /* check that we're writing and that there's no error */
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return Z_STREAM_ERROR;
    }
    /* make sure we have some buffer space */
    if (*state).size == 0 as libc::c_int as libc::c_uint && gz_init(state) == -(1 as libc::c_int) {
        return (*state).err;
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return (*state).err;
        }
    }
    /* do the printf() into the input buffer, put length in len -- the input
    buffer is double-sized just for this function, so there is guaranteed to
    be state->size bytes available after the current contents */
    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
        (*strm).next_in = (*state).in_0
    }
    next = (*state)
        .in_0
        .offset((*strm).next_in.offset_from((*state).in_0) as libc::c_long as isize)
        .offset((*strm).avail_in as isize) as *mut libc::c_char;
    *next.offset((*state).size.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize) =
        0 as libc::c_int as libc::c_char;
    len = vsnprintf(
        next,
        (*state).size as libc::c_ulong,
        format,
        va.as_va_list(),
    );
    /* check that printf() results fit in buffer */
    if len == 0 as libc::c_int
        || len as libc::c_uint >= (*state).size
        || *next.offset((*state).size.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int
            != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    /* update buffer and position, compress first half if past that */
    (*strm).avail_in =
        ((*strm).avail_in as libc::c_uint).wrapping_add(len as libc::c_uint) as uInt as uInt;
    (*state).x.pos += len as libc::c_long;
    if (*strm).avail_in >= (*state).size {
        left = (*strm).avail_in.wrapping_sub((*state).size);
        (*strm).avail_in = (*state).size;
        if gz_comp(state, Z_NO_FLUSH) == -(1 as libc::c_int) {
            return (*state).err;
        }
        memcpy(
            (*state).in_0 as *mut libc::c_void,
            (*state).in_0.offset((*state).size as isize) as *const libc::c_void,
            left as libc::c_ulong,
        );
        (*strm).next_in = (*state).in_0;
        (*strm).avail_in = left
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn gzprintf(
    mut file: gzFile,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut va: ::std::ffi::VaListImpl;
    let mut ret: libc::c_int = 0;
    va = args.clone();
    ret = gzvprintf(file, format, va.as_va_list());
    return ret;
}
/* !STDC && !Z_HAVE_STDARG_H */
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzflush(mut file: gzFile, mut flush: libc::c_int) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    /* check that we're writing and that there's no error */
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return Z_STREAM_ERROR;
    }
    /* check flush parameter */
    if flush < 0 as libc::c_int || flush > Z_FINISH {
        return Z_STREAM_ERROR;
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return (*state).err;
        }
    }
    /* compress remaining data with requested flush */
    gz_comp(state, flush);
    return (*state).err;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzsetparams(
    mut file: gzFile,
    mut level: libc::c_int,
    mut strategy: libc::c_int,
) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    /* get internal structure */
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    /* check that we're writing and that there's no error */
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return Z_STREAM_ERROR;
    }
    /* if no change is requested, then do nothing */
    if level == (*state).level && strategy == (*state).strategy {
        return Z_OK;
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return (*state).err;
        }
    }
    /* change compression parameters for subsequent input */
    if (*state).size != 0 {
        /* flush previous input with previous parameters before changing */
        if (*strm).avail_in != 0 && gz_comp(state, Z_BLOCK) == -(1 as libc::c_int) {
            return (*state).err;
        }
        deflateParams(strm, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return Z_OK;
}
/* -- see zlib.h -- */
#[no_mangle]
pub unsafe extern "C" fn gzclose_w(mut file: gzFile) -> libc::c_int {
    let mut ret: libc::c_int = Z_OK;
    let mut state: gz_statep = 0 as *mut gz_state;
    /* get internal structure */
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    /* check that we're writing */
    if (*state).mode != GZ_WRITE {
        return Z_STREAM_ERROR;
    }
    /* check for seek request */
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            ret = (*state).err
        }
    }
    /* flush, free memory, and close file */
    if gz_comp(state, Z_FINISH) == -(1 as libc::c_int) {
        ret = (*state).err
    }
    if (*state).size != 0 {
        if (*state).direct == 0 {
            deflateEnd(&mut (*state).strm);
            free((*state).out as *mut libc::c_void);
        }
        free((*state).in_0 as *mut libc::c_void);
    }
    gz_error(state, Z_OK, NULL as *const libc::c_char);
    free((*state).path as *mut libc::c_void);
    if close((*state).fd) == -(1 as libc::c_int) {
        ret = Z_ERRNO
    }
    free(state as *mut libc::c_void);
    return ret;
}
