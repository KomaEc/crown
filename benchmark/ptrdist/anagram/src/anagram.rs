use ::libc;
extern "C" {
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
/*
 *  Anagram program by Raymond Chen,
 *  inspired by a similar program by Brian Scearce
 *
 *  This program is Copyright 1991 by Raymond Chen.
 *              (rjc@math.princeton.edu)
 *
 *  This program may be freely distributed provided all alterations
 *  to the original are clearly indicated as such.
 */
/* There are two tricks.  First is the Basic Idea:
 *
 * When the user types in a phrase, the phrase is first preprocessed to
 * determine how many of each letter appears.  A bit field is then constructed
 * dynamically, such that each field is large enough to hold the next power
 * of two larger than the number of times the character appears.  For example,
 * if the phrase is hello, world, the bit field would be
 *
 *   00 00 00 000 000 00 00
 *    d e   h   l   o  r  w
 *
 * The phrase hello, world, itself would be encoded as
 *
 *   01 01 01 011 010 01 01
 *    d e   h   l   o  r  w
 *
 * and the word hollow would be encoded as
 *
 *   00 00 01 010 010 00 01
 *    d  e  h   l   o  r  w
 *
 * The top bit of each field is set in a special value called the sign.
 * Here, the sign would be
 *
 *   10 10 10 100 100 10 10
 *    d  e  h   l   o  r  w
 *
 * The reason for packing the values into a bit field is that the operation
 * of subtracting out the letters of a word from the current phrase can be
 * carried out in parallel.  for example, subtracting the word hello from
 * the phrase hello, world, is merely
 *
 *    d e   h   l   o  r  w
 *   01 01 01 011 010 01 01 (dehllloorw)
 * - 00 00 01 010 010 00 01 (hlloow)
 * ========================
 *   01 01 00 001 000 01 00 (delr)
 *
 * Since none of the sign bits is set, the word fits, and we can continue.
 * Suppose the next word we tried was hood.
 *
 *    d e   h   l   o  r  w
 *   01 01 00 001 000 01 00 (delr)
 * - 01 00 01 000 010 00 00 (hood)
 * ========================
 *   00 00 11 000 110 01 00
 *         ^      ^
 * A sign bit is set.  (Two, actually.)  This means that hood does not
 * fit in delr, so we skip it and try another word.  (Observe that
 * when a sign bit becomes set, it screws up the values for the letters to
 * the left of that bit, but that's not important.)
 *
 * The inner loop of an anagram program is testing to see if a
 * word fits in the collection of untried letters.  Traditional methods
 * keep an array of 26 integers, which are then compared in turn.  This
 * means that there are 26 comparisons per word.
 *
 * This method reduces the number of comparisons to MAX_QUAD, typically 2.
 * Instead of looping through an array, we merely perform the indicated
 * subtraction and test if any of the sign bits is set.
 */
/* The nuts and bolts:
 *
 * The dictionary is loaded and preprocessed.  The preprocessed dictionary
 * is a concatenation of copies of the structure:
 *
 * struct dictword {
 *     char bStructureSize;             -- size of this structure
 *     char cLetters;                   -- number of letters in the word
 *     char achWord[];                  -- the word itself (0-terminated)
 * }
 *
 * Since this is a variable-sized structure, we keep its size in the structure
 * itself for rapid stepping through the table.
 *
 * When a phrase is typed in, it is first preprocessed as described in the
 * Basic Idea.  We then go through the dictionary, testing each word.  If
 * the word fits in our phrase, we build the bit field for its frequency
 * table and add it to the list of candidates.
 */
/*
 * The Second Trick:
 *
 * Before diving into our anagram search, we then tabulate how many times
 * each letter appears in our list of candidates, and sort the table, with
 * the rarest letter first.
 *
 * We then do our anagram search.
 *
 * Like most anagram programs, this program does a depth-first search.
 * Although most anagram programs do some sort of heuristics to decide what
 * order to place words in the list_of_candidates, the search itself proceeds
 * according to a greedy algorithm.  That is, once you find a word that fits,
 * subtract it and recurse.
 *
 * This anagram program exercises some restraint and does not march down
 * every branch that shows itself.  Instead, it only goes down branches
 * that use the rarest unused letter.  This helps to find dead ends faster.
 *
 * FindAnagram(unused_letters, list_of_candidates) {
 *  l = the rarest letter as yet unused
 *  For word in list_of_candidates {
 *     if word does not fit in unused_letters, go on to the next word.
 *     if word does not contain l, defer.
 *      FindAnagram(unused_letters - word, list_of_candidates[word,...])
 *  }
 * }
 *
 *
 * The heuristic of the Second Trick can probably be improved.  I invite
 * anyone willing to improve it to do so.
 */
/* Use the accompanying unproto perl script to remove the ANSI-style
 * prototypes, for those of you who have K&R compilers.
 */
/* Before compiling, make sure Quad and MASK_BITS are set properly.  For best
 * results, make Quad the largest integer size supported on your machine.
 * So if your machine has long longs, make Quad an unsigned long long.
 * (I called it Quad because on most machines, the largest integer size
 * supported is a four-byte unsigned long.)
 *
 * If you need to be able to anagram larger phrases, increase MAX_QUADS.
 * If you increase it beyond 4, you'll have to add a few more loop unrolling
 * steps to FindAnagram.
 */
pub type Quad = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Word {
    pub aqMask: [Quad; 2],
    pub pchWord: *mut libc::c_char,
    pub cchLength: libc::c_uint,
}
pub type PWord = *mut Word;
pub type PPWord = *mut *mut Word;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Letter {
    pub uFrequency: libc::c_uint,
    pub uShift: libc::c_uint,
    pub uBits: libc::c_uint,
    pub iq: libc::c_uint,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const EOF: libc::c_int = -(1 as libc::c_int);
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        NULL as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub const _STAT_VER_LINUX: libc::c_int = 1 as libc::c_int;
pub const _STAT_VER: libc::c_int = _STAT_VER_LINUX;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
/* for building our bit mask */
pub const MASK_BITS: libc::c_int = 32 as libc::c_int;
/* number of bits in a Quad */
pub const MAX_QUADS: libc::c_int = 2 as libc::c_int;
/* controls largest phrase */
pub const MAXWORDS: libc::c_int = 26000 as libc::c_int;
/* dictionary length */
pub const MAXCAND: libc::c_int = 5000 as libc::c_int;
/* candidates */
/* words in the solution */
pub const ALPHABET: libc::c_int = 26 as libc::c_int;
/* convert index to letter */
/* IBM PC's don't like globs of memory larger than 64K without
 * special gyrations.  That's where the huges get stuck in.  And the
 * two types of allocations on an IBM PC need to be handled differently.
 *
 * HaltProcessing is called during the anagram search.	If it returns nonzero,
 * then the search is aborted.
 *
 * Cdecl is a macro expanded before the name of all functions that must
 * use C-style parameter passing.  This lets you change the default parameter
 * passing style for the other functions.
 */
/* char *malloc(); */
pub const StringFormat: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"%15s%c\x00") };
#[no_mangle]
pub static mut apwCand: [PWord; 5000] = [0 as *const Word as *mut Word; 5000];
/* candidates we've found so far */
#[no_mangle]
pub static mut cpwCand: libc::c_uint = 0;
#[no_mangle]
pub static mut alPhrase: [Letter; 26] = [Letter {
    uFrequency: 0,
    uShift: 0,
    uBits: 0,
    iq: 0,
}; 26];
/* quick access to a letter */
#[no_mangle]
pub static mut cchPhraseLength: libc::c_int = 0;
/* number of letters in phrase */
#[no_mangle]
pub static mut aqMainMask: [Quad; 2] = [0; 2];
/* the bit field for the full phrase */
#[no_mangle]
pub static mut aqMainSign: [Quad; 2] = [0; 2];
/* where the sign bits are */
#[no_mangle]
pub static mut cchMinLength: libc::c_int = 3 as libc::c_int;
/* auGlobalFrequency counts the number of times each letter appears, summed
 * over all candidate words.  This is used to decide which letter to attack
 * first.
 */
#[no_mangle]
pub static mut auGlobalFrequency: [libc::c_uint; 26] = [0; 26];
#[no_mangle]
pub static mut achByFrequency: [libc::c_char; 26] = [0; 26];
/* for sorting */
#[no_mangle]
pub static mut pchDictionary: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* the dictionary is read here */
/* quickly zero out an integer array */
/* Fatal -- print a message before expiring */
#[no_mangle]
pub unsafe extern "C" fn Fatal(mut pchMsg: *mut libc::c_char, mut u: libc::c_uint) {
    fprintf(stderr, pchMsg, u);
    exit(1 as libc::c_int);
}
/* ReadDict -- read the dictionary file into memory and preprocess it
 *
 * A word of length cch in the dictionary is encoded as follows:
 *
 *    byte 0    = cch + 3
 *    byte 1    = number of letters in the word
 *    byte 2... = the word itself, null-terminated
 *
 * Observe that cch+3 is the length of the total encoding.  These
 * byte streams are concatenated, and terminated with a 0.
 */
#[no_mangle]
pub unsafe extern "C" fn ReadDict(mut pchFile: *mut libc::c_char) {
    let mut fp: *mut FILE = 0 as *mut FILE; /* reserve for length */
    let mut pch: *mut libc::c_char = 0 as *mut libc::c_char; /* which Quad? */
    let mut pchBase: *mut libc::c_char = 0 as *mut libc::c_char; /* bits used in the current Quad */
    let mut ulLen: libc::c_ulong = 0; /* bits needed for current letter */
    let mut cWords: libc::c_uint = 0 as libc::c_int as libc::c_uint; /* used to build the mask */
    let mut cLetters: libc::c_uint = 0;
    let mut ch: libc::c_int = 0;
    let mut statBuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if stat(pchFile, &mut statBuf) != 0 {
        Fatal(
            b"Cannot stat dictionary\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    }
    ulLen = (statBuf.st_size as libc::c_ulong)
        .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(MAXWORDS as libc::c_ulong));
    pchDictionary = malloc(ulLen) as *mut libc::c_char;
    pchBase = pchDictionary;
    if pchDictionary.is_null() {
        Fatal(
            b"Unable to allocate memory for dictionary\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    }
    fp = fopen(pchFile, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        Fatal(
            b"Cannot open dictionary\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    }
    while feof(fp) == 0 {
        pch = pchBase.offset(2 as libc::c_int as isize);
        cLetters = 0 as libc::c_int as libc::c_uint;
        loop {
            ch = fgetc(fp);
            if !(ch != '\n' as i32 && ch != EOF) {
                break;
            }
            if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                cLetters = cLetters.wrapping_add(1)
            }
            let fresh0 = pch;
            pch = pch.offset(1);
            *fresh0 = ch as libc::c_char
        }
        let fresh1 = pch;
        pch = pch.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char;
        *pchBase = pch.wrapping_offset_from(pchBase) as libc::c_long as libc::c_char;
        *pchBase.offset(1 as libc::c_int as isize) = cLetters as libc::c_char;
        pchBase = pch;
        cWords = cWords.wrapping_add(1)
    }
    fclose(fp);
    let fresh2 = pchBase;
    pchBase = pchBase.offset(1);
    *fresh2 = 0 as libc::c_int as libc::c_char;
    fprintf(
        stderr,
        b"main dictionary has %u entries\n\x00" as *const u8 as *const libc::c_char,
        cWords,
    );
    if cWords >= MAXWORDS as libc::c_uint {
        Fatal(
            b"Dictionary too large; increase MAXWORDS\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    }
    fprintf(
        stderr,
        b"%lu bytes wasted\n\x00" as *const u8 as *const libc::c_char,
        ulLen.wrapping_sub(
            pchBase.wrapping_offset_from(pchDictionary) as libc::c_long as libc::c_ulong
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn BuildMask(mut pchPhrase: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut iq: libc::c_uint = 0;
    let mut cbtUsed: libc::c_int = 0;
    let mut cbtNeed: libc::c_int = 0;
    let mut qNeed: Quad = 0;
    bzero(
        alPhrase.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<Letter>() as libc::c_ulong).wrapping_mul(ALPHABET as libc::c_ulong),
    );
    bzero(
        aqMainMask.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<Quad>() as libc::c_ulong).wrapping_mul(MAX_QUADS as libc::c_ulong),
    );
    bzero(
        aqMainSign.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<Quad>() as libc::c_ulong).wrapping_mul(MAX_QUADS as libc::c_ulong),
    );
    /*
        Zero(alPhrase);
        Zero(aqMainMask);
        Zero(aqMainSign);
    */
    /* Tabulate letter frequencies in the phrase */
    cchPhraseLength = 0 as libc::c_int;
    loop {
        let fresh3 = pchPhrase;
        pchPhrase = pchPhrase.offset(1);
        ch = *fresh3 as libc::c_int;
        if !(ch != '\u{0}' as i32) {
            break;
        }
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            ch = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = ch;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(ch)
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(ch as isize)
                }
                __res
            });
            alPhrase[(ch - 'a' as i32) as usize].uFrequency = alPhrase[(ch - 'a' as i32) as usize]
                .uFrequency
                .wrapping_add(1);
            cchPhraseLength += 1
        }
    }
    /* Build  masks */
    iq = 0 as libc::c_int as libc::c_uint; /* which quad being used */
    cbtUsed = 0 as libc::c_int; /* bits used so far */
    i = 0 as libc::c_int;
    while i < ALPHABET {
        if alPhrase[i as usize].uFrequency == 0 as libc::c_int as libc::c_uint {
            auGlobalFrequency[i as usize] = !(0 as libc::c_int) as libc::c_uint
        /* to make it sort last */
        } else {
            auGlobalFrequency[i as usize] = 0 as libc::c_int as libc::c_uint;
            cbtNeed = 1 as libc::c_int;
            qNeed = 1 as libc::c_int as Quad;
            while alPhrase[i as usize].uFrequency as libc::c_ulong >= qNeed {
                cbtNeed += 1;
                qNeed <<= 1 as libc::c_int
            }
            if cbtUsed + cbtNeed > MASK_BITS {
                iq = iq.wrapping_add(1);
                if iq >= MAX_QUADS as libc::c_uint {
                    Fatal(
                        b"MAX_QUADS not large enough\n\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as libc::c_int as libc::c_uint,
                    );
                }
                cbtUsed = 0 as libc::c_int
            }
            alPhrase[i as usize].uBits =
                qNeed.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
            if cbtUsed != 0 {
                qNeed <<= cbtUsed
            }
            aqMainSign[iq as usize] |= qNeed;
            aqMainMask[iq as usize] |= (alPhrase[i as usize].uFrequency as Quad) << cbtUsed;
            alPhrase[i as usize].uShift = cbtUsed as libc::c_uint;
            alPhrase[i as usize].iq = iq;
            cbtUsed += cbtNeed
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn NewWord() -> PWord {
    let mut pw: PWord = 0 as *mut Word;
    pw = malloc(::std::mem::size_of::<Word>() as libc::c_ulong) as *mut Word;
    if pw.is_null() {
        Fatal(
            b"Out of memory after %d candidates\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cpwCand,
        );
    }
    return pw;
}
/* wprint -- print a word, followed by a space
 *
 * We would normally just use printf, but the string being printed is
 * is a huge pointer (on an IBM PC), so special care must be taken.
 */
#[no_mangle]
pub unsafe extern "C" fn wprint(mut pch: *mut libc::c_char) {
    printf(b"%s \x00" as *const u8 as *const libc::c_char, pch);
}
/* NextWord -- get another candidate entry, creating if necessary */
#[no_mangle]
pub unsafe extern "C" fn NextWord() -> PWord {
    let mut pw: PWord = 0 as *mut Word;
    if cpwCand >= MAXCAND as libc::c_uint {
        Fatal(
            b"Too many candidates\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    }
    let fresh4 = cpwCand;
    cpwCand = cpwCand.wrapping_add(1);
    pw = apwCand[fresh4 as usize];
    if !pw.is_null() {
        return pw;
    }
    apwCand[cpwCand.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] = NewWord();
    return apwCand[cpwCand.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
}
/* BuildWord -- build a Word structure from an ASCII word
 * If the word does not fit, then do nothing.
 */
#[no_mangle]
pub unsafe extern "C" fn BuildWord(mut pchWord: *mut libc::c_char) {
    let mut cchFrequency: [libc::c_uchar; 26] = [0; 26];
    let mut i: libc::c_int = 0;
    let mut pch: *mut libc::c_char = pchWord;
    let mut pw: PWord = 0 as *mut Word;
    let mut cchLength: libc::c_int = 0 as libc::c_int;
    bzero(
        cchFrequency.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            .wrapping_mul(ALPHABET as libc::c_ulong),
    );
    loop
    /* Zero(cchFrequency); */
    /* Build frequency table */
    {
        let fresh5 = pch;
        pch = pch.offset(1);
        i = *fresh5 as libc::c_int;
        if !(i != '\u{0}' as i32) {
            break;
        }
        if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            continue;
        }
        i = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = i;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    })
                } else {
                    __res = tolower(i)
                }
            } else {
                __res = *(*__ctype_tolower_loc()).offset(i as isize)
            }
            __res
        }) - 'a' as i32;
        cchFrequency[i as usize] = cchFrequency[i as usize].wrapping_add(1);
        if cchFrequency[i as usize] as libc::c_uint > alPhrase[i as usize].uFrequency {
            return;
        }
        cchLength += 1
    }
    /* Update global count */
    i = 0 as libc::c_int;
    while i < ALPHABET {
        auGlobalFrequency[i as usize] =
            auGlobalFrequency[i as usize].wrapping_add(cchFrequency[i as usize] as libc::c_uint);
        i += 1
    }
    /* Create a Word structure and fill it in, including building the
     * bitfield of frequencies.
     */
    pw = NextWord();
    bzero(
        (*pw).aqMask.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<Quad>() as libc::c_ulong).wrapping_mul(MAX_QUADS as libc::c_ulong),
    );
    /* Zero(pw->aqMask); */
    (*pw).pchWord = pchWord;
    (*pw).cchLength = cchLength as libc::c_uint;
    i = 0 as libc::c_int;
    while i < ALPHABET {
        (*pw).aqMask[alPhrase[i as usize].iq as usize] |=
            (cchFrequency[i as usize] as Quad) << alPhrase[i as usize].uShift;
        i += 1
    }
}
/* AddWords -- build the list of candidates */
#[no_mangle]
pub unsafe extern "C" fn AddWords() {
    let mut pch: *mut libc::c_char = pchDictionary; /* walk through the dictionary */
    cpwCand = 0 as libc::c_int as libc::c_uint;
    while *pch != 0 {
        if *pch.offset(1 as libc::c_int as isize) as libc::c_int >= cchMinLength
            && *pch.offset(1 as libc::c_int as isize) as libc::c_int + cchMinLength
                <= cchPhraseLength
            || *pch.offset(1 as libc::c_int as isize) as libc::c_int == cchPhraseLength
        {
            BuildWord(pch.offset(2 as libc::c_int as isize));
        }
        pch = pch.offset(*pch as libc::c_int as isize)
    }
    fprintf(
        stderr,
        b"%d candidates\n\x00" as *const u8 as *const libc::c_char,
        cpwCand,
    );
}
#[no_mangle]
pub unsafe extern "C" fn DumpCandidates() {
    let mut u: libc::c_uint = 0;
    u = 0 as libc::c_int as libc::c_uint;
    while u < cpwCand {
        printf(
            StringFormat.as_ptr(),
            (*apwCand[u as usize]).pchWord,
            if u.wrapping_rem(4 as libc::c_int as libc::c_uint) == 3 as libc::c_int as libc::c_uint
            {
                '\n' as i32
            } else {
                ' ' as i32
            },
        );
        u = u.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub static mut apwSol: [PWord; 51] = [0 as *const Word as *mut Word; 51];
/* the answers */
#[no_mangle]
pub static mut cpwLast: libc::c_int = 0;
/* End of debug code */
#[no_mangle]
pub unsafe extern "C" fn DumpWords() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < cpwLast {
        wprint((*apwSol[i as usize]).pchWord);
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub static mut jbAnagram: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub unsafe extern "C" fn FindAnagram(
    mut pqMask: *mut Quad,
    mut ppwStart: PPWord,
    mut iLetter: libc::c_int,
) {
    let mut aqNext: [Quad; 2] = [0; 2];
    let mut pw: PWord = 0 as *mut Word;
    let mut qMask: Quad = 0;
    let mut iq: libc::c_uint = 0;
    let mut ppwEnd: PPWord =
        &mut *apwCand.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut PWord;
    ppwEnd = ppwEnd.offset(cpwCand as isize);
    loop {
        iq = alPhrase[achByFrequency[iLetter as usize] as usize].iq;
        qMask = (alPhrase[achByFrequency[iLetter as usize] as usize].uBits
            << alPhrase[achByFrequency[iLetter as usize] as usize].uShift) as Quad;
        if *pqMask.offset(iq as isize) & qMask != 0 {
            break;
        }
        iLetter += 1
    }
    while ppwStart < ppwEnd {
        /* Half of the program execution */
        pw = *ppwStart; /* time is spent in these three */
        aqNext[0 as libc::c_int as usize] = (*pqMask.offset(0 as libc::c_int as isize))
            .wrapping_sub((*pw).aqMask[0 as libc::c_int as usize]);
        if aqNext[0 as libc::c_int as usize] & aqMainSign[0 as libc::c_int as usize] != 0 {
            ppwStart = ppwStart.offset(1)
        } else {
            /* lines of code. */
            aqNext[1 as libc::c_int as usize] = (*pqMask.offset(1 as libc::c_int as isize))
                .wrapping_sub((*pw).aqMask[1 as libc::c_int as usize]);
            if aqNext[1 as libc::c_int as usize] & aqMainSign[1 as libc::c_int as usize] != 0 {
                ppwStart = ppwStart.offset(1)
            } else if (*pw).aqMask[iq as usize] & qMask == 0 as libc::c_int as libc::c_ulong {
                ppwEnd = ppwEnd.offset(-1);
                *ppwStart = *ppwEnd;
                *ppwEnd = pw
            } else {
                /* If the pivot letter isn't present, defer this word until later */
                /* If we get here, this means the word fits. */
                let fresh6 = cpwLast; /* found one */
                cpwLast = cpwLast + 1;
                apwSol[fresh6 as usize] = pw;
                cchPhraseLength = (cchPhraseLength as libc::c_uint).wrapping_sub((*pw).cchLength)
                    as libc::c_int as libc::c_int;
                if cchPhraseLength != 0 {
                    /* recurse */
                    /* The recursive call scrambles the tail, so we have to be
                     * pessimistic.
                     */
                    ppwEnd =
                        &mut *apwCand.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut PWord;
                    ppwEnd = ppwEnd.offset(cpwCand as isize);
                    FindAnagram(
                        &mut *aqNext.as_mut_ptr().offset(0 as libc::c_int as isize),
                        ppwStart,
                        iLetter,
                    );
                } else {
                    DumpWords();
                }
                cchPhraseLength = (cchPhraseLength as libc::c_uint).wrapping_add((*pw).cchLength)
                    as libc::c_int as libc::c_int;
                cpwLast -= 1;
                ppwStart = ppwStart.offset(1)
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CompareFrequency(
    mut pch1: *mut libc::c_char,
    mut pch2: *mut libc::c_char,
) -> libc::c_int {
    return if auGlobalFrequency[*pch1 as usize] < auGlobalFrequency[*pch2 as usize] {
        -(1 as libc::c_int)
    } else if auGlobalFrequency[*pch1 as usize] == auGlobalFrequency[*pch2 as usize] {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SortCandidates() {
    let mut i: libc::c_int = 0;
    /* Sort the letters by frequency */
    i = 0 as libc::c_int;
    while i < ALPHABET {
        achByFrequency[i as usize] = i as libc::c_char;
        i += 1
    }
    qsort(
        achByFrequency.as_mut_ptr() as *mut libc::c_void,
        ALPHABET as size_t,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int>,
            Option<
                unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
            >,
        >(Some(
            CompareFrequency
                as unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_char) -> libc::c_int,
        )),
    );
    fprintf(
        stderr,
        b"Order of search will be \x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < ALPHABET {
        fputc(
            achByFrequency[i as usize] as libc::c_int + 'a' as i32,
            stderr,
        );
        i += 1
    }
    fputc('\n' as i32, stderr);
}
#[no_mangle]
pub static mut fInteractive: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn GetPhrase(mut pch: *mut libc::c_char) -> *mut libc::c_char {
    if fInteractive != 0 {
        printf(b">\x00" as *const u8 as *const libc::c_char);
    }
    fflush(stdout);
    if (gets(pch) as *mut libc::c_void).is_null() {
        /* PLUS_STATS */
        exit(0 as libc::c_int);
    }
    return pch;
}
#[no_mangle]
pub static mut achPhrase: [libc::c_char; 255] = [0; 255];
unsafe fn main_0(mut cpchArgc: libc::c_int, mut ppchArgv: *mut *mut libc::c_char) -> libc::c_int {
    if cpchArgc != 2 as libc::c_int && cpchArgc != 3 as libc::c_int {
        Fatal(
            b"Usage: anagram dictionary [length]\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    }
    if cpchArgc == 3 as libc::c_int {
        cchMinLength = atoi(*ppchArgv.offset(2 as libc::c_int as isize))
    }
    fInteractive = isatty(1 as libc::c_int);
    ReadDict(*ppchArgv.offset(1 as libc::c_int as isize));
    while !GetPhrase(&mut *achPhrase.as_mut_ptr().offset(0 as libc::c_int as isize)).is_null() {
        if *(*__ctype_b_loc()).offset(achPhrase[0 as libc::c_int as usize] as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            cchMinLength = atoi(achPhrase.as_mut_ptr());
            printf(
                b"New length: %d\n\x00" as *const u8 as *const libc::c_char,
                cchMinLength,
            );
        } else if achPhrase[0 as libc::c_int as usize] as libc::c_int == '?' as i32 {
            DumpCandidates();
        } else {
            BuildMask(&mut *achPhrase.as_mut_ptr().offset(0 as libc::c_int as isize));
            AddWords();
            if cpwCand == 0 as libc::c_int as libc::c_uint || cchPhraseLength == 0 as libc::c_int {
                continue;
            }
            cpwLast = 0 as libc::c_int;
            SortCandidates();
            if _setjmp(jbAnagram.as_mut_ptr()) == 0 as libc::c_int {
                FindAnagram(
                    &mut *aqMainMask.as_mut_ptr().offset(0 as libc::c_int as isize),
                    &mut *apwCand.as_mut_ptr().offset(0 as libc::c_int as isize),
                    0 as libc::c_int,
                );
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
