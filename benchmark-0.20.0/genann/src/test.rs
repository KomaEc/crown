use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn genann_init(
        inputs: libc::c_int,
        hidden_layers: libc::c_int,
        hidden: libc::c_int,
        outputs: libc::c_int,
    ) -> *mut genann;
    fn genann_read(in_0: *mut FILE) -> *mut genann;
    fn genann_randomize(ann: *mut genann);
    fn genann_copy(ann: *const genann) -> *mut genann;
    fn genann_free(ann: *mut genann);
    fn genann_run(ann: *const genann, inputs: *const libc::c_double) -> *const libc::c_double;
    fn genann_train(
        ann: *const genann,
        inputs: *const libc::c_double,
        desired_outputs: *const libc::c_double,
        learning_rate: libc::c_double,
    );
    fn genann_write(ann: *const genann, out: *mut FILE);
    fn genann_act_sigmoid(ann: *const genann, a: libc::c_double) -> libc::c_double;
    fn genann_act_sigmoid_cached(ann: *const genann, a: libc::c_double) -> libc::c_double;
    fn genann_act_threshold(ann: *const genann, a: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn clock() -> clock_t;
    fn srand(__seed: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
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
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: libc::c_int,
    pub hidden_layers: libc::c_int,
    pub hidden: libc::c_int,
    pub outputs: libc::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: libc::c_int,
    pub total_neurons: libc::c_int,
    pub weight: *mut libc::c_double,
    pub output: *mut libc::c_double,
    pub delta: *mut libc::c_double,
}
pub type genann_actfun =
    Option<unsafe extern "C" fn(*const genann, libc::c_double) -> libc::c_double>;
pub type clock_t = __clock_t;
static mut ltests: libc::c_int = 0 as libc::c_int;
static mut lfails: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn basic() {
    let mut ann: *mut genann = genann_init(
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    ltests += 1;
    ltests;
    if (*ann).total_weights != 2 as libc::c_int {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            (*ann).total_weights,
            2 as libc::c_int,
        );
    }
    let mut a: libc::c_double = 0.;
    a = 0 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    ltests += 1;
    ltests;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            0.5f64,
            *genann_run(ann, &mut a),
        );
    }
    a = 1 as libc::c_int as libc::c_double;
    ltests += 1;
    ltests;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            0.5f64,
            *genann_run(ann, &mut a),
        );
    }
    a = 11 as libc::c_int as libc::c_double;
    ltests += 1;
    ltests;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            0.5f64,
            *genann_run(ann, &mut a),
        );
    }
    a = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    ltests += 1;
    ltests;
    if fabs(0.5f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            0.5f64,
            *genann_run(ann, &mut a),
        );
    }
    a = 10 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    ltests += 1;
    ltests;
    if fabs(1.0f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            1.0f64,
            *genann_run(ann, &mut a),
        );
    }
    a = -(10 as libc::c_int) as libc::c_double;
    ltests += 1;
    ltests;
    if fabs(0.0f64 - *genann_run(ann, &mut a)) > 0.001f64 {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            0.0f64,
            *genann_run(ann, &mut a),
        );
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn xor() {
    let mut ann: *mut genann = genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    (*ann).activation_hidden = Some(
        genann_act_threshold
            as unsafe extern "C" fn(*const genann, libc::c_double) -> libc::c_double,
    );
    (*ann).activation_output = Some(
        genann_act_threshold
            as unsafe extern "C" fn(*const genann, libc::c_double) -> libc::c_double,
    );
    ltests += 1;
    ltests;
    if (*ann).total_weights != 9 as libc::c_int {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            (*ann).total_weights,
            9 as libc::c_int,
        );
    }
    *((*ann).weight).offset(0 as libc::c_int as isize) = 0.5f64;
    *((*ann).weight).offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(5 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(6 as libc::c_int as isize) = 0.5f64;
    *((*ann).weight).offset(7 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *((*ann).weight).offset(8 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
    let mut input: [[libc::c_double; 2]; 4] = [
        [
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    ltests += 1;
    ltests;
    if fabs(
        output[0 as libc::c_int as usize]
            - *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            output[0 as libc::c_int as usize],
            *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[1 as libc::c_int as usize]
            - *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            output[1 as libc::c_int as usize],
            *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[2 as libc::c_int as usize]
            - *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            output[2 as libc::c_int as usize],
            *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[3 as libc::c_int as usize]
            - *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            output[3 as libc::c_int as usize],
            *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn backprop() {
    let mut ann: *mut genann = genann_init(
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut input: libc::c_double = 0.;
    let mut output: libc::c_double = 0.;
    input = 0.5f64;
    output = 1 as libc::c_int as libc::c_double;
    let mut first_try: libc::c_double = *genann_run(ann, &mut input);
    genann_train(ann, &mut input, &mut output, 0.5f64);
    let mut second_try: libc::c_double = *genann_run(ann, &mut input);
    ltests += 1;
    ltests;
    if !(fabs(first_try - output) > fabs(second_try - output)) {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
        );
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_and() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    let mut ann: *mut genann = genann_init(
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            genann_train(
                ann,
                (input[j as usize]).as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                0.8f64,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*ann).activation_output = Some(
        genann_act_threshold
            as unsafe extern "C" fn(*const genann, libc::c_double) -> libc::c_double,
    );
    ltests += 1;
    ltests;
    if fabs(
        output[0 as libc::c_int as usize]
            - *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            output[0 as libc::c_int as usize],
            *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[1 as libc::c_int as usize]
            - *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            output[1 as libc::c_int as usize],
            *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[2 as libc::c_int as usize]
            - *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            output[2 as libc::c_int as usize],
            *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[3 as libc::c_int as usize]
            - *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            output[3 as libc::c_int as usize],
            *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_or() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    let mut ann: *mut genann = genann_init(
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    genann_randomize(ann);
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            genann_train(
                ann,
                (input[j as usize]).as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                0.8f64,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*ann).activation_output = Some(
        genann_act_threshold
            as unsafe extern "C" fn(*const genann, libc::c_double) -> libc::c_double,
    );
    ltests += 1;
    ltests;
    if fabs(
        output[0 as libc::c_int as usize]
            - *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            output[0 as libc::c_int as usize],
            *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[1 as libc::c_int as usize]
            - *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            output[1 as libc::c_int as usize],
            *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[2 as libc::c_int as usize]
            - *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            output[2 as libc::c_int as usize],
            *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[3 as libc::c_int as usize]
            - *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            output[3 as libc::c_int as usize],
            *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_xor() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut ann: *mut genann = genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            genann_train(
                ann,
                (input[j as usize]).as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                3 as libc::c_int as libc::c_double,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*ann).activation_output = Some(
        genann_act_threshold
            as unsafe extern "C" fn(*const genann, libc::c_double) -> libc::c_double,
    );
    ltests += 1;
    ltests;
    if fabs(
        output[0 as libc::c_int as usize]
            - *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            output[0 as libc::c_int as usize],
            *genann_run(ann, (input[0 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[1 as libc::c_int as usize]
            - *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            output[1 as libc::c_int as usize],
            *genann_run(ann, (input[1 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[2 as libc::c_int as usize]
            - *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            output[2 as libc::c_int as usize],
            *genann_run(ann, (input[2 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    ltests += 1;
    ltests;
    if fabs(
        output[3 as libc::c_int as usize]
            - *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
    ) > 0.001f64
    {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            output[3 as libc::c_int as usize],
            *genann_run(ann, (input[3 as libc::c_int as usize]).as_mut_ptr()),
        );
    }
    genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn persist() {
    let mut first: *mut genann = genann_init(
        1000 as libc::c_int,
        5 as libc::c_int,
        50 as libc::c_int,
        10 as libc::c_int,
    );
    let mut out: *mut FILE = fopen(
        b"persist.txt\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    genann_write(first, out);
    fclose(out);
    let mut in_0: *mut FILE = fopen(
        b"persist.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut second: *mut genann = genann_read(in_0);
    fclose(in_0);
    ltests += 1;
    ltests;
    if (*first).inputs != (*second).inputs {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            (*first).inputs,
            (*second).inputs,
        );
    }
    ltests += 1;
    ltests;
    if (*first).hidden_layers != (*second).hidden_layers {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            (*first).hidden_layers,
            (*second).hidden_layers,
        );
    }
    ltests += 1;
    ltests;
    if (*first).hidden != (*second).hidden {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            (*first).hidden,
            (*second).hidden,
        );
    }
    ltests += 1;
    ltests;
    if (*first).outputs != (*second).outputs {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            (*first).outputs,
            (*second).outputs,
        );
    }
    ltests += 1;
    ltests;
    if (*first).total_weights != (*second).total_weights {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            (*first).total_weights,
            (*second).total_weights,
        );
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*first).total_weights {
        ltests += 1;
        ltests;
        if !(*((*first).weight).offset(i as isize) == *((*second).weight).offset(i as isize)) {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int,
            );
        }
        i += 1;
        i;
    }
    genann_free(first);
    genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn copy() {
    let mut first: *mut genann = genann_init(
        1000 as libc::c_int,
        5 as libc::c_int,
        50 as libc::c_int,
        10 as libc::c_int,
    );
    let mut second: *mut genann = genann_copy(first);
    ltests += 1;
    ltests;
    if (*first).inputs != (*second).inputs {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int,
            (*first).inputs,
            (*second).inputs,
        );
    }
    ltests += 1;
    ltests;
    if (*first).hidden_layers != (*second).hidden_layers {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            (*first).hidden_layers,
            (*second).hidden_layers,
        );
    }
    ltests += 1;
    ltests;
    if (*first).hidden != (*second).hidden {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            (*first).hidden,
            (*second).hidden,
        );
    }
    ltests += 1;
    ltests;
    if (*first).outputs != (*second).outputs {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            (*first).outputs,
            (*second).outputs,
        );
    }
    ltests += 1;
    ltests;
    if (*first).total_weights != (*second).total_weights {
        lfails += 1;
        lfails;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            (*first).total_weights,
            (*second).total_weights,
        );
    }
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*first).total_weights {
        ltests += 1;
        ltests;
        if fabs(*((*first).weight).offset(i as isize) - *((*second).weight).offset(i as isize))
            > 0.001f64
        {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                237 as libc::c_int,
                *((*first).weight).offset(i as isize),
                *((*second).weight).offset(i as isize),
            );
        }
        i += 1;
        i;
    }
    genann_free(first);
    genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn sigmoid() {
    let mut i: libc::c_double = -(20 as libc::c_int) as libc::c_double;
    let max: libc::c_double = 20 as libc::c_int as libc::c_double;
    let d: libc::c_double = 0.0001f64;
    while i < max {
        ltests += 1;
        ltests;
        if fabs(
            genann_act_sigmoid(0 as *const genann, i)
                - genann_act_sigmoid_cached(0 as *const genann, i),
        ) > 0.001f64
        {
            lfails += 1;
            lfails;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int,
                genann_act_sigmoid(0 as *const genann, i),
                genann_act_sigmoid_cached(0 as *const genann, i),
            );
        }
        i += d;
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    printf(b"GENANN TEST SUITE\n\0" as *const u8 as *const libc::c_char);
    srand(100 as libc::c_int as libc::c_uint);
    let ts: libc::c_int = ltests;
    let fs: libc::c_int = lfails;
    let start: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"basic\0" as *const u8 as *const libc::c_char,
    );
    basic();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts - (lfails - fs),
        lfails - fs,
        ((clock() - start) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_0: libc::c_int = ltests;
    let fs_0: libc::c_int = lfails;
    let start_0: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"xor\0" as *const u8 as *const libc::c_char,
    );
    xor();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_0 - (lfails - fs_0),
        lfails - fs_0,
        ((clock() - start_0) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_1: libc::c_int = ltests;
    let fs_1: libc::c_int = lfails;
    let start_1: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"backprop\0" as *const u8 as *const libc::c_char,
    );
    backprop();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_1 - (lfails - fs_1),
        lfails - fs_1,
        ((clock() - start_1) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_2: libc::c_int = ltests;
    let fs_2: libc::c_int = lfails;
    let start_2: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train and\0" as *const u8 as *const libc::c_char,
    );
    train_and();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_2 - (lfails - fs_2),
        lfails - fs_2,
        ((clock() - start_2) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_3: libc::c_int = ltests;
    let fs_3: libc::c_int = lfails;
    let start_3: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train or\0" as *const u8 as *const libc::c_char,
    );
    train_or();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_3 - (lfails - fs_3),
        lfails - fs_3,
        ((clock() - start_3) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_4: libc::c_int = ltests;
    let fs_4: libc::c_int = lfails;
    let start_4: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train xor\0" as *const u8 as *const libc::c_char,
    );
    train_xor();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_4 - (lfails - fs_4),
        lfails - fs_4,
        ((clock() - start_4) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_5: libc::c_int = ltests;
    let fs_5: libc::c_int = lfails;
    let start_5: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"persist\0" as *const u8 as *const libc::c_char,
    );
    persist();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_5 - (lfails - fs_5),
        lfails - fs_5,
        ((clock() - start_5) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_6: libc::c_int = ltests;
    let fs_6: libc::c_int = lfails;
    let start_6: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"copy\0" as *const u8 as *const libc::c_char,
    );
    copy();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_6 - (lfails - fs_6),
        lfails - fs_6,
        ((clock() - start_6) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_7: libc::c_int = ltests;
    let fs_7: libc::c_int = lfails;
    let start_7: clock_t = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"sigmoid\0" as *const u8 as *const libc::c_char,
    );
    sigmoid();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        ltests - ts_7 - (lfails - fs_7),
        lfails - fs_7,
        ((clock() - start_7) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    if lfails == 0 as libc::c_int {
        printf(
            b"ALL TESTS PASSED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            ltests,
            ltests,
        );
    } else {
        printf(
            b"SOME TESTS FAILED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            ltests - lfails,
            ltests,
        );
    }
    return (lfails != 0 as libc::c_int) as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw()
//         );
//     }
//     args.push(::core::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(main_0(
//             (args.len() - 1) as libc::c_int,
//             args.as_mut_ptr() as *mut *mut libc::c_char
//         ) as i32)
//     }
// }
