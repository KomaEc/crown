use ::libc;
extern "C" {
    fn zswap(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zseti(_: *mut C2RustUnnamed, _: libc::c_longlong);
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    fn zsub(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    static mut libzahl_tmp_divmod_ds: [z_t; 32];
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zbset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t, _: libc::c_int);
    static mut libzahl_tmp_divmod_d: z_t;
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zlsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zbits(_: *mut C2RustUnnamed) -> size_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zdivmod(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
    mut d: *mut C2RustUnnamed,
) {
    let mut c_bits: size_t = 0;
    let mut d_bits: size_t = 0;
    let mut bit: size_t = 0;
    let mut sign: libc::c_int = 0;
    let mut cmpmag: libc::c_int = 0;
    sign = zsignum(c) * zsignum(d);
    if sign == 0 {
        if zzero(c) != 0 {
            if zzero(d) != 0 {
                libzahl_error = 33 as libc::c_int;
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            } else {
                (*a).sign = 0 as libc::c_int;
                (*b).sign = 0 as libc::c_int;
            }
        } else {
            libzahl_error = 33 as libc::c_int;
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        }
        return;
    } else {
        cmpmag = zcmpmag(c, d);
        if cmpmag <= 0 as libc::c_int {
            if cmpmag == 0 as libc::c_int {
                zseti(a, sign as libc::c_longlong);
                (*b).sign = 0 as libc::c_int;
                return;
            } else {
                if b != c {
                    zset(b, c);
                }
            }
            (*b).sign = 1 as libc::c_int;
            (*a).sign = 0 as libc::c_int;
            return;
        }
    }
    c_bits = zbits(c);
    d_bits = zbits(d);
    bit = c_bits.wrapping_sub(d_bits);
    zlsh(libzahl_tmp_divmod_d.as_mut_ptr(), d, bit);
    (*libzahl_tmp_divmod_d.as_mut_ptr()).sign = 1 as libc::c_int;
    if zcmpmag(libzahl_tmp_divmod_d.as_mut_ptr(), c) > 0 as libc::c_int {
        zrsh(
            libzahl_tmp_divmod_d.as_mut_ptr(),
            libzahl_tmp_divmod_d.as_mut_ptr(),
            1 as libc::c_int as size_t,
        );
        bit = (bit as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    (*libzahl_tmp_divmod_a.as_mut_ptr()).sign = 0 as libc::c_int;
    zabs(libzahl_tmp_divmod_b.as_mut_ptr(), c);
    if bit < 32 as libc::c_int as libc::c_ulong {
        loop {
            if zcmpmag(
                libzahl_tmp_divmod_d.as_mut_ptr(),
                libzahl_tmp_divmod_b.as_mut_ptr(),
            ) <= 0 as libc::c_int
            {
                zsub(
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                    libzahl_tmp_divmod_d.as_mut_ptr(),
                );
                zbset(
                    libzahl_tmp_divmod_a.as_mut_ptr(),
                    libzahl_tmp_divmod_a.as_mut_ptr(),
                    bit,
                    1 as libc::c_int,
                );
            }
            let fresh0 = bit;
            bit = bit.wrapping_sub(1);
            if fresh0 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                break;
            }
            zrsh(
                libzahl_tmp_divmod_d.as_mut_ptr(),
                libzahl_tmp_divmod_d.as_mut_ptr(),
                1 as libc::c_int as size_t,
            );
        }
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < 32 as libc::c_int as libc::c_ulong {
            zrsh(
                (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                libzahl_tmp_divmod_d.as_mut_ptr(),
                i,
            );
            i = i.wrapping_add(1);
        }
        's_253: loop {
            i = 0 as libc::c_int as size_t;
            while i < 32 as libc::c_int as libc::c_ulong {
                if zcmpmag(
                    (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                ) <= 0 as libc::c_int
                {
                    zsub(
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                    );
                    zbset(
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        bit,
                        1 as libc::c_int,
                    );
                }
                let fresh1 = bit;
                bit = bit.wrapping_sub(1);
                if fresh1 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                    break 's_253;
                }
                i = i.wrapping_add(1);
            }
            i = (if bit < (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
                bit
            } else {
                (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            loop {
                let fresh2 = i;
                i = i.wrapping_sub(1);
                if !(fresh2 != 0) {
                    break;
                }
                zrsh(
                    (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                    (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                    32 as libc::c_int as size_t,
                );
            }
        }
    }
    zswap(a, libzahl_tmp_divmod_a.as_mut_ptr());
    zswap(b, libzahl_tmp_divmod_b.as_mut_ptr());
    (*a).sign = sign;
}
