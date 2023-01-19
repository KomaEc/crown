use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _XGC;
    pub type _XDisplay;
    pub type _XPrivate;
    pub type _XrmHashBucketRec;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn XGetImage(
        _: *mut Display,
        _: Drawable,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_uint,
        _: libc::c_ulong,
        _: libc::c_int,
    ) -> *mut XImage;
    fn XOpenDisplay(_: *const libc::c_char) -> *mut Display;
    fn XCreateFontCursor(_: *mut Display, _: libc::c_uint) -> Cursor;
    fn XRootWindow(_: *mut Display, _: libc::c_int) -> Window;
    fn XSetErrorHandler(_: XErrorHandler) -> XErrorHandler;
    fn XAllowEvents(_: *mut Display, _: libc::c_int, _: Time) -> libc::c_int;
    fn XDefaultScreen(_: *mut Display) -> libc::c_int;
    fn XFreeCursor(_: *mut Display, _: Cursor) -> libc::c_int;
    fn XGrabPointer(
        _: *mut Display,
        _: Window,
        _: libc::c_int,
        _: libc::c_uint,
        _: libc::c_int,
        _: libc::c_int,
        _: Window,
        _: Cursor,
        _: Time,
    ) -> libc::c_int;
    fn XQueryColor(_: *mut Display, _: Colormap, _: *mut XColor) -> libc::c_int;
    fn XTranslateCoordinates(
        _: *mut Display,
        _: Window,
        _: Window,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut Window,
    ) -> libc::c_int;
    fn XUngrabPointer(_: *mut Display, _: Time) -> libc::c_int;
    fn XWindowEvent(_: *mut Display, _: Window, _: libc::c_long, _: *mut XEvent) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *const libc::c_char,
    pub _IO_read_end: *const libc::c_char,
    pub _IO_read_base: *const libc::c_char,
    pub _IO_write_base: *const libc::c_char,
    pub _IO_write_ptr: *const libc::c_char,
    pub _IO_write_end: *const libc::c_char,
    pub _IO_buf_base: *const libc::c_char,
    pub _IO_buf_end: *const libc::c_char,
    pub _IO_save_base: *const libc::c_char,
    pub _IO_backup_base: *const libc::c_char,
    pub _IO_save_end: *const libc::c_char,
    pub _markers: *const _IO_marker,
    pub _chain: *const _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *const libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *const _IO_codecvt,
    pub _wide_data: *const _IO_wide_data,
    pub _freeres_list: *const _IO_FILE,
    pub _freeres_buf: *const libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
impl Default for _IO_FILE {
    fn default() -> Self {
        Self {
            _flags: Default::default(),
            _IO_read_ptr: std::ptr::null_mut(),
            _IO_read_end: std::ptr::null_mut(),
            _IO_read_base: std::ptr::null_mut(),
            _IO_write_base: std::ptr::null_mut(),
            _IO_write_ptr: std::ptr::null_mut(),
            _IO_write_end: std::ptr::null_mut(),
            _IO_buf_base: std::ptr::null_mut(),
            _IO_buf_end: std::ptr::null_mut(),
            _IO_save_base: std::ptr::null_mut(),
            _IO_backup_base: std::ptr::null_mut(),
            _IO_save_end: std::ptr::null_mut(),
            _markers: std::ptr::null_mut(),
            _chain: std::ptr::null_mut(),
            _fileno: Default::default(),
            _flags2: Default::default(),
            _old_offset: Default::default(),
            _cur_column: Default::default(),
            _vtable_offset: Default::default(),
            _shortbuf: Default::default(),
            _lock: std::ptr::null_mut(),
            _offset: Default::default(),
            _codecvt: std::ptr::null_mut(),
            _wide_data: std::ptr::null_mut(),
            _freeres_list: std::ptr::null_mut(),
            _freeres_buf: std::ptr::null_mut(),
            __pad5: Default::default(),
            _mode: Default::default(),
            _unused2: Default::default(),
        }
    }
}

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type XID = libc::c_ulong;
pub type Atom = libc::c_ulong;
pub type VisualID = libc::c_ulong;
pub type Time = libc::c_ulong;
pub type Window = XID;
pub type Drawable = XID;
pub type Cursor = XID;
pub type Colormap = XID;
pub type XPointer = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XExtData {
    pub number: libc::c_int,
    pub next: *const _XExtData,
    pub free_private: Option<unsafe extern "C" fn(*mut _XExtData) -> libc::c_int>,
    pub private_data: XPointer,
}
impl Default for _XExtData {
    fn default() -> Self {
        Self {
            number: Default::default(),
            next: std::ptr::null_mut(),
            free_private: Default::default(),
            private_data: std::ptr::null_mut(),
        }
    }
}

pub type XExtData = _XExtData;
pub type GC = *mut _XGC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Visual {
    pub ext_data: *const XExtData,
    pub visualid: VisualID,
    pub class: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub bits_per_rgb: libc::c_int,
    pub map_entries: libc::c_int,
}
impl Default for Visual {
    fn default() -> Self {
        Self {
            ext_data: std::ptr::null_mut(),
            visualid: Default::default(),
            class: Default::default(),
            red_mask: Default::default(),
            green_mask: Default::default(),
            blue_mask: Default::default(),
            bits_per_rgb: Default::default(),
            map_entries: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Depth {
    pub depth: libc::c_int,
    pub nvisuals: libc::c_int,
    pub visuals: *const Visual,
}
impl Default for Depth {
    fn default() -> Self {
        Self {
            depth: Default::default(),
            nvisuals: Default::default(),
            visuals: std::ptr::null_mut(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Screen {
    pub ext_data: *const XExtData,
    pub display: *const _XDisplay,
    pub root: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub mwidth: libc::c_int,
    pub mheight: libc::c_int,
    pub ndepths: libc::c_int,
    pub depths: *const Depth,
    pub root_depth: libc::c_int,
    pub root_visual: *const Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: libc::c_ulong,
    pub black_pixel: libc::c_ulong,
    pub max_maps: libc::c_int,
    pub min_maps: libc::c_int,
    pub backing_store: libc::c_int,
    pub save_unders: libc::c_int,
    pub root_input_mask: libc::c_long,
}
impl Default for Screen {
    fn default() -> Self {
        Self {
            ext_data: std::ptr::null_mut(),
            display: std::ptr::null_mut(),
            root: Default::default(),
            width: Default::default(),
            height: Default::default(),
            mwidth: Default::default(),
            mheight: Default::default(),
            ndepths: Default::default(),
            depths: std::ptr::null_mut(),
            root_depth: Default::default(),
            root_visual: std::ptr::null_mut(),
            default_gc: std::ptr::null_mut(),
            cmap: Default::default(),
            white_pixel: Default::default(),
            black_pixel: Default::default(),
            max_maps: Default::default(),
            min_maps: Default::default(),
            backing_store: Default::default(),
            save_unders: Default::default(),
            root_input_mask: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScreenFormat {
    pub ext_data: *const XExtData,
    pub depth: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub scanline_pad: libc::c_int,
}
impl Default for ScreenFormat {
    fn default() -> Self {
        Self {
            ext_data: std::ptr::null_mut(),
            depth: Default::default(),
            bits_per_pixel: Default::default(),
            scanline_pad: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XImage {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub xoffset: libc::c_int,
    pub format: libc::c_int,
    pub data: *const libc::c_char,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub depth: libc::c_int,
    pub bytes_per_line: libc::c_int,
    pub bits_per_pixel: libc::c_int,
    pub red_mask: libc::c_ulong,
    pub green_mask: libc::c_ulong,
    pub blue_mask: libc::c_ulong,
    pub obdata: XPointer,
    pub f: funcs,
}
impl Default for _XImage {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            xoffset: Default::default(),
            format: Default::default(),
            data: std::ptr::null_mut(),
            byte_order: Default::default(),
            bitmap_unit: Default::default(),
            bitmap_bit_order: Default::default(),
            bitmap_pad: Default::default(),
            depth: Default::default(),
            bytes_per_line: Default::default(),
            bits_per_pixel: Default::default(),
            red_mask: Default::default(),
            green_mask: Default::default(),
            blue_mask: Default::default(),
            obdata: std::ptr::null_mut(),
            f: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct funcs {
    pub create_image: Option<
        unsafe extern "C" fn(
            *mut _XDisplay,
            *mut Visual,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
            libc::c_uint,
            libc::c_uint,
            libc::c_int,
            libc::c_int,
        ) -> *mut _XImage,
    >,
    pub destroy_image: Option<unsafe extern "C" fn(*mut _XImage) -> libc::c_int>,
    pub get_pixel:
        Option<unsafe extern "C" fn(*mut _XImage, libc::c_int, libc::c_int) -> libc::c_ulong>,
    pub put_pixel: Option<
        unsafe extern "C" fn(*mut _XImage, libc::c_int, libc::c_int, libc::c_ulong) -> libc::c_int,
    >,
    pub sub_image: Option<
        unsafe extern "C" fn(
            *mut _XImage,
            libc::c_int,
            libc::c_int,
            libc::c_uint,
            libc::c_uint,
        ) -> *mut _XImage,
    >,
    pub add_pixel: Option<unsafe extern "C" fn(*mut _XImage, libc::c_long) -> libc::c_int>,
}
impl Default for funcs {
    fn default() -> Self {
        Self {
            create_image: Default::default(),
            destroy_image: Default::default(),
            get_pixel: Default::default(),
            put_pixel: Default::default(),
            sub_image: Default::default(),
            add_pixel: Default::default(),
        }
    }
}

pub type XImage = _XImage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColor {
    pub pixel: libc::c_ulong,
    pub red: libc::c_ushort,
    pub green: libc::c_ushort,
    pub blue: libc::c_ushort,
    pub flags: libc::c_char,
    pub pad: libc::c_char,
}
impl Default for XColor {
    fn default() -> Self {
        Self {
            pixel: Default::default(),
            red: Default::default(),
            green: Default::default(),
            blue: Default::default(),
            flags: Default::default(),
            pad: Default::default(),
        }
    }
}

pub type Display = _XDisplay;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ext_data: *const XExtData,
    pub private1: *const _XPrivate,
    pub fd: libc::c_int,
    pub private2: libc::c_int,
    pub proto_major_version: libc::c_int,
    pub proto_minor_version: libc::c_int,
    pub vendor: *const libc::c_char,
    pub private3: XID,
    pub private4: XID,
    pub private5: XID,
    pub private6: libc::c_int,
    pub resource_alloc: Option<unsafe extern "C" fn(*mut _XDisplay) -> XID>,
    pub byte_order: libc::c_int,
    pub bitmap_unit: libc::c_int,
    pub bitmap_pad: libc::c_int,
    pub bitmap_bit_order: libc::c_int,
    pub nformats: libc::c_int,
    pub pixmap_format: *const ScreenFormat,
    pub private8: libc::c_int,
    pub release: libc::c_int,
    pub private9: *const _XPrivate,
    pub private10: *const _XPrivate,
    pub qlen: libc::c_int,
    pub last_request_read: libc::c_ulong,
    pub request: libc::c_ulong,
    pub private11: XPointer,
    pub private12: XPointer,
    pub private13: XPointer,
    pub private14: XPointer,
    pub max_request_size: libc::c_uint,
    pub db: *const _XrmHashBucketRec,
    pub private15: Option<unsafe extern "C" fn(*mut _XDisplay) -> libc::c_int>,
    pub display_name: *const libc::c_char,
    pub default_screen: libc::c_int,
    pub nscreens: libc::c_int,
    pub screens: *const Screen,
    pub motion_buffer: libc::c_ulong,
    pub private16: libc::c_ulong,
    pub min_keycode: libc::c_int,
    pub max_keycode: libc::c_int,
    pub private17: XPointer,
    pub private18: XPointer,
    pub private19: libc::c_int,
    pub xdefaults: *const libc::c_char,
}
impl Default for C2RustUnnamed {
    fn default() -> Self {
        Self {
            ext_data: std::ptr::null_mut(),
            private1: std::ptr::null_mut(),
            fd: Default::default(),
            private2: Default::default(),
            proto_major_version: Default::default(),
            proto_minor_version: Default::default(),
            vendor: std::ptr::null_mut(),
            private3: Default::default(),
            private4: Default::default(),
            private5: Default::default(),
            private6: Default::default(),
            resource_alloc: Default::default(),
            byte_order: Default::default(),
            bitmap_unit: Default::default(),
            bitmap_pad: Default::default(),
            bitmap_bit_order: Default::default(),
            nformats: Default::default(),
            pixmap_format: std::ptr::null_mut(),
            private8: Default::default(),
            release: Default::default(),
            private9: std::ptr::null_mut(),
            private10: std::ptr::null_mut(),
            qlen: Default::default(),
            last_request_read: Default::default(),
            request: Default::default(),
            private11: std::ptr::null_mut(),
            private12: std::ptr::null_mut(),
            private13: std::ptr::null_mut(),
            private14: std::ptr::null_mut(),
            max_request_size: Default::default(),
            db: std::ptr::null_mut(),
            private15: Default::default(),
            display_name: std::ptr::null_mut(),
            default_screen: Default::default(),
            nscreens: Default::default(),
            screens: std::ptr::null_mut(),
            motion_buffer: Default::default(),
            private16: Default::default(),
            min_keycode: Default::default(),
            max_keycode: Default::default(),
            private17: std::ptr::null_mut(),
            private18: std::ptr::null_mut(),
            private19: Default::default(),
            xdefaults: std::ptr::null_mut(),
        }
    }
}

pub type _XPrivDisplay = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub keycode: libc::c_uint,
    pub same_screen: libc::c_int,
}
impl Default for XKeyEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            root: Default::default(),
            subwindow: Default::default(),
            time: Default::default(),
            x: Default::default(),
            y: Default::default(),
            x_root: Default::default(),
            y_root: Default::default(),
            state: Default::default(),
            keycode: Default::default(),
            same_screen: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub button: libc::c_uint,
    pub same_screen: libc::c_int,
}
impl Default for XButtonEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            root: Default::default(),
            subwindow: Default::default(),
            time: Default::default(),
            x: Default::default(),
            y: Default::default(),
            x_root: Default::default(),
            y_root: Default::default(),
            state: Default::default(),
            button: Default::default(),
            same_screen: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub state: libc::c_uint,
    pub is_hint: libc::c_char,
    pub same_screen: libc::c_int,
}
impl Default for XMotionEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            root: Default::default(),
            subwindow: Default::default(),
            time: Default::default(),
            x: Default::default(),
            y: Default::default(),
            x_root: Default::default(),
            y_root: Default::default(),
            state: Default::default(),
            is_hint: Default::default(),
            same_screen: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub x_root: libc::c_int,
    pub y_root: libc::c_int,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
    pub same_screen: libc::c_int,
    pub focus: libc::c_int,
    pub state: libc::c_uint,
}
impl Default for XCrossingEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            root: Default::default(),
            subwindow: Default::default(),
            time: Default::default(),
            x: Default::default(),
            y: Default::default(),
            x_root: Default::default(),
            y_root: Default::default(),
            mode: Default::default(),
            detail: Default::default(),
            same_screen: Default::default(),
            focus: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub mode: libc::c_int,
    pub detail: libc::c_int,
}
impl Default for XFocusChangeEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            mode: Default::default(),
            detail: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub key_vector: [libc::c_char; 32],
}
impl Default for XKeymapEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            key_vector: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
}
impl Default for XExposeEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            count: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub drawable: Drawable,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub count: libc::c_int,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
impl Default for XGraphicsExposeEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            drawable: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            count: Default::default(),
            major_code: Default::default(),
            minor_code: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub drawable: Drawable,
    pub major_code: libc::c_int,
    pub minor_code: libc::c_int,
}
impl Default for XNoExposeEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            drawable: Default::default(),
            major_code: Default::default(),
            minor_code: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub state: libc::c_int,
}
impl Default for XVisibilityEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub override_redirect: libc::c_int,
}
impl Default for XCreateWindowEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            parent: Default::default(),
            window: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            border_width: Default::default(),
            override_redirect: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
}
impl Default for XDestroyWindowEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            event: Default::default(),
            window: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: libc::c_int,
}
impl Default for XUnmapEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            event: Default::default(),
            window: Default::default(),
            from_configure: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: libc::c_int,
}
impl Default for XMapEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            event: Default::default(),
            window: Default::default(),
            override_redirect: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
}
impl Default for XMapRequestEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            parent: Default::default(),
            window: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub override_redirect: libc::c_int,
}
impl Default for XReparentEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            event: Default::default(),
            window: Default::default(),
            parent: Default::default(),
            x: Default::default(),
            y: Default::default(),
            override_redirect: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub override_redirect: libc::c_int,
}
impl Default for XConfigureEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            event: Default::default(),
            window: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            border_width: Default::default(),
            above: Default::default(),
            override_redirect: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
impl Default for XGravityEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            event: Default::default(),
            window: Default::default(),
            x: Default::default(),
            y: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
impl Default for XResizeRequestEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub border_width: libc::c_int,
    pub above: Window,
    pub detail: libc::c_int,
    pub value_mask: libc::c_ulong,
}
impl Default for XConfigureRequestEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            parent: Default::default(),
            window: Default::default(),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            border_width: Default::default(),
            above: Default::default(),
            detail: Default::default(),
            value_mask: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub event: Window,
    pub window: Window,
    pub place: libc::c_int,
}
impl Default for XCirculateEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            event: Default::default(),
            window: Default::default(),
            place: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub parent: Window,
    pub window: Window,
    pub place: libc::c_int,
}
impl Default for XCirculateRequestEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            parent: Default::default(),
            window: Default::default(),
            place: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: libc::c_int,
}
impl Default for XPropertyEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            atom: Default::default(),
            time: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}
impl Default for XSelectionClearEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            selection: Default::default(),
            time: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
impl Default for XSelectionRequestEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            owner: Default::default(),
            requestor: Default::default(),
            selection: Default::default(),
            target: Default::default(),
            property: Default::default(),
            time: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}
impl Default for XSelectionEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            requestor: Default::default(),
            selection: Default::default(),
            target: Default::default(),
            property: Default::default(),
            time: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: libc::c_int,
    pub state: libc::c_int,
}
impl Default for XColormapEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            colormap: Default::default(),
            new: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: libc::c_int,
    pub data: C2RustUnnamed_0,
}
impl Default for XClientMessageEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            message_type: Default::default(),
            format: Default::default(),
            data: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [libc::c_char; 20],
    pub s: [libc::c_short; 10],
    pub l: [libc::c_long; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
    pub request: libc::c_int,
    pub first_keycode: libc::c_int,
    pub count: libc::c_int,
}
impl Default for XMappingEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
            request: Default::default(),
            first_keycode: Default::default(),
            count: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_0: libc::c_int,
    pub display: *const Display,
    pub resourceid: XID,
    pub serial: libc::c_ulong,
    pub error_code: libc::c_uchar,
    pub request_code: libc::c_uchar,
    pub minor_code: libc::c_uchar,
}
impl Default for XErrorEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            display: std::ptr::null_mut(),
            resourceid: Default::default(),
            serial: Default::default(),
            error_code: Default::default(),
            request_code: Default::default(),
            minor_code: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub window: Window,
}
impl Default for XAnyEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            window: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
}
impl Default for XGenericEvent {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            extension: Default::default(),
            evtype: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *const Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub cookie: libc::c_uint,
    pub data: *const libc::c_void,
}
impl Default for XGenericEventCookie {
    fn default() -> Self {
        Self {
            type_0: Default::default(),
            serial: Default::default(),
            send_event: Default::default(),
            display: std::ptr::null_mut(),
            extension: Default::default(),
            evtype: Default::default(),
            cookie: Default::default(),
            data: std::ptr::null_mut(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union _XEvent {
    pub type_0: libc::c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [libc::c_long; 24],
}
pub type XEvent = _XEvent;
pub type XErrorHandler =
    Option<unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int>;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *const *const libc::c_char) -> libc::c_int {
    let mut display = 0 as *mut Display;
    let mut status: libc::c_int = 0;
    let mut color = XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    let mut cmap: Colormap = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"grabc 1.1 by Muhammad A Muquit\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i += 1;
    }
    display = XOpenDisplay(0 as *mut libc::c_void as *mut libc::c_char);
    cmap = (*((*(display as _XPrivDisplay)).screens)
        .offset((*(display as _XPrivDisplay)).default_screen as isize))
    .cmap;
    XSetErrorHandler(Some(
        MXError as unsafe extern "C" fn(*mut Display, *mut XErrorEvent) -> libc::c_int,
    ));
    if display.is_null() {
        ();
        fprintf(
            stderr,
            b"Failed to open local DISPLAY!'n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    status = getWindowColor(display, Some(&mut color));
    if status == 1 as libc::c_int {
        XQueryColor(display, cmap, core::ptr::addr_of_mut!(color));
        r = color.red as libc::c_int >> 8 as libc::c_int;
        g = color.green as libc::c_int >> 8 as libc::c_int;
        b = color.blue as libc::c_int >> 8 as libc::c_int;
        fprintf(
            stdout,
            b"#%02x%02x%02x\n\0" as *const u8 as *const libc::c_char,
            r,
            g,
            b,
        );
        fflush(stdout);
        fprintf(
            stderr,
            b"%d,%d,%d\n\0" as *const u8 as *const libc::c_char,
            r,
            g,
            b,
        );
    } else {
        fprintf(
            stderr,
            b"Failed to grab color!\n\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
static mut cross_cursor: Cursor = 0;
unsafe extern "C" fn selectWindow(
    mut display: *mut Display,
    mut x: Option<&mut libc::c_int>,
    mut y: Option<&mut libc::c_int>,
) -> Window {
    let mut target_cursor: Cursor = 0;
    let mut status: libc::c_int = 0;
    let mut target_window: Window = 0;
    let mut root_window: Window = 0;
    let mut event = _XEvent { type_0: 0 };
    target_window = 0 as *mut libc::c_void as Window;
    if cross_cursor == 0 as *mut libc::c_void as Cursor {
        cross_cursor = XCreateFontCursor(display, 130 as libc::c_int as libc::c_uint);
        if cross_cursor == 0 as *mut libc::c_void as Cursor {
            fprintf(
                stderr,
                b"Failed to create Cross Cursor!\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut libc::c_void as Window;
        }
    }
    target_cursor = cross_cursor;
    root_window = XRootWindow(display, XDefaultScreen(display));
    status = XGrabPointer(
        display,
        root_window,
        0 as libc::c_int,
        ((1 as libc::c_long) << 2 as libc::c_int) as libc::c_uint,
        0 as libc::c_int,
        1 as libc::c_int,
        root_window,
        target_cursor,
        0 as libc::c_long as Time,
    );
    if status == 0 as libc::c_int {
        XAllowEvents(display, 1 as libc::c_int, 0 as libc::c_long as Time);
        XWindowEvent(
            display,
            root_window,
            (1 as libc::c_long) << 2 as libc::c_int,
            core::ptr::addr_of_mut!(event),
        );
        if event.type_0 == 4 as libc::c_int {
            target_window = findSubWindow(
                display,
                root_window,
                event.xbutton.subwindow,
                core::ptr::addr_of_mut!(event.xbutton.x),
                core::ptr::addr_of_mut!(event.xbutton.y),
            );
            if target_window == 0 as *mut libc::c_void as Window {
                fprintf(
                    stderr,
                    b"Failed to get target window, getting root window!\n\0" as *const u8
                        as *const libc::c_char,
                );
                target_window = root_window;
            }
            XUngrabPointer(display, 0 as libc::c_long as Time);
        }
    } else {
        fprintf(
            stderr,
            b"Failed to grab mouse!\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    XFreeCursor(display, cross_cursor);
    *x.as_deref_mut().unwrap() = event.xbutton.x;
    *y.as_deref_mut().unwrap() = event.xbutton.y;
    return target_window;
}
unsafe extern "C" fn findSubWindow(
    mut display: *mut Display,
    mut top_window: Window,
    mut window_to_check: Window,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
) -> Window {
    let mut newx: libc::c_int = 0;
    let mut newy: libc::c_int = 0;
    let mut window: Window = 0;
    if top_window == 0 as *mut libc::c_void as Window {
        return 0 as *mut libc::c_void as Window;
    }
    if window_to_check == 0 as *mut libc::c_void as Window {
        return 0 as *mut libc::c_void as Window;
    }
    window = window_to_check;
    while XTranslateCoordinates(
        display,
        top_window,
        window_to_check,
        (*x),
        (*y),
        core::ptr::addr_of_mut!(newx),
        core::ptr::addr_of_mut!(newy),
        core::ptr::addr_of_mut!(window),
    ) != 0 as libc::c_int
        && window != 0 as *mut libc::c_void as Window
    {
        if window != 0 as *mut libc::c_void as Window {
            top_window = window_to_check;
            window_to_check = window;
            *x = newx;
            *y = newy;
        }
    }
    if window == 0 as *mut libc::c_void as Window {
        window = window_to_check;
    }
    *x = newx;
    *y = newy;
    return window;
}
unsafe extern "C" fn getWindowColor(
    mut display: *mut Display,
    mut color: Option<&mut XColor>,
) -> libc::c_int {
    let mut root_window: Window = 0;
    let mut target_window: Window = 0;
    let mut ximage = 0 as *mut XImage;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    root_window = XRootWindow(display, XDefaultScreen(display));
    target_window = selectWindow(display, Some(&mut x), Some(&mut y));
    if target_window == 0 as *mut libc::c_void as Window {
        return 0 as libc::c_int;
    }
    ximage = XGetImage(
        display,
        target_window,
        x,
        y,
        1 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        !(0 as libc::c_long) as libc::c_ulong,
        2 as libc::c_int,
    );
    if ximage.is_null() {
        ();
        return 0 as libc::c_int;
    }
    (*color.as_deref_mut().unwrap()).pixel =
        (Some((*ximage).f.get_pixel.expect("non-null function pointer")))
            .expect("non-null function pointer")(ximage, 0 as libc::c_int, 0 as libc::c_int);
    (Some(
        (*ximage)
            .f
            .destroy_image
            .expect("non-null function pointer"),
    ))
    .expect("non-null function pointer")(ximage);
    return 1 as libc::c_int;
}
unsafe extern "C" fn MXError(
    mut display: *const Display,
    mut error: *const XErrorEvent,
) -> libc::c_int {
    let mut xerrcode: libc::c_int = 0;
    xerrcode = (*error).error_code as libc::c_int;
    if xerrcode == 11 as libc::c_int
        || xerrcode == 10 as libc::c_int
            && (*error).request_code as libc::c_int == 88 as libc::c_int
    {
        return 0 as libc::c_int;
    } else {
        match (*error).request_code as libc::c_int {
            14 => {
                if (*error).error_code as libc::c_int == 9 as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            3 | 15 => {
                if (*error).error_code as libc::c_int == 3 as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            91 => {
                if (*error).error_code as libc::c_int == 2 as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
    }
    return 1 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }
unsafe extern "C" fn run_static_initializers() {
    cross_cursor = 0 as *const libc::c_void as *mut libc::c_void as Cursor;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
