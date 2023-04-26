
extern "C" {
    #[no_mangle]
    fn BZ2_bz__AssertH__fail(errcode: std::os::raw::c_int);
}
pub type Bool = std::os::raw::c_uchar;
pub type UChar = std::os::raw::c_uchar;
pub type Int32 = std::os::raw::c_int;
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbMakeCodeLengths(mut len: *mut UChar,
                                               mut freq: *mut Int32,
                                               mut alphaSize: Int32,
                                               mut maxLen: Int32) {
    /*--
      Nodes and heap entries run from 1.  Entry 0
      for both the heap and nodes is a sentinel.
   --*/
    let mut nNodes: Int32 = 0;
    let mut nHeap: Int32 = 0;
    let mut n1: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut tooLong: Bool = 0;
    let mut heap: [Int32; 260] = [0; 260];
    let mut weight: [Int32; 516] = [0; 516];
    let mut parent: [Int32; 516] = [0; 516];
    i = 0 as std::os::raw::c_int;
    while i < alphaSize {
        weight[(i + 1 as std::os::raw::c_int) as usize] =
            (if *freq.offset(i as isize) == 0 as std::os::raw::c_int {
                 1 as std::os::raw::c_int
             } else { *freq.offset(i as isize) }) << 8 as std::os::raw::c_int;
        i += 1
    }
    while 1 as std::os::raw::c_int as Bool != 0 {
        nNodes = alphaSize;
        nHeap = 0 as std::os::raw::c_int;
        heap[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int;
        weight[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int;
        parent[0 as std::os::raw::c_int as usize] = -(2 as std::os::raw::c_int);
        i = 1 as std::os::raw::c_int;
        while i <= alphaSize {
            parent[i as usize] = -(1 as std::os::raw::c_int);
            nHeap += 1;
            heap[nHeap as usize] = i;
            let mut zz: Int32 = 0;
            let mut tmp: Int32 = 0;
            zz = nHeap;
            tmp = heap[zz as usize];
            while weight[tmp as usize] <
                      weight[heap[(zz >> 1 as std::os::raw::c_int) as usize] as usize]
                  {
                heap[zz as usize] = heap[(zz >> 1 as std::os::raw::c_int) as usize];
                zz >>= 1 as std::os::raw::c_int
            }
            heap[zz as usize] = tmp;
            i += 1
        }
        if !(nHeap < 258 as std::os::raw::c_int + 2 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(2001 as std::os::raw::c_int);
        }
        while nHeap > 1 as std::os::raw::c_int {
            n1 = heap[1 as std::os::raw::c_int as usize];
            heap[1 as std::os::raw::c_int as usize] = heap[nHeap as usize];
            nHeap -= 1;
            let mut zz_0: Int32 = 0;
            let mut yy: Int32 = 0;
            let mut tmp_0: Int32 = 0;
            zz_0 = 1 as std::os::raw::c_int;
            tmp_0 = heap[zz_0 as usize];
            while 1 as std::os::raw::c_int as Bool != 0 {
                yy = zz_0 << 1 as std::os::raw::c_int;
                if yy > nHeap { break ; }
                if yy < nHeap &&
                       weight[heap[(yy + 1 as std::os::raw::c_int) as usize] as usize]
                           < weight[heap[yy as usize] as usize] {
                    yy += 1
                }
                if weight[tmp_0 as usize] < weight[heap[yy as usize] as usize]
                   {
                    break ;
                }
                heap[zz_0 as usize] = heap[yy as usize];
                zz_0 = yy
            }
            heap[zz_0 as usize] = tmp_0;
            n2 = heap[1 as std::os::raw::c_int as usize];
            heap[1 as std::os::raw::c_int as usize] = heap[nHeap as usize];
            nHeap -= 1;
            let mut zz_1: Int32 = 0;
            let mut yy_0: Int32 = 0;
            let mut tmp_1: Int32 = 0;
            zz_1 = 1 as std::os::raw::c_int;
            tmp_1 = heap[zz_1 as usize];
            while 1 as std::os::raw::c_int as Bool != 0 {
                yy_0 = zz_1 << 1 as std::os::raw::c_int;
                if yy_0 > nHeap { break ; }
                if yy_0 < nHeap &&
                       weight[heap[(yy_0 + 1 as std::os::raw::c_int) as usize] as
                                  usize] <
                           weight[heap[yy_0 as usize] as usize] {
                    yy_0 += 1
                }
                if weight[tmp_1 as usize] <
                       weight[heap[yy_0 as usize] as usize] {
                    break ;
                }
                heap[zz_1 as usize] = heap[yy_0 as usize];
                zz_1 = yy_0
            }
            heap[zz_1 as usize] = tmp_1;
            nNodes += 1;
            parent[n2 as usize] = nNodes;
            parent[n1 as usize] = parent[n2 as usize];
            weight[nNodes as usize] =
                ((weight[n1 as usize] as std::os::raw::c_uint &
                      0xffffff00 as
                          std::os::raw::c_uint).wrapping_add(weight[n2 as usize] as
                                                         std::os::raw::c_uint &
                                                         0xffffff00 as
                                                             std::os::raw::c_uint) |
                     (1 as std::os::raw::c_int +
                          (if weight[n1 as usize] & 0xff as std::os::raw::c_int >
                                  weight[n2 as usize] & 0xff as std::os::raw::c_int {
                               (weight[n1 as usize]) & 0xff as std::os::raw::c_int
                           } else {
                               (weight[n2 as usize]) & 0xff as std::os::raw::c_int
                           })) as std::os::raw::c_uint) as Int32;
            parent[nNodes as usize] = -(1 as std::os::raw::c_int);
            nHeap += 1;
            heap[nHeap as usize] = nNodes;
            let mut zz_2: Int32 = 0;
            let mut tmp_2: Int32 = 0;
            zz_2 = nHeap;
            tmp_2 = heap[zz_2 as usize];
            while weight[tmp_2 as usize] <
                      weight[heap[(zz_2 >> 1 as std::os::raw::c_int) as usize] as
                                 usize] {
                heap[zz_2 as usize] =
                    heap[(zz_2 >> 1 as std::os::raw::c_int) as usize];
                zz_2 >>= 1 as std::os::raw::c_int
            }
            heap[zz_2 as usize] = tmp_2
        }
        if !(nNodes < 258 as std::os::raw::c_int * 2 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(2002 as std::os::raw::c_int);
        }
        tooLong = 0 as std::os::raw::c_int as Bool;
        i = 1 as std::os::raw::c_int;
        while i <= alphaSize {
            j = 0 as std::os::raw::c_int;
            k = i;
            while parent[k as usize] >= 0 as std::os::raw::c_int {
                k = parent[k as usize];
                j += 1
            }
            *len.offset((i - 1 as std::os::raw::c_int) as isize) = j as UChar;
            if j > maxLen { tooLong = 1 as std::os::raw::c_int as Bool }
            i += 1
        }
        if tooLong == 0 { break ; }
        /* 17 Oct 04: keep-going condition for the following loop used
         to be 'i < alphaSize', which missed the last element,
         theoretically leading to the possibility of the compressor
         looping.  However, this count-scaling step is only needed if
         one of the generated Huffman code words is longer than
         maxLen, which up to and including version 1.0.2 was 20 bits,
         which is extremely unlikely.  In version 1.0.3 maxLen was
         changed to 17 bits, which has minimal effect on compression
         ratio, but does mean this scaling step is used from time to
         time, enough to verify that it works.

         This means that bzip2-1.0.3 and later will only produce
         Huffman codes with a maximum length of 17 bits.  However, in
         order to preserve backwards compatibility with bitstreams
         produced by versions pre-1.0.3, the decompressor must still
         handle lengths of up to 20. */
        i = 1 as std::os::raw::c_int;
        while i <= alphaSize {
            j = weight[i as usize] >> 8 as std::os::raw::c_int;
            j = 1 as std::os::raw::c_int + j / 2 as std::os::raw::c_int;
            weight[i as usize] = j << 8 as std::os::raw::c_int;
            i += 1
        }
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbAssignCodes(mut code: *mut Int32,
                                           mut length: *mut UChar,
                                           mut minLen: Int32,
                                           mut maxLen: Int32,
                                           mut alphaSize: Int32) {
    let mut n: Int32 = 0;
    let mut vec: Int32 = 0;
    let mut i: Int32 = 0;
    vec = 0 as std::os::raw::c_int;
    n = minLen;
    while n <= maxLen {
        i = 0 as std::os::raw::c_int;
        while i < alphaSize {
            if *length.offset(i as isize) as std::os::raw::c_int == n {
                *code.offset(i as isize) = vec;
                vec += 1
            }
            i += 1
        }
        vec <<= 1 as std::os::raw::c_int;
        n += 1
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_hbCreateDecodeTables(mut limit: *mut Int32,
                                                  mut base: *mut Int32,
                                                  mut perm: *mut Int32,
                                                  mut length: *mut UChar,
                                                  mut minLen: Int32,
                                                  mut maxLen: Int32,
                                                  mut alphaSize: Int32) {
    let mut pp: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut vec: Int32 = 0;
    pp = 0 as std::os::raw::c_int;
    i = minLen;
    while i <= maxLen {
        j = 0 as std::os::raw::c_int;
        while j < alphaSize {
            if *length.offset(j as isize) as std::os::raw::c_int == i {
                *perm.offset(pp as isize) = j;
                pp += 1
            }
            j += 1
        }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 23 as std::os::raw::c_int {
        *base.offset(i as isize) = 0 as std::os::raw::c_int;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < alphaSize {
        let ref mut fresh0 =
            *base.offset((*length.offset(i as isize) as std::os::raw::c_int +
                              1 as std::os::raw::c_int) as isize);
        *fresh0 += 1;
        i += 1
    }
    i = 1 as std::os::raw::c_int;
    while i < 23 as std::os::raw::c_int {
        let ref mut fresh1 = *base.offset(i as isize);
        *fresh1 += *base.offset((i - 1 as std::os::raw::c_int) as isize);
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 23 as std::os::raw::c_int {
        *limit.offset(i as isize) = 0 as std::os::raw::c_int;
        i += 1
    }
    vec = 0 as std::os::raw::c_int;
    i = minLen;
    while i <= maxLen {
        vec +=
            *base.offset((i + 1 as std::os::raw::c_int) as isize) -
                *base.offset(i as isize);
        *limit.offset(i as isize) = vec - 1 as std::os::raw::c_int;
        vec <<= 1 as std::os::raw::c_int;
        i += 1
    }
    i = minLen + 1 as std::os::raw::c_int;
    while i <= maxLen {
        *base.offset(i as isize) =
            ((*limit.offset((i - 1 as std::os::raw::c_int) as isize) +
                  1 as std::os::raw::c_int) << 1 as std::os::raw::c_int) -
                *base.offset(i as isize);
        i += 1
    };
}
/*-------------------------------------------------------------*/
/*--- end                                         huffman.c ---*/
/*-------------------------------------------------------------*/
