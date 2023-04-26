
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: * mut crate::src::lodepng::_IO_FILE) -> std::os::raw::c_int;
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::src::lodepng::_IO_FILE;
    fn ftell(__stream: * mut crate::src::lodepng::_IO_FILE) -> std::os::raw::c_long;
    fn fseek(
        __stream: * mut crate::src::lodepng::_IO_FILE,
        __off: std::os::raw::c_long,
        __whence: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    fn fwrite(
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: * mut crate::src::lodepng::_IO_FILE,
    ) -> std::os::raw::c_ulong;
    fn fread(
        _: * mut core::ffi::c_void,
        _: std::os::raw::c_ulong,
        _: std::os::raw::c_ulong,
        _: * mut crate::src::lodepng::_IO_FILE,
    ) -> std::os::raw::c_ulong;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn realloc(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
}
pub type size_t = std::os::raw::c_ulong;
pub type LodePNGColorType = std::os::raw::c_uint;
pub const LCT_MAX_OCTET_VALUE: LodePNGColorType = 255;
pub const LCT_RGBA: LodePNGColorType = 6;
pub const LCT_GREY_ALPHA: LodePNGColorType = 4;
pub const LCT_PALETTE: LodePNGColorType = 3;
pub const LCT_RGB: LodePNGColorType = 2;
pub const LCT_GREY: LodePNGColorType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGState {
    pub decoder: crate::src::lodepng::LodePNGDecoderSettings,
    pub encoder: crate::src::lodepng::LodePNGEncoderSettings,
    pub info_raw: crate::src::lodepng::LodePNGColorMode,
    pub info_png: crate::src::lodepng::LodePNGInfo,
    pub error: std::os::raw::c_uint,
}
impl LodePNGState {
    pub const fn new() -> Self {
        LodePNGState {
        decoder: crate::src::lodepng::LodePNGDecoderSettings::new(),
        encoder: crate::src::lodepng::LodePNGEncoderSettings::new(),
        info_raw: crate::src::lodepng::LodePNGColorMode::new(),
        info_png: crate::src::lodepng::LodePNGInfo::new(),
        error: 0
        }
    }
}

impl std::default::Default for LodePNGState {
    fn default() -> Self { LodePNGState::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGInfo {
    pub compression_method: std::os::raw::c_uint,
    pub filter_method: std::os::raw::c_uint,
    pub interlace_method: std::os::raw::c_uint,
    pub color: crate::src::lodepng::LodePNGColorMode,
    pub background_defined: std::os::raw::c_uint,
    pub background_r: std::os::raw::c_uint,
    pub background_g: std::os::raw::c_uint,
    pub background_b: std::os::raw::c_uint,
    pub text_num: std::os::raw::c_ulong,
    pub text_keys: * mut * mut std::os::raw::c_char,
    pub text_strings: * mut * mut std::os::raw::c_char,
    pub itext_num: std::os::raw::c_ulong,
    pub itext_keys: * mut * mut std::os::raw::c_char,
    pub itext_langtags: * mut * mut std::os::raw::c_char,
    pub itext_transkeys: * mut * mut std::os::raw::c_char,
    pub itext_strings: * mut * mut std::os::raw::c_char,
    pub time_defined: std::os::raw::c_uint,
    pub time: crate::src::lodepng::LodePNGTime,
    pub phys_defined: std::os::raw::c_uint,
    pub phys_x: std::os::raw::c_uint,
    pub phys_y: std::os::raw::c_uint,
    pub phys_unit: std::os::raw::c_uint,
    pub gama_defined: std::os::raw::c_uint,
    pub gama_gamma: std::os::raw::c_uint,
    pub chrm_defined: std::os::raw::c_uint,
    pub chrm_white_x: std::os::raw::c_uint,
    pub chrm_white_y: std::os::raw::c_uint,
    pub chrm_red_x: std::os::raw::c_uint,
    pub chrm_red_y: std::os::raw::c_uint,
    pub chrm_green_x: std::os::raw::c_uint,
    pub chrm_green_y: std::os::raw::c_uint,
    pub chrm_blue_x: std::os::raw::c_uint,
    pub chrm_blue_y: std::os::raw::c_uint,
    pub srgb_defined: std::os::raw::c_uint,
    pub srgb_intent: std::os::raw::c_uint,
    pub iccp_defined: std::os::raw::c_uint,
    pub iccp_name: * mut std::os::raw::c_char,
    pub iccp_profile: * mut std::os::raw::c_uchar,
    pub iccp_profile_size: std::os::raw::c_uint,
    pub sbit_defined: std::os::raw::c_uint,
    pub sbit_r: std::os::raw::c_uint,
    pub sbit_g: std::os::raw::c_uint,
    pub sbit_b: std::os::raw::c_uint,
    pub sbit_a: std::os::raw::c_uint,
    pub unknown_chunks_data: [* mut std::os::raw::c_uchar; 3],
    pub unknown_chunks_size: [std::os::raw::c_ulong; 3],
}
impl LodePNGInfo {
    pub const fn new() -> Self {
        LodePNGInfo {
        compression_method: 0,
        filter_method: 0,
        interlace_method: 0,
        color: crate::src::lodepng::LodePNGColorMode::new(),
        background_defined: 0,
        background_r: 0,
        background_g: 0,
        background_b: 0,
        text_num: 0,
        text_keys: (0 as * mut * mut std::os::raw::c_char),
        text_strings: (0 as * mut * mut std::os::raw::c_char),
        itext_num: 0,
        itext_keys: (0 as * mut * mut std::os::raw::c_char),
        itext_langtags: (0 as * mut * mut std::os::raw::c_char),
        itext_transkeys: (0 as * mut * mut std::os::raw::c_char),
        itext_strings: (0 as * mut * mut std::os::raw::c_char),
        time_defined: 0,
        time: crate::src::lodepng::LodePNGTime::new(),
        phys_defined: 0,
        phys_x: 0,
        phys_y: 0,
        phys_unit: 0,
        gama_defined: 0,
        gama_gamma: 0,
        chrm_defined: 0,
        chrm_white_x: 0,
        chrm_white_y: 0,
        chrm_red_x: 0,
        chrm_red_y: 0,
        chrm_green_x: 0,
        chrm_green_y: 0,
        chrm_blue_x: 0,
        chrm_blue_y: 0,
        srgb_defined: 0,
        srgb_intent: 0,
        iccp_defined: 0,
        iccp_name: (0 as * mut std::os::raw::c_char),
        iccp_profile: (0 as * mut std::os::raw::c_uchar),
        iccp_profile_size: 0,
        sbit_defined: 0,
        sbit_r: 0,
        sbit_g: 0,
        sbit_b: 0,
        sbit_a: 0,
        unknown_chunks_data: [(0 as * mut std::os::raw::c_uchar),(0 as * mut std::os::raw::c_uchar),(0 as * mut std::os::raw::c_uchar),],
        unknown_chunks_size: [0,0,0,]
        }
    }
}

impl std::default::Default for LodePNGInfo {
    fn default() -> Self { LodePNGInfo::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGTime {
    pub year: std::os::raw::c_uint,
    pub month: std::os::raw::c_uint,
    pub day: std::os::raw::c_uint,
    pub hour: std::os::raw::c_uint,
    pub minute: std::os::raw::c_uint,
    pub second: std::os::raw::c_uint,
}
impl LodePNGTime {
    pub const fn new() -> Self {
        LodePNGTime {
        year: 0,
        month: 0,
        day: 0,
        hour: 0,
        minute: 0,
        second: 0
        }
    }
}

impl std::default::Default for LodePNGTime {
    fn default() -> Self { LodePNGTime::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorMode {
    pub colortype: std::os::raw::c_uint,
    pub bitdepth: std::os::raw::c_uint,
    pub palette: * mut std::os::raw::c_uchar,
    pub palettesize: std::os::raw::c_ulong,
    pub key_defined: std::os::raw::c_uint,
    pub key_r: std::os::raw::c_uint,
    pub key_g: std::os::raw::c_uint,
    pub key_b: std::os::raw::c_uint,
}
impl LodePNGColorMode {
    pub const fn new() -> Self {
        LodePNGColorMode {
        colortype: 0,
        bitdepth: 0,
        palette: (0 as * mut std::os::raw::c_uchar),
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0
        }
    }
}

impl std::default::Default for LodePNGColorMode {
    fn default() -> Self { LodePNGColorMode::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGEncoderSettings {
    pub zlibsettings: crate::src::lodepng::LodePNGCompressSettings,
    pub auto_convert: std::os::raw::c_uint,
    pub filter_palette_zero: std::os::raw::c_uint,
    pub filter_strategy: std::os::raw::c_uint,
    pub predefined_filters: * const std::os::raw::c_uchar,
    pub force_palette: std::os::raw::c_uint,
    pub add_id: std::os::raw::c_uint,
    pub text_compression: std::os::raw::c_uint,
}
impl LodePNGEncoderSettings {
    pub const fn new() -> Self {
        LodePNGEncoderSettings {
        zlibsettings: crate::src::lodepng::LodePNGCompressSettings::new(),
        auto_convert: 0,
        filter_palette_zero: 0,
        filter_strategy: 0,
        predefined_filters: (0 as * const std::os::raw::c_uchar),
        force_palette: 0,
        add_id: 0,
        text_compression: 0
        }
    }
}

impl std::default::Default for LodePNGEncoderSettings {
    fn default() -> Self { LodePNGEncoderSettings::new() }
}

pub type LodePNGFilterStrategy = std::os::raw::c_uint;
pub const LFS_PREDEFINED: LodePNGFilterStrategy = 8;
pub const LFS_BRUTE_FORCE: LodePNGFilterStrategy = 7;
pub const LFS_ENTROPY: LodePNGFilterStrategy = 6;
pub const LFS_MINSUM: LodePNGFilterStrategy = 5;
pub const LFS_FOUR: LodePNGFilterStrategy = 4;
pub const LFS_THREE: LodePNGFilterStrategy = 3;
pub const LFS_TWO: LodePNGFilterStrategy = 2;
pub const LFS_ONE: LodePNGFilterStrategy = 1;
pub const LFS_ZERO: LodePNGFilterStrategy = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGCompressSettings {
    pub btype: std::os::raw::c_uint,
    pub use_lz77: std::os::raw::c_uint,
    pub windowsize: std::os::raw::c_uint,
    pub minmatch: std::os::raw::c_uint,
    pub nicematch: std::os::raw::c_uint,
    pub lazymatching: std::os::raw::c_uint,
    pub custom_zlib: Option<unsafe extern "C"  fn(_: * mut * mut std::os::raw::c_uchar,_: * mut std::os::raw::c_ulong,_: * const std::os::raw::c_uchar,_: std::os::raw::c_ulong,_: * const crate::src::lodepng::LodePNGCompressSettings,) -> std::os::raw::c_uint>,
    pub custom_deflate: Option<unsafe extern "C"  fn(_: * mut * mut std::os::raw::c_uchar,_: * mut std::os::raw::c_ulong,_: * const std::os::raw::c_uchar,_: std::os::raw::c_ulong,_: * const crate::src::lodepng::LodePNGCompressSettings,) -> std::os::raw::c_uint>,
    pub custom_context: * const core::ffi::c_void,
}
impl LodePNGCompressSettings {
    pub const fn new() -> Self {
        LodePNGCompressSettings {
        btype: 0,
        use_lz77: 0,
        windowsize: 0,
        minmatch: 0,
        nicematch: 0,
        lazymatching: 0,
        custom_zlib: None,
        custom_deflate: None,
        custom_context: (0 as * const core::ffi::c_void)
        }
    }
}

impl std::default::Default for LodePNGCompressSettings {
    fn default() -> Self { LodePNGCompressSettings::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecoderSettings {
    pub zlibsettings: crate::src::lodepng::LodePNGDecompressSettings,
    pub ignore_crc: std::os::raw::c_uint,
    pub ignore_critical: std::os::raw::c_uint,
    pub ignore_end: std::os::raw::c_uint,
    pub color_convert: std::os::raw::c_uint,
    pub read_text_chunks: std::os::raw::c_uint,
    pub remember_unknown_chunks: std::os::raw::c_uint,
    pub max_text_size: std::os::raw::c_ulong,
    pub max_icc_size: std::os::raw::c_ulong,
}
impl LodePNGDecoderSettings {
    pub const fn new() -> Self {
        LodePNGDecoderSettings {
        zlibsettings: crate::src::lodepng::LodePNGDecompressSettings::new(),
        ignore_crc: 0,
        ignore_critical: 0,
        ignore_end: 0,
        color_convert: 0,
        read_text_chunks: 0,
        remember_unknown_chunks: 0,
        max_text_size: 0,
        max_icc_size: 0
        }
    }
}

impl std::default::Default for LodePNGDecoderSettings {
    fn default() -> Self { LodePNGDecoderSettings::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecompressSettings {
    pub ignore_adler32: std::os::raw::c_uint,
    pub ignore_nlen: std::os::raw::c_uint,
    pub max_output_size: std::os::raw::c_ulong,
    pub custom_zlib: Option<unsafe extern "C"  fn(_: * mut * mut std::os::raw::c_uchar,_: * mut std::os::raw::c_ulong,_: * const std::os::raw::c_uchar,_: std::os::raw::c_ulong,_: * const crate::src::lodepng::LodePNGDecompressSettings,) -> std::os::raw::c_uint>,
    pub custom_inflate: Option<unsafe extern "C"  fn(_: * mut * mut std::os::raw::c_uchar,_: * mut std::os::raw::c_ulong,_: * const std::os::raw::c_uchar,_: std::os::raw::c_ulong,_: * const crate::src::lodepng::LodePNGDecompressSettings,) -> std::os::raw::c_uint>,
    pub custom_context: * const core::ffi::c_void,
}
impl LodePNGDecompressSettings {
    pub const fn new() -> Self {
        LodePNGDecompressSettings {
        ignore_adler32: 0,
        ignore_nlen: 0,
        max_output_size: 0,
        custom_zlib: None,
        custom_inflate: None,
        custom_context: (0 as * const core::ffi::c_void)
        }
    }
}

impl std::default::Default for LodePNGDecompressSettings {
    fn default() -> Self { LodePNGDecompressSettings::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorTree {
    pub children: [* mut crate::src::lodepng::ColorTree; 16],
    pub index: std::os::raw::c_int,
}
impl ColorTree {
    pub const fn new() -> Self {
        ColorTree {
        children: [(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),(0 as * mut crate::src::lodepng::ColorTree),],
        index: 0
        }
    }
}

impl std::default::Default for ColorTree {
    fn default() -> Self { ColorTree::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucvector {
    pub data: * mut std::os::raw::c_uchar,
    pub size: std::os::raw::c_ulong,
    pub allocsize: std::os::raw::c_ulong,
}
impl ucvector {
    pub const fn new() -> Self {
        ucvector {
        data: (0 as * mut std::os::raw::c_uchar),
        size: 0,
        allocsize: 0
        }
    }
}

impl std::default::Default for ucvector {
    fn default() -> Self { ucvector::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGBitReader {
    pub data: * const std::os::raw::c_uchar,
    pub size: std::os::raw::c_ulong,
    pub bitsize: std::os::raw::c_ulong,
    pub bp: std::os::raw::c_ulong,
    pub buffer: std::os::raw::c_uint,
}
impl LodePNGBitReader {
    pub const fn new() -> Self {
        LodePNGBitReader {
        data: (0 as * const std::os::raw::c_uchar),
        size: 0,
        bitsize: 0,
        bp: 0,
        buffer: 0
        }
    }
}

impl std::default::Default for LodePNGBitReader {
    fn default() -> Self { LodePNGBitReader::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub codes: * mut std::os::raw::c_uint,
    pub lengths: * mut std::os::raw::c_uint,
    pub maxbitlen: std::os::raw::c_uint,
    pub numcodes: std::os::raw::c_uint,
    pub table_len: * mut std::os::raw::c_uchar,
    pub table_value: * mut std::os::raw::c_ushort,
}
impl HuffmanTree {
    pub const fn new() -> Self {
        HuffmanTree {
        codes: (0 as * mut std::os::raw::c_uint),
        lengths: (0 as * mut std::os::raw::c_uint),
        maxbitlen: 0,
        numcodes: 0,
        table_len: (0 as * mut std::os::raw::c_uchar),
        table_value: (0 as * mut std::os::raw::c_ushort)
        }
    }
}

impl std::default::Default for HuffmanTree {
    fn default() -> Self { HuffmanTree::new() }
}

pub type FILE = crate::src::lodepng::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: * mut std::os::raw::c_char,
    pub _IO_read_end: * mut std::os::raw::c_char,
    pub _IO_read_base: * mut std::os::raw::c_char,
    pub _IO_write_base: * mut std::os::raw::c_char,
    pub _IO_write_ptr: * mut std::os::raw::c_char,
    pub _IO_write_end: * mut std::os::raw::c_char,
    pub _IO_buf_base: * mut std::os::raw::c_char,
    pub _IO_buf_end: * mut std::os::raw::c_char,
    pub _IO_save_base: * mut std::os::raw::c_char,
    pub _IO_backup_base: * mut std::os::raw::c_char,
    pub _IO_save_end: * mut std::os::raw::c_char,
    pub _markers: * mut crate::src::lodepng::_IO_marker,
    pub _chain: * mut crate::src::lodepng::_IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: std::os::raw::c_long,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: * mut core::ffi::c_void,
    pub _offset: std::os::raw::c_long,
    pub _codecvt: * mut crate::src::lodepng::_IO_codecvt,
    pub _wide_data: * mut crate::src::lodepng::_IO_wide_data,
    pub _freeres_list: * mut crate::src::lodepng::_IO_FILE,
    pub _freeres_buf: * mut core::ffi::c_void,
    pub __pad5: std::os::raw::c_ulong,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
impl _IO_FILE {
    pub const fn new() -> Self {
        _IO_FILE {
        _flags: 0,
        _IO_read_ptr: (0 as * mut std::os::raw::c_char),
        _IO_read_end: (0 as * mut std::os::raw::c_char),
        _IO_read_base: (0 as * mut std::os::raw::c_char),
        _IO_write_base: (0 as * mut std::os::raw::c_char),
        _IO_write_ptr: (0 as * mut std::os::raw::c_char),
        _IO_write_end: (0 as * mut std::os::raw::c_char),
        _IO_buf_base: (0 as * mut std::os::raw::c_char),
        _IO_buf_end: (0 as * mut std::os::raw::c_char),
        _IO_save_base: (0 as * mut std::os::raw::c_char),
        _IO_backup_base: (0 as * mut std::os::raw::c_char),
        _IO_save_end: (0 as * mut std::os::raw::c_char),
        _markers: (0 as * mut crate::src::lodepng::_IO_marker),
        _chain: (0 as * mut crate::src::lodepng::_IO_FILE),
        _fileno: 0,
        _flags2: 0,
        _old_offset: 0,
        _cur_column: 0,
        _vtable_offset: 0,
        _shortbuf: [0,],
        _lock: (0 as * mut core::ffi::c_void),
        _offset: 0,
        _codecvt: (0 as * mut crate::src::lodepng::_IO_codecvt),
        _wide_data: (0 as * mut crate::src::lodepng::_IO_wide_data),
        _freeres_list: (0 as * mut crate::src::lodepng::_IO_FILE),
        _freeres_buf: (0 as * mut core::ffi::c_void),
        __pad5: 0,
        _mode: 0,
        _unused2: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for _IO_FILE {
    fn default() -> Self { _IO_FILE::new() }
}

pub type __off64_t = std::os::raw::c_long;
pub type _IO_lock_t = ();
pub type __off_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub head: * mut std::os::raw::c_int,
    pub chain: * mut std::os::raw::c_ushort,
    pub val: * mut std::os::raw::c_int,
    pub headz: * mut std::os::raw::c_int,
    pub chainz: * mut std::os::raw::c_ushort,
    pub zeros: * mut std::os::raw::c_ushort,
}
impl Hash {
    pub const fn new() -> Self {
        Hash {
        head: (0 as * mut std::os::raw::c_int),
        chain: (0 as * mut std::os::raw::c_ushort),
        val: (0 as * mut std::os::raw::c_int),
        headz: (0 as * mut std::os::raw::c_int),
        chainz: (0 as * mut std::os::raw::c_ushort),
        zeros: (0 as * mut std::os::raw::c_ushort)
        }
    }
}

impl std::default::Default for Hash {
    fn default() -> Self { Hash::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGBitWriter {
    pub data: * mut crate::src::lodepng::ucvector,
    pub bp: std::os::raw::c_uchar,
}
impl LodePNGBitWriter {
    pub const fn new() -> Self {
        LodePNGBitWriter {
        data: (0 as * mut crate::src::lodepng::ucvector),
        bp: 0
        }
    }
}

impl std::default::Default for LodePNGBitWriter {
    fn default() -> Self { LodePNGBitWriter::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uivector {
    pub data: * mut std::os::raw::c_uint,
    pub size: std::os::raw::c_ulong,
    pub allocsize: std::os::raw::c_ulong,
}
impl uivector {
    pub const fn new() -> Self {
        uivector {
        data: (0 as * mut std::os::raw::c_uint),
        size: 0,
        allocsize: 0
        }
    }
}

impl std::default::Default for uivector {
    fn default() -> Self { uivector::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BPMNode {
    pub weight: std::os::raw::c_int,
    pub index: std::os::raw::c_uint,
    pub tail: * mut crate::src::lodepng::BPMNode,
    pub in_use: std::os::raw::c_int,
}
impl BPMNode {
    pub const fn new() -> Self {
        BPMNode {
        weight: 0,
        index: 0,
        tail: (0 as * mut crate::src::lodepng::BPMNode),
        in_use: 0
        }
    }
}

impl std::default::Default for BPMNode {
    fn default() -> Self { BPMNode::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BPMLists {
    pub memsize: std::os::raw::c_uint,
    pub memory: * mut crate::src::lodepng::BPMNode,
    pub numfree: std::os::raw::c_uint,
    pub nextfree: std::os::raw::c_uint,
    pub freelist: * mut * mut crate::src::lodepng::BPMNode,
    pub listsize: std::os::raw::c_uint,
    pub chains0: * mut * mut crate::src::lodepng::BPMNode,
    pub chains1: * mut * mut crate::src::lodepng::BPMNode,
}
impl BPMLists {
    pub const fn new() -> Self {
        BPMLists {
        memsize: 0,
        memory: (0 as * mut crate::src::lodepng::BPMNode),
        numfree: 0,
        nextfree: 0,
        freelist: (0 as * mut * mut crate::src::lodepng::BPMNode),
        listsize: 0,
        chains0: (0 as * mut * mut crate::src::lodepng::BPMNode),
        chains1: (0 as * mut * mut crate::src::lodepng::BPMNode)
        }
    }
}

impl std::default::Default for BPMLists {
    fn default() -> Self { BPMLists::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorStats {
    pub colored: std::os::raw::c_uint,
    pub key: std::os::raw::c_uint,
    pub key_r: std::os::raw::c_ushort,
    pub key_g: std::os::raw::c_ushort,
    pub key_b: std::os::raw::c_ushort,
    pub alpha: std::os::raw::c_uint,
    pub numcolors: std::os::raw::c_uint,
    pub palette: [std::os::raw::c_uchar; 1024],
    pub bits: std::os::raw::c_uint,
    pub numpixels: std::os::raw::c_ulong,
    pub allow_palette: std::os::raw::c_uint,
    pub allow_greyscale: std::os::raw::c_uint,
}
impl LodePNGColorStats {
    pub const fn new() -> Self {
        LodePNGColorStats {
        colored: 0,
        key: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
        alpha: 0,
        numcolors: 0,
        palette: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
        bits: 0,
        numpixels: 0,
        allow_palette: 0,
        allow_greyscale: 0
        }
    }
}

impl std::default::Default for LodePNGColorStats {
    fn default() -> Self { LodePNGColorStats::new() }
}

#[no_mangle]
pub static mut LODEPNG_VERSION_STRING: * const std::os::raw::c_char = (0 as * const std::os::raw::c_char); unsafe fn laertes_init_LODEPNG_VERSION_STRING() {
LODEPNG_VERSION_STRING = b"20221108\0" as *const u8
    as *const std::os::raw::c_char;}//;
unsafe extern "C" fn lodepng_malloc(mut size: std::os::raw::c_ulong) -> * mut core::ffi::c_void {
    return malloc(size);
}
unsafe extern "C" fn lodepng_realloc(
    mut ptr: * mut core::ffi::c_void,
    mut new_size: std::os::raw::c_ulong,
) -> * mut core::ffi::c_void {
    return realloc(ptr, new_size);
}
unsafe extern "C" fn lodepng_free(mut ptr: * mut core::ffi::c_void) {
    free(ptr);
}
unsafe extern "C" fn lodepng_memcpy(
    mut dst: * mut core::ffi::c_void,
    mut src: * const core::ffi::c_void,
    mut size: std::os::raw::c_ulong,
) {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < size {
        *(dst as *mut std::os::raw::c_char)
            .offset(i as isize) = *(src as *const std::os::raw::c_char).offset(i as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn lodepng_memset(
    mut dst: * mut core::ffi::c_void,
    mut value: std::os::raw::c_int,
    mut num: std::os::raw::c_ulong,
) {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < num {
        *(dst as *mut std::os::raw::c_char).offset(i as isize) = value as std::os::raw::c_char;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn lodepng_strlen(mut a: * const std::os::raw::c_char) -> std::os::raw::c_ulong {
    let mut orig = a;
    while *a != 0 {
        a = a.offset(1);
    }
    return a.offset_from(orig) as std::os::raw::c_long as size_t;
}
unsafe extern "C" fn lodepng_addofl<'a1>(
    mut a: std::os::raw::c_ulong,
    mut b: std::os::raw::c_ulong,
    mut result: Option<&'a1 mut std::os::raw::c_ulong>,
) -> std::os::raw::c_int {
    *(borrow_mut(&mut result)).unwrap() = a.wrapping_add(b);
    return (*(borrow(& result)).unwrap() < a) as std::os::raw::c_int;
}
unsafe extern "C" fn lodepng_mulofl<'a1>(
    mut a: std::os::raw::c_ulong,
    mut b: std::os::raw::c_ulong,
    mut result: Option<&'a1 mut std::os::raw::c_ulong>,
) -> std::os::raw::c_int {
    *(borrow_mut(&mut result)).unwrap() = a.wrapping_mul(b);
    return (a != 0 as std::os::raw::c_int as std::os::raw::c_ulong && (*(borrow(& result)).unwrap()).wrapping_div(a) != b)
        as std::os::raw::c_int;
}
unsafe extern "C" fn lodepng_gtofl(
    mut a: std::os::raw::c_ulong,
    mut b: std::os::raw::c_ulong,
    mut c: std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut d: u64 = 0;
    if lodepng_addofl(a, b, Some(&mut d)) != 0 {
        return 1 as std::os::raw::c_int;
    }
    return (d > c) as std::os::raw::c_int;
}
unsafe extern "C" fn uivector_cleanup(mut p: * mut core::ffi::c_void) {
    let ref mut fresh0 = (*(p as *mut uivector)).allocsize;
    *fresh0 = 0 as std::os::raw::c_int as size_t;
    (*(p as *mut uivector)).size = *fresh0;
    lodepng_free((*(p as *mut uivector)).data as *mut std::os::raw::c_void);
    let ref mut fresh1 = (*(p as *mut uivector)).data;
    *fresh1 = 0 as *mut std::os::raw::c_uint;
}
unsafe extern "C" fn uivector_resize(
    mut p: * mut crate::src::lodepng::uivector,
    mut size: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut allocsize = size
        .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong);
    if allocsize > (*p).allocsize {
        let mut newsize = allocsize.wrapping_add((*p).allocsize >> 1 as std::os::raw::c_uint);
        let mut data = lodepng_realloc((*p).data as *mut std::os::raw::c_void, newsize);
        if !data.is_null() {
            (*p).allocsize = newsize;
            let ref mut fresh2 = (*p).data;
            *fresh2 = data as *mut std::os::raw::c_uint;
        } else {
            return 0 as std::os::raw::c_int as std::os::raw::c_uint
        }
    }
    (*p).size = size;
    return 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn uivector_init<'a1>(mut p: Option<&'a1 mut crate::src::lodepng::uivector>) {
    let ref mut fresh3 = (*(borrow_mut(&mut p)).unwrap()).data;
    *fresh3 = 0 as *mut std::os::raw::c_uint;
    let ref mut fresh4 = (*(borrow_mut(&mut p)).unwrap()).allocsize;
    *fresh4 = 0 as std::os::raw::c_int as size_t;
    (*(borrow_mut(&mut p)).unwrap()).size = *fresh4;
}
unsafe extern "C" fn uivector_push_back(
    mut p: * mut crate::src::lodepng::uivector,
    mut c: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    if uivector_resize(p, ((*p).size).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong))
        == 0
    {
        return 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    *((*p).data)
        .offset(
            ((*p).size).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        ) = c;
    return 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn ucvector_reserve(
    mut p: * mut crate::src::lodepng::ucvector,
    mut size: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    if size > (*p).allocsize {
        let mut newsize = size.wrapping_add((*p).allocsize >> 1 as std::os::raw::c_uint);
        let mut data = lodepng_realloc((*p).data as *mut std::os::raw::c_void, newsize);
        if !data.is_null() {
            (*p).allocsize = newsize;
            let ref mut fresh5 = (*p).data;
            *fresh5 = data as *mut std::os::raw::c_uchar;
        } else {
            return 0 as std::os::raw::c_int as std::os::raw::c_uint
        }
    }
    return 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn ucvector_resize(
    mut p: * mut crate::src::lodepng::ucvector,
    mut size: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    (*p).size = size;
    return ucvector_reserve(p, size);
}
 extern "C" fn ucvector_init(
    mut buffer: * mut std::os::raw::c_uchar,
    mut size: std::os::raw::c_ulong,
) -> crate::src::lodepng::ucvector {
    let mut v = ucvector {
        data: 0 as *mut std::os::raw::c_uchar,
        size: 0,
        allocsize: 0,
    };
    v.data = buffer;
    v.size = size;
    v.allocsize = v.size;
    return v;
}
unsafe extern "C" fn string_cleanup<'a1>(mut out: Option<&'a1 mut * mut std::os::raw::c_char>) {
    lodepng_free(*(borrow_mut(&mut out)).unwrap() as *mut std::os::raw::c_void);
    *(borrow_mut(&mut out)).unwrap() = 0 as *mut std::os::raw::c_char;
}
unsafe extern "C" fn alloc_string_sized(
    mut in_0: * const std::os::raw::c_char,
    mut insize: std::os::raw::c_ulong,
) -> * mut std::os::raw::c_char {
    let mut out = lodepng_malloc(insize.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong))
        as *mut std::os::raw::c_char;
    if !out.is_null() {
        lodepng_memcpy(out as *mut std::os::raw::c_void, in_0 as *const std::os::raw::c_void, insize);
        *out.offset(insize as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    }
    return out;
}
unsafe extern "C" fn alloc_string(mut in_0: * const std::os::raw::c_char) -> * mut std::os::raw::c_char {
    return alloc_string_sized(in_0, lodepng_strlen(in_0));
}
unsafe extern "C" fn lodepng_read32bitInt(
    mut buffer: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    return (*buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        << 24 as std::os::raw::c_uint
        | (*buffer.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            << 16 as std::os::raw::c_uint
        | (*buffer.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            << 8 as std::os::raw::c_uint
        | *buffer.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
}
unsafe extern "C" fn lodepng_set32bitInt(
    mut buffer: * mut std::os::raw::c_uchar,
    mut value: std::os::raw::c_uint,
) {
    *buffer
        .offset(
            0 as std::os::raw::c_int as isize,
        ) = (value >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_uchar;
    *buffer
        .offset(
            1 as std::os::raw::c_int as isize,
        ) = (value >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_uchar;
    *buffer
        .offset(
            2 as std::os::raw::c_int as isize,
        ) = (value >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_uchar;
    *buffer
        .offset(
            3 as std::os::raw::c_int as isize,
        ) = (value & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
}
unsafe extern "C" fn lodepng_filesize(
    mut filename: * const std::os::raw::c_char,
) -> std::os::raw::c_long {
    let mut file = 0 as *mut FILE;
    let mut size: i64 = 0;
    file = fopen(filename, b"rb\0" as *const u8 as *const std::os::raw::c_char);
    if file.is_null() {
        return -(1 as std::os::raw::c_int) as std::os::raw::c_long;
    }
    if fseek(file, 0 as std::os::raw::c_int as std::os::raw::c_long, 2 as std::os::raw::c_int)
        != 0 as std::os::raw::c_int
    {
        fclose(file);
        return -(1 as std::os::raw::c_int) as std::os::raw::c_long;
    }
    size = ftell(file);
    if size == 9223372036854775807 as std::os::raw::c_long {
        size = -(1 as std::os::raw::c_int) as std::os::raw::c_long;
    }
    fclose(file);
    return size;
}
unsafe extern "C" fn lodepng_buffer_file(
    mut out: * mut std::os::raw::c_uchar,
    mut size: std::os::raw::c_ulong,
    mut filename: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    let mut file = 0 as *mut FILE;
    let mut readsize: u64 = 0;
    file = fopen(filename, b"rb\0" as *const u8 as *const std::os::raw::c_char);
    if file.is_null() {
        return 78 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    readsize = fread(
        out as *mut std::os::raw::c_void,
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        size,
        file,
    );
    fclose(file);
    if readsize != size {
        return 78 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_load_file<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut filename: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    let mut size = lodepng_filesize(filename);
    if size < 0 as std::os::raw::c_int as std::os::raw::c_long {
        return 78 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    *(borrow_mut(&mut outsize)).unwrap() = size as size_t;
    *(borrow_mut(&mut out)).unwrap() = lodepng_malloc(size as size_t) as *mut std::os::raw::c_uchar;
    if (*(borrow(& out)).unwrap()).is_null() && size > 0 as std::os::raw::c_int as std::os::raw::c_long {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    return lodepng_buffer_file(*(borrow_mut(&mut out)).unwrap(), size as size_t, filename);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_save_file(
    mut buffer: * const std::os::raw::c_uchar,
    mut buffersize: std::os::raw::c_ulong,
    mut filename: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    let mut file = 0 as *mut FILE;
    file = fopen(filename, b"wb\0" as *const u8 as *const std::os::raw::c_char);
    if file.is_null() {
        return 79 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    fwrite(
        buffer as *const std::os::raw::c_void,
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        buffersize,
        file,
    );
    fclose(file);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn LodePNGBitWriter_init<'a1>(
    mut writer: Option<&'a1 mut crate::src::lodepng::LodePNGBitWriter>,
    mut data: * mut crate::src::lodepng::ucvector,
) {
    let ref mut fresh6 = (*(borrow_mut(&mut writer)).unwrap()).data;
    *fresh6 = data;
    (*(borrow_mut(&mut writer)).unwrap()).bp = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
}
unsafe extern "C" fn writeBits<'a1>(
    mut writer: Option<&'a1 mut crate::src::lodepng::LodePNGBitWriter>,
    mut value: std::os::raw::c_uint,
    mut nbits: std::os::raw::c_ulong,
) {
    if nbits == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        if (*(borrow(& writer)).unwrap()).bp as std::os::raw::c_uint & 7 as std::os::raw::c_uint
            == 0 as std::os::raw::c_int as std::os::raw::c_uint
        {
            if ucvector_resize(
                (*(borrow_mut(&mut writer)).unwrap()).data,
                ((*(*(borrow(& writer)).unwrap()).data).size).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ) == 0
            {
                return;
            }
            *((*(*(borrow(& writer)).unwrap()).data).data)
                .offset(
                    ((*(*(borrow(& writer)).unwrap()).data).size)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        }
        let ref mut fresh7 = *((*(*(borrow(& writer)).unwrap()).data).data)
            .offset(
                ((*(*(borrow(& writer)).unwrap()).data).size).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            );
        *fresh7 = (*fresh7 as std::os::raw::c_uint
            | value << ((*(borrow(& writer)).unwrap()).bp as std::os::raw::c_uint & 7 as std::os::raw::c_uint))
            as std::os::raw::c_uchar;
        let ref mut fresh8 = (*(borrow_mut(&mut writer)).unwrap()).bp;
        *fresh8 = (*fresh8).wrapping_add(1);
    } else {
        let mut i: u64 = 0;
        i = 0 as std::os::raw::c_int as size_t;
        while i != nbits {
            if (*(borrow(& writer)).unwrap()).bp as std::os::raw::c_uint & 7 as std::os::raw::c_uint
                == 0 as std::os::raw::c_int as std::os::raw::c_uint
            {
                if ucvector_resize(
                    (*(borrow_mut(&mut writer)).unwrap()).data,
                    ((*(*(borrow(& writer)).unwrap()).data).size)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                ) == 0
                {
                    return;
                }
                *((*(*(borrow(& writer)).unwrap()).data).data)
                    .offset(
                        ((*(*(borrow(& writer)).unwrap()).data).size)
                            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            }
            let ref mut fresh9 = *((*(*(borrow(& writer)).unwrap()).data).data)
                .offset(
                    ((*(*(borrow(& writer)).unwrap()).data).size)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh9 = (*fresh9 as std::os::raw::c_int
                | ((value >> i & 1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar
                    as std::os::raw::c_int)
                    << ((*(borrow(& writer)).unwrap()).bp as std::os::raw::c_uint & 7 as std::os::raw::c_uint))
                as std::os::raw::c_uchar;
            let ref mut fresh10 = (*(borrow_mut(&mut writer)).unwrap()).bp;
            *fresh10 = (*fresh10).wrapping_add(1);
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn writeBitsReversed<'a1>(
    mut writer: Option<&'a1 mut crate::src::lodepng::LodePNGBitWriter>,
    mut value: std::os::raw::c_uint,
    mut nbits: std::os::raw::c_ulong,
) {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i != nbits {
        if (*(borrow(& writer)).unwrap()).bp as std::os::raw::c_uint & 7 as std::os::raw::c_uint
            == 0 as std::os::raw::c_int as std::os::raw::c_uint
        {
            if ucvector_resize(
                (*(borrow_mut(&mut writer)).unwrap()).data,
                ((*(*(borrow(& writer)).unwrap()).data).size).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ) == 0
            {
                return;
            }
            *((*(*(borrow(& writer)).unwrap()).data).data)
                .offset(
                    ((*(*(borrow(& writer)).unwrap()).data).size)
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        }
        let ref mut fresh11 = *((*(*(borrow(& writer)).unwrap()).data).data)
            .offset(
                ((*(*(borrow(& writer)).unwrap()).data).size).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            );
        *fresh11 = (*fresh11 as std::os::raw::c_int
            | ((value
                >> nbits.wrapping_sub(1 as std::os::raw::c_uint as std::os::raw::c_ulong).wrapping_sub(i)
                & 1 as std::os::raw::c_uint) as std::os::raw::c_uchar as std::os::raw::c_int)
                << ((*(borrow(& writer)).unwrap()).bp as std::os::raw::c_uint & 7 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
        let ref mut fresh12 = (*(borrow_mut(&mut writer)).unwrap()).bp;
        *fresh12 = (*fresh12).wrapping_add(1);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn LodePNGBitReader_init<'a1>(
    mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>,
    mut data: * const std::os::raw::c_uchar,
    mut size: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut temp: u64 = 0;
    let ref mut fresh13 = (*(borrow_mut(&mut reader)).unwrap()).data;
    *fresh13 = data;
    (*(borrow_mut(&mut reader)).unwrap()).size = size;
    if lodepng_mulofl(size, 8 as std::os::raw::c_uint as size_t, Some(&mut (*(borrow_mut(&mut reader)).unwrap()).bitsize)) != 0 {
        return 105 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if lodepng_addofl((*(borrow_mut(&mut reader)).unwrap()).bitsize, 64 as std::os::raw::c_uint as size_t, Some(&mut temp)) != 0 {
        return 105 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*(borrow_mut(&mut reader)).unwrap()).bp = 0 as std::os::raw::c_int as size_t;
    (*(borrow_mut(&mut reader)).unwrap()).buffer = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn ensureBits9<'a1>(mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>, mut nbits: std::os::raw::c_ulong) {
    let mut start = (*(borrow(& reader)).unwrap()).bp >> 3 as std::os::raw::c_uint;
    let mut size = (*(borrow_mut(&mut reader)).unwrap()).size;
    if start.wrapping_add(1 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
        (*(borrow_mut(&mut reader)).unwrap())
            .buffer = *((*(borrow(& reader)).unwrap()).data)
            .offset(start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 8 as std::os::raw::c_uint;
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
    } else {
        (*(borrow_mut(&mut reader)).unwrap()).buffer = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        if start.wrapping_add(0 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap())
                .buffer = *((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint;
        }
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
    };
}
unsafe extern "C" fn ensureBits17<'a1>(mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>, mut nbits: std::os::raw::c_ulong) {
    let mut start = (*(borrow(& reader)).unwrap()).bp >> 3 as std::os::raw::c_uint;
    let mut size = (*(borrow_mut(&mut reader)).unwrap()).size;
    if start.wrapping_add(2 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
        (*(borrow_mut(&mut reader)).unwrap())
            .buffer = *((*(borrow(& reader)).unwrap()).data)
            .offset(start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 8 as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 16 as std::os::raw::c_uint;
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
    } else {
        (*(borrow_mut(&mut reader)).unwrap()).buffer = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        if start.wrapping_add(0 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= *((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint;
        }
        if start.wrapping_add(1 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= (*((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint) << 8 as std::os::raw::c_uint;
        }
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
    };
}
unsafe extern "C" fn ensureBits25<'a1>(mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>, mut nbits: std::os::raw::c_ulong) {
    let mut start = (*(borrow(& reader)).unwrap()).bp >> 3 as std::os::raw::c_uint;
    let mut size = (*(borrow_mut(&mut reader)).unwrap()).size;
    if start.wrapping_add(3 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
        (*(borrow_mut(&mut reader)).unwrap())
            .buffer = *((*(borrow(& reader)).unwrap()).data)
            .offset(start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 8 as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 16 as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 24 as std::os::raw::c_uint;
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
    } else {
        (*(borrow_mut(&mut reader)).unwrap()).buffer = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        if start.wrapping_add(0 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= *((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint;
        }
        if start.wrapping_add(1 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= (*((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint) << 8 as std::os::raw::c_uint;
        }
        if start.wrapping_add(2 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= (*((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint) << 16 as std::os::raw::c_uint;
        }
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
    };
}
unsafe extern "C" fn ensureBits32<'a1>(mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>, mut nbits: std::os::raw::c_ulong) {
    let mut start = (*(borrow(& reader)).unwrap()).bp >> 3 as std::os::raw::c_uint;
    let mut size = (*(borrow_mut(&mut reader)).unwrap()).size;
    if start.wrapping_add(4 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
        (*(borrow_mut(&mut reader)).unwrap())
            .buffer = *((*(borrow(& reader)).unwrap()).data)
            .offset(start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 8 as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 16 as std::os::raw::c_uint
            | (*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 24 as std::os::raw::c_uint;
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
        (*(borrow_mut(&mut reader)).unwrap()).buffer
            |= ((*((*(borrow(& reader)).unwrap()).data)
                .offset(start.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 24 as std::os::raw::c_uint)
                << (8 as std::os::raw::c_uint as std::os::raw::c_ulong)
                    .wrapping_sub((*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong);
    } else {
        (*(borrow_mut(&mut reader)).unwrap()).buffer = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        if start.wrapping_add(0 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= *((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint;
        }
        if start.wrapping_add(1 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= (*((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint) << 8 as std::os::raw::c_uint;
        }
        if start.wrapping_add(2 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= (*((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint) << 16 as std::os::raw::c_uint;
        }
        if start.wrapping_add(3 as std::os::raw::c_uint as std::os::raw::c_ulong) < size {
            (*(borrow_mut(&mut reader)).unwrap()).buffer
                |= (*((*(borrow(& reader)).unwrap()).data)
                    .offset(
                        start.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint) << 24 as std::os::raw::c_uint;
        }
        (*(borrow_mut(&mut reader)).unwrap()).buffer >>= (*(borrow(& reader)).unwrap()).bp & 7 as std::os::raw::c_uint as std::os::raw::c_ulong;
    };
}
unsafe extern "C" fn peekBits<'a1>(
    mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>,
    mut nbits: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    return (*(borrow(& reader)).unwrap()).buffer
        & ((1 as std::os::raw::c_uint) << nbits).wrapping_sub(1 as std::os::raw::c_uint);
}
unsafe extern "C" fn advanceBits<'a1>(mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>, mut nbits: std::os::raw::c_ulong) {
    (*(borrow_mut(&mut reader)).unwrap()).buffer >>= nbits;
    let ref mut fresh14 = (*(borrow_mut(&mut reader)).unwrap()).bp;
    *fresh14 = (*fresh14 as std::os::raw::c_ulong).wrapping_add(nbits) as size_t as size_t;
}
unsafe extern "C" fn readBits<'a1>(
    mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>,
    mut nbits: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut result = peekBits(borrow_mut(&mut reader), nbits);
    advanceBits(borrow_mut(&mut reader), nbits);
    return result;
}
 extern "C" fn reverseBits(
    mut bits: std::os::raw::c_uint,
    mut num: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    let mut result = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < num {
        result
            |= (bits >> num.wrapping_sub(i).wrapping_sub(1 as std::os::raw::c_uint)
                & 1 as std::os::raw::c_uint) << i;
        i = i.wrapping_add(1);
    }
    return result;
}
static mut LENGTHBASE: [std::os::raw::c_uint; 29] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_LENGTHBASE() {
LENGTHBASE = [
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    6 as std::os::raw::c_int as std::os::raw::c_uint,
    7 as std::os::raw::c_int as std::os::raw::c_uint,
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    9 as std::os::raw::c_int as std::os::raw::c_uint,
    10 as std::os::raw::c_int as std::os::raw::c_uint,
    11 as std::os::raw::c_int as std::os::raw::c_uint,
    13 as std::os::raw::c_int as std::os::raw::c_uint,
    15 as std::os::raw::c_int as std::os::raw::c_uint,
    17 as std::os::raw::c_int as std::os::raw::c_uint,
    19 as std::os::raw::c_int as std::os::raw::c_uint,
    23 as std::os::raw::c_int as std::os::raw::c_uint,
    27 as std::os::raw::c_int as std::os::raw::c_uint,
    31 as std::os::raw::c_int as std::os::raw::c_uint,
    35 as std::os::raw::c_int as std::os::raw::c_uint,
    43 as std::os::raw::c_int as std::os::raw::c_uint,
    51 as std::os::raw::c_int as std::os::raw::c_uint,
    59 as std::os::raw::c_int as std::os::raw::c_uint,
    67 as std::os::raw::c_int as std::os::raw::c_uint,
    83 as std::os::raw::c_int as std::os::raw::c_uint,
    99 as std::os::raw::c_int as std::os::raw::c_uint,
    115 as std::os::raw::c_int as std::os::raw::c_uint,
    131 as std::os::raw::c_int as std::os::raw::c_uint,
    163 as std::os::raw::c_int as std::os::raw::c_uint,
    195 as std::os::raw::c_int as std::os::raw::c_uint,
    227 as std::os::raw::c_int as std::os::raw::c_uint,
    258 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
static mut LENGTHEXTRA: [std::os::raw::c_uint; 29] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_LENGTHEXTRA() {
LENGTHEXTRA = [
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
static mut DISTANCEBASE: [std::os::raw::c_uint; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_DISTANCEBASE() {
DISTANCEBASE = [
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    7 as std::os::raw::c_int as std::os::raw::c_uint,
    9 as std::os::raw::c_int as std::os::raw::c_uint,
    13 as std::os::raw::c_int as std::os::raw::c_uint,
    17 as std::os::raw::c_int as std::os::raw::c_uint,
    25 as std::os::raw::c_int as std::os::raw::c_uint,
    33 as std::os::raw::c_int as std::os::raw::c_uint,
    49 as std::os::raw::c_int as std::os::raw::c_uint,
    65 as std::os::raw::c_int as std::os::raw::c_uint,
    97 as std::os::raw::c_int as std::os::raw::c_uint,
    129 as std::os::raw::c_int as std::os::raw::c_uint,
    193 as std::os::raw::c_int as std::os::raw::c_uint,
    257 as std::os::raw::c_int as std::os::raw::c_uint,
    385 as std::os::raw::c_int as std::os::raw::c_uint,
    513 as std::os::raw::c_int as std::os::raw::c_uint,
    769 as std::os::raw::c_int as std::os::raw::c_uint,
    1025 as std::os::raw::c_int as std::os::raw::c_uint,
    1537 as std::os::raw::c_int as std::os::raw::c_uint,
    2049 as std::os::raw::c_int as std::os::raw::c_uint,
    3073 as std::os::raw::c_int as std::os::raw::c_uint,
    4097 as std::os::raw::c_int as std::os::raw::c_uint,
    6145 as std::os::raw::c_int as std::os::raw::c_uint,
    8193 as std::os::raw::c_int as std::os::raw::c_uint,
    12289 as std::os::raw::c_int as std::os::raw::c_uint,
    16385 as std::os::raw::c_int as std::os::raw::c_uint,
    24577 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
static mut DISTANCEEXTRA: [std::os::raw::c_uint; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_DISTANCEEXTRA() {
DISTANCEEXTRA = [
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    6 as std::os::raw::c_int as std::os::raw::c_uint,
    6 as std::os::raw::c_int as std::os::raw::c_uint,
    7 as std::os::raw::c_int as std::os::raw::c_uint,
    7 as std::os::raw::c_int as std::os::raw::c_uint,
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    9 as std::os::raw::c_int as std::os::raw::c_uint,
    9 as std::os::raw::c_int as std::os::raw::c_uint,
    10 as std::os::raw::c_int as std::os::raw::c_uint,
    10 as std::os::raw::c_int as std::os::raw::c_uint,
    11 as std::os::raw::c_int as std::os::raw::c_uint,
    11 as std::os::raw::c_int as std::os::raw::c_uint,
    12 as std::os::raw::c_int as std::os::raw::c_uint,
    12 as std::os::raw::c_int as std::os::raw::c_uint,
    13 as std::os::raw::c_int as std::os::raw::c_uint,
    13 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
static mut CLCL_ORDER: [std::os::raw::c_uint; 19] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_CLCL_ORDER() {
CLCL_ORDER = [
    16 as std::os::raw::c_int as std::os::raw::c_uint,
    17 as std::os::raw::c_int as std::os::raw::c_uint,
    18 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    7 as std::os::raw::c_int as std::os::raw::c_uint,
    9 as std::os::raw::c_int as std::os::raw::c_uint,
    6 as std::os::raw::c_int as std::os::raw::c_uint,
    10 as std::os::raw::c_int as std::os::raw::c_uint,
    5 as std::os::raw::c_int as std::os::raw::c_uint,
    11 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    12 as std::os::raw::c_int as std::os::raw::c_uint,
    3 as std::os::raw::c_int as std::os::raw::c_uint,
    13 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    14 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    15 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
unsafe extern "C" fn HuffmanTree_init<'a1>(mut tree: Option<&'a1 mut crate::src::lodepng::HuffmanTree>) {
    let ref mut fresh15 = (*(borrow_mut(&mut tree)).unwrap()).codes;
    *fresh15 = 0 as *mut std::os::raw::c_uint;
    let ref mut fresh16 = (*(borrow_mut(&mut tree)).unwrap()).lengths;
    *fresh16 = 0 as *mut std::os::raw::c_uint;
    let ref mut fresh17 = (*(borrow_mut(&mut tree)).unwrap()).table_len;
    *fresh17 = 0 as *mut std::os::raw::c_uchar;
    let ref mut fresh18 = (*(borrow_mut(&mut tree)).unwrap()).table_value;
    *fresh18 = 0 as *mut std::os::raw::c_ushort;
}
unsafe extern "C" fn HuffmanTree_cleanup<'a1>(mut tree: Option<&'a1 mut crate::src::lodepng::HuffmanTree>) {
    lodepng_free((*(borrow_mut(&mut tree)).unwrap()).codes as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut tree)).unwrap()).lengths as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut tree)).unwrap()).table_len as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut tree)).unwrap()).table_value as *mut std::os::raw::c_void);
}
static mut mask: std::os::raw::c_uint = 0; unsafe fn laertes_init_mask() {
mask = 0;}//;
unsafe extern "C" fn HuffmanTree_makeTable(mut tree: * mut crate::src::lodepng::HuffmanTree) -> std::os::raw::c_uint {
    static mut headsize: std::os::raw::c_uint = 0; unsafe fn laertes_init_headsize() {
headsize = (1 as std::os::raw::c_uint) << 9 as std::os::raw::c_uint;}//;
    let mut i: u64 = 0;
    let mut numpresent: u64 = 0;
    let mut pointer: u64 = 0;
    let mut size: u64 = 0;
    let mut maxlens = lodepng_malloc(
        (headsize as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if maxlens.is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    lodepng_memset(
        maxlens as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (headsize as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    );
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*tree).numcodes as std::os::raw::c_ulong {
        let mut symbol = *((*tree).codes).offset(i as isize);
        let mut l = *((*tree).lengths).offset(i as isize);
        let mut index: u32 = 0;
        if !(l <= 9 as std::os::raw::c_uint) {
            index = reverseBits(
                symbol >> l.wrapping_sub(9 as std::os::raw::c_uint),
                9 as std::os::raw::c_uint,
            );
            *maxlens
                .offset(
                    index as isize,
                ) = if *maxlens.offset(index as isize) > l {
                *maxlens.offset(index as isize)
            } else {
                l
            };
        }
        i = i.wrapping_add(1);
    }
    size = headsize as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i < headsize as std::os::raw::c_ulong {
        let mut l_0 = *maxlens.offset(i as isize);
        if l_0 > 9 as std::os::raw::c_uint {
            size = (size as std::os::raw::c_ulong)
                .wrapping_add(
                    ((1 as std::os::raw::c_uint) << l_0.wrapping_sub(9 as std::os::raw::c_uint))
                        as std::os::raw::c_ulong,
                ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
    }
    let ref mut fresh19 = (*tree).table_len;
    *fresh19 = lodepng_malloc(
        size.wrapping_mul(::std::mem::size_of::<std::os::raw::c_uchar>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uchar;
    let ref mut fresh20 = (*tree).table_value;
    *fresh20 = lodepng_malloc(
        size.wrapping_mul(::std::mem::size_of::<std::os::raw::c_ushort>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_ushort;
    if ((*tree).table_len).is_null() || ((*tree).table_value).is_null() {
        lodepng_free(maxlens as *mut std::os::raw::c_void);
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    i = 0 as std::os::raw::c_int as size_t;
    while i < size {
        *((*tree).table_len).offset(i as isize) = 16 as std::os::raw::c_int as std::os::raw::c_uchar;
        i = i.wrapping_add(1);
    }
    pointer = headsize as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i < headsize as std::os::raw::c_ulong {
        let mut l_1 = *maxlens.offset(i as isize);
        if !(l_1 <= 9 as std::os::raw::c_uint) {
            *((*tree).table_len).offset(i as isize) = l_1 as std::os::raw::c_uchar;
            *((*tree).table_value).offset(i as isize) = pointer as std::os::raw::c_ushort;
            pointer = (pointer as std::os::raw::c_ulong)
                .wrapping_add(
                    ((1 as std::os::raw::c_uint) << l_1.wrapping_sub(9 as std::os::raw::c_uint))
                        as std::os::raw::c_ulong,
                ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
    }
    lodepng_free(maxlens as *mut std::os::raw::c_void);
    numpresent = 0 as std::os::raw::c_int as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*tree).numcodes as std::os::raw::c_ulong {
        let mut l_2 = *((*tree).lengths).offset(i as isize);
        let mut symbol_0: u32 = 0;
        let mut reverse: u32 = 0;
        if !(l_2 == 0 as std::os::raw::c_int as std::os::raw::c_uint) {
            symbol_0 = *((*tree).codes).offset(i as isize);
            reverse = reverseBits(symbol_0, l_2);
            numpresent = numpresent.wrapping_add(1);
            if l_2 <= 9 as std::os::raw::c_uint {
                let mut num = (1 as std::os::raw::c_uint)
                    << (9 as std::os::raw::c_uint).wrapping_sub(l_2);
                let mut j: u32 = 0;
                j = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while j < num {
                    let mut index_0 = reverse | j << l_2;
                    if *((*tree).table_len).offset(index_0 as isize) as std::os::raw::c_int
                        != 16 as std::os::raw::c_int
                    {
                        return 55 as std::os::raw::c_int as std::os::raw::c_uint;
                    }
                    *((*tree).table_len).offset(index_0 as isize) = l_2 as std::os::raw::c_uchar;
                    *((*tree).table_value)
                        .offset(index_0 as isize) = i as std::os::raw::c_ushort;
                    j = j.wrapping_add(1);
                }
            } else {
                let mut index_1 = reverse & mask;
                let mut maxlen = *((*tree).table_len).offset(index_1 as isize)
                    as std::os::raw::c_uint;
                let mut tablelen = maxlen.wrapping_sub(9 as std::os::raw::c_uint);
                let mut start = *((*tree).table_value).offset(index_1 as isize)
                    as std::os::raw::c_uint;
                let mut num_0 = (1 as std::os::raw::c_uint)
                    << tablelen.wrapping_sub(l_2.wrapping_sub(9 as std::os::raw::c_uint));
                let mut j_0: u32 = 0;
                if maxlen < l_2 {
                    return 55 as std::os::raw::c_int as std::os::raw::c_uint;
                }
                j_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while j_0 < num_0 {
                    let mut reverse2 = reverse >> 9 as std::os::raw::c_uint;
                    let mut index2 = start
                        .wrapping_add(
                            reverse2 | j_0 << l_2.wrapping_sub(9 as std::os::raw::c_uint),
                        );
                    *((*tree).table_len).offset(index2 as isize) = l_2 as std::os::raw::c_uchar;
                    *((*tree).table_value).offset(index2 as isize) = i as std::os::raw::c_ushort;
                    j_0 = j_0.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if numpresent < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        i = 0 as std::os::raw::c_int as size_t;
        while i < size {
            if *((*tree).table_len).offset(i as isize) as std::os::raw::c_int
                == 16 as std::os::raw::c_int
            {
                *((*tree).table_len)
                    .offset(
                        i as isize,
                    ) = (if i < headsize as std::os::raw::c_ulong {
                    1 as std::os::raw::c_int as std::os::raw::c_uint
                } else {
                    (9 as std::os::raw::c_uint).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                }) as std::os::raw::c_uchar;
                *((*tree).table_value)
                    .offset(i as isize) = 65535 as std::os::raw::c_uint as std::os::raw::c_ushort;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as std::os::raw::c_int as size_t;
        while i < size {
            if *((*tree).table_len).offset(i as isize) as std::os::raw::c_int
                == 16 as std::os::raw::c_int
            {
                return 55 as std::os::raw::c_int as std::os::raw::c_uint;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn HuffmanTree_makeFromLengths2(
    mut tree: * mut crate::src::lodepng::HuffmanTree,
) -> std::os::raw::c_uint {
    let mut blcount = 0 as *mut std::os::raw::c_uint;
    let mut nextcode = 0 as *mut std::os::raw::c_uint;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut bits: u32 = 0;
    let mut n: u32 = 0;
    let ref mut fresh21 = (*tree).codes;
    *fresh21 = lodepng_malloc(
        ((*tree).numcodes as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    blcount = lodepng_malloc(
        (((*tree).maxbitlen).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    nextcode = lodepng_malloc(
        (((*tree).maxbitlen).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if ((*tree).codes).is_null() || blcount.is_null() || nextcode.is_null() {
        error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if error == 0 {
        n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while n != ((*tree).maxbitlen).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) {
            let ref mut fresh22 = *nextcode.offset(n as isize);
            *fresh22 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            *blcount.offset(n as isize) = *fresh22;
            n = n.wrapping_add(1);
        }
        bits = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while bits != (*tree).numcodes {
            let ref mut fresh23 = *blcount
                .offset(*((*tree).lengths).offset(bits as isize) as isize);
            *fresh23 = (*fresh23).wrapping_add(1);
            bits = bits.wrapping_add(1);
        }
        bits = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        while bits <= (*tree).maxbitlen {
            *nextcode
                .offset(
                    bits as isize,
                ) = (*nextcode
                .offset(bits.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize))
                .wrapping_add(
                    *blcount
                        .offset(
                            bits.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                        ),
                ) << 1 as std::os::raw::c_uint;
            bits = bits.wrapping_add(1);
        }
        n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while n != (*tree).numcodes {
            if *((*tree).lengths).offset(n as isize) != 0 as std::os::raw::c_int as std::os::raw::c_uint
            {
                let ref mut fresh24 = *nextcode
                    .offset(*((*tree).lengths).offset(n as isize) as isize);
                let mut fresh25 = *fresh24;
                *fresh24 = (*fresh24).wrapping_add(1);
                *((*tree).codes).offset(n as isize) = fresh25;
                *((*tree).codes).offset(n as isize)
                    &= ((1 as std::os::raw::c_uint) << *((*tree).lengths).offset(n as isize))
                        .wrapping_sub(1 as std::os::raw::c_uint);
            }
            n = n.wrapping_add(1);
        }
    }
    lodepng_free(blcount as *mut std::os::raw::c_void);
    lodepng_free(nextcode as *mut std::os::raw::c_void);
    if error == 0 {
        error = HuffmanTree_makeTable(tree);
    }
    return error;
}
unsafe extern "C" fn HuffmanTree_makeFromLengths(
    mut tree: * mut crate::src::lodepng::HuffmanTree,
    mut bitlen: * const std::os::raw::c_uint,
    mut numcodes: std::os::raw::c_ulong,
    mut maxbitlen: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    let ref mut fresh26 = (*tree).lengths;
    *fresh26 = lodepng_malloc(
        numcodes.wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if ((*tree).lengths).is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i as std::os::raw::c_ulong != numcodes {
        *((*tree).lengths).offset(i as isize) = *bitlen.offset(i as isize);
        i = i.wrapping_add(1);
    }
    (*tree).numcodes = numcodes as std::os::raw::c_uint;
    (*tree).maxbitlen = maxbitlen;
    return HuffmanTree_makeFromLengths2(tree);
}
unsafe extern "C" fn bpmnode_create(
    mut lists: * mut crate::src::lodepng::BPMLists,
    mut weight: std::os::raw::c_int,
    mut index: std::os::raw::c_uint,
    mut tail: * mut crate::src::lodepng::BPMNode,
) -> * mut crate::src::lodepng::BPMNode {
    let mut i: u32 = 0;
    let mut result = 0 as *mut BPMNode;
    if (*lists).nextfree >= (*lists).numfree {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != (*lists).memsize {
            (*((*lists).memory).offset(i as isize)).in_use = 0 as std::os::raw::c_int;
            i = i.wrapping_add(1);
        }
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != (*lists).listsize {
            let mut node = 0 as *mut BPMNode;
            node = *((*lists).chains0).offset(i as isize);
            while !node.is_null() {
                (*node).in_use = 1 as std::os::raw::c_int;
                node = (*node).tail;
            }
            node = *((*lists).chains1).offset(i as isize);
            while !node.is_null() {
                (*node).in_use = 1 as std::os::raw::c_int;
                node = (*node).tail;
            }
            i = i.wrapping_add(1);
        }
        (*lists).numfree = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != (*lists).memsize {
            if (*((*lists).memory).offset(i as isize)).in_use == 0 {
                let ref mut fresh27 = (*lists).numfree;
                let mut fresh28 = *fresh27;
                *fresh27 = (*fresh27).wrapping_add(1);
                let ref mut fresh29 = *((*lists).freelist).offset(fresh28 as isize);
                *fresh29 = &mut *((*lists).memory).offset(i as isize) as *mut BPMNode;
            }
            i = i.wrapping_add(1);
        }
        (*lists).nextfree = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let ref mut fresh30 = (*lists).nextfree;
    let mut fresh31 = *fresh30;
    *fresh30 = (*fresh30).wrapping_add(1);
    result = *((*lists).freelist).offset(fresh31 as isize);
    (*result).weight = weight;
    (*result).index = index;
    let ref mut fresh32 = (*result).tail;
    *fresh32 = tail;
    return result;
}
unsafe extern "C" fn bpmnode_sort(mut leaves: * mut crate::src::lodepng::BPMNode, mut num: std::os::raw::c_ulong) {
    let mut mem = lodepng_malloc(
        (::std::mem::size_of::<BPMNode>() as std::os::raw::c_ulong).wrapping_mul(num),
    ) as *mut BPMNode;
    let mut width: u64 = 0;
    let mut counter = 0 as std::os::raw::c_int as size_t;
    width = 1 as std::os::raw::c_int as size_t;
    while width < num {
        let mut a = if counter & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
            mem
        } else {
            leaves
        };
        let mut b = if counter & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
            leaves
        } else {
            mem
        };
        let mut p: u64 = 0;
        p = 0 as std::os::raw::c_int as size_t;
        while p < num {
            let mut q = if p.wrapping_add(width) > num {
                num
            } else {
                p.wrapping_add(width)
            };
            let mut r = if p
                .wrapping_add((2 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_mul(width))
                > num
            {
                num
            } else {
                p.wrapping_add((2 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_mul(width))
            };
            let mut i = p;
            let mut j = q;
            let mut k: u64 = 0;
            k = p;
            while k < r {
                if i < q
                    && (j >= r
                        || (*a.offset(i as isize)).weight
                            <= (*a.offset(j as isize)).weight)
                {
                    let mut fresh33 = i;
                    i = i.wrapping_add(1);
                    *b.offset(k as isize) = *a.offset(fresh33 as isize);
                } else {
                    let mut fresh34 = j;
                    j = j.wrapping_add(1);
                    *b.offset(k as isize) = *a.offset(fresh34 as isize);
                }
                k = k.wrapping_add(1);
            }
            p = (p as std::os::raw::c_ulong)
                .wrapping_add((2 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_mul(width))
                as size_t as size_t;
        }
        counter = counter.wrapping_add(1);
        width = (width as std::os::raw::c_ulong).wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            as size_t as size_t;
    }
    if counter & 1 as std::os::raw::c_int as std::os::raw::c_ulong != 0 {
        lodepng_memcpy(
            leaves as *mut std::os::raw::c_void,
            mem as *const std::os::raw::c_void,
            (::std::mem::size_of::<BPMNode>() as std::os::raw::c_ulong).wrapping_mul(num),
        );
    }
    lodepng_free(mem as *mut std::os::raw::c_void);
}
unsafe extern "C" fn boundaryPM(
    mut lists: * mut crate::src::lodepng::BPMLists,
    mut leaves: * mut crate::src::lodepng::BPMNode,
    mut numpresent: std::os::raw::c_ulong,
    mut c: std::os::raw::c_int,
    mut num: std::os::raw::c_int,
) {
    let mut lastindex = (**((*lists).chains1).offset(c as isize)).index;
    if c == 0 as std::os::raw::c_int {
        if lastindex as std::os::raw::c_ulong >= numpresent {
            return;
        }
        let ref mut fresh35 = *((*lists).chains0).offset(c as isize);
        *fresh35 = *((*lists).chains1).offset(c as isize);
        let ref mut fresh36 = *((*lists).chains1).offset(c as isize);
        *fresh36 = bpmnode_create(
            lists,
            (*leaves.offset(lastindex as isize)).weight,
            lastindex.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint),
            0 as *mut BPMNode,
        );
    } else {
        let mut sum = (**((*lists).chains0).offset((c - 1 as std::os::raw::c_int) as isize))
            .weight
            + (**((*lists).chains1).offset((c - 1 as std::os::raw::c_int) as isize)).weight;
        let ref mut fresh37 = *((*lists).chains0).offset(c as isize);
        *fresh37 = *((*lists).chains1).offset(c as isize);
        if (lastindex as std::os::raw::c_ulong) < numpresent
            && sum > (*leaves.offset(lastindex as isize)).weight
        {
            let ref mut fresh38 = *((*lists).chains1).offset(c as isize);
            *fresh38 = bpmnode_create(
                lists,
                (*leaves.offset(lastindex as isize)).weight,
                lastindex.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint),
                (**((*lists).chains1).offset(c as isize)).tail,
            );
            return;
        }
        let ref mut fresh39 = *((*lists).chains1).offset(c as isize);
        *fresh39 = bpmnode_create(
            lists,
            sum,
            lastindex,
            *((*lists).chains1).offset((c - 1 as std::os::raw::c_int) as isize),
        );
        if (num + 1 as std::os::raw::c_int)
            < (2 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul(numpresent)
                .wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int
        {
            boundaryPM(lists, leaves, numpresent, c - 1 as std::os::raw::c_int, num);
            boundaryPM(lists, leaves, numpresent, c - 1 as std::os::raw::c_int, num);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_huffman_code_lengths(
    mut lengths: * mut std::os::raw::c_uint,
    mut frequencies: * const std::os::raw::c_uint,
    mut numcodes: std::os::raw::c_ulong,
    mut maxbitlen: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u32 = 0;
    let mut numpresent = 0 as std::os::raw::c_int as size_t;
    let mut leaves = 0 as *mut BPMNode;
    if numcodes == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 80 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (1 as std::os::raw::c_uint) << maxbitlen < numcodes as std::os::raw::c_uint {
        return 80 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    leaves = lodepng_malloc(
        numcodes.wrapping_mul(::std::mem::size_of::<BPMNode>() as std::os::raw::c_ulong),
    ) as *mut BPMNode;
    if leaves.is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i as std::os::raw::c_ulong != numcodes {
        if *frequencies.offset(i as isize) > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*leaves.offset(numpresent as isize))
                .weight = *frequencies.offset(i as isize) as std::os::raw::c_int;
            (*leaves.offset(numpresent as isize)).index = i;
            numpresent = numpresent.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    lodepng_memset(
        lengths as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        numcodes.wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    );
    if numpresent == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        let ref mut fresh40 = *lengths.offset(1 as std::os::raw::c_int as isize);
        *fresh40 = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        *lengths.offset(0 as std::os::raw::c_int as isize) = *fresh40;
    } else if numpresent == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        *lengths
            .offset(
                (*leaves.offset(0 as std::os::raw::c_int as isize)).index as isize,
            ) = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        *lengths
            .offset(
                (if (*leaves.offset(0 as std::os::raw::c_int as isize)).index
                    == 0 as std::os::raw::c_int as std::os::raw::c_uint
                {
                    1 as std::os::raw::c_int
                } else {
                    0 as std::os::raw::c_int
                }) as isize,
            ) = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    } else {
        let mut lists = BPMLists {
            memsize: 0,
            memory: 0 as *mut BPMNode,
            numfree: 0,
            nextfree: 0,
            freelist: 0 as *mut *mut BPMNode,
            listsize: 0,
            chains0: 0 as *mut *mut BPMNode,
            chains1: 0 as *mut *mut BPMNode,
        };
        let mut node = 0 as *mut BPMNode;
        bpmnode_sort(leaves, numpresent);
        lists.listsize = maxbitlen;
        lists
            .memsize = (2 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_mul(maxbitlen)
            .wrapping_mul(maxbitlen.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint));
        lists.nextfree = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        lists.numfree = lists.memsize;
        lists
            .memory = lodepng_malloc(
            (lists.memsize as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<BPMNode>() as std::os::raw::c_ulong),
        ) as *mut BPMNode;
        lists
            .freelist = lodepng_malloc(
            (lists.memsize as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut BPMNode>() as std::os::raw::c_ulong),
        ) as *mut *mut BPMNode;
        lists
            .chains0 = lodepng_malloc(
            (lists.listsize as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut BPMNode>() as std::os::raw::c_ulong),
        ) as *mut *mut BPMNode;
        lists
            .chains1 = lodepng_malloc(
            (lists.listsize as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut BPMNode>() as std::os::raw::c_ulong),
        ) as *mut *mut BPMNode;
        if (lists.memory).is_null() || (lists.freelist).is_null()
            || (lists.chains0).is_null() || (lists.chains1).is_null()
        {
            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if error == 0 {
            i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while i != lists.memsize {
                let ref mut fresh41 = *(lists.freelist).offset(i as isize);
                *fresh41 = &mut *(lists.memory).offset(i as isize) as *mut BPMNode;
                i = i.wrapping_add(1);
            }
            bpmnode_create(
                &mut lists,
                (*leaves.offset(0 as std::os::raw::c_int as isize)).weight,
                1 as std::os::raw::c_int as std::os::raw::c_uint,
                0 as *mut BPMNode,
            );
            bpmnode_create(
                &mut lists,
                (*leaves.offset(1 as std::os::raw::c_int as isize)).weight,
                2 as std::os::raw::c_int as std::os::raw::c_uint,
                0 as *mut BPMNode,
            );
            i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while i != lists.listsize {
                let ref mut fresh42 = *(lists.chains0).offset(i as isize);
                *fresh42 = &mut *(lists.memory).offset(0 as std::os::raw::c_int as isize)
                    as *mut BPMNode;
                let ref mut fresh43 = *(lists.chains1).offset(i as isize);
                *fresh43 = &mut *(lists.memory).offset(1 as std::os::raw::c_int as isize)
                    as *mut BPMNode;
                i = i.wrapping_add(1);
            }
            i = 2 as std::os::raw::c_int as std::os::raw::c_uint;
            while i as std::os::raw::c_ulong
                != (2 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_mul(numpresent)
                    .wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            {
                boundaryPM(
                    &mut lists,
                    leaves,
                    numpresent,
                    maxbitlen as std::os::raw::c_int - 1 as std::os::raw::c_int,
                    i as std::os::raw::c_int,
                );
                i = i.wrapping_add(1);
            }
            node = *(lists.chains1)
                .offset(
                    maxbitlen.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                );
            while !node.is_null() {
                i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while i != (*node).index {
                    let ref mut fresh44 = *lengths
                        .offset((*leaves.offset(i as isize)).index as isize);
                    *fresh44 = (*fresh44).wrapping_add(1);
                    i = i.wrapping_add(1);
                }
                node = (*node).tail;
            }
        }
        lodepng_free(lists.memory as *mut std::os::raw::c_void);
        lodepng_free(lists.freelist as *mut std::os::raw::c_void);
        lodepng_free(lists.chains0 as *mut std::os::raw::c_void);
        lodepng_free(lists.chains1 as *mut std::os::raw::c_void);
    }
    lodepng_free(leaves as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn HuffmanTree_makeFromFrequencies(
    mut tree: * mut crate::src::lodepng::HuffmanTree,
    mut frequencies: * const std::os::raw::c_uint,
    mut mincodes: std::os::raw::c_ulong,
    mut numcodes: std::os::raw::c_ulong,
    mut maxbitlen: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while *frequencies
        .offset(numcodes.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize) == 0
        && numcodes > mincodes
    {
        numcodes = numcodes.wrapping_sub(1);
    }
    let ref mut fresh45 = (*tree).lengths;
    *fresh45 = lodepng_malloc(
        numcodes.wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if ((*tree).lengths).is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*tree).maxbitlen = maxbitlen;
    (*tree).numcodes = numcodes as std::os::raw::c_uint;
    error = lodepng_huffman_code_lengths(
        (*tree).lengths,
        frequencies,
        numcodes,
        maxbitlen,
    );
    if error == 0 {
        error = HuffmanTree_makeFromLengths2(tree);
    }
    return error;
}
unsafe extern "C" fn generateFixedLitLenTree(
    mut tree: * mut crate::src::lodepng::HuffmanTree,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut bitlen = lodepng_malloc(
        (288 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if bitlen.is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i <= 143 as std::os::raw::c_int as std::os::raw::c_uint {
        *bitlen.offset(i as isize) = 8 as std::os::raw::c_int as std::os::raw::c_uint;
        i = i.wrapping_add(1);
    }
    i = 144 as std::os::raw::c_int as std::os::raw::c_uint;
    while i <= 255 as std::os::raw::c_int as std::os::raw::c_uint {
        *bitlen.offset(i as isize) = 9 as std::os::raw::c_int as std::os::raw::c_uint;
        i = i.wrapping_add(1);
    }
    i = 256 as std::os::raw::c_int as std::os::raw::c_uint;
    while i <= 279 as std::os::raw::c_int as std::os::raw::c_uint {
        *bitlen.offset(i as isize) = 7 as std::os::raw::c_int as std::os::raw::c_uint;
        i = i.wrapping_add(1);
    }
    i = 280 as std::os::raw::c_int as std::os::raw::c_uint;
    while i <= 287 as std::os::raw::c_int as std::os::raw::c_uint {
        *bitlen.offset(i as isize) = 8 as std::os::raw::c_int as std::os::raw::c_uint;
        i = i.wrapping_add(1);
    }
    error = HuffmanTree_makeFromLengths(
        tree,
        bitlen,
        288 as std::os::raw::c_int as size_t,
        15 as std::os::raw::c_int as std::os::raw::c_uint,
    );
    lodepng_free(bitlen as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn generateFixedDistanceTree(
    mut tree: * mut crate::src::lodepng::HuffmanTree,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut bitlen = lodepng_malloc(
        (32 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if bitlen.is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 32 as std::os::raw::c_int as std::os::raw::c_uint {
        *bitlen.offset(i as isize) = 5 as std::os::raw::c_int as std::os::raw::c_uint;
        i = i.wrapping_add(1);
    }
    error = HuffmanTree_makeFromLengths(
        tree,
        bitlen,
        32 as std::os::raw::c_int as size_t,
        15 as std::os::raw::c_int as std::os::raw::c_uint,
    );
    lodepng_free(bitlen as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn huffmanDecodeSymbol<'a1, 'a2>(
    mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>,
    mut codetree: Option<&'a2 crate::src::lodepng::HuffmanTree>,
) -> std::os::raw::c_uint {
    let mut code = peekBits(borrow_mut(&mut reader), 9 as std::os::raw::c_uint as size_t) as std::os::raw::c_ushort;
    let mut l = *((*((codetree).clone()).unwrap()).table_len).offset(code as isize) as std::os::raw::c_ushort;
    let mut value = *((*((codetree).clone()).unwrap()).table_value).offset(code as isize);
    if l as std::os::raw::c_uint <= 9 as std::os::raw::c_uint {
        advanceBits(borrow_mut(&mut reader), l as size_t);
        return value as std::os::raw::c_uint;
    } else {
        advanceBits(borrow_mut(&mut reader), 9 as std::os::raw::c_uint as size_t);
        value = (value as std::os::raw::c_uint)
            .wrapping_add(
                peekBits(
                    borrow_mut(&mut reader),
                    (l as std::os::raw::c_uint).wrapping_sub(9 as std::os::raw::c_uint) as size_t,
                ),
            ) as std::os::raw::c_ushort as std::os::raw::c_ushort;
        advanceBits(
            borrow_mut(&mut reader),
            (*((*((codetree).clone()).unwrap()).table_len).offset(value as isize) as std::os::raw::c_uint)
                .wrapping_sub(9 as std::os::raw::c_uint) as size_t,
        );
        return *((*((codetree).clone()).unwrap()).table_value).offset(value as isize) as std::os::raw::c_uint;
    };
}
unsafe extern "C" fn getTreeInflateFixed(
    mut tree_ll: * mut crate::src::lodepng::HuffmanTree,
    mut tree_d: * mut crate::src::lodepng::HuffmanTree,
) -> std::os::raw::c_uint {
    let mut error = generateFixedLitLenTree(tree_ll);
    if error != 0 {
        return error;
    }
    return generateFixedDistanceTree(tree_d);
}
unsafe extern "C" fn getTreeInflateDynamic<'a1>(
    mut tree_ll: * mut crate::src::lodepng::HuffmanTree,
    mut tree_d: * mut crate::src::lodepng::HuffmanTree,
    mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut n: u32 = 0;
    let mut HLIT: u32 = 0;
    let mut HDIST: u32 = 0;
    let mut HCLEN: u32 = 0;
    let mut i: u32 = 0;
    let mut bitlen_ll = 0 as *mut std::os::raw::c_uint;
    let mut bitlen_d = 0 as *mut std::os::raw::c_uint;
    let mut bitlen_cl = 0 as *mut std::os::raw::c_uint;
    let mut tree_cl = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    if ((*(borrow(& reader)).unwrap()).bitsize).wrapping_sub((*(borrow(& reader)).unwrap()).bp)
        < 14 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 49 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    ensureBits17(borrow_mut(&mut reader), 14 as std::os::raw::c_int as size_t);
    HLIT = (readBits(borrow_mut(&mut reader), 5 as std::os::raw::c_int as size_t))
        .wrapping_add(257 as std::os::raw::c_int as std::os::raw::c_uint);
    HDIST = (readBits(borrow_mut(&mut reader), 5 as std::os::raw::c_int as size_t))
        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
    HCLEN = (readBits(borrow_mut(&mut reader), 4 as std::os::raw::c_int as size_t))
        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_uint);
    bitlen_cl = lodepng_malloc(
        (19 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if bitlen_cl.is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    HuffmanTree_init(Some(&mut tree_cl));
    if error == 0 {
        if lodepng_gtofl(
            (*(borrow_mut(&mut reader)).unwrap()).bp,
            HCLEN.wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
            (*(borrow_mut(&mut reader)).unwrap()).bitsize,
        ) != 0
        {
            error = 50 as std::os::raw::c_int as std::os::raw::c_uint;
        } else {
            i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while i != HCLEN {
                ensureBits9(borrow_mut(&mut reader), 3 as std::os::raw::c_int as size_t);
                *bitlen_cl
                    .offset(
                        CLCL_ORDER[i as usize] as isize,
                    ) = readBits(borrow_mut(&mut reader), 3 as std::os::raw::c_int as size_t);
                i = i.wrapping_add(1);
            }
            i = HCLEN;
            while i != 19 as std::os::raw::c_int as std::os::raw::c_uint {
                *bitlen_cl
                    .offset(
                        CLCL_ORDER[i as usize] as isize,
                    ) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                i = i.wrapping_add(1);
            }
            error = HuffmanTree_makeFromLengths(
                &mut tree_cl,
                bitlen_cl,
                19 as std::os::raw::c_int as size_t,
                7 as std::os::raw::c_int as std::os::raw::c_uint,
            );
            if !(error != 0) {
                bitlen_ll = lodepng_malloc(
                    (288 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong,
                        ),
                ) as *mut std::os::raw::c_uint;
                bitlen_d = lodepng_malloc(
                    (32 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong,
                        ),
                ) as *mut std::os::raw::c_uint;
                if bitlen_ll.is_null() || bitlen_d.is_null() {
                    error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                } else {
                    lodepng_memset(
                        bitlen_ll as *mut std::os::raw::c_void,
                        0 as std::os::raw::c_int,
                        (288 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong,
                            ),
                    );
                    lodepng_memset(
                        bitlen_d as *mut std::os::raw::c_void,
                        0 as std::os::raw::c_int,
                        (32 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong,
                            ),
                    );
                    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    while i < HLIT.wrapping_add(HDIST) {
                        let mut code: u32 = 0;
                        ensureBits25(borrow_mut(&mut reader), 22 as std::os::raw::c_int as size_t);
                        code = huffmanDecodeSymbol(borrow_mut(&mut reader), Some(&mut tree_cl));
                        if code <= 15 as std::os::raw::c_int as std::os::raw::c_uint {
                            if i < HLIT {
                                *bitlen_ll.offset(i as isize) = code;
                            } else {
                                *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) = code;
                            }
                            i = i.wrapping_add(1);
                        } else if code == 16 as std::os::raw::c_int as std::os::raw::c_uint {
                            let mut replength = 3 as std::os::raw::c_int as std::os::raw::c_uint;
                            let mut value: u32 = 0;
                            if i == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                error = 54 as std::os::raw::c_int as std::os::raw::c_uint;
                                break;
                            } else {
                                replength = replength
                                    .wrapping_add(readBits(borrow_mut(&mut reader), 2 as std::os::raw::c_int as size_t));
                                if i < HLIT.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) {
                                    value = *bitlen_ll
                                        .offset(
                                            i.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                                        );
                                } else {
                                    value = *bitlen_d
                                        .offset(
                                            i
                                                .wrapping_sub(HLIT)
                                                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                                        );
                                }
                                n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                while n < replength {
                                    if i >= HLIT.wrapping_add(HDIST) {
                                        error = 13 as std::os::raw::c_int as std::os::raw::c_uint;
                                        break;
                                    } else {
                                        if i < HLIT {
                                            *bitlen_ll.offset(i as isize) = value;
                                        } else {
                                            *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) = value;
                                        }
                                        i = i.wrapping_add(1);
                                        n = n.wrapping_add(1);
                                    }
                                }
                            }
                        } else if code == 17 as std::os::raw::c_int as std::os::raw::c_uint {
                            let mut replength_0 = 3 as std::os::raw::c_int as std::os::raw::c_uint;
                            replength_0 = replength_0
                                .wrapping_add(readBits(borrow_mut(&mut reader), 3 as std::os::raw::c_int as size_t));
                            n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            while n < replength_0 {
                                if i >= HLIT.wrapping_add(HDIST) {
                                    error = 14 as std::os::raw::c_int as std::os::raw::c_uint;
                                    break;
                                } else {
                                    if i < HLIT {
                                        *bitlen_ll
                                            .offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                    } else {
                                        *bitlen_d
                                            .offset(
                                                i.wrapping_sub(HLIT) as isize,
                                            ) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                    }
                                    i = i.wrapping_add(1);
                                    n = n.wrapping_add(1);
                                }
                            }
                        } else if code == 18 as std::os::raw::c_int as std::os::raw::c_uint {
                            let mut replength_1 = 11 as std::os::raw::c_int as std::os::raw::c_uint;
                            replength_1 = replength_1
                                .wrapping_add(readBits(borrow_mut(&mut reader), 7 as std::os::raw::c_int as size_t));
                            n = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            while n < replength_1 {
                                if i >= HLIT.wrapping_add(HDIST) {
                                    error = 15 as std::os::raw::c_int as std::os::raw::c_uint;
                                    break;
                                } else {
                                    if i < HLIT {
                                        *bitlen_ll
                                            .offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                    } else {
                                        *bitlen_d
                                            .offset(
                                                i.wrapping_sub(HLIT) as isize,
                                            ) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                    }
                                    i = i.wrapping_add(1);
                                    n = n.wrapping_add(1);
                                }
                            }
                        } else {
                            error = 16 as std::os::raw::c_int as std::os::raw::c_uint;
                            break;
                        }
                        if !((*(borrow(& reader)).unwrap()).bp > (*(borrow(& reader)).unwrap()).bitsize) {
                            continue;
                        }
                        error = 50 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    }
                    if !(error != 0) {
                        if *bitlen_ll.offset(256 as std::os::raw::c_int as isize)
                            == 0 as std::os::raw::c_int as std::os::raw::c_uint
                        {
                            error = 64 as std::os::raw::c_int as std::os::raw::c_uint;
                        } else {
                            error = HuffmanTree_makeFromLengths(
                                tree_ll,
                                bitlen_ll,
                                288 as std::os::raw::c_int as size_t,
                                15 as std::os::raw::c_int as std::os::raw::c_uint,
                            );
                            if !(error != 0) {
                                error = HuffmanTree_makeFromLengths(
                                    tree_d,
                                    bitlen_d,
                                    32 as std::os::raw::c_int as size_t,
                                    15 as std::os::raw::c_int as std::os::raw::c_uint,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    lodepng_free(bitlen_cl as *mut std::os::raw::c_void);
    lodepng_free(bitlen_ll as *mut std::os::raw::c_void);
    lodepng_free(bitlen_d as *mut std::os::raw::c_void);
    HuffmanTree_cleanup(Some(&mut tree_cl));
    return error;
}
unsafe extern "C" fn inflateHuffmanBlock<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>,
    mut btype: std::os::raw::c_uint,
    mut max_output_size: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut tree_ll = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    let mut tree_d = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    let reserved_size = 260 as std::os::raw::c_int as size_t;
    let mut done = 0 as std::os::raw::c_int;
    if ucvector_reserve(out, ((*out).size).wrapping_add(reserved_size)) == 0 {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    HuffmanTree_init(Some(&mut tree_ll));
    HuffmanTree_init(Some(&mut tree_d));
    if btype == 1 as std::os::raw::c_int as std::os::raw::c_uint {
        error = getTreeInflateFixed(&mut tree_ll, &mut tree_d);
    } else {
        error = getTreeInflateDynamic(&mut tree_ll, &mut tree_d, borrow_mut(&mut reader));
    }
    while error == 0 && done == 0 {
        let mut code_ll: u32 = 0;
        ensureBits32(borrow_mut(&mut reader), 30 as std::os::raw::c_int as size_t);
        code_ll = huffmanDecodeSymbol(borrow_mut(&mut reader), Some(&mut tree_ll));
        if code_ll <= 255 as std::os::raw::c_int as std::os::raw::c_uint {
            let ref mut fresh46 = (*out).size;
            let mut fresh47 = *fresh46;
            *fresh46 = (*fresh46).wrapping_add(1);
            *((*out).data).offset(fresh47 as isize) = code_ll as std::os::raw::c_uchar;
            code_ll = huffmanDecodeSymbol(borrow_mut(&mut reader), Some(&mut tree_ll));
        }
        if code_ll <= 255 as std::os::raw::c_int as std::os::raw::c_uint {
            let ref mut fresh48 = (*out).size;
            let mut fresh49 = *fresh48;
            *fresh48 = (*fresh48).wrapping_add(1);
            *((*out).data).offset(fresh49 as isize) = code_ll as std::os::raw::c_uchar;
        } else if code_ll >= 257 as std::os::raw::c_int as std::os::raw::c_uint
            && code_ll <= 285 as std::os::raw::c_int as std::os::raw::c_uint
        {
            let mut code_d: u32 = 0;
            let mut distance: u32 = 0;
            let mut numextrabits_l: u32 = 0;
            let mut numextrabits_d: u32 = 0;
            let mut start: u64 = 0;
            let mut backward: u64 = 0;
            let mut length: u64 = 0;
            length = LENGTHBASE[code_ll.wrapping_sub(257 as std::os::raw::c_int as std::os::raw::c_uint)
                as usize] as size_t;
            numextrabits_l = LENGTHEXTRA[code_ll
                .wrapping_sub(257 as std::os::raw::c_int as std::os::raw::c_uint) as usize];
            if numextrabits_l != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                ensureBits25(borrow_mut(&mut reader), 5 as std::os::raw::c_int as size_t);
                length = (length as std::os::raw::c_ulong)
                    .wrapping_add(
                        readBits(borrow_mut(&mut reader), numextrabits_l as size_t) as std::os::raw::c_ulong,
                    ) as size_t as size_t;
            }
            ensureBits32(borrow_mut(&mut reader), 28 as std::os::raw::c_int as size_t);
            code_d = huffmanDecodeSymbol(borrow_mut(&mut reader), Some(&mut tree_d));
            if code_d > 29 as std::os::raw::c_int as std::os::raw::c_uint {
                if code_d <= 31 as std::os::raw::c_int as std::os::raw::c_uint {
                    error = 18 as std::os::raw::c_int as std::os::raw::c_uint;
                    break;
                } else {
                    error = 16 as std::os::raw::c_int as std::os::raw::c_uint;
                    break;
                }
            } else {
                distance = DISTANCEBASE[code_d as usize];
                numextrabits_d = DISTANCEEXTRA[code_d as usize];
                if numextrabits_d != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    distance = distance
                        .wrapping_add(readBits(borrow_mut(&mut reader), numextrabits_d as size_t));
                }
                start = (*out).size;
                if distance as std::os::raw::c_ulong > start {
                    error = 52 as std::os::raw::c_int as std::os::raw::c_uint;
                    break;
                } else {
                    backward = start.wrapping_sub(distance as std::os::raw::c_ulong);
                    let ref mut fresh50 = (*out).size;
                    *fresh50 = (*fresh50 as std::os::raw::c_ulong).wrapping_add(length) as size_t
                        as size_t;
                    if (distance as std::os::raw::c_ulong) < length {
                        let mut forward: u64 = 0;
                        lodepng_memcpy(
                            ((*out).data).offset(start as isize) as *mut std::os::raw::c_void,
                            ((*out).data).offset(backward as isize)
                                as *const std::os::raw::c_void,
                            distance as size_t,
                        );
                        start = (start as std::os::raw::c_ulong)
                            .wrapping_add(distance as std::os::raw::c_ulong) as size_t as size_t;
                        forward = distance as size_t;
                        while forward < length {
                            let mut fresh51 = backward;
                            backward = backward.wrapping_add(1);
                            let mut fresh52 = start;
                            start = start.wrapping_add(1);
                            *((*out).data)
                                .offset(
                                    fresh52 as isize,
                                ) = *((*out).data).offset(fresh51 as isize);
                            forward = forward.wrapping_add(1);
                        }
                    } else {
                        lodepng_memcpy(
                            ((*out).data).offset(start as isize) as *mut std::os::raw::c_void,
                            ((*out).data).offset(backward as isize)
                                as *const std::os::raw::c_void,
                            length,
                        );
                    }
                }
            }
        } else if code_ll == 256 as std::os::raw::c_int as std::os::raw::c_uint {
            done = 1 as std::os::raw::c_int;
        } else {
            error = 16 as std::os::raw::c_int as std::os::raw::c_uint;
            break;
        }
        if ((*out).allocsize).wrapping_sub((*out).size) < reserved_size {
            if ucvector_reserve(out, ((*out).size).wrapping_add(reserved_size)) == 0 {
                error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                break;
            }
        }
        if (*(borrow(& reader)).unwrap()).bp > (*(borrow(& reader)).unwrap()).bitsize {
            error = 51 as std::os::raw::c_int as std::os::raw::c_uint;
            break;
        } else {
            if !(max_output_size != 0 && (*out).size > max_output_size) {
                continue;
            }
            error = 109 as std::os::raw::c_int as std::os::raw::c_uint;
            break;
        }
    }
    HuffmanTree_cleanup(Some(&mut tree_ll));
    HuffmanTree_cleanup(Some(&mut tree_d));
    return error;
}
unsafe extern "C" fn inflateNoCompression<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut reader: Option<&'a1 mut crate::src::lodepng::LodePNGBitReader>,
    mut settings: * const crate::src::lodepng::LodePNGDecompressSettings,
) -> std::os::raw::c_uint {
    let mut bytepos: u64 = 0;
    let mut size = (*(borrow_mut(&mut reader)).unwrap()).size;
    let mut LEN: u32 = 0;
    let mut NLEN: u32 = 0;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    bytepos = ((*(borrow(& reader)).unwrap()).bp).wrapping_add(7 as std::os::raw::c_uint as std::os::raw::c_ulong)
        >> 3 as std::os::raw::c_uint;
    if bytepos.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) >= size {
        return 52 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    LEN = (*((*(borrow(& reader)).unwrap()).data).offset(bytepos as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (*((*(borrow(& reader)).unwrap()).data)
                .offset(bytepos.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 8 as std::os::raw::c_uint,
        );
    bytepos = (bytepos as std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    NLEN = (*((*(borrow(& reader)).unwrap()).data).offset(bytepos as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (*((*(borrow(& reader)).unwrap()).data)
                .offset(bytepos.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 8 as std::os::raw::c_uint,
        );
    bytepos = (bytepos as std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    if (*settings).ignore_nlen == 0
        && LEN.wrapping_add(NLEN) != 65535 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return 21 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if ucvector_resize(out, ((*out).size).wrapping_add(LEN as std::os::raw::c_ulong)) == 0 {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if bytepos.wrapping_add(LEN as std::os::raw::c_ulong) > size {
        return 23 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if LEN != 0 {
        lodepng_memcpy(
            ((*out).data).offset((*out).size as isize).offset(-(LEN as isize))
                as *mut std::os::raw::c_void,
            ((*(borrow(& reader)).unwrap()).data).offset(bytepos as isize) as *const std::os::raw::c_void,
            LEN as size_t,
        );
        bytepos = (bytepos as std::os::raw::c_ulong).wrapping_add(LEN as std::os::raw::c_ulong) as size_t
            as size_t;
    }
    (*(borrow_mut(&mut reader)).unwrap()).bp = bytepos << 3 as std::os::raw::c_uint;
    return error;
}
unsafe extern "C" fn lodepng_inflatev(
    mut out: * mut crate::src::lodepng::ucvector,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGDecompressSettings,
) -> std::os::raw::c_uint {
    let mut BFINAL = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut reader = LodePNGBitReader {
        data: 0 as *const std::os::raw::c_uchar,
        size: 0,
        bitsize: 0,
        bp: 0,
        buffer: 0,
    };
    let mut error = LodePNGBitReader_init(Some(&mut reader), in_0, insize);
    if error != 0 {
        return error;
    }
    while BFINAL == 0 {
        let mut BTYPE: u32 = 0;
        if (reader.bitsize).wrapping_sub(reader.bp) < 3 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 52 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        ensureBits9(Some(&mut reader), 3 as std::os::raw::c_int as size_t);
        BFINAL = readBits(Some(&mut reader), 1 as std::os::raw::c_int as size_t);
        BTYPE = readBits(Some(&mut reader), 2 as std::os::raw::c_int as size_t);
        if BTYPE == 3 as std::os::raw::c_int as std::os::raw::c_uint {
            return 20 as std::os::raw::c_int as std::os::raw::c_uint
        } else {
            if BTYPE == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                error = inflateNoCompression(out, Some(&mut reader), settings);
            } else {
                error = inflateHuffmanBlock(
                    out,
                    Some(&mut reader),
                    BTYPE,
                    (*settings).max_output_size,
                );
            }
        }
        if error == 0 && (*settings).max_output_size != 0
            && (*out).size > (*settings).max_output_size
        {
            error = 109 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if error != 0 {
            break;
        }
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_inflate<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGDecompressSettings,
) -> std::os::raw::c_uint {
    let mut v = ucvector_init(*(borrow_mut(&mut out)).unwrap(), *(borrow_mut(&mut outsize)).unwrap());
    let mut error = lodepng_inflatev(&mut v, in_0, insize, settings);
    *(borrow_mut(&mut out)).unwrap() = v.data;
    *(borrow_mut(&mut outsize)).unwrap() = v.size;
    return error;
}
unsafe extern "C" fn inflatev(
    mut out: * mut crate::src::lodepng::ucvector,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGDecompressSettings,
) -> std::os::raw::c_uint {
    if ((*settings).custom_inflate).is_some() {
        let mut error = ((*settings).custom_inflate)
            .expect(
                "non-null function pointer",
            )(&mut (*out).data, &mut (*out).size, in_0, insize, settings);
        (*out).allocsize = (*out).size;
        if error != 0 {
            error = 110 as std::os::raw::c_int as std::os::raw::c_uint;
            if (*settings).max_output_size != 0
                && (*out).size > (*settings).max_output_size
            {
                error = 109 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
        return error;
    } else {
        return lodepng_inflatev(out, in_0, insize, settings)
    };
}
static mut MAX_SUPPORTED_DEFLATE_LENGTH: std::os::raw::c_ulong = 0; unsafe fn laertes_init_MAX_SUPPORTED_DEFLATE_LENGTH() {
MAX_SUPPORTED_DEFLATE_LENGTH = 258 as std::os::raw::c_int as size_t;}//;
unsafe extern "C" fn searchCodeIndex(
    mut array: * const std::os::raw::c_uint,
    mut array_size: std::os::raw::c_ulong,
    mut value: std::os::raw::c_ulong,
) -> std::os::raw::c_ulong {
    let mut left = 1 as std::os::raw::c_int as size_t;
    let mut right = array_size.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    while left <= right {
        let mut mid = left.wrapping_add(right) >> 1 as std::os::raw::c_int;
        if *array.offset(mid as isize) as std::os::raw::c_ulong >= value {
            right = mid.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
        } else {
            left = mid.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
        }
    }
    if left >= array_size || *array.offset(left as isize) as std::os::raw::c_ulong > value {
        left = left.wrapping_sub(1);
    }
    return left;
}
unsafe extern "C" fn addLengthDistance(
    mut values: * mut crate::src::lodepng::uivector,
    mut length: std::os::raw::c_ulong,
    mut distance: std::os::raw::c_ulong,
) {
    let mut length_code = searchCodeIndex(
        LENGTHBASE.as_ptr(),
        29 as std::os::raw::c_int as size_t,
        length,
    ) as std::os::raw::c_uint;
    let mut extra_length = length
        .wrapping_sub(LENGTHBASE[length_code as usize] as std::os::raw::c_ulong) as std::os::raw::c_uint;
    let mut dist_code = searchCodeIndex(
        DISTANCEBASE.as_ptr(),
        30 as std::os::raw::c_int as size_t,
        distance,
    ) as std::os::raw::c_uint;
    let mut extra_distance = distance
        .wrapping_sub(DISTANCEBASE[dist_code as usize] as std::os::raw::c_ulong) as std::os::raw::c_uint;
    let mut pos = (*values).size;
    let mut ok = uivector_resize(
        values,
        ((*values).size).wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    if ok != 0 {
        *((*values).data)
            .offset(
                pos.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = length_code.wrapping_add(257 as std::os::raw::c_int as std::os::raw::c_uint);
        *((*values).data)
            .offset(
                pos.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = extra_length;
        *((*values).data)
            .offset(
                pos.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = dist_code;
        *((*values).data)
            .offset(
                pos.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = extra_distance;
    }
}
static mut HASH_NUM_VALUES: std::os::raw::c_uint = 0; unsafe fn laertes_init_HASH_NUM_VALUES() {
HASH_NUM_VALUES = 65536 as std::os::raw::c_int as std::os::raw::c_uint;}//;
static mut HASH_BIT_MASK: std::os::raw::c_uint = 0; unsafe fn laertes_init_HASH_BIT_MASK() {
HASH_BIT_MASK = 65535 as std::os::raw::c_int as std::os::raw::c_uint;}//;
unsafe extern "C" fn hash_init<'a1>(
    mut hash: Option<&'a1 mut crate::src::lodepng::Hash>,
    mut windowsize: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    let ref mut fresh53 = (*(borrow_mut(&mut hash)).unwrap()).head;
    *fresh53 = lodepng_malloc(
        (::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(HASH_NUM_VALUES as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let ref mut fresh54 = (*(borrow_mut(&mut hash)).unwrap()).val;
    *fresh54 = lodepng_malloc(
        (::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(windowsize as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    let ref mut fresh55 = (*(borrow_mut(&mut hash)).unwrap()).chain;
    *fresh55 = lodepng_malloc(
        (::std::mem::size_of::<std::os::raw::c_ushort>() as std::os::raw::c_ulong)
            .wrapping_mul(windowsize as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_ushort;
    let ref mut fresh56 = (*(borrow_mut(&mut hash)).unwrap()).zeros;
    *fresh56 = lodepng_malloc(
        (::std::mem::size_of::<std::os::raw::c_ushort>() as std::os::raw::c_ulong)
            .wrapping_mul(windowsize as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_ushort;
    let ref mut fresh57 = (*(borrow_mut(&mut hash)).unwrap()).headz;
    *fresh57 = lodepng_malloc(
        (::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(
                MAX_SUPPORTED_DEFLATE_LENGTH
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ),
    ) as *mut std::os::raw::c_int;
    let ref mut fresh58 = (*(borrow_mut(&mut hash)).unwrap()).chainz;
    *fresh58 = lodepng_malloc(
        (::std::mem::size_of::<std::os::raw::c_ushort>() as std::os::raw::c_ulong)
            .wrapping_mul(windowsize as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_ushort;
    if ((*(borrow_mut(&mut hash)).unwrap()).head).is_null() || ((*(borrow_mut(&mut hash)).unwrap()).chain).is_null() || ((*(borrow_mut(&mut hash)).unwrap()).val).is_null()
        || ((*(borrow_mut(&mut hash)).unwrap()).headz).is_null() || ((*(borrow_mut(&mut hash)).unwrap()).chainz).is_null()
        || ((*(borrow_mut(&mut hash)).unwrap()).zeros).is_null()
    {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != HASH_NUM_VALUES {
        *((*(borrow(& hash)).unwrap()).head).offset(i as isize) = -(1 as std::os::raw::c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != windowsize {
        *((*(borrow(& hash)).unwrap()).val).offset(i as isize) = -(1 as std::os::raw::c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != windowsize {
        *((*(borrow(& hash)).unwrap()).chain).offset(i as isize) = i as std::os::raw::c_ushort;
        i = i.wrapping_add(1);
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i as std::os::raw::c_ulong <= MAX_SUPPORTED_DEFLATE_LENGTH {
        *((*(borrow(& hash)).unwrap()).headz).offset(i as isize) = -(1 as std::os::raw::c_int);
        i = i.wrapping_add(1);
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != windowsize {
        *((*(borrow(& hash)).unwrap()).chainz).offset(i as isize) = i as std::os::raw::c_ushort;
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn hash_cleanup<'a1>(mut hash: Option<&'a1 mut crate::src::lodepng::Hash>) {
    lodepng_free((*(borrow_mut(&mut hash)).unwrap()).head as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut hash)).unwrap()).val as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut hash)).unwrap()).chain as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut hash)).unwrap()).zeros as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut hash)).unwrap()).headz as *mut std::os::raw::c_void);
    lodepng_free((*(borrow_mut(&mut hash)).unwrap()).chainz as *mut std::os::raw::c_void);
}
unsafe extern "C" fn getHash(
    mut data: * const std::os::raw::c_uchar,
    mut size: std::os::raw::c_ulong,
    mut pos: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut result = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if pos.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) < size {
        result
            ^= (*data
                .offset(pos.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 0 as std::os::raw::c_uint;
        result
            ^= (*data
                .offset(pos.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 4 as std::os::raw::c_uint;
        result
            ^= (*data
                .offset(pos.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_uint) << 8 as std::os::raw::c_uint;
    } else {
        let mut amount: u64 = 0;
        let mut i: u64 = 0;
        if pos >= size {
            return 0 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        amount = size.wrapping_sub(pos);
        i = 0 as std::os::raw::c_int as size_t;
        while i != amount {
            result
                ^= (*data.offset(pos.wrapping_add(i) as isize) as std::os::raw::c_uint)
                    << i.wrapping_mul(8 as std::os::raw::c_uint as std::os::raw::c_ulong);
            i = i.wrapping_add(1);
        }
    }
    return result & HASH_BIT_MASK;
}
unsafe extern "C" fn countZeros(
    mut data: * const std::os::raw::c_uchar,
    mut size: std::os::raw::c_ulong,
    mut pos: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut start = data.offset(pos as isize);
    let mut end = start.offset(MAX_SUPPORTED_DEFLATE_LENGTH as isize);
    if end > data.offset(size as isize) {
        end = data.offset(size as isize);
    }
    data = start;
    while data != end && *data as std::os::raw::c_int == 0 as std::os::raw::c_int {
        data = data.offset(1);
    }
    return data.offset_from(start) as std::os::raw::c_long as std::os::raw::c_uint;
}
unsafe extern "C" fn updateHashChain<'a1>(
    mut hash: Option<&'a1 mut crate::src::lodepng::Hash>,
    mut wpos: std::os::raw::c_ulong,
    mut hashval: std::os::raw::c_uint,
    mut numzeros: std::os::raw::c_ushort,
) {
    *((*(borrow(& hash)).unwrap()).val).offset(wpos as isize) = hashval as std::os::raw::c_int;
    if *((*(borrow(& hash)).unwrap()).head).offset(hashval as isize) != -(1 as std::os::raw::c_int) {
        *((*(borrow(& hash)).unwrap()).chain)
            .offset(
                wpos as isize,
            ) = *((*(borrow(& hash)).unwrap()).head).offset(hashval as isize) as std::os::raw::c_ushort;
    }
    *((*(borrow(& hash)).unwrap()).head).offset(hashval as isize) = wpos as std::os::raw::c_int;
    *((*(borrow(& hash)).unwrap()).zeros).offset(wpos as isize) = numzeros;
    if *((*(borrow(& hash)).unwrap()).headz).offset(numzeros as isize) != -(1 as std::os::raw::c_int) {
        *((*(borrow(& hash)).unwrap()).chainz)
            .offset(
                wpos as isize,
            ) = *((*(borrow(& hash)).unwrap()).headz).offset(numzeros as isize) as std::os::raw::c_ushort;
    }
    *((*(borrow(& hash)).unwrap()).headz).offset(numzeros as isize) = wpos as std::os::raw::c_int;
}
unsafe extern "C" fn encodeLZ77<'a1>(
    mut out: * mut crate::src::lodepng::uivector,
    mut hash: Option<&'a1 mut crate::src::lodepng::Hash>,
    mut in_0: * const std::os::raw::c_uchar,
    mut inpos: std::os::raw::c_ulong,
    mut insize: std::os::raw::c_ulong,
    mut windowsize: std::os::raw::c_uint,
    mut minmatch: std::os::raw::c_uint,
    mut nicematch: std::os::raw::c_uint,
    mut lazymatching: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut pos: u64 = 0;
    let mut i: u32 = 0;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut maxchainlength = if windowsize >= 8192 as std::os::raw::c_int as std::os::raw::c_uint {
        windowsize
    } else {
        windowsize.wrapping_div(8 as std::os::raw::c_uint)
    };
    let mut maxlazymatch = (if windowsize >= 8192 as std::os::raw::c_int as std::os::raw::c_uint {
        MAX_SUPPORTED_DEFLATE_LENGTH
    } else {
        64 as std::os::raw::c_int as std::os::raw::c_ulong
    }) as std::os::raw::c_uint;
    let mut usezeros = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut numzeros = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut offset: u32 = 0;
    let mut length: u32 = 0;
    let mut lazy = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut lazylength = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut lazyoffset = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut hashval: u32 = 0;
    let mut current_offset: u32 = 0;
    let mut current_length: u32 = 0;
    let mut prev_offset: u32 = 0;
    let mut lastptr = 0 as *const std::os::raw::c_uchar;
    let mut foreptr = 0 as *const std::os::raw::c_uchar;
    let mut backptr = 0 as *const std::os::raw::c_uchar;
    let mut hashpos: u32 = 0;
    if windowsize == 0 as std::os::raw::c_int as std::os::raw::c_uint
        || windowsize > 32768 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return 60 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if windowsize & windowsize.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
        != 0 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return 90 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if nicematch as std::os::raw::c_ulong > MAX_SUPPORTED_DEFLATE_LENGTH {
        nicematch = MAX_SUPPORTED_DEFLATE_LENGTH as std::os::raw::c_uint;
    }
    let mut current_block_78: u64;
    pos = inpos;
    while pos < insize {
        let mut wpos = pos
            & windowsize.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong;
        let mut chainlength = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        hashval = getHash(in_0, insize, pos);
        if usezeros != 0 && hashval == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if numzeros == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                numzeros = countZeros(in_0, insize, pos);
            } else if pos.wrapping_add(numzeros as std::os::raw::c_ulong) > insize
                || *in_0
                    .offset(
                        pos
                            .wrapping_add(numzeros as std::os::raw::c_ulong)
                            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_int != 0 as std::os::raw::c_int
            {
                numzeros = numzeros.wrapping_sub(1);
            }
        } else {
            numzeros = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        updateHashChain(borrow_mut(&mut hash), wpos, hashval, numzeros as std::os::raw::c_ushort);
        length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        offset = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        hashpos = *((*(borrow(& hash)).unwrap()).chain).offset(wpos as isize) as std::os::raw::c_uint;
        lastptr = &*in_0
            .offset(
                (if insize < pos.wrapping_add(MAX_SUPPORTED_DEFLATE_LENGTH) {
                    insize
                } else {
                    pos.wrapping_add(MAX_SUPPORTED_DEFLATE_LENGTH)
                }) as isize,
            ) as *const std::os::raw::c_uchar;
        prev_offset = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        loop {
            let mut fresh59 = chainlength;
            chainlength = chainlength.wrapping_add(1);
            if fresh59 >= maxchainlength {
                break;
            }
            current_offset = (if hashpos as std::os::raw::c_ulong <= wpos {
                wpos.wrapping_sub(hashpos as std::os::raw::c_ulong)
            } else {
                wpos.wrapping_sub(hashpos as std::os::raw::c_ulong)
                    .wrapping_add(windowsize as std::os::raw::c_ulong)
            }) as std::os::raw::c_uint;
            if current_offset < prev_offset {
                break;
            }
            prev_offset = current_offset;
            if current_offset > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                foreptr = &*in_0.offset(pos as isize) as *const std::os::raw::c_uchar;
                backptr = &*in_0
                    .offset(pos.wrapping_sub(current_offset as std::os::raw::c_ulong) as isize)
                    as *const std::os::raw::c_uchar;
                if numzeros >= 3 as std::os::raw::c_int as std::os::raw::c_uint {
                    let mut skip = *((*(borrow(& hash)).unwrap()).zeros).offset(hashpos as isize)
                        as std::os::raw::c_uint;
                    if skip > numzeros {
                        skip = numzeros;
                    }
                    backptr = backptr.offset(skip as isize);
                    foreptr = foreptr.offset(skip as isize);
                }
                while foreptr != lastptr
                    && *backptr as std::os::raw::c_int == *foreptr as std::os::raw::c_int
                {
                    backptr = backptr.offset(1);
                    foreptr = foreptr.offset(1);
                }
                current_length = foreptr
                    .offset_from(&*in_0.offset(pos as isize) as *const std::os::raw::c_uchar)
                    as std::os::raw::c_long as std::os::raw::c_uint;
                if current_length > length {
                    length = current_length;
                    offset = current_offset;
                    if current_length >= nicematch {
                        break;
                    }
                }
            }
            if hashpos == *((*(borrow(& hash)).unwrap()).chain).offset(hashpos as isize) as std::os::raw::c_uint {
                break;
            }
            if numzeros >= 3 as std::os::raw::c_int as std::os::raw::c_uint && length > numzeros {
                hashpos = *((*(borrow(& hash)).unwrap()).chainz).offset(hashpos as isize) as std::os::raw::c_uint;
                if *((*(borrow(& hash)).unwrap()).zeros).offset(hashpos as isize) as std::os::raw::c_uint != numzeros
                {
                    break;
                }
            } else {
                hashpos = *((*(borrow(& hash)).unwrap()).chain).offset(hashpos as isize) as std::os::raw::c_uint;
                if *((*(borrow(& hash)).unwrap()).val).offset(hashpos as isize) != hashval as std::os::raw::c_int {
                    break;
                }
            }
        }
        if lazymatching != 0 {
            if lazy == 0 && length >= 3 as std::os::raw::c_int as std::os::raw::c_uint
                && length <= maxlazymatch
                && (length as std::os::raw::c_ulong) < MAX_SUPPORTED_DEFLATE_LENGTH
            {
                lazy = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                lazylength = length;
                lazyoffset = offset;
                current_block_78 = 8236137900636309791;
            } else if lazy != 0 {
                lazy = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                if pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    error = 81 as std::os::raw::c_int as std::os::raw::c_uint;
                    break;
                } else if length
                    > lazylength.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                {
                    if uivector_push_back(
                        out,
                        *in_0
                            .offset(
                                pos.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    ) == 0
                    {
                        error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    }
                } else {
                    length = lazylength;
                    offset = lazyoffset;
                    *((*(borrow(& hash)).unwrap()).head).offset(hashval as isize) = -(1 as std::os::raw::c_int);
                    *((*(borrow(& hash)).unwrap()).headz).offset(numzeros as isize) = -(1 as std::os::raw::c_int);
                    pos = pos.wrapping_sub(1);
                }
                current_block_78 = 8716029205547827362;
            } else {
                current_block_78 = 8716029205547827362;
            }
        } else {
            current_block_78 = 8716029205547827362;
        }
        match current_block_78 {
            8716029205547827362 => {
                if length >= 3 as std::os::raw::c_int as std::os::raw::c_uint && offset > windowsize {
                    error = 86 as std::os::raw::c_int as std::os::raw::c_uint;
                    break;
                } else if length < 3 as std::os::raw::c_int as std::os::raw::c_uint {
                    if uivector_push_back(
                        out,
                        *in_0.offset(pos as isize) as std::os::raw::c_uint,
                    ) == 0
                    {
                        error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    }
                } else if length < minmatch
                    || length == 3 as std::os::raw::c_int as std::os::raw::c_uint
                        && offset > 4096 as std::os::raw::c_int as std::os::raw::c_uint
                {
                    if uivector_push_back(
                        out,
                        *in_0.offset(pos as isize) as std::os::raw::c_uint,
                    ) == 0
                    {
                        error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    }
                } else {
                    addLengthDistance(out, length as size_t, offset as size_t);
                    i = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                    while i < length {
                        pos = pos.wrapping_add(1);
                        wpos = pos
                            & windowsize.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                as std::os::raw::c_ulong;
                        hashval = getHash(in_0, insize, pos);
                        if usezeros != 0 && hashval == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            if numzeros == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                numzeros = countZeros(in_0, insize, pos);
                            } else if pos.wrapping_add(numzeros as std::os::raw::c_ulong)
                                > insize
                                || *in_0
                                    .offset(
                                        pos
                                            .wrapping_add(numzeros as std::os::raw::c_ulong)
                                            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                    ) as std::os::raw::c_int != 0 as std::os::raw::c_int
                            {
                                numzeros = numzeros.wrapping_sub(1);
                            }
                        } else {
                            numzeros = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        }
                        updateHashChain(borrow_mut(&mut hash), wpos, hashval, numzeros as std::os::raw::c_ushort);
                        i = i.wrapping_add(1);
                    }
                }
            }
            _ => {}
        }
        pos = pos.wrapping_add(1);
    }
    return error;
}
unsafe extern "C" fn deflateNoCompression(
    mut out: * mut crate::src::lodepng::ucvector,
    mut data: * const std::os::raw::c_uchar,
    mut datasize: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut i: u64 = 0;
    let mut numdeflateblocks = datasize
        .wrapping_add(65534 as std::os::raw::c_uint as std::os::raw::c_ulong)
        .wrapping_div(65535 as std::os::raw::c_uint as std::os::raw::c_ulong);
    let mut datapos = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    i = 0 as std::os::raw::c_int as size_t;
    while i != numdeflateblocks {
        let mut BFINAL: u32 = 0;
        let mut BTYPE: u32 = 0;
        let mut LEN: u32 = 0;
        let mut NLEN: u32 = 0;
        let mut firstbyte: u8 = 0;
        let mut pos = (*out).size;
        BFINAL = (i == numdeflateblocks.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong))
            as std::os::raw::c_int as std::os::raw::c_uint;
        BTYPE = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        LEN = 65535 as std::os::raw::c_int as std::os::raw::c_uint;
        if datasize.wrapping_sub(datapos as std::os::raw::c_ulong)
            < 65535 as std::os::raw::c_uint as std::os::raw::c_ulong
        {
            LEN = (datasize as std::os::raw::c_uint).wrapping_sub(datapos);
        }
        NLEN = (65535 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_sub(LEN);
        if ucvector_resize(
            out,
            ((*out).size)
                .wrapping_add(LEN as std::os::raw::c_ulong)
                .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong),
        ) == 0
        {
            return 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        firstbyte = BFINAL
            .wrapping_add((BTYPE & 1 as std::os::raw::c_uint) << 1 as std::os::raw::c_uint)
            .wrapping_add((BTYPE & 2 as std::os::raw::c_uint) << 1 as std::os::raw::c_uint)
            as std::os::raw::c_uchar;
        *((*out).data)
            .offset(
                pos.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = firstbyte;
        *((*out).data)
            .offset(
                pos.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (LEN & 255 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
        *((*out).data)
            .offset(
                pos.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (LEN >> 8 as std::os::raw::c_uint) as std::os::raw::c_uchar;
        *((*out).data)
            .offset(
                pos.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (NLEN & 255 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
        *((*out).data)
            .offset(
                pos.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (NLEN >> 8 as std::os::raw::c_uint) as std::os::raw::c_uchar;
        lodepng_memcpy(
            ((*out).data).offset(pos as isize).offset(5 as std::os::raw::c_int as isize)
                as *mut std::os::raw::c_void,
            data.offset(datapos as isize) as *const std::os::raw::c_void,
            LEN as size_t,
        );
        datapos = datapos.wrapping_add(LEN);
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn writeLZ77data<'a1, 'a2, 'a3, 'a4>(
    mut writer: Option<&'a1 mut crate::src::lodepng::LodePNGBitWriter>,
    mut lz77_encoded: Option<&'a2 crate::src::lodepng::uivector>,
    mut tree_ll: Option<&'a3 crate::src::lodepng::HuffmanTree>,
    mut tree_d: Option<&'a4 crate::src::lodepng::HuffmanTree>,
) {
    let mut i = 0 as std::os::raw::c_int as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i != (*((lz77_encoded).clone()).unwrap()).size {
        let mut val = *((*((lz77_encoded).clone()).unwrap()).data).offset(i as isize);
        writeBitsReversed(
            borrow_mut(&mut writer),
            *((*((tree_ll).clone()).unwrap()).codes).offset(val as isize),
            *((*((tree_ll).clone()).unwrap()).lengths).offset(val as isize) as size_t,
        );
        if val > 256 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut length_index = val.wrapping_sub(257 as std::os::raw::c_int as std::os::raw::c_uint);
            let mut n_length_extra_bits = LENGTHEXTRA[length_index as usize];
            i = i.wrapping_add(1);
            let mut length_extra_bits = *((*((lz77_encoded).clone()).unwrap()).data).offset(i as isize);
            i = i.wrapping_add(1);
            let mut distance_code = *((*((lz77_encoded).clone()).unwrap()).data).offset(i as isize);
            let mut distance_index = distance_code;
            let mut n_distance_extra_bits = DISTANCEEXTRA[distance_index as usize];
            i = i.wrapping_add(1);
            let mut distance_extra_bits = *((*((lz77_encoded).clone()).unwrap()).data).offset(i as isize);
            writeBits(borrow_mut(&mut writer), length_extra_bits, n_length_extra_bits as size_t);
            writeBitsReversed(
                borrow_mut(&mut writer),
                *((*((tree_d).clone()).unwrap()).codes).offset(distance_code as isize),
                *((*((tree_d).clone()).unwrap()).lengths).offset(distance_code as isize) as size_t,
            );
            writeBits(borrow_mut(&mut writer), distance_extra_bits, n_distance_extra_bits as size_t);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn deflateDynamic<'a1, 'a2>(
    mut writer: Option<&'a1 mut crate::src::lodepng::LodePNGBitWriter>,
    mut hash: Option<&'a2 mut crate::src::lodepng::Hash>,
    mut data: * const std::os::raw::c_uchar,
    mut datapos: std::os::raw::c_ulong,
    mut dataend: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGCompressSettings,
    mut final_0: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut lz77_encoded = uivector {
        data: 0 as *mut std::os::raw::c_uint,
        size: 0,
        allocsize: 0,
    };
    let mut tree_ll = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    let mut tree_d = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    let mut tree_cl = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    let mut frequencies_ll = 0 as *mut std::os::raw::c_uint;
    let mut frequencies_d = 0 as *mut std::os::raw::c_uint;
    let mut frequencies_cl = 0 as *mut std::os::raw::c_uint;
    let mut bitlen_lld = 0 as *mut std::os::raw::c_uint;
    let mut bitlen_lld_e = 0 as *mut std::os::raw::c_uint;
    let mut datasize = dataend.wrapping_sub(datapos);
    let mut BFINAL = final_0;
    let mut i: u64 = 0;
    let mut numcodes_ll: u64 = 0;
    let mut numcodes_d: u64 = 0;
    let mut numcodes_lld: u64 = 0;
    let mut numcodes_lld_e: u64 = 0;
    let mut numcodes_cl: u64 = 0;
    let mut HLIT: u32 = 0;
    let mut HDIST: u32 = 0;
    let mut HCLEN: u32 = 0;
    uivector_init(Some(&mut lz77_encoded));
    HuffmanTree_init(Some(&mut tree_ll));
    HuffmanTree_init(Some(&mut tree_d));
    HuffmanTree_init(Some(&mut tree_cl));
    frequencies_ll = lodepng_malloc(
        (286 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    frequencies_d = lodepng_malloc(
        (30 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    frequencies_cl = lodepng_malloc(
        (19 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_uint;
    if frequencies_ll.is_null() || frequencies_d.is_null() || frequencies_cl.is_null() {
        error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let mut current_block_113: u64;
    if error == 0 {
        lodepng_memset(
            frequencies_ll as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            (286 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
        );
        lodepng_memset(
            frequencies_d as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            (30 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
        );
        lodepng_memset(
            frequencies_cl as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            (19 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong),
        );
        if (*settings).use_lz77 != 0 {
            error = encodeLZ77(
                &mut lz77_encoded,
                borrow_mut(&mut hash),
                data,
                datapos,
                dataend,
                (*settings).windowsize,
                (*settings).minmatch,
                (*settings).nicematch,
                (*settings).lazymatching,
            );
            if error != 0 {
                current_block_113 = 6988365858197790817;
            } else {
                current_block_113 = 17788412896529399552;
            }
        } else if uivector_resize(&mut lz77_encoded, datasize) == 0 {
            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
            current_block_113 = 6988365858197790817;
        } else {
            i = datapos;
            while i < dataend {
                *(lz77_encoded.data)
                    .offset(
                        i.wrapping_sub(datapos) as isize,
                    ) = *data.offset(i as isize) as std::os::raw::c_uint;
                i = i.wrapping_add(1);
            }
            current_block_113 = 17788412896529399552;
        }
        match current_block_113 {
            6988365858197790817 => {}
            _ => {
                i = 0 as std::os::raw::c_int as size_t;
                while i != lz77_encoded.size {
                    let mut symbol = *(lz77_encoded.data).offset(i as isize);
                    let ref mut fresh60 = *frequencies_ll.offset(symbol as isize);
                    *fresh60 = (*fresh60).wrapping_add(1);
                    if symbol > 256 as std::os::raw::c_int as std::os::raw::c_uint {
                        let mut dist = *(lz77_encoded.data)
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let ref mut fresh61 = *frequencies_d.offset(dist as isize);
                        *fresh61 = (*fresh61).wrapping_add(1);
                        i = (i as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                    i = i.wrapping_add(1);
                }
                *frequencies_ll
                    .offset(
                        256 as std::os::raw::c_int as isize,
                    ) = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                error = HuffmanTree_makeFromFrequencies(
                    &mut tree_ll,
                    frequencies_ll,
                    257 as std::os::raw::c_int as size_t,
                    286 as std::os::raw::c_int as size_t,
                    15 as std::os::raw::c_int as std::os::raw::c_uint,
                );
                if !(error != 0) {
                    error = HuffmanTree_makeFromFrequencies(
                        &mut tree_d,
                        frequencies_d,
                        2 as std::os::raw::c_int as size_t,
                        30 as std::os::raw::c_int as size_t,
                        15 as std::os::raw::c_int as std::os::raw::c_uint,
                    );
                    if !(error != 0) {
                        numcodes_ll = (if tree_ll.numcodes
                            < 286 as std::os::raw::c_int as std::os::raw::c_uint
                        {
                            tree_ll.numcodes
                        } else {
                            286 as std::os::raw::c_int as std::os::raw::c_uint
                        }) as size_t;
                        numcodes_d = (if tree_d.numcodes
                            < 30 as std::os::raw::c_int as std::os::raw::c_uint
                        {
                            tree_d.numcodes
                        } else {
                            30 as std::os::raw::c_int as std::os::raw::c_uint
                        }) as size_t;
                        numcodes_lld = numcodes_ll.wrapping_add(numcodes_d);
                        bitlen_lld = lodepng_malloc(
                            numcodes_lld
                                .wrapping_mul(
                                    ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong,
                                ),
                        ) as *mut std::os::raw::c_uint;
                        bitlen_lld_e = lodepng_malloc(
                            numcodes_lld
                                .wrapping_mul(
                                    ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong,
                                ),
                        ) as *mut std::os::raw::c_uint;
                        if bitlen_lld.is_null() || bitlen_lld_e.is_null() {
                            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                        } else {
                            numcodes_lld_e = 0 as std::os::raw::c_int as size_t;
                            i = 0 as std::os::raw::c_int as size_t;
                            while i != numcodes_ll {
                                *bitlen_lld
                                    .offset(i as isize) = *(tree_ll.lengths).offset(i as isize);
                                i = i.wrapping_add(1);
                            }
                            i = 0 as std::os::raw::c_int as size_t;
                            while i != numcodes_d {
                                *bitlen_lld
                                    .offset(
                                        numcodes_ll.wrapping_add(i) as isize,
                                    ) = *(tree_d.lengths).offset(i as isize);
                                i = i.wrapping_add(1);
                            }
                            i = 0 as std::os::raw::c_int as size_t;
                            while i != numcodes_lld {
                                let mut j = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                while i
                                    .wrapping_add(j as std::os::raw::c_ulong)
                                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    < numcodes_lld
                                    && *bitlen_lld
                                        .offset(
                                            i
                                                .wrapping_add(j as std::os::raw::c_ulong)
                                                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                        ) == *bitlen_lld.offset(i as isize)
                                {
                                    j = j.wrapping_add(1);
                                }
                                if *bitlen_lld.offset(i as isize)
                                    == 0 as std::os::raw::c_int as std::os::raw::c_uint
                                    && j >= 2 as std::os::raw::c_int as std::os::raw::c_uint
                                {
                                    j = j.wrapping_add(1);
                                    if j <= 10 as std::os::raw::c_int as std::os::raw::c_uint {
                                        let mut fresh62 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh62 as isize,
                                            ) = 17 as std::os::raw::c_int as std::os::raw::c_uint;
                                        let mut fresh63 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh63 as isize,
                                            ) = j.wrapping_sub(3 as std::os::raw::c_int as std::os::raw::c_uint);
                                    } else {
                                        if j > 138 as std::os::raw::c_int as std::os::raw::c_uint {
                                            j = 138 as std::os::raw::c_int as std::os::raw::c_uint;
                                        }
                                        let mut fresh64 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh64 as isize,
                                            ) = 18 as std::os::raw::c_int as std::os::raw::c_uint;
                                        let mut fresh65 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh65 as isize,
                                            ) = j.wrapping_sub(11 as std::os::raw::c_int as std::os::raw::c_uint);
                                    }
                                    i = (i as std::os::raw::c_ulong)
                                        .wrapping_add(
                                            j.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                                as std::os::raw::c_ulong,
                                        ) as size_t as size_t;
                                } else if j >= 3 as std::os::raw::c_int as std::os::raw::c_uint {
                                    let mut k: u64 = 0;
                                    let mut num = j.wrapping_div(6 as std::os::raw::c_uint);
                                    let mut rest = j.wrapping_rem(6 as std::os::raw::c_uint);
                                    let mut fresh66 = numcodes_lld_e;
                                    numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                    *bitlen_lld_e
                                        .offset(fresh66 as isize) = *bitlen_lld.offset(i as isize);
                                    k = 0 as std::os::raw::c_int as size_t;
                                    while k < num as std::os::raw::c_ulong {
                                        let mut fresh67 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh67 as isize,
                                            ) = 16 as std::os::raw::c_int as std::os::raw::c_uint;
                                        let mut fresh68 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh68 as isize,
                                            ) = (6 as std::os::raw::c_int - 3 as std::os::raw::c_int) as std::os::raw::c_uint;
                                        k = k.wrapping_add(1);
                                    }
                                    if rest >= 3 as std::os::raw::c_int as std::os::raw::c_uint {
                                        let mut fresh69 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh69 as isize,
                                            ) = 16 as std::os::raw::c_int as std::os::raw::c_uint;
                                        let mut fresh70 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e
                                            .offset(
                                                fresh70 as isize,
                                            ) = rest.wrapping_sub(3 as std::os::raw::c_int as std::os::raw::c_uint);
                                    } else {
                                        j = j.wrapping_sub(rest);
                                    }
                                    i = (i as std::os::raw::c_ulong).wrapping_add(j as std::os::raw::c_ulong)
                                        as size_t as size_t;
                                } else {
                                    let mut fresh71 = numcodes_lld_e;
                                    numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                    *bitlen_lld_e
                                        .offset(fresh71 as isize) = *bitlen_lld.offset(i as isize);
                                }
                                i = i.wrapping_add(1);
                            }
                            i = 0 as std::os::raw::c_int as size_t;
                            while i != numcodes_lld_e {
                                let ref mut fresh72 = *frequencies_cl
                                    .offset(*bitlen_lld_e.offset(i as isize) as isize);
                                *fresh72 = (*fresh72).wrapping_add(1);
                                if *bitlen_lld_e.offset(i as isize)
                                    >= 16 as std::os::raw::c_int as std::os::raw::c_uint
                                {
                                    i = i.wrapping_add(1);
                                }
                                i = i.wrapping_add(1);
                            }
                            error = HuffmanTree_makeFromFrequencies(
                                &mut tree_cl,
                                frequencies_cl,
                                19 as std::os::raw::c_int as size_t,
                                19 as std::os::raw::c_int as size_t,
                                7 as std::os::raw::c_int as std::os::raw::c_uint,
                            );
                            if !(error != 0) {
                                numcodes_cl = 19 as std::os::raw::c_int as size_t;
                                while numcodes_cl > 4 as std::os::raw::c_uint as std::os::raw::c_ulong
                                    && *(tree_cl.lengths)
                                        .offset(
                                            CLCL_ORDER[numcodes_cl
                                                .wrapping_sub(1 as std::os::raw::c_uint as std::os::raw::c_ulong) as usize]
                                                as isize,
                                        ) == 0 as std::os::raw::c_int as std::os::raw::c_uint
                                {
                                    numcodes_cl = numcodes_cl.wrapping_sub(1);
                                }
                                writeBits(borrow_mut(&mut writer), BFINAL, 1 as std::os::raw::c_int as size_t);
                                writeBits(
                                    borrow_mut(&mut writer),
                                    0 as std::os::raw::c_int as std::os::raw::c_uint,
                                    1 as std::os::raw::c_int as size_t,
                                );
                                writeBits(
                                    borrow_mut(&mut writer),
                                    1 as std::os::raw::c_int as std::os::raw::c_uint,
                                    1 as std::os::raw::c_int as size_t,
                                );
                                HLIT = numcodes_ll
                                    .wrapping_sub(257 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as std::os::raw::c_uint;
                                HDIST = numcodes_d
                                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as std::os::raw::c_uint;
                                HCLEN = numcodes_cl
                                    .wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as std::os::raw::c_uint;
                                writeBits(borrow_mut(&mut writer), HLIT, 5 as std::os::raw::c_int as size_t);
                                writeBits(borrow_mut(&mut writer), HDIST, 5 as std::os::raw::c_int as size_t);
                                writeBits(borrow_mut(&mut writer), HCLEN, 4 as std::os::raw::c_int as size_t);
                                i = 0 as std::os::raw::c_int as size_t;
                                while i != numcodes_cl {
                                    writeBits(
                                        borrow_mut(&mut writer),
                                        *(tree_cl.lengths).offset(CLCL_ORDER[i as usize] as isize),
                                        3 as std::os::raw::c_int as size_t,
                                    );
                                    i = i.wrapping_add(1);
                                }
                                i = 0 as std::os::raw::c_int as size_t;
                                while i != numcodes_lld_e {
                                    writeBitsReversed(
                                        borrow_mut(&mut writer),
                                        *(tree_cl.codes)
                                            .offset(*bitlen_lld_e.offset(i as isize) as isize),
                                        *(tree_cl.lengths)
                                            .offset(*bitlen_lld_e.offset(i as isize) as isize) as size_t,
                                    );
                                    if *bitlen_lld_e.offset(i as isize)
                                        == 16 as std::os::raw::c_int as std::os::raw::c_uint
                                    {
                                        i = i.wrapping_add(1);
                                        writeBits(
                                            borrow_mut(&mut writer),
                                            *bitlen_lld_e.offset(i as isize),
                                            2 as std::os::raw::c_int as size_t,
                                        );
                                    } else if *bitlen_lld_e.offset(i as isize)
                                        == 17 as std::os::raw::c_int as std::os::raw::c_uint
                                    {
                                        i = i.wrapping_add(1);
                                        writeBits(
                                            borrow_mut(&mut writer),
                                            *bitlen_lld_e.offset(i as isize),
                                            3 as std::os::raw::c_int as size_t,
                                        );
                                    } else if *bitlen_lld_e.offset(i as isize)
                                        == 18 as std::os::raw::c_int as std::os::raw::c_uint
                                    {
                                        i = i.wrapping_add(1);
                                        writeBits(
                                            borrow_mut(&mut writer),
                                            *bitlen_lld_e.offset(i as isize),
                                            7 as std::os::raw::c_int as size_t,
                                        );
                                    }
                                    i = i.wrapping_add(1);
                                }
                                writeLZ77data(
                                    borrow_mut(&mut writer),
                                    Some(&mut lz77_encoded),
                                    Some(&mut tree_ll),
                                    Some(&mut tree_d),
                                );
                                if *(tree_ll.lengths).offset(256 as std::os::raw::c_int as isize)
                                    == 0 as std::os::raw::c_int as std::os::raw::c_uint
                                {
                                    error = 64 as std::os::raw::c_int as std::os::raw::c_uint;
                                } else {
                                    writeBitsReversed(
                                        borrow_mut(&mut writer),
                                        *(tree_ll.codes).offset(256 as std::os::raw::c_int as isize),
                                        *(tree_ll.lengths).offset(256 as std::os::raw::c_int as isize)
                                            as size_t,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    uivector_cleanup(&mut lz77_encoded as *mut uivector as *mut std::os::raw::c_void);
    HuffmanTree_cleanup(Some(&mut tree_ll));
    HuffmanTree_cleanup(Some(&mut tree_d));
    HuffmanTree_cleanup(Some(&mut tree_cl));
    lodepng_free(frequencies_ll as *mut std::os::raw::c_void);
    lodepng_free(frequencies_d as *mut std::os::raw::c_void);
    lodepng_free(frequencies_cl as *mut std::os::raw::c_void);
    lodepng_free(bitlen_lld as *mut std::os::raw::c_void);
    lodepng_free(bitlen_lld_e as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn deflateFixed<'a1, 'a2>(
    mut writer: Option<&'a1 mut crate::src::lodepng::LodePNGBitWriter>,
    mut hash: Option<&'a2 mut crate::src::lodepng::Hash>,
    mut data: * const std::os::raw::c_uchar,
    mut datapos: std::os::raw::c_ulong,
    mut dataend: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGCompressSettings,
    mut final_0: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut tree_ll = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    let mut tree_d = HuffmanTree {
        codes: 0 as *mut std::os::raw::c_uint,
        lengths: 0 as *mut std::os::raw::c_uint,
        maxbitlen: 0,
        numcodes: 0,
        table_len: 0 as *mut std::os::raw::c_uchar,
        table_value: 0 as *mut std::os::raw::c_ushort,
    };
    let mut BFINAL = final_0;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u64 = 0;
    HuffmanTree_init(Some(&mut tree_ll));
    HuffmanTree_init(Some(&mut tree_d));
    error = generateFixedLitLenTree(&mut tree_ll);
    if error == 0 {
        error = generateFixedDistanceTree(&mut tree_d);
    }
    if error == 0 {
        writeBits(borrow_mut(&mut writer), BFINAL, 1 as std::os::raw::c_int as size_t);
        writeBits(borrow_mut(&mut writer), 1 as std::os::raw::c_int as std::os::raw::c_uint, 1 as std::os::raw::c_int as size_t);
        writeBits(borrow_mut(&mut writer), 0 as std::os::raw::c_int as std::os::raw::c_uint, 1 as std::os::raw::c_int as size_t);
        if (*settings).use_lz77 != 0 {
            let mut lz77_encoded = uivector {
                data: 0 as *mut std::os::raw::c_uint,
                size: 0,
                allocsize: 0,
            };
            uivector_init(Some(&mut lz77_encoded));
            error = encodeLZ77(
                &mut lz77_encoded,
                borrow_mut(&mut hash),
                data,
                datapos,
                dataend,
                (*settings).windowsize,
                (*settings).minmatch,
                (*settings).nicematch,
                (*settings).lazymatching,
            );
            if error == 0 {
                writeLZ77data(borrow_mut(&mut writer), Some(&mut lz77_encoded), Some(&mut tree_ll), Some(&mut tree_d));
            }
            uivector_cleanup(&mut lz77_encoded as *mut uivector as *mut std::os::raw::c_void);
        } else {
            i = datapos;
            while i < dataend {
                writeBitsReversed(
                    borrow_mut(&mut writer),
                    *(tree_ll.codes).offset(*data.offset(i as isize) as isize),
                    *(tree_ll.lengths).offset(*data.offset(i as isize) as isize)
                        as size_t,
                );
                i = i.wrapping_add(1);
            }
        }
        if error == 0 {
            writeBitsReversed(
                borrow_mut(&mut writer),
                *(tree_ll.codes).offset(256 as std::os::raw::c_int as isize),
                *(tree_ll.lengths).offset(256 as std::os::raw::c_int as isize) as size_t,
            );
        }
    }
    HuffmanTree_cleanup(Some(&mut tree_ll));
    HuffmanTree_cleanup(Some(&mut tree_d));
    return error;
}
unsafe extern "C" fn lodepng_deflatev(
    mut out: * mut crate::src::lodepng::ucvector,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u64 = 0;
    let mut blocksize: u64 = 0;
    let mut numdeflateblocks: u64 = 0;
    let mut hash = Hash {
        head: 0 as *mut std::os::raw::c_int,
        chain: 0 as *mut std::os::raw::c_ushort,
        val: 0 as *mut std::os::raw::c_int,
        headz: 0 as *mut std::os::raw::c_int,
        chainz: 0 as *mut std::os::raw::c_ushort,
        zeros: 0 as *mut std::os::raw::c_ushort,
    };
    let mut writer = LodePNGBitWriter {
        data: (0 as * mut crate::src::lodepng::ucvector),
        bp: 0,
    };
    LodePNGBitWriter_init(Some(&mut writer), out);
    if (*settings).btype > 2 as std::os::raw::c_int as std::os::raw::c_uint {
        return 61 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        if (*settings).btype == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            return deflateNoCompression(out, in_0, insize)
        } else {
            if (*settings).btype == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                blocksize = insize;
            } else {
                blocksize = insize
                    .wrapping_div(8 as std::os::raw::c_uint as std::os::raw::c_ulong)
                    .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong);
                if blocksize < 65536 as std::os::raw::c_int as std::os::raw::c_ulong {
                    blocksize = 65536 as std::os::raw::c_int as size_t;
                }
                if blocksize > 262144 as std::os::raw::c_int as std::os::raw::c_ulong {
                    blocksize = 262144 as std::os::raw::c_int as size_t;
                }
            }
        }
    }
    numdeflateblocks = insize
        .wrapping_add(blocksize)
        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        .wrapping_div(blocksize);
    if numdeflateblocks == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        numdeflateblocks = 1 as std::os::raw::c_int as size_t;
    }
    error = hash_init(Some(&mut hash), (*settings).windowsize);
    if error == 0 {
        i = 0 as std::os::raw::c_int as size_t;
        while i != numdeflateblocks && error == 0 {
            let mut final_0 = (i
                == numdeflateblocks.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong))
                as std::os::raw::c_int as std::os::raw::c_uint;
            let mut start = i.wrapping_mul(blocksize);
            let mut end = start.wrapping_add(blocksize);
            if end > insize {
                end = insize;
            }
            if (*settings).btype == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                error = deflateFixed(
                    Some(&mut writer),
                    Some(&mut hash),
                    in_0,
                    start,
                    end,
                    settings,
                    final_0,
                );
            } else if (*settings).btype == 2 as std::os::raw::c_int as std::os::raw::c_uint {
                error = deflateDynamic(
                    Some(&mut writer),
                    Some(&mut hash),
                    in_0,
                    start,
                    end,
                    settings,
                    final_0,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    hash_cleanup(Some(&mut hash));
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_deflate(
    mut out: * mut * mut std::os::raw::c_uchar,
    mut outsize: * mut std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    let mut v = ucvector_init(*out, *outsize);
    let mut error = lodepng_deflatev(&mut v, in_0, insize, settings);
    *out = v.data;
    *outsize = v.size;
    return error;
}
unsafe extern "C" fn deflate(
    mut out: * mut * mut std::os::raw::c_uchar,
    mut outsize: * mut std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    if ((*settings).custom_deflate).is_some() {
        let mut error = ((*settings).custom_deflate)
            .expect("non-null function pointer")(out, outsize, in_0, insize, settings);
        return (if error != 0 { 111 as std::os::raw::c_int } else { 0 as std::os::raw::c_int })
            as std::os::raw::c_uint;
    } else {
        return lodepng_deflate(out, outsize, in_0, insize, settings)
    };
}
unsafe extern "C" fn update_adler32(
    mut adler: std::os::raw::c_uint,
    mut data: * const std::os::raw::c_uchar,
    mut len: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut s1 = adler & 0xffff as std::os::raw::c_uint;
    let mut s2 = adler >> 16 as std::os::raw::c_uint & 0xffff as std::os::raw::c_uint;
    while len != 0 as std::os::raw::c_uint {
        let mut i: u32 = 0;
        let mut amount = if len > 5552 as std::os::raw::c_uint {
            5552 as std::os::raw::c_uint
        } else {
            len
        };
        len = len.wrapping_sub(amount);
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != amount {
            let mut fresh73 = data;
            data = data.offset(1);
            s1 = s1.wrapping_add(*fresh73 as std::os::raw::c_uint);
            s2 = s2.wrapping_add(s1);
            i = i.wrapping_add(1);
        }
        s1 = s1.wrapping_rem(65521 as std::os::raw::c_uint);
        s2 = s2.wrapping_rem(65521 as std::os::raw::c_uint);
    }
    return s2 << 16 as std::os::raw::c_uint | s1;
}
unsafe extern "C" fn adler32(
    mut data: * const std::os::raw::c_uchar,
    mut len: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    return update_adler32(1 as std::os::raw::c_uint, data, len);
}
unsafe extern "C" fn lodepng_zlib_decompressv(
    mut out: * mut crate::src::lodepng::ucvector,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGDecompressSettings,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut CM: u32 = 0;
    let mut CINFO: u32 = 0;
    let mut FDICT: u32 = 0;
    if insize < 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 53 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int * 256 as std::os::raw::c_int
        + *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int) % 31 as std::os::raw::c_int
        != 0 as std::os::raw::c_int
    {
        return 24 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    CM = (*in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int & 15 as std::os::raw::c_int)
        as std::os::raw::c_uint;
    CINFO = (*in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int >> 4 as std::os::raw::c_int
        & 15 as std::os::raw::c_int) as std::os::raw::c_uint;
    FDICT = (*in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int >> 5 as std::os::raw::c_int
        & 1 as std::os::raw::c_int) as std::os::raw::c_uint;
    if CM != 8 as std::os::raw::c_int as std::os::raw::c_uint || CINFO > 7 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return 25 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if FDICT != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return 26 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    error = inflatev(
        out,
        in_0.offset(2 as std::os::raw::c_int as isize),
        insize.wrapping_sub(2 as std::os::raw::c_int as std::os::raw::c_ulong),
        settings,
    );
    if error != 0 {
        return error;
    }
    if (*settings).ignore_adler32 == 0 {
        let mut ADLER32 = lodepng_read32bitInt(
            &*in_0
                .offset(insize.wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize),
        );
        let mut checksum = adler32((*out).data, (*out).size as std::os::raw::c_uint);
        if checksum != ADLER32 {
            return 58 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_zlib_decompress<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGDecompressSettings,
) -> std::os::raw::c_uint {
    let mut v = ucvector_init(*(borrow_mut(&mut out)).unwrap(), *(borrow_mut(&mut outsize)).unwrap());
    let mut error = lodepng_zlib_decompressv(&mut v, in_0, insize, settings);
    *(borrow_mut(&mut out)).unwrap() = v.data;
    *(borrow_mut(&mut outsize)).unwrap() = v.size;
    return error;
}
unsafe extern "C" fn zlib_decompress(
    mut out: * mut * mut std::os::raw::c_uchar,
    mut outsize: * mut std::os::raw::c_ulong,
    mut expected_size: std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGDecompressSettings,
) -> std::os::raw::c_uint {
    let mut error: u32 = 0;
    if ((*settings).custom_zlib).is_some() {
        error = ((*settings).custom_zlib)
            .expect("non-null function pointer")(out, outsize, in_0, insize, settings);
        if error != 0 {
            error = 110 as std::os::raw::c_int as std::os::raw::c_uint;
            if (*settings).max_output_size != 0 && *outsize > (*settings).max_output_size
            {
                error = 109 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
    } else {
        let mut v = ucvector_init(*out, *outsize);
        if expected_size != 0 {
            ucvector_resize(&mut v, (*outsize).wrapping_add(expected_size));
            v.size = *outsize;
        }
        error = lodepng_zlib_decompressv(&mut v, in_0, insize, settings);
        *out = v.data;
        *outsize = v.size;
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_zlib_compress(
    mut out: * mut * mut std::os::raw::c_uchar,
    mut outsize: * mut std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    let mut i: u64 = 0;
    let mut error: u32 = 0;
    let mut deflatedata = 0 as *mut std::os::raw::c_uchar;
    let mut deflatesize = 0 as std::os::raw::c_int as size_t;
    error = deflate(&mut deflatedata, &mut deflatesize, in_0, insize, settings);
    *out = 0 as *mut std::os::raw::c_uchar;
    *outsize = 0 as std::os::raw::c_int as size_t;
    if error == 0 {
        *outsize = deflatesize.wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong);
        *out = lodepng_malloc(*outsize) as *mut std::os::raw::c_uchar;
        if (*out).is_null() {
            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    if error == 0 {
        let mut ADLER32 = adler32(in_0, insize as std::os::raw::c_uint);
        let mut CMF = 120 as std::os::raw::c_int as std::os::raw::c_uint;
        let mut FLEVEL = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        let mut FDICT = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        let mut CMFFLG = (256 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_mul(CMF)
            .wrapping_add(FDICT.wrapping_mul(32 as std::os::raw::c_int as std::os::raw::c_uint))
            .wrapping_add(FLEVEL.wrapping_mul(64 as std::os::raw::c_int as std::os::raw::c_uint));
        let mut FCHECK = (31 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_sub(CMFFLG.wrapping_rem(31 as std::os::raw::c_int as std::os::raw::c_uint));
        CMFFLG = CMFFLG.wrapping_add(FCHECK);
        *(*out)
            .offset(
                0 as std::os::raw::c_int as isize,
            ) = (CMFFLG >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *(*out)
            .offset(
                1 as std::os::raw::c_int as isize,
            ) = (CMFFLG & 255 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
        i = 0 as std::os::raw::c_int as size_t;
        while i != deflatesize {
            *(*out)
                .offset(
                    i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *deflatedata.offset(i as isize);
            i = i.wrapping_add(1);
        }
        lodepng_set32bitInt(
            &mut *(*out)
                .offset(
                    (*outsize).wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ),
            ADLER32,
        );
    }
    lodepng_free(deflatedata as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn zlib_compress(
    mut out: * mut * mut std::os::raw::c_uchar,
    mut outsize: * mut std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut settings: * const crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    if ((*settings).custom_zlib).is_some() {
        let mut error = ((*settings).custom_zlib)
            .expect("non-null function pointer")(out, outsize, in_0, insize, settings);
        return (if error != 0 { 111 as std::os::raw::c_int } else { 0 as std::os::raw::c_int })
            as std::os::raw::c_uint;
    } else {
        return lodepng_zlib_compress(out, outsize, in_0, insize, settings)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_compress_settings_init<'a1>(
    mut settings: Option<&'a1 mut crate::src::lodepng::LodePNGCompressSettings>,
) {
    (*(borrow_mut(&mut settings)).unwrap()).btype = 2 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).use_lz77 = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).windowsize = 2048 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).minmatch = 3 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).nicematch = 128 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).lazymatching = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh74 = (*(borrow_mut(&mut settings)).unwrap()).custom_zlib;
    *fresh74 = None;
    let ref mut fresh75 = (*(borrow_mut(&mut settings)).unwrap()).custom_deflate;
    *fresh75 = None;
    let ref mut fresh76 = (*(borrow_mut(&mut settings)).unwrap()).custom_context;
    *fresh76 = 0 as *const std::os::raw::c_void;
}
#[no_mangle]
pub static mut lodepng_default_compress_settings: crate::src::lodepng::LodePNGCompressSettings = crate::src::lodepng::LodePNGCompressSettings::new(); unsafe fn laertes_init_lodepng_default_compress_settings() {
lodepng_default_compress_settings = {
    let mut init = LodePNGCompressSettings {
        btype: 2 as std::os::raw::c_int as std::os::raw::c_uint,
        use_lz77: 1 as std::os::raw::c_int as std::os::raw::c_uint,
        windowsize: 2048 as std::os::raw::c_int as std::os::raw::c_uint,
        minmatch: 3 as std::os::raw::c_int as std::os::raw::c_uint,
        nicematch: 128 as std::os::raw::c_int as std::os::raw::c_uint,
        lazymatching: 1 as std::os::raw::c_int as std::os::raw::c_uint,
        custom_zlib: None,
        custom_deflate: None,
        custom_context: 0 as *const std::os::raw::c_void,
    };
    init
};}//;
#[no_mangle]
pub unsafe extern "C" fn lodepng_decompress_settings_init<'a1>(
    mut settings: Option<&'a1 mut crate::src::lodepng::LodePNGDecompressSettings>,
) {
    (*(borrow_mut(&mut settings)).unwrap()).ignore_adler32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).ignore_nlen = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).max_output_size = 0 as std::os::raw::c_int as size_t;
    let ref mut fresh77 = (*(borrow_mut(&mut settings)).unwrap()).custom_zlib;
    *fresh77 = None;
    let ref mut fresh78 = (*(borrow_mut(&mut settings)).unwrap()).custom_inflate;
    *fresh78 = None;
    let ref mut fresh79 = (*(borrow_mut(&mut settings)).unwrap()).custom_context;
    *fresh79 = 0 as *const std::os::raw::c_void;
}
#[no_mangle]
pub static mut lodepng_default_decompress_settings: crate::src::lodepng::LodePNGDecompressSettings = crate::src::lodepng::LodePNGDecompressSettings::new(); unsafe fn laertes_init_lodepng_default_decompress_settings() {
lodepng_default_decompress_settings = {
    let mut init = LodePNGDecompressSettings {
        ignore_adler32: 0 as std::os::raw::c_int as std::os::raw::c_uint,
        ignore_nlen: 0 as std::os::raw::c_int as std::os::raw::c_uint,
        max_output_size: 0 as std::os::raw::c_int as size_t,
        custom_zlib: None,
        custom_inflate: None,
        custom_context: 0 as *const std::os::raw::c_void,
    };
    init
};}//;
static mut lodepng_crc32_table: [std::os::raw::c_uint; 256] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_lodepng_crc32_table() {
lodepng_crc32_table = [
    0 as std::os::raw::c_uint,
    1996959894 as std::os::raw::c_uint,
    3993919788 as std::os::raw::c_uint,
    2567524794 as std::os::raw::c_uint,
    124634137 as std::os::raw::c_uint,
    1886057615 as std::os::raw::c_uint,
    3915621685 as std::os::raw::c_uint,
    2657392035 as std::os::raw::c_uint,
    249268274 as std::os::raw::c_uint,
    2044508324 as std::os::raw::c_uint,
    3772115230 as std::os::raw::c_uint,
    2547177864 as std::os::raw::c_uint,
    162941995 as std::os::raw::c_uint,
    2125561021 as std::os::raw::c_uint,
    3887607047 as std::os::raw::c_uint,
    2428444049 as std::os::raw::c_uint,
    498536548 as std::os::raw::c_uint,
    1789927666 as std::os::raw::c_uint,
    4089016648 as std::os::raw::c_uint,
    2227061214 as std::os::raw::c_uint,
    450548861 as std::os::raw::c_uint,
    1843258603 as std::os::raw::c_uint,
    4107580753 as std::os::raw::c_uint,
    2211677639 as std::os::raw::c_uint,
    325883990 as std::os::raw::c_uint,
    1684777152 as std::os::raw::c_uint,
    4251122042 as std::os::raw::c_uint,
    2321926636 as std::os::raw::c_uint,
    335633487 as std::os::raw::c_uint,
    1661365465 as std::os::raw::c_uint,
    4195302755 as std::os::raw::c_uint,
    2366115317 as std::os::raw::c_uint,
    997073096 as std::os::raw::c_uint,
    1281953886 as std::os::raw::c_uint,
    3579855332 as std::os::raw::c_uint,
    2724688242 as std::os::raw::c_uint,
    1006888145 as std::os::raw::c_uint,
    1258607687 as std::os::raw::c_uint,
    3524101629 as std::os::raw::c_uint,
    2768942443 as std::os::raw::c_uint,
    901097722 as std::os::raw::c_uint,
    1119000684 as std::os::raw::c_uint,
    3686517206 as std::os::raw::c_uint,
    2898065728 as std::os::raw::c_uint,
    853044451 as std::os::raw::c_uint,
    1172266101 as std::os::raw::c_uint,
    3705015759 as std::os::raw::c_uint,
    2882616665 as std::os::raw::c_uint,
    651767980 as std::os::raw::c_uint,
    1373503546 as std::os::raw::c_uint,
    3369554304 as std::os::raw::c_uint,
    3218104598 as std::os::raw::c_uint,
    565507253 as std::os::raw::c_uint,
    1454621731 as std::os::raw::c_uint,
    3485111705 as std::os::raw::c_uint,
    3099436303 as std::os::raw::c_uint,
    671266974 as std::os::raw::c_uint,
    1594198024 as std::os::raw::c_uint,
    3322730930 as std::os::raw::c_uint,
    2970347812 as std::os::raw::c_uint,
    795835527 as std::os::raw::c_uint,
    1483230225 as std::os::raw::c_uint,
    3244367275 as std::os::raw::c_uint,
    3060149565 as std::os::raw::c_uint,
    1994146192 as std::os::raw::c_uint,
    31158534 as std::os::raw::c_uint,
    2563907772 as std::os::raw::c_uint,
    4023717930 as std::os::raw::c_uint,
    1907459465 as std::os::raw::c_uint,
    112637215 as std::os::raw::c_uint,
    2680153253 as std::os::raw::c_uint,
    3904427059 as std::os::raw::c_uint,
    2013776290 as std::os::raw::c_uint,
    251722036 as std::os::raw::c_uint,
    2517215374 as std::os::raw::c_uint,
    3775830040 as std::os::raw::c_uint,
    2137656763 as std::os::raw::c_uint,
    141376813 as std::os::raw::c_uint,
    2439277719 as std::os::raw::c_uint,
    3865271297 as std::os::raw::c_uint,
    1802195444 as std::os::raw::c_uint,
    476864866 as std::os::raw::c_uint,
    2238001368 as std::os::raw::c_uint,
    4066508878 as std::os::raw::c_uint,
    1812370925 as std::os::raw::c_uint,
    453092731 as std::os::raw::c_uint,
    2181625025 as std::os::raw::c_uint,
    4111451223 as std::os::raw::c_uint,
    1706088902 as std::os::raw::c_uint,
    314042704 as std::os::raw::c_uint,
    2344532202 as std::os::raw::c_uint,
    4240017532 as std::os::raw::c_uint,
    1658658271 as std::os::raw::c_uint,
    366619977 as std::os::raw::c_uint,
    2362670323 as std::os::raw::c_uint,
    4224994405 as std::os::raw::c_uint,
    1303535960 as std::os::raw::c_uint,
    984961486 as std::os::raw::c_uint,
    2747007092 as std::os::raw::c_uint,
    3569037538 as std::os::raw::c_uint,
    1256170817 as std::os::raw::c_uint,
    1037604311 as std::os::raw::c_uint,
    2765210733 as std::os::raw::c_uint,
    3554079995 as std::os::raw::c_uint,
    1131014506 as std::os::raw::c_uint,
    879679996 as std::os::raw::c_uint,
    2909243462 as std::os::raw::c_uint,
    3663771856 as std::os::raw::c_uint,
    1141124467 as std::os::raw::c_uint,
    855842277 as std::os::raw::c_uint,
    2852801631 as std::os::raw::c_uint,
    3708648649 as std::os::raw::c_uint,
    1342533948 as std::os::raw::c_uint,
    654459306 as std::os::raw::c_uint,
    3188396048 as std::os::raw::c_uint,
    3373015174 as std::os::raw::c_uint,
    1466479909 as std::os::raw::c_uint,
    544179635 as std::os::raw::c_uint,
    3110523913 as std::os::raw::c_uint,
    3462522015 as std::os::raw::c_uint,
    1591671054 as std::os::raw::c_uint,
    702138776 as std::os::raw::c_uint,
    2966460450 as std::os::raw::c_uint,
    3352799412 as std::os::raw::c_uint,
    1504918807 as std::os::raw::c_uint,
    783551873 as std::os::raw::c_uint,
    3082640443 as std::os::raw::c_uint,
    3233442989 as std::os::raw::c_uint,
    3988292384 as std::os::raw::c_uint,
    2596254646 as std::os::raw::c_uint,
    62317068 as std::os::raw::c_uint,
    1957810842 as std::os::raw::c_uint,
    3939845945 as std::os::raw::c_uint,
    2647816111 as std::os::raw::c_uint,
    81470997 as std::os::raw::c_uint,
    1943803523 as std::os::raw::c_uint,
    3814918930 as std::os::raw::c_uint,
    2489596804 as std::os::raw::c_uint,
    225274430 as std::os::raw::c_uint,
    2053790376 as std::os::raw::c_uint,
    3826175755 as std::os::raw::c_uint,
    2466906013 as std::os::raw::c_uint,
    167816743 as std::os::raw::c_uint,
    2097651377 as std::os::raw::c_uint,
    4027552580 as std::os::raw::c_uint,
    2265490386 as std::os::raw::c_uint,
    503444072 as std::os::raw::c_uint,
    1762050814 as std::os::raw::c_uint,
    4150417245 as std::os::raw::c_uint,
    2154129355 as std::os::raw::c_uint,
    426522225 as std::os::raw::c_uint,
    1852507879 as std::os::raw::c_uint,
    4275313526 as std::os::raw::c_uint,
    2312317920 as std::os::raw::c_uint,
    282753626 as std::os::raw::c_uint,
    1742555852 as std::os::raw::c_uint,
    4189708143 as std::os::raw::c_uint,
    2394877945 as std::os::raw::c_uint,
    397917763 as std::os::raw::c_uint,
    1622183637 as std::os::raw::c_uint,
    3604390888 as std::os::raw::c_uint,
    2714866558 as std::os::raw::c_uint,
    953729732 as std::os::raw::c_uint,
    1340076626 as std::os::raw::c_uint,
    3518719985 as std::os::raw::c_uint,
    2797360999 as std::os::raw::c_uint,
    1068828381 as std::os::raw::c_uint,
    1219638859 as std::os::raw::c_uint,
    3624741850 as std::os::raw::c_uint,
    2936675148 as std::os::raw::c_uint,
    906185462 as std::os::raw::c_uint,
    1090812512 as std::os::raw::c_uint,
    3747672003 as std::os::raw::c_uint,
    2825379669 as std::os::raw::c_uint,
    829329135 as std::os::raw::c_uint,
    1181335161 as std::os::raw::c_uint,
    3412177804 as std::os::raw::c_uint,
    3160834842 as std::os::raw::c_uint,
    628085408 as std::os::raw::c_uint,
    1382605366 as std::os::raw::c_uint,
    3423369109 as std::os::raw::c_uint,
    3138078467 as std::os::raw::c_uint,
    570562233 as std::os::raw::c_uint,
    1426400815 as std::os::raw::c_uint,
    3317316542 as std::os::raw::c_uint,
    2998733608 as std::os::raw::c_uint,
    733239954 as std::os::raw::c_uint,
    1555261956 as std::os::raw::c_uint,
    3268935591 as std::os::raw::c_uint,
    3050360625 as std::os::raw::c_uint,
    752459403 as std::os::raw::c_uint,
    1541320221 as std::os::raw::c_uint,
    2607071920 as std::os::raw::c_uint,
    3965973030 as std::os::raw::c_uint,
    1969922972 as std::os::raw::c_uint,
    40735498 as std::os::raw::c_uint,
    2617837225 as std::os::raw::c_uint,
    3943577151 as std::os::raw::c_uint,
    1913087877 as std::os::raw::c_uint,
    83908371 as std::os::raw::c_uint,
    2512341634 as std::os::raw::c_uint,
    3803740692 as std::os::raw::c_uint,
    2075208622 as std::os::raw::c_uint,
    213261112 as std::os::raw::c_uint,
    2463272603 as std::os::raw::c_uint,
    3855990285 as std::os::raw::c_uint,
    2094854071 as std::os::raw::c_uint,
    198958881 as std::os::raw::c_uint,
    2262029012 as std::os::raw::c_uint,
    4057260610 as std::os::raw::c_uint,
    1759359992 as std::os::raw::c_uint,
    534414190 as std::os::raw::c_uint,
    2176718541 as std::os::raw::c_uint,
    4139329115 as std::os::raw::c_uint,
    1873836001 as std::os::raw::c_uint,
    414664567 as std::os::raw::c_uint,
    2282248934 as std::os::raw::c_uint,
    4279200368 as std::os::raw::c_uint,
    1711684554 as std::os::raw::c_uint,
    285281116 as std::os::raw::c_uint,
    2405801727 as std::os::raw::c_uint,
    4167216745 as std::os::raw::c_uint,
    1634467795 as std::os::raw::c_uint,
    376229701 as std::os::raw::c_uint,
    2685067896 as std::os::raw::c_uint,
    3608007406 as std::os::raw::c_uint,
    1308918612 as std::os::raw::c_uint,
    956543938 as std::os::raw::c_uint,
    2808555105 as std::os::raw::c_uint,
    3495958263 as std::os::raw::c_uint,
    1231636301 as std::os::raw::c_uint,
    1047427035 as std::os::raw::c_uint,
    2932959818 as std::os::raw::c_uint,
    3654703836 as std::os::raw::c_uint,
    1088359270 as std::os::raw::c_uint,
    936918000 as std::os::raw::c_uint,
    2847714899 as std::os::raw::c_uint,
    3736837829 as std::os::raw::c_uint,
    1202900863 as std::os::raw::c_uint,
    817233897 as std::os::raw::c_uint,
    3183342108 as std::os::raw::c_uint,
    3401237130 as std::os::raw::c_uint,
    1404277552 as std::os::raw::c_uint,
    615818150 as std::os::raw::c_uint,
    3134207493 as std::os::raw::c_uint,
    3453421203 as std::os::raw::c_uint,
    1423857449 as std::os::raw::c_uint,
    601450431 as std::os::raw::c_uint,
    3009837614 as std::os::raw::c_uint,
    3294710456 as std::os::raw::c_uint,
    1567103746 as std::os::raw::c_uint,
    711928724 as std::os::raw::c_uint,
    3020668471 as std::os::raw::c_uint,
    3272380065 as std::os::raw::c_uint,
    1510334235 as std::os::raw::c_uint,
    755167117 as std::os::raw::c_uint,
];}//;
#[no_mangle]
pub unsafe extern "C" fn lodepng_crc32(
    mut data: * const std::os::raw::c_uchar,
    mut length: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut r = 0xffffffff as std::os::raw::c_uint;
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < length {
        r = lodepng_crc32_table[((r ^ *data.offset(i as isize) as std::os::raw::c_uint)
            & 0xff as std::os::raw::c_uint) as usize] ^ r >> 8 as std::os::raw::c_uint;
        i = i.wrapping_add(1);
    }
    return r ^ 0xffffffff as std::os::raw::c_uint;
}
unsafe extern "C" fn readBitFromReversedStream<'a1>(
    mut bitpointer: Option<&'a1 mut std::os::raw::c_ulong>,
    mut bitstream: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uchar {
    let mut result = (*bitstream.offset((*(borrow(& bitpointer)).unwrap() >> 3 as std::os::raw::c_int) as isize)
        as std::os::raw::c_int
        >> (7 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_sub(*(borrow(& bitpointer)).unwrap() & 0x7 as std::os::raw::c_int as std::os::raw::c_ulong)
        & 1 as std::os::raw::c_int) as std::os::raw::c_uchar;
    *(borrow_mut(&mut bitpointer)).unwrap() = (*(borrow(& bitpointer)).unwrap()).wrapping_add(1);
    return result;
}
unsafe extern "C" fn readBitsFromReversedStream<'a1>(
    mut bitpointer: Option<&'a1 mut std::os::raw::c_ulong>,
    mut bitstream: * const std::os::raw::c_uchar,
    mut nbits: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut result = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < nbits {
        result <<= 1 as std::os::raw::c_uint;
        result |= readBitFromReversedStream(borrow_mut(&mut bitpointer), bitstream) as std::os::raw::c_uint;
        i = i.wrapping_add(1);
    }
    return result;
}
unsafe extern "C" fn setBitOfReversedStream<'a1>(
    mut bitpointer: Option<&'a1 mut std::os::raw::c_ulong>,
    mut bitstream: * mut std::os::raw::c_uchar,
    mut bit: std::os::raw::c_uchar,
) {
    if bit as std::os::raw::c_int == 0 as std::os::raw::c_int {
        let ref mut fresh80 = *bitstream
            .offset((*(borrow(& bitpointer)).unwrap() >> 3 as std::os::raw::c_uint) as isize);
        *fresh80 = (*fresh80 as std::os::raw::c_int
            & !((1 as std::os::raw::c_uint)
                << (7 as std::os::raw::c_uint as std::os::raw::c_ulong)
                    .wrapping_sub(*(borrow(& bitpointer)).unwrap() & 7 as std::os::raw::c_uint as std::os::raw::c_ulong))
                as std::os::raw::c_uchar as std::os::raw::c_int) as std::os::raw::c_uchar;
    } else {
        let ref mut fresh81 = *bitstream
            .offset((*(borrow(& bitpointer)).unwrap() >> 3 as std::os::raw::c_uint) as isize);
        *fresh81 = (*fresh81 as std::os::raw::c_uint
            | (1 as std::os::raw::c_uint)
                << (7 as std::os::raw::c_uint as std::os::raw::c_ulong)
                    .wrapping_sub(*(borrow(& bitpointer)).unwrap() & 7 as std::os::raw::c_uint as std::os::raw::c_ulong))
            as std::os::raw::c_uchar;
    }
    *(borrow_mut(&mut bitpointer)).unwrap() = (*(borrow(& bitpointer)).unwrap()).wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_length(
    mut chunk: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    return lodepng_read32bitInt(chunk);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_type(
    mut type_0: * mut std::os::raw::c_char,
    mut chunk: * const std::os::raw::c_uchar,
) {
    let mut i: u32 = 0;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 4 as std::os::raw::c_int as std::os::raw::c_uint {
        *type_0
            .offset(
                i as isize,
            ) = *chunk
            .offset((4 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_add(i) as isize)
            as std::os::raw::c_char;
        i = i.wrapping_add(1);
    }
    *type_0.offset(4 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_type_equals(
    mut chunk: * const std::os::raw::c_uchar,
    mut type_0: * const std::os::raw::c_char,
) -> std::os::raw::c_uchar {
    if lodepng_strlen(type_0) != 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    }
    return (*chunk.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int
        == *type_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
        && *chunk.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int
            == *type_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
        && *chunk.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_int
            == *type_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
        && *chunk.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int
            == *type_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int) as std::os::raw::c_int
        as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_ancillary(
    mut chunk: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uchar {
    return (*chunk.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int & 32 as std::os::raw::c_int
        != 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_private(
    mut chunk: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uchar {
    return (*chunk.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_int & 32 as std::os::raw::c_int
        != 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_safetocopy(
    mut chunk: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uchar {
    return (*chunk.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int & 32 as std::os::raw::c_int
        != 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_data<'a1>(
    mut chunk: * mut std::os::raw::c_uchar,
) -> Option<&'a1 mut std::os::raw::c_uchar> {
    return Some(&mut *chunk.offset(8 as std::os::raw::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_data_const(
    mut chunk: * const std::os::raw::c_uchar,
) -> * const std::os::raw::c_uchar {
    return &*chunk.offset(8 as std::os::raw::c_int as isize) as *const std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_check_crc(
    mut chunk: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    let mut length = lodepng_chunk_length(chunk);
    let mut CRC = lodepng_read32bitInt(
        &*chunk.offset(length.wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_uint) as isize),
    );
    let mut checksum = lodepng_crc32(
        &*chunk.offset(4 as std::os::raw::c_int as isize),
        length.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
    );
    if CRC != checksum {
        return 1 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        return 0 as std::os::raw::c_int as std::os::raw::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_generate_crc(mut chunk: * mut std::os::raw::c_uchar) {
    let mut length = lodepng_chunk_length(chunk);
    let mut CRC = lodepng_crc32(
        &mut *chunk.offset(4 as std::os::raw::c_int as isize),
        length.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
    );
    lodepng_set32bitInt(
        chunk.offset(8 as std::os::raw::c_int as isize).offset(length as isize),
        CRC,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_next(
    mut chunk: * mut std::os::raw::c_uchar,
    mut end: * mut std::os::raw::c_uchar,
) -> * mut std::os::raw::c_uchar {
    let mut available_size = end.offset_from(chunk) as std::os::raw::c_long as size_t;
    if chunk >= end || available_size < 12 as std::os::raw::c_int as std::os::raw::c_ulong {
        return end;
    }
    if *chunk.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x89 as std::os::raw::c_int
        && *chunk.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x50 as std::os::raw::c_int
        && *chunk.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x4e as std::os::raw::c_int
        && *chunk.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x47 as std::os::raw::c_int
        && *chunk.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0xd as std::os::raw::c_int
        && *chunk.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0xa as std::os::raw::c_int
        && *chunk.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x1a as std::os::raw::c_int
        && *chunk.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0xa as std::os::raw::c_int
    {
        return chunk.offset(8 as std::os::raw::c_int as isize)
    } else {
        let mut total_chunk_length: u64 = 0;
        if lodepng_addofl(
            lodepng_chunk_length(chunk) as size_t,
            12 as std::os::raw::c_int as size_t,
            Some(&mut total_chunk_length),
        ) != 0
        {
            return end;
        }
        if total_chunk_length > available_size {
            return end;
        }
        return chunk.offset(total_chunk_length as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_next_const(
    mut chunk: * const std::os::raw::c_uchar,
    mut end: * const std::os::raw::c_uchar,
) -> * const std::os::raw::c_uchar {
    let mut available_size = end.offset_from(chunk) as std::os::raw::c_long as size_t;
    if chunk >= end || available_size < 12 as std::os::raw::c_int as std::os::raw::c_ulong {
        return end;
    }
    if *chunk.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x89 as std::os::raw::c_int
        && *chunk.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x50 as std::os::raw::c_int
        && *chunk.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x4e as std::os::raw::c_int
        && *chunk.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x47 as std::os::raw::c_int
        && *chunk.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0xd as std::os::raw::c_int
        && *chunk.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0xa as std::os::raw::c_int
        && *chunk.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0x1a as std::os::raw::c_int
        && *chunk.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0xa as std::os::raw::c_int
    {
        return chunk.offset(8 as std::os::raw::c_int as isize)
    } else {
        let mut total_chunk_length: u64 = 0;
        if lodepng_addofl(
            lodepng_chunk_length(chunk) as size_t,
            12 as std::os::raw::c_int as size_t,
            Some(&mut total_chunk_length),
        ) != 0
        {
            return end;
        }
        if total_chunk_length > available_size {
            return end;
        }
        return chunk.offset(total_chunk_length as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_find(
    mut chunk: * mut std::os::raw::c_uchar,
    mut end: * mut std::os::raw::c_uchar,
    mut type_0: * const std::os::raw::c_char,
) -> * mut std::os::raw::c_uchar {
    loop {
        if chunk >= end
            || (end.offset_from(chunk) as std::os::raw::c_long)
                < 12 as std::os::raw::c_int as std::os::raw::c_long
        {
            return 0 as *mut std::os::raw::c_uchar;
        }
        if lodepng_chunk_type_equals(chunk, type_0) != 0 {
            return chunk;
        }
        chunk = lodepng_chunk_next(chunk, end);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_find_const(
    mut chunk: * const std::os::raw::c_uchar,
    mut end: * const std::os::raw::c_uchar,
    mut type_0: * const std::os::raw::c_char,
) -> * const std::os::raw::c_uchar {
    loop {
        if chunk >= end
            || (end.offset_from(chunk) as std::os::raw::c_long)
                < 12 as std::os::raw::c_int as std::os::raw::c_long
        {
            return 0 as *const std::os::raw::c_uchar;
        }
        if lodepng_chunk_type_equals(chunk, type_0) != 0 {
            return chunk;
        }
        chunk = lodepng_chunk_next_const(chunk, end);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_append<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut chunk: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    let mut total_chunk_length: u64 = 0;
    let mut new_length: u64 = 0;
    let mut chunk_start = 0 as *mut std::os::raw::c_uchar;
    let mut new_buffer = 0 as *mut std::os::raw::c_uchar;
    if lodepng_addofl(
        lodepng_chunk_length(chunk) as size_t,
        12 as std::os::raw::c_int as size_t,
        Some(&mut total_chunk_length),
    ) != 0
    {
        return 77 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if lodepng_addofl(*(borrow_mut(&mut outsize)).unwrap(), total_chunk_length, Some(&mut new_length)) != 0 {
        return 77 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    new_buffer = lodepng_realloc(*(borrow_mut(&mut out)).unwrap() as *mut std::os::raw::c_void, new_length)
        as *mut std::os::raw::c_uchar;
    if new_buffer.is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    *(borrow_mut(&mut out)).unwrap() = new_buffer;
    *(borrow_mut(&mut outsize)).unwrap() = new_length;
    chunk_start = &mut *(*(borrow(& out)).unwrap())
        .offset(new_length.wrapping_sub(total_chunk_length) as isize)
        as *mut std::os::raw::c_uchar;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i as std::os::raw::c_ulong != total_chunk_length {
        *chunk_start.offset(i as isize) = *chunk.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn lodepng_chunk_init<'a1>(
    mut chunk: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut out: * mut crate::src::lodepng::ucvector,
    mut length: std::os::raw::c_uint,
    mut type_0: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    let mut new_length = (*out).size;
    if lodepng_addofl(new_length, length as size_t, Some(&mut new_length)) != 0 {
        return 77 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if lodepng_addofl(new_length, 12 as std::os::raw::c_int as size_t, Some(&mut new_length)) != 0 {
        return 77 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if ucvector_resize(out, new_length) == 0 {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    *(borrow_mut(&mut chunk)).unwrap() = ((*out).data)
        .offset(new_length as isize)
        .offset(-(length as isize))
        .offset(-(12 as std::os::raw::c_uint as isize));
    lodepng_set32bitInt(*(borrow_mut(&mut chunk)).unwrap(), length);
    lodepng_memcpy(
        (*(borrow(& chunk)).unwrap()).offset(4 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
        type_0 as *const std::os::raw::c_void,
        4 as std::os::raw::c_int as size_t,
    );
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn lodepng_chunk_createv(
    mut out: * mut crate::src::lodepng::ucvector,
    mut length: std::os::raw::c_uint,
    mut type_0: * const std::os::raw::c_char,
    mut data: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut error = lodepng_chunk_init(Some(&mut chunk), out, length, type_0);
    if error != 0 {
        return error;
    }
    lodepng_memcpy(
        chunk.offset(8 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
        data as *const std::os::raw::c_void,
        length as size_t,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_chunk_create<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut length: std::os::raw::c_uint,
    mut type_0: * const std::os::raw::c_char,
    mut data: * const std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    let mut v = ucvector_init(*(borrow_mut(&mut out)).unwrap(), *(borrow_mut(&mut outsize)).unwrap());
    let mut error = lodepng_chunk_createv(&mut v, length, type_0, data);
    *(borrow_mut(&mut out)).unwrap() = v.data;
    *(borrow_mut(&mut outsize)).unwrap() = v.size;
    return error;
}
 extern "C" fn checkColorValidity(
    mut colortype: std::os::raw::c_uint,
    mut bd: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    match colortype as std::os::raw::c_uint {
        0 => {
            if !(bd == 1 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 2 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 4 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 8 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 16 as std::os::raw::c_int as std::os::raw::c_uint)
            {
                return 37 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
        2 => {
            if !(bd == 8 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 16 as std::os::raw::c_int as std::os::raw::c_uint)
            {
                return 37 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
        3 => {
            if !(bd == 1 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 2 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 4 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 8 as std::os::raw::c_int as std::os::raw::c_uint)
            {
                return 37 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
        4 => {
            if !(bd == 8 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 16 as std::os::raw::c_int as std::os::raw::c_uint)
            {
                return 37 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
        6 => {
            if !(bd == 8 as std::os::raw::c_int as std::os::raw::c_uint
                || bd == 16 as std::os::raw::c_int as std::os::raw::c_uint)
            {
                return 37 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
        255 => return 31 as std::os::raw::c_int as std::os::raw::c_uint,
        _ => return 31 as std::os::raw::c_int as std::os::raw::c_uint,
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
 extern "C" fn getNumColorChannels(
    mut colortype: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    match colortype as std::os::raw::c_uint {
        0 => return 1 as std::os::raw::c_int as std::os::raw::c_uint,
        2 => return 3 as std::os::raw::c_int as std::os::raw::c_uint,
        3 => return 1 as std::os::raw::c_int as std::os::raw::c_uint,
        4 => return 2 as std::os::raw::c_int as std::os::raw::c_uint,
        6 => return 4 as std::os::raw::c_int as std::os::raw::c_uint,
        255 => return 0 as std::os::raw::c_int as std::os::raw::c_uint,
        _ => return 0 as std::os::raw::c_int as std::os::raw::c_uint,
    };
}
 extern "C" fn lodepng_get_bpp_lct(
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    return (getNumColorChannels(colortype)).wrapping_mul(bitdepth);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_init(mut info: * mut crate::src::lodepng::LodePNGColorMode) {
    (*info).key_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh82 = (*info).key_b;
    *fresh82 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh83 = (*info).key_g;
    *fresh83 = *fresh82;
    (*info).key_r = *fresh83;
    (*info).colortype = LCT_RGBA;
    (*info).bitdepth = 8 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh84 = (*info).palette;
    *fresh84 = 0 as *mut std::os::raw::c_uchar;
    (*info).palettesize = 0 as std::os::raw::c_int as size_t;
}
unsafe extern "C" fn lodepng_color_mode_alloc_palette(mut info: * mut crate::src::lodepng::LodePNGColorMode) {
    let mut i: u64 = 0;
    if ((*info).palette).is_null() {
        let ref mut fresh85 = (*info).palette;
        *fresh85 = lodepng_malloc(1024 as std::os::raw::c_int as size_t) as *mut std::os::raw::c_uchar;
    }
    if ((*info).palette).is_null() {
        return;
    }
    i = 0 as std::os::raw::c_int as size_t;
    while i != 256 as std::os::raw::c_int as std::os::raw::c_ulong {
        *((*info).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        *((*info).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        *((*info).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        *((*info).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_cleanup(mut info: * mut crate::src::lodepng::LodePNGColorMode) {
    lodepng_palette_clear(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_copy(
    mut dest: * mut crate::src::lodepng::LodePNGColorMode,
    mut source: * const crate::src::lodepng::LodePNGColorMode,
) -> std::os::raw::c_uint {
    lodepng_color_mode_cleanup(dest);
    lodepng_memcpy(
        dest as *mut std::os::raw::c_void,
        source as *const std::os::raw::c_void,
        ::std::mem::size_of::<LodePNGColorMode>() as std::os::raw::c_ulong,
    );
    if !((*source).palette).is_null() {
        let ref mut fresh86 = (*dest).palette;
        *fresh86 = lodepng_malloc(1024 as std::os::raw::c_int as size_t) as *mut std::os::raw::c_uchar;
        if ((*dest).palette).is_null() && (*source).palettesize != 0 {
            return 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        lodepng_memcpy(
            (*dest).palette as *mut std::os::raw::c_void,
            (*source).palette as *const std::os::raw::c_void,
            ((*source).palettesize).wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong),
        );
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_mode_make(
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
) -> crate::src::lodepng::LodePNGColorMode {
    let mut result = LodePNGColorMode {
        colortype: LCT_GREY,
        bitdepth: 0,
        palette: 0 as *mut std::os::raw::c_uchar,
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
    };
    lodepng_color_mode_init(&mut result);
    result.colortype = colortype;
    result.bitdepth = bitdepth;
    return result;
}
unsafe extern "C" fn lodepng_color_mode_equal<'a1, 'a2>(
    mut a: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
    mut b: Option<&'a2 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_int {
    let mut i: u64 = 0;
    if (*((a).clone()).unwrap()).colortype as std::os::raw::c_uint != (*((b).clone()).unwrap()).colortype as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int;
    }
    if (*((a).clone()).unwrap()).bitdepth != (*((b).clone()).unwrap()).bitdepth {
        return 0 as std::os::raw::c_int;
    }
    if (*((a).clone()).unwrap()).key_defined != (*((b).clone()).unwrap()).key_defined {
        return 0 as std::os::raw::c_int;
    }
    if (*((a).clone()).unwrap()).key_defined != 0 {
        if (*((a).clone()).unwrap()).key_r != (*((b).clone()).unwrap()).key_r {
            return 0 as std::os::raw::c_int;
        }
        if (*((a).clone()).unwrap()).key_g != (*((b).clone()).unwrap()).key_g {
            return 0 as std::os::raw::c_int;
        }
        if (*((a).clone()).unwrap()).key_b != (*((b).clone()).unwrap()).key_b {
            return 0 as std::os::raw::c_int;
        }
    }
    if (*((a).clone()).unwrap()).palettesize != (*((b).clone()).unwrap()).palettesize {
        return 0 as std::os::raw::c_int;
    }
    i = 0 as std::os::raw::c_int as size_t;
    while i != ((*((a).clone()).unwrap()).palettesize).wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong) {
        if *((*((a).clone()).unwrap()).palette).offset(i as isize) as std::os::raw::c_int
            != *((*((b).clone()).unwrap()).palette).offset(i as isize) as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_palette_clear(mut info: * mut crate::src::lodepng::LodePNGColorMode) {
    if !((*info).palette).is_null() {
        lodepng_free((*info).palette as *mut std::os::raw::c_void);
    }
    let ref mut fresh87 = (*info).palette;
    *fresh87 = 0 as *mut std::os::raw::c_uchar;
    (*info).palettesize = 0 as std::os::raw::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_palette_add(
    mut info: * mut crate::src::lodepng::LodePNGColorMode,
    mut r: std::os::raw::c_uchar,
    mut g: std::os::raw::c_uchar,
    mut b: std::os::raw::c_uchar,
    mut a: std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    if ((*info).palette).is_null() {
        lodepng_color_mode_alloc_palette(info);
        if ((*info).palette).is_null() {
            return 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    if (*info).palettesize >= 256 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 108 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    *((*info).palette)
        .offset(
            (4 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul((*info).palettesize)
                .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        ) = r;
    *((*info).palette)
        .offset(
            (4 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul((*info).palettesize)
                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        ) = g;
    *((*info).palette)
        .offset(
            (4 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul((*info).palettesize)
                .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        ) = b;
    *((*info).palette)
        .offset(
            (4 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_mul((*info).palettesize)
                .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        ) = a;
    let ref mut fresh88 = (*info).palettesize;
    *fresh88 = (*fresh88).wrapping_add(1);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_get_bpp<'a1>(
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    return lodepng_get_bpp_lct((*(info).unwrap()).colortype, (*(info).unwrap()).bitdepth);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_get_channels<'a1>(
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    return getNumColorChannels((*(info).unwrap()).colortype);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_is_greyscale_type<'a1>(
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    return ((*((info).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
        || (*((info).clone()).unwrap()).colortype as std::os::raw::c_uint
            == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int
        as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_is_alpha_type<'a1>(
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    return ((*((info).clone()).unwrap()).colortype as std::os::raw::c_uint & 4 as std::os::raw::c_int as std::os::raw::c_uint
        != 0 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_is_palette_type<'a1>(
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    return ((*((info).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_has_palette_alpha<'a1>(
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i != (*((info).clone()).unwrap()).palettesize {
        if (*((*((info).clone()).unwrap()).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) as std::os::raw::c_int) < 255 as std::os::raw::c_int
        {
            return 1 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_can_have_alpha<'a1>(
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    return ((*((info).clone()).unwrap()).key_defined != 0 || lodepng_is_alpha_type((info).clone()) != 0
        || lodepng_has_palette_alpha((info).clone()) != 0) as std::os::raw::c_int as std::os::raw::c_uint;
}
 extern "C" fn lodepng_get_raw_size_lct(
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
) -> std::os::raw::c_ulong {
    let mut bpp = lodepng_get_bpp_lct(colortype, bitdepth) as size_t;
    let mut n = (w as size_t).wrapping_mul(h as size_t);
    return n
        .wrapping_div(8 as std::os::raw::c_uint as std::os::raw::c_ulong)
        .wrapping_mul(bpp)
        .wrapping_add(
            (n & 7 as std::os::raw::c_uint as std::os::raw::c_ulong)
                .wrapping_mul(bpp)
                .wrapping_add(7 as std::os::raw::c_uint as std::os::raw::c_ulong)
                .wrapping_div(8 as std::os::raw::c_uint as std::os::raw::c_ulong),
        );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_get_raw_size<'a1>(
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut color: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_ulong {
    return lodepng_get_raw_size_lct(w, h, (*(color).unwrap()).colortype, (*(color).unwrap()).bitdepth);
}
 extern "C" fn lodepng_get_raw_size_idat(
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut bpp: std::os::raw::c_uint,
) -> std::os::raw::c_ulong {
    let mut line = (w.wrapping_div(8 as std::os::raw::c_uint) as size_t)
        .wrapping_mul(bpp as std::os::raw::c_ulong)
        .wrapping_add(1 as std::os::raw::c_uint as std::os::raw::c_ulong)
        .wrapping_add(
            (w & 7 as std::os::raw::c_uint)
                .wrapping_mul(bpp)
                .wrapping_add(7 as std::os::raw::c_uint)
                .wrapping_div(8 as std::os::raw::c_uint) as std::os::raw::c_ulong,
        );
    return (h as size_t).wrapping_mul(line);
}
unsafe extern "C" fn lodepng_pixel_overflow<'a1, 'a2>(
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut pngcolor: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
    mut rawcolor: Option<&'a2 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_int {
    let mut bpp = (if lodepng_get_bpp((pngcolor).clone()) > lodepng_get_bpp((rawcolor).clone()) {
        lodepng_get_bpp((pngcolor).clone())
    } else {
        lodepng_get_bpp((rawcolor).clone())
    }) as size_t;
    let mut numpixels: u64 = 0;
    let mut total: u64 = 0;
    let mut line: u64 = 0;
    if lodepng_mulofl(w as size_t, h as size_t, Some(&mut numpixels)) != 0 {
        return 1 as std::os::raw::c_int;
    }
    if lodepng_mulofl(numpixels, 8 as std::os::raw::c_int as size_t, Some(&mut total)) != 0 {
        return 1 as std::os::raw::c_int;
    }
    if lodepng_mulofl(w.wrapping_div(8 as std::os::raw::c_uint) as size_t, bpp, Some(&mut line)) != 0 {
        return 1 as std::os::raw::c_int;
    }
    if lodepng_addofl(
        line,
        ((w & 7 as std::os::raw::c_uint) as std::os::raw::c_ulong)
            .wrapping_mul(bpp)
            .wrapping_add(7 as std::os::raw::c_uint as std::os::raw::c_ulong)
            .wrapping_div(8 as std::os::raw::c_uint as std::os::raw::c_ulong),
        Some(&mut line),
    ) != 0
    {
        return 1 as std::os::raw::c_int;
    }
    if lodepng_addofl(line, 5 as std::os::raw::c_int as size_t, Some(&mut line)) != 0 {
        return 1 as std::os::raw::c_int;
    }
    if lodepng_mulofl(line, h as size_t, Some(&mut total)) != 0 {
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn LodePNGUnknownChunks_init(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    let mut i: u32 = 0;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 3 as std::os::raw::c_int as std::os::raw::c_uint {
        let ref mut fresh89 = (*info).unknown_chunks_data[i as usize];
        *fresh89 = 0 as *mut std::os::raw::c_uchar;
        i = i.wrapping_add(1);
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 3 as std::os::raw::c_int as std::os::raw::c_uint {
        (*info).unknown_chunks_size[i as usize] = 0 as std::os::raw::c_int as size_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn LodePNGUnknownChunks_cleanup(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    let mut i: u32 = 0;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 3 as std::os::raw::c_int as std::os::raw::c_uint {
        lodepng_free((*info).unknown_chunks_data[i as usize] as *mut std::os::raw::c_void);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn LodePNGUnknownChunks_copy(
    mut dest: * mut crate::src::lodepng::LodePNGInfo,
    mut src: * const crate::src::lodepng::LodePNGInfo,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    LodePNGUnknownChunks_cleanup(dest);
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 3 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut j: u64 = 0;
        (*dest).unknown_chunks_size[i as usize] = (*src).unknown_chunks_size[i as usize];
        let ref mut fresh90 = (*dest).unknown_chunks_data[i as usize];
        *fresh90 = lodepng_malloc((*src).unknown_chunks_size[i as usize])
            as *mut std::os::raw::c_uchar;
        if ((*dest).unknown_chunks_data[i as usize]).is_null()
            && (*dest).unknown_chunks_size[i as usize] != 0
        {
            return 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        j = 0 as std::os::raw::c_int as size_t;
        while j < (*src).unknown_chunks_size[i as usize] {
            *((*dest).unknown_chunks_data[i as usize])
                .offset(
                    j as isize,
                ) = *((*src).unknown_chunks_data[i as usize]).offset(j as isize);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn LodePNGText_init(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    (*info).text_num = 0 as std::os::raw::c_int as size_t;
    let ref mut fresh91 = (*info).text_keys;
    *fresh91 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh92 = (*info).text_strings;
    *fresh92 = 0 as *mut *mut std::os::raw::c_char;
}
unsafe extern "C" fn LodePNGText_cleanup(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i != (*info).text_num {
        string_cleanup(Some(&mut *((*info).text_keys).offset(i as isize)));
        string_cleanup(Some(&mut *((*info).text_strings).offset(i as isize)));
        i = i.wrapping_add(1);
    }
    lodepng_free((*info).text_keys as *mut std::os::raw::c_void);
    lodepng_free((*info).text_strings as *mut std::os::raw::c_void);
}
unsafe extern "C" fn LodePNGText_copy(
    mut dest: * mut crate::src::lodepng::LodePNGInfo,
    mut source: * const crate::src::lodepng::LodePNGInfo,
) -> std::os::raw::c_uint {
    let mut i = 0 as std::os::raw::c_int as size_t;
    let ref mut fresh93 = (*dest).text_keys;
    *fresh93 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh94 = (*dest).text_strings;
    *fresh94 = 0 as *mut *mut std::os::raw::c_char;
    (*dest).text_num = 0 as std::os::raw::c_int as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i != (*source).text_num {
        let mut error = lodepng_add_text(
            dest,
            *((*source).text_keys).offset(i as isize),
            *((*source).text_strings).offset(i as isize),
        );
        if error != 0 {
            return error;
        }
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn lodepng_add_text_sized(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut key: * const std::os::raw::c_char,
    mut str: * const std::os::raw::c_char,
    mut size: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut new_keys = lodepng_realloc(
        (*info).text_keys as *mut std::os::raw::c_void,
        (::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong)
            .wrapping_mul(
                ((*info).text_num).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ),
    ) as *mut *mut std::os::raw::c_char;
    let mut new_strings = lodepng_realloc(
        (*info).text_strings as *mut std::os::raw::c_void,
        (::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong)
            .wrapping_mul(
                ((*info).text_num).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ),
    ) as *mut *mut std::os::raw::c_char;
    if !new_keys.is_null() {
        let ref mut fresh95 = (*info).text_keys;
        *fresh95 = new_keys;
    }
    if !new_strings.is_null() {
        let ref mut fresh96 = (*info).text_strings;
        *fresh96 = new_strings;
    }
    if new_keys.is_null() || new_strings.is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let ref mut fresh97 = (*info).text_num;
    *fresh97 = (*fresh97).wrapping_add(1);
    let ref mut fresh98 = *((*info).text_keys)
        .offset(
            ((*info).text_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        );
    *fresh98 = alloc_string(key);
    let ref mut fresh99 = *((*info).text_strings)
        .offset(
            ((*info).text_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        );
    *fresh99 = alloc_string_sized(str, size);
    if (*((*info).text_keys)
        .offset(
            ((*info).text_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        ))
        .is_null()
        || (*((*info).text_strings)
            .offset(
                ((*info).text_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            ))
            .is_null()
    {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_add_text(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut key: * const std::os::raw::c_char,
    mut str: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    return lodepng_add_text_sized(info, key, str, lodepng_strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_clear_text(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    LodePNGText_cleanup(info);
}
unsafe extern "C" fn LodePNGIText_init(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    (*info).itext_num = 0 as std::os::raw::c_int as size_t;
    let ref mut fresh100 = (*info).itext_keys;
    *fresh100 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh101 = (*info).itext_langtags;
    *fresh101 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh102 = (*info).itext_transkeys;
    *fresh102 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh103 = (*info).itext_strings;
    *fresh103 = 0 as *mut *mut std::os::raw::c_char;
}
unsafe extern "C" fn LodePNGIText_cleanup(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i != (*info).itext_num {
        string_cleanup(Some(&mut *((*info).itext_keys).offset(i as isize)));
        string_cleanup(Some(&mut *((*info).itext_langtags).offset(i as isize)));
        string_cleanup(Some(&mut *((*info).itext_transkeys).offset(i as isize)));
        string_cleanup(Some(&mut *((*info).itext_strings).offset(i as isize)));
        i = i.wrapping_add(1);
    }
    lodepng_free((*info).itext_keys as *mut std::os::raw::c_void);
    lodepng_free((*info).itext_langtags as *mut std::os::raw::c_void);
    lodepng_free((*info).itext_transkeys as *mut std::os::raw::c_void);
    lodepng_free((*info).itext_strings as *mut std::os::raw::c_void);
}
unsafe extern "C" fn LodePNGIText_copy(
    mut dest: * mut crate::src::lodepng::LodePNGInfo,
    mut source: * const crate::src::lodepng::LodePNGInfo,
) -> std::os::raw::c_uint {
    let mut i = 0 as std::os::raw::c_int as size_t;
    let ref mut fresh104 = (*dest).itext_keys;
    *fresh104 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh105 = (*dest).itext_langtags;
    *fresh105 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh106 = (*dest).itext_transkeys;
    *fresh106 = 0 as *mut *mut std::os::raw::c_char;
    let ref mut fresh107 = (*dest).itext_strings;
    *fresh107 = 0 as *mut *mut std::os::raw::c_char;
    (*dest).itext_num = 0 as std::os::raw::c_int as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i != (*source).itext_num {
        let mut error = lodepng_add_itext(
            dest,
            *((*source).itext_keys).offset(i as isize),
            *((*source).itext_langtags).offset(i as isize),
            *((*source).itext_transkeys).offset(i as isize),
            *((*source).itext_strings).offset(i as isize),
        );
        if error != 0 {
            return error;
        }
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_clear_itext(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    LodePNGIText_cleanup(info);
}
unsafe extern "C" fn lodepng_add_itext_sized(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut key: * const std::os::raw::c_char,
    mut langtag: * const std::os::raw::c_char,
    mut transkey: * const std::os::raw::c_char,
    mut str: * const std::os::raw::c_char,
    mut size: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut new_keys = lodepng_realloc(
        (*info).itext_keys as *mut std::os::raw::c_void,
        (::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong)
            .wrapping_mul(
                ((*info).itext_num).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ),
    ) as *mut *mut std::os::raw::c_char;
    let mut new_langtags = lodepng_realloc(
        (*info).itext_langtags as *mut std::os::raw::c_void,
        (::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong)
            .wrapping_mul(
                ((*info).itext_num).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ),
    ) as *mut *mut std::os::raw::c_char;
    let mut new_transkeys = lodepng_realloc(
        (*info).itext_transkeys as *mut std::os::raw::c_void,
        (::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong)
            .wrapping_mul(
                ((*info).itext_num).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ),
    ) as *mut *mut std::os::raw::c_char;
    let mut new_strings = lodepng_realloc(
        (*info).itext_strings as *mut std::os::raw::c_void,
        (::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong)
            .wrapping_mul(
                ((*info).itext_num).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ),
    ) as *mut *mut std::os::raw::c_char;
    if !new_keys.is_null() {
        let ref mut fresh108 = (*info).itext_keys;
        *fresh108 = new_keys;
    }
    if !new_langtags.is_null() {
        let ref mut fresh109 = (*info).itext_langtags;
        *fresh109 = new_langtags;
    }
    if !new_transkeys.is_null() {
        let ref mut fresh110 = (*info).itext_transkeys;
        *fresh110 = new_transkeys;
    }
    if !new_strings.is_null() {
        let ref mut fresh111 = (*info).itext_strings;
        *fresh111 = new_strings;
    }
    if new_keys.is_null() || new_langtags.is_null() || new_transkeys.is_null()
        || new_strings.is_null()
    {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let ref mut fresh112 = (*info).itext_num;
    *fresh112 = (*fresh112).wrapping_add(1);
    let ref mut fresh113 = *((*info).itext_keys)
        .offset(
            ((*info).itext_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        );
    *fresh113 = alloc_string(key);
    let ref mut fresh114 = *((*info).itext_langtags)
        .offset(
            ((*info).itext_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        );
    *fresh114 = alloc_string(langtag);
    let ref mut fresh115 = *((*info).itext_transkeys)
        .offset(
            ((*info).itext_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        );
    *fresh115 = alloc_string(transkey);
    let ref mut fresh116 = *((*info).itext_strings)
        .offset(
            ((*info).itext_num).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
        );
    *fresh116 = alloc_string_sized(str, size);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_add_itext(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut key: * const std::os::raw::c_char,
    mut langtag: * const std::os::raw::c_char,
    mut transkey: * const std::os::raw::c_char,
    mut str: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    return lodepng_add_itext_sized(
        info,
        key,
        langtag,
        transkey,
        str,
        lodepng_strlen(str),
    );
}
unsafe extern "C" fn lodepng_assign_icc(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut name: * const std::os::raw::c_char,
    mut profile: * const std::os::raw::c_uchar,
    mut profile_size: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    if profile_size == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return 100 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let ref mut fresh117 = (*info).iccp_name;
    *fresh117 = alloc_string(name);
    let ref mut fresh118 = (*info).iccp_profile;
    *fresh118 = lodepng_malloc(profile_size as size_t) as *mut std::os::raw::c_uchar;
    if ((*info).iccp_name).is_null() || ((*info).iccp_profile).is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    lodepng_memcpy(
        (*info).iccp_profile as *mut std::os::raw::c_void,
        profile as *const std::os::raw::c_void,
        profile_size as size_t,
    );
    (*info).iccp_profile_size = profile_size;
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_set_icc(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut name: * const std::os::raw::c_char,
    mut profile: * const std::os::raw::c_uchar,
    mut profile_size: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    if !((*info).iccp_name).is_null() {
        lodepng_clear_icc(info);
    }
    (*info).iccp_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    return lodepng_assign_icc(info, name, profile, profile_size);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_clear_icc(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    string_cleanup(Some(&mut (*info).iccp_name));
    lodepng_free((*info).iccp_profile as *mut std::os::raw::c_void);
    let ref mut fresh119 = (*info).iccp_profile;
    *fresh119 = 0 as *mut std::os::raw::c_uchar;
    (*info).iccp_profile_size = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).iccp_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_info_init(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    lodepng_color_mode_init(&mut (*info).color);
    (*info).interlace_method = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).compression_method = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).filter_method = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).background_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh120 = (*info).background_b;
    *fresh120 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh121 = (*info).background_g;
    *fresh121 = *fresh120;
    (*info).background_r = *fresh121;
    LodePNGText_init(info);
    LodePNGIText_init(info);
    (*info).time_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).phys_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).gama_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).chrm_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).srgb_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*info).iccp_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh122 = (*info).iccp_name;
    *fresh122 = 0 as *mut std::os::raw::c_char;
    let ref mut fresh123 = (*info).iccp_profile;
    *fresh123 = 0 as *mut std::os::raw::c_uchar;
    (*info).sbit_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh124 = (*info).sbit_a;
    *fresh124 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh125 = (*info).sbit_b;
    *fresh125 = *fresh124;
    let ref mut fresh126 = (*info).sbit_g;
    *fresh126 = *fresh125;
    (*info).sbit_r = *fresh126;
    LodePNGUnknownChunks_init(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_info_cleanup(mut info: * mut crate::src::lodepng::LodePNGInfo) {
    lodepng_color_mode_cleanup(&mut (*info).color);
    LodePNGText_cleanup(info);
    LodePNGIText_cleanup(info);
    lodepng_clear_icc(info);
    LodePNGUnknownChunks_cleanup(info);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_info_copy(
    mut dest: * mut crate::src::lodepng::LodePNGInfo,
    mut source: * const crate::src::lodepng::LodePNGInfo,
) -> std::os::raw::c_uint {
    lodepng_info_cleanup(dest);
    lodepng_memcpy(
        dest as *mut std::os::raw::c_void,
        source as *const std::os::raw::c_void,
        ::std::mem::size_of::<LodePNGInfo>() as std::os::raw::c_ulong,
    );
    lodepng_color_mode_init(&mut (*dest).color);
    let mut error = lodepng_color_mode_copy(&mut (*dest).color, &(*source).color);
    if error != 0 {
        return error;
    }
    let mut error_0 = LodePNGText_copy(dest, source);
    if error_0 != 0 {
        return error_0;
    }
    let mut error_1 = LodePNGIText_copy(dest, source);
    if error_1 != 0 {
        return error_1;
    }
    if (*source).iccp_defined != 0 {
        let mut error_2 = lodepng_assign_icc(
            dest,
            (*source).iccp_name,
            (*source).iccp_profile,
            (*source).iccp_profile_size,
        );
        if error_2 != 0 {
            return error_2;
        }
    }
    LodePNGUnknownChunks_init(dest);
    let mut error_3 = LodePNGUnknownChunks_copy(dest, source);
    if error_3 != 0 {
        return error_3;
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addColorBits(
    mut out: * mut std::os::raw::c_uchar,
    mut index: std::os::raw::c_ulong,
    mut bits: std::os::raw::c_uint,
    mut in_0: std::os::raw::c_uint,
) {
    let mut m = (if bits == 1 as std::os::raw::c_int as std::os::raw::c_uint {
        7 as std::os::raw::c_int
    } else if bits == 2 as std::os::raw::c_int as std::os::raw::c_uint {
        3 as std::os::raw::c_int
    } else {
        1 as std::os::raw::c_int
    }) as std::os::raw::c_uint;
    let mut p = (index & m as std::os::raw::c_ulong) as std::os::raw::c_uint;
    in_0 &= ((1 as std::os::raw::c_uint) << bits).wrapping_sub(1 as std::os::raw::c_uint);
    in_0 = in_0 << bits.wrapping_mul(m.wrapping_sub(p));
    if p == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        *out
            .offset(
                index
                    .wrapping_mul(bits as std::os::raw::c_ulong)
                    .wrapping_div(8 as std::os::raw::c_uint as std::os::raw::c_ulong) as isize,
            ) = in_0 as std::os::raw::c_uchar;
    } else {
        let ref mut fresh127 = *out
            .offset(
                index
                    .wrapping_mul(bits as std::os::raw::c_ulong)
                    .wrapping_div(8 as std::os::raw::c_uint as std::os::raw::c_ulong) as isize,
            );
        *fresh127 = (*fresh127 as std::os::raw::c_uint | in_0) as std::os::raw::c_uchar;
    };
}
unsafe extern "C" fn color_tree_init(mut tree: * mut crate::src::lodepng::ColorTree) {
    lodepng_memset(
        ((*tree).children).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (16 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut ColorTree>() as std::os::raw::c_ulong),
    );
    (*tree).index = -(1 as std::os::raw::c_int);
}
unsafe extern "C" fn color_tree_cleanup(mut tree: * mut crate::src::lodepng::ColorTree) {
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i != 16 as std::os::raw::c_int {
        if !((*tree).children[i as usize]).is_null() {
            color_tree_cleanup((*tree).children[i as usize]);
            lodepng_free((*tree).children[i as usize] as *mut std::os::raw::c_void);
        }
        i += 1;
    }
}
unsafe extern "C" fn color_tree_get(
    mut tree: * mut crate::src::lodepng::ColorTree,
    mut r: std::os::raw::c_uchar,
    mut g: std::os::raw::c_uchar,
    mut b: std::os::raw::c_uchar,
    mut a: std::os::raw::c_uchar,
) -> std::os::raw::c_int {
    let mut bit = 0 as std::os::raw::c_int;
    bit = 0 as std::os::raw::c_int;
    while bit < 8 as std::os::raw::c_int {
        let mut i = 8 as std::os::raw::c_int * (r as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int)
            + 4 as std::os::raw::c_int * (g as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int)
            + 2 as std::os::raw::c_int * (b as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int)
            + 1 as std::os::raw::c_int * (a as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int);
        if ((*tree).children[i as usize]).is_null() {
            return -(1 as std::os::raw::c_int)
        } else {
            tree = (*tree).children[i as usize];
        }
        bit += 1;
    }
    return if !tree.is_null() { (*tree).index } else { -(1 as std::os::raw::c_int) };
}
unsafe extern "C" fn color_tree_has(
    mut tree: * mut crate::src::lodepng::ColorTree,
    mut r: std::os::raw::c_uchar,
    mut g: std::os::raw::c_uchar,
    mut b: std::os::raw::c_uchar,
    mut a: std::os::raw::c_uchar,
) -> std::os::raw::c_int {
    return (color_tree_get(tree, r, g, b, a) >= 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
unsafe extern "C" fn color_tree_add(
    mut tree: * mut crate::src::lodepng::ColorTree,
    mut r: std::os::raw::c_uchar,
    mut g: std::os::raw::c_uchar,
    mut b: std::os::raw::c_uchar,
    mut a: std::os::raw::c_uchar,
    mut index: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut bit: i32 = 0;
    bit = 0 as std::os::raw::c_int;
    while bit < 8 as std::os::raw::c_int {
        let mut i = 8 as std::os::raw::c_int * (r as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int)
            + 4 as std::os::raw::c_int * (g as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int)
            + 2 as std::os::raw::c_int * (b as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int)
            + 1 as std::os::raw::c_int * (a as std::os::raw::c_int >> bit & 1 as std::os::raw::c_int);
        if ((*tree).children[i as usize]).is_null() {
            let ref mut fresh128 = (*tree).children[i as usize];
            *fresh128 = lodepng_malloc(
                ::std::mem::size_of::<ColorTree>() as std::os::raw::c_ulong,
            ) as *mut ColorTree;
            if ((*tree).children[i as usize]).is_null() {
                return 83 as std::os::raw::c_int as std::os::raw::c_uint;
            }
            color_tree_init((*tree).children[i as usize]);
        }
        tree = (*tree).children[i as usize];
        bit += 1;
    }
    (*tree).index = index as std::os::raw::c_int;
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn rgba8ToPixel<'a1>(
    mut out: * mut std::os::raw::c_uchar,
    mut i: std::os::raw::c_ulong,
    mut mode: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
    mut tree: * mut crate::src::lodepng::ColorTree,
    mut r: std::os::raw::c_uchar,
    mut g: std::os::raw::c_uchar,
    mut b: std::os::raw::c_uchar,
    mut a: std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint {
        let mut gray = r;
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *out.offset(i as isize) = gray;
        } else if (*((mode).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
            let ref mut fresh129 = *out
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh129 = gray;
            *out
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh129;
        } else {
            gray = (gray as std::os::raw::c_uint
                >> (8 as std::os::raw::c_uint).wrapping_sub((*((mode).clone()).unwrap()).bitdepth)
                & ((1 as std::os::raw::c_uint) << (*((mode).clone()).unwrap()).bitdepth)
                    .wrapping_sub(1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
            addColorBits(out, i, (*(mode).unwrap()).bitdepth, gray as std::os::raw::c_uint);
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *out
                .offset(
                    i
                        .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = r;
            *out
                .offset(
                    i
                        .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = g;
            *out
                .offset(
                    i
                        .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = b;
        } else {
            let ref mut fresh130 = *out
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh130 = r;
            *out
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh130;
            let ref mut fresh131 = *out
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh131 = g;
            *out
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh131;
            let ref mut fresh132 = *out
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh132 = b;
            *out
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh132;
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut index = color_tree_get(tree, r, g, b, a);
        if index < 0 as std::os::raw::c_int {
            return 82 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *out.offset(i as isize) = index as std::os::raw::c_uchar;
        } else {
            addColorBits(out, i, (*(mode).unwrap()).bitdepth, index as std::os::raw::c_uint);
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut gray_0 = r;
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *out
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = gray_0;
            *out
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = a;
        } else if (*((mode).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
            let ref mut fresh133 = *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh133 = gray_0;
            *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh133;
            let ref mut fresh134 = *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh134 = a;
            *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh134;
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = r;
            *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = g;
            *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = b;
            *out
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = a;
        } else {
            let ref mut fresh135 = *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh135 = r;
            *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh135;
            let ref mut fresh136 = *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh136 = g;
            *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh136;
            let ref mut fresh137 = *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh137 = b;
            *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh137;
            let ref mut fresh138 = *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *fresh138 = a;
            *out
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) = *fresh138;
        }
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn rgba16ToPixel<'a1>(
    mut out: * mut std::os::raw::c_uchar,
    mut i: std::os::raw::c_ulong,
    mut mode: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
    mut r: std::os::raw::c_ushort,
    mut g: std::os::raw::c_ushort,
    mut b: std::os::raw::c_ushort,
    mut a: std::os::raw::c_ushort,
) {
    if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint {
        let mut gray = r;
        *out
            .offset(
                i
                    .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (gray as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (gray as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        *out
            .offset(
                i
                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (r as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (r as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (g as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (g as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (b as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (b as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut gray_0 = r;
        *out
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (gray_0 as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (gray_0 as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (a as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (a as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (r as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (r as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (g as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (g as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (b as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (b as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (a as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        *out
            .offset(
                i
                    .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = (a as std::os::raw::c_int & 255 as std::os::raw::c_int) as std::os::raw::c_uchar;
    }
}
unsafe extern "C" fn getPixelColorRGBA8<'a1, 'a2, 'a3, 'a4, 'a5>(
    mut r: Option<&'a1 mut std::os::raw::c_uchar>,
    mut g: Option<&'a2 mut std::os::raw::c_uchar>,
    mut b: Option<&'a3 mut std::os::raw::c_uchar>,
    mut a: Option<&'a4 mut std::os::raw::c_uchar>,
    mut in_0: * const std::os::raw::c_uchar,
    mut i: std::os::raw::c_ulong,
    mut mode: Option<&'a5 crate::src::lodepng::LodePNGColorMode>,
) {
    if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *(borrow_mut(&mut b)).unwrap() = *in_0.offset(i as isize);
            *(borrow_mut(&mut g)).unwrap() = *(borrow_mut(&mut b)).unwrap();
            *(borrow_mut(&mut r)).unwrap() = *(borrow_mut(&mut g)).unwrap();
            if (*((mode).clone()).unwrap()).key_defined != 0 && *(borrow(& r)).unwrap() as std::os::raw::c_uint == (*((mode).clone()).unwrap()).key_r {
                *(borrow_mut(&mut a)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            } else {
                *(borrow_mut(&mut a)).unwrap() = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
            }
        } else if (*((mode).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
            *(borrow_mut(&mut b)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut g)).unwrap() = *(borrow_mut(&mut b)).unwrap();
            *(borrow_mut(&mut r)).unwrap() = *(borrow_mut(&mut g)).unwrap();
            if (*((mode).clone()).unwrap()).key_defined != 0
                && (256 as std::os::raw::c_uint)
                    .wrapping_mul(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    )
                    .wrapping_add(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    ) == (*((mode).clone()).unwrap()).key_r
            {
                *(borrow_mut(&mut a)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            } else {
                *(borrow_mut(&mut a)).unwrap() = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
            }
        } else {
            let mut highest = ((1 as std::os::raw::c_uint) << (*((mode).clone()).unwrap()).bitdepth)
                .wrapping_sub(1 as std::os::raw::c_uint);
            let mut j = i.wrapping_mul((*((mode).clone()).unwrap()).bitdepth as std::os::raw::c_ulong);
            let mut value = readBitsFromReversedStream(
                Some(&mut j),
                in_0,
                (*(mode).unwrap()).bitdepth as size_t,
            );
            *(borrow_mut(&mut b)).unwrap() = value
                .wrapping_mul(255 as std::os::raw::c_int as std::os::raw::c_uint)
                .wrapping_div(highest) as std::os::raw::c_uchar;
            *(borrow_mut(&mut g)).unwrap() = *(borrow_mut(&mut b)).unwrap();
            *(borrow_mut(&mut r)).unwrap() = *(borrow_mut(&mut g)).unwrap();
            if (*((mode).clone()).unwrap()).key_defined != 0 && value == (*((mode).clone()).unwrap()).key_r {
                *(borrow_mut(&mut a)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            } else {
                *(borrow_mut(&mut a)).unwrap() = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *(borrow_mut(&mut r)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut g)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut b)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            if (*((mode).clone()).unwrap()).key_defined != 0 && *(borrow(& r)).unwrap() as std::os::raw::c_uint == (*((mode).clone()).unwrap()).key_r
                && *(borrow(& g)).unwrap() as std::os::raw::c_uint == (*((mode).clone()).unwrap()).key_g
                && *(borrow(& b)).unwrap() as std::os::raw::c_uint == (*((mode).clone()).unwrap()).key_b
            {
                *(borrow_mut(&mut a)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            } else {
                *(borrow_mut(&mut a)).unwrap() = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
            }
        } else {
            *(borrow_mut(&mut r)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut g)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut b)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            if (*((mode).clone()).unwrap()).key_defined != 0
                && (256 as std::os::raw::c_uint)
                    .wrapping_mul(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    )
                    .wrapping_add(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    ) == (*((mode).clone()).unwrap()).key_r
                && (256 as std::os::raw::c_uint)
                    .wrapping_mul(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    )
                    .wrapping_add(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    ) == (*((mode).clone()).unwrap()).key_g
                && (256 as std::os::raw::c_uint)
                    .wrapping_mul(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    )
                    .wrapping_add(
                        *in_0
                            .offset(
                                i
                                    .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) as std::os::raw::c_uint,
                    ) == (*((mode).clone()).unwrap()).key_b
            {
                *(borrow_mut(&mut a)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            } else {
                *(borrow_mut(&mut a)).unwrap() = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut index: u32 = 0;
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            index = *in_0.offset(i as isize) as std::os::raw::c_uint;
        } else {
            let mut j_0 = i.wrapping_mul((*((mode).clone()).unwrap()).bitdepth as std::os::raw::c_ulong);
            index = readBitsFromReversedStream(
                Some(&mut j_0),
                in_0,
                (*(mode).unwrap()).bitdepth as size_t,
            );
        }
        *(borrow_mut(&mut r)).unwrap() = *((*((mode).clone()).unwrap()).palette)
            .offset(
                index
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            );
        *(borrow_mut(&mut g)).unwrap() = *((*((mode).clone()).unwrap()).palette)
            .offset(
                index
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            );
        *(borrow_mut(&mut b)).unwrap() = *((*((mode).clone()).unwrap()).palette)
            .offset(
                index
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            );
        *(borrow_mut(&mut a)).unwrap() = *((*((mode).clone()).unwrap()).palette)
            .offset(
                index
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            );
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *(borrow_mut(&mut b)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut g)).unwrap() = *(borrow_mut(&mut b)).unwrap();
            *(borrow_mut(&mut r)).unwrap() = *(borrow_mut(&mut g)).unwrap();
            *(borrow_mut(&mut a)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
        } else {
            *(borrow_mut(&mut b)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut g)).unwrap() = *(borrow_mut(&mut b)).unwrap();
            *(borrow_mut(&mut r)).unwrap() = *(borrow_mut(&mut g)).unwrap();
            *(borrow_mut(&mut a)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            *(borrow_mut(&mut r)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut g)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut b)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut a)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
        } else {
            *(borrow_mut(&mut r)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut g)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut b)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            *(borrow_mut(&mut a)).unwrap() = *in_0
                .offset(
                    i
                        .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
        }
    }
}
unsafe extern "C" fn getPixelColorsRGBA8<'a1>(
    mut buffer: * mut std::os::raw::c_uchar,
    mut numpixels: std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut mode: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) {
    let mut num_channels = 4 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u64 = 0;
    if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh139 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh139 = *in_0.offset(i as isize);
                let ref mut fresh140 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh140 = *fresh139;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh140;
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
            if (*((mode).clone()).unwrap()).key_defined != 0 {
                buffer = buffer
                    .offset(
                        -(numpixels.wrapping_mul(num_channels as std::os::raw::c_ulong) as isize),
                    );
                i = 0 as std::os::raw::c_int as size_t;
                while i != numpixels {
                    if *buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint
                        == (*((mode).clone()).unwrap()).key_r
                    {
                        *buffer
                            .offset(
                                3 as std::os::raw::c_int as isize,
                            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                    }
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*((mode).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh141 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh141 = *in_0
                    .offset(i.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize);
                let ref mut fresh142 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh142 = *fresh141;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh142;
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = (if (*((mode).clone()).unwrap()).key_defined != 0
                    && (256 as std::os::raw::c_uint)
                        .wrapping_mul(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        )
                        .wrapping_add(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        ) == (*((mode).clone()).unwrap()).key_r
                {
                    0 as std::os::raw::c_int
                } else {
                    255 as std::os::raw::c_int
                }) as std::os::raw::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut highest = ((1 as std::os::raw::c_uint) << (*((mode).clone()).unwrap()).bitdepth)
                .wrapping_sub(1 as std::os::raw::c_uint);
            let mut j = 0 as std::os::raw::c_int as size_t;
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let mut value = readBitsFromReversedStream(
                    Some(&mut j),
                    in_0,
                    (*(mode).unwrap()).bitdepth as size_t,
                );
                let ref mut fresh143 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh143 = value
                    .wrapping_mul(255 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_div(highest) as std::os::raw::c_uchar;
                let ref mut fresh144 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh144 = *fresh143;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh144;
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = (if (*((mode).clone()).unwrap()).key_defined != 0 && value == (*((mode).clone()).unwrap()).key_r {
                    0 as std::os::raw::c_int
                } else {
                    255 as std::os::raw::c_int
                }) as std::os::raw::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                lodepng_memcpy(
                    buffer as *mut std::os::raw::c_void,
                    &*in_0
                        .offset(
                            i.wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as *const std::os::raw::c_uchar as *const std::os::raw::c_void,
                    3 as std::os::raw::c_int as size_t,
                );
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
            if (*((mode).clone()).unwrap()).key_defined != 0 {
                buffer = buffer
                    .offset(
                        -(numpixels.wrapping_mul(num_channels as std::os::raw::c_ulong) as isize),
                    );
                i = 0 as std::os::raw::c_int as size_t;
                while i != numpixels {
                    if *buffer.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint
                        == (*((mode).clone()).unwrap()).key_r
                        && *buffer.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint
                            == (*((mode).clone()).unwrap()).key_g
                        && *buffer.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint
                            == (*((mode).clone()).unwrap()).key_b
                    {
                        *buffer
                            .offset(
                                3 as std::os::raw::c_int as isize,
                            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                    }
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                *buffer
                    .offset(
                        0 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        1 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        2 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = (if (*((mode).clone()).unwrap()).key_defined != 0
                    && (256 as std::os::raw::c_uint)
                        .wrapping_mul(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        )
                        .wrapping_add(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        ) == (*((mode).clone()).unwrap()).key_r
                    && (256 as std::os::raw::c_uint)
                        .wrapping_mul(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        )
                        .wrapping_add(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        ) == (*((mode).clone()).unwrap()).key_g
                    && (256 as std::os::raw::c_uint)
                        .wrapping_mul(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        )
                        .wrapping_add(
                            *in_0
                                .offset(
                                    i
                                        .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_uint,
                        ) == (*((mode).clone()).unwrap()).key_b
                {
                    0 as std::os::raw::c_int
                } else {
                    255 as std::os::raw::c_int
                }) as std::os::raw::c_uchar;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let mut index = *in_0.offset(i as isize) as std::os::raw::c_uint;
                lodepng_memcpy(
                    buffer as *mut std::os::raw::c_void,
                    &mut *((*((mode).clone()).unwrap()).palette)
                        .offset(
                            index.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                        ) as *mut std::os::raw::c_uchar as *const std::os::raw::c_void,
                    4 as std::os::raw::c_int as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut j_0 = 0 as std::os::raw::c_int as size_t;
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let mut index_0 = readBitsFromReversedStream(
                    Some(&mut j_0),
                    in_0,
                    (*(mode).unwrap()).bitdepth as size_t,
                );
                lodepng_memcpy(
                    buffer as *mut std::os::raw::c_void,
                    &mut *((*((mode).clone()).unwrap()).palette)
                        .offset(
                            index_0.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                                as isize,
                        ) as *mut std::os::raw::c_uchar as *const std::os::raw::c_void,
                    4 as std::os::raw::c_int as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh145 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh145 = *in_0
                    .offset(
                        i
                            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                let ref mut fresh146 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh146 = *fresh145;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh146;
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh147 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh147 = *in_0
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                let ref mut fresh148 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh148 = *fresh147;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh148;
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            lodepng_memcpy(
                buffer as *mut std::os::raw::c_void,
                in_0 as *const std::os::raw::c_void,
                numpixels.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong),
            );
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                *buffer
                    .offset(
                        0 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        1 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        2 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        3 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    }
}
unsafe extern "C" fn getPixelColorsRGB8<'a1>(
    mut buffer: * mut std::os::raw::c_uchar,
    mut numpixels: std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut mode: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) {
    let num_channels = 3 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u64 = 0;
    if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh149 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh149 = *in_0.offset(i as isize);
                let ref mut fresh150 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh150 = *fresh149;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh150;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else if (*((mode).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh151 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh151 = *in_0
                    .offset(i.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize);
                let ref mut fresh152 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh152 = *fresh151;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh152;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut highest = ((1 as std::os::raw::c_uint) << (*((mode).clone()).unwrap()).bitdepth)
                .wrapping_sub(1 as std::os::raw::c_uint);
            let mut j = 0 as std::os::raw::c_int as size_t;
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let mut value = readBitsFromReversedStream(
                    Some(&mut j),
                    in_0,
                    (*(mode).unwrap()).bitdepth as size_t,
                );
                let ref mut fresh153 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh153 = value
                    .wrapping_mul(255 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_div(highest) as std::os::raw::c_uchar;
                let ref mut fresh154 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh154 = *fresh153;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh154;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            lodepng_memcpy(
                buffer as *mut std::os::raw::c_void,
                in_0 as *const std::os::raw::c_void,
                numpixels.wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong),
            );
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                *buffer
                    .offset(
                        0 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        1 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        2 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let mut index = *in_0.offset(i as isize) as std::os::raw::c_uint;
                lodepng_memcpy(
                    buffer as *mut std::os::raw::c_void,
                    &mut *((*((mode).clone()).unwrap()).palette)
                        .offset(
                            index.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                        ) as *mut std::os::raw::c_uchar as *const std::os::raw::c_void,
                    3 as std::os::raw::c_int as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            let mut j_0 = 0 as std::os::raw::c_int as size_t;
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let mut index_0 = readBitsFromReversedStream(
                    Some(&mut j_0),
                    in_0,
                    (*(mode).unwrap()).bitdepth as size_t,
                );
                lodepng_memcpy(
                    buffer as *mut std::os::raw::c_void,
                    &mut *((*((mode).clone()).unwrap()).palette)
                        .offset(
                            index_0.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                                as isize,
                        ) as *mut std::os::raw::c_uchar as *const std::os::raw::c_void,
                    3 as std::os::raw::c_int as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh155 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh155 = *in_0
                    .offset(
                        i
                            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                let ref mut fresh156 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh156 = *fresh155;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh156;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let ref mut fresh157 = *buffer.offset(2 as std::os::raw::c_int as isize);
                *fresh157 = *in_0
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                let ref mut fresh158 = *buffer.offset(1 as std::os::raw::c_int as isize);
                *fresh158 = *fresh157;
                *buffer.offset(0 as std::os::raw::c_int as isize) = *fresh158;
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((mode).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                lodepng_memcpy(
                    buffer as *mut std::os::raw::c_void,
                    &*in_0
                        .offset(
                            i.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as *const std::os::raw::c_uchar as *const std::os::raw::c_void,
                    3 as std::os::raw::c_int as size_t,
                );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        } else {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                *buffer
                    .offset(
                        0 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        1 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                *buffer
                    .offset(
                        2 as std::os::raw::c_int as isize,
                    ) = *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                i = i.wrapping_add(1);
                buffer = buffer.offset(num_channels as isize);
            }
        }
    }
}
unsafe extern "C" fn getPixelColorRGBA16<'a1, 'a2, 'a3, 'a4, 'a5>(
    mut r: Option<&'a1 mut std::os::raw::c_ushort>,
    mut g: Option<&'a2 mut std::os::raw::c_ushort>,
    mut b: Option<&'a3 mut std::os::raw::c_ushort>,
    mut a: Option<&'a4 mut std::os::raw::c_ushort>,
    mut in_0: * const std::os::raw::c_uchar,
    mut i: std::os::raw::c_ulong,
    mut mode: Option<&'a5 crate::src::lodepng::LodePNGColorMode>,
) {
    if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint {
        *(borrow_mut(&mut b)).unwrap() = (256 as std::os::raw::c_int
            * *in_0
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) as std::os::raw::c_int
            + *in_0
                .offset(
                    i
                        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) as std::os::raw::c_int) as std::os::raw::c_ushort;
        *(borrow_mut(&mut g)).unwrap() = *(borrow_mut(&mut b)).unwrap();
        *(borrow_mut(&mut r)).unwrap() = *(borrow_mut(&mut g)).unwrap();
        if (*((mode).clone()).unwrap()).key_defined != 0
            && (256 as std::os::raw::c_uint)
                .wrapping_mul(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                )
                .wrapping_add(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                ) == (*((mode).clone()).unwrap()).key_r
        {
            *(borrow_mut(&mut a)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
        } else {
            *(borrow_mut(&mut a)).unwrap() = 65535 as std::os::raw::c_int as std::os::raw::c_ushort;
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        *(borrow_mut(&mut r)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
        *(borrow_mut(&mut g)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
        *(borrow_mut(&mut b)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
        if (*((mode).clone()).unwrap()).key_defined != 0
            && (256 as std::os::raw::c_uint)
                .wrapping_mul(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                )
                .wrapping_add(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                ) == (*((mode).clone()).unwrap()).key_r
            && (256 as std::os::raw::c_uint)
                .wrapping_mul(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                )
                .wrapping_add(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                ) == (*((mode).clone()).unwrap()).key_g
            && (256 as std::os::raw::c_uint)
                .wrapping_mul(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                )
                .wrapping_add(
                    *in_0
                        .offset(
                            i
                                .wrapping_mul(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                        ) as std::os::raw::c_uint,
                ) == (*((mode).clone()).unwrap()).key_b
        {
            *(borrow_mut(&mut a)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
        } else {
            *(borrow_mut(&mut a)).unwrap() = 65535 as std::os::raw::c_int as std::os::raw::c_ushort;
        }
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        *(borrow_mut(&mut b)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
        *(borrow_mut(&mut g)).unwrap() = *(borrow_mut(&mut b)).unwrap();
        *(borrow_mut(&mut r)).unwrap() = *(borrow_mut(&mut g)).unwrap();
        *(borrow_mut(&mut a)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
    } else if (*((mode).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        *(borrow_mut(&mut r)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
        *(borrow_mut(&mut g)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
        *(borrow_mut(&mut b)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
        *(borrow_mut(&mut a)).unwrap() = (256 as std::os::raw::c_uint)
            .wrapping_mul(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            )
            .wrapping_add(
                *in_0
                    .offset(
                        i
                            .wrapping_mul(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_uint,
            ) as std::os::raw::c_ushort;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_convert<'a1, 'a2>(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * const std::os::raw::c_uchar,
    mut mode_out: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
    mut mode_in: Option<&'a2 crate::src::lodepng::LodePNGColorMode>,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut i: u64 = 0;
    let mut tree = ColorTree {
        children: [0 as *mut ColorTree; 16],
        index: 0,
    };
    let mut numpixels = (w as size_t).wrapping_mul(h as size_t);
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if (*((mode_in).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
        && ((*((mode_in).clone()).unwrap()).palette).is_null()
    {
        return 107 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if lodepng_color_mode_equal((mode_out).clone(), (mode_in).clone()) != 0 {
        let mut numbytes = lodepng_get_raw_size(w, h, (mode_in).clone());
        lodepng_memcpy(out as *mut std::os::raw::c_void, in_0 as *const std::os::raw::c_void, numbytes);
        return 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut palettesize = (*(mode_out).unwrap()).palettesize;
        let mut palette: * const u8 = (*((mode_out).clone()).unwrap()).palette;
        let mut palsize = (1 as std::os::raw::c_uint as size_t) << (*((mode_out).clone()).unwrap()).bitdepth;
        if palettesize == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            palettesize = (*(mode_in).unwrap()).palettesize;
            palette = (*((mode_in).clone()).unwrap()).palette;
            if (*((mode_in).clone()).unwrap()).colortype as std::os::raw::c_uint
                == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
                && (*((mode_in).clone()).unwrap()).bitdepth == (*((mode_out).clone()).unwrap()).bitdepth
            {
                let mut numbytes_0 = lodepng_get_raw_size(w, h, (mode_in).clone());
                lodepng_memcpy(
                    out as *mut std::os::raw::c_void,
                    in_0 as *const std::os::raw::c_void,
                    numbytes_0,
                );
                return 0 as std::os::raw::c_int as std::os::raw::c_uint;
            }
        }
        if palettesize < palsize {
            palsize = palettesize;
        }
        color_tree_init(&mut tree);
        i = 0 as std::os::raw::c_int as size_t;
        while i != palsize {
            let mut p: * const u8 = &*palette
                .offset(i.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as *const std::os::raw::c_uchar;
            error = color_tree_add(
                &mut tree,
                *p.offset(0 as std::os::raw::c_int as isize),
                *p.offset(1 as std::os::raw::c_int as isize),
                *p.offset(2 as std::os::raw::c_int as isize),
                *p.offset(3 as std::os::raw::c_int as isize),
                i as std::os::raw::c_uint,
            );
            if error != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    if error == 0 {
        if (*((mode_in).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint
            && (*((mode_out).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint
        {
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                let mut r = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut g = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut b = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut a = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                getPixelColorRGBA16(Some(&mut r), Some(&mut g), Some(&mut b), Some(&mut a), in_0, i, (mode_in).clone());
                rgba16ToPixel(out, i, (mode_out).clone(), r, g, b, a);
                i = i.wrapping_add(1);
            }
        } else if (*((mode_out).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint
            && (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
                == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
        {
            getPixelColorsRGBA8(out, numpixels, in_0, (mode_in).clone());
        } else if (*((mode_out).clone()).unwrap()).bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint
            && (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
                == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
        {
            getPixelColorsRGB8(out, numpixels, in_0, (mode_in).clone());
        } else {
            let mut r_0 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            let mut g_0 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            let mut b_0 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            let mut a_0 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
            i = 0 as std::os::raw::c_int as size_t;
            while i != numpixels {
                getPixelColorRGBA8(
                    Some(&mut r_0),
                    Some(&mut g_0),
                    Some(&mut b_0),
                    Some(&mut a_0),
                    in_0,
                    i,
                    (mode_in).clone(),
                );
                error = rgba8ToPixel(out, i, (mode_out).clone(), &mut tree, r_0, g_0, b_0, a_0);
                if error != 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
    }
    if (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        color_tree_cleanup(&mut tree);
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_convert_rgb<'a1, 'a2, 'a3, 'a4, 'a5>(
    mut r_out: Option<&'a1 mut std::os::raw::c_uint>,
    mut g_out: Option<&'a2 mut std::os::raw::c_uint>,
    mut b_out: Option<&'a3 mut std::os::raw::c_uint>,
    mut r_in: std::os::raw::c_uint,
    mut g_in: std::os::raw::c_uint,
    mut b_in: std::os::raw::c_uint,
    mut mode_out: Option<&'a4 crate::src::lodepng::LodePNGColorMode>,
    mut mode_in: Option<&'a5 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    let mut r = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut g = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut b = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut mul = (65535 as std::os::raw::c_int as std::os::raw::c_uint)
        .wrapping_div(
            ((1 as std::os::raw::c_uint) << (*((mode_in).clone()).unwrap()).bitdepth).wrapping_sub(1 as std::os::raw::c_uint),
        );
    let mut shift = (16 as std::os::raw::c_int as std::os::raw::c_uint)
        .wrapping_sub((*((mode_out).clone()).unwrap()).bitdepth);
    if (*((mode_in).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
        || (*((mode_in).clone()).unwrap()).colortype as std::os::raw::c_uint
            == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        b = r_in.wrapping_mul(mul);
        g = b;
        r = g;
    } else if (*((mode_in).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
        || (*((mode_in).clone()).unwrap()).colortype as std::os::raw::c_uint
            == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        r = r_in.wrapping_mul(mul);
        g = g_in.wrapping_mul(mul);
        b = b_in.wrapping_mul(mul);
    } else if (*((mode_in).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        if r_in as std::os::raw::c_ulong >= (*((mode_in).clone()).unwrap()).palettesize {
            return 82 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        r = (*((*((mode_in).clone()).unwrap()).palette)
            .offset(
                r_in
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) as std::os::raw::c_uint)
            .wrapping_mul(257 as std::os::raw::c_uint);
        g = (*((*((mode_in).clone()).unwrap()).palette)
            .offset(
                r_in
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) as std::os::raw::c_uint)
            .wrapping_mul(257 as std::os::raw::c_uint);
        b = (*((*((mode_in).clone()).unwrap()).palette)
            .offset(
                r_in
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) as std::os::raw::c_uint)
            .wrapping_mul(257 as std::os::raw::c_uint);
    } else {
        return 31 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
        || (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
            == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        *(borrow_mut(&mut r_out)).unwrap() = r >> shift;
    } else if (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
        || (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
            == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        *(borrow_mut(&mut r_out)).unwrap() = r >> shift;
        *(borrow_mut(&mut g_out)).unwrap() = g >> shift;
        *(borrow_mut(&mut b_out)).unwrap() = b >> shift;
    } else if (*((mode_out).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut i: u32 = 0;
        if r >> 8 as std::os::raw::c_int != r & 255 as std::os::raw::c_int as std::os::raw::c_uint
            || g >> 8 as std::os::raw::c_int != g & 255 as std::os::raw::c_int as std::os::raw::c_uint
            || b >> 8 as std::os::raw::c_int != b & 255 as std::os::raw::c_int as std::os::raw::c_uint
        {
            return 82 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while (i as std::os::raw::c_ulong) < (*((mode_out).clone()).unwrap()).palettesize {
            let mut j = i.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint);
            if r >> 8 as std::os::raw::c_int
                == *((*((mode_out).clone()).unwrap()).palette)
                    .offset(j.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint) as isize)
                    as std::os::raw::c_uint
                && g >> 8 as std::os::raw::c_int
                    == *((*((mode_out).clone()).unwrap()).palette)
                        .offset(
                            j.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                        ) as std::os::raw::c_uint
                && b >> 8 as std::os::raw::c_int
                    == *((*((mode_out).clone()).unwrap()).palette)
                        .offset(
                            j.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                        ) as std::os::raw::c_uint
            {
                *(borrow_mut(&mut r_out)).unwrap() = i;
                return 0 as std::os::raw::c_int as std::os::raw::c_uint;
            }
            i = i.wrapping_add(1);
        }
        return 82 as std::os::raw::c_int as std::os::raw::c_uint;
    } else {
        return 31 as std::os::raw::c_int as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_color_stats_init(mut stats: * mut crate::src::lodepng::LodePNGColorStats) {
    (*stats).colored = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*stats).key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh159 = (*stats).key_b;
    *fresh159 = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
    let ref mut fresh160 = (*stats).key_g;
    *fresh160 = *fresh159;
    (*stats).key_r = *fresh160;
    (*stats).alpha = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*stats).numcolors = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*stats).bits = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*stats).numpixels = 0 as std::os::raw::c_int as size_t;
    (*stats).allow_palette = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*stats).allow_greyscale = 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
 extern "C" fn getValueRequiredBits(mut value: std::os::raw::c_uchar) -> std::os::raw::c_uint {
    if value as std::os::raw::c_int == 0 as std::os::raw::c_int
        || value as std::os::raw::c_int == 255 as std::os::raw::c_int
    {
        return 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if value as std::os::raw::c_int % 17 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return (if value as std::os::raw::c_int % 85 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            2 as std::os::raw::c_int
        } else {
            4 as std::os::raw::c_int
        }) as std::os::raw::c_uint;
    }
    return 8 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_compute_color_stats<'a1>(
    mut stats: * mut crate::src::lodepng::LodePNGColorStats,
    mut in_0: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut mode_in: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    let mut current_block: u64;
    let mut i: u64 = 0;
    let mut tree = ColorTree {
        children: [0 as *mut ColorTree; 16],
        index: 0,
    };
    let mut numpixels = (w as size_t).wrapping_mul(h as size_t);
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut colored_done = (if lodepng_is_greyscale_type((mode_in).clone()) != 0 {
        1 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    }) as std::os::raw::c_uint;
    let mut alpha_done = (if lodepng_can_have_alpha((mode_in).clone()) != 0 {
        0 as std::os::raw::c_int
    } else {
        1 as std::os::raw::c_int
    }) as std::os::raw::c_uint;
    let mut numcolors_done = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut bpp = lodepng_get_bpp((mode_in).clone());
    let mut bits_done = (if (*stats).bits == 1 as std::os::raw::c_int as std::os::raw::c_uint
        && bpp == 1 as std::os::raw::c_int as std::os::raw::c_uint
    {
        1 as std::os::raw::c_int
    } else {
        0 as std::os::raw::c_int
    }) as std::os::raw::c_uint;
    let mut sixteen = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut maxnumcolors = 257 as std::os::raw::c_int as std::os::raw::c_uint;
    if bpp <= 8 as std::os::raw::c_int as std::os::raw::c_uint {
        maxnumcolors = if (257 as std::os::raw::c_int as std::os::raw::c_uint)
            < ((*stats).numcolors).wrapping_add((1 as std::os::raw::c_uint) << bpp)
        {
            257 as std::os::raw::c_int as std::os::raw::c_uint
        } else {
            ((*stats).numcolors).wrapping_add((1 as std::os::raw::c_uint) << bpp)
        };
    }
    let ref mut fresh161 = (*stats).numpixels;
    *fresh161 = (*fresh161 as std::os::raw::c_ulong).wrapping_add(numpixels) as size_t as size_t;
    if (*stats).allow_palette == 0 {
        numcolors_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    color_tree_init(&mut tree);
    if (*stats).alpha != 0 {
        alpha_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*stats).colored != 0 {
        colored_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*stats).bits == 16 as std::os::raw::c_int as std::os::raw::c_uint {
        numcolors_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*stats).bits >= bpp {
        bits_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*stats).numcolors >= maxnumcolors {
        numcolors_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if numcolors_done == 0 {
        i = 0 as std::os::raw::c_int as size_t;
        loop {
            if !(i < (*stats).numcolors as std::os::raw::c_ulong) {
                current_block = 15925075030174552612;
                break;
            }
            let mut color: * const u8 = &mut *((*stats).palette)
                .as_mut_ptr()
                .offset(i.wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as *mut std::os::raw::c_uchar;
            error = color_tree_add(
                &mut tree,
                *color.offset(0 as std::os::raw::c_int as isize),
                *color.offset(1 as std::os::raw::c_int as isize),
                *color.offset(2 as std::os::raw::c_int as isize),
                *color.offset(3 as std::os::raw::c_int as isize),
                i as std::os::raw::c_uint,
            );
            if error != 0 {
                current_block = 2485031591849961683;
                break;
            }
            i = i.wrapping_add(1);
        }
    } else {
        current_block = 15925075030174552612;
    }
    match current_block {
        15925075030174552612 => {
            if (*((mode_in).clone()).unwrap()).bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint && sixteen == 0 {
                let mut r = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut g = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut b = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut a = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                i = 0 as std::os::raw::c_int as size_t;
                while i != numpixels {
                    getPixelColorRGBA16(
                        Some(&mut r),
                        Some(&mut g),
                        Some(&mut b),
                        Some(&mut a),
                        in_0,
                        i,
                        (mode_in).clone(),
                    );
                    if r as std::os::raw::c_int & 255 as std::os::raw::c_int
                        != r as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int
                        || g as std::os::raw::c_int & 255 as std::os::raw::c_int
                            != g as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int
                        || b as std::os::raw::c_int & 255 as std::os::raw::c_int
                            != b as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int
                        || a as std::os::raw::c_int & 255 as std::os::raw::c_int
                            != a as std::os::raw::c_int >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int
                    {
                        (*stats).bits = 16 as std::os::raw::c_int as std::os::raw::c_uint;
                        sixteen = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        bits_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        numcolors_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
            }
            if sixteen != 0 {
                let mut r_0 = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut g_0 = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut b_0 = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                let mut a_0 = 0 as std::os::raw::c_int as std::os::raw::c_ushort;
                i = 0 as std::os::raw::c_int as size_t;
                while i != numpixels {
                    getPixelColorRGBA16(
                        Some(&mut r_0),
                        Some(&mut g_0),
                        Some(&mut b_0),
                        Some(&mut a_0),
                        in_0,
                        i,
                        (mode_in).clone(),
                    );
                    if colored_done == 0
                        && (r_0 as std::os::raw::c_int != g_0 as std::os::raw::c_int
                            || r_0 as std::os::raw::c_int != b_0 as std::os::raw::c_int)
                    {
                        (*stats).colored = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        colored_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                    }
                    if alpha_done == 0 {
                        let mut matchkey = (r_0 as std::os::raw::c_int
                            == (*stats).key_r as std::os::raw::c_int
                            && g_0 as std::os::raw::c_int == (*stats).key_g as std::os::raw::c_int
                            && b_0 as std::os::raw::c_int == (*stats).key_b as std::os::raw::c_int)
                            as std::os::raw::c_int as std::os::raw::c_uint;
                        if a_0 as std::os::raw::c_int != 65535 as std::os::raw::c_int
                            && (a_0 as std::os::raw::c_int != 0 as std::os::raw::c_int
                                || (*stats).key != 0 && matchkey == 0)
                        {
                            (*stats).alpha = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*stats).key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            alpha_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        } else if a_0 as std::os::raw::c_int == 0 as std::os::raw::c_int
                            && (*stats).alpha == 0 && (*stats).key == 0
                        {
                            (*stats).key = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*stats).key_r = r_0;
                            (*stats).key_g = g_0;
                            (*stats).key_b = b_0;
                        } else if a_0 as std::os::raw::c_int == 65535 as std::os::raw::c_int
                            && (*stats).key != 0 && matchkey != 0
                        {
                            (*stats).alpha = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*stats).key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            alpha_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        }
                    }
                    if alpha_done != 0 && numcolors_done != 0 && colored_done != 0
                        && bits_done != 0
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if (*stats).key != 0 && (*stats).alpha == 0 {
                    i = 0 as std::os::raw::c_int as size_t;
                    while i != numpixels {
                        getPixelColorRGBA16(
                            Some(&mut r_0),
                            Some(&mut g_0),
                            Some(&mut b_0),
                            Some(&mut a_0),
                            in_0,
                            i,
                            (mode_in).clone(),
                        );
                        if a_0 as std::os::raw::c_int != 0 as std::os::raw::c_int
                            && r_0 as std::os::raw::c_int == (*stats).key_r as std::os::raw::c_int
                            && g_0 as std::os::raw::c_int == (*stats).key_g as std::os::raw::c_int
                            && b_0 as std::os::raw::c_int == (*stats).key_b as std::os::raw::c_int
                        {
                            (*stats).alpha = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*stats).key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            alpha_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        }
                        i = i.wrapping_add(1);
                    }
                }
            } else {
                let mut r_1 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                let mut g_1 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                let mut b_1 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                let mut a_1 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                i = 0 as std::os::raw::c_int as size_t;
                loop {
                    if !(i != numpixels) {
                        current_block = 17736998403848444560;
                        break;
                    }
                    getPixelColorRGBA8(
                        Some(&mut r_1),
                        Some(&mut g_1),
                        Some(&mut b_1),
                        Some(&mut a_1),
                        in_0,
                        i,
                        (mode_in).clone(),
                    );
                    if bits_done == 0 && (*stats).bits < 8 as std::os::raw::c_int as std::os::raw::c_uint
                    {
                        let mut bits = getValueRequiredBits(r_1);
                        if bits > (*stats).bits {
                            (*stats).bits = bits;
                        }
                    }
                    bits_done = ((*stats).bits >= bpp) as std::os::raw::c_int as std::os::raw::c_uint;
                    if colored_done == 0
                        && (r_1 as std::os::raw::c_int != g_1 as std::os::raw::c_int
                            || r_1 as std::os::raw::c_int != b_1 as std::os::raw::c_int)
                    {
                        (*stats).colored = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        colored_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                        if (*stats).bits < 8 as std::os::raw::c_int as std::os::raw::c_uint {
                            (*stats).bits = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                        }
                    }
                    if alpha_done == 0 {
                        let mut matchkey_0 = (r_1 as std::os::raw::c_int
                            == (*stats).key_r as std::os::raw::c_int
                            && g_1 as std::os::raw::c_int == (*stats).key_g as std::os::raw::c_int
                            && b_1 as std::os::raw::c_int == (*stats).key_b as std::os::raw::c_int)
                            as std::os::raw::c_int as std::os::raw::c_uint;
                        if a_1 as std::os::raw::c_int != 255 as std::os::raw::c_int
                            && (a_1 as std::os::raw::c_int != 0 as std::os::raw::c_int
                                || (*stats).key != 0 && matchkey_0 == 0)
                        {
                            (*stats).alpha = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*stats).key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            alpha_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            if (*stats).bits < 8 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*stats).bits = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                            }
                        } else if a_1 as std::os::raw::c_int == 0 as std::os::raw::c_int
                            && (*stats).alpha == 0 && (*stats).key == 0
                        {
                            (*stats).key = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*stats).key_r = r_1 as std::os::raw::c_ushort;
                            (*stats).key_g = g_1 as std::os::raw::c_ushort;
                            (*stats).key_b = b_1 as std::os::raw::c_ushort;
                        } else if a_1 as std::os::raw::c_int == 255 as std::os::raw::c_int
                            && (*stats).key != 0 && matchkey_0 != 0
                        {
                            (*stats).alpha = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            (*stats).key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            alpha_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                            if (*stats).bits < 8 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*stats).bits = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                            }
                        }
                    }
                    if numcolors_done == 0 {
                        if color_tree_has(&mut tree, r_1, g_1, b_1, a_1) == 0 {
                            error = color_tree_add(
                                &mut tree,
                                r_1,
                                g_1,
                                b_1,
                                a_1,
                                (*stats).numcolors,
                            );
                            if error != 0 {
                                current_block = 2485031591849961683;
                                break;
                            }
                            if (*stats).numcolors < 256 as std::os::raw::c_int as std::os::raw::c_uint {
                                let mut p = ((*stats).palette).as_mut_ptr();
                                let mut n = (*stats).numcolors;
                                *p
                                    .offset(
                                        n
                                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                                    ) = r_1;
                                *p
                                    .offset(
                                        n
                                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                                    ) = g_1;
                                *p
                                    .offset(
                                        n
                                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                                    ) = b_1;
                                *p
                                    .offset(
                                        n
                                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_uint)
                                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                                    ) = a_1;
                            }
                            let ref mut fresh162 = (*stats).numcolors;
                            *fresh162 = (*fresh162).wrapping_add(1);
                            numcolors_done = ((*stats).numcolors >= maxnumcolors)
                                as std::os::raw::c_int as std::os::raw::c_uint;
                        }
                    }
                    if alpha_done != 0 && numcolors_done != 0 && colored_done != 0
                        && bits_done != 0
                    {
                        current_block = 17736998403848444560;
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                match current_block {
                    2485031591849961683 => {}
                    _ => {
                        if (*stats).key != 0 && (*stats).alpha == 0 {
                            i = 0 as std::os::raw::c_int as size_t;
                            while i != numpixels {
                                getPixelColorRGBA8(
                                    Some(&mut r_1),
                                    Some(&mut g_1),
                                    Some(&mut b_1),
                                    Some(&mut a_1),
                                    in_0,
                                    i,
                                    (mode_in).clone(),
                                );
                                if a_1 as std::os::raw::c_int != 0 as std::os::raw::c_int
                                    && r_1 as std::os::raw::c_int == (*stats).key_r as std::os::raw::c_int
                                    && g_1 as std::os::raw::c_int == (*stats).key_g as std::os::raw::c_int
                                    && b_1 as std::os::raw::c_int == (*stats).key_b as std::os::raw::c_int
                                {
                                    (*stats).alpha = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                    (*stats).key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                    alpha_done = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                    if (*stats).bits < 8 as std::os::raw::c_int as std::os::raw::c_uint {
                                        (*stats).bits = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                        }
                        let ref mut fresh163 = (*stats).key_r;
                        *fresh163 = (*fresh163 as std::os::raw::c_int
                            + (((*stats).key_r as std::os::raw::c_int) << 8 as std::os::raw::c_int))
                            as std::os::raw::c_ushort;
                        let ref mut fresh164 = (*stats).key_g;
                        *fresh164 = (*fresh164 as std::os::raw::c_int
                            + (((*stats).key_g as std::os::raw::c_int) << 8 as std::os::raw::c_int))
                            as std::os::raw::c_ushort;
                        let ref mut fresh165 = (*stats).key_b;
                        *fresh165 = (*fresh165 as std::os::raw::c_int
                            + (((*stats).key_b as std::os::raw::c_int) << 8 as std::os::raw::c_int))
                            as std::os::raw::c_ushort;
                    }
                }
            }
        }
        _ => {}
    }
    color_tree_cleanup(&mut tree);
    return error;
}
unsafe extern "C" fn lodepng_color_stats_add(
    mut stats: * mut crate::src::lodepng::LodePNGColorStats,
    mut r: std::os::raw::c_uint,
    mut g: std::os::raw::c_uint,
    mut b: std::os::raw::c_uint,
    mut a: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut image: [u8; 8] = [0; 8];
    let mut mode = LodePNGColorMode {
        colortype: LCT_GREY,
        bitdepth: 0,
        palette: 0 as *mut std::os::raw::c_uchar,
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
    };
    lodepng_color_mode_init(&mut mode);
    image[0 as std::os::raw::c_int as usize] = (r >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
    image[1 as std::os::raw::c_int as usize] = r as std::os::raw::c_uchar;
    image[2 as std::os::raw::c_int as usize] = (g >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
    image[3 as std::os::raw::c_int as usize] = g as std::os::raw::c_uchar;
    image[4 as std::os::raw::c_int as usize] = (b >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
    image[5 as std::os::raw::c_int as usize] = b as std::os::raw::c_uchar;
    image[6 as std::os::raw::c_int as usize] = (a >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
    image[7 as std::os::raw::c_int as usize] = a as std::os::raw::c_uchar;
    mode.bitdepth = 16 as std::os::raw::c_int as std::os::raw::c_uint;
    mode.colortype = LCT_RGBA;
    error = lodepng_compute_color_stats(
        stats,
        image.as_mut_ptr(),
        1 as std::os::raw::c_int as std::os::raw::c_uint,
        1 as std::os::raw::c_int as std::os::raw::c_uint,
        Some(&mut mode),
    );
    lodepng_color_mode_cleanup(&mut mode);
    return error;
}
unsafe extern "C" fn auto_choose_color<'a1>(
    mut mode_out: * mut crate::src::lodepng::LodePNGColorMode,
    mut mode_in: * const crate::src::lodepng::LodePNGColorMode,
    mut stats: Option<&'a1 crate::src::lodepng::LodePNGColorStats>,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut palettebits: u32 = 0;
    let mut i: u64 = 0;
    let mut n: u64 = 0;
    let mut numpixels = (*(stats).unwrap()).numpixels;
    let mut palette_ok: u32 = 0;
    let mut gray_ok: u32 = 0;
    let mut alpha = (*(stats).unwrap()).alpha;
    let mut key = (*(stats).unwrap()).key;
    let mut bits = (*(stats).unwrap()).bits;
    (*mode_out).key_defined = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if key != 0 && numpixels <= 16 as std::os::raw::c_int as std::os::raw::c_ulong {
        alpha = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        key = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        if bits < 8 as std::os::raw::c_int as std::os::raw::c_uint {
            bits = 8 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    gray_ok = ((*((stats).clone()).unwrap()).colored == 0) as std::os::raw::c_int as std::os::raw::c_uint;
    if (*((stats).clone()).unwrap()).allow_greyscale == 0 {
        gray_ok = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if gray_ok == 0 && bits < 8 as std::os::raw::c_int as std::os::raw::c_uint {
        bits = 8 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    n = (*(stats).unwrap()).numcolors as size_t;
    palettebits = (if n <= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        1 as std::os::raw::c_int
    } else if n <= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        2 as std::os::raw::c_int
    } else if n <= 16 as std::os::raw::c_int as std::os::raw::c_ulong {
        4 as std::os::raw::c_int
    } else {
        8 as std::os::raw::c_int
    }) as std::os::raw::c_uint;
    palette_ok = (n <= 256 as std::os::raw::c_int as std::os::raw::c_ulong
        && bits <= 8 as std::os::raw::c_int as std::os::raw::c_uint
        && n != 0 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int as std::os::raw::c_uint;
    if numpixels < n.wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong) {
        palette_ok = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if gray_ok != 0 && alpha == 0 && bits <= palettebits {
        palette_ok = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*((stats).clone()).unwrap()).allow_palette == 0 {
        palette_ok = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if palette_ok != 0 {
        let mut p = ((*((stats).clone()).unwrap()).palette).as_ptr();
        lodepng_palette_clear(mode_out);
        i = 0 as std::os::raw::c_int as size_t;
        while i != (*((stats).clone()).unwrap()).numcolors as std::os::raw::c_ulong {
            error = lodepng_palette_add(
                mode_out,
                *p
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ),
                *p
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ),
                *p
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ),
                *p
                    .offset(
                        i
                            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ),
            );
            if error != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        (*mode_out).colortype = LCT_PALETTE;
        (*mode_out).bitdepth = palettebits;
        if (*mode_in).colortype as std::os::raw::c_uint
            == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
            && (*mode_in).palettesize >= (*mode_out).palettesize
            && (*mode_in).bitdepth == (*mode_out).bitdepth
        {
            lodepng_color_mode_cleanup(mode_out);
            lodepng_color_mode_copy(mode_out, mode_in);
        }
    } else {
        (*mode_out).bitdepth = bits;
        (*mode_out)
            .colortype = (if alpha != 0 {
            if gray_ok != 0 {
                LCT_GREY_ALPHA as std::os::raw::c_int
            } else {
                LCT_RGBA as std::os::raw::c_int
            }
        } else if gray_ok != 0 {
            LCT_GREY as std::os::raw::c_int
        } else {
            LCT_RGB as std::os::raw::c_int
        }) as LodePNGColorType;
        if key != 0 {
            let mut mask_0 = ((1 as std::os::raw::c_uint) << (*mode_out).bitdepth)
                .wrapping_sub(1 as std::os::raw::c_uint);
            (*mode_out).key_r = (*((stats).clone()).unwrap()).key_r as std::os::raw::c_uint & mask_0;
            (*mode_out).key_g = (*((stats).clone()).unwrap()).key_g as std::os::raw::c_uint & mask_0;
            (*mode_out).key_b = (*((stats).clone()).unwrap()).key_b as std::os::raw::c_uint & mask_0;
            (*mode_out).key_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    return error;
}
 extern "C" fn paethPredictor(
    mut a: std::os::raw::c_short,
    mut b: std::os::raw::c_short,
    mut c: std::os::raw::c_short,
) -> std::os::raw::c_uchar {
    let mut pa = (if (b as std::os::raw::c_int - c as std::os::raw::c_int) < 0 as std::os::raw::c_int {
        -(b as std::os::raw::c_int - c as std::os::raw::c_int)
    } else {
        b as std::os::raw::c_int - c as std::os::raw::c_int
    }) as std::os::raw::c_short;
    let mut pb = (if (a as std::os::raw::c_int - c as std::os::raw::c_int) < 0 as std::os::raw::c_int {
        -(a as std::os::raw::c_int - c as std::os::raw::c_int)
    } else {
        a as std::os::raw::c_int - c as std::os::raw::c_int
    }) as std::os::raw::c_short;
    let mut pc = (if (a as std::os::raw::c_int + b as std::os::raw::c_int - c as std::os::raw::c_int
        - c as std::os::raw::c_int) < 0 as std::os::raw::c_int
    {
        -(a as std::os::raw::c_int + b as std::os::raw::c_int - c as std::os::raw::c_int - c as std::os::raw::c_int)
    } else {
        a as std::os::raw::c_int + b as std::os::raw::c_int - c as std::os::raw::c_int - c as std::os::raw::c_int
    }) as std::os::raw::c_short;
    if (pb as std::os::raw::c_int) < pa as std::os::raw::c_int {
        a = b;
        pa = pb;
    }
    return (if (pc as std::os::raw::c_int) < pa as std::os::raw::c_int {
        c as std::os::raw::c_int
    } else {
        a as std::os::raw::c_int
    }) as std::os::raw::c_uchar;
}
static mut ADAM7_IX: [std::os::raw::c_uint; 7] = [0,0,0,0,0,0,0,]; unsafe fn laertes_init_ADAM7_IX() {
ADAM7_IX = [
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
static mut ADAM7_IY: [std::os::raw::c_uint; 7] = [0,0,0,0,0,0,0,]; unsafe fn laertes_init_ADAM7_IY() {
ADAM7_IY = [
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    0 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
static mut ADAM7_DX: [std::os::raw::c_uint; 7] = [0,0,0,0,0,0,0,]; unsafe fn laertes_init_ADAM7_DX() {
ADAM7_DX = [
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    1 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
static mut ADAM7_DY: [std::os::raw::c_uint; 7] = [0,0,0,0,0,0,0,]; unsafe fn laertes_init_ADAM7_DY() {
ADAM7_DY = [
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    8 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    4 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
    2 as std::os::raw::c_int as std::os::raw::c_uint,
];}//;
unsafe extern "C" fn Adam7_getpassvalues(
    mut passw: * mut std::os::raw::c_uint,
    mut passh: * mut std::os::raw::c_uint,
    mut filter_passstart: * mut std::os::raw::c_ulong,
    mut padded_passstart: * mut std::os::raw::c_ulong,
    mut passstart: * mut std::os::raw::c_ulong,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut bpp: std::os::raw::c_uint,
) {
    let mut i: u32 = 0;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
        *passw
            .offset(
                i as isize,
            ) = w
            .wrapping_add(ADAM7_DX[i as usize])
            .wrapping_sub(ADAM7_IX[i as usize])
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_div(ADAM7_DX[i as usize]);
        *passh
            .offset(
                i as isize,
            ) = h
            .wrapping_add(ADAM7_DY[i as usize])
            .wrapping_sub(ADAM7_IY[i as usize])
            .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
            .wrapping_div(ADAM7_DY[i as usize]);
        if *passw.offset(i as isize) == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            *passh.offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *passh.offset(i as isize) == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            *passw.offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        i = i.wrapping_add(1);
    }
    let ref mut fresh166 = *passstart.offset(0 as std::os::raw::c_int as isize);
    *fresh166 = 0 as std::os::raw::c_int as size_t;
    let ref mut fresh167 = *padded_passstart.offset(0 as std::os::raw::c_int as isize);
    *fresh167 = *fresh166;
    *filter_passstart.offset(0 as std::os::raw::c_int as isize) = *fresh167;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
        *filter_passstart
            .offset(
                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) = (*filter_passstart.offset(i as isize))
            .wrapping_add(
                (if *passw.offset(i as isize) != 0 && *passh.offset(i as isize) != 0 {
                    (*passh.offset(i as isize))
                        .wrapping_mul(
                            (1 as std::os::raw::c_uint)
                                .wrapping_add(
                                    (*passw.offset(i as isize))
                                        .wrapping_mul(bpp)
                                        .wrapping_add(7 as std::os::raw::c_uint)
                                        .wrapping_div(8 as std::os::raw::c_uint),
                                ),
                        )
                } else {
                    0 as std::os::raw::c_int as std::os::raw::c_uint
                }) as std::os::raw::c_ulong,
            );
        *padded_passstart
            .offset(
                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) = (*padded_passstart.offset(i as isize))
            .wrapping_add(
                (*passh.offset(i as isize))
                    .wrapping_mul(
                        (*passw.offset(i as isize))
                            .wrapping_mul(bpp)
                            .wrapping_add(7 as std::os::raw::c_uint)
                            .wrapping_div(8 as std::os::raw::c_uint),
                    ) as std::os::raw::c_ulong,
            );
        *passstart
            .offset(
                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) = (*passstart.offset(i as isize))
            .wrapping_add(
                (*passh.offset(i as isize))
                    .wrapping_mul(*passw.offset(i as isize))
                    .wrapping_mul(bpp)
                    .wrapping_add(7 as std::os::raw::c_uint)
                    .wrapping_div(8 as std::os::raw::c_uint) as std::os::raw::c_ulong,
            );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_inspect<'a1, 'a2>(
    mut w: Option<&'a1 mut std::os::raw::c_uint>,
    mut h: Option<&'a2 mut std::os::raw::c_uint>,
    mut state: * mut crate::src::lodepng::LodePNGState,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut info: * mut crate::src::lodepng::LodePNGInfo = &mut (*state).info_png;
    if insize == 0 as std::os::raw::c_int as std::os::raw::c_ulong || in_0.is_null() {
        (*state).error = 48 as std::os::raw::c_int as std::os::raw::c_uint;
        return 48 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if insize < 33 as std::os::raw::c_int as std::os::raw::c_ulong {
        (*state).error = 27 as std::os::raw::c_int as std::os::raw::c_uint;
        return 27 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    lodepng_info_cleanup(info);
    lodepng_info_init(info);
    if *in_0.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != 137 as std::os::raw::c_int
        || *in_0.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int != 80 as std::os::raw::c_int
        || *in_0.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int != 78 as std::os::raw::c_int
        || *in_0.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int != 71 as std::os::raw::c_int
        || *in_0.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int != 13 as std::os::raw::c_int
        || *in_0.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int != 10 as std::os::raw::c_int
        || *in_0.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_int != 26 as std::os::raw::c_int
        || *in_0.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int != 10 as std::os::raw::c_int
    {
        (*state).error = 28 as std::os::raw::c_int as std::os::raw::c_uint;
        return 28 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if lodepng_chunk_length(in_0.offset(8 as std::os::raw::c_int as isize))
        != 13 as std::os::raw::c_int as std::os::raw::c_uint
    {
        (*state).error = 94 as std::os::raw::c_int as std::os::raw::c_uint;
        return 94 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if lodepng_chunk_type_equals(
        in_0.offset(8 as std::os::raw::c_int as isize),
        b"IHDR\0" as *const u8 as *const std::os::raw::c_char,
    ) == 0
    {
        (*state).error = 29 as std::os::raw::c_int as std::os::raw::c_uint;
        return 29 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    width = lodepng_read32bitInt(&*in_0.offset(16 as std::os::raw::c_int as isize));
    height = lodepng_read32bitInt(&*in_0.offset(20 as std::os::raw::c_int as isize));
    if !borrow(& w).is_none() {
        *(borrow_mut(&mut w)).unwrap() = width;
    }
    if !borrow(& h).is_none() {
        *(borrow_mut(&mut h)).unwrap() = height;
    }
    (*info).color.bitdepth = *in_0.offset(24 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    (*info)
        .color
        .colortype = *in_0.offset(25 as std::os::raw::c_int as isize) as LodePNGColorType;
    (*info)
        .compression_method = *in_0.offset(26 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    (*info).filter_method = *in_0.offset(27 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    (*info).interlace_method = *in_0.offset(28 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    if width == 0 as std::os::raw::c_int as std::os::raw::c_uint
        || height == 0 as std::os::raw::c_int as std::os::raw::c_uint
    {
        (*state).error = 93 as std::os::raw::c_int as std::os::raw::c_uint;
        return 93 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*state).error = checkColorValidity((*info).color.colortype, (*info).color.bitdepth);
    if (*state).error != 0 {
        return (*state).error;
    }
    if (*info).compression_method != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).error = 32 as std::os::raw::c_int as std::os::raw::c_uint;
        return 32 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*info).filter_method != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).error = 33 as std::os::raw::c_int as std::os::raw::c_uint;
        return 33 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*info).interlace_method > 1 as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).error = 34 as std::os::raw::c_int as std::os::raw::c_uint;
        return 34 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*state).decoder.ignore_crc == 0 {
        let mut CRC = lodepng_read32bitInt(&*in_0.offset(29 as std::os::raw::c_int as isize));
        let mut checksum = lodepng_crc32(
            &*in_0.offset(12 as std::os::raw::c_int as isize),
            17 as std::os::raw::c_int as size_t,
        );
        if CRC != checksum {
            (*state).error = 57 as std::os::raw::c_int as std::os::raw::c_uint;
            return 57 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    return (*state).error;
}
unsafe extern "C" fn unfilterScanline(
    mut recon: * mut std::os::raw::c_uchar,
    mut scanline: * const std::os::raw::c_uchar,
    mut precon: * const std::os::raw::c_uchar,
    mut bytewidth: std::os::raw::c_ulong,
    mut filterType: std::os::raw::c_uchar,
    mut length: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut i: u64 = 0;
    match filterType as std::os::raw::c_int {
        0 => {
            i = 0 as std::os::raw::c_int as size_t;
            while i != length {
                *recon.offset(i as isize) = *scanline.offset(i as isize);
                i = i.wrapping_add(1);
            }
        }
        1 => {
            let mut j = 0 as std::os::raw::c_int as size_t;
            i = 0 as std::os::raw::c_int as size_t;
            while i != bytewidth {
                *recon.offset(i as isize) = *scanline.offset(i as isize);
                i = i.wrapping_add(1);
            }
            i = bytewidth;
            while i != length {
                *recon
                    .offset(
                        i as isize,
                    ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                    + *recon.offset(j as isize) as std::os::raw::c_int) as std::os::raw::c_uchar;
                i = i.wrapping_add(1);
                j = j.wrapping_add(1);
            }
        }
        2 => {
            if !precon.is_null() {
                i = 0 as std::os::raw::c_int as size_t;
                while i != length {
                    *recon
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        + *precon.offset(i as isize) as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0 as std::os::raw::c_int as size_t;
                while i != length {
                    *recon.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
            }
        }
        3 => {
            if !precon.is_null() {
                let mut j_0 = 0 as std::os::raw::c_int as size_t;
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *recon
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        + (*precon.offset(i as isize) as std::os::raw::c_int
                            >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
                if bytewidth >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
                    while i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) < length {
                        let mut s0 = *scanline
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s1 = *scanline
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s2 = *scanline
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s3 = *scanline
                            .offset(
                                i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r0 = *recon
                            .offset(
                                j_0.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r1 = *recon
                            .offset(
                                j_0.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r2 = *recon
                            .offset(
                                j_0.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r3 = *recon
                            .offset(
                                j_0.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p0 = *precon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p1 = *precon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p2 = *precon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p3 = *precon
                            .offset(
                                i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        *recon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s0 as std::os::raw::c_int
                            + (r0 as std::os::raw::c_int + p0 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s1 as std::os::raw::c_int
                            + (r1 as std::os::raw::c_int + p1 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s2 as std::os::raw::c_int
                            + (r2 as std::os::raw::c_int + p2 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s3 as std::os::raw::c_int
                            + (r3 as std::os::raw::c_int + p3 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        i = (i as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        j_0 = (j_0 as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                } else if bytewidth >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
                    while i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) < length {
                        let mut s0_0 = *scanline
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s1_0 = *scanline
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s2_0 = *scanline
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r0_0 = *recon
                            .offset(
                                j_0.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r1_0 = *recon
                            .offset(
                                j_0.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r2_0 = *recon
                            .offset(
                                j_0.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p0_0 = *precon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p1_0 = *precon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p2_0 = *precon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        *recon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s0_0 as std::os::raw::c_int
                            + (r0_0 as std::os::raw::c_int + p0_0 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s1_0 as std::os::raw::c_int
                            + (r1_0 as std::os::raw::c_int + p1_0 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s2_0 as std::os::raw::c_int
                            + (r2_0 as std::os::raw::c_int + p2_0 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        i = (i as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        j_0 = (j_0 as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                } else if bytewidth >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                    while i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) < length {
                        let mut s0_1 = *scanline
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s1_1 = *scanline
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r0_1 = *recon
                            .offset(
                                j_0.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r1_1 = *recon
                            .offset(
                                j_0.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p0_1 = *precon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p1_1 = *precon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        *recon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s0_1 as std::os::raw::c_int
                            + (r0_1 as std::os::raw::c_int + p0_1 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s1_1 as std::os::raw::c_int
                            + (r1_1 as std::os::raw::c_int + p1_1 as std::os::raw::c_int
                                >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                        i = (i as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        j_0 = (j_0 as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                }
                while i != length {
                    *recon
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        + (*recon.offset(j_0 as isize) as std::os::raw::c_int
                            + *precon.offset(i as isize) as std::os::raw::c_int
                            >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                    j_0 = j_0.wrapping_add(1);
                }
            } else {
                let mut j_1 = 0 as std::os::raw::c_int as size_t;
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *recon.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i != length {
                    *recon
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        + (*recon.offset(j_1 as isize) as std::os::raw::c_int
                            >> 1 as std::os::raw::c_uint)) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                    j_1 = j_1.wrapping_add(1);
                }
            }
        }
        4 => {
            if !precon.is_null() {
                let mut j_2 = 0 as std::os::raw::c_int as size_t;
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *recon
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        + *precon.offset(i as isize) as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
                if bytewidth >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
                    while i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) < length {
                        let mut s0_2 = *scanline
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s1_2 = *scanline
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s2_1 = *scanline
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s3_0 = *scanline
                            .offset(
                                i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r0_2 = *recon
                            .offset(
                                j_2.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r1_2 = *recon
                            .offset(
                                j_2.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r2_1 = *recon
                            .offset(
                                j_2.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r3_0 = *recon
                            .offset(
                                j_2.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p0_2 = *precon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p1_2 = *precon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p2_1 = *precon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p3_0 = *precon
                            .offset(
                                i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q0 = *precon
                            .offset(
                                j_2.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q1 = *precon
                            .offset(
                                j_2.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q2 = *precon
                            .offset(
                                j_2.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q3 = *precon
                            .offset(
                                j_2.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        *recon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s0_2 as std::os::raw::c_int
                            + paethPredictor(
                                r0_2 as std::os::raw::c_short,
                                p0_2 as std::os::raw::c_short,
                                q0 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s1_2 as std::os::raw::c_int
                            + paethPredictor(
                                r1_2 as std::os::raw::c_short,
                                p1_2 as std::os::raw::c_short,
                                q1 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s2_1 as std::os::raw::c_int
                            + paethPredictor(
                                r2_1 as std::os::raw::c_short,
                                p2_1 as std::os::raw::c_short,
                                q2 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s3_0 as std::os::raw::c_int
                            + paethPredictor(
                                r3_0 as std::os::raw::c_short,
                                p3_0 as std::os::raw::c_short,
                                q3 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        i = (i as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        j_2 = (j_2 as std::os::raw::c_ulong)
                            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                } else if bytewidth >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
                    while i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) < length {
                        let mut s0_3 = *scanline
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s1_3 = *scanline
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s2_2 = *scanline
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r0_3 = *recon
                            .offset(
                                j_2.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r1_3 = *recon
                            .offset(
                                j_2.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r2_2 = *recon
                            .offset(
                                j_2.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p0_3 = *precon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p1_3 = *precon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p2_2 = *precon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q0_0 = *precon
                            .offset(
                                j_2.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q1_0 = *precon
                            .offset(
                                j_2.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q2_0 = *precon
                            .offset(
                                j_2.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        *recon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s0_3 as std::os::raw::c_int
                            + paethPredictor(
                                r0_3 as std::os::raw::c_short,
                                p0_3 as std::os::raw::c_short,
                                q0_0 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s1_3 as std::os::raw::c_int
                            + paethPredictor(
                                r1_3 as std::os::raw::c_short,
                                p1_3 as std::os::raw::c_short,
                                q1_0 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s2_2 as std::os::raw::c_int
                            + paethPredictor(
                                r2_2 as std::os::raw::c_short,
                                p2_2 as std::os::raw::c_short,
                                q2_0 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        i = (i as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        j_2 = (j_2 as std::os::raw::c_ulong)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                } else if bytewidth >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                    while i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) < length {
                        let mut s0_4 = *scanline
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut s1_4 = *scanline
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r0_4 = *recon
                            .offset(
                                j_2.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut r1_4 = *recon
                            .offset(
                                j_2.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p0_4 = *precon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut p1_4 = *precon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q0_1 = *precon
                            .offset(
                                j_2.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        let mut q1_1 = *precon
                            .offset(
                                j_2.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            );
                        *recon
                            .offset(
                                i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s0_4 as std::os::raw::c_int
                            + paethPredictor(
                                r0_4 as std::os::raw::c_short,
                                p0_4 as std::os::raw::c_short,
                                q0_1 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        *recon
                            .offset(
                                i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                            ) = (s1_4 as std::os::raw::c_int
                            + paethPredictor(
                                r1_4 as std::os::raw::c_short,
                                p1_4 as std::os::raw::c_short,
                                q1_1 as std::os::raw::c_short,
                            ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                        i = (i as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        j_2 = (j_2 as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                }
                while i != length {
                    *recon
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        + paethPredictor(
                            *recon.offset(i.wrapping_sub(bytewidth) as isize)
                                as std::os::raw::c_short,
                            *precon.offset(i as isize) as std::os::raw::c_short,
                            *precon.offset(j_2 as isize) as std::os::raw::c_short,
                        ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                    j_2 = j_2.wrapping_add(1);
                }
            } else {
                let mut j_3 = 0 as std::os::raw::c_int as size_t;
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *recon.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i != length {
                    *recon
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        + *recon.offset(j_3 as isize) as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                    j_3 = j_3.wrapping_add(1);
                }
            }
        }
        _ => return 36 as std::os::raw::c_int as std::os::raw::c_uint,
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn unfilter(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut bpp: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut y: u32 = 0;
    let mut prevline = 0 as *mut std::os::raw::c_uchar;
    let mut bytewidth = bpp
        .wrapping_add(7 as std::os::raw::c_uint)
        .wrapping_div(8 as std::os::raw::c_uint) as size_t;
    let mut linebytes = (lodepng_get_raw_size_idat(
        w,
        1 as std::os::raw::c_int as std::os::raw::c_uint,
        bpp,
    ))
        .wrapping_sub(1 as std::os::raw::c_uint as std::os::raw::c_ulong);
    y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while y < h {
        let mut outindex = linebytes.wrapping_mul(y as std::os::raw::c_ulong);
        let mut inindex = (1 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(linebytes)
            .wrapping_mul(y as std::os::raw::c_ulong);
        let mut filterType = *in_0.offset(inindex as isize);
        let mut error = unfilterScanline(
            &mut *out.offset(outindex as isize),
            &*in_0
                .offset(
                    inindex.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ),
            prevline,
            bytewidth,
            filterType,
            linebytes,
        );
        if error != 0 {
            return error;
        }
        prevline = &mut *out.offset(outindex as isize) as *mut std::os::raw::c_uchar;
        y = y.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn Adam7_deinterlace(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut bpp: std::os::raw::c_uint,
) {
    let mut passw: [u32; 7] = [0; 7];
    let mut passh: [u32; 7] = [0; 7];
    let mut filter_passstart: [u64; 8] = [0; 8];
    let mut padded_passstart: [u64; 8] = [0; 8];
    let mut passstart: [u64; 8] = [0; 8];
    let mut i: u32 = 0;
    Adam7_getpassvalues(
        passw.as_mut_ptr(),
        passh.as_mut_ptr(),
        filter_passstart.as_mut_ptr(),
        padded_passstart.as_mut_ptr(),
        passstart.as_mut_ptr(),
        w,
        h,
        bpp,
    );
    if bpp >= 8 as std::os::raw::c_int as std::os::raw::c_uint {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut x: u32 = 0;
            let mut y: u32 = 0;
            let mut b: u32 = 0;
            let mut bytewidth = bpp.wrapping_div(8 as std::os::raw::c_uint) as size_t;
            y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while y < passh[i as usize] {
                x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while x < passw[i as usize] {
                    let mut pixelinstart = (passstart[i as usize])
                        .wrapping_add(
                            (y.wrapping_mul(passw[i as usize]).wrapping_add(x)
                                as std::os::raw::c_ulong)
                                .wrapping_mul(bytewidth),
                        );
                    let mut pixeloutstart = (ADAM7_IY[i as usize] as std::os::raw::c_ulong)
                        .wrapping_add(
                            (y as size_t)
                                .wrapping_mul(ADAM7_DY[i as usize] as std::os::raw::c_ulong),
                        )
                        .wrapping_mul(w as size_t)
                        .wrapping_add(ADAM7_IX[i as usize] as std::os::raw::c_ulong)
                        .wrapping_add(
                            (x as size_t)
                                .wrapping_mul(ADAM7_DX[i as usize] as std::os::raw::c_ulong),
                        )
                        .wrapping_mul(bytewidth);
                    b = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    while (b as std::os::raw::c_ulong) < bytewidth {
                        *out
                            .offset(
                                pixeloutstart.wrapping_add(b as std::os::raw::c_ulong) as isize,
                            ) = *in_0
                            .offset(
                                pixelinstart.wrapping_add(b as std::os::raw::c_ulong) as isize,
                            );
                        b = b.wrapping_add(1);
                    }
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut x_0: u32 = 0;
            let mut y_0: u32 = 0;
            let mut b_0: u32 = 0;
            let mut ilinebits = bpp.wrapping_mul(passw[i as usize]);
            let mut olinebits = bpp.wrapping_mul(w);
            let mut obp: u64 = 0;
            let mut ibp: u64 = 0;
            y_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while y_0 < passh[i as usize] {
                x_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while x_0 < passw[i as usize] {
                    ibp = (8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(passstart[i as usize])
                        .wrapping_add(
                            y_0
                                .wrapping_mul(ilinebits)
                                .wrapping_add(x_0.wrapping_mul(bpp)) as std::os::raw::c_ulong,
                        );
                    obp = (ADAM7_IY[i as usize] as std::os::raw::c_ulong)
                        .wrapping_add(
                            (y_0 as size_t)
                                .wrapping_mul(ADAM7_DY[i as usize] as std::os::raw::c_ulong),
                        )
                        .wrapping_mul(olinebits as std::os::raw::c_ulong)
                        .wrapping_add(
                            (ADAM7_IX[i as usize] as std::os::raw::c_ulong)
                                .wrapping_add(
                                    (x_0 as size_t)
                                        .wrapping_mul(ADAM7_DX[i as usize] as std::os::raw::c_ulong),
                                )
                                .wrapping_mul(bpp as std::os::raw::c_ulong),
                        );
                    b_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    while b_0 < bpp {
                        let mut bit = readBitFromReversedStream(Some(&mut ibp), in_0);
                        setBitOfReversedStream(Some(&mut obp), out, bit);
                        b_0 = b_0.wrapping_add(1);
                    }
                    x_0 = x_0.wrapping_add(1);
                }
                y_0 = y_0.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn removePaddingBits(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * const std::os::raw::c_uchar,
    mut olinebits: std::os::raw::c_ulong,
    mut ilinebits: std::os::raw::c_ulong,
    mut h: std::os::raw::c_uint,
) {
    let mut y: u32 = 0;
    let mut diff = ilinebits.wrapping_sub(olinebits);
    let mut ibp = 0 as std::os::raw::c_int as size_t;
    let mut obp = 0 as std::os::raw::c_int as size_t;
    y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while y < h {
        let mut x: u64 = 0;
        x = 0 as std::os::raw::c_int as size_t;
        while x < olinebits {
            let mut bit = readBitFromReversedStream(Some(&mut ibp), in_0);
            setBitOfReversedStream(Some(&mut obp), out, bit);
            x = x.wrapping_add(1);
        }
        ibp = (ibp as std::os::raw::c_ulong).wrapping_add(diff) as size_t as size_t;
        y = y.wrapping_add(1);
    }
}
unsafe extern "C" fn postProcessScanlines<'a1>(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * mut std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut info_png: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
) -> std::os::raw::c_uint {
    let mut bpp = lodepng_get_bpp((Some(&(*((info_png).clone()).unwrap()).color)).clone());
    if bpp == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return 31 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*((info_png).clone()).unwrap()).interlace_method == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        if bpp < 8 as std::os::raw::c_int as std::os::raw::c_uint
            && w.wrapping_mul(bpp)
                != w
                    .wrapping_mul(bpp)
                    .wrapping_add(7 as std::os::raw::c_uint)
                    .wrapping_div(8 as std::os::raw::c_uint)
                    .wrapping_mul(8 as std::os::raw::c_uint)
        {
            let mut error = unfilter(in_0, in_0, w, h, bpp);
            if error != 0 {
                return error;
            }
            removePaddingBits(
                out,
                in_0,
                w.wrapping_mul(bpp) as size_t,
                w
                    .wrapping_mul(bpp)
                    .wrapping_add(7 as std::os::raw::c_uint)
                    .wrapping_div(8 as std::os::raw::c_uint)
                    .wrapping_mul(8 as std::os::raw::c_uint) as size_t,
                h,
            );
        } else {
            let mut error_0 = unfilter(out, in_0, w, h, bpp);
            if error_0 != 0 {
                return error_0;
            }
        }
    } else {
        let mut passw: [u32; 7] = [0; 7];
        let mut passh: [u32; 7] = [0; 7];
        let mut filter_passstart: [u64; 8] = [0; 8];
        let mut padded_passstart: [u64; 8] = [0; 8];
        let mut passstart: [u64; 8] = [0; 8];
        let mut i: u32 = 0;
        Adam7_getpassvalues(
            passw.as_mut_ptr(),
            passh.as_mut_ptr(),
            filter_passstart.as_mut_ptr(),
            padded_passstart.as_mut_ptr(),
            passstart.as_mut_ptr(),
            w,
            h,
            bpp,
        );
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut error_1 = unfilter(
                &mut *in_0
                    .offset(*padded_passstart.as_mut_ptr().offset(i as isize) as isize),
                &mut *in_0
                    .offset(*filter_passstart.as_mut_ptr().offset(i as isize) as isize),
                passw[i as usize],
                passh[i as usize],
                bpp,
            );
            if error_1 != 0 {
                return error_1;
            }
            if bpp < 8 as std::os::raw::c_int as std::os::raw::c_uint {
                removePaddingBits(
                    &mut *in_0
                        .offset(*passstart.as_mut_ptr().offset(i as isize) as isize),
                    &mut *in_0
                        .offset(
                            *padded_passstart.as_mut_ptr().offset(i as isize) as isize,
                        ),
                    (passw[i as usize]).wrapping_mul(bpp) as size_t,
                    (passw[i as usize])
                        .wrapping_mul(bpp)
                        .wrapping_add(7 as std::os::raw::c_uint)
                        .wrapping_div(8 as std::os::raw::c_uint)
                        .wrapping_mul(8 as std::os::raw::c_uint) as size_t,
                    passh[i as usize],
                );
            }
            i = i.wrapping_add(1);
        }
        Adam7_deinterlace(out, in_0, w, h, bpp);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_PLTE(
    mut color: * mut crate::src::lodepng::LodePNGColorMode,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut pos = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u32 = 0;
    (*color).palettesize = chunkLength.wrapping_div(3 as std::os::raw::c_uint as std::os::raw::c_ulong);
    if (*color).palettesize == 0 as std::os::raw::c_int as std::os::raw::c_ulong
        || (*color).palettesize > 256 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 38 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    lodepng_color_mode_alloc_palette(color);
    if ((*color).palette).is_null() && (*color).palettesize != 0 {
        (*color).palettesize = 0 as std::os::raw::c_int as size_t;
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i as std::os::raw::c_ulong != (*color).palettesize {
        let mut fresh168 = pos;
        pos = pos.wrapping_add(1);
        *((*color).palette)
            .offset(
                (4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) = *data.offset(fresh168 as isize);
        let mut fresh169 = pos;
        pos = pos.wrapping_add(1);
        *((*color).palette)
            .offset(
                (4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) = *data.offset(fresh169 as isize);
        let mut fresh170 = pos;
        pos = pos.wrapping_add(1);
        *((*color).palette)
            .offset(
                (4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) = *data.offset(fresh170 as isize);
        *((*color).palette)
            .offset(
                (4 as std::os::raw::c_int as std::os::raw::c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
            ) = 255 as std::os::raw::c_int as std::os::raw::c_uchar;
        i = i.wrapping_add(1);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_tRNS(
    mut color: * mut crate::src::lodepng::LodePNGColorMode,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut i: u32 = 0;
    if (*color).colortype as std::os::raw::c_uint == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint {
        if chunkLength > (*color).palettesize {
            return 39 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i as std::os::raw::c_ulong != chunkLength {
            *((*color).palette)
                .offset(
                    (4 as std::os::raw::c_int as std::os::raw::c_uint)
                        .wrapping_mul(i)
                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                ) = *data.offset(i as isize);
            i = i.wrapping_add(1);
        }
    } else if (*color).colortype as std::os::raw::c_uint
        == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 30 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*color).key_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        let ref mut fresh171 = (*color).key_b;
        *fresh171 = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
        let ref mut fresh172 = (*color).key_g;
        *fresh172 = *fresh171;
        (*color).key_r = *fresh172;
    } else if (*color).colortype as std::os::raw::c_uint
        == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 6 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 41 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*color).key_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        (*color)
            .key_r = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
        (*color)
            .key_g = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
        (*color)
            .key_b = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    } else {
        return 42 as std::os::raw::c_int as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_bKGD(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    if (*info).color.colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 43 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_ulong
            >= (*info).color.palettesize
        {
            return 103 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*info).background_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        let ref mut fresh173 = (*info).background_b;
        *fresh173 = *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        let ref mut fresh174 = (*info).background_g;
        *fresh174 = *fresh173;
        (*info).background_r = *fresh174;
    } else if (*info).color.colortype as std::os::raw::c_uint
        == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
        || (*info).color.colortype as std::os::raw::c_uint
            == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 44 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*info).background_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        let ref mut fresh175 = (*info).background_b;
        *fresh175 = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
        let ref mut fresh176 = (*info).background_g;
        *fresh176 = *fresh175;
        (*info).background_r = *fresh176;
    } else if (*info).color.colortype as std::os::raw::c_uint
        == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
        || (*info).color.colortype as std::os::raw::c_uint
            == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 6 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 45 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*info).background_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        (*info)
            .background_r = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
        (*info)
            .background_g = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
        (*info)
            .background_b = (256 as std::os::raw::c_uint)
            .wrapping_mul(*data.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
            .wrapping_add(*data.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_tEXt(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut key = 0 as *mut std::os::raw::c_char;
    let mut str = 0 as *mut std::os::raw::c_char;
    if error == 0 {
        let mut length: u32 = 0;
        let mut string2_begin: u32 = 0;
        length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while (length as std::os::raw::c_ulong) < chunkLength
            && *data.offset(length as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int
        {
            length = length.wrapping_add(1);
        }
        if length < 1 as std::os::raw::c_int as std::os::raw::c_uint
            || length > 79 as std::os::raw::c_int as std::os::raw::c_uint
        {
            error = 89 as std::os::raw::c_int as std::os::raw::c_uint;
        } else {
            key = lodepng_malloc(
                length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
            ) as *mut std::os::raw::c_char;
            if key.is_null() {
                error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
            } else {
                lodepng_memcpy(
                    key as *mut std::os::raw::c_void,
                    data as *const std::os::raw::c_void,
                    length as size_t,
                );
                *key.offset(length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                string2_begin = length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint);
                length = (if chunkLength < string2_begin as std::os::raw::c_ulong {
                    0 as std::os::raw::c_int as std::os::raw::c_ulong
                } else {
                    chunkLength.wrapping_sub(string2_begin as std::os::raw::c_ulong)
                }) as std::os::raw::c_uint;
                str = lodepng_malloc(
                    length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
                ) as *mut std::os::raw::c_char;
                if str.is_null() {
                    error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                } else {
                    lodepng_memcpy(
                        str as *mut std::os::raw::c_void,
                        data.offset(string2_begin as isize) as *const std::os::raw::c_void,
                        length as size_t,
                    );
                    *str.offset(length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                    error = lodepng_add_text(info, key, str);
                }
            }
        }
    }
    lodepng_free(key as *mut std::os::raw::c_void);
    lodepng_free(str as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn readChunk_zTXt<'a1>(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut decoder: Option<&'a1 crate::src::lodepng::LodePNGDecoderSettings>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut zlibsettings = (*(decoder).unwrap()).zlibsettings;
    let mut length: u32 = 0;
    let mut string2_begin: u32 = 0;
    let mut key = 0 as *mut std::os::raw::c_char;
    let mut str = 0 as *mut std::os::raw::c_uchar;
    let mut size = 0 as std::os::raw::c_int as size_t;
    if error == 0 {
        length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while (length as std::os::raw::c_ulong) < chunkLength
            && *data.offset(length as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int
        {
            length = length.wrapping_add(1);
        }
        if length.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong
            >= chunkLength
        {
            error = 75 as std::os::raw::c_int as std::os::raw::c_uint;
        } else if length < 1 as std::os::raw::c_int as std::os::raw::c_uint
            || length > 79 as std::os::raw::c_int as std::os::raw::c_uint
        {
            error = 89 as std::os::raw::c_int as std::os::raw::c_uint;
        } else {
            key = lodepng_malloc(
                length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
            ) as *mut std::os::raw::c_char;
            if key.is_null() {
                error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
            } else {
                lodepng_memcpy(
                    key as *mut std::os::raw::c_void,
                    data as *const std::os::raw::c_void,
                    length as size_t,
                );
                *key.offset(length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                if *data
                    .offset(
                        length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize,
                    ) as std::os::raw::c_int != 0 as std::os::raw::c_int
                {
                    error = 72 as std::os::raw::c_int as std::os::raw::c_uint;
                } else {
                    string2_begin = length
                        .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint);
                    if string2_begin as std::os::raw::c_ulong > chunkLength {
                        error = 75 as std::os::raw::c_int as std::os::raw::c_uint;
                    } else {
                        length = (chunkLength as std::os::raw::c_uint)
                            .wrapping_sub(string2_begin);
                        zlibsettings.max_output_size = (*(decoder).unwrap()).max_text_size;
                        error = zlib_decompress(
                            &mut str,
                            &mut size,
                            0 as std::os::raw::c_int as size_t,
                            &*data.offset(string2_begin as isize),
                            length as size_t,
                            &mut zlibsettings,
                        );
                        if error != 0 && size > zlibsettings.max_output_size {
                            error = 112 as std::os::raw::c_int as std::os::raw::c_uint;
                        }
                        if !(error != 0) {
                            error = lodepng_add_text_sized(
                                info,
                                key,
                                str as *mut std::os::raw::c_char,
                                size,
                            );
                        }
                    }
                }
            }
        }
    }
    lodepng_free(key as *mut std::os::raw::c_void);
    lodepng_free(str as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn readChunk_iTXt<'a1>(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut decoder: Option<&'a1 crate::src::lodepng::LodePNGDecoderSettings>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u32 = 0;
    let mut zlibsettings = (*(decoder).unwrap()).zlibsettings;
    let mut length: u32 = 0;
    let mut begin: u32 = 0;
    let mut compressed: u32 = 0;
    let mut key = 0 as *mut std::os::raw::c_char;
    let mut langtag = 0 as *mut std::os::raw::c_char;
    let mut transkey = 0 as *mut std::os::raw::c_char;
    if error == 0 {
        if chunkLength < 5 as std::os::raw::c_int as std::os::raw::c_ulong {
            error = 30 as std::os::raw::c_int as std::os::raw::c_uint;
        } else {
            length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while (length as std::os::raw::c_ulong) < chunkLength
                && *data.offset(length as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int
            {
                length = length.wrapping_add(1);
            }
            if length.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong
                >= chunkLength
            {
                error = 75 as std::os::raw::c_int as std::os::raw::c_uint;
            } else if length < 1 as std::os::raw::c_int as std::os::raw::c_uint
                || length > 79 as std::os::raw::c_int as std::os::raw::c_uint
            {
                error = 89 as std::os::raw::c_int as std::os::raw::c_uint;
            } else {
                key = lodepng_malloc(
                    length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
                ) as *mut std::os::raw::c_char;
                if key.is_null() {
                    error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                } else {
                    lodepng_memcpy(
                        key as *mut std::os::raw::c_void,
                        data as *const std::os::raw::c_void,
                        length as size_t,
                    );
                    *key.offset(length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                    compressed = *data
                        .offset(
                            length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                as isize,
                        ) as std::os::raw::c_uint;
                    if *data
                        .offset(
                            length.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                                as isize,
                        ) as std::os::raw::c_int != 0 as std::os::raw::c_int
                    {
                        error = 72 as std::os::raw::c_int as std::os::raw::c_uint;
                    } else {
                        begin = length.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint);
                        length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        i = begin;
                        while (i as std::os::raw::c_ulong) < chunkLength
                            && *data.offset(i as isize) as std::os::raw::c_int
                                != 0 as std::os::raw::c_int
                        {
                            length = length.wrapping_add(1);
                            i = i.wrapping_add(1);
                        }
                        langtag = lodepng_malloc(
                            length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                as size_t,
                        ) as *mut std::os::raw::c_char;
                        if langtag.is_null() {
                            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                        } else {
                            lodepng_memcpy(
                                langtag as *mut std::os::raw::c_void,
                                data.offset(begin as isize) as *const std::os::raw::c_void,
                                length as size_t,
                            );
                            *langtag
                                .offset(length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                            begin = begin
                                .wrapping_add(
                                    length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint),
                                );
                            length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            i = begin;
                            while (i as std::os::raw::c_ulong) < chunkLength
                                && *data.offset(i as isize) as std::os::raw::c_int
                                    != 0 as std::os::raw::c_int
                            {
                                length = length.wrapping_add(1);
                                i = i.wrapping_add(1);
                            }
                            transkey = lodepng_malloc(
                                length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                    as size_t,
                            ) as *mut std::os::raw::c_char;
                            if transkey.is_null() {
                                error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                            } else {
                                lodepng_memcpy(
                                    transkey as *mut std::os::raw::c_void,
                                    data.offset(begin as isize) as *const std::os::raw::c_void,
                                    length as size_t,
                                );
                                *transkey
                                    .offset(length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                                begin = begin
                                    .wrapping_add(
                                        length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint),
                                    );
                                length = if (chunkLength as std::os::raw::c_uint) < begin {
                                    0 as std::os::raw::c_int as std::os::raw::c_uint
                                } else {
                                    (chunkLength as std::os::raw::c_uint).wrapping_sub(begin)
                                };
                                if compressed != 0 {
                                    let mut str = 0 as *mut std::os::raw::c_uchar;
                                    let mut size = 0 as std::os::raw::c_int as size_t;
                                    zlibsettings.max_output_size = (*(decoder).unwrap()).max_text_size;
                                    error = zlib_decompress(
                                        &mut str,
                                        &mut size,
                                        0 as std::os::raw::c_int as size_t,
                                        &*data.offset(begin as isize),
                                        length as size_t,
                                        &mut zlibsettings,
                                    );
                                    if error != 0 && size > zlibsettings.max_output_size {
                                        error = 112 as std::os::raw::c_int as std::os::raw::c_uint;
                                    }
                                    if error == 0 {
                                        error = lodepng_add_itext_sized(
                                            info,
                                            key,
                                            langtag,
                                            transkey,
                                            str as *mut std::os::raw::c_char,
                                            size,
                                        );
                                    }
                                    lodepng_free(str as *mut std::os::raw::c_void);
                                } else {
                                    error = lodepng_add_itext_sized(
                                        info,
                                        key,
                                        langtag,
                                        transkey,
                                        data.offset(begin as isize) as *mut std::os::raw::c_char,
                                        length as size_t,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    lodepng_free(key as *mut std::os::raw::c_void);
    lodepng_free(langtag as *mut std::os::raw::c_void);
    lodepng_free(transkey as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn readChunk_tIME<'a1>(
    mut info: Option<&'a1 mut crate::src::lodepng::LodePNGInfo>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    if chunkLength != 7 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 73 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*(borrow_mut(&mut info)).unwrap()).time_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap())
        .time
        .year = (256 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap()).time.month = *data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap()).time.day = *data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap()).time.hour = *data.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap()).time.minute = *data.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap()).time.second = *data.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_pHYs<'a1>(
    mut info: Option<&'a1 mut crate::src::lodepng::LodePNGInfo>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    if chunkLength != 9 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 74 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*(borrow_mut(&mut info)).unwrap()).phys_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap())
        .phys_x = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .phys_y = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap()).phys_unit = *data.offset(8 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_gAMA<'a1>(
    mut info: Option<&'a1 mut crate::src::lodepng::LodePNGInfo>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    if chunkLength != 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 96 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*(borrow_mut(&mut info)).unwrap()).gama_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap())
        .gama_gamma = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_cHRM<'a1>(
    mut info: Option<&'a1 mut crate::src::lodepng::LodePNGInfo>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    if chunkLength != 32 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 97 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*(borrow_mut(&mut info)).unwrap()).chrm_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_white_x = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_white_y = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_red_x = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(8 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(9 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(10 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(11 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_red_y = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(12 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(13 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(14 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(15 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_green_x = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(16 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(17 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(18 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(19 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_green_y = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(20 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(21 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(22 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(23 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_blue_x = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(24 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(25 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(26 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(27 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    (*(borrow_mut(&mut info)).unwrap())
        .chrm_blue_y = (16777216 as std::os::raw::c_uint)
        .wrapping_mul(*data.offset(28 as std::os::raw::c_int as isize) as std::os::raw::c_uint)
        .wrapping_add(
            (65536 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(29 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(
            (256 as std::os::raw::c_uint)
                .wrapping_mul(*data.offset(30 as std::os::raw::c_int as isize) as std::os::raw::c_uint),
        )
        .wrapping_add(*data.offset(31 as std::os::raw::c_int as isize) as std::os::raw::c_uint);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_sRGB<'a1>(
    mut info: Option<&'a1 mut crate::src::lodepng::LodePNGInfo>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    if chunkLength != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 98 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*(borrow_mut(&mut info)).unwrap()).srgb_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut info)).unwrap()).srgb_intent = *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn readChunk_iCCP<'a1>(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut decoder: Option<&'a1 crate::src::lodepng::LodePNGDecoderSettings>,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut i: u32 = 0;
    let mut size = 0 as std::os::raw::c_int as size_t;
    let mut zlibsettings = (*(decoder).unwrap()).zlibsettings;
    let mut length: u32 = 0;
    let mut string2_begin: u32 = 0;
    (*info).iccp_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    if !((*info).iccp_name).is_null() {
        lodepng_clear_icc(info);
    }
    length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while (length as std::os::raw::c_ulong) < chunkLength
        && *data.offset(length as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int
    {
        length = length.wrapping_add(1);
    }
    if length.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong
        >= chunkLength
    {
        return 75 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if length < 1 as std::os::raw::c_int as std::os::raw::c_uint
        || length > 79 as std::os::raw::c_int as std::os::raw::c_uint
    {
        return 89 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let ref mut fresh177 = (*info).iccp_name;
    *fresh177 = lodepng_malloc(
        length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as size_t,
    ) as *mut std::os::raw::c_char;
    if ((*info).iccp_name).is_null() {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    *((*info).iccp_name).offset(length as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i != length {
        *((*info).iccp_name)
            .offset(i as isize) = *data.offset(i as isize) as std::os::raw::c_char;
        i = i.wrapping_add(1);
    }
    if *data.offset(length.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as isize)
        as std::os::raw::c_int != 0 as std::os::raw::c_int
    {
        return 72 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    string2_begin = length.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint);
    if string2_begin as std::os::raw::c_ulong > chunkLength {
        return 75 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    length = (chunkLength as std::os::raw::c_uint).wrapping_sub(string2_begin);
    zlibsettings.max_output_size = (*(decoder).unwrap()).max_icc_size;
    error = zlib_decompress(
        &mut (*info).iccp_profile,
        &mut size,
        0 as std::os::raw::c_int as size_t,
        &*data.offset(string2_begin as isize),
        length as size_t,
        &mut zlibsettings,
    );
    if error != 0 && size > zlibsettings.max_output_size {
        error = 113 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    (*info).iccp_profile_size = size as std::os::raw::c_uint;
    if error == 0 && (*info).iccp_profile_size == 0 {
        error = 100 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    return error;
}
unsafe extern "C" fn readChunk_sBIT(
    mut info: * mut crate::src::lodepng::LodePNGInfo,
    mut data: * const std::os::raw::c_uchar,
    mut chunkLength: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut bitdepth = if (*info).color.colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        8 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        (*info).color.bitdepth
    };
    if (*info).color.colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 114 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
            || *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*info).sbit_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        let ref mut fresh178 = (*info).sbit_b;
        *fresh178 = *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        let ref mut fresh179 = (*info).sbit_g;
        *fresh179 = *fresh178;
        (*info).sbit_r = *fresh179;
    } else if (*info).color.colortype as std::os::raw::c_uint
        == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
        || (*info).color.colortype as std::os::raw::c_uint
            == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 3 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 114 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
            || *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
            || *data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
            || *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
            || *data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*info).sbit_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        (*info).sbit_r = *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        (*info).sbit_g = *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        (*info).sbit_b = *data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    } else if (*info).color.colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 2 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 114 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
            || *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
            || *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*info).sbit_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        let ref mut fresh180 = (*info).sbit_b;
        *fresh180 = *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        let ref mut fresh181 = (*info).sbit_g;
        *fresh181 = *fresh180;
        (*info).sbit_r = *fresh181;
        (*info).sbit_a = *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    } else if (*info).color.colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if chunkLength != 4 as std::os::raw::c_int as std::os::raw::c_ulong {
            return 114 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
            || *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
            || *data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
            || *data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
            || *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
            || *data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
            || *data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint > bitdepth
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        (*info).sbit_defined = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        (*info).sbit_r = *data.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        (*info).sbit_g = *data.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        (*info).sbit_b = *data.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        (*info).sbit_a = *data.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_inspect_chunk<'a1>(
    mut state: Option<&'a1 mut crate::src::lodepng::LodePNGState>,
    mut pos: std::os::raw::c_ulong,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut chunk = in_0.offset(pos as isize);
    let mut chunkLength: u32 = 0;
    let mut data = 0 as *const std::os::raw::c_uchar;
    let mut unhandled = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if pos.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) > insize {
        return 30 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    chunkLength = lodepng_chunk_length(chunk);
    if chunkLength > 2147483647 as std::os::raw::c_int as std::os::raw::c_uint {
        return 63 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    data = lodepng_chunk_data_const(chunk);
    if chunkLength.wrapping_add(12 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_ulong
        > insize.wrapping_sub(pos)
    {
        return 30 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if lodepng_chunk_type_equals(chunk, b"PLTE\0" as *const u8 as *const std::os::raw::c_char)
        != 0
    {
        error = readChunk_PLTE(
            &mut (*(borrow_mut(&mut state)).unwrap()).info_png.color,
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(
        chunk,
        b"tRNS\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_tRNS(
            &mut (*(borrow_mut(&mut state)).unwrap()).info_png.color,
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(
        chunk,
        b"bKGD\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_bKGD(&mut (*(borrow_mut(&mut state)).unwrap()).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(
        chunk,
        b"tEXt\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_tEXt(&mut (*(borrow_mut(&mut state)).unwrap()).info_png, data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(
        chunk,
        b"zTXt\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_zTXt(
            &mut (*(borrow_mut(&mut state)).unwrap()).info_png,
            Some(&mut (*(borrow_mut(&mut state)).unwrap()).decoder),
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(
        chunk,
        b"iTXt\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_iTXt(
            &mut (*(borrow_mut(&mut state)).unwrap()).info_png,
            Some(&mut (*(borrow_mut(&mut state)).unwrap()).decoder),
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(
        chunk,
        b"tIME\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_tIME(Some(&mut (*(borrow_mut(&mut state)).unwrap()).info_png), data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(
        chunk,
        b"pHYs\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_pHYs(Some(&mut (*(borrow_mut(&mut state)).unwrap()).info_png), data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(
        chunk,
        b"gAMA\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_gAMA(Some(&mut (*(borrow_mut(&mut state)).unwrap()).info_png), data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(
        chunk,
        b"cHRM\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_cHRM(Some(&mut (*(borrow_mut(&mut state)).unwrap()).info_png), data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(
        chunk,
        b"sRGB\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_sRGB(Some(&mut (*(borrow_mut(&mut state)).unwrap()).info_png), data, chunkLength as size_t);
    } else if lodepng_chunk_type_equals(
        chunk,
        b"iCCP\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_iCCP(
            &mut (*(borrow_mut(&mut state)).unwrap()).info_png,
            Some(&mut (*(borrow_mut(&mut state)).unwrap()).decoder),
            data,
            chunkLength as size_t,
        );
    } else if lodepng_chunk_type_equals(
        chunk,
        b"sBIT\0" as *const u8 as *const std::os::raw::c_char,
    ) != 0
    {
        error = readChunk_sBIT(&mut (*(borrow_mut(&mut state)).unwrap()).info_png, data, chunkLength as size_t);
    } else {
        unhandled = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if error == 0 && unhandled == 0 && (*(borrow(& state)).unwrap()).decoder.ignore_crc == 0 {
        if lodepng_chunk_check_crc(chunk) != 0 {
            return 57 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    return error;
}
unsafe extern "C" fn decodeGeneric<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut state: * mut crate::src::lodepng::LodePNGState,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
) {
    let mut IEND = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    let mut chunk = 0 as *const std::os::raw::c_uchar;
    let mut idat = 0 as *mut std::os::raw::c_uchar;
    let mut idatsize = 0 as std::os::raw::c_int as size_t;
    let mut scanlines = 0 as *mut std::os::raw::c_uchar;
    let mut scanlines_size = 0 as std::os::raw::c_int as size_t;
    let mut expected_size = 0 as std::os::raw::c_int as size_t;
    let mut outsize = 0 as std::os::raw::c_int as size_t;
    let mut unknown = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut critical_pos = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    *(borrow_mut(&mut out)).unwrap() = 0 as *mut std::os::raw::c_uchar;
    *(borrow_mut(&mut h)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    *(borrow_mut(&mut w)).unwrap() = *(borrow_mut(&mut h)).unwrap();
    (*state).error = lodepng_inspect(borrow_mut(&mut w), borrow_mut(&mut h), state, in_0, insize);
    if (*state).error != 0 {
        return;
    }
    if lodepng_pixel_overflow(
        *(borrow_mut(&mut w)).unwrap(),
        *(borrow_mut(&mut h)).unwrap(),
        Some(&mut (*state).info_png.color),
        Some(&mut (*state).info_raw),
    ) != 0
    {
        (*state).error = 92 as std::os::raw::c_int as std::os::raw::c_uint;
        return;
    }
    idat = lodepng_malloc(insize) as *mut std::os::raw::c_uchar;
    if idat.is_null() {
        (*state).error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        return;
    }
    chunk = &*in_0.offset(33 as std::os::raw::c_int as isize) as *const std::os::raw::c_uchar;
    while IEND == 0 && (*state).error == 0 {
        let mut chunkLength: u32 = 0;
        let mut data = 0 as *const std::os::raw::c_uchar;
        let mut pos = chunk.offset_from(in_0) as std::os::raw::c_long as size_t;
        if chunk < in_0 || pos.wrapping_add(12 as std::os::raw::c_int as std::os::raw::c_ulong) > insize
        {
            if (*state).decoder.ignore_end != 0 {
                break;
            }
            (*state).error = 30 as std::os::raw::c_int as std::os::raw::c_uint;
            break;
        } else {
            chunkLength = lodepng_chunk_length(chunk);
            if chunkLength > 2147483647 as std::os::raw::c_int as std::os::raw::c_uint {
                if (*state).decoder.ignore_end != 0 {
                    break;
                }
                (*state).error = 63 as std::os::raw::c_int as std::os::raw::c_uint;
                break;
            } else if pos
                .wrapping_add(chunkLength as size_t)
                .wrapping_add(12 as std::os::raw::c_int as std::os::raw::c_ulong) > insize
                || pos
                    .wrapping_add(chunkLength as size_t)
                    .wrapping_add(12 as std::os::raw::c_int as std::os::raw::c_ulong) < pos
            {
                (*state).error = 64 as std::os::raw::c_int as std::os::raw::c_uint;
                break;
            } else {
                data = lodepng_chunk_data_const(chunk);
                unknown = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                if lodepng_chunk_type_equals(
                    chunk,
                    b"IDAT\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    let mut newsize: u64 = 0;
                    if lodepng_addofl(idatsize, chunkLength as size_t, Some(&mut newsize)) != 0
                    {
                        (*state).error = 95 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    } else if newsize > insize {
                        (*state).error = 95 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    } else {
                        lodepng_memcpy(
                            idat.offset(idatsize as isize) as *mut std::os::raw::c_void,
                            data as *const std::os::raw::c_void,
                            chunkLength as size_t,
                        );
                        idatsize = (idatsize as std::os::raw::c_ulong)
                            .wrapping_add(chunkLength as std::os::raw::c_ulong) as size_t
                            as size_t;
                        critical_pos = 3 as std::os::raw::c_int as std::os::raw::c_uint;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"IEND\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    IEND = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"PLTE\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_PLTE(
                        &mut (*state).info_png.color,
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                    critical_pos = 2 as std::os::raw::c_int as std::os::raw::c_uint;
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"tRNS\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_tRNS(
                        &mut (*state).info_png.color,
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"bKGD\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_bKGD(
                        &mut (*state).info_png,
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"tEXt\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    if (*state).decoder.read_text_chunks != 0 {
                        (*state)
                            .error = readChunk_tEXt(
                            &mut (*state).info_png,
                            data,
                            chunkLength as size_t,
                        );
                        if (*state).error != 0 {
                            break;
                        }
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"zTXt\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    if (*state).decoder.read_text_chunks != 0 {
                        (*state)
                            .error = readChunk_zTXt(
                            &mut (*state).info_png,
                            Some(&mut (*state).decoder),
                            data,
                            chunkLength as size_t,
                        );
                        if (*state).error != 0 {
                            break;
                        }
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"iTXt\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    if (*state).decoder.read_text_chunks != 0 {
                        (*state)
                            .error = readChunk_iTXt(
                            &mut (*state).info_png,
                            Some(&mut (*state).decoder),
                            data,
                            chunkLength as size_t,
                        );
                        if (*state).error != 0 {
                            break;
                        }
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"tIME\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_tIME(
                        Some(&mut (*state).info_png),
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"pHYs\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_pHYs(
                        Some(&mut (*state).info_png),
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"gAMA\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_gAMA(
                        Some(&mut (*state).info_png),
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"cHRM\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_cHRM(
                        Some(&mut (*state).info_png),
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"sRGB\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_sRGB(
                        Some(&mut (*state).info_png),
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"iCCP\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_iCCP(
                        &mut (*state).info_png,
                        Some(&mut (*state).decoder),
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if lodepng_chunk_type_equals(
                    chunk,
                    b"sBIT\0" as *const u8 as *const std::os::raw::c_char,
                ) != 0
                {
                    (*state)
                        .error = readChunk_sBIT(
                        &mut (*state).info_png,
                        data,
                        chunkLength as size_t,
                    );
                    if (*state).error != 0 {
                        break;
                    }
                } else if (*state).decoder.ignore_critical == 0
                    && lodepng_chunk_ancillary(chunk) == 0
                {
                    (*state).error = 69 as std::os::raw::c_int as std::os::raw::c_uint;
                    break;
                } else {
                    unknown = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                    if (*state).decoder.remember_unknown_chunks != 0 {
                        (*state)
                            .error = lodepng_chunk_append(
                            Some(&mut *((*state).info_png.unknown_chunks_data)
                                .as_mut_ptr()
                                .offset(
                                    critical_pos.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                        as isize,
                                )),
                            Some(&mut *((*state).info_png.unknown_chunks_size)
                                .as_mut_ptr()
                                .offset(
                                    critical_pos.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                        as isize,
                                )),
                            chunk,
                        );
                        if (*state).error != 0 {
                            break;
                        }
                    }
                }
                if (*state).decoder.ignore_crc == 0 && unknown == 0 {
                    if lodepng_chunk_check_crc(chunk) != 0 {
                        (*state).error = 57 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    }
                }
                if IEND == 0 {
                    chunk = lodepng_chunk_next_const(
                        chunk,
                        in_0.offset(insize as isize),
                    );
                }
            }
        }
    }
    if (*state).error == 0
        && (*state).info_png.color.colortype as std::os::raw::c_uint
            == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
        && ((*state).info_png.color.palette).is_null()
    {
        (*state).error = 106 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if (*state).error == 0 {
        if (*state).info_png.interlace_method == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut bpp = lodepng_get_bpp(Some(&mut (*state).info_png.color)) as size_t;
            expected_size = lodepng_get_raw_size_idat(*(borrow_mut(&mut w)).unwrap(), *(borrow_mut(&mut h)).unwrap(), bpp as std::os::raw::c_uint);
        } else {
            let mut bpp_0 = lodepng_get_bpp(Some(&mut (*state).info_png.color)) as size_t;
            expected_size = 0 as std::os::raw::c_int as size_t;
            expected_size = (expected_size as std::os::raw::c_ulong)
                .wrapping_add(
                    lodepng_get_raw_size_idat(
                        (*(borrow(& w)).unwrap()).wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_uint)
                            >> 3 as std::os::raw::c_int,
                        (*(borrow(& h)).unwrap()).wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_uint)
                            >> 3 as std::os::raw::c_int,
                        bpp_0 as std::os::raw::c_uint,
                    ),
                ) as size_t as size_t;
            if *(borrow(& w)).unwrap() > 4 as std::os::raw::c_int as std::os::raw::c_uint {
                expected_size = (expected_size as std::os::raw::c_ulong)
                    .wrapping_add(
                        lodepng_get_raw_size_idat(
                            (*(borrow(& w)).unwrap()).wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint)
                                >> 3 as std::os::raw::c_int,
                            (*(borrow(& h)).unwrap()).wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_uint)
                                >> 3 as std::os::raw::c_int,
                            bpp_0 as std::os::raw::c_uint,
                        ),
                    ) as size_t as size_t;
            }
            expected_size = (expected_size as std::os::raw::c_ulong)
                .wrapping_add(
                    lodepng_get_raw_size_idat(
                        (*(borrow(& w)).unwrap()).wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint)
                            >> 2 as std::os::raw::c_int,
                        (*(borrow(& h)).unwrap()).wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint)
                            >> 3 as std::os::raw::c_int,
                        bpp_0 as std::os::raw::c_uint,
                    ),
                ) as size_t as size_t;
            if *(borrow(& w)).unwrap() > 2 as std::os::raw::c_int as std::os::raw::c_uint {
                expected_size = (expected_size as std::os::raw::c_ulong)
                    .wrapping_add(
                        lodepng_get_raw_size_idat(
                            (*(borrow(& w)).unwrap()).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                >> 2 as std::os::raw::c_int,
                            (*(borrow(& h)).unwrap()).wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_uint)
                                >> 2 as std::os::raw::c_int,
                            bpp_0 as std::os::raw::c_uint,
                        ),
                    ) as size_t as size_t;
            }
            expected_size = (expected_size as std::os::raw::c_ulong)
                .wrapping_add(
                    lodepng_get_raw_size_idat(
                        (*(borrow(& w)).unwrap()).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                            >> 1 as std::os::raw::c_int,
                        (*(borrow(& h)).unwrap()).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                            >> 2 as std::os::raw::c_int,
                        bpp_0 as std::os::raw::c_uint,
                    ),
                ) as size_t as size_t;
            if *(borrow(& w)).unwrap() > 1 as std::os::raw::c_int as std::os::raw::c_uint {
                expected_size = (expected_size as std::os::raw::c_ulong)
                    .wrapping_add(
                        lodepng_get_raw_size_idat(
                            (*(borrow(& w)).unwrap()).wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint)
                                >> 1 as std::os::raw::c_int,
                            (*(borrow(& h)).unwrap()).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint)
                                >> 1 as std::os::raw::c_int,
                            bpp_0 as std::os::raw::c_uint,
                        ),
                    ) as size_t as size_t;
            }
            expected_size = (expected_size as std::os::raw::c_ulong)
                .wrapping_add(
                    lodepng_get_raw_size_idat(
                        (*(borrow(& w)).unwrap()).wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint),
                        (*(borrow(& h)).unwrap()).wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_uint)
                            >> 1 as std::os::raw::c_int,
                        bpp_0 as std::os::raw::c_uint,
                    ),
                ) as size_t as size_t;
        }
        (*state)
            .error = zlib_decompress(
            &mut scanlines,
            &mut scanlines_size,
            expected_size,
            idat,
            idatsize,
            &mut (*state).decoder.zlibsettings,
        );
    }
    if (*state).error == 0 && scanlines_size != expected_size {
        (*state).error = 91 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    lodepng_free(idat as *mut std::os::raw::c_void);
    if (*state).error == 0 {
        outsize = lodepng_get_raw_size(*(borrow_mut(&mut w)).unwrap(), *(borrow_mut(&mut h)).unwrap(), Some(&mut (*state).info_png.color));
        *(borrow_mut(&mut out)).unwrap() = lodepng_malloc(outsize) as *mut std::os::raw::c_uchar;
        if (*(borrow_mut(&mut out)).unwrap()).is_null() {
            (*state).error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
    }
    if (*state).error == 0 {
        lodepng_memset(*(borrow_mut(&mut out)).unwrap() as *mut std::os::raw::c_void, 0 as std::os::raw::c_int, outsize);
        (*state)
            .error = postProcessScanlines(
            *(borrow_mut(&mut out)).unwrap(),
            scanlines,
            *(borrow_mut(&mut w)).unwrap(),
            *(borrow_mut(&mut h)).unwrap(),
            Some(&mut (*state).info_png),
        );
    }
    lodepng_free(scanlines as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut state: * mut crate::src::lodepng::LodePNGState,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    *(borrow_mut(&mut out)).unwrap() = 0 as *mut std::os::raw::c_uchar;
    decodeGeneric(borrow_mut(&mut out), borrow_mut(&mut w), borrow_mut(&mut h), state, in_0, insize);
    if (*state).error != 0 {
        return (*state).error;
    }
    if (*state).decoder.color_convert == 0
        || lodepng_color_mode_equal(Some(&mut (*state).info_raw), Some(&mut (*state).info_png.color))
            != 0
    {
        if (*state).decoder.color_convert == 0 {
            (*state)
                .error = lodepng_color_mode_copy(
                &mut (*state).info_raw,
                &mut (*state).info_png.color,
            );
            if (*state).error != 0 {
                return (*state).error;
            }
        }
    } else {
        let mut data = *(borrow_mut(&mut out)).unwrap();
        let mut outsize: u64 = 0;
        if !((*state).info_raw.colortype as std::os::raw::c_uint
            == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
            || (*state).info_raw.colortype as std::os::raw::c_uint
                == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint)
            && !((*state).info_raw.bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint)
        {
            return 56 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        outsize = lodepng_get_raw_size(*(borrow_mut(&mut w)).unwrap(), *(borrow_mut(&mut h)).unwrap(), Some(&mut (*state).info_raw));
        *(borrow_mut(&mut out)).unwrap() = lodepng_malloc(outsize) as *mut std::os::raw::c_uchar;
        if (*(borrow_mut(&mut out)).unwrap()).is_null() {
            (*state).error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        } else {
            (*state)
                .error = lodepng_convert(
                *(borrow_mut(&mut out)).unwrap(),
                data,
                Some(&mut (*state).info_raw),
                Some(&mut (*state).info_png.color),
                *(borrow_mut(&mut w)).unwrap(),
                *(borrow_mut(&mut h)).unwrap(),
            );
        }
        lodepng_free(data as *mut std::os::raw::c_void);
    }
    return (*state).error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode_memory<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut error: u32 = 0;
    let mut state = LodePNGState {
        decoder: LodePNGDecoderSettings {
            zlibsettings: LodePNGDecompressSettings {
                ignore_adler32: 0,
                ignore_nlen: 0,
                max_output_size: 0,
                custom_zlib: None,
                custom_inflate: None,
                custom_context: 0 as *const std::os::raw::c_void,
            },
            ignore_crc: 0,
            ignore_critical: 0,
            ignore_end: 0,
            color_convert: 0,
            read_text_chunks: 0,
            remember_unknown_chunks: 0,
            max_text_size: 0,
            max_icc_size: 0,
        },
        encoder: LodePNGEncoderSettings {
            zlibsettings: LodePNGCompressSettings {
                btype: 0,
                use_lz77: 0,
                windowsize: 0,
                minmatch: 0,
                nicematch: 0,
                lazymatching: 0,
                custom_zlib: None,
                custom_deflate: None,
                custom_context: 0 as *const std::os::raw::c_void,
            },
            auto_convert: 0,
            filter_palette_zero: 0,
            filter_strategy: LFS_ZERO,
            predefined_filters: (0 as * const u8),
            force_palette: 0,
            add_id: 0,
            text_compression: 0,
        },
        info_raw: LodePNGColorMode {
            colortype: LCT_GREY,
            bitdepth: 0,
            palette: 0 as *mut std::os::raw::c_uchar,
            palettesize: 0,
            key_defined: 0,
            key_r: 0,
            key_g: 0,
            key_b: 0,
        },
        info_png: LodePNGInfo {
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            color: LodePNGColorMode {
                colortype: LCT_GREY,
                bitdepth: 0,
                palette: 0 as *mut std::os::raw::c_uchar,
                palettesize: 0,
                key_defined: 0,
                key_r: 0,
                key_g: 0,
                key_b: 0,
            },
            background_defined: 0,
            background_r: 0,
            background_g: 0,
            background_b: 0,
            text_num: 0,
            text_keys: 0 as *mut *mut std::os::raw::c_char,
            text_strings: 0 as *mut *mut std::os::raw::c_char,
            itext_num: 0,
            itext_keys: 0 as *mut *mut std::os::raw::c_char,
            itext_langtags: 0 as *mut *mut std::os::raw::c_char,
            itext_transkeys: 0 as *mut *mut std::os::raw::c_char,
            itext_strings: 0 as *mut *mut std::os::raw::c_char,
            time_defined: 0,
            time: LodePNGTime {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
            },
            phys_defined: 0,
            phys_x: 0,
            phys_y: 0,
            phys_unit: 0,
            gama_defined: 0,
            gama_gamma: 0,
            chrm_defined: 0,
            chrm_white_x: 0,
            chrm_white_y: 0,
            chrm_red_x: 0,
            chrm_red_y: 0,
            chrm_green_x: 0,
            chrm_green_y: 0,
            chrm_blue_x: 0,
            chrm_blue_y: 0,
            srgb_defined: 0,
            srgb_intent: 0,
            iccp_defined: 0,
            iccp_name: 0 as *mut std::os::raw::c_char,
            iccp_profile: 0 as *mut std::os::raw::c_uchar,
            iccp_profile_size: 0,
            sbit_defined: 0,
            sbit_r: 0,
            sbit_g: 0,
            sbit_b: 0,
            sbit_a: 0,
            unknown_chunks_data: [0 as *mut std::os::raw::c_uchar; 3],
            unknown_chunks_size: [0; 3],
        },
        error: 0,
    };
    lodepng_state_init(Some(&mut state));
    state.info_raw.colortype = colortype;
    state.info_raw.bitdepth = bitdepth;
    state.decoder.read_text_chunks = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    state.decoder.remember_unknown_chunks = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    error = lodepng_decode(borrow_mut(&mut out), borrow_mut(&mut w), borrow_mut(&mut h), &mut state, in_0, insize);
    lodepng_state_cleanup(Some(&mut state));
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode32<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    return lodepng_decode_memory(
        borrow_mut(&mut out),
        borrow_mut(&mut w),
        borrow_mut(&mut h),
        in_0,
        insize,
        LCT_RGBA,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode24<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut in_0: * const std::os::raw::c_uchar,
    mut insize: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    return lodepng_decode_memory(
        borrow_mut(&mut out),
        borrow_mut(&mut w),
        borrow_mut(&mut h),
        in_0,
        insize,
        LCT_RGB,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode_file<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut filename: * const std::os::raw::c_char,
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut buffer = 0 as *mut std::os::raw::c_uchar;
    let mut buffersize: u64 = 0;
    let mut error: u32 = 0;
    *(borrow_mut(&mut out)).unwrap() = 0 as *mut std::os::raw::c_uchar;
    *(borrow_mut(&mut h)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    *(borrow_mut(&mut w)).unwrap() = *(borrow_mut(&mut h)).unwrap();
    error = lodepng_load_file(Some(&mut buffer), Some(&mut buffersize), filename);
    if error == 0 {
        error = lodepng_decode_memory(
            borrow_mut(&mut out),
            borrow_mut(&mut w),
            borrow_mut(&mut h),
            buffer,
            buffersize,
            colortype,
            bitdepth,
        );
    }
    lodepng_free(buffer as *mut std::os::raw::c_void);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode32_file<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut filename: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    return lodepng_decode_file(
        borrow_mut(&mut out),
        borrow_mut(&mut w),
        borrow_mut(&mut h),
        filename,
        LCT_RGBA,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decode24_file<'a1, 'a2, 'a3>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut w: Option<&'a2 mut std::os::raw::c_uint>,
    mut h: Option<&'a3 mut std::os::raw::c_uint>,
    mut filename: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    return lodepng_decode_file(
        borrow_mut(&mut out),
        borrow_mut(&mut w),
        borrow_mut(&mut h),
        filename,
        LCT_RGB,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_decoder_settings_init<'a1>(
    mut settings: Option<&'a1 mut crate::src::lodepng::LodePNGDecoderSettings>,
) {
    (*(borrow_mut(&mut settings)).unwrap()).color_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).read_text_chunks = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).remember_unknown_chunks = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).max_text_size = 16777216 as std::os::raw::c_int as size_t;
    (*(borrow_mut(&mut settings)).unwrap()).max_icc_size = 16777216 as std::os::raw::c_int as size_t;
    (*(borrow_mut(&mut settings)).unwrap()).ignore_crc = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).ignore_critical = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).ignore_end = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    lodepng_decompress_settings_init(Some(&mut (*(borrow_mut(&mut settings)).unwrap()).zlibsettings));
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_state_init<'a1>(mut state: Option<&'a1 mut crate::src::lodepng::LodePNGState>) {
    lodepng_decoder_settings_init(Some(&mut (*(borrow_mut(&mut state)).unwrap()).decoder));
    lodepng_encoder_settings_init(Some(&mut (*(borrow_mut(&mut state)).unwrap()).encoder));
    lodepng_color_mode_init(&mut (*(borrow_mut(&mut state)).unwrap()).info_raw);
    lodepng_info_init(&mut (*(borrow_mut(&mut state)).unwrap()).info_png);
    (*(borrow_mut(&mut state)).unwrap()).error = 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_state_cleanup<'a1>(mut state: Option<&'a1 mut crate::src::lodepng::LodePNGState>) {
    lodepng_color_mode_cleanup(&mut (*(borrow_mut(&mut state)).unwrap()).info_raw);
    lodepng_info_cleanup(&mut (*(borrow_mut(&mut state)).unwrap()).info_png);
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_state_copy<'a1, 'a2>(
    mut dest: Option<&'a1 mut crate::src::lodepng::LodePNGState>,
    mut source: Option<&'a2 crate::src::lodepng::LodePNGState>,
) {
    lodepng_state_cleanup(borrow_mut(&mut dest));
    *(borrow_mut(&mut dest)).unwrap() = *(source).unwrap();
    lodepng_color_mode_init(&mut (*(borrow_mut(&mut dest)).unwrap()).info_raw);
    lodepng_info_init(&mut (*(borrow_mut(&mut dest)).unwrap()).info_png);
    (*(borrow_mut(&mut dest)).unwrap()).error = lodepng_color_mode_copy(&mut (*(borrow_mut(&mut dest)).unwrap()).info_raw, &(*((source).clone()).unwrap()).info_raw);
    if (*(borrow(& dest)).unwrap()).error != 0 {
        return;
    }
    (*(borrow_mut(&mut dest)).unwrap()).error = lodepng_info_copy(&mut (*(borrow_mut(&mut dest)).unwrap()).info_png, &(*((source).clone()).unwrap()).info_png);
    if (*(borrow(& dest)).unwrap()).error != 0 {
        return;
    }
}
unsafe extern "C" fn writeSignature(mut out: * mut crate::src::lodepng::ucvector) -> std::os::raw::c_uint {
    let mut pos = (*out).size;
    let signature: [u8; 8] = [
        137 as std::os::raw::c_int as std::os::raw::c_uchar,
        80 as std::os::raw::c_int as std::os::raw::c_uchar,
        78 as std::os::raw::c_int as std::os::raw::c_uchar,
        71 as std::os::raw::c_int as std::os::raw::c_uchar,
        13 as std::os::raw::c_int as std::os::raw::c_uchar,
        10 as std::os::raw::c_int as std::os::raw::c_uchar,
        26 as std::os::raw::c_int as std::os::raw::c_uchar,
        10 as std::os::raw::c_int as std::os::raw::c_uchar,
    ];
    if ucvector_resize(
        out,
        ((*out).size).wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) == 0
    {
        return 83 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    lodepng_memcpy(
        ((*out).data).offset(pos as isize) as *mut std::os::raw::c_void,
        signature.as_ptr() as *const std::os::raw::c_void,
        8 as std::os::raw::c_int as size_t,
    );
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_IHDR(
    mut out: * mut crate::src::lodepng::ucvector,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
    mut interlace_method: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut data = 0 as *mut std::os::raw::c_uchar;
    let mut error = lodepng_chunk_init(
        Some(&mut chunk),
        out,
        13 as std::os::raw::c_int as std::os::raw::c_uint,
        b"IHDR\0" as *const u8 as *const std::os::raw::c_char,
    );
    if error != 0 {
        return error;
    }
    data = chunk.offset(8 as std::os::raw::c_int as isize);
    lodepng_set32bitInt(data.offset(0 as std::os::raw::c_int as isize), w);
    lodepng_set32bitInt(data.offset(4 as std::os::raw::c_int as isize), h);
    *data.offset(8 as std::os::raw::c_int as isize) = bitdepth as std::os::raw::c_uchar;
    *data.offset(9 as std::os::raw::c_int as isize) = colortype as std::os::raw::c_uchar;
    *data.offset(10 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    *data.offset(11 as std::os::raw::c_int as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    *data.offset(12 as std::os::raw::c_int as isize) = interlace_method as std::os::raw::c_uchar;
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_PLTE<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut i: u64 = 0;
    let mut j = 8 as std::os::raw::c_int as size_t;
    if (*((info).clone()).unwrap()).palettesize == 0 as std::os::raw::c_int as std::os::raw::c_ulong
        || (*((info).clone()).unwrap()).palettesize > 256 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 68 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let mut error = lodepng_chunk_init(
        Some(&mut chunk),
        out,
        ((*((info).clone()).unwrap()).palettesize).wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
            as std::os::raw::c_uint,
        b"PLTE\0" as *const u8 as *const std::os::raw::c_char,
    );
    if error != 0 {
        return error;
    }
    i = 0 as std::os::raw::c_int as size_t;
    while i != (*((info).clone()).unwrap()).palettesize {
        let mut fresh182 = j;
        j = j.wrapping_add(1);
        *chunk
            .offset(
                fresh182 as isize,
            ) = *((*((info).clone()).unwrap()).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            );
        let mut fresh183 = j;
        j = j.wrapping_add(1);
        *chunk
            .offset(
                fresh183 as isize,
            ) = *((*((info).clone()).unwrap()).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            );
        let mut fresh184 = j;
        j = j.wrapping_add(1);
        *chunk
            .offset(
                fresh184 as isize,
            ) = *((*((info).clone()).unwrap()).palette)
            .offset(
                i
                    .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
    }
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_tRNS<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    if (*((info).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint {
        let mut i: u64 = 0;
        let mut amount = (*(info).unwrap()).palettesize;
        i = (*(info).unwrap()).palettesize;
        while i != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            if *((*((info).clone()).unwrap()).palette)
                .offset(
                    (4 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(i.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong))
                        .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                ) as std::os::raw::c_int != 255 as std::os::raw::c_int
            {
                break;
            }
            amount = amount.wrapping_sub(1);
            i = i.wrapping_sub(1);
        }
        if amount != 0 {
            let mut error = lodepng_chunk_init(
                Some(&mut chunk),
                out,
                amount as std::os::raw::c_uint,
                b"tRNS\0" as *const u8 as *const std::os::raw::c_char,
            );
            if error != 0 {
                return error;
            }
            i = 0 as std::os::raw::c_int as size_t;
            while i != amount {
                *chunk
                    .offset(
                        (8 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(i) as isize,
                    ) = *((*((info).clone()).unwrap()).palette)
                    .offset(
                        (4 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_mul(i)
                            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    );
                i = i.wrapping_add(1);
            }
        }
    } else if (*((info).clone()).unwrap()).colortype as std::os::raw::c_uint
        == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((info).clone()).unwrap()).key_defined != 0 {
            let mut error_0 = lodepng_chunk_init(
                Some(&mut chunk),
                out,
                2 as std::os::raw::c_int as std::os::raw::c_uint,
                b"tRNS\0" as *const u8 as *const std::os::raw::c_char,
            );
            if error_0 != 0 {
                return error_0;
            }
            *chunk
                .offset(
                    8 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_r >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
            *chunk
                .offset(
                    9 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_r & 255 as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_uchar;
        }
    } else if (*((info).clone()).unwrap()).colortype as std::os::raw::c_uint == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((info).clone()).unwrap()).key_defined != 0 {
            let mut error_1 = lodepng_chunk_init(
                Some(&mut chunk),
                out,
                6 as std::os::raw::c_int as std::os::raw::c_uint,
                b"tRNS\0" as *const u8 as *const std::os::raw::c_char,
            );
            if error_1 != 0 {
                return error_1;
            }
            *chunk
                .offset(
                    8 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_r >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
            *chunk
                .offset(
                    9 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_r & 255 as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_uchar;
            *chunk
                .offset(
                    10 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_g >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
            *chunk
                .offset(
                    11 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_g & 255 as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_uchar;
            *chunk
                .offset(
                    12 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_b >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
            *chunk
                .offset(
                    13 as std::os::raw::c_int as isize,
                ) = ((*((info).clone()).unwrap()).key_b & 255 as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_uchar;
        }
    }
    if !chunk.is_null() {
        lodepng_chunk_generate_crc(chunk);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_IDAT(
    mut out: * mut crate::src::lodepng::ucvector,
    mut data: * const std::os::raw::c_uchar,
    mut datasize: std::os::raw::c_ulong,
    mut zlibsettings: * mut crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut zlib = 0 as *mut std::os::raw::c_uchar;
    let mut zlibsize = 0 as std::os::raw::c_int as size_t;
    error = zlib_compress(&mut zlib, &mut zlibsize, data, datasize, zlibsettings);
    if error == 0 {
        error = lodepng_chunk_createv(
            out,
            zlibsize as std::os::raw::c_uint,
            b"IDAT\0" as *const u8 as *const std::os::raw::c_char,
            zlib,
        );
    }
    lodepng_free(zlib as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn addChunk_IEND(mut out: * mut crate::src::lodepng::ucvector) -> std::os::raw::c_uint {
    return lodepng_chunk_createv(
        out,
        0 as std::os::raw::c_int as std::os::raw::c_uint,
        b"IEND\0" as *const u8 as *const std::os::raw::c_char,
        0 as *const std::os::raw::c_uchar,
    );
}
unsafe extern "C" fn addChunk_tEXt(
    mut out: * mut crate::src::lodepng::ucvector,
    mut keyword: * const std::os::raw::c_char,
    mut textstring: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut keysize = lodepng_strlen(keyword);
    let mut textsize = lodepng_strlen(textstring);
    let mut size = keysize
        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        .wrapping_add(textsize);
    if keysize < 1 as std::os::raw::c_int as std::os::raw::c_ulong
        || keysize > 79 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 89 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    let mut error = lodepng_chunk_init(
        Some(&mut chunk),
        out,
        size as std::os::raw::c_uint,
        b"tEXt\0" as *const u8 as *const std::os::raw::c_char,
    );
    if error != 0 {
        return error;
    }
    lodepng_memcpy(
        chunk.offset(8 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
        keyword as *const std::os::raw::c_void,
        keysize,
    );
    *chunk
        .offset(
            (8 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(keysize) as isize,
        ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    lodepng_memcpy(
        chunk.offset(9 as std::os::raw::c_int as isize).offset(keysize as isize)
            as *mut std::os::raw::c_void,
        textstring as *const std::os::raw::c_void,
        textsize,
    );
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_zTXt(
    mut out: * mut crate::src::lodepng::ucvector,
    mut keyword: * const std::os::raw::c_char,
    mut textstring: * const std::os::raw::c_char,
    mut zlibsettings: * mut crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut compressed = 0 as *mut std::os::raw::c_uchar;
    let mut compressedsize = 0 as std::os::raw::c_int as size_t;
    let mut textsize = lodepng_strlen(textstring);
    let mut keysize = lodepng_strlen(keyword);
    if keysize < 1 as std::os::raw::c_int as std::os::raw::c_ulong
        || keysize > 79 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 89 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    error = zlib_compress(
        &mut compressed,
        &mut compressedsize,
        textstring as *const std::os::raw::c_uchar,
        textsize,
        zlibsettings,
    );
    if error == 0 {
        let mut size = keysize
            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(compressedsize);
        error = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            size as std::os::raw::c_uint,
            b"zTXt\0" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if error == 0 {
        lodepng_memcpy(
            chunk.offset(8 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
            keyword as *const std::os::raw::c_void,
            keysize,
        );
        *chunk
            .offset(
                (8 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(keysize) as isize,
            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        *chunk
            .offset(
                (9 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(keysize) as isize,
            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        lodepng_memcpy(
            chunk.offset(10 as std::os::raw::c_int as isize).offset(keysize as isize)
                as *mut std::os::raw::c_void,
            compressed as *const std::os::raw::c_void,
            compressedsize,
        );
        lodepng_chunk_generate_crc(chunk);
    }
    lodepng_free(compressed as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn addChunk_iTXt(
    mut out: * mut crate::src::lodepng::ucvector,
    mut compress: std::os::raw::c_uint,
    mut keyword: * const std::os::raw::c_char,
    mut langtag: * const std::os::raw::c_char,
    mut transkey: * const std::os::raw::c_char,
    mut textstring: * const std::os::raw::c_char,
    mut zlibsettings: * mut crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut compressed = 0 as *mut std::os::raw::c_uchar;
    let mut compressedsize = 0 as std::os::raw::c_int as size_t;
    let mut textsize = lodepng_strlen(textstring);
    let mut keysize = lodepng_strlen(keyword);
    let mut langsize = lodepng_strlen(langtag);
    let mut transsize = lodepng_strlen(transkey);
    if keysize < 1 as std::os::raw::c_int as std::os::raw::c_ulong
        || keysize > 79 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 89 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if compress != 0 {
        error = zlib_compress(
            &mut compressed,
            &mut compressedsize,
            textstring as *const std::os::raw::c_uchar,
            textsize,
            zlibsettings,
        );
    }
    if error == 0 {
        let mut size = keysize
            .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(langsize)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(transsize)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add((if compress != 0 { compressedsize } else { textsize }));
        error = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            size as std::os::raw::c_uint,
            b"iTXt\0" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if error == 0 {
        let mut pos = 8 as std::os::raw::c_int as size_t;
        lodepng_memcpy(
            chunk.offset(pos as isize) as *mut std::os::raw::c_void,
            keyword as *const std::os::raw::c_void,
            keysize,
        );
        pos = (pos as std::os::raw::c_ulong).wrapping_add(keysize) as size_t as size_t;
        let mut fresh185 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh185 as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        let mut fresh186 = pos;
        pos = pos.wrapping_add(1);
        *chunk
            .offset(
                fresh186 as isize,
            ) = (if compress != 0 { 1 as std::os::raw::c_int } else { 0 as std::os::raw::c_int })
            as std::os::raw::c_uchar;
        let mut fresh187 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh187 as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        lodepng_memcpy(
            chunk.offset(pos as isize) as *mut std::os::raw::c_void,
            langtag as *const std::os::raw::c_void,
            langsize,
        );
        pos = (pos as std::os::raw::c_ulong).wrapping_add(langsize) as size_t as size_t;
        let mut fresh188 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh188 as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        lodepng_memcpy(
            chunk.offset(pos as isize) as *mut std::os::raw::c_void,
            transkey as *const std::os::raw::c_void,
            transsize,
        );
        pos = (pos as std::os::raw::c_ulong).wrapping_add(transsize) as size_t as size_t;
        let mut fresh189 = pos;
        pos = pos.wrapping_add(1);
        *chunk.offset(fresh189 as isize) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        if compress != 0 {
            lodepng_memcpy(
                chunk.offset(pos as isize) as *mut std::os::raw::c_void,
                compressed as *const std::os::raw::c_void,
                compressedsize,
            );
        } else {
            lodepng_memcpy(
                chunk.offset(pos as isize) as *mut std::os::raw::c_void,
                textstring as *const std::os::raw::c_void,
                textsize,
            );
        }
        lodepng_chunk_generate_crc(chunk);
    }
    lodepng_free(compressed as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn addChunk_bKGD<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
        || (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
            == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut error = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            2 as std::os::raw::c_int as std::os::raw::c_uint,
            b"bKGD\0" as *const u8 as *const std::os::raw::c_char,
        );
        if error != 0 {
            return error;
        }
        *chunk
            .offset(
                8 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_r >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *chunk
            .offset(
                9 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_r & 255 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_uchar;
    } else if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
        == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
        || (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
            == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut error_0 = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            6 as std::os::raw::c_int as std::os::raw::c_uint,
            b"bKGD\0" as *const u8 as *const std::os::raw::c_char,
        );
        if error_0 != 0 {
            return error_0;
        }
        *chunk
            .offset(
                8 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_r >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *chunk
            .offset(
                9 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_r & 255 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_uchar;
        *chunk
            .offset(
                10 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_g >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *chunk
            .offset(
                11 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_g & 255 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_uchar;
        *chunk
            .offset(
                12 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_b >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        *chunk
            .offset(
                13 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_b & 255 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_uchar;
    } else if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut error_1 = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            1 as std::os::raw::c_int as std::os::raw::c_uint,
            b"bKGD\0" as *const u8 as *const std::os::raw::c_char,
        );
        if error_1 != 0 {
            return error_1;
        }
        *chunk
            .offset(
                8 as std::os::raw::c_int as isize,
            ) = ((*((info).clone()).unwrap()).background_r & 255 as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_uchar;
    }
    if !chunk.is_null() {
        lodepng_chunk_generate_crc(chunk);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_tIME<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut time: Option<&'a1 crate::src::lodepng::LodePNGTime>,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut error = lodepng_chunk_init(
        Some(&mut chunk),
        out,
        7 as std::os::raw::c_int as std::os::raw::c_uint,
        b"tIME\0" as *const u8 as *const std::os::raw::c_char,
    );
    if error != 0 {
        return error;
    }
    *chunk
        .offset(
            8 as std::os::raw::c_int as isize,
        ) = ((*((time).clone()).unwrap()).year >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
    *chunk
        .offset(
            9 as std::os::raw::c_int as isize,
        ) = ((*((time).clone()).unwrap()).year & 255 as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
    *chunk.offset(10 as std::os::raw::c_int as isize) = (*(time).unwrap()).month as std::os::raw::c_uchar;
    *chunk.offset(11 as std::os::raw::c_int as isize) = (*(time).unwrap()).day as std::os::raw::c_uchar;
    *chunk.offset(12 as std::os::raw::c_int as isize) = (*(time).unwrap()).hour as std::os::raw::c_uchar;
    *chunk.offset(13 as std::os::raw::c_int as isize) = (*(time).unwrap()).minute as std::os::raw::c_uchar;
    *chunk.offset(14 as std::os::raw::c_int as isize) = (*(time).unwrap()).second as std::os::raw::c_uchar;
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_pHYs<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut error = lodepng_chunk_init(
        Some(&mut chunk),
        out,
        9 as std::os::raw::c_int as std::os::raw::c_uint,
        b"pHYs\0" as *const u8 as *const std::os::raw::c_char,
    );
    if error != 0 {
        return error;
    }
    lodepng_set32bitInt(chunk.offset(8 as std::os::raw::c_int as isize), (*(info).unwrap()).phys_x);
    lodepng_set32bitInt(chunk.offset(12 as std::os::raw::c_int as isize), (*(info).unwrap()).phys_y);
    *chunk.offset(16 as std::os::raw::c_int as isize) = (*(info).unwrap()).phys_unit as std::os::raw::c_uchar;
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_gAMA<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut error = lodepng_chunk_init(
        Some(&mut chunk),
        out,
        4 as std::os::raw::c_int as std::os::raw::c_uint,
        b"gAMA\0" as *const u8 as *const std::os::raw::c_char,
    );
    if error != 0 {
        return error;
    }
    lodepng_set32bitInt(chunk.offset(8 as std::os::raw::c_int as isize), (*(info).unwrap()).gama_gamma);
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_cHRM<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
) -> std::os::raw::c_uint {
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut error = lodepng_chunk_init(
        Some(&mut chunk),
        out,
        32 as std::os::raw::c_int as std::os::raw::c_uint,
        b"cHRM\0" as *const u8 as *const std::os::raw::c_char,
    );
    if error != 0 {
        return error;
    }
    lodepng_set32bitInt(chunk.offset(8 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_white_x);
    lodepng_set32bitInt(chunk.offset(12 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_white_y);
    lodepng_set32bitInt(chunk.offset(16 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_red_x);
    lodepng_set32bitInt(chunk.offset(20 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_red_y);
    lodepng_set32bitInt(chunk.offset(24 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_green_x);
    lodepng_set32bitInt(chunk.offset(28 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_green_y);
    lodepng_set32bitInt(chunk.offset(32 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_blue_x);
    lodepng_set32bitInt(chunk.offset(36 as std::os::raw::c_int as isize), (*(info).unwrap()).chrm_blue_y);
    lodepng_chunk_generate_crc(chunk);
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn addChunk_sRGB<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
) -> std::os::raw::c_uint {
    let mut data = (*(info).unwrap()).srgb_intent as std::os::raw::c_uchar;
    return lodepng_chunk_createv(
        out,
        1 as std::os::raw::c_int as std::os::raw::c_uint,
        b"sRGB\0" as *const u8 as *const std::os::raw::c_char,
        &mut data,
    );
}
unsafe extern "C" fn addChunk_iCCP<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
    mut zlibsettings: * mut crate::src::lodepng::LodePNGCompressSettings,
) -> std::os::raw::c_uint {
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    let mut compressed = 0 as *mut std::os::raw::c_uchar;
    let mut compressedsize = 0 as std::os::raw::c_int as size_t;
    let mut keysize = lodepng_strlen((*((info).clone()).unwrap()).iccp_name);
    if keysize < 1 as std::os::raw::c_int as std::os::raw::c_ulong
        || keysize > 79 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        return 89 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    error = zlib_compress(
        &mut compressed,
        &mut compressedsize,
        (*((info).clone()).unwrap()).iccp_profile,
        (*(info).unwrap()).iccp_profile_size as size_t,
        zlibsettings,
    );
    if error == 0 {
        let mut size = keysize
            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_add(compressedsize);
        error = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            size as std::os::raw::c_uint,
            b"iCCP\0" as *const u8 as *const std::os::raw::c_char,
        );
    }
    if error == 0 {
        lodepng_memcpy(
            chunk.offset(8 as std::os::raw::c_int as isize) as *mut std::os::raw::c_void,
            (*((info).clone()).unwrap()).iccp_name as *const std::os::raw::c_void,
            keysize,
        );
        *chunk
            .offset(
                (8 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(keysize) as isize,
            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        *chunk
            .offset(
                (9 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(keysize) as isize,
            ) = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        lodepng_memcpy(
            chunk.offset(10 as std::os::raw::c_int as isize).offset(keysize as isize)
                as *mut std::os::raw::c_void,
            compressed as *const std::os::raw::c_void,
            compressedsize,
        );
        lodepng_chunk_generate_crc(chunk);
    }
    lodepng_free(compressed as *mut std::os::raw::c_void);
    return error;
}
unsafe extern "C" fn addChunk_sBIT<'a1>(
    mut out: * mut crate::src::lodepng::ucvector,
    mut info: Option<&'a1 crate::src::lodepng::LodePNGInfo>,
) -> std::os::raw::c_uint {
    let mut bitdepth = if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        8 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        (*(info).unwrap()).color.bitdepth
    };
    let mut chunk = 0 as *mut std::os::raw::c_uchar;
    if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((info).clone()).unwrap()).sbit_r == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_r > bitdepth
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        let mut error = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            1 as std::os::raw::c_int as std::os::raw::c_uint,
            b"sBIT\0" as *const u8 as *const std::os::raw::c_char,
        );
        if error != 0 {
            return error;
        }
        *chunk.offset(8 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_r as std::os::raw::c_uchar;
    } else if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
        == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
        || (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
            == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((info).clone()).unwrap()).sbit_r == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_g == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_b == 0 as std::os::raw::c_int as std::os::raw::c_uint
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if (*((info).clone()).unwrap()).sbit_r > bitdepth || (*((info).clone()).unwrap()).sbit_g > bitdepth
            || (*((info).clone()).unwrap()).sbit_b > bitdepth
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        let mut error_0 = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            3 as std::os::raw::c_int as std::os::raw::c_uint,
            b"sBIT\0" as *const u8 as *const std::os::raw::c_char,
        );
        if error_0 != 0 {
            return error_0;
        }
        *chunk.offset(8 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_r as std::os::raw::c_uchar;
        *chunk.offset(9 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_g as std::os::raw::c_uchar;
        *chunk.offset(10 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_b as std::os::raw::c_uchar;
    } else if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
        == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((info).clone()).unwrap()).sbit_r == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_a == 0 as std::os::raw::c_int as std::os::raw::c_uint
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if (*((info).clone()).unwrap()).sbit_r > bitdepth || (*((info).clone()).unwrap()).sbit_a > bitdepth {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        let mut error_1 = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            2 as std::os::raw::c_int as std::os::raw::c_uint,
            b"sBIT\0" as *const u8 as *const std::os::raw::c_char,
        );
        if error_1 != 0 {
            return error_1;
        }
        *chunk.offset(8 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_r as std::os::raw::c_uchar;
        *chunk.offset(9 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_a as std::os::raw::c_uchar;
    } else if (*((info).clone()).unwrap()).color.colortype as std::os::raw::c_uint
        == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
    {
        if (*((info).clone()).unwrap()).sbit_r == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_g == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_b == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_a == 0 as std::os::raw::c_int as std::os::raw::c_uint
            || (*((info).clone()).unwrap()).sbit_r > bitdepth || (*((info).clone()).unwrap()).sbit_g > bitdepth
            || (*((info).clone()).unwrap()).sbit_b > bitdepth || (*((info).clone()).unwrap()).sbit_a > bitdepth
        {
            return 115 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        let mut error_2 = lodepng_chunk_init(
            Some(&mut chunk),
            out,
            4 as std::os::raw::c_int as std::os::raw::c_uint,
            b"sBIT\0" as *const u8 as *const std::os::raw::c_char,
        );
        if error_2 != 0 {
            return error_2;
        }
        *chunk.offset(8 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_r as std::os::raw::c_uchar;
        *chunk.offset(9 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_g as std::os::raw::c_uchar;
        *chunk.offset(10 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_b as std::os::raw::c_uchar;
        *chunk.offset(11 as std::os::raw::c_int as isize) = (*(info).unwrap()).sbit_a as std::os::raw::c_uchar;
    }
    if !chunk.is_null() {
        lodepng_chunk_generate_crc(chunk);
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn filterScanline(
    mut out: * mut std::os::raw::c_uchar,
    mut scanline: * const std::os::raw::c_uchar,
    mut prevline: * const std::os::raw::c_uchar,
    mut length: std::os::raw::c_ulong,
    mut bytewidth: std::os::raw::c_ulong,
    mut filterType: std::os::raw::c_uchar,
) {
    let mut i: u64 = 0;
    match filterType as std::os::raw::c_int {
        0 => {
            i = 0 as std::os::raw::c_int as size_t;
            while i != length {
                *out.offset(i as isize) = *scanline.offset(i as isize);
                i = i.wrapping_add(1);
            }
        }
        1 => {
            i = 0 as std::os::raw::c_int as size_t;
            while i != bytewidth {
                *out.offset(i as isize) = *scanline.offset(i as isize);
                i = i.wrapping_add(1);
            }
            i = bytewidth;
            while i < length {
                *out
                    .offset(
                        i as isize,
                    ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                    - *scanline.offset(i.wrapping_sub(bytewidth) as isize)
                        as std::os::raw::c_int) as std::os::raw::c_uchar;
                i = i.wrapping_add(1);
            }
        }
        2 => {
            if !prevline.is_null() {
                i = 0 as std::os::raw::c_int as size_t;
                while i != length {
                    *out
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        - *prevline.offset(i as isize) as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0 as std::os::raw::c_int as size_t;
                while i != length {
                    *out.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
            }
        }
        3 => {
            if !prevline.is_null() {
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *out
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        - (*prevline.offset(i as isize) as std::os::raw::c_int
                            >> 1 as std::os::raw::c_int)) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        - (*scanline.offset(i.wrapping_sub(bytewidth) as isize)
                            as std::os::raw::c_int + *prevline.offset(i as isize) as std::os::raw::c_int
                            >> 1 as std::os::raw::c_int)) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *out.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        - (*scanline.offset(i.wrapping_sub(bytewidth) as isize)
                            as std::os::raw::c_int >> 1 as std::os::raw::c_int)) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
            }
        }
        4 => {
            if !prevline.is_null() {
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *out
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        - *prevline.offset(i as isize) as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        - paethPredictor(
                            *scanline.offset(i.wrapping_sub(bytewidth) as isize)
                                as std::os::raw::c_short,
                            *prevline.offset(i as isize) as std::os::raw::c_short,
                            *prevline.offset(i.wrapping_sub(bytewidth) as isize)
                                as std::os::raw::c_short,
                        ) as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0 as std::os::raw::c_int as size_t;
                while i != bytewidth {
                    *out.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out
                        .offset(
                            i as isize,
                        ) = (*scanline.offset(i as isize) as std::os::raw::c_int
                        - *scanline.offset(i.wrapping_sub(bytewidth) as isize)
                            as std::os::raw::c_int) as std::os::raw::c_uchar;
                    i = i.wrapping_add(1);
                }
            }
        }
        _ => return,
    };
}
 extern "C" fn ilog2(mut i: std::os::raw::c_ulong) -> std::os::raw::c_ulong {
    let mut result = 0 as std::os::raw::c_int as size_t;
    if i >= 65536 as std::os::raw::c_int as std::os::raw::c_ulong {
        result = (result as std::os::raw::c_ulong)
            .wrapping_add(16 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        i >>= 16 as std::os::raw::c_int;
    }
    if i >= 256 as std::os::raw::c_int as std::os::raw::c_ulong {
        result = (result as std::os::raw::c_ulong)
            .wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        i >>= 8 as std::os::raw::c_int;
    }
    if i >= 16 as std::os::raw::c_int as std::os::raw::c_ulong {
        result = (result as std::os::raw::c_ulong)
            .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        i >>= 4 as std::os::raw::c_int;
    }
    if i >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        result = (result as std::os::raw::c_ulong)
            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        i >>= 2 as std::os::raw::c_int;
    }
    if i >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        result = (result as std::os::raw::c_ulong)
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
    }
    return result;
}
 extern "C" fn ilog2i(mut i: std::os::raw::c_ulong) -> std::os::raw::c_ulong {
    let mut l: u64 = 0;
    if i == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int as size_t;
    }
    l = ilog2(i);
    return i
        .wrapping_mul(l)
        .wrapping_add(
            i.wrapping_sub(((1 as std::os::raw::c_uint) << l) as std::os::raw::c_ulong)
                << 1 as std::os::raw::c_uint,
        );
}
unsafe extern "C" fn filter<'a1, 'a2>(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut color: Option<&'a1 crate::src::lodepng::LodePNGColorMode>,
    mut settings: Option<&'a2 crate::src::lodepng::LodePNGEncoderSettings>,
) -> std::os::raw::c_uint {
    let mut bpp = lodepng_get_bpp((color).clone());
    let mut linebytes = (lodepng_get_raw_size_idat(
        w,
        1 as std::os::raw::c_int as std::os::raw::c_uint,
        bpp,
    ))
        .wrapping_sub(1 as std::os::raw::c_uint as std::os::raw::c_ulong);
    let mut bytewidth = bpp
        .wrapping_add(7 as std::os::raw::c_uint)
        .wrapping_div(8 as std::os::raw::c_uint) as size_t;
    let mut prevline = 0 as *const std::os::raw::c_uchar;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut strategy = (*(settings).unwrap()).filter_strategy;
    if (*((settings).clone()).unwrap()).filter_palette_zero != 0
        && ((*((color).clone()).unwrap()).colortype as std::os::raw::c_uint
            == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
            || (*((color).clone()).unwrap()).bitdepth < 8 as std::os::raw::c_int as std::os::raw::c_uint)
    {
        strategy = LFS_ZERO;
    }
    if bpp == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return 31 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    if strategy as std::os::raw::c_uint >= LFS_ZERO as std::os::raw::c_int as std::os::raw::c_uint
        && strategy as std::os::raw::c_uint <= LFS_FOUR as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut type_0 = strategy as std::os::raw::c_uchar;
        y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while y != h {
            let mut outindex = (1 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_add(linebytes)
                .wrapping_mul(y as std::os::raw::c_ulong);
            let mut inindex = linebytes.wrapping_mul(y as std::os::raw::c_ulong);
            *out.offset(outindex as isize) = type_0;
            filterScanline(
                &mut *out
                    .offset(
                        outindex.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ),
                &*in_0.offset(inindex as isize),
                prevline,
                linebytes,
                bytewidth,
                type_0,
            );
            prevline = &*in_0.offset(inindex as isize) as *const std::os::raw::c_uchar;
            y = y.wrapping_add(1);
        }
    } else if strategy as std::os::raw::c_uint == LFS_MINSUM as std::os::raw::c_int as std::os::raw::c_uint {
        let mut attempt: [* mut u8; 5] = [0 as *mut std::os::raw::c_uchar; 5];
        let mut smallest = 0 as std::os::raw::c_int as size_t;
        let mut type_1: u8 = 0;
        let mut bestType = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        type_1 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        while type_1 as std::os::raw::c_int != 5 as std::os::raw::c_int {
            attempt[type_1 as usize] = lodepng_malloc(linebytes) as *mut std::os::raw::c_uchar;
            if (attempt[type_1 as usize]).is_null() {
                error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
            }
            type_1 = type_1.wrapping_add(1);
        }
        if error == 0 {
            y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while y != h {
                type_1 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
                while type_1 as std::os::raw::c_int != 5 as std::os::raw::c_int {
                    let mut sum = 0 as std::os::raw::c_int as size_t;
                    filterScanline(
                        attempt[type_1 as usize],
                        &*in_0
                            .offset(
                                (y as std::os::raw::c_ulong).wrapping_mul(linebytes) as isize,
                            ),
                        prevline,
                        linebytes,
                        bytewidth,
                        type_1,
                    );
                    if type_1 as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        while x as std::os::raw::c_ulong != linebytes {
                            sum = (sum as std::os::raw::c_ulong)
                                .wrapping_add(
                                    *(attempt[type_1 as usize]).offset(x as isize)
                                        as std::os::raw::c_ulong,
                                ) as size_t as size_t;
                            x = x.wrapping_add(1);
                        }
                    } else {
                        x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        while x as std::os::raw::c_ulong != linebytes {
                            let mut s = *(attempt[type_1 as usize]).offset(x as isize);
                            sum = (sum as std::os::raw::c_ulong)
                                .wrapping_add(
                                    (if (s as std::os::raw::c_int) < 128 as std::os::raw::c_int {
                                        s as std::os::raw::c_uint
                                    } else {
                                        (255 as std::os::raw::c_uint).wrapping_sub(s as std::os::raw::c_uint)
                                    }) as std::os::raw::c_ulong,
                                ) as size_t as size_t;
                            x = x.wrapping_add(1);
                        }
                    }
                    if type_1 as std::os::raw::c_int == 0 as std::os::raw::c_int || sum < smallest {
                        bestType = type_1;
                        smallest = sum;
                    }
                    type_1 = type_1.wrapping_add(1);
                }
                prevline = &*in_0
                    .offset((y as std::os::raw::c_ulong).wrapping_mul(linebytes) as isize)
                    as *const std::os::raw::c_uchar;
                *out
                    .offset(
                        (y as std::os::raw::c_ulong)
                            .wrapping_mul(
                                linebytes.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                            ) as isize,
                    ) = bestType;
                x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while x as std::os::raw::c_ulong != linebytes {
                    *out
                        .offset(
                            (y as std::os::raw::c_ulong)
                                .wrapping_mul(
                                    linebytes.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                                )
                                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(x as std::os::raw::c_ulong) as isize,
                        ) = *(attempt[bestType as usize]).offset(x as isize);
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
        }
        type_1 = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
        while type_1 as std::os::raw::c_int != 5 as std::os::raw::c_int {
            lodepng_free(attempt[type_1 as usize] as *mut std::os::raw::c_void);
            type_1 = type_1.wrapping_add(1);
        }
    } else if strategy as std::os::raw::c_uint == LFS_ENTROPY as std::os::raw::c_int as std::os::raw::c_uint {
        let mut attempt_0: [* mut u8; 5] = [0 as *mut std::os::raw::c_uchar; 5];
        let mut bestSum = 0 as std::os::raw::c_int as size_t;
        let mut type_2: u32 = 0;
        let mut bestType_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        let mut count: [u32; 256] = [0; 256];
        type_2 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while type_2 != 5 as std::os::raw::c_int as std::os::raw::c_uint {
            attempt_0[type_2 as usize] = lodepng_malloc(linebytes) as *mut std::os::raw::c_uchar;
            if (attempt_0[type_2 as usize]).is_null() {
                error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
            }
            type_2 = type_2.wrapping_add(1);
        }
        if error == 0 {
            y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while y != h {
                type_2 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while type_2 != 5 as std::os::raw::c_int as std::os::raw::c_uint {
                    let mut sum_0 = 0 as std::os::raw::c_int as size_t;
                    filterScanline(
                        attempt_0[type_2 as usize],
                        &*in_0
                            .offset(
                                (y as std::os::raw::c_ulong).wrapping_mul(linebytes) as isize,
                            ),
                        prevline,
                        linebytes,
                        bytewidth,
                        type_2 as std::os::raw::c_uchar,
                    );
                    lodepng_memset(
                        count.as_mut_ptr() as *mut std::os::raw::c_void,
                        0 as std::os::raw::c_int,
                        (256 as std::os::raw::c_int as std::os::raw::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<std::os::raw::c_uint>() as std::os::raw::c_ulong,
                            ),
                    );
                    x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    while x as std::os::raw::c_ulong != linebytes {
                        count[*(attempt_0[type_2 as usize]).offset(x as isize)
                            as usize] = (count[*(attempt_0[type_2 as usize])
                            .offset(x as isize) as usize])
                            .wrapping_add(1);
                        x = x.wrapping_add(1);
                    }
                    count[type_2 as usize] = (count[type_2 as usize]).wrapping_add(1);
                    x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    while x != 256 as std::os::raw::c_int as std::os::raw::c_uint {
                        sum_0 = (sum_0 as std::os::raw::c_ulong)
                            .wrapping_add(ilog2i(count[x as usize] as size_t)) as size_t
                            as size_t;
                        x = x.wrapping_add(1);
                    }
                    if type_2 == 0 as std::os::raw::c_int as std::os::raw::c_uint || sum_0 > bestSum {
                        bestType_0 = type_2;
                        bestSum = sum_0;
                    }
                    type_2 = type_2.wrapping_add(1);
                }
                prevline = &*in_0
                    .offset((y as std::os::raw::c_ulong).wrapping_mul(linebytes) as isize)
                    as *const std::os::raw::c_uchar;
                *out
                    .offset(
                        (y as std::os::raw::c_ulong)
                            .wrapping_mul(
                                linebytes.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                            ) as isize,
                    ) = bestType_0 as std::os::raw::c_uchar;
                x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while x as std::os::raw::c_ulong != linebytes {
                    *out
                        .offset(
                            (y as std::os::raw::c_ulong)
                                .wrapping_mul(
                                    linebytes.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                                )
                                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(x as std::os::raw::c_ulong) as isize,
                        ) = *(attempt_0[bestType_0 as usize]).offset(x as isize);
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
        }
        type_2 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while type_2 != 5 as std::os::raw::c_int as std::os::raw::c_uint {
            lodepng_free(attempt_0[type_2 as usize] as *mut std::os::raw::c_void);
            type_2 = type_2.wrapping_add(1);
        }
    } else if strategy as std::os::raw::c_uint == LFS_PREDEFINED as std::os::raw::c_int as std::os::raw::c_uint {
        y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while y != h {
            let mut outindex_0 = (1 as std::os::raw::c_int as std::os::raw::c_ulong)
                .wrapping_add(linebytes)
                .wrapping_mul(y as std::os::raw::c_ulong);
            let mut inindex_0 = linebytes.wrapping_mul(y as std::os::raw::c_ulong);
            let mut type_3 = *((*((settings).clone()).unwrap()).predefined_filters).offset(y as isize);
            *out.offset(outindex_0 as isize) = type_3;
            filterScanline(
                &mut *out
                    .offset(
                        outindex_0.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as isize,
                    ),
                &*in_0.offset(inindex_0 as isize),
                prevline,
                linebytes,
                bytewidth,
                type_3,
            );
            prevline = &*in_0.offset(inindex_0 as isize) as *const std::os::raw::c_uchar;
            y = y.wrapping_add(1);
        }
    } else if strategy as std::os::raw::c_uint == LFS_BRUTE_FORCE as std::os::raw::c_int as std::os::raw::c_uint
    {
        let mut size: [u64; 5] = [0; 5];
        let mut attempt_1: [* mut u8; 5] = [0 as *mut std::os::raw::c_uchar; 5];
        let mut smallest_0 = 0 as std::os::raw::c_int as size_t;
        let mut type_4 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        let mut bestType_1 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        let mut dummy = 0 as *mut std::os::raw::c_uchar;
        let mut zlibsettings = LodePNGCompressSettings {
            btype: 0,
            use_lz77: 0,
            windowsize: 0,
            minmatch: 0,
            nicematch: 0,
            lazymatching: 0,
            custom_zlib: None,
            custom_deflate: None,
            custom_context: 0 as *const std::os::raw::c_void,
        };
        lodepng_memcpy(
            &mut zlibsettings as *mut LodePNGCompressSettings as *mut std::os::raw::c_void,
            &(*((settings).clone()).unwrap()).zlibsettings as *const LodePNGCompressSettings
                as *const std::os::raw::c_void,
            ::std::mem::size_of::<LodePNGCompressSettings>() as std::os::raw::c_ulong,
        );
        zlibsettings.btype = 1 as std::os::raw::c_int as std::os::raw::c_uint;
        zlibsettings.custom_zlib = None;
        zlibsettings.custom_deflate = None;
        type_4 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while type_4 != 5 as std::os::raw::c_int as std::os::raw::c_uint {
            attempt_1[type_4 as usize] = lodepng_malloc(linebytes) as *mut std::os::raw::c_uchar;
            if (attempt_1[type_4 as usize]).is_null() {
                error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
            }
            type_4 = type_4.wrapping_add(1);
        }
        if error == 0 {
            y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while y != h {
                type_4 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while type_4 != 5 as std::os::raw::c_int as std::os::raw::c_uint {
                    let mut testsize = linebytes as std::os::raw::c_uint;
                    filterScanline(
                        attempt_1[type_4 as usize],
                        &*in_0
                            .offset(
                                (y as std::os::raw::c_ulong).wrapping_mul(linebytes) as isize,
                            ),
                        prevline,
                        linebytes,
                        bytewidth,
                        type_4 as std::os::raw::c_uchar,
                    );
                    size[type_4 as usize] = 0 as std::os::raw::c_int as size_t;
                    dummy = 0 as *mut std::os::raw::c_uchar;
                    zlib_compress(
                        &mut dummy,
                        &mut *size.as_mut_ptr().offset(type_4 as isize),
                        attempt_1[type_4 as usize],
                        testsize as size_t,
                        &mut zlibsettings,
                    );
                    lodepng_free(dummy as *mut std::os::raw::c_void);
                    if type_4 == 0 as std::os::raw::c_int as std::os::raw::c_uint
                        || size[type_4 as usize] < smallest_0
                    {
                        bestType_1 = type_4;
                        smallest_0 = size[type_4 as usize];
                    }
                    type_4 = type_4.wrapping_add(1);
                }
                prevline = &*in_0
                    .offset((y as std::os::raw::c_ulong).wrapping_mul(linebytes) as isize)
                    as *const std::os::raw::c_uchar;
                *out
                    .offset(
                        (y as std::os::raw::c_ulong)
                            .wrapping_mul(
                                linebytes.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                            ) as isize,
                    ) = bestType_1 as std::os::raw::c_uchar;
                x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while x as std::os::raw::c_ulong != linebytes {
                    *out
                        .offset(
                            (y as std::os::raw::c_ulong)
                                .wrapping_mul(
                                    linebytes.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                                )
                                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                .wrapping_add(x as std::os::raw::c_ulong) as isize,
                        ) = *(attempt_1[bestType_1 as usize]).offset(x as isize);
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
        }
        type_4 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while type_4 != 5 as std::os::raw::c_int as std::os::raw::c_uint {
            lodepng_free(attempt_1[type_4 as usize] as *mut std::os::raw::c_void);
            type_4 = type_4.wrapping_add(1);
        }
    } else {
        return 88 as std::os::raw::c_int as std::os::raw::c_uint
    }
    return error;
}
unsafe extern "C" fn addPaddingBits(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * const std::os::raw::c_uchar,
    mut olinebits: std::os::raw::c_ulong,
    mut ilinebits: std::os::raw::c_ulong,
    mut h: std::os::raw::c_uint,
) {
    let mut y: u32 = 0;
    let mut diff = olinebits.wrapping_sub(ilinebits);
    let mut obp = 0 as std::os::raw::c_int as size_t;
    let mut ibp = 0 as std::os::raw::c_int as size_t;
    y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while y != h {
        let mut x: u64 = 0;
        x = 0 as std::os::raw::c_int as size_t;
        while x < ilinebits {
            let mut bit = readBitFromReversedStream(Some(&mut ibp), in_0);
            setBitOfReversedStream(Some(&mut obp), out, bit);
            x = x.wrapping_add(1);
        }
        x = 0 as std::os::raw::c_int as size_t;
        while x != diff {
            setBitOfReversedStream(Some(&mut obp), out, 0 as std::os::raw::c_int as std::os::raw::c_uchar);
            x = x.wrapping_add(1);
        }
        y = y.wrapping_add(1);
    }
}
unsafe extern "C" fn Adam7_interlace(
    mut out: * mut std::os::raw::c_uchar,
    mut in_0: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut bpp: std::os::raw::c_uint,
) {
    let mut passw: [u32; 7] = [0; 7];
    let mut passh: [u32; 7] = [0; 7];
    let mut filter_passstart: [u64; 8] = [0; 8];
    let mut padded_passstart: [u64; 8] = [0; 8];
    let mut passstart: [u64; 8] = [0; 8];
    let mut i: u32 = 0;
    Adam7_getpassvalues(
        passw.as_mut_ptr(),
        passh.as_mut_ptr(),
        filter_passstart.as_mut_ptr(),
        padded_passstart.as_mut_ptr(),
        passstart.as_mut_ptr(),
        w,
        h,
        bpp,
    );
    if bpp >= 8 as std::os::raw::c_int as std::os::raw::c_uint {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut x: u32 = 0;
            let mut y: u32 = 0;
            let mut b: u32 = 0;
            let mut bytewidth = bpp.wrapping_div(8 as std::os::raw::c_uint) as size_t;
            y = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while y < passh[i as usize] {
                x = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while x < passw[i as usize] {
                    let mut pixelinstart = ((ADAM7_IY[i as usize])
                        .wrapping_add(y.wrapping_mul(ADAM7_DY[i as usize]))
                        .wrapping_mul(w)
                        .wrapping_add(ADAM7_IX[i as usize])
                        .wrapping_add(x.wrapping_mul(ADAM7_DX[i as usize]))
                        as std::os::raw::c_ulong)
                        .wrapping_mul(bytewidth);
                    let mut pixeloutstart = (passstart[i as usize])
                        .wrapping_add(
                            (y.wrapping_mul(passw[i as usize]).wrapping_add(x)
                                as std::os::raw::c_ulong)
                                .wrapping_mul(bytewidth),
                        );
                    b = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    while (b as std::os::raw::c_ulong) < bytewidth {
                        *out
                            .offset(
                                pixeloutstart.wrapping_add(b as std::os::raw::c_ulong) as isize,
                            ) = *in_0
                            .offset(
                                pixelinstart.wrapping_add(b as std::os::raw::c_ulong) as isize,
                            );
                        b = b.wrapping_add(1);
                    }
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
            let mut x_0: u32 = 0;
            let mut y_0: u32 = 0;
            let mut b_0: u32 = 0;
            let mut ilinebits = bpp.wrapping_mul(passw[i as usize]);
            let mut olinebits = bpp.wrapping_mul(w);
            let mut obp: u64 = 0;
            let mut ibp: u64 = 0;
            y_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while y_0 < passh[i as usize] {
                x_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while x_0 < passw[i as usize] {
                    ibp = (ADAM7_IY[i as usize])
                        .wrapping_add(y_0.wrapping_mul(ADAM7_DY[i as usize]))
                        .wrapping_mul(olinebits)
                        .wrapping_add(
                            (ADAM7_IX[i as usize])
                                .wrapping_add(x_0.wrapping_mul(ADAM7_DX[i as usize]))
                                .wrapping_mul(bpp),
                        ) as size_t;
                    obp = (8 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(passstart[i as usize])
                        .wrapping_add(
                            y_0
                                .wrapping_mul(ilinebits)
                                .wrapping_add(x_0.wrapping_mul(bpp)) as std::os::raw::c_ulong,
                        );
                    b_0 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    while b_0 < bpp {
                        let mut bit = readBitFromReversedStream(Some(&mut ibp), in_0);
                        setBitOfReversedStream(Some(&mut obp), out, bit);
                        b_0 = b_0.wrapping_add(1);
                    }
                    x_0 = x_0.wrapping_add(1);
                }
                y_0 = y_0.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn preProcessScanlines<'a1, 'a2, 'a3, 'a4>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut in_0: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut info_png: Option<&'a3 crate::src::lodepng::LodePNGInfo>,
    mut settings: Option<&'a4 crate::src::lodepng::LodePNGEncoderSettings>,
) -> std::os::raw::c_uint {
    let mut bpp = lodepng_get_bpp((Some(&(*((info_png).clone()).unwrap()).color)).clone());
    let mut error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if (*((info_png).clone()).unwrap()).interlace_method == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        *(borrow_mut(&mut outsize)).unwrap() = h
            .wrapping_add(
                h
                    .wrapping_mul(
                        w
                            .wrapping_mul(bpp)
                            .wrapping_add(7 as std::os::raw::c_uint)
                            .wrapping_div(8 as std::os::raw::c_uint),
                    ),
            ) as size_t;
        *(borrow_mut(&mut out)).unwrap() = lodepng_malloc(*(borrow_mut(&mut outsize)).unwrap()) as *mut std::os::raw::c_uchar;
        if (*(borrow(& out)).unwrap()).is_null() && *(borrow(& outsize)).unwrap() != 0 {
            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if error == 0 {
            if bpp < 8 as std::os::raw::c_int as std::os::raw::c_uint
                && w.wrapping_mul(bpp)
                    != w
                        .wrapping_mul(bpp)
                        .wrapping_add(7 as std::os::raw::c_uint)
                        .wrapping_div(8 as std::os::raw::c_uint)
                        .wrapping_mul(8 as std::os::raw::c_uint)
            {
                let mut padded = lodepng_malloc(
                    h
                        .wrapping_mul(
                            w
                                .wrapping_mul(bpp)
                                .wrapping_add(7 as std::os::raw::c_uint)
                                .wrapping_div(8 as std::os::raw::c_uint),
                        ) as size_t,
                ) as *mut std::os::raw::c_uchar;
                if padded.is_null() {
                    error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                }
                if error == 0 {
                    addPaddingBits(
                        padded,
                        in_0,
                        w
                            .wrapping_mul(bpp)
                            .wrapping_add(7 as std::os::raw::c_uint)
                            .wrapping_div(8 as std::os::raw::c_uint)
                            .wrapping_mul(8 as std::os::raw::c_uint) as size_t,
                        w.wrapping_mul(bpp) as size_t,
                        h,
                    );
                    error = filter(*(borrow_mut(&mut out)).unwrap(), padded, w, h, (Some(&(*((info_png).clone()).unwrap()).color)).clone(), (settings).clone());
                }
                lodepng_free(padded as *mut std::os::raw::c_void);
            } else {
                error = filter(*(borrow_mut(&mut out)).unwrap(), in_0, w, h, (Some(&(*((info_png).clone()).unwrap()).color)).clone(), (settings).clone());
            }
        }
    } else {
        let mut passw: [u32; 7] = [0; 7];
        let mut passh: [u32; 7] = [0; 7];
        let mut filter_passstart: [u64; 8] = [0; 8];
        let mut padded_passstart: [u64; 8] = [0; 8];
        let mut passstart: [u64; 8] = [0; 8];
        let mut adam7 = 0 as *mut std::os::raw::c_uchar;
        Adam7_getpassvalues(
            passw.as_mut_ptr(),
            passh.as_mut_ptr(),
            filter_passstart.as_mut_ptr(),
            padded_passstart.as_mut_ptr(),
            passstart.as_mut_ptr(),
            w,
            h,
            bpp,
        );
        *(borrow_mut(&mut outsize)).unwrap() = filter_passstart[7 as std::os::raw::c_int as usize];
        *(borrow_mut(&mut out)).unwrap() = lodepng_malloc(*(borrow_mut(&mut outsize)).unwrap()) as *mut std::os::raw::c_uchar;
        if (*(borrow_mut(&mut out)).unwrap()).is_null() {
            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        adam7 = lodepng_malloc(passstart[7 as std::os::raw::c_int as usize])
            as *mut std::os::raw::c_uchar;
        if adam7.is_null() && passstart[7 as std::os::raw::c_int as usize] != 0 {
            error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
        }
        if error == 0 {
            let mut i: u32 = 0;
            Adam7_interlace(adam7, in_0, w, h, bpp);
            i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            while i != 7 as std::os::raw::c_int as std::os::raw::c_uint {
                if bpp < 8 as std::os::raw::c_int as std::os::raw::c_uint {
                    let mut padded_0 = lodepng_malloc(
                        (padded_passstart[i
                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as usize])
                            .wrapping_sub(padded_passstart[i as usize]),
                    ) as *mut std::os::raw::c_uchar;
                    if padded_0.is_null() {
                        error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                        break;
                    } else {
                        addPaddingBits(
                            padded_0,
                            &mut *adam7
                                .offset(
                                    *passstart.as_mut_ptr().offset(i as isize) as isize,
                                ),
                            (passw[i as usize])
                                .wrapping_mul(bpp)
                                .wrapping_add(7 as std::os::raw::c_uint)
                                .wrapping_div(8 as std::os::raw::c_uint)
                                .wrapping_mul(8 as std::os::raw::c_uint) as size_t,
                            (passw[i as usize]).wrapping_mul(bpp) as size_t,
                            passh[i as usize],
                        );
                        error = filter(
                            &mut *(*(borrow(& out)).unwrap())
                                .offset(
                                    *filter_passstart.as_mut_ptr().offset(i as isize) as isize,
                                ),
                            padded_0,
                            passw[i as usize],
                            passh[i as usize],
                            (Some(&(*((info_png).clone()).unwrap()).color)).clone(),
                            (settings).clone(),
                        );
                        lodepng_free(padded_0 as *mut std::os::raw::c_void);
                    }
                } else {
                    error = filter(
                        &mut *(*(borrow(& out)).unwrap())
                            .offset(
                                *filter_passstart.as_mut_ptr().offset(i as isize) as isize,
                            ),
                        &mut *adam7
                            .offset(
                                *padded_passstart.as_mut_ptr().offset(i as isize) as isize,
                            ),
                        passw[i as usize],
                        passh[i as usize],
                        (Some(&(*((info_png).clone()).unwrap()).color)).clone(),
                        (settings).clone(),
                    );
                }
                if error != 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        lodepng_free(adam7 as *mut std::os::raw::c_void);
    }
    return error;
}
unsafe extern "C" fn addUnknownChunks(
    mut out: * mut crate::src::lodepng::ucvector,
    mut data: * mut std::os::raw::c_uchar,
    mut datasize: std::os::raw::c_ulong,
) -> std::os::raw::c_uint {
    let mut inchunk = data;
    while (inchunk.offset_from(data) as std::os::raw::c_long as size_t) < datasize {
        let mut error = lodepng_chunk_append(
            Some(&mut (*out).data),
            Some(&mut (*out).size),
            inchunk,
        );
        if error != 0 {
            return error;
        }
        (*out).allocsize = (*out).size;
        inchunk = lodepng_chunk_next(inchunk, data.offset(datasize as isize));
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn isGrayICCProfile(
    mut profile: * const std::os::raw::c_uchar,
    mut size: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    if size < 20 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    return (*profile.offset(16 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'G' as i32
        && *profile.offset(17 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'R' as i32
        && *profile.offset(18 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'A' as i32
        && *profile.offset(19 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'Y' as i32)
        as std::os::raw::c_int as std::os::raw::c_uint;
}
unsafe extern "C" fn isRGBICCProfile(
    mut profile: * const std::os::raw::c_uchar,
    mut size: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    if size < 20 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    return (*profile.offset(16 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'R' as i32
        && *profile.offset(17 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'G' as i32
        && *profile.offset(18 as std::os::raw::c_int as isize) as std::os::raw::c_int == 'B' as i32
        && *profile.offset(19 as std::os::raw::c_int as isize) as std::os::raw::c_int == ' ' as i32)
        as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut image: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut state: * mut crate::src::lodepng::LodePNGState,
) -> std::os::raw::c_uint {
    let mut current_block: u64;
    let mut data = 0 as *mut std::os::raw::c_uchar;
    let mut datasize = 0 as std::os::raw::c_int as size_t;
    let mut outv = ucvector_init(0 as *mut std::os::raw::c_uchar, 0 as std::os::raw::c_int as size_t);
    let mut info = LodePNGInfo {
        compression_method: 0,
        filter_method: 0,
        interlace_method: 0,
        color: LodePNGColorMode {
            colortype: LCT_GREY,
            bitdepth: 0,
            palette: 0 as *mut std::os::raw::c_uchar,
            palettesize: 0,
            key_defined: 0,
            key_r: 0,
            key_g: 0,
            key_b: 0,
        },
        background_defined: 0,
        background_r: 0,
        background_g: 0,
        background_b: 0,
        text_num: 0,
        text_keys: 0 as *mut *mut std::os::raw::c_char,
        text_strings: 0 as *mut *mut std::os::raw::c_char,
        itext_num: 0,
        itext_keys: 0 as *mut *mut std::os::raw::c_char,
        itext_langtags: 0 as *mut *mut std::os::raw::c_char,
        itext_transkeys: 0 as *mut *mut std::os::raw::c_char,
        itext_strings: 0 as *mut *mut std::os::raw::c_char,
        time_defined: 0,
        time: LodePNGTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        },
        phys_defined: 0,
        phys_x: 0,
        phys_y: 0,
        phys_unit: 0,
        gama_defined: 0,
        gama_gamma: 0,
        chrm_defined: 0,
        chrm_white_x: 0,
        chrm_white_y: 0,
        chrm_red_x: 0,
        chrm_red_y: 0,
        chrm_green_x: 0,
        chrm_green_y: 0,
        chrm_blue_x: 0,
        chrm_blue_y: 0,
        srgb_defined: 0,
        srgb_intent: 0,
        iccp_defined: 0,
        iccp_name: 0 as *mut std::os::raw::c_char,
        iccp_profile: 0 as *mut std::os::raw::c_uchar,
        iccp_profile_size: 0,
        sbit_defined: 0,
        sbit_r: 0,
        sbit_g: 0,
        sbit_b: 0,
        sbit_a: 0,
        unknown_chunks_data: [0 as *mut std::os::raw::c_uchar; 3],
        unknown_chunks_size: [0; 3],
    };
    let mut info_png: Option<&'_ crate::src::lodepng::LodePNGInfo> = Some(&mut (*state).info_png);
    let mut auto_color = LodePNGColorMode {
        colortype: LCT_GREY,
        bitdepth: 0,
        palette: 0 as *mut std::os::raw::c_uchar,
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
    };
    lodepng_info_init(&mut info);
    lodepng_color_mode_init(&mut auto_color);
    *(borrow_mut(&mut out)).unwrap() = 0 as *mut std::os::raw::c_uchar;
    *(borrow_mut(&mut outsize)).unwrap() = 0 as std::os::raw::c_int as size_t;
    (*state).error = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    if ((*((info_png).clone()).unwrap()).color.colortype as std::os::raw::c_uint
        == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
        || (*state).encoder.force_palette != 0)
        && ((*((info_png).clone()).unwrap()).color.palettesize == 0 as std::os::raw::c_int as std::os::raw::c_ulong
            || (*((info_png).clone()).unwrap()).color.palettesize > 256 as std::os::raw::c_int as std::os::raw::c_ulong)
    {
        (*state).error = 68 as std::os::raw::c_int as std::os::raw::c_uint;
    } else if (*state).encoder.zlibsettings.btype > 2 as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).error = 61 as std::os::raw::c_int as std::os::raw::c_uint;
    } else if (*((info_png).clone()).unwrap()).interlace_method > 1 as std::os::raw::c_int as std::os::raw::c_uint {
        (*state).error = 71 as std::os::raw::c_int as std::os::raw::c_uint;
    } else {
        (*state)
            .error = checkColorValidity(
            (*(info_png).unwrap()).color.colortype,
            (*(info_png).unwrap()).color.bitdepth,
        );
        if !((*state).error != 0) {
            (*state)
                .error = checkColorValidity(
                (*state).info_raw.colortype,
                (*state).info_raw.bitdepth,
            );
            if !((*state).error != 0) {
                lodepng_info_copy(&mut info, &mut (*state).info_png);
                if (*state).encoder.auto_convert != 0 {
                    let mut stats = LodePNGColorStats {
                        colored: 0,
                        key: 0,
                        key_r: 0,
                        key_g: 0,
                        key_b: 0,
                        alpha: 0,
                        numcolors: 0,
                        palette: [0; 1024],
                        bits: 0,
                        numpixels: 0,
                        allow_palette: 0,
                        allow_greyscale: 0,
                    };
                    let mut allow_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                    lodepng_color_stats_init(&mut stats);
                    if (*((info_png).clone()).unwrap()).iccp_defined != 0
                        && isGrayICCProfile(
                            (*((info_png).clone()).unwrap()).iccp_profile,
                            (*(info_png).unwrap()).iccp_profile_size,
                        ) != 0
                    {
                        stats.allow_palette = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    }
                    if (*((info_png).clone()).unwrap()).iccp_defined != 0
                        && isRGBICCProfile(
                            (*((info_png).clone()).unwrap()).iccp_profile,
                            (*(info_png).unwrap()).iccp_profile_size,
                        ) != 0
                    {
                        stats.allow_greyscale = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    }
                    (*state)
                        .error = lodepng_compute_color_stats(
                        &mut stats,
                        image,
                        w,
                        h,
                        Some(&mut (*state).info_raw),
                    );
                    if (*state).error != 0 {
                        current_block = 11418055246242690407;
                    } else {
                        if (*((info_png).clone()).unwrap()).background_defined != 0 {
                            let mut r = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            let mut g = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            let mut b = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            let mut mode16 = lodepng_color_mode_make(
                                LCT_RGB,
                                16 as std::os::raw::c_int as std::os::raw::c_uint,
                            );
                            lodepng_convert_rgb(
                                Some(&mut r),
                                Some(&mut g),
                                Some(&mut b),
                                (*(info_png).unwrap()).background_r,
                                (*(info_png).unwrap()).background_g,
                                (*(info_png).unwrap()).background_b,
                                Some(&mut mode16),
                                (Some(&(*((info_png).clone()).unwrap()).color)).clone(),
                            );
                            (*state)
                                .error = lodepng_color_stats_add(
                                &mut stats,
                                r,
                                g,
                                b,
                                65535 as std::os::raw::c_int as std::os::raw::c_uint,
                            );
                            if (*state).error != 0 {
                                current_block = 11418055246242690407;
                            } else {
                                current_block = 2604890879466389055;
                            }
                        } else {
                            current_block = 2604890879466389055;
                        }
                        match current_block {
                            11418055246242690407 => {}
                            _ => {
                                (*state)
                                    .error = auto_choose_color(
                                    &mut auto_color,
                                    &mut (*state).info_raw,
                                    Some(&mut stats),
                                );
                                if (*state).error != 0 {
                                    current_block = 11418055246242690407;
                                } else {
                                    if (*((info_png).clone()).unwrap()).sbit_defined != 0 {
                                        let mut sbit_max = if (if (if (*((info_png).clone()).unwrap()).sbit_r
                                            > (*((info_png).clone()).unwrap()).sbit_g
                                        {
                                            (*((info_png).clone()).unwrap()).sbit_r
                                        } else {
                                            (*((info_png).clone()).unwrap()).sbit_g
                                        }) > (*((info_png).clone()).unwrap()).sbit_b
                                        {
                                            (if (*((info_png).clone()).unwrap()).sbit_r > (*((info_png).clone()).unwrap()).sbit_g {
                                                (*((info_png).clone()).unwrap()).sbit_r
                                            } else {
                                                (*((info_png).clone()).unwrap()).sbit_g
                                            })
                                        } else {
                                            (*((info_png).clone()).unwrap()).sbit_b
                                        }) > (*((info_png).clone()).unwrap()).sbit_a
                                        {
                                            if (if (*((info_png).clone()).unwrap()).sbit_r > (*((info_png).clone()).unwrap()).sbit_g {
                                                (*((info_png).clone()).unwrap()).sbit_r
                                            } else {
                                                (*((info_png).clone()).unwrap()).sbit_g
                                            }) > (*((info_png).clone()).unwrap()).sbit_b
                                            {
                                                if (*((info_png).clone()).unwrap()).sbit_r > (*((info_png).clone()).unwrap()).sbit_g {
                                                    (*(info_png).unwrap()).sbit_r
                                                } else {
                                                    (*(info_png).unwrap()).sbit_g
                                                }
                                            } else {
                                                (*(info_png).unwrap()).sbit_b
                                            }
                                        } else {
                                            (*(info_png).unwrap()).sbit_a
                                        };
                                        let mut equal = (((*((info_png).clone()).unwrap()).sbit_g == 0
                                            || (*((info_png).clone()).unwrap()).sbit_g == (*((info_png).clone()).unwrap()).sbit_r)
                                            && ((*((info_png).clone()).unwrap()).sbit_b == 0
                                                || (*((info_png).clone()).unwrap()).sbit_b == (*((info_png).clone()).unwrap()).sbit_r)
                                            && ((*((info_png).clone()).unwrap()).sbit_a == 0
                                                || (*((info_png).clone()).unwrap()).sbit_a == (*((info_png).clone()).unwrap()).sbit_r)) as std::os::raw::c_int
                                            as std::os::raw::c_uint;
                                        allow_convert = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                        if info.color.colortype as std::os::raw::c_uint
                                            == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
                                            && auto_color.colortype as std::os::raw::c_uint
                                                == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
                                        {
                                            allow_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                        }
                                        if info.color.colortype as std::os::raw::c_uint
                                            == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
                                            && auto_color.colortype as std::os::raw::c_uint
                                                == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
                                            && sbit_max <= 8 as std::os::raw::c_int as std::os::raw::c_uint
                                        {
                                            allow_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                        }
                                        if info.color.colortype as std::os::raw::c_uint
                                            == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint
                                            && auto_color.colortype as std::os::raw::c_uint
                                                == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
                                            && (*((info_png).clone()).unwrap()).sbit_a == 8 as std::os::raw::c_int as std::os::raw::c_uint
                                            && sbit_max <= 8 as std::os::raw::c_int as std::os::raw::c_uint
                                        {
                                            allow_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                        }
                                        if (info.color.colortype as std::os::raw::c_uint
                                            == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
                                            || info.color.colortype as std::os::raw::c_uint
                                                == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint)
                                            && info.color.bitdepth == 16 as std::os::raw::c_int as std::os::raw::c_uint
                                            && auto_color.colortype as std::os::raw::c_uint
                                                == info.color.colortype as std::os::raw::c_uint
                                            && auto_color.bitdepth == 8 as std::os::raw::c_int as std::os::raw::c_uint
                                            && sbit_max <= 8 as std::os::raw::c_int as std::os::raw::c_uint
                                        {
                                            allow_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                        }
                                        if info.color.colortype as std::os::raw::c_uint
                                            != LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
                                            && auto_color.colortype as std::os::raw::c_uint
                                                != LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint && equal != 0
                                            && (*((info_png).clone()).unwrap()).sbit_r == auto_color.bitdepth
                                        {
                                            allow_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                        }
                                    }
                                    if (*state).encoder.force_palette != 0 {
                                        if info.color.colortype as std::os::raw::c_uint
                                            != LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
                                            && info.color.colortype as std::os::raw::c_uint
                                                != LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint
                                            && (auto_color.colortype as std::os::raw::c_uint
                                                == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
                                                || auto_color.colortype as std::os::raw::c_uint
                                                    == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint)
                                        {
                                            allow_convert = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                                        }
                                    }
                                    if allow_convert != 0 {
                                        lodepng_color_mode_copy(&mut info.color, &mut auto_color);
                                        if (*((info_png).clone()).unwrap()).background_defined != 0 {
                                            if lodepng_convert_rgb(
                                                Some(&mut info.background_r),
                                                Some(&mut info.background_g),
                                                Some(&mut info.background_b),
                                                (*(info_png).unwrap()).background_r,
                                                (*(info_png).unwrap()).background_g,
                                                (*(info_png).unwrap()).background_b,
                                                Some(&mut info.color),
                                                (Some(&(*((info_png).clone()).unwrap()).color)).clone(),
                                            ) != 0
                                            {
                                                (*state).error = 104 as std::os::raw::c_int as std::os::raw::c_uint;
                                                current_block = 11418055246242690407;
                                            } else {
                                                current_block = 10067844863897285902;
                                            }
                                        } else {
                                            current_block = 10067844863897285902;
                                        }
                                    } else {
                                        current_block = 10067844863897285902;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    current_block = 10067844863897285902;
                }
                match current_block {
                    11418055246242690407 => {}
                    _ => {
                        if (*((info_png).clone()).unwrap()).iccp_defined != 0 {
                            let mut gray_icc = isGrayICCProfile(
                                (*((info_png).clone()).unwrap()).iccp_profile,
                                (*(info_png).unwrap()).iccp_profile_size,
                            );
                            let mut rgb_icc = isRGBICCProfile(
                                (*((info_png).clone()).unwrap()).iccp_profile,
                                (*(info_png).unwrap()).iccp_profile_size,
                            );
                            let mut gray_png = (info.color.colortype as std::os::raw::c_uint
                                == LCT_GREY as std::os::raw::c_int as std::os::raw::c_uint
                                || info.color.colortype as std::os::raw::c_uint
                                    == LCT_GREY_ALPHA as std::os::raw::c_int as std::os::raw::c_uint)
                                as std::os::raw::c_int as std::os::raw::c_uint;
                            if gray_icc == 0 && rgb_icc == 0 {
                                (*state).error = 100 as std::os::raw::c_int as std::os::raw::c_uint;
                                current_block = 11418055246242690407;
                            } else if gray_icc != gray_png {
                                (*state)
                                    .error = (if (*state).encoder.auto_convert != 0 {
                                    102 as std::os::raw::c_int
                                } else {
                                    101 as std::os::raw::c_int
                                }) as std::os::raw::c_uint;
                                current_block = 11418055246242690407;
                            } else {
                                current_block = 10809827304263610514;
                            }
                        } else {
                            current_block = 10809827304263610514;
                        }
                        match current_block {
                            11418055246242690407 => {}
                            _ => {
                                if lodepng_color_mode_equal(
                                    Some(&mut (*state).info_raw),
                                    Some(&mut info.color),
                                ) == 0
                                {
                                    let mut converted = 0 as *mut std::os::raw::c_uchar;
                                    let mut size = (w as size_t)
                                        .wrapping_mul(h as size_t)
                                        .wrapping_mul(lodepng_get_bpp(Some(&mut info.color)) as size_t)
                                        .wrapping_add(7 as std::os::raw::c_uint as std::os::raw::c_ulong)
                                        .wrapping_div(8 as std::os::raw::c_uint as std::os::raw::c_ulong);
                                    converted = lodepng_malloc(size) as *mut std::os::raw::c_uchar;
                                    if converted.is_null() && size != 0 {
                                        (*state).error = 83 as std::os::raw::c_int as std::os::raw::c_uint;
                                    }
                                    if (*state).error == 0 {
                                        (*state)
                                            .error = lodepng_convert(
                                            converted,
                                            image,
                                            Some(&mut info.color),
                                            Some(&mut (*state).info_raw),
                                            w,
                                            h,
                                        );
                                    }
                                    if (*state).error == 0 {
                                        (*state)
                                            .error = preProcessScanlines(
                                            Some(&mut data),
                                            Some(&mut datasize),
                                            converted,
                                            w,
                                            h,
                                            Some(&mut info),
                                            Some(&mut (*state).encoder),
                                        );
                                    }
                                    lodepng_free(converted as *mut std::os::raw::c_void);
                                    if (*state).error != 0 {
                                        current_block = 11418055246242690407;
                                    } else {
                                        current_block = 11071260907632769126;
                                    }
                                } else {
                                    (*state)
                                        .error = preProcessScanlines(
                                        Some(&mut data),
                                        Some(&mut datasize),
                                        image,
                                        w,
                                        h,
                                        Some(&mut info),
                                        Some(&mut (*state).encoder),
                                    );
                                    if (*state).error != 0 {
                                        current_block = 11418055246242690407;
                                    } else {
                                        current_block = 11071260907632769126;
                                    }
                                }
                                match current_block {
                                    11418055246242690407 => {}
                                    _ => {
                                        let mut i: u64 = 0;
                                        (*state).error = writeSignature(&mut outv);
                                        if !((*state).error != 0) {
                                            (*state)
                                                .error = addChunk_IHDR(
                                                &mut outv,
                                                w,
                                                h,
                                                info.color.colortype,
                                                info.color.bitdepth,
                                                info.interlace_method,
                                            );
                                            if !((*state).error != 0) {
                                                if !(info.unknown_chunks_data[0 as std::os::raw::c_int as usize])
                                                    .is_null()
                                                {
                                                    (*state)
                                                        .error = addUnknownChunks(
                                                        &mut outv,
                                                        info.unknown_chunks_data[0 as std::os::raw::c_int as usize],
                                                        info.unknown_chunks_size[0 as std::os::raw::c_int as usize],
                                                    );
                                                    if (*state).error != 0 {
                                                        current_block = 11418055246242690407;
                                                    } else {
                                                        current_block = 4899250571165509867;
                                                    }
                                                } else {
                                                    current_block = 4899250571165509867;
                                                }
                                                match current_block {
                                                    11418055246242690407 => {}
                                                    _ => {
                                                        if info.iccp_defined != 0 {
                                                            (*state)
                                                                .error = addChunk_iCCP(
                                                                &mut outv,
                                                                Some(&mut info),
                                                                &mut (*state).encoder.zlibsettings,
                                                            );
                                                            if (*state).error != 0 {
                                                                current_block = 11418055246242690407;
                                                            } else {
                                                                current_block = 5265702136860997526;
                                                            }
                                                        } else {
                                                            current_block = 5265702136860997526;
                                                        }
                                                        match current_block {
                                                            11418055246242690407 => {}
                                                            _ => {
                                                                if info.srgb_defined != 0 {
                                                                    (*state).error = addChunk_sRGB(&mut outv, Some(&mut info));
                                                                    if (*state).error != 0 {
                                                                        current_block = 11418055246242690407;
                                                                    } else {
                                                                        current_block = 5409161009579131794;
                                                                    }
                                                                } else {
                                                                    current_block = 5409161009579131794;
                                                                }
                                                                match current_block {
                                                                    11418055246242690407 => {}
                                                                    _ => {
                                                                        if info.gama_defined != 0 {
                                                                            (*state).error = addChunk_gAMA(&mut outv, Some(&mut info));
                                                                            if (*state).error != 0 {
                                                                                current_block = 11418055246242690407;
                                                                            } else {
                                                                                current_block = 10109057886293123569;
                                                                            }
                                                                        } else {
                                                                            current_block = 10109057886293123569;
                                                                        }
                                                                        match current_block {
                                                                            11418055246242690407 => {}
                                                                            _ => {
                                                                                if info.chrm_defined != 0 {
                                                                                    (*state).error = addChunk_cHRM(&mut outv, Some(&mut info));
                                                                                    if (*state).error != 0 {
                                                                                        current_block = 11418055246242690407;
                                                                                    } else {
                                                                                        current_block = 14612007084265645573;
                                                                                    }
                                                                                } else {
                                                                                    current_block = 14612007084265645573;
                                                                                }
                                                                                match current_block {
                                                                                    11418055246242690407 => {}
                                                                                    _ => {
                                                                                        if (*((info_png).clone()).unwrap()).sbit_defined != 0 {
                                                                                            (*state).error = addChunk_sBIT(&mut outv, Some(&mut info));
                                                                                            if (*state).error != 0 {
                                                                                                current_block = 11418055246242690407;
                                                                                            } else {
                                                                                                current_block = 12963528325254160332;
                                                                                            }
                                                                                        } else {
                                                                                            current_block = 12963528325254160332;
                                                                                        }
                                                                                        match current_block {
                                                                                            11418055246242690407 => {}
                                                                                            _ => {
                                                                                                if info.color.colortype as std::os::raw::c_uint
                                                                                                    == LCT_PALETTE as std::os::raw::c_int as std::os::raw::c_uint
                                                                                                {
                                                                                                    (*state).error = addChunk_PLTE(&mut outv, Some(&mut info.color));
                                                                                                    if (*state).error != 0 {
                                                                                                        current_block = 11418055246242690407;
                                                                                                    } else {
                                                                                                        current_block = 15417752026496523887;
                                                                                                    }
                                                                                                } else {
                                                                                                    current_block = 15417752026496523887;
                                                                                                }
                                                                                                match current_block {
                                                                                                    11418055246242690407 => {}
                                                                                                    _ => {
                                                                                                        if (*state).encoder.force_palette != 0
                                                                                                            && (info.color.colortype as std::os::raw::c_uint
                                                                                                                == LCT_RGB as std::os::raw::c_int as std::os::raw::c_uint
                                                                                                                || info.color.colortype as std::os::raw::c_uint
                                                                                                                    == LCT_RGBA as std::os::raw::c_int as std::os::raw::c_uint)
                                                                                                        {
                                                                                                            (*state).error = addChunk_PLTE(&mut outv, Some(&mut info.color));
                                                                                                            if (*state).error != 0 {
                                                                                                                current_block = 11418055246242690407;
                                                                                                            } else {
                                                                                                                current_block = 17736998403848444560;
                                                                                                            }
                                                                                                        } else {
                                                                                                            current_block = 17736998403848444560;
                                                                                                        }
                                                                                                        match current_block {
                                                                                                            11418055246242690407 => {}
                                                                                                            _ => {
                                                                                                                (*state).error = addChunk_tRNS(&mut outv, Some(&mut info.color));
                                                                                                                if !((*state).error != 0) {
                                                                                                                    if info.background_defined != 0 {
                                                                                                                        (*state).error = addChunk_bKGD(&mut outv, Some(&mut info));
                                                                                                                        if (*state).error != 0 {
                                                                                                                            current_block = 11418055246242690407;
                                                                                                                        } else {
                                                                                                                            current_block = 18201902862271706575;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        current_block = 18201902862271706575;
                                                                                                                    }
                                                                                                                    match current_block {
                                                                                                                        11418055246242690407 => {}
                                                                                                                        _ => {
                                                                                                                            if info.phys_defined != 0 {
                                                                                                                                (*state).error = addChunk_pHYs(&mut outv, Some(&mut info));
                                                                                                                                if (*state).error != 0 {
                                                                                                                                    current_block = 11418055246242690407;
                                                                                                                                } else {
                                                                                                                                    current_block = 12608488225262500095;
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                current_block = 12608488225262500095;
                                                                                                                            }
                                                                                                                            match current_block {
                                                                                                                                11418055246242690407 => {}
                                                                                                                                _ => {
                                                                                                                                    if !(info.unknown_chunks_data[1 as std::os::raw::c_int as usize])
                                                                                                                                        .is_null()
                                                                                                                                    {
                                                                                                                                        (*state)
                                                                                                                                            .error = addUnknownChunks(
                                                                                                                                            &mut outv,
                                                                                                                                            info.unknown_chunks_data[1 as std::os::raw::c_int as usize],
                                                                                                                                            info.unknown_chunks_size[1 as std::os::raw::c_int as usize],
                                                                                                                                        );
                                                                                                                                        if (*state).error != 0 {
                                                                                                                                            current_block = 11418055246242690407;
                                                                                                                                        } else {
                                                                                                                                            current_block = 9343041660989783267;
                                                                                                                                        }
                                                                                                                                    } else {
                                                                                                                                        current_block = 9343041660989783267;
                                                                                                                                    }
                                                                                                                                    match current_block {
                                                                                                                                        11418055246242690407 => {}
                                                                                                                                        _ => {
                                                                                                                                            (*state)
                                                                                                                                                .error = addChunk_IDAT(
                                                                                                                                                &mut outv,
                                                                                                                                                data,
                                                                                                                                                datasize,
                                                                                                                                                &mut (*state).encoder.zlibsettings,
                                                                                                                                            );
                                                                                                                                            if !((*state).error != 0) {
                                                                                                                                                if info.time_defined != 0 {
                                                                                                                                                    (*state).error = addChunk_tIME(&mut outv, Some(&mut info.time));
                                                                                                                                                    if (*state).error != 0 {
                                                                                                                                                        current_block = 11418055246242690407;
                                                                                                                                                    } else {
                                                                                                                                                        current_block = 5710330377809666066;
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    current_block = 5710330377809666066;
                                                                                                                                                }
                                                                                                                                                match current_block {
                                                                                                                                                    11418055246242690407 => {}
                                                                                                                                                    _ => {
                                                                                                                                                        i = 0 as std::os::raw::c_int as size_t;
                                                                                                                                                        loop {
                                                                                                                                                            if !(i != info.text_num) {
                                                                                                                                                                current_block = 5511877782510663281;
                                                                                                                                                                break;
                                                                                                                                                            }
                                                                                                                                                            if lodepng_strlen(*(info.text_keys).offset(i as isize))
                                                                                                                                                                > 79 as std::os::raw::c_int as std::os::raw::c_ulong
                                                                                                                                                            {
                                                                                                                                                                (*state).error = 66 as std::os::raw::c_int as std::os::raw::c_uint;
                                                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                                                break;
                                                                                                                                                            } else if lodepng_strlen(
                                                                                                                                                                *(info.text_keys).offset(i as isize),
                                                                                                                                                            ) < 1 as std::os::raw::c_int as std::os::raw::c_ulong
                                                                                                                                                            {
                                                                                                                                                                (*state).error = 67 as std::os::raw::c_int as std::os::raw::c_uint;
                                                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                                                break;
                                                                                                                                                            } else {
                                                                                                                                                                if (*state).encoder.text_compression != 0 {
                                                                                                                                                                    (*state)
                                                                                                                                                                        .error = addChunk_zTXt(
                                                                                                                                                                        &mut outv,
                                                                                                                                                                        *(info.text_keys).offset(i as isize),
                                                                                                                                                                        *(info.text_strings).offset(i as isize),
                                                                                                                                                                        &mut (*state).encoder.zlibsettings,
                                                                                                                                                                    );
                                                                                                                                                                    if (*state).error != 0 {
                                                                                                                                                                        current_block = 11418055246242690407;
                                                                                                                                                                        break;
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    (*state)
                                                                                                                                                                        .error = addChunk_tEXt(
                                                                                                                                                                        &mut outv,
                                                                                                                                                                        *(info.text_keys).offset(i as isize),
                                                                                                                                                                        *(info.text_strings).offset(i as isize),
                                                                                                                                                                    );
                                                                                                                                                                    if (*state).error != 0 {
                                                                                                                                                                        current_block = 11418055246242690407;
                                                                                                                                                                        break;
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                                i = i.wrapping_add(1);
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                        match current_block {
                                                                                                                                                            11418055246242690407 => {}
                                                                                                                                                            _ => {
                                                                                                                                                                if (*state).encoder.add_id != 0 {
                                                                                                                                                                    let mut already_added_id_text = 0 as std::os::raw::c_int
                                                                                                                                                                        as std::os::raw::c_uint;
                                                                                                                                                                    i = 0 as std::os::raw::c_int as size_t;
                                                                                                                                                                    while i != info.text_num {
                                                                                                                                                                        let mut k: * const i8 = *(info.text_keys)
                                                                                                                                                                            .offset(i as isize);
                                                                                                                                                                        if *k.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                            == 'L' as i32
                                                                                                                                                                            && *k.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                                == 'o' as i32
                                                                                                                                                                            && *k.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                                == 'd' as i32
                                                                                                                                                                            && *k.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                                == 'e' as i32
                                                                                                                                                                            && *k.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                                == 'P' as i32
                                                                                                                                                                            && *k.offset(5 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                                == 'N' as i32
                                                                                                                                                                            && *k.offset(6 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                                == 'G' as i32
                                                                                                                                                                            && *k.offset(7 as std::os::raw::c_int as isize) as std::os::raw::c_int
                                                                                                                                                                                == '\0' as i32
                                                                                                                                                                        {
                                                                                                                                                                            already_added_id_text = 1 as std::os::raw::c_int as std::os::raw::c_uint;
                                                                                                                                                                            break;
                                                                                                                                                                        } else {
                                                                                                                                                                            i = i.wrapping_add(1);
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                    if already_added_id_text == 0 as std::os::raw::c_int as std::os::raw::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        (*state)
                                                                                                                                                                            .error = addChunk_tEXt(
                                                                                                                                                                            &mut outv,
                                                                                                                                                                            b"LodePNG\0" as *const u8 as *const std::os::raw::c_char,
                                                                                                                                                                            LODEPNG_VERSION_STRING,
                                                                                                                                                                        );
                                                                                                                                                                        if (*state).error != 0 {
                                                                                                                                                                            current_block = 11418055246242690407;
                                                                                                                                                                        } else {
                                                                                                                                                                            current_block = 6880299496751257707;
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        current_block = 6880299496751257707;
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    current_block = 6880299496751257707;
                                                                                                                                                                }
                                                                                                                                                                match current_block {
                                                                                                                                                                    11418055246242690407 => {}
                                                                                                                                                                    _ => {
                                                                                                                                                                        i = 0 as std::os::raw::c_int as size_t;
                                                                                                                                                                        loop {
                                                                                                                                                                            if !(i != info.itext_num) {
                                                                                                                                                                                current_block = 13796196565926322821;
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                            if lodepng_strlen(*(info.itext_keys).offset(i as isize))
                                                                                                                                                                                > 79 as std::os::raw::c_int as std::os::raw::c_ulong
                                                                                                                                                                            {
                                                                                                                                                                                (*state).error = 66 as std::os::raw::c_int as std::os::raw::c_uint;
                                                                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                                                                break;
                                                                                                                                                                            } else if lodepng_strlen(
                                                                                                                                                                                *(info.itext_keys).offset(i as isize),
                                                                                                                                                                            ) < 1 as std::os::raw::c_int as std::os::raw::c_ulong
                                                                                                                                                                            {
                                                                                                                                                                                (*state).error = 67 as std::os::raw::c_int as std::os::raw::c_uint;
                                                                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                                                                break;
                                                                                                                                                                            } else {
                                                                                                                                                                                (*state)
                                                                                                                                                                                    .error = addChunk_iTXt(
                                                                                                                                                                                    &mut outv,
                                                                                                                                                                                    (*state).encoder.text_compression,
                                                                                                                                                                                    *(info.itext_keys).offset(i as isize),
                                                                                                                                                                                    *(info.itext_langtags).offset(i as isize),
                                                                                                                                                                                    *(info.itext_transkeys).offset(i as isize),
                                                                                                                                                                                    *(info.itext_strings).offset(i as isize),
                                                                                                                                                                                    &mut (*state).encoder.zlibsettings,
                                                                                                                                                                                );
                                                                                                                                                                                if (*state).error != 0 {
                                                                                                                                                                                    current_block = 11418055246242690407;
                                                                                                                                                                                    break;
                                                                                                                                                                                }
                                                                                                                                                                                i = i.wrapping_add(1);
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                        match current_block {
                                                                                                                                                                            11418055246242690407 => {}
                                                                                                                                                                            _ => {
                                                                                                                                                                                if !(info.unknown_chunks_data[2 as std::os::raw::c_int as usize])
                                                                                                                                                                                    .is_null()
                                                                                                                                                                                {
                                                                                                                                                                                    (*state)
                                                                                                                                                                                        .error = addUnknownChunks(
                                                                                                                                                                                        &mut outv,
                                                                                                                                                                                        info.unknown_chunks_data[2 as std::os::raw::c_int as usize],
                                                                                                                                                                                        info.unknown_chunks_size[2 as std::os::raw::c_int as usize],
                                                                                                                                                                                    );
                                                                                                                                                                                    if (*state).error != 0 {
                                                                                                                                                                                        current_block = 11418055246242690407;
                                                                                                                                                                                    } else {
                                                                                                                                                                                        current_block = 11322929247169729670;
                                                                                                                                                                                    }
                                                                                                                                                                                } else {
                                                                                                                                                                                    current_block = 11322929247169729670;
                                                                                                                                                                                }
                                                                                                                                                                                match current_block {
                                                                                                                                                                                    11418055246242690407 => {}
                                                                                                                                                                                    _ => {
                                                                                                                                                                                        (*state).error = addChunk_IEND(&mut outv);
                                                                                                                                                                                        (*state).error != 0;
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    lodepng_info_cleanup(&mut info);
    lodepng_free(data as *mut std::os::raw::c_void);
    lodepng_color_mode_cleanup(&mut auto_color);
    *(borrow_mut(&mut out)).unwrap() = outv.data;
    *(borrow_mut(&mut outsize)).unwrap() = outv.size;
    return (*state).error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode_memory<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut image: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut error: u32 = 0;
    let mut state = LodePNGState {
        decoder: LodePNGDecoderSettings {
            zlibsettings: LodePNGDecompressSettings {
                ignore_adler32: 0,
                ignore_nlen: 0,
                max_output_size: 0,
                custom_zlib: None,
                custom_inflate: None,
                custom_context: 0 as *const std::os::raw::c_void,
            },
            ignore_crc: 0,
            ignore_critical: 0,
            ignore_end: 0,
            color_convert: 0,
            read_text_chunks: 0,
            remember_unknown_chunks: 0,
            max_text_size: 0,
            max_icc_size: 0,
        },
        encoder: LodePNGEncoderSettings {
            zlibsettings: LodePNGCompressSettings {
                btype: 0,
                use_lz77: 0,
                windowsize: 0,
                minmatch: 0,
                nicematch: 0,
                lazymatching: 0,
                custom_zlib: None,
                custom_deflate: None,
                custom_context: 0 as *const std::os::raw::c_void,
            },
            auto_convert: 0,
            filter_palette_zero: 0,
            filter_strategy: LFS_ZERO,
            predefined_filters: (0 as * const u8),
            force_palette: 0,
            add_id: 0,
            text_compression: 0,
        },
        info_raw: LodePNGColorMode {
            colortype: LCT_GREY,
            bitdepth: 0,
            palette: 0 as *mut std::os::raw::c_uchar,
            palettesize: 0,
            key_defined: 0,
            key_r: 0,
            key_g: 0,
            key_b: 0,
        },
        info_png: LodePNGInfo {
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            color: LodePNGColorMode {
                colortype: LCT_GREY,
                bitdepth: 0,
                palette: 0 as *mut std::os::raw::c_uchar,
                palettesize: 0,
                key_defined: 0,
                key_r: 0,
                key_g: 0,
                key_b: 0,
            },
            background_defined: 0,
            background_r: 0,
            background_g: 0,
            background_b: 0,
            text_num: 0,
            text_keys: 0 as *mut *mut std::os::raw::c_char,
            text_strings: 0 as *mut *mut std::os::raw::c_char,
            itext_num: 0,
            itext_keys: 0 as *mut *mut std::os::raw::c_char,
            itext_langtags: 0 as *mut *mut std::os::raw::c_char,
            itext_transkeys: 0 as *mut *mut std::os::raw::c_char,
            itext_strings: 0 as *mut *mut std::os::raw::c_char,
            time_defined: 0,
            time: LodePNGTime {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
            },
            phys_defined: 0,
            phys_x: 0,
            phys_y: 0,
            phys_unit: 0,
            gama_defined: 0,
            gama_gamma: 0,
            chrm_defined: 0,
            chrm_white_x: 0,
            chrm_white_y: 0,
            chrm_red_x: 0,
            chrm_red_y: 0,
            chrm_green_x: 0,
            chrm_green_y: 0,
            chrm_blue_x: 0,
            chrm_blue_y: 0,
            srgb_defined: 0,
            srgb_intent: 0,
            iccp_defined: 0,
            iccp_name: 0 as *mut std::os::raw::c_char,
            iccp_profile: 0 as *mut std::os::raw::c_uchar,
            iccp_profile_size: 0,
            sbit_defined: 0,
            sbit_r: 0,
            sbit_g: 0,
            sbit_b: 0,
            sbit_a: 0,
            unknown_chunks_data: [0 as *mut std::os::raw::c_uchar; 3],
            unknown_chunks_size: [0; 3],
        },
        error: 0,
    };
    lodepng_state_init(Some(&mut state));
    state.info_raw.colortype = colortype;
    state.info_raw.bitdepth = bitdepth;
    state.info_png.color.colortype = colortype;
    state.info_png.color.bitdepth = bitdepth;
    lodepng_encode(borrow_mut(&mut out), borrow_mut(&mut outsize), image, w, h, &mut state);
    error = state.error;
    lodepng_state_cleanup(Some(&mut state));
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode32<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut image: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    return lodepng_encode_memory(
        borrow_mut(&mut out),
        borrow_mut(&mut outsize),
        image,
        w,
        h,
        LCT_RGBA,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode24<'a1, 'a2>(
    mut out: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut outsize: Option<&'a2 mut std::os::raw::c_ulong>,
    mut image: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    return lodepng_encode_memory(
        borrow_mut(&mut out),
        borrow_mut(&mut outsize),
        image,
        w,
        h,
        LCT_RGB,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode_file(
    mut filename: * const std::os::raw::c_char,
    mut image: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
    mut colortype: std::os::raw::c_uint,
    mut bitdepth: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    let mut buffer = 0 as *mut std::os::raw::c_uchar;
    let mut buffersize: u64 = 0;
    let mut error = lodepng_encode_memory(
        Some(&mut buffer),
        Some(&mut buffersize),
        image,
        w,
        h,
        colortype,
        bitdepth,
    );
    if error == 0 {
        error = lodepng_save_file(buffer, buffersize, filename);
    }
    lodepng_free(buffer as *mut std::os::raw::c_void);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode32_file(
    mut filename: * const std::os::raw::c_char,
    mut image: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    return lodepng_encode_file(
        filename,
        image,
        w,
        h,
        LCT_RGBA,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encode24_file(
    mut filename: * const std::os::raw::c_char,
    mut image: * const std::os::raw::c_uchar,
    mut w: std::os::raw::c_uint,
    mut h: std::os::raw::c_uint,
) -> std::os::raw::c_uint {
    return lodepng_encode_file(
        filename,
        image,
        w,
        h,
        LCT_RGB,
        8 as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lodepng_encoder_settings_init<'a1>(
    mut settings: Option<&'a1 mut crate::src::lodepng::LodePNGEncoderSettings>,
) {
    lodepng_compress_settings_init(Some(&mut (*(borrow_mut(&mut settings)).unwrap()).zlibsettings));
    (*(borrow_mut(&mut settings)).unwrap()).filter_palette_zero = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).filter_strategy = LFS_MINSUM;
    (*(borrow_mut(&mut settings)).unwrap()).auto_convert = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).force_palette = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    let ref mut fresh190 = (*(borrow_mut(&mut settings)).unwrap()).predefined_filters;
    *fresh190 = (0 as * const u8);
    (*(borrow_mut(&mut settings)).unwrap()).add_id = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*(borrow_mut(&mut settings)).unwrap()).text_compression = 1 as std::os::raw::c_int as std::os::raw::c_uint;
}
#[no_mangle]
pub extern "C" fn lodepng_error_text(
    mut code: std::os::raw::c_uint,
) -> * const std::os::raw::c_char {
    match code {
        0 => return b"no error, everything went ok\0" as *const u8 as *const std::os::raw::c_char,
        1 => return b"nothing done yet\0" as *const u8 as *const std::os::raw::c_char,
        10 => {
            return b"end of input memory reached without huffman end code\0" as *const u8
                as *const std::os::raw::c_char;
        }
        11 => {
            return b"error in code tree made it jump outside of huffman tree\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        13 => {
            return b"problem while processing dynamic deflate block\0" as *const u8
                as *const std::os::raw::c_char;
        }
        14 => {
            return b"problem while processing dynamic deflate block\0" as *const u8
                as *const std::os::raw::c_char;
        }
        15 => {
            return b"problem while processing dynamic deflate block\0" as *const u8
                as *const std::os::raw::c_char;
        }
        16 => {
            return b"invalid code while processing dynamic deflate block\0" as *const u8
                as *const std::os::raw::c_char;
        }
        17 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8
                as *const std::os::raw::c_char;
        }
        18 => {
            return b"invalid distance code while inflating\0" as *const u8
                as *const std::os::raw::c_char;
        }
        19 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8
                as *const std::os::raw::c_char;
        }
        20 => {
            return b"invalid deflate block BTYPE encountered while decoding\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        21 => {
            return b"NLEN is not ones complement of LEN in a deflate block\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        22 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8
                as *const std::os::raw::c_char;
        }
        23 => {
            return b"end of in buffer memory reached while inflating\0" as *const u8
                as *const std::os::raw::c_char;
        }
        24 => {
            return b"invalid FCHECK in zlib header\0" as *const u8 as *const std::os::raw::c_char;
        }
        25 => {
            return b"invalid compression method in zlib header\0" as *const u8
                as *const std::os::raw::c_char;
        }
        26 => {
            return b"FDICT encountered in zlib header while it's not used for PNG\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        27 => {
            return b"PNG file is smaller than a PNG header\0" as *const u8
                as *const std::os::raw::c_char;
        }
        28 => {
            return b"incorrect PNG signature, it's no PNG or corrupted\0" as *const u8
                as *const std::os::raw::c_char;
        }
        29 => {
            return b"first chunk is not the header chunk\0" as *const u8
                as *const std::os::raw::c_char;
        }
        30 => {
            return b"chunk length too large, chunk broken off at end of file\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        31 => {
            return b"illegal PNG color type or bpp\0" as *const u8 as *const std::os::raw::c_char;
        }
        32 => {
            return b"illegal PNG compression method\0" as *const u8
                as *const std::os::raw::c_char;
        }
        33 => return b"illegal PNG filter method\0" as *const u8 as *const std::os::raw::c_char,
        34 => {
            return b"illegal PNG interlace method\0" as *const u8 as *const std::os::raw::c_char;
        }
        35 => {
            return b"chunk length of a chunk is too large or the chunk too small\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        36 => {
            return b"illegal PNG filter type encountered\0" as *const u8
                as *const std::os::raw::c_char;
        }
        37 => {
            return b"illegal bit depth for this color type given\0" as *const u8
                as *const std::os::raw::c_char;
        }
        38 => {
            return b"the palette is too small or too big\0" as *const u8
                as *const std::os::raw::c_char;
        }
        39 => {
            return b"tRNS chunk before PLTE or has more entries than palette size\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        40 => {
            return b"tRNS chunk has wrong size for grayscale image\0" as *const u8
                as *const std::os::raw::c_char;
        }
        41 => {
            return b"tRNS chunk has wrong size for RGB image\0" as *const u8
                as *const std::os::raw::c_char;
        }
        42 => {
            return b"tRNS chunk appeared while it was not allowed for this color type\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        43 => {
            return b"bKGD chunk has wrong size for palette image\0" as *const u8
                as *const std::os::raw::c_char;
        }
        44 => {
            return b"bKGD chunk has wrong size for grayscale image\0" as *const u8
                as *const std::os::raw::c_char;
        }
        45 => {
            return b"bKGD chunk has wrong size for RGB image\0" as *const u8
                as *const std::os::raw::c_char;
        }
        48 => {
            return b"empty input buffer given to decoder. Maybe caused by non-existing file?\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        49 => {
            return b"jumped past memory while generating dynamic huffman tree\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        50 => {
            return b"jumped past memory while generating dynamic huffman tree\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        51 => {
            return b"jumped past memory while inflating huffman block\0" as *const u8
                as *const std::os::raw::c_char;
        }
        52 => {
            return b"jumped past memory while inflating\0" as *const u8
                as *const std::os::raw::c_char;
        }
        53 => return b"size of zlib data too small\0" as *const u8 as *const std::os::raw::c_char,
        54 => {
            return b"repeat symbol in tree while there was no value symbol yet\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        55 => {
            return b"jumped past tree while generating huffman tree\0" as *const u8
                as *const std::os::raw::c_char;
        }
        56 => {
            return b"given output image colortype or bitdepth not supported for color conversion\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        57 => {
            return b"invalid CRC encountered (checking CRC can be disabled)\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        58 => {
            return b"invalid ADLER32 encountered (checking ADLER32 can be disabled)\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        59 => {
            return b"requested color conversion not supported\0" as *const u8
                as *const std::os::raw::c_char;
        }
        60 => {
            return b"invalid window size given in the settings of the encoder (must be 0-32768)\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        61 => {
            return b"invalid BTYPE given in the settings of the encoder (only 0, 1 and 2 are allowed)\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        62 => {
            return b"conversion from color to grayscale not supported\0" as *const u8
                as *const std::os::raw::c_char;
        }
        63 => {
            return b"length of a chunk too long, max allowed for PNG is 2147483647 bytes per chunk\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        64 => {
            return b"the length of the END symbol 256 in the Huffman tree is 0\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        66 => {
            return b"the length of a text chunk keyword given to the encoder is longer than the maximum of 79 bytes\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        67 => {
            return b"the length of a text chunk keyword given to the encoder is smaller than the minimum of 1 byte\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        68 => {
            return b"tried to encode a PLTE chunk with a palette that has less than 1 or more than 256 colors\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        69 => {
            return b"unknown chunk type with 'critical' flag encountered by the decoder\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        71 => {
            return b"invalid interlace mode given to encoder (must be 0 or 1)\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        72 => {
            return b"while decoding, invalid compression method encountering in zTXt or iTXt chunk (it must be 0)\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        73 => return b"invalid tIME chunk size\0" as *const u8 as *const std::os::raw::c_char,
        74 => return b"invalid pHYs chunk size\0" as *const u8 as *const std::os::raw::c_char,
        75 => {
            return b"no null termination char found while decoding text chunk\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        76 => {
            return b"iTXt chunk too short to contain required bytes\0" as *const u8
                as *const std::os::raw::c_char;
        }
        77 => {
            return b"integer overflow in buffer size\0" as *const u8
                as *const std::os::raw::c_char;
        }
        78 => {
            return b"failed to open file for reading\0" as *const u8
                as *const std::os::raw::c_char;
        }
        79 => {
            return b"failed to open file for writing\0" as *const u8
                as *const std::os::raw::c_char;
        }
        80 => {
            return b"tried creating a tree of 0 symbols\0" as *const u8
                as *const std::os::raw::c_char;
        }
        81 => {
            return b"lazy matching at pos 0 is impossible\0" as *const u8
                as *const std::os::raw::c_char;
        }
        82 => {
            return b"color conversion to palette requested while a color isn't in palette, or index out of bounds\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        83 => return b"memory allocation failed\0" as *const u8 as *const std::os::raw::c_char,
        84 => {
            return b"given image too small to contain all pixels to be encoded\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        86 => {
            return b"impossible offset in lz77 encoding (internal bug)\0" as *const u8
                as *const std::os::raw::c_char;
        }
        87 => {
            return b"must provide custom zlib function pointer if LODEPNG_COMPILE_ZLIB is not defined\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        88 => {
            return b"invalid filter strategy given for LodePNGEncoderSettings.filter_strategy\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        89 => {
            return b"text chunk keyword too short or long: must have size 1-79\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        90 => {
            return b"windowsize must be a power of two\0" as *const u8
                as *const std::os::raw::c_char;
        }
        91 => {
            return b"invalid decompressed idat size\0" as *const u8
                as *const std::os::raw::c_char;
        }
        92 => {
            return b"integer overflow due to too many pixels\0" as *const u8
                as *const std::os::raw::c_char;
        }
        93 => {
            return b"zero width or height is invalid\0" as *const u8
                as *const std::os::raw::c_char;
        }
        94 => {
            return b"header chunk must have a size of 13 bytes\0" as *const u8
                as *const std::os::raw::c_char;
        }
        95 => {
            return b"integer overflow with combined idat chunk size\0" as *const u8
                as *const std::os::raw::c_char;
        }
        96 => return b"invalid gAMA chunk size\0" as *const u8 as *const std::os::raw::c_char,
        97 => return b"invalid cHRM chunk size\0" as *const u8 as *const std::os::raw::c_char,
        98 => return b"invalid sRGB chunk size\0" as *const u8 as *const std::os::raw::c_char,
        99 => {
            return b"invalid sRGB rendering intent\0" as *const u8 as *const std::os::raw::c_char;
        }
        100 => {
            return b"invalid ICC profile color type, the PNG specification only allows RGB or GRAY\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        101 => {
            return b"PNG specification does not allow RGB ICC profile on gray color types and vice versa\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        102 => {
            return b"not allowed to set grayscale ICC profile with colored pixels by PNG specification\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        103 => {
            return b"invalid palette index in bKGD chunk. Maybe it came before PLTE chunk?\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        104 => {
            return b"invalid bKGD color while encoding (e.g. palette index out of range)\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        105 => {
            return b"integer overflow of bitsize\0" as *const u8 as *const std::os::raw::c_char;
        }
        106 => {
            return b"PNG file must have PLTE chunk if color type is palette\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        107 => {
            return b"color convert from palette mode requested without setting the palette data in it\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        108 => {
            return b"tried to add more than 256 values to a palette\0" as *const u8
                as *const std::os::raw::c_char;
        }
        109 => {
            return b"tried to decompress zlib or deflate data larger than desired max_output_size\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        110 => {
            return b"custom zlib or inflate decompression failed\0" as *const u8
                as *const std::os::raw::c_char;
        }
        111 => {
            return b"custom zlib or deflate compression failed\0" as *const u8
                as *const std::os::raw::c_char;
        }
        112 => {
            return b"compressed text unreasonably large\0" as *const u8
                as *const std::os::raw::c_char;
        }
        113 => {
            return b"ICC profile unreasonably large\0" as *const u8
                as *const std::os::raw::c_char;
        }
        114 => {
            return b"sBIT chunk has wrong size for the color type of the image\0"
                as *const u8 as *const std::os::raw::c_char;
        }
        115 => return b"sBIT value out of range\0" as *const u8 as *const std::os::raw::c_char,
        _ => {}
    }
    return b"unknown error code\0" as *const u8 as *const std::os::raw::c_char;
}
unsafe extern "C" fn run_static_initializers() {
    mask = ((1 as std::os::raw::c_uint) << 9 as std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_uint);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C"  fn() -> (); 1] = [run_static_initializers];
use crate::laertes_rt::*;
