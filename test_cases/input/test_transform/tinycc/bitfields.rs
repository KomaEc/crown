macro_rules! ranges {
    () => ();

    ($get:ident/$set:ident: $ty:ty = $range:expr $(; $($rest:tt)*)?) => {
        ranges!(@getter $get: $ty = $range);
        ranges!(@setter $set: $ty = $range);

        ranges! { $($($rest)*)? }
    };

    (@getter $name:ident: $ty:ty = $range:expr) => {
        pub fn $name(&self) -> $ty { get_bits(&self.0, $range) }
    };

    (@setter $name:ident: $ty:ty = $range:expr) => {
        pub fn $name(&mut self, __val: $ty) { set_bits(&mut self.0, __val, $range) }
    }
}

#[inline(always)]
fn get_bits<T: From<u8> + std::ops::BitOrAssign<T> + std::ops::Shl<usize, Output = T>>(
    bits: &[u8],
    range: std::ops::RangeInclusive<usize>,
) -> T {
    let mut result = T::from(0);
    for i in range {
        if (bits[i / 8] >> (i % 8)) & 1 != 0 {
            result |= T::from(1) << i;
        }
    }
    result
}

#[inline(always)]
fn set_bits<
    T: Copy + From<u8> + Eq + std::ops::BitAnd<T, Output = T> + std::ops::Shr<usize, Output = T>,
>(
    bits: &mut [u8],
    value: T,
    range: std::ops::RangeInclusive<usize>,
) {
    let mut j = 0;

    for i in range {
        let bit = 1 << (i % 8);

        if !((value >> j) & T::from(1)).eq(&0.into()) {
            bits[i / 8] = bits[i / 8] | bit;
        } else {
            bits[i / 8] = bits[i / 8] & !bit;
        }
        j = j + 1;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncAttr(pub [u8; 4]);

impl FuncAttr {
    ranges! {
        func_call/set_func_call: std::os::raw::c_uint = 0..=2;
        func_type/set_func_type: std::os::raw::c_uint = 3..=4;
        func_noreturn/set_func_noreturn: std::os::raw::c_uint = 5..=5;
        func_ctor/set_func_ctor: std::os::raw::c_uint = 6..=6;
        func_dtor/set_func_dtor: std::os::raw::c_uint = 7..=7;
        func_args/set_func_args: std::os::raw::c_uint = 8..=15;
        func_alwinl/set_func_alwinl: std::os::raw::c_uint = 16..=16;
        xxxx/set_xxxx: std::os::raw::c_uint = 17..=31;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymAttr(pub [u8; 2]);

impl SymAttr {
    ranges! {
        aligned/set_aligned: std::os::raw::c_ushort = 0..=4;
        packed/set_packed: std::os::raw::c_ushort = 5..=5;
        weak/set_weak: std::os::raw::c_ushort = 6..=6;
        visibility/set_visibility: std::os::raw::c_ushort = 7..=8;
        dllexport/set_dllexport: std::os::raw::c_ushort = 9..=9;
        nodecorate/set_nodecorate: std::os::raw::c_ushort = 10..=10;
        dllimport/set_dllimport: std::os::raw::c_ushort = 11..=11;
        addrtaken/set_addrtaken: std::os::raw::c_ushort = 12..=12;
        xxxx/set_xxxx: std::os::raw::c_ushort = 13..=15;
    }
}
