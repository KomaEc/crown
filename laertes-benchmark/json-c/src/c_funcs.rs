

extern "C" {
    // json_pointer_f.c
    pub fn json_pointer_getf(obj: *mut ::json_object::json_object,
                             res: *mut *mut ::json_object::json_object,
                             path_fmt: *const std::os::raw::c_char,
                             ...) -> std::os::raw::c_int;
    pub fn json_pointer_setf(obj: *mut *mut ::json_object::json_object,
                             value: *mut ::json_object::json_object,
                             path_fmt: *const std::os::raw::c_char,
                             ...) -> std::os::raw::c_int;

    // last_err.c
    pub fn json_util_get_last_err() -> *const std::os::raw::c_char;
    pub fn _json_c_set_last_err(err_fmt: *const std::os::raw::c_char, ...);

    // sprintbuf.c
    pub fn sprintbuf(p: *mut ::printbuf::printbuf,
                     msg: *const std::os::raw::c_char,
                     ...) -> std::os::raw::c_int;
}
