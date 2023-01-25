use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn lrint(_: libc::c_double) -> libc::c_long;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor6 { dummy: () }
pub type src_callback_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_float) -> libc::c_long,
>;
pub type SRC_MODE = libc::c_uint;
pub const SRC_MODE_CALLBACK: SRC_MODE = 1;
pub const SRC_MODE_PROCESS: SRC_MODE = 0;
pub type SRC_ERROR = libc::c_uint;
pub const SRC_ERR_MAX_ERROR: SRC_ERROR = 23;
pub const SRC_ERR_BAD_INTERNAL_STATE: SRC_ERROR = 22;
pub const SRC_ERR_SINC_PREPARE_DATA_BAD_LEN: SRC_ERROR = 21;
pub const SRC_ERR_NO_VARIABLE_RATIO: SRC_ERROR = 20;
pub const SRC_ERR_NULL_CALLBACK: SRC_ERROR = 19;
pub const SRC_ERR_BAD_MODE: SRC_ERROR = 18;
pub const SRC_ERR_BAD_CALLBACK: SRC_ERROR = 17;
pub const SRC_ERR_DATA_OVERLAP: SRC_ERROR = 16;
pub const SRC_ERR_BAD_SINC_STATE: SRC_ERROR = 15;
pub const SRC_ERR_BAD_PRIV_PTR: SRC_ERROR = 14;
pub const SRC_ERR_SIZE_INCOMPATIBILITY: SRC_ERROR = 13;
pub const SRC_ERR_SINC_BAD_BUFFER_LEN: SRC_ERROR = 12;
pub const SRC_ERR_BAD_CHANNEL_COUNT: SRC_ERROR = 11;
pub const SRC_ERR_BAD_CONVERTER: SRC_ERROR = 10;
pub const SRC_ERR_FILTER_LEN: SRC_ERROR = 9;
pub const SRC_ERR_SHIFT_BITS: SRC_ERROR = 8;
pub const SRC_ERR_BAD_PROC_PTR: SRC_ERROR = 7;
pub const SRC_ERR_BAD_SRC_RATIO: SRC_ERROR = 6;
pub const SRC_ERR_NO_PRIVATE: SRC_ERROR = 5;
pub const SRC_ERR_BAD_DATA_PTR: SRC_ERROR = 4;
pub const SRC_ERR_BAD_DATA: SRC_ERROR = 3;
pub const SRC_ERR_BAD_STATE: SRC_ERROR = 2;
pub const SRC_ERR_MALLOC_FAILED: SRC_ERROR = 1;
pub const SRC_ERR_NO_ERROR: SRC_ERROR = 0;
pub type SRC_STATE_VT = crate::src::samplerate::SRC_STATE_VT_tag;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7 { dummy: () }
pub type SRC_STATE = crate::src::samplerate::SRC_STATE_tag;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor8 { dummy: () }
pub type C2RustUnnamed = libc::c_uint;
pub const SRC_LINEAR: C2RustUnnamed = 4;
pub const SRC_ZERO_ORDER_HOLD: C2RustUnnamed = 3;
pub const SRC_SINC_FASTEST: C2RustUnnamed = 2;
pub const SRC_SINC_MEDIUM_QUALITY: C2RustUnnamed = 1;
pub const SRC_SINC_BEST_QUALITY: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer3;
#[repr(C)]
pub struct ZOH_DATA {
    pub zoh_magic_marker: libc::c_int,
    pub dirty: bool,
    pub in_count: libc::c_long,
    pub in_used: libc::c_long,
    pub out_count: libc::c_long,
    pub out_gen: libc::c_long,
    pub last_value: *mut /* owning */ libc::c_float,
}
impl Default for ZOH_DATA {fn default() -> Self {Self {
zoh_magic_marker: Default::default(),
dirty: Default::default(),
in_count: Default::default(),
in_used: Default::default(),
out_count: Default::default(),
out_gen: Default::default(),
last_value: std::ptr::null_mut(),
}}}
impl ZOH_DATA {pub fn take(&mut self) -> Self {core::mem::take(self)}}

#[inline]
unsafe extern "C" fn is_bad_src_ratio(mut ratio: libc::c_double) -> libc::c_int {
    return (ratio < 1.0f64 / 256 as libc::c_int as libc::c_double
        || ratio > 1.0f64 * 256 as libc::c_int as libc::c_double) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fmod_one(mut x: libc::c_double) -> libc::c_double {
    let mut res: libc::c_double = 0.;
    res= x - lrint(x) as libc::c_double;
    if res < 0.0f64 {
        return res + 1.0f64;
    }
    return res;
}
static mut zoh_state_vt: SRC_STATE_VT = unsafe {
    {
        let mut init = crate::src::samplerate::SRC_STATE_VT_tag {
            vari_process: Some(
                zoh_vari_process
                    as unsafe extern "C" fn(*mut SRC_STATE, *mut crate::src::samplerate::SRC_DATA) -> SRC_ERROR,
            ),
            const_process: Some(
                zoh_vari_process
                    as unsafe extern "C" fn(*mut SRC_STATE, *mut crate::src::samplerate::SRC_DATA) -> SRC_ERROR,
            ),
            reset: Some(zoh_reset as unsafe extern "C" fn(*mut SRC_STATE) -> ()),
            copy: Some(
                zoh_copy as unsafe extern "C" fn(*mut SRC_STATE) -> *mut SRC_STATE,
            ),
            close: Some(zoh_close as unsafe extern "C" fn(*mut SRC_STATE) -> ()),
        };
        init
    }
};
unsafe extern "C" fn zoh_vari_process(
    mut state: Option<&mut SRC_STATE>,
    mut data: Option<&mut crate::src::samplerate::SRC_DATA>,
) -> SRC_ERROR {
    let mut priv_0 = 0 as *mut ZOH_DATA;
    let mut src_ratio: libc::c_double = 0.;
    let mut input_index: libc::c_double = 0.;
    let mut rem: libc::c_double = 0.;
    let mut ch: libc::c_int = 0;
    if (*data.as_deref().unwrap()).input_frames <= 0 as libc::c_int as libc::c_long {
        return SRC_ERR_NO_ERROR;
    }
    if (*state.as_deref().unwrap()).private_data.is_null() {();
        return SRC_ERR_NO_PRIVATE;
    }
    priv_0= (*state.as_deref().unwrap()).private_data as *mut ZOH_DATA;
    if !(*priv_0).dirty {
        ch= 0 as libc::c_int;
        while ch < (*state.as_deref().unwrap()).channels {
            *(*priv_0).last_value
                .offset(ch as isize) = *(*data.as_deref().unwrap()).data_in.offset(ch as isize);
            ch+= 1;
        }
        (*priv_0).dirty= 1 as libc::c_int != 0;
    }
    (*priv_0).in_count= (*data.as_deref().unwrap()).input_frames * (*state.as_deref().unwrap()).channels as libc::c_long;
    (*priv_0).out_count= (*data.as_deref().unwrap()).output_frames * (*state.as_deref().unwrap()).channels as libc::c_long;
    (*priv_0).out_gen= 0 as libc::c_int as libc::c_long; (*priv_0).in_used= (*priv_0).out_gen;
    src_ratio= (*state.as_deref().unwrap()).last_ratio;
    if is_bad_src_ratio(src_ratio) != 0 {
        return SRC_ERR_BAD_INTERNAL_STATE;
    }
    input_index= (*state.as_deref().unwrap()).last_position;
    while input_index < 1.0f64 && (*priv_0).out_gen < (*priv_0).out_count {
        if (*priv_0).in_used as libc::c_double
            + (*state.as_deref().unwrap()).channels as libc::c_double * input_index
            >= (*priv_0).in_count as libc::c_double
        {
            break;
        }
        if (*priv_0).out_count > 0 as libc::c_int as libc::c_long
            && fabs((*state.as_deref().unwrap()).last_ratio - (*data.as_deref().unwrap()).src_ratio) > 1e-20f64
        {
            src_ratio= (*state.as_deref().unwrap()).last_ratio
                + (*priv_0).out_gen as libc::c_double
                    * ((*data.as_deref().unwrap()).src_ratio - (*state.as_deref().unwrap()).last_ratio)
                    / (*priv_0).out_count as libc::c_double;
        }
        ch= 0 as libc::c_int;
        while ch < (*state.as_deref().unwrap()).channels {
            *(*data.as_deref().unwrap()).data_out
                .offset(
                    (*priv_0).out_gen as isize,
                ) = *(*priv_0).last_value.offset(ch as isize);
            (*priv_0).out_gen+= 1;
            ch+= 1;
        }
        input_index+= 1.0f64 / src_ratio;
    }
    rem= fmod_one(input_index);
    (*priv_0).in_used+= (*state.as_deref().unwrap()).channels as libc::c_long * lrint(input_index - rem);
    input_index= rem;
    while (*priv_0).out_gen < (*priv_0).out_count
        && (*priv_0).in_used as libc::c_double
            + (*state.as_deref().unwrap()).channels as libc::c_double * input_index
            <= (*priv_0).in_count as libc::c_double
    {
        if (*priv_0).out_count > 0 as libc::c_int as libc::c_long
            && fabs((*state.as_deref().unwrap()).last_ratio - (*data.as_deref().unwrap()).src_ratio) > 1e-20f64
        {
            src_ratio= (*state.as_deref().unwrap()).last_ratio
                + (*priv_0).out_gen as libc::c_double
                    * ((*data.as_deref().unwrap()).src_ratio - (*state.as_deref().unwrap()).last_ratio)
                    / (*priv_0).out_count as libc::c_double;
        }
        ch= 0 as libc::c_int;
        while ch < (*state.as_deref().unwrap()).channels {
            *(*data.as_deref().unwrap()).data_out
                .offset(
                    (*priv_0).out_gen as isize,
                ) = *(*data.as_deref().unwrap()).data_in
                .offset(
                    ((*priv_0).in_used - (*state.as_deref().unwrap()).channels as libc::c_long
                        + ch as libc::c_long) as isize,
                );
            (*priv_0).out_gen+= 1;
            ch+= 1;
        }
        input_index+= 1.0f64 / src_ratio;
        rem= fmod_one(input_index);
        (*priv_0).in_used+= (*state.as_deref().unwrap()).channels as libc::c_long * lrint(input_index - rem);
        input_index= rem;
    }
    if (*priv_0).in_used > (*priv_0).in_count {
        input_index+= (((*priv_0).in_used - (*priv_0).in_count)
                / (*state.as_deref().unwrap()).channels as libc::c_long) as libc::c_double;
        (*priv_0).in_used= (*priv_0).in_count;
    }
    (*state.as_deref_mut().unwrap()).last_position= input_index;
    if (*priv_0).in_used > 0 as libc::c_int as libc::c_long {
        ch= 0 as libc::c_int;
        while ch < (*state.as_deref().unwrap()).channels {
            *(*priv_0).last_value
                .offset(
                    ch as isize,
                ) = *(*data.as_deref().unwrap()).data_in
                .offset(
                    ((*priv_0).in_used - (*state.as_deref().unwrap()).channels as libc::c_long
                        + ch as libc::c_long) as isize,
                );
            ch+= 1;
        }
    }
    (*state.as_deref_mut().unwrap()).last_ratio= src_ratio;
    (*data.as_deref_mut().unwrap()).input_frames_used= (*priv_0).in_used / (*state.as_deref().unwrap()).channels as libc::c_long;
    (*data.as_deref_mut().unwrap()).output_frames_gen= (*priv_0).out_gen / (*state.as_deref().unwrap()).channels as libc::c_long;
    return SRC_ERR_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn zoh_get_name(mut src_enum: libc::c_int) -> *const libc::c_char {
    if src_enum == SRC_ZERO_ORDER_HOLD as libc::c_int {
        return b"ZOH Interpolator\0" as *const u8 as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn zoh_get_description(
    mut src_enum: libc::c_int,
) -> *const libc::c_char {
    if src_enum == SRC_ZERO_ORDER_HOLD as libc::c_int {
        return b"Zero order hold interpolator, very fast, poor quality.\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn zoh_data_new(mut channels: libc::c_int) -> *mut /* owning */ ZOH_DATA {
    let mut priv_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ZOH_DATA>() as libc::c_ulong,
    ) as *mut ZOH_DATA;
    if !priv_0.is_null() {
        (*priv_0).zoh_magic_marker= 's' as i32 + (('r' as i32) << 4 as libc::c_int)
            + (('c' as i32) << 8 as libc::c_int) + (('z' as i32) << 12 as libc::c_int)
            + (('o' as i32) << 16 as libc::c_int) + (('h' as i32) << 20 as libc::c_int);
        (*priv_0).last_value= calloc(
            channels as libc::c_ulong,
            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        ) as *mut libc::c_float;
        if (*priv_0).last_value.is_null() {();
            free(priv_0 as *mut libc::c_void);
            priv_0= 0 as *mut ZOH_DATA;
        }
    }else { (); }
    return priv_0;
}
#[no_mangle]
pub unsafe extern "C" fn zoh_state_new(
    mut channels: libc::c_int,
    mut error: Option<&mut SRC_ERROR>,
) -> *mut /* owning */ SRC_STATE {
    let mut state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<SRC_STATE>() as libc::c_ulong,
    ) as *mut SRC_STATE;
    if state.is_null() {();
        *error.as_deref_mut().unwrap()= SRC_ERR_MALLOC_FAILED;
        return 0 as *mut SRC_STATE;
    }
    (*state).channels= channels;
    (*state).mode= SRC_MODE_PROCESS;
    (*state).private_data= zoh_data_new((*state).channels) as *mut libc::c_void;
    if (*state).private_data.is_null() {();
        free(state as *mut libc::c_void);
        *error.as_deref_mut().unwrap()= SRC_ERR_MALLOC_FAILED;
        return 0 as *mut SRC_STATE;
    }
    (*state).vt= core::ptr::addr_of_mut!(crate::src::src_zoh::zoh_state_vt);
    zoh_reset(state.as_mut());
    *error.as_deref_mut().unwrap()= SRC_ERR_NO_ERROR;
    return state;
}
unsafe extern "C" fn zoh_reset(mut state: Option<&mut SRC_STATE>) {
    let mut priv_0 = 0 as *mut ZOH_DATA;
    priv_0= (*state.as_deref().unwrap()).private_data as *mut ZOH_DATA;
    if priv_0.is_null() {();
        return;
    }
    (*priv_0).dirty= 0 as libc::c_int != 0;
    memset(
        (*priv_0).last_value as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((*state.as_deref().unwrap()).channels as libc::c_ulong),
    );
}
unsafe extern "C" fn zoh_copy(mut state: *mut SRC_STATE) -> *mut /* owning */ SRC_STATE {
    if (*state).private_data.is_null() {();
        return 0 as *mut SRC_STATE;
    }
    let mut to = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<SRC_STATE>() as libc::c_ulong,
    ) as *mut SRC_STATE;
    if to.is_null() {();
        return 0 as *mut SRC_STATE;
    }
    memcpy(
        to as *mut libc::c_void,
        state as *const libc::c_void,
        ::std::mem::size_of::<SRC_STATE>() as libc::c_ulong,
    );
    let mut from_priv = (*state).private_data as *mut ZOH_DATA;
    let mut to_priv = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ZOH_DATA>() as libc::c_ulong,
    ) as *mut ZOH_DATA;
    if to_priv.is_null() {();
        free(to as *mut libc::c_void);
        return 0 as *mut SRC_STATE;
    }
    memcpy(
        to_priv as *mut libc::c_void,
        from_priv as *const libc::c_void,
        ::std::mem::size_of::<ZOH_DATA>() as libc::c_ulong,
    );
    (*to_priv).last_value= malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((*state).channels as libc::c_ulong),
    ) as *mut libc::c_float;
    if (*to_priv).last_value.is_null() {();
        free(to as *mut libc::c_void);
        free(to_priv as *mut libc::c_void);
        return 0 as *mut SRC_STATE;
    }
    memcpy(
        (*to_priv).last_value as *mut libc::c_void,
        (*from_priv).last_value as *const f32 as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((*state).channels as libc::c_ulong),
    );
    (*to).private_data= to_priv as *mut libc::c_void;
    return to;
}
unsafe extern "C" fn zoh_close(mut state: *mut /* owning */ SRC_STATE) {
    if !state.is_null() {
        let mut zoh = (*state).private_data as *mut ZOH_DATA;
        if !zoh.is_null() {
            if !(*zoh).last_value.is_null() {
                free((*zoh).last_value as *mut libc::c_void);
                (*zoh).last_value= 0 as *mut libc::c_float;
            }else { (); }
            free(zoh as *mut libc::c_void);
            zoh= 0 as *mut ZOH_DATA;
        }else { (); }
        free(state as *mut libc::c_void);
        state= 0 as *mut SRC_STATE;
    }else { (); }
}
