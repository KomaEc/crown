
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: std::os::raw::c_uchar,
    pub bits: std::os::raw::c_uchar,
    pub val: std::os::raw::c_ushort,
}
pub type codetype = std::os::raw::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
#[no_mangle]
pub static mut inflate_copyright: [std::os::raw::c_char; 48] =
    unsafe {
        *::std::mem::transmute::<&[u8; 48],
                                 &[std::os::raw::c_char; 48]>(b" inflate 1.2.11 Copyright 1995-2017 Mark Adler \x00")
    };
/*
  If you use the zlib library in a product, an acknowledgment is welcome
  in the documentation of your product. If for some reason you cannot
  include such an acknowledgment, I would appreciate that you keep this
  copyright string in the executable of your product.
 */
/*
   Build a set of tables to decode the provided canonical Huffman code.
   The code lengths are lens[0..codes-1].  The result starts at *table,
   whose indices are 0..2^bits-1.  work is a writable array of at least
   lens shorts, which is used as a work area.  type is the type of code
   to be generated, CODES, LENS, or DISTS.  On return, zero is success,
   -1 is an invalid code, and +1 means that ENOUGH isn't enough.  table
   on return points to the next available entry's address.  bits is the
   requested root table index bits, and on return it is the actual root
   table index bits.  It will differ if the request is greater than the
   longest code or if it is less than the shortest code.
 */
#[no_mangle]
pub unsafe extern "C" fn inflate_table(mut type_0: codetype,
                                       mut lens: *mut std::os::raw::c_ushort,
                                       mut codes: std::os::raw::c_uint,
                                       mut table: *mut *mut code,
                                       mut bits: *mut std::os::raw::c_uint,
                                       mut work: *mut std::os::raw::c_ushort)
 -> std::os::raw::c_int {
    let mut len: std::os::raw::c_uint = 0; /* a code's length in bits */
    let mut sym: std::os::raw::c_uint = 0; /* index of code symbols */
    let mut min: std::os::raw::c_uint = 0; /* minimum and maximum code lengths */
    let mut max: std::os::raw::c_uint = 0; /* number of index bits for root table */
    let mut root: std::os::raw::c_uint =
        0; /* number of index bits for current table */
    let mut curr: std::os::raw::c_uint = 0; /* code bits to drop for sub-table */
    let mut drop_0: std::os::raw::c_uint = 0; /* number of prefix codes available */
    let mut left: std::os::raw::c_int = 0; /* code entries in table used */
    let mut used: std::os::raw::c_uint = 0; /* Huffman code */
    let mut huff: std::os::raw::c_uint = 0; /* for incrementing code, index */
    let mut incr: std::os::raw::c_uint = 0; /* index for replicating entries */
    let mut fill: std::os::raw::c_uint = 0; /* low bits for current root entry */
    let mut low: std::os::raw::c_uint = 0; /* mask for low root bits */
    let mut mask: std::os::raw::c_uint = 0; /* table entry for duplication */
    let mut here: code =
        code{op: 0, bits: 0, val: 0,}; /* next available space in table */
    let mut next: *mut code = 0 as *mut code; /* base value table to use */
    let mut base: *const std::os::raw::c_ushort =
        0 as *const std::os::raw::c_ushort; /* extra bits table to use */
    let mut extra: *const std::os::raw::c_ushort =
        0 as
            *const std::os::raw::c_ushort; /* use base and extra for symbol >= match */
    let mut match_0: std::os::raw::c_uint = 0; /* number of codes of each length */
    let mut count: [std::os::raw::c_ushort; 16] =
        [0; 16]; /* offsets in table for each length */
    let mut offs: [std::os::raw::c_ushort; 16] = [0; 16];
    static mut lbase: [std::os::raw::c_ushort; 31] =
        [3 as std::os::raw::c_int as std::os::raw::c_ushort,
         4 as std::os::raw::c_int as std::os::raw::c_ushort,
         5 as std::os::raw::c_int as std::os::raw::c_ushort,
         6 as std::os::raw::c_int as std::os::raw::c_ushort,
         7 as std::os::raw::c_int as std::os::raw::c_ushort,
         8 as std::os::raw::c_int as std::os::raw::c_ushort,
         9 as std::os::raw::c_int as std::os::raw::c_ushort,
         10 as std::os::raw::c_int as std::os::raw::c_ushort,
         11 as std::os::raw::c_int as std::os::raw::c_ushort,
         13 as std::os::raw::c_int as std::os::raw::c_ushort,
         15 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         19 as std::os::raw::c_int as std::os::raw::c_ushort,
         23 as std::os::raw::c_int as std::os::raw::c_ushort,
         27 as std::os::raw::c_int as std::os::raw::c_ushort,
         31 as std::os::raw::c_int as std::os::raw::c_ushort,
         35 as std::os::raw::c_int as std::os::raw::c_ushort,
         43 as std::os::raw::c_int as std::os::raw::c_ushort,
         51 as std::os::raw::c_int as std::os::raw::c_ushort,
         59 as std::os::raw::c_int as std::os::raw::c_ushort,
         67 as std::os::raw::c_int as std::os::raw::c_ushort,
         83 as std::os::raw::c_int as std::os::raw::c_ushort,
         99 as std::os::raw::c_int as std::os::raw::c_ushort,
         115 as std::os::raw::c_int as std::os::raw::c_ushort,
         131 as std::os::raw::c_int as std::os::raw::c_ushort,
         163 as std::os::raw::c_int as std::os::raw::c_ushort,
         195 as std::os::raw::c_int as std::os::raw::c_ushort,
         227 as std::os::raw::c_int as std::os::raw::c_ushort,
         258 as std::os::raw::c_int as std::os::raw::c_ushort,
         0 as std::os::raw::c_int as std::os::raw::c_ushort,
         0 as std::os::raw::c_int as std::os::raw::c_ushort];
    static mut lext: [std::os::raw::c_ushort; 31] =
        [16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         19 as std::os::raw::c_int as std::os::raw::c_ushort,
         19 as std::os::raw::c_int as std::os::raw::c_ushort,
         19 as std::os::raw::c_int as std::os::raw::c_ushort,
         19 as std::os::raw::c_int as std::os::raw::c_ushort,
         20 as std::os::raw::c_int as std::os::raw::c_ushort,
         20 as std::os::raw::c_int as std::os::raw::c_ushort,
         20 as std::os::raw::c_int as std::os::raw::c_ushort,
         20 as std::os::raw::c_int as std::os::raw::c_ushort,
         21 as std::os::raw::c_int as std::os::raw::c_ushort,
         21 as std::os::raw::c_int as std::os::raw::c_ushort,
         21 as std::os::raw::c_int as std::os::raw::c_ushort,
         21 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         77 as std::os::raw::c_int as std::os::raw::c_ushort,
         202 as std::os::raw::c_int as std::os::raw::c_ushort];
    static mut dbase: [std::os::raw::c_ushort; 32] =
        [1 as std::os::raw::c_int as std::os::raw::c_ushort,
         2 as std::os::raw::c_int as std::os::raw::c_ushort,
         3 as std::os::raw::c_int as std::os::raw::c_ushort,
         4 as std::os::raw::c_int as std::os::raw::c_ushort,
         5 as std::os::raw::c_int as std::os::raw::c_ushort,
         7 as std::os::raw::c_int as std::os::raw::c_ushort,
         9 as std::os::raw::c_int as std::os::raw::c_ushort,
         13 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         25 as std::os::raw::c_int as std::os::raw::c_ushort,
         33 as std::os::raw::c_int as std::os::raw::c_ushort,
         49 as std::os::raw::c_int as std::os::raw::c_ushort,
         65 as std::os::raw::c_int as std::os::raw::c_ushort,
         97 as std::os::raw::c_int as std::os::raw::c_ushort,
         129 as std::os::raw::c_int as std::os::raw::c_ushort,
         193 as std::os::raw::c_int as std::os::raw::c_ushort,
         257 as std::os::raw::c_int as std::os::raw::c_ushort,
         385 as std::os::raw::c_int as std::os::raw::c_ushort,
         513 as std::os::raw::c_int as std::os::raw::c_ushort,
         769 as std::os::raw::c_int as std::os::raw::c_ushort,
         1025 as std::os::raw::c_int as std::os::raw::c_ushort,
         1537 as std::os::raw::c_int as std::os::raw::c_ushort,
         2049 as std::os::raw::c_int as std::os::raw::c_ushort,
         3073 as std::os::raw::c_int as std::os::raw::c_ushort,
         4097 as std::os::raw::c_int as std::os::raw::c_ushort,
         6145 as std::os::raw::c_int as std::os::raw::c_ushort,
         8193 as std::os::raw::c_int as std::os::raw::c_ushort,
         12289 as std::os::raw::c_int as std::os::raw::c_ushort,
         16385 as std::os::raw::c_int as std::os::raw::c_ushort,
         24577 as std::os::raw::c_int as std::os::raw::c_ushort,
         0 as std::os::raw::c_int as std::os::raw::c_ushort,
         0 as std::os::raw::c_int as std::os::raw::c_ushort];
    static mut dext: [std::os::raw::c_ushort; 32] =
        [16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         16 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         19 as std::os::raw::c_int as std::os::raw::c_ushort,
         19 as std::os::raw::c_int as std::os::raw::c_ushort,
         20 as std::os::raw::c_int as std::os::raw::c_ushort,
         20 as std::os::raw::c_int as std::os::raw::c_ushort,
         21 as std::os::raw::c_int as std::os::raw::c_ushort,
         21 as std::os::raw::c_int as std::os::raw::c_ushort,
         22 as std::os::raw::c_int as std::os::raw::c_ushort,
         22 as std::os::raw::c_int as std::os::raw::c_ushort,
         23 as std::os::raw::c_int as std::os::raw::c_ushort,
         23 as std::os::raw::c_int as std::os::raw::c_ushort,
         24 as std::os::raw::c_int as std::os::raw::c_ushort,
         24 as std::os::raw::c_int as std::os::raw::c_ushort,
         25 as std::os::raw::c_int as std::os::raw::c_ushort,
         25 as std::os::raw::c_int as std::os::raw::c_ushort,
         26 as std::os::raw::c_int as std::os::raw::c_ushort,
         26 as std::os::raw::c_int as std::os::raw::c_ushort,
         27 as std::os::raw::c_int as std::os::raw::c_ushort,
         27 as std::os::raw::c_int as std::os::raw::c_ushort,
         28 as std::os::raw::c_int as std::os::raw::c_ushort,
         28 as std::os::raw::c_int as std::os::raw::c_ushort,
         29 as std::os::raw::c_int as std::os::raw::c_ushort,
         29 as std::os::raw::c_int as std::os::raw::c_ushort,
         64 as std::os::raw::c_int as std::os::raw::c_ushort,
         64 as std::os::raw::c_int as std::os::raw::c_ushort];
    /*
       Process a set of code lengths to create a canonical Huffman code.  The
       code lengths are lens[0..codes-1].  Each length corresponds to the
       symbols 0..codes-1.  The Huffman code is generated by first sorting the
       symbols by length from short to long, and retaining the symbol order
       for codes with equal lengths.  Then the code starts with all zero bits
       for the first code of the shortest length, and the codes are integer
       increments for the same length, and zeros are appended as the length
       increases.  For the deflate format, these bits are stored backwards
       from their more natural integer increment ordering, and so when the
       decoding tables are built in the large loop below, the integer codes
       are incremented backwards.

       This routine assumes, but does not check, that all of the entries in
       lens[] are in the range 0..MAXBITS.  The caller must assure this.
       1..MAXBITS is interpreted as that code length.  zero means that that
       symbol does not occur in this code.

       The codes are sorted by computing a count of codes for each length,
       creating from that a table of starting indices for each length in the
       sorted table, and then entering the symbols in order in the sorted
       table.  The sorted table is work[], with that space being provided by
       the caller.

       The length counts are used for other purposes as well, i.e. finding
       the minimum and maximum length codes, determining if there are any
       codes at all, checking for a valid set of lengths, and looking ahead
       at length counts to determine sub-table sizes when building the
       decoding tables.
     */
    /* accumulate lengths for codes (assumes lens[] all in 0..MAXBITS) */
    len = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while len <= 15 as std::os::raw::c_int as std::os::raw::c_uint {
        count[len as usize] = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
        len = len.wrapping_add(1)
    }
    sym = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while sym < codes {
        count[*lens.offset(sym as isize) as usize] =
            count[*lens.offset(sym as isize) as usize].wrapping_add(1);
        sym = sym.wrapping_add(1)
    }
    /* bound code lengths, force root to be within code lengths */
    root = *bits;
    max = 15 as std::os::raw::c_int as std::os::raw::c_uint;
    while max >= 1 as std::os::raw::c_int as std::os::raw::c_uint {
        if count[max as usize] as std::os::raw::c_int != 0 as std::os::raw::c_int { break ; }
        max = max.wrapping_sub(1)
    }
    if root > max { root = max }
    if max == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        /* no symbols to code at all */
        here.op = 64 as std::os::raw::c_int as std::os::raw::c_uchar;
        here.bits = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
        here.val =
            0 as std::os::raw::c_int as std::os::raw::c_ushort; /* invalid code marker */
        /* no symbols, but wait for decoding to report error */
        let fresh0 = *table; /* make a table to force an error */
        *table = (*table).offset(1);
        *fresh0 = here;
        let fresh1 = *table;
        *table = (*table).offset(1);
        *fresh1 = here;
        *bits = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        return 0 as std::os::raw::c_int
    }
    min = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    while min < max {
        if count[min as usize] as std::os::raw::c_int != 0 as std::os::raw::c_int { break ; }
        min = min.wrapping_add(1)
    }
    if root < min { root = min }
    /* check for an over-subscribed or incomplete set of lengths */
    left = 1 as std::os::raw::c_int;
    len = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    while len <= 15 as std::os::raw::c_int as std::os::raw::c_uint {
        left <<= 1 as std::os::raw::c_int;
        left -= count[len as usize] as std::os::raw::c_int;
        if left < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        len = len.wrapping_add(1)
        /* over-subscribed */
    } /* incomplete set */
    if left > 0 as std::os::raw::c_int &&
           (type_0 as std::os::raw::c_uint == CODES as std::os::raw::c_int as std::os::raw::c_uint ||
                max != 1 as std::os::raw::c_int as std::os::raw::c_uint) {
        return -(1 as std::os::raw::c_int)
    }
    /* generate offsets into symbol table for each length for sorting */
    offs[1 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
    len = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    while len < 15 as std::os::raw::c_int as std::os::raw::c_uint {
        offs[len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as usize] =
            (offs[len as usize] as std::os::raw::c_int +
                 count[len as usize] as std::os::raw::c_int) as std::os::raw::c_ushort;
        len = len.wrapping_add(1)
    }
    /* sort symbols by length, by symbol order within each length */
    sym = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while sym < codes {
        if *lens.offset(sym as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int {
            let fresh2 = offs[*lens.offset(sym as isize) as usize];
            offs[*lens.offset(sym as isize) as usize] =
                offs[*lens.offset(sym as isize) as usize].wrapping_add(1);
            *work.offset(fresh2 as isize) = sym as std::os::raw::c_ushort
        }
        sym = sym.wrapping_add(1)
    }
    /*
       Create and fill in decoding tables.  In this loop, the table being
       filled is at next and has curr index bits.  The code being used is huff
       with length len.  That code is converted to an index by dropping drop
       bits off of the bottom.  For codes where len is less than drop + curr,
       those top drop + curr - len bits are incremented through all values to
       fill the table with replicated entries.

       root is the number of index bits for the root table.  When len exceeds
       root, sub-tables are created pointed to by the root entry with an index
       of the low root bits of huff.  This is saved in low to check for when a
       new sub-table should be started.  drop is zero when the root table is
       being filled, and drop is root when sub-tables are being filled.

       When a new sub-table is needed, it is necessary to look ahead in the
       code lengths to determine what size sub-table is needed.  The length
       counts are used for this, and so count[] is decremented as codes are
       entered in the tables.

       used keeps track of how many table entries have been allocated from the
       provided *table space.  It is checked for LENS and DIST tables against
       the constants ENOUGH_LENS and ENOUGH_DISTS to guard against changes in
       the initial root table size constants.  See the comments in inftrees.h
       for more information.

       sym increments through all symbols, and the loop terminates when
       all codes of length max, i.e. all codes, have been processed.  This
       routine permits incomplete codes, so another loop after this one fills
       in the rest of the decoding tables with invalid code markers.
     */
    /* set up for code type */
    match type_0 as std::os::raw::c_uint {
        0 => {
            extra = work; /* dummy value--not used */
            base = extra;
            match_0 = 20 as std::os::raw::c_int as std::os::raw::c_uint
        }
        1 => {
            base = lbase.as_ptr();
            extra = lext.as_ptr();
            match_0 = 257 as std::os::raw::c_int as std::os::raw::c_uint
        }
        _ => {
            /* DISTS */
            base = dbase.as_ptr();
            extra = dext.as_ptr();
            match_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint
        }
    }
    /* initialize state for loop */
    huff = 0 as std::os::raw::c_int as std::os::raw::c_uint; /* starting code */
    sym = 0 as std::os::raw::c_int as std::os::raw::c_uint; /* starting code symbol */
    len = min; /* starting code length */
    next = *table; /* current table to fill in */
    curr = root; /* current table index bits */
    drop_0 =
        0 as std::os::raw::c_int as
            std::os::raw::c_uint; /* current bits to drop from code for index */
    low =
        -(1 as std::os::raw::c_int) as
            std::os::raw::c_uint; /* trigger new sub-table when len > root */
    used = (1 as std::os::raw::c_uint) << root; /* use root table entries */
    mask =
        used.wrapping_sub(1 as std::os::raw::c_int as
                              std::os::raw::c_uint); /* mask for comparing low */
    /* check available table space */
    if type_0 as std::os::raw::c_uint == LENS as std::os::raw::c_int as std::os::raw::c_uint &&
           used > 852 as std::os::raw::c_int as std::os::raw::c_uint ||
           type_0 as std::os::raw::c_uint == DISTS as std::os::raw::c_int as std::os::raw::c_uint &&
               used > 592 as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    }
    loop 
         /* process all codes and make table entries */
         /* create table entry */
         {
        here.bits =
            len.wrapping_sub(drop_0) as std::os::raw::c_uchar; /* end of block */
        if (*work.offset(sym as isize) as
                std::os::raw::c_uint).wrapping_add(1 as std::os::raw::c_uint) < match_0 {
            here.op = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            here.val = *work.offset(sym as isize)
        } else if *work.offset(sym as isize) as std::os::raw::c_uint >= match_0 {
            here.op =
                *extra.offset((*work.offset(sym as isize) as
                                   std::os::raw::c_uint).wrapping_sub(match_0) as
                                  isize) as std::os::raw::c_uchar;
            here.val =
                *base.offset((*work.offset(sym as isize) as
                                  std::os::raw::c_uint).wrapping_sub(match_0) as
                                 isize)
        } else {
            here.op =
                (32 as std::os::raw::c_int + 64 as std::os::raw::c_int) as std::os::raw::c_uchar;
            here.val = 0 as std::os::raw::c_int as std::os::raw::c_ushort
        }
        /* replicate for those indices with low len bits equal to huff */
        incr =
            (1 as std::os::raw::c_uint) <<
                len.wrapping_sub(drop_0); /* save offset to next table */
        fill = (1 as std::os::raw::c_uint) << curr;
        min = fill;
        loop  {
            fill = fill.wrapping_sub(incr);
            *next.offset((huff >> drop_0).wrapping_add(fill) as isize) = here;
            if !(fill != 0 as std::os::raw::c_int as std::os::raw::c_uint) { break ; }
        }
        /* backwards increment the len-bit code huff */
        incr =
            (1 as std::os::raw::c_uint) <<
                len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
        while huff & incr != 0 { incr >>= 1 as std::os::raw::c_int }
        if incr != 0 as std::os::raw::c_int as std::os::raw::c_uint {
            huff &= incr.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
            huff = huff.wrapping_add(incr)
        } else { huff = 0 as std::os::raw::c_int as std::os::raw::c_uint }
        /* go to next symbol, update count, len */
        sym = sym.wrapping_add(1);
        count[len as usize] = count[len as usize].wrapping_sub(1);
        if count[len as usize] as std::os::raw::c_int == 0 as std::os::raw::c_int {
            if len == max { break ; }
            len =
                *lens.offset(*work.offset(sym as isize) as isize) as
                    std::os::raw::c_uint
        }
        /* create new sub-table if needed */
        if len > root && huff & mask != low {
            /* if first time, transition to sub-tables */
            if drop_0 == 0 as std::os::raw::c_int as std::os::raw::c_uint { drop_0 = root }
            /* increment past last table */
            next = next.offset(min as isize); /* here min is 1 << curr */
            /* determine length of next table */
            curr = len.wrapping_sub(drop_0);
            left = (1 as std::os::raw::c_int) << curr;
            while curr.wrapping_add(drop_0) < max {
                left -=
                    count[curr.wrapping_add(drop_0) as usize] as std::os::raw::c_int;
                if left <= 0 as std::os::raw::c_int { break ; }
                curr = curr.wrapping_add(1);
                left <<= 1 as std::os::raw::c_int
            }
            /* check for enough space */
            used = used.wrapping_add((1 as std::os::raw::c_uint) << curr);
            if type_0 as std::os::raw::c_uint == LENS as std::os::raw::c_int as std::os::raw::c_uint
                   && used > 852 as std::os::raw::c_int as std::os::raw::c_uint ||
                   type_0 as std::os::raw::c_uint ==
                       DISTS as std::os::raw::c_int as std::os::raw::c_uint &&
                       used > 592 as std::os::raw::c_int as std::os::raw::c_uint {
                return 1 as std::os::raw::c_int
            }
            /* point entry in root table to sub-table */
            low = huff & mask;
            (*(*table).offset(low as isize)).op = curr as std::os::raw::c_uchar;
            (*(*table).offset(low as isize)).bits = root as std::os::raw::c_uchar;
            (*(*table).offset(low as isize)).val =
                next.offset_from(*table) as std::os::raw::c_long as
                    std::os::raw::c_ushort
        }
    }
    /* fill in remaining table entry if code is incomplete (guaranteed to have
       at most one remaining entry, since if the code is incomplete, the
       maximum code length that was allowed to get this far is one bit) */
    if huff != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        here.op =
            64 as std::os::raw::c_int as std::os::raw::c_uchar; /* invalid code marker */
        here.bits = len.wrapping_sub(drop_0) as std::os::raw::c_uchar;
        here.val = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
        *next.offset(huff as isize) = here
    }
    /* set return parameters */
    *table = (*table).offset(used as isize);
    *bits = root;
    return 0 as std::os::raw::c_int;
}
