use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgba_t {
    pub r: libc::c_double,
    pub g: libc::c_double,
    pub b: libc::c_double,
    pub a: libc::c_double,
}
impl Default for rgba_t {fn default() -> Self {Self {
r: Default::default(),
g: Default::default(),
b: Default::default(),
a: Default::default(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_color {
    pub name: *const libc::c_char,
    pub val: uint32_t,
}
impl Default for named_color {fn default() -> Self {Self {
name: std::ptr::null(),
val: Default::default(),
}}}

static mut named_colors: [named_color; 149] = [
    {
        let mut init = named_color {
            name: b"transparent\0" as *const u8 as *const libc::c_char,
            val: 0xffffff00 as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aliceblue\0" as *const u8 as *const libc::c_char,
            val: 0xf0f8ffff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"antiquewhite\0" as *const u8 as *const libc::c_char,
            val: 0xfaebd7ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aqua\0" as *const u8 as *const libc::c_char,
            val: 0xffffff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aquamarine\0" as *const u8 as *const libc::c_char,
            val: 0x7fffd4ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"azure\0" as *const u8 as *const libc::c_char,
            val: 0xf0ffffff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"beige\0" as *const u8 as *const libc::c_char,
            val: 0xf5f5dcff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"bisque\0" as *const u8 as *const libc::c_char,
            val: 0xffe4c4ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"black\0" as *const u8 as *const libc::c_char,
            val: 0xff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blanchedalmond\0" as *const u8 as *const libc::c_char,
            val: 0xffebcdff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blue\0" as *const u8 as *const libc::c_char,
            val: 0xffff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blueviolet\0" as *const u8 as *const libc::c_char,
            val: 0x8a2be2ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"brown\0" as *const u8 as *const libc::c_char,
            val: 0xa52a2aff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"burlywood\0" as *const u8 as *const libc::c_char,
            val: 0xdeb887ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cadetblue\0" as *const u8 as *const libc::c_char,
            val: 0x5f9ea0ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"chartreuse\0" as *const u8 as *const libc::c_char,
            val: 0x7fff00ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"chocolate\0" as *const u8 as *const libc::c_char,
            val: 0xd2691eff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"coral\0" as *const u8 as *const libc::c_char,
            val: 0xff7f50ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cornflowerblue\0" as *const u8 as *const libc::c_char,
            val: 0x6495edff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cornsilk\0" as *const u8 as *const libc::c_char,
            val: 0xfff8dcff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"crimson\0" as *const u8 as *const libc::c_char,
            val: 0xdc143cff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cyan\0" as *const u8 as *const libc::c_char,
            val: 0xffffff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkblue\0" as *const u8 as *const libc::c_char,
            val: 0x8bff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkcyan\0" as *const u8 as *const libc::c_char,
            val: 0x8b8bff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgoldenrod\0" as *const u8 as *const libc::c_char,
            val: 0xb8860bff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgray\0" as *const u8 as *const libc::c_char,
            val: 0xa9a9a9ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgreen\0" as *const u8 as *const libc::c_char,
            val: 0x6400ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgrey\0" as *const u8 as *const libc::c_char,
            val: 0xa9a9a9ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkkhaki\0" as *const u8 as *const libc::c_char,
            val: 0xbdb76bff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkmagenta\0" as *const u8 as *const libc::c_char,
            val: 0x8b008bff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkolivegreen\0" as *const u8 as *const libc::c_char,
            val: 0x556b2fff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkorange\0" as *const u8 as *const libc::c_char,
            val: 0xff8c00ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkorchid\0" as *const u8 as *const libc::c_char,
            val: 0x9932ccff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkred\0" as *const u8 as *const libc::c_char,
            val: 0x8b0000ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darksalmon\0" as *const u8 as *const libc::c_char,
            val: 0xe9967aff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkseagreen\0" as *const u8 as *const libc::c_char,
            val: 0x8fbc8fff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslateblue\0" as *const u8 as *const libc::c_char,
            val: 0x483d8bff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslategray\0" as *const u8 as *const libc::c_char,
            val: 0x2f4f4fff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslategrey\0" as *const u8 as *const libc::c_char,
            val: 0x2f4f4fff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkturquoise\0" as *const u8 as *const libc::c_char,
            val: 0xced1ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkviolet\0" as *const u8 as *const libc::c_char,
            val: 0x9400d3ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"deeppink\0" as *const u8 as *const libc::c_char,
            val: 0xff1493ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"deepskyblue\0" as *const u8 as *const libc::c_char,
            val: 0xbfffff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dimgray\0" as *const u8 as *const libc::c_char,
            val: 0x696969ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dimgrey\0" as *const u8 as *const libc::c_char,
            val: 0x696969ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dodgerblue\0" as *const u8 as *const libc::c_char,
            val: 0x1e90ffff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"firebrick\0" as *const u8 as *const libc::c_char,
            val: 0xb22222ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"floralwhite\0" as *const u8 as *const libc::c_char,
            val: 0xfffaf0ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"forestgreen\0" as *const u8 as *const libc::c_char,
            val: 0x228b22ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"fuchsia\0" as *const u8 as *const libc::c_char,
            val: 0xff00ffff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gainsboro\0" as *const u8 as *const libc::c_char,
            val: 0xdcdcdcff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"ghostwhite\0" as *const u8 as *const libc::c_char,
            val: 0xf8f8ffff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gold\0" as *const u8 as *const libc::c_char,
            val: 0xffd700ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"goldenrod\0" as *const u8 as *const libc::c_char,
            val: 0xdaa520ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gray\0" as *const u8 as *const libc::c_char,
            val: 0x808080ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"green\0" as *const u8 as *const libc::c_char,
            val: 0x8000ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"greenyellow\0" as *const u8 as *const libc::c_char,
            val: 0xadff2fff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"grey\0" as *const u8 as *const libc::c_char,
            val: 0x808080ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"honeydew\0" as *const u8 as *const libc::c_char,
            val: 0xf0fff0ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"hotpink\0" as *const u8 as *const libc::c_char,
            val: 0xff69b4ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"indianred\0" as *const u8 as *const libc::c_char,
            val: 0xcd5c5cff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"indigo\0" as *const u8 as *const libc::c_char,
            val: 0x4b0082ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"ivory\0" as *const u8 as *const libc::c_char,
            val: 0xfffff0ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"khaki\0" as *const u8 as *const libc::c_char,
            val: 0xf0e68cff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lavender\0" as *const u8 as *const libc::c_char,
            val: 0xe6e6faff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lavenderblush\0" as *const u8 as *const libc::c_char,
            val: 0xfff0f5ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lawngreen\0" as *const u8 as *const libc::c_char,
            val: 0x7cfc00ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lemonchiffon\0" as *const u8 as *const libc::c_char,
            val: 0xfffacdff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightblue\0" as *const u8 as *const libc::c_char,
            val: 0xadd8e6ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightcoral\0" as *const u8 as *const libc::c_char,
            val: 0xf08080ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightcyan\0" as *const u8 as *const libc::c_char,
            val: 0xe0ffffff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgoldenrodyellow\0" as *const u8 as *const libc::c_char,
            val: 0xfafad2ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgray\0" as *const u8 as *const libc::c_char,
            val: 0xd3d3d3ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgreen\0" as *const u8 as *const libc::c_char,
            val: 0x90ee90ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgrey\0" as *const u8 as *const libc::c_char,
            val: 0xd3d3d3ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightpink\0" as *const u8 as *const libc::c_char,
            val: 0xffb6c1ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightsalmon\0" as *const u8 as *const libc::c_char,
            val: 0xffa07aff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightseagreen\0" as *const u8 as *const libc::c_char,
            val: 0x20b2aaff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightskyblue\0" as *const u8 as *const libc::c_char,
            val: 0x87cefaff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightslategray\0" as *const u8 as *const libc::c_char,
            val: 0x778899ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightslategrey\0" as *const u8 as *const libc::c_char,
            val: 0x778899ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightsteelblue\0" as *const u8 as *const libc::c_char,
            val: 0xb0c4deff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightyellow\0" as *const u8 as *const libc::c_char,
            val: 0xffffe0ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lime\0" as *const u8 as *const libc::c_char,
            val: 0xff00ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"limegreen\0" as *const u8 as *const libc::c_char,
            val: 0x32cd32ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"linen\0" as *const u8 as *const libc::c_char,
            val: 0xfaf0e6ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"magenta\0" as *const u8 as *const libc::c_char,
            val: 0xff00ffff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"maroon\0" as *const u8 as *const libc::c_char,
            val: 0x800000ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumaquamarine\0" as *const u8 as *const libc::c_char,
            val: 0x66cdaaff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumblue\0" as *const u8 as *const libc::c_char,
            val: 0xcdff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumorchid\0" as *const u8 as *const libc::c_char,
            val: 0xba55d3ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumpurple\0" as *const u8 as *const libc::c_char,
            val: 0x9370dbff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumseagreen\0" as *const u8 as *const libc::c_char,
            val: 0x3cb371ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumslateblue\0" as *const u8 as *const libc::c_char,
            val: 0x7b68eeff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumspringgreen\0" as *const u8 as *const libc::c_char,
            val: 0xfa9aff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumturquoise\0" as *const u8 as *const libc::c_char,
            val: 0x48d1ccff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumvioletred\0" as *const u8 as *const libc::c_char,
            val: 0xc71585ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"midnightblue\0" as *const u8 as *const libc::c_char,
            val: 0x191970ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mintcream\0" as *const u8 as *const libc::c_char,
            val: 0xf5fffaff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mistyrose\0" as *const u8 as *const libc::c_char,
            val: 0xffe4e1ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"moccasin\0" as *const u8 as *const libc::c_char,
            val: 0xffe4b5ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"navajowhite\0" as *const u8 as *const libc::c_char,
            val: 0xffdeadff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"navy\0" as *const u8 as *const libc::c_char,
            val: 0x80ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"oldlace\0" as *const u8 as *const libc::c_char,
            val: 0xfdf5e6ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"olive\0" as *const u8 as *const libc::c_char,
            val: 0x808000ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"olivedrab\0" as *const u8 as *const libc::c_char,
            val: 0x6b8e23ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orange\0" as *const u8 as *const libc::c_char,
            val: 0xffa500ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orangered\0" as *const u8 as *const libc::c_char,
            val: 0xff4500ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orchid\0" as *const u8 as *const libc::c_char,
            val: 0xda70d6ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palegoldenrod\0" as *const u8 as *const libc::c_char,
            val: 0xeee8aaff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palegreen\0" as *const u8 as *const libc::c_char,
            val: 0x98fb98ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"paleturquoise\0" as *const u8 as *const libc::c_char,
            val: 0xafeeeeff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palevioletred\0" as *const u8 as *const libc::c_char,
            val: 0xdb7093ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"papayawhip\0" as *const u8 as *const libc::c_char,
            val: 0xffefd5ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"peachpuff\0" as *const u8 as *const libc::c_char,
            val: 0xffdab9ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"peru\0" as *const u8 as *const libc::c_char,
            val: 0xcd853fff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"pink\0" as *const u8 as *const libc::c_char,
            val: 0xffc0cbff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"plum\0" as *const u8 as *const libc::c_char,
            val: 0xdda0ddff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"powderblue\0" as *const u8 as *const libc::c_char,
            val: 0xb0e0e6ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"purple\0" as *const u8 as *const libc::c_char,
            val: 0x800080ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"red\0" as *const u8 as *const libc::c_char,
            val: 0xff0000ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"rosybrown\0" as *const u8 as *const libc::c_char,
            val: 0xbc8f8fff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"royalblue\0" as *const u8 as *const libc::c_char,
            val: 0x4169e1ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"saddlebrown\0" as *const u8 as *const libc::c_char,
            val: 0x8b4513ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"salmon\0" as *const u8 as *const libc::c_char,
            val: 0xfa8072ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"sandybrown\0" as *const u8 as *const libc::c_char,
            val: 0xf4a460ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"seagreen\0" as *const u8 as *const libc::c_char,
            val: 0x2e8b57ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"seashell\0" as *const u8 as *const libc::c_char,
            val: 0xfff5eeff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"sienna\0" as *const u8 as *const libc::c_char,
            val: 0xa0522dff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"silver\0" as *const u8 as *const libc::c_char,
            val: 0xc0c0c0ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"skyblue\0" as *const u8 as *const libc::c_char,
            val: 0x87ceebff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slateblue\0" as *const u8 as *const libc::c_char,
            val: 0x6a5acdff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slategray\0" as *const u8 as *const libc::c_char,
            val: 0x708090ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slategrey\0" as *const u8 as *const libc::c_char,
            val: 0x708090ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"snow\0" as *const u8 as *const libc::c_char,
            val: 0xfffafaff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"springgreen\0" as *const u8 as *const libc::c_char,
            val: 0xff7fff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"steelblue\0" as *const u8 as *const libc::c_char,
            val: 0x4682b4ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"tan\0" as *const u8 as *const libc::c_char,
            val: 0xd2b48cff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"teal\0" as *const u8 as *const libc::c_char,
            val: 0x8080ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"thistle\0" as *const u8 as *const libc::c_char,
            val: 0xd8bfd8ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"tomato\0" as *const u8 as *const libc::c_char,
            val: 0xff6347ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"turquoise\0" as *const u8 as *const libc::c_char,
            val: 0x40e0d0ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"violet\0" as *const u8 as *const libc::c_char,
            val: 0xee82eeff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"wheat\0" as *const u8 as *const libc::c_char,
            val: 0xf5deb3ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"white\0" as *const u8 as *const libc::c_char,
            val: 0xffffffff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"whitesmoke\0" as *const u8 as *const libc::c_char,
            val: 0xf5f5f5ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"yellow\0" as *const u8 as *const libc::c_char,
            val: 0xffff00ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"yellowgreen\0" as *const u8 as *const libc::c_char,
            val: 0x9acd32ff as libc::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: 0 as *const libc::c_char,
            val: 0 as libc::c_int as uint32_t,
        };
        init
    },
];
unsafe extern "C" fn h(mut c: libc::c_char) -> libc::c_int {
    match  c as libc::c_int {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            return c as libc::c_int - '0' as i32;
        }
        97 | 98 | 99 | 100 | 101 | 102 => {
            return c as libc::c_int - 'a' as i32 + 10 as libc::c_int;
        }
        65 | 66 | 67 | 68 | 69 | 70 => {
            return c as libc::c_int - 'A' as i32 + 10 as libc::c_int;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rgba_new(mut rgba: uint32_t) -> rgba_t {
    let mut color = rgba_t {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    color.r= (rgba >> 24 as libc::c_int) as libc::c_double
        / 255 as libc::c_int as libc::c_double;
    color.g= (rgba >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_double / 255 as libc::c_int as libc::c_double;
    color.b= (rgba >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_double / 255 as libc::c_int as libc::c_double;
    color.a= (rgba & 0xff as libc::c_int as libc::c_uint) as libc::c_double
        / 255 as libc::c_int as libc::c_double;
    return color;
}
#[no_mangle]
pub unsafe extern "C" fn rgba_to_string(
    mut rgba: rgba_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) {
    if 1 as libc::c_int as libc::c_double == rgba.a {
        snprintf(
            buf,
            len,
            b"#%.2x%.2x%.2x\0" as *const u8 as *const libc::c_char,
            (rgba.r * 255 as libc::c_int as libc::c_double) as libc::c_int,
            (rgba.g * 255 as libc::c_int as libc::c_double) as libc::c_int,
            (rgba.b * 255 as libc::c_int as libc::c_double) as libc::c_int,
        );
    } else {
        snprintf(
            buf,
            len,
            b"rgba(%d, %d, %d, %.2f)\0" as *const u8 as *const libc::c_char,
            (rgba.r * 255 as libc::c_int as libc::c_double) as libc::c_int,
            (rgba.g * 255 as libc::c_int as libc::c_double) as libc::c_int,
            (rgba.b * 255 as libc::c_int as libc::c_double) as libc::c_int,
            rgba.a,
        );
    };
}
#[inline]
unsafe extern "C" fn rgba_from_rgba(
    mut r: uint8_t,
    mut g: uint8_t,
    mut b: uint8_t,
    mut a: uint8_t,
) -> uint32_t {
    return ((r as libc::c_int) << 24 as libc::c_int
        | (g as libc::c_int) << 16 as libc::c_int
        | (b as libc::c_int) << 8 as libc::c_int | a as libc::c_int) as uint32_t;
}
unsafe extern "C" fn rgba_from_rgb(
    mut r: uint8_t,
    mut g: uint8_t,
    mut b: uint8_t,
) -> int32_t {
    return rgba_from_rgba(r, g, b, 255 as libc::c_int as uint8_t) as int32_t;
}
unsafe extern "C" fn rgba_from_hex6_string(mut str: *const libc::c_char) -> uint32_t {
    return rgba_from_rgb(
        ((h(*str.offset(0 as libc::c_int as isize)) << 4 as libc::c_int)
            + h(*str.offset(1 as libc::c_int as isize))) as uint8_t,
        ((h(*str.offset(2 as libc::c_int as isize)) << 4 as libc::c_int)
            + h(*str.offset(3 as libc::c_int as isize))) as uint8_t,
        ((h(*str.offset(4 as libc::c_int as isize)) << 4 as libc::c_int)
            + h(*str.offset(5 as libc::c_int as isize))) as uint8_t,
    ) as uint32_t;
}
unsafe extern "C" fn rgba_from_hex3_string(mut str: *const libc::c_char) -> int32_t {
    return rgba_from_rgb(
        ((h(*str.offset(0 as libc::c_int as isize)) << 4 as libc::c_int)
            + h(*str.offset(0 as libc::c_int as isize))) as uint8_t,
        ((h(*str.offset(1 as libc::c_int as isize)) << 4 as libc::c_int)
            + h(*str.offset(1 as libc::c_int as isize))) as uint8_t,
        ((h(*str.offset(2 as libc::c_int as isize)) << 4 as libc::c_int)
            + h(*str.offset(2 as libc::c_int as isize))) as uint8_t,
    );
}
unsafe extern "C" fn rgba_from_rgb_string(
    mut str: *const libc::c_char,
    mut ok: Option<&mut libc::c_short>,
) -> int32_t {
    if str
        == strstr(str, b"rgb(\0" as *const u8 as *const libc::c_char)
            as *const libc::c_char
    {
        str= str.offset(4 as libc::c_int as isize);
        while ' ' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        let mut r = 0 as libc::c_int as uint8_t;
        let mut g = 0 as libc::c_int as uint8_t;
        let mut b = 0 as libc::c_int as uint8_t;
        let mut c: libc::c_int = 0;
        c= 0 as libc::c_int;
        if (*str) as libc::c_int >= '0' as i32 && (*str) as libc::c_int <= '9' as i32 {
            loop {
                c*= 10 as libc::c_int;
                let fresh0 = str;
                str= str.offset(1);
                c+= (*fresh0) as libc::c_int - '0' as i32;
                if !((*str) as libc::c_int >= '0' as i32
                    && (*str) as libc::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as libc::c_int
        }
        if c > 255 as libc::c_int {
            c= 255 as libc::c_int;
        }
        r= c as uint8_t;
        while ' ' as i32 == (*str) as libc::c_int || ',' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        c= 0 as libc::c_int;
        if (*str) as libc::c_int >= '0' as i32 && (*str) as libc::c_int <= '9' as i32 {
            loop {
                c*= 10 as libc::c_int;
                let fresh1 = str;
                str= str.offset(1);
                c+= (*fresh1) as libc::c_int - '0' as i32;
                if !((*str) as libc::c_int >= '0' as i32
                    && (*str) as libc::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as libc::c_int
        }
        if c > 255 as libc::c_int {
            c= 255 as libc::c_int;
        }
        g= c as uint8_t;
        while ' ' as i32 == (*str) as libc::c_int || ',' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        c= 0 as libc::c_int;
        if (*str) as libc::c_int >= '0' as i32 && (*str) as libc::c_int <= '9' as i32 {
            loop {
                c*= 10 as libc::c_int;
                let fresh2 = str;
                str= str.offset(1);
                c+= (*fresh2) as libc::c_int - '0' as i32;
                if !((*str) as libc::c_int >= '0' as i32
                    && (*str) as libc::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as libc::c_int
        }
        if c > 255 as libc::c_int {
            c= 255 as libc::c_int;
        }
        b= c as uint8_t;
        while ' ' as i32 == (*str) as libc::c_int || ',' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        *ok.as_deref_mut().unwrap()= 1 as libc::c_int as libc::c_short;
        return rgba_from_rgb(r, g, b);
    }
    *ok.as_deref_mut().unwrap()= 0 as libc::c_int as libc::c_short;
    return (*ok.as_deref().unwrap()) as int32_t;
}
unsafe extern "C" fn rgba_from_rgba_string(
    mut str: *const libc::c_char,
    mut ok: Option<&mut libc::c_short>,
) -> int32_t {
    if str
        == strstr(str, b"rgba(\0" as *const u8 as *const libc::c_char)
            as *const libc::c_char
    {
        str= str.offset(5 as libc::c_int as isize);
        while ' ' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        let mut r = 0 as libc::c_int as uint8_t;
        let mut g = 0 as libc::c_int as uint8_t;
        let mut b = 0 as libc::c_int as uint8_t;
        let mut c: libc::c_int = 0;
        let mut a = 0 as libc::c_int as libc::c_float;
        c= 0 as libc::c_int;
        if (*str) as libc::c_int >= '0' as i32 && (*str) as libc::c_int <= '9' as i32 {
            loop {
                c*= 10 as libc::c_int;
                let fresh3 = str;
                str= str.offset(1);
                c+= (*fresh3) as libc::c_int - '0' as i32;
                if !((*str) as libc::c_int >= '0' as i32
                    && (*str) as libc::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as libc::c_int
        }
        if c > 255 as libc::c_int {
            c= 255 as libc::c_int;
        }
        r= c as uint8_t;
        while ' ' as i32 == (*str) as libc::c_int || ',' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        c= 0 as libc::c_int;
        if (*str) as libc::c_int >= '0' as i32 && (*str) as libc::c_int <= '9' as i32 {
            loop {
                c*= 10 as libc::c_int;
                let fresh4 = str;
                str= str.offset(1);
                c+= (*fresh4) as libc::c_int - '0' as i32;
                if !((*str) as libc::c_int >= '0' as i32
                    && (*str) as libc::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as libc::c_int
        }
        if c > 255 as libc::c_int {
            c= 255 as libc::c_int;
        }
        g= c as uint8_t;
        while ' ' as i32 == (*str) as libc::c_int || ',' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        c= 0 as libc::c_int;
        if (*str) as libc::c_int >= '0' as i32 && (*str) as libc::c_int <= '9' as i32 {
            loop {
                c*= 10 as libc::c_int;
                let fresh5 = str;
                str= str.offset(1);
                c+= (*fresh5) as libc::c_int - '0' as i32;
                if !((*str) as libc::c_int >= '0' as i32
                    && (*str) as libc::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as libc::c_int
        }
        if c > 255 as libc::c_int {
            c= 255 as libc::c_int;
        }
        b= c as uint8_t;
        while ' ' as i32 == (*str) as libc::c_int || ',' as i32 == (*str) as libc::c_int {
            str= str.offset(1);
        }
        if (*str) as libc::c_int >= '1' as i32 && (*str) as libc::c_int <= '9' as i32 {
            a= 1 as libc::c_int as libc::c_float;
        } else {
            if '0' as i32 == (*str) as libc::c_int {
                str= str.offset(1);
            }
            if '.' as i32 == (*str) as libc::c_int {
                str= str.offset(1);
                let mut n = 0.1f64 as libc::c_float;
                while (*str) as libc::c_int >= '0' as i32
                    && (*str) as libc::c_int <= '9' as i32
                {
                    let fresh6 = str;
                    str= str.offset(1);
                    a+= ((*fresh6) as libc::c_int - '0' as i32) as libc::c_float * n;
                    n= (n as libc::c_double * 0.1f64) as libc::c_float;
                }
            }
        }
        *ok.as_deref_mut().unwrap()= 1 as libc::c_int as libc::c_short;
        return rgba_from_rgba(
            r,
            g,
            b,
            (a * 255 as libc::c_int as libc::c_float) as libc::c_int as uint8_t,
        ) as int32_t;
    }
    *ok.as_deref_mut().unwrap()= 0 as libc::c_int as libc::c_short;
    return (*ok.as_deref().unwrap()) as int32_t;
}
unsafe extern "C" fn rgba_from_hex_string(
    mut str: *const libc::c_char,
    mut ok: Option<&mut libc::c_short>,
) -> int32_t {
    let mut len = strlen(str);
    *ok.as_deref_mut().unwrap()= 1 as libc::c_int as libc::c_short;
    if 6 as libc::c_int as libc::c_ulong == len {
        return rgba_from_hex6_string(str) as int32_t;
    }
    if 3 as libc::c_int as libc::c_ulong == len {
        return rgba_from_hex3_string(str);
    }
    *ok.as_deref_mut().unwrap()= 0 as libc::c_int as libc::c_short;
    return (*ok.as_deref().unwrap()) as int32_t;
}
unsafe extern "C" fn rgba_from_name_string(
    mut str: *const libc::c_char,
    mut ok: Option<&mut libc::c_short>,
) -> int32_t {
    let mut i = 0 as libc::c_int;
    let mut color = named_color {
        name: 0 as *const libc::c_char,
        val: 0,
    };
    loop {
        let fresh7 = i;
        i= i + 1;
        color= crate::src::src::rgba::named_colors[fresh7 as usize];
        if color.name.is_null() {();
            break;
        }
        if (*str) as libc::c_int == (*color.name) as libc::c_int
            && 0 as libc::c_int == strcmp(str, color.name)
        {
            *ok.as_deref_mut().unwrap()= 1 as libc::c_int as libc::c_short;
            return color.val as int32_t;
        }
    }
    *ok.as_deref_mut().unwrap()= 0 as libc::c_int as libc::c_short;
    return (*ok.as_deref().unwrap()) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn rgba_from_string(
    mut str: *const libc::c_char,
    mut ok: Option<&mut libc::c_short>,
) -> uint32_t {
    if '#' as i32 == *str.offset(0 as libc::c_int as isize) as libc::c_int {
        str= str.offset(1);
        return rgba_from_hex_string(str, ok.as_deref_mut()) as uint32_t;
    }
    if str
        == strstr(str, b"rgba\0" as *const u8 as *const libc::c_char)
            as *const libc::c_char
    {
        return rgba_from_rgba_string(str, ok.as_deref_mut()) as uint32_t;
    }
    if str
        == strstr(str, b"rgb\0" as *const u8 as *const libc::c_char)
            as *const libc::c_char
    {
        return rgba_from_rgb_string(str, ok.as_deref_mut()) as uint32_t;
    }
    return rgba_from_name_string(str, ok.as_deref_mut()) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn rgba_inspect(mut rgba: uint32_t) {
    printf(
        b"rgba(%d,%d,%d,%d)\n\0" as *const u8 as *const libc::c_char,
        rgba >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
        rgba >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
        rgba >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
        rgba & 0xff as libc::c_int as libc::c_uint,
    );
}
