use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn lrintf(_: libc::c_float) -> libc::c_long;
    fn lrint(_: libc::c_double) -> libc::c_long;
    
    
    
    
    
    
    
    
    
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct SRC_STATE_tag {
    pub vt: *mut SRC_STATE_VT,
    pub last_ratio: libc::c_double,
    pub last_position: libc::c_double,
    pub error: SRC_ERROR,
    pub channels: libc::c_int,
    pub mode: SRC_MODE,
    pub callback_func: src_callback_t,
    pub user_callback_data: *mut libc::c_void,
    pub saved_frames: libc::c_long,
    pub saved_data: *const libc::c_float,
    pub private_data: *mut /* owning */ libc::c_void,
}
impl Default for SRC_STATE_tag {fn default() -> Self {Self {
vt: std::ptr::null_mut(),
last_ratio: Default::default(),
last_position: Default::default(),
error: Default::default(),
channels: Default::default(),
mode: Default::default(),
callback_func: Default::default(),
user_callback_data: std::ptr::null_mut(),
saved_frames: Default::default(),
saved_data: std::ptr::null(),
private_data: std::ptr::null_mut(),
}}}
impl SRC_STATE_tag {pub fn take(&mut self) -> Self {core::mem::take(self)}}

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
pub type SRC_STATE_VT = SRC_STATE_VT_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_STATE_VT_tag {
    pub vari_process: Option::<
        unsafe extern "C" fn(*mut SRC_STATE, *mut SRC_DATA) -> SRC_ERROR,
    >,
    pub const_process: Option::<
        unsafe extern "C" fn(*mut SRC_STATE, *mut SRC_DATA) -> SRC_ERROR,
    >,
    pub reset: Option::<unsafe extern "C" fn(*mut SRC_STATE) -> ()>,
    pub copy: Option::<unsafe extern "C" fn(*mut SRC_STATE) -> *mut SRC_STATE>,
    pub close: Option::<unsafe extern "C" fn(*mut SRC_STATE) -> ()>,
}
pub type SRC_STATE = SRC_STATE_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRC_DATA {
    pub data_in: *const libc::c_float,
    pub data_out: *mut libc::c_float,
    pub input_frames: libc::c_long,
    pub output_frames: libc::c_long,
    pub input_frames_used: libc::c_long,
    pub output_frames_gen: libc::c_long,
    pub end_of_input: libc::c_int,
    pub src_ratio: libc::c_double,
}
pub const SRC_LINEAR: C2RustUnnamed = 4;
pub const SRC_ZERO_ORDER_HOLD: C2RustUnnamed = 3;
pub const SRC_SINC_FASTEST: C2RustUnnamed = 2;
pub const SRC_SINC_MEDIUM_QUALITY: C2RustUnnamed = 1;
pub const SRC_SINC_BEST_QUALITY: C2RustUnnamed = 0;
pub const SRC_TRUE: C2RustUnnamed_0 = 1;
pub const SRC_FALSE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn is_bad_src_ratio(mut ratio: libc::c_double) -> libc::c_int {
    return (ratio < 1.0f64 / 256 as libc::c_int as libc::c_double
        || ratio > 1.0f64 * 256 as libc::c_int as libc::c_double) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn src_new(
    mut converter_type: libc::c_int,
    mut channels: libc::c_int,
    mut error: Option<&mut libc::c_int>,
) -> *mut /* owning */ SRC_STATE {
    return psrc_set_converter(converter_type, channels, error.as_deref_mut());
}
#[no_mangle]
pub unsafe extern "C" fn src_clone(
    mut orig: *mut SRC_STATE,
    mut error: Option<&mut libc::c_int>,
) -> *mut SRC_STATE {
    if orig.is_null() {();
        if !error.as_deref().is_none() {
            *error.as_deref_mut().unwrap()= SRC_ERR_BAD_STATE as libc::c_int;
        }else { (); }
        return 0 as *mut SRC_STATE;
    }
    if !error.as_deref().is_none() {
        *error.as_deref_mut().unwrap()= SRC_ERR_NO_ERROR as libc::c_int;
    }else { (); }
    let mut state = (*(*orig).vt).copy.expect("non-null function pointer")(orig);
    if state.is_null() {();
        if !error.as_deref().is_none() {
            *error.as_deref_mut().unwrap()= SRC_ERR_MALLOC_FAILED as libc::c_int;
        }else { (); }
    }
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn src_callback_new(
    mut func: src_callback_t,
    mut converter_type: libc::c_int,
    mut channels: libc::c_int,
    mut error: Option<&mut libc::c_int>,
    mut cb_data: *mut libc::c_void,
) -> *mut /* owning */ SRC_STATE {
    let mut state = 0 as *mut SRC_STATE;
    if func.is_none() {
        if !error.as_deref().is_none() {
            *error.as_deref_mut().unwrap()= SRC_ERR_BAD_CALLBACK as libc::c_int;
        }else { (); }
        return 0 as *mut SRC_STATE;
    }
    if !error.as_deref().is_none() {
        *error.as_deref_mut().unwrap()= 0 as libc::c_int;
    }else { (); }
    state= src_new(converter_type, channels, error.as_deref_mut());
    if state.is_null() {();
        return 0 as *mut SRC_STATE;
    }
    src_reset(state);
    (*state).mode= SRC_MODE_CALLBACK;
    (*state).callback_func= func;
    (*state).user_callback_data= cb_data;
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn src_delete(mut state: *mut /* owning */ SRC_STATE) -> *mut SRC_STATE {
    if !state.is_null() {
        (*(*state).vt).close.expect("non-null function pointer")(state);
    }else { (); }
    return 0 as *mut SRC_STATE;
}
#[no_mangle]
pub unsafe extern "C" fn src_process(
    mut state: *mut SRC_STATE,
    mut data: *mut SRC_DATA,
) -> libc::c_int {
    let mut error: libc::c_int = 0;
    if state.is_null() {();
        return SRC_ERR_BAD_STATE as libc::c_int;
    }
    if (*state).mode as libc::c_uint != SRC_MODE_PROCESS as libc::c_int as libc::c_uint {
        return SRC_ERR_BAD_MODE as libc::c_int;
    }
    if data.is_null() {();
        return SRC_ERR_BAD_DATA as libc::c_int;
    }
    if (*data).data_in.is_null()
        && (*data).input_frames > 0 as libc::c_int as libc::c_long
        || (*data).data_out.is_null()
            && (*data).output_frames > 0 as libc::c_int as libc::c_long
    {
        return SRC_ERR_BAD_DATA_PTR as libc::c_int;
    }
    if is_bad_src_ratio((*data).src_ratio) != 0 {
        return SRC_ERR_BAD_SRC_RATIO as libc::c_int;
    }
    if (*data).input_frames < 0 as libc::c_int as libc::c_long {
        (*data).input_frames= 0 as libc::c_int as libc::c_long;
    }
    if (*data).output_frames < 0 as libc::c_int as libc::c_long {
        (*data).output_frames= 0 as libc::c_int as libc::c_long;
    }
    if (*data).data_in < (*data).data_out as *const libc::c_float {
        if (*data).data_in
            .offset(((*data).input_frames * (*state).channels as libc::c_long) as isize)
            > (*data).data_out as *const libc::c_float
        {
            return SRC_ERR_DATA_OVERLAP as libc::c_int;
        }
    } else if (*data).data_out
        .offset(((*data).output_frames * (*state).channels as libc::c_long) as isize)
        > (*data).data_in as *mut libc::c_float
    {
        return SRC_ERR_DATA_OVERLAP as libc::c_int
    }
    (*data).input_frames_used= 0 as libc::c_int as libc::c_long;
    (*data).output_frames_gen= 0 as libc::c_int as libc::c_long;
    if (*state).last_ratio < 1.0f64 / 256 as libc::c_int as libc::c_double {
        (*state).last_ratio= (*data).src_ratio;
    }
    if fabs((*state).last_ratio - (*data).src_ratio) < 1e-15f64 {
        error= (*(*state).vt).const_process
            .expect("non-null function pointer")(state, data) as libc::c_int;
    } else {
        error= (*(*state).vt).vari_process
            .expect("non-null function pointer")(state, data) as libc::c_int;
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn src_callback_read(
    mut state: Option<&mut SRC_STATE>,
    mut src_ratio: libc::c_double,
    mut frames: libc::c_long,
    mut data: *mut libc::c_float,
) -> libc::c_long {
    let mut src_data = SRC_DATA {
        data_in: 0 as *const libc::c_float,
        data_out: 0 as *mut libc::c_float,
        input_frames: 0,
        output_frames: 0,
        input_frames_used: 0,
        output_frames_gen: 0,
        end_of_input: 0,
        src_ratio: 0.,
    };
    let mut output_frames_gen: libc::c_long = 0;
    let mut error = 0 as libc::c_int;
    if state.as_deref().is_none() {();
        return 0 as libc::c_int as libc::c_long;
    }
    if frames <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as libc::c_long;
    }
    if (*state.as_deref().unwrap()).mode as libc::c_uint != SRC_MODE_CALLBACK as libc::c_int as libc::c_uint
    {
        (*state.as_deref_mut().unwrap()).error= SRC_ERR_BAD_MODE;
        return 0 as libc::c_int as libc::c_long;
    }
    if (*state.as_deref().unwrap()).callback_func.is_none() {
        (*state.as_deref_mut().unwrap()).error= SRC_ERR_NULL_CALLBACK;
        return 0 as libc::c_int as libc::c_long;
    }
    memset(
        core::ptr::addr_of_mut!(src_data) as *mut SRC_DATA as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<SRC_DATA>() as libc::c_ulong,
    );
    if is_bad_src_ratio(src_ratio) != 0 {
        (*state.as_deref_mut().unwrap()).error= SRC_ERR_BAD_SRC_RATIO;
        return 0 as libc::c_int as libc::c_long;
    }
    src_data.src_ratio= src_ratio;
    src_data.data_out= data;
    src_data.output_frames= frames;
    src_data.data_in= (*state.as_deref().unwrap()).saved_data;
    src_data.input_frames= (*state.as_deref().unwrap()).saved_frames;
    output_frames_gen= 0 as libc::c_int as libc::c_long;
    while output_frames_gen < frames {
        let mut dummy: [libc::c_float; 1] = [0.; 1];
        if src_data.input_frames == 0 as libc::c_int as libc::c_long {
            let mut ptr = dummy.as_mut_ptr();
            src_data.input_frames= (*state.as_deref().unwrap()).callback_func
                .expect(
                    "non-null function pointer",
                )((*state.as_deref().unwrap()).user_callback_data, core::ptr::addr_of_mut!(ptr));
            src_data.data_in= ptr;
            if src_data.input_frames == 0 as libc::c_int as libc::c_long {
                src_data.end_of_input= 1 as libc::c_int;
            }
        }
        (*state.as_deref_mut().unwrap()).mode= SRC_MODE_PROCESS;
        error= src_process(state.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of_mut!(src_data));
        (*state.as_deref_mut().unwrap()).mode= SRC_MODE_CALLBACK;
        if error != 0 as libc::c_int {
            break;
        }
        src_data.data_in= src_data.data_in
            .offset(
                (src_data.input_frames_used * (*state.as_deref().unwrap()).channels as libc::c_long) as isize,
            );
        src_data.input_frames-= src_data.input_frames_used;
        src_data.data_out= src_data.data_out
            .offset(
                (src_data.output_frames_gen * (*state.as_deref().unwrap()).channels as libc::c_long) as isize,
            );
        src_data.output_frames-= src_data.output_frames_gen;
        output_frames_gen+= src_data.output_frames_gen;
        if src_data.end_of_input == SRC_TRUE as libc::c_int
            && src_data.output_frames_gen == 0 as libc::c_int as libc::c_long
        {
            break;
        }
    }
    (*state.as_deref_mut().unwrap()).saved_data= src_data.data_in;
    (*state.as_deref_mut().unwrap()).saved_frames= src_data.input_frames;
    if error != 0 as libc::c_int {
        (*state.as_deref_mut().unwrap()).error= error as SRC_ERROR;
        return 0 as libc::c_int as libc::c_long;
    }
    return output_frames_gen;
}
#[no_mangle]
pub unsafe extern "C" fn src_set_ratio(
    mut state: Option<&mut SRC_STATE>,
    mut new_ratio: libc::c_double,
) -> libc::c_int {
    if state.as_deref().is_none() {();
        return SRC_ERR_BAD_STATE as libc::c_int;
    }
    if is_bad_src_ratio(new_ratio) != 0 {
        return SRC_ERR_BAD_SRC_RATIO as libc::c_int;
    }
    (*state.as_deref_mut().unwrap()).last_ratio= new_ratio;
    return SRC_ERR_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn src_get_channels(mut state: *mut SRC_STATE) -> libc::c_int {
    if state.is_null() {();
        return -(SRC_ERR_BAD_STATE as libc::c_int);
    }
    return (*state).channels;
}
#[no_mangle]
pub unsafe extern "C" fn src_reset(mut state: *mut SRC_STATE) -> libc::c_int {
    if state.is_null() {();
        return SRC_ERR_BAD_STATE as libc::c_int;
    }
    (*(*state).vt).reset.expect("non-null function pointer")(state);
    (*state).last_position= 0.0f64;
    (*state).last_ratio= 0.0f64;
    (*state).saved_data= 0 as *const libc::c_float;
    (*state).saved_frames= 0 as libc::c_int as libc::c_long;
    (*state).error= SRC_ERR_NO_ERROR;
    return SRC_ERR_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn src_get_name(
    mut converter_type: libc::c_int,
) -> *const libc::c_char {
    let mut desc = 0 as *const libc::c_char;
    desc= crate::src::src_sinc::sinc_get_name(converter_type);
    if !desc.is_null() {
        return desc;
    }else { (); }
    desc= crate::src::src_zoh::zoh_get_name(converter_type);
    if !desc.is_null() {
        return desc;
    }else { (); }
    desc= crate::src::src_linear::linear_get_name(converter_type);
    if !desc.is_null() {
        return desc;
    }else { (); }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn src_get_description(
    mut converter_type: libc::c_int,
) -> *const libc::c_char {
    let mut desc = 0 as *const libc::c_char;
    desc= crate::src::src_sinc::sinc_get_description(converter_type);
    if !desc.is_null() {
        return desc;
    }else { (); }
    desc= crate::src::src_zoh::zoh_get_description(converter_type);
    if !desc.is_null() {
        return desc;
    }else { (); }
    desc= crate::src::src_linear::linear_get_description(converter_type);
    if !desc.is_null() {
        return desc;
    }else { (); }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn src_get_version() -> *const libc::c_char {
    return b"libsamplerate-0.2.2 (c) 2002-2008 Erik de Castro Lopo\0" as *const u8
        as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn src_is_valid_ratio(mut ratio: libc::c_double) -> libc::c_int {
    if is_bad_src_ratio(ratio) != 0 {
        return SRC_FALSE as libc::c_int;
    }
    return SRC_TRUE as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn src_error(mut state: *mut SRC_STATE) -> libc::c_int {
    if !state.is_null() {
        return (*state).error as libc::c_int;
    }else { (); }
    return SRC_ERR_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn src_strerror(mut error: libc::c_int) -> *const libc::c_char {
    match error {
        0 => return b"No error.\0" as *const u8 as *const libc::c_char,
        1 => return b"Malloc failed.\0" as *const u8 as *const libc::c_char,
        2 => return b"SRC_STATE pointer is NULL.\0" as *const u8 as *const libc::c_char,
        3 => return b"SRC_DATA pointer is NULL.\0" as *const u8 as *const libc::c_char,
        4 => {
            return b"SRC_DATA->data_out or SRC_DATA->data_in is NULL.\0" as *const u8
                as *const libc::c_char;
        }
        5 => {
            return b"Internal error. No private data.\0" as *const u8
                as *const libc::c_char;
        }
        6 => {
            return b"SRC ratio outside [1/256, 256] range.\0" as *const u8
                as *const libc::c_char;
        }
        15 => {
            return b"src_process() called without reset after end_of_input.\0"
                as *const u8 as *const libc::c_char;
        }
        7 => {
            return b"Internal error. No process pointer.\0" as *const u8
                as *const libc::c_char;
        }
        8 => {
            return b"Internal error. SHIFT_BITS too large.\0" as *const u8
                as *const libc::c_char;
        }
        9 => {
            return b"Internal error. Filter length too large.\0" as *const u8
                as *const libc::c_char;
        }
        10 => return b"Bad converter number.\0" as *const u8 as *const libc::c_char,
        11 => return b"Channel count must be >= 1.\0" as *const u8 as *const libc::c_char,
        12 => {
            return b"Internal error. Bad buffer length. Please report this.\0"
                as *const u8 as *const libc::c_char;
        }
        13 => {
            return b"Internal error. Input data / internal buffer size difference. Please report this.\0"
                as *const u8 as *const libc::c_char;
        }
        14 => {
            return b"Internal error. Private pointer is NULL. Please report this.\0"
                as *const u8 as *const libc::c_char;
        }
        16 => {
            return b"Input and output data arrays overlap.\0" as *const u8
                as *const libc::c_char;
        }
        17 => {
            return b"Supplied callback function pointer is NULL.\0" as *const u8
                as *const libc::c_char;
        }
        18 => {
            return b"Calling mode differs from initialisation mode (ie process v callback).\0"
                as *const u8 as *const libc::c_char;
        }
        19 => {
            return b"Callback function pointer is NULL in src_callback_read ().\0"
                as *const u8 as *const libc::c_char;
        }
        20 => {
            return b"This converter only allows constant conversion ratios.\0"
                as *const u8 as *const libc::c_char;
        }
        21 => {
            return b"Internal error : Bad length in prepare_data ().\0" as *const u8
                as *const libc::c_char;
        }
        22 => {
            return b"Error : Someone is trampling on my internal state.\0" as *const u8
                as *const libc::c_char;
        }
        23 => {
            return b"Placeholder. No error defined for this error number.\0" as *const u8
                as *const libc::c_char;
        }
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn src_simple(
    mut src_data: Option<&mut SRC_DATA>,
    mut converter: libc::c_int,
    mut channels: libc::c_int,
) -> libc::c_int {
    let mut src_state = 0 as *mut SRC_STATE;
    let mut error: libc::c_int = 0;
    src_state= src_new(converter, channels, Some(&mut error));
    if src_state.is_null() {();
        return error;
    }
    (*src_data.as_deref_mut().unwrap()).end_of_input= 1 as libc::c_int;
    error= src_process(src_state, src_data.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    src_delete(src_state);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn src_short_to_float_array(
    mut in_0: *const libc::c_short,
    mut out: *mut libc::c_float,
    mut len: libc::c_int,
) {
    let mut i = 0 as libc::c_int;
    while i < len {
        *out
            .offset(
                i as isize,
            ) = (*in_0.offset(i as isize) as libc::c_int as libc::c_double
            / (1.0f64 * 0x8000 as libc::c_int as libc::c_double)) as libc::c_float;
        i+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn src_float_to_short_array(
    mut in_0: *const libc::c_float,
    mut out: *mut libc::c_short,
    mut len: libc::c_int,
) {
    let mut i = 0 as libc::c_int;
    while i < len {
        let mut scaled_value: libc::c_float = 0.;
        scaled_value= *in_0.offset(i as isize) * 32768.0f32;
        if scaled_value >= 32767.0f32 {
            *out.offset(i as isize) = 32767 as libc::c_int as libc::c_short;
        } else if scaled_value <= -32768.0f32 {
            *out.offset(i as isize) = -(32768 as libc::c_int) as libc::c_short;
        } else {
            *out.offset(i as isize) = lrintf(scaled_value) as libc::c_short;
        }
        i+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn src_int_to_float_array(
    mut in_0: *const libc::c_int,
    mut out: *mut libc::c_float,
    mut len: libc::c_int,
) {
    let mut i = 0 as libc::c_int;
    while i < len {
        *out
            .offset(
                i as isize,
            ) = (*in_0.offset(i as isize) as libc::c_double
            / (8.0f64 * 0x10000000 as libc::c_int as libc::c_double)) as libc::c_float;
        i+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn src_float_to_int_array(
    mut in_0: *const libc::c_float,
    mut out: *mut libc::c_int,
    mut len: libc::c_int,
) {
    let mut scaled_value: libc::c_double = 0.;
    let mut i = 0 as libc::c_int;
    while i < len {
        scaled_value= *in_0.offset(i as isize) as libc::c_double
            * (8.0f64 * 0x10000000 as libc::c_int as libc::c_double);
        if scaled_value >= 1.0f64 * 0x7fffffff as libc::c_int as libc::c_double {
            *out.offset(i as isize) = 0x7fffffff as libc::c_int;
        } else if scaled_value <= -8.0f64 * 0x10000000 as libc::c_int as libc::c_double {
            *out.offset(i as isize) = -(1 as libc::c_int) - 0x7fffffff as libc::c_int;
        } else {
            *out.offset(i as isize) = lrint(scaled_value) as libc::c_int;
        }
        i+= 1;
    }
}
unsafe extern "C" fn psrc_set_converter(
    mut converter_type: libc::c_int,
    mut channels: libc::c_int,
    mut error: Option<&mut libc::c_int>,
) -> *mut /* owning */ SRC_STATE {
    let mut temp_error = SRC_ERR_NO_ERROR;
    let mut state = 0 as *mut SRC_STATE;
    match converter_type {
        0 => {
            state= crate::src::src_sinc::sinc_state_new(converter_type, channels, Some(&mut temp_error));
        }
        1 => {
            state= crate::src::src_sinc::sinc_state_new(converter_type, channels, Some(&mut temp_error));
        }
        2 => {
            state= crate::src::src_sinc::sinc_state_new(converter_type, channels, Some(&mut temp_error));
        }
        3 => {
            state= crate::src::src_zoh::zoh_state_new(channels, Some(&mut temp_error));
        }
        4 => {
            state= crate::src::src_linear::linear_state_new(channels, Some(&mut temp_error));
        }
        _ => {
            temp_error= SRC_ERR_BAD_CONVERTER;
            state= 0 as *mut SRC_STATE;
        }
    }
    if !error.as_deref().is_none() {
        *error.as_deref_mut().unwrap()= temp_error as libc::c_int;
    }else { (); }
    return state;
}
