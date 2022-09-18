use ::libc;
extern "C" {
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn gzdopen(fd: libc::c_int, mode: *const libc::c_char) -> gzFile;
    #[no_mangle]
    fn gzread(file: gzFile, buf: voidp, len: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn gzwrite(file: gzFile, buf: voidpc, len: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn gzclose(file: gzFile) -> libc::c_int;
    #[no_mangle]
    fn gzerror(file: gzFile, errnum: *mut libc::c_int) -> *const libc::c_char;
    /* backward compatibility */
    /* provide 64-bit offset functions if _LARGEFILE64_SOURCE defined, and/or
     * change the regular functions to 64 bits if _FILE_OFFSET_BITS is 64 (if
     * both are true, the application gets the *64 functions, and the regular
     * functions are changed to 64 bits) -- in case these are set on systems
     * without large file support, _LFS64_LARGEFILE must also be true
     */
    #[no_mangle]
    fn gzopen64(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char);
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type voidpc = *const libc::c_void;
pub type voidp = *mut libc::c_void;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off64_t = __off64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub const gzopen: unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> gzFile =
    gzopen64;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
/* minigzip.c -- simulate gzip using the zlib compression library
 * Copyright (C) 1995-2006, 2010, 2011, 2016 Jean-loup Gailly
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
 * minigzip is a minimal implementation of the gzip utility. This is
 * only an example of using zlib and isn't meant to replace the
 * full-featured gzip. No attempt is made to deal with file systems
 * limiting names to 14 or 8+3 characters, etc... Error checking is
 * very limited. So use minigzip only for testing; use gzip for the
 * real thing. On MSDOS, use only on file names without extension
 * or in pipe mode.
 */
/* @(#) $Id$ */
/* UNDER_CE */
pub const GZ_SUFFIX: [libc::c_char; 4] =
    unsafe { *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b".gz\x00") };
static mut prog: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* ===========================================================================
 * Display error message and exit
 */
#[no_mangle]
pub unsafe extern "C" fn error(mut msg: *const libc::c_char) {
    fprintf(
        stderr,
        b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
        prog,
        msg,
    );
    exit(1 as libc::c_int);
}
/* ===========================================================================
 * Compress input to output then close both files.
 */
#[no_mangle]
pub unsafe extern "C" fn gz_compress(mut in_0: *mut FILE, mut out: gzFile) {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut len: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    loop {
        len = fread(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
            in_0,
        ) as libc::c_int;
        if ferror(in_0) != 0 {
            perror(b"fread\x00" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        if len == 0 as libc::c_int {
            break;
        }
        if gzwrite(out, buf.as_mut_ptr() as voidpc, len as libc::c_uint) != len {
            error(gzerror(out, &mut err));
        }
    }
    fclose(in_0);
    if gzclose(out) != Z_OK {
        error(b"failed gzclose\x00" as *const u8 as *const libc::c_char);
    };
}
/* MMAP version, Miguel Albrecht <malbrech@eso.org> */
/* USE_MMAP */
/* ===========================================================================
 * Uncompress input to output then close both files.
 */
#[no_mangle]
pub unsafe extern "C" fn gz_uncompress(mut in_0: gzFile, mut out: *mut FILE) {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut len: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    loop {
        len = gzread(
            in_0,
            buf.as_mut_ptr() as voidp,
            ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong as libc::c_uint,
        );
        if len < 0 as libc::c_int {
            error(gzerror(in_0, &mut err));
        }
        if len == 0 as libc::c_int {
            break;
        }
        if fwrite(
            buf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len as libc::c_uint as libc::c_ulong,
            out,
        ) as libc::c_int
            != len
        {
            error(b"failed fwrite\x00" as *const u8 as *const libc::c_char);
        }
    }
    if fclose(out) != 0 {
        error(b"failed fclose\x00" as *const u8 as *const libc::c_char);
    }
    if gzclose(in_0) != Z_OK {
        error(b"failed gzclose\x00" as *const u8 as *const libc::c_char);
    };
}
/* ===========================================================================
 * Compress the given file: create a corresponding .gz file and remove the
 * original.
 */
#[no_mangle]
pub unsafe extern "C" fn file_compress(mut file: *mut libc::c_char, mut mode: *mut libc::c_char) {
    let mut outfile: [libc::c_char; 1024] = [0; 1024];
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut out: gzFile = 0 as *mut gzFile_s;
    if strlen(file).wrapping_add(strlen(GZ_SUFFIX.as_ptr()))
        >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
    {
        fprintf(
            stderr,
            b"%s: filename too long\n\x00" as *const u8 as *const libc::c_char,
            prog,
        );
        exit(1 as libc::c_int);
    }
    snprintf(
        outfile.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        file,
        GZ_SUFFIX.as_ptr(),
    );
    in_0 = fopen(file, b"rb\x00" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        perror(file);
        exit(1 as libc::c_int);
    }
    out = gzopen64(outfile.as_mut_ptr(), mode);
    if out.is_null() {
        fprintf(
            stderr,
            b"%s: can\'t gzopen %s\n\x00" as *const u8 as *const libc::c_char,
            prog,
            outfile.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    gz_compress(in_0, out);
    unlink(file);
}
/* ===========================================================================
 * Uncompress the given file and remove the original.
 */
#[no_mangle]
pub unsafe extern "C" fn file_uncompress(mut file: *mut libc::c_char) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut infile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut in_0: gzFile = 0 as *mut gzFile_s;
    let mut len: libc::c_uint = strlen(file) as libc::c_uint;
    if (len as libc::c_ulong).wrapping_add(strlen(GZ_SUFFIX.as_ptr()))
        >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
    {
        fprintf(
            stderr,
            b"%s: filename too long\n\x00" as *const u8 as *const libc::c_char,
            prog,
        );
        exit(1 as libc::c_int);
    }
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%s\x00" as *const u8 as *const libc::c_char,
        file,
    );
    if len as libc::c_ulong
        > (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && strcmp(
            file.offset(len as isize).offset(
                -((::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
            ),
            GZ_SUFFIX.as_ptr(),
        ) == 0 as libc::c_int
    {
        infile = file;
        outfile = buf.as_mut_ptr();
        *outfile.offset(len.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize) =
            '\u{0}' as i32 as libc::c_char
    } else {
        outfile = file;
        infile = buf.as_mut_ptr();
        snprintf(
            buf.as_mut_ptr().offset(len as isize),
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(len as libc::c_ulong),
            b"%s\x00" as *const u8 as *const libc::c_char,
            GZ_SUFFIX.as_ptr(),
        );
    }
    in_0 = gzopen64(infile, b"rb\x00" as *const u8 as *const libc::c_char);
    if in_0.is_null() {
        fprintf(
            stderr,
            b"%s: can\'t gzopen %s\n\x00" as *const u8 as *const libc::c_char,
            prog,
            infile,
        );
        exit(1 as libc::c_int);
    }
    out = fopen(outfile, b"wb\x00" as *const u8 as *const libc::c_char);
    if out.is_null() {
        perror(file);
        exit(1 as libc::c_int);
    }
    gz_uncompress(in_0, out);
    unlink(infile);
}
/* ===========================================================================
 * Usage:  minigzip [-c] [-d] [-f] [-h] [-r] [-1 to -9] [files...]
 *   -c : write to standard output
 *   -d : decompress
 *   -f : compress with Z_FILTERED
 *   -h : compress with Z_HUFFMAN_ONLY
 *   -r : compress with Z_RLE
 *   -1 to -9 : compression level
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut copyout: libc::c_int = 0 as libc::c_int;
    let mut uncompr: libc::c_int = 0 as libc::c_int;
    let mut file: gzFile = 0 as *mut gzFile_s;
    let mut bname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outmode: [libc::c_char; 20] = [0; 20];
    snprintf(
        outmode.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
        b"%s\x00" as *const u8 as *const libc::c_char,
        b"wb6 \x00" as *const u8 as *const libc::c_char,
    );
    prog = *argv.offset(0 as libc::c_int as isize);
    bname = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if !bname.is_null() {
        bname = bname.offset(1)
    } else {
        bname = *argv.offset(0 as libc::c_int as isize)
    }
    argc -= 1;
    argv = argv.offset(1);
    if strcmp(bname, b"gunzip\x00" as *const u8 as *const libc::c_char) == 0 {
        uncompr = 1 as libc::c_int
    } else if strcmp(bname, b"zcat\x00" as *const u8 as *const libc::c_char) == 0 {
        uncompr = 1 as libc::c_int;
        copyout = uncompr
    }
    while argc > 0 as libc::c_int {
        if strcmp(*argv, b"-c\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            copyout = 1 as libc::c_int
        } else if strcmp(*argv, b"-d\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            uncompr = 1 as libc::c_int
        } else if strcmp(*argv, b"-f\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            outmode[3 as libc::c_int as usize] = 'f' as i32 as libc::c_char
        } else if strcmp(*argv, b"-h\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            outmode[3 as libc::c_int as usize] = 'h' as i32 as libc::c_char
        } else if strcmp(*argv, b"-r\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            outmode[3 as libc::c_int as usize] = 'R' as i32 as libc::c_char
        } else {
            if !(*(*argv).offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *(*argv).offset(1 as libc::c_int as isize) as libc::c_int >= '1' as i32
                && *(*argv).offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
                && *(*argv).offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
            {
                break;
            }
            outmode[2 as libc::c_int as usize] = *(*argv).offset(1 as libc::c_int as isize)
        }
        argc -= 1;
        argv = argv.offset(1)
    }
    if outmode[3 as libc::c_int as usize] as libc::c_int == ' ' as i32 {
        outmode[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
    }
    if argc == 0 as libc::c_int {
        if uncompr != 0 {
            file = gzdopen(fileno(stdin), b"rb\x00" as *const u8 as *const libc::c_char);
            if file.is_null() {
                error(b"can\'t gzdopen stdin\x00" as *const u8 as *const libc::c_char);
            }
            gz_uncompress(file, stdout);
        } else {
            file = gzdopen(fileno(stdout), outmode.as_mut_ptr());
            if file.is_null() {
                error(b"can\'t gzdopen stdout\x00" as *const u8 as *const libc::c_char);
            }
            gz_compress(stdin, file);
        }
    } else {
        (copyout) != 0;
        loop {
            if uncompr != 0 {
                if copyout != 0 {
                    file = gzopen64(*argv, b"rb\x00" as *const u8 as *const libc::c_char);
                    if file.is_null() {
                        fprintf(
                            stderr,
                            b"%s: can\'t gzopen %s\n\x00" as *const u8 as *const libc::c_char,
                            prog,
                            *argv,
                        );
                    } else {
                        gz_uncompress(file, stdout);
                    }
                } else {
                    file_uncompress(*argv);
                }
            } else if copyout != 0 {
                let mut in_0: *mut FILE =
                    fopen(*argv, b"rb\x00" as *const u8 as *const libc::c_char);
                if in_0.is_null() {
                    perror(*argv);
                } else {
                    file = gzdopen(fileno(stdout), outmode.as_mut_ptr());
                    if file.is_null() {
                        error(b"can\'t gzdopen stdout\x00" as *const u8 as *const libc::c_char);
                    }
                    gz_compress(in_0, file);
                }
            } else {
                file_compress(*argv, outmode.as_mut_ptr());
            }
            argv = argv.offset(1);
            argc -= 1;
            if !(argc != 0) {
                break;
            }
        }
    }
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
