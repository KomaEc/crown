use ::libc;
extern "C" {
    
    
    
    
    
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    static mut libzahl_tmp_divmod_ds: [z_t; 32];
    
    
    static mut libzahl_tmp_divmod_d: z_t;
    
    
    
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    fn longjmp(_: *mut crate::src::allocator::__jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]

struct ErasedByPreprocessor11 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor12 { dummy: () }
pub type jmp_buf = [crate::src::allocator::__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor13 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zdivmod(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
    mut d: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut c_bits: size_t = 0;
    let mut d_bits: size_t = 0;
    let mut bit: size_t = 0;
    let mut sign: libc::c_int = 0;
    let mut cmpmag: libc::c_int = 0;
    sign= zsignum(c) * zsignum(d);
    if sign == 0 {
        if zzero(c) != 0 {
            if zzero(d) != 0 {
                crate::src::zdivmod::libzahl_error= 33 as libc::c_int;
                longjmp(crate::src::zdivmod::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            } else {
                (*a).sign= 0 as libc::c_int;
                (*b).sign= 0 as libc::c_int;
            }
        } else {
            crate::src::zdivmod::libzahl_error= 33 as libc::c_int;
            longjmp(crate::src::zdivmod::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        }
        return;
    } else {
        cmpmag= crate::src::zcmpmag::zcmpmag(c, d);
        if cmpmag <= 0 as libc::c_int {
            if cmpmag == 0 as libc::c_int {
                crate::src::zseti::zseti(a.as_mut(), sign as libc::c_longlong);
                (*b).sign= 0 as libc::c_int;
                return;
            } else {
                if b != c {
                    crate::src::zset::zset(b, c);
                }
            }
            (*b).sign= 1 as libc::c_int;
            (*a).sign= 0 as libc::c_int;
            return;
        }
    }
    c_bits= crate::src::zbits::zbits(c.as_mut());
    d_bits= crate::src::zbits::zbits(d.as_mut());
    bit= c_bits.wrapping_sub(d_bits);
    crate::src::zlsh::zlsh(crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(), d, bit);
    (*crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr()).sign = 1 as libc::c_int;
    if crate::src::zcmpmag::zcmpmag(crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(), c) > 0 as libc::c_int {
        crate::src::zrsh::zrsh(
            crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(),
            crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(),
            1 as libc::c_int as size_t,
        );
        bit= (bit as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    (*crate::src::zdivmod::libzahl_tmp_divmod_a.as_mut_ptr()).sign = 0 as libc::c_int;
    crate::src::zabs::zabs(crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr(), c);
    if bit < 32 as libc::c_int as libc::c_ulong {
        loop {
            if crate::src::zcmpmag::zcmpmag(
                crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(),
                crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr(),
            ) <= 0 as libc::c_int
            {
                crate::src::zsub::zsub(
                    crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr(),
                    crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr(),
                    crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(),
                );
                crate::src::zbset::zbset(
                    crate::src::zdivmod::libzahl_tmp_divmod_a.as_mut_ptr(),
                    crate::src::zdivmod::libzahl_tmp_divmod_a.as_mut_ptr(),
                    bit,
                    1 as libc::c_int,
                );
            }
            let fresh0 = bit;
            bit= bit.wrapping_sub(1);
            if fresh0 == 0 || zzero(crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                break;
            }
            crate::src::zrsh::zrsh(
                crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(),
                crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(),
                1 as libc::c_int as size_t,
            );
        }
    } else {
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < 32 as libc::c_int as libc::c_ulong {
            crate::src::zrsh::zrsh(
                crate::src::zdivmod::libzahl_tmp_divmod_ds[i as usize].as_mut_ptr(),
                crate::src::zdivmod::libzahl_tmp_divmod_d.as_mut_ptr(),
                i,
            );
            i= i.wrapping_add(1);
        }
        's_253: loop {
            i= 0 as libc::c_int as size_t;
            while i < 32 as libc::c_int as libc::c_ulong {
                if crate::src::zcmpmag::zcmpmag(
                    crate::src::zdivmod::libzahl_tmp_divmod_ds[i as usize].as_mut_ptr(),
                    crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr(),
                ) <= 0 as libc::c_int
                {
                    crate::src::zsub::zsub(
                        crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr(),
                        crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr(),
                        crate::src::zdivmod::libzahl_tmp_divmod_ds[i as usize].as_mut_ptr(),
                    );
                    crate::src::zbset::zbset(
                        crate::src::zdivmod::libzahl_tmp_divmod_a.as_mut_ptr(),
                        crate::src::zdivmod::libzahl_tmp_divmod_a.as_mut_ptr(),
                        bit,
                        1 as libc::c_int,
                    );
                }
                let fresh1 = bit;
                bit= bit.wrapping_sub(1);
                if fresh1 == 0 || zzero(crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                    break 's_253;
                }
                i= i.wrapping_add(1);
            }
            i= (if bit < (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
                bit
            } else {
                (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            loop {
                let fresh2 = i;
                i= i.wrapping_sub(1);
                if !(fresh2 != 0) {
                    break;
                }
                crate::src::zrsh::zrsh(
                    crate::src::zdivmod::libzahl_tmp_divmod_ds[i as usize].as_mut_ptr(),
                    crate::src::zdivmod::libzahl_tmp_divmod_ds[i as usize].as_mut_ptr(),
                    32 as libc::c_int as size_t,
                );
            }
        }
    }
    crate::src::zswap::zswap(a.as_mut(), crate::src::zdivmod::libzahl_tmp_divmod_a.as_mut_ptr().as_mut());
    crate::src::zswap::zswap(b.as_mut(), crate::src::zdivmod::libzahl_tmp_divmod_b.as_mut_ptr().as_mut());
    (*a).sign= sign;
}
