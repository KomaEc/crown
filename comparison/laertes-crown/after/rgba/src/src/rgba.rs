
extern "C" {
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn snprintf(
        _: * mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: * const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strstr(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut std::os::raw::c_char;
    fn strlen(_: * const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint8_t = std::os::raw::c_uchar;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type int32_t = std::os::raw::c_int;
pub type uint8_t = std::os::raw::c_uchar;
pub type uint32_t = std::os::raw::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgba_t {
    pub r: std::os::raw::c_double,
    pub g: std::os::raw::c_double,
    pub b: std::os::raw::c_double,
    pub a: std::os::raw::c_double,
}
impl rgba_t {
    pub const fn new() -> Self {
        rgba_t {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0
        }
    }
}

impl std::default::Default for rgba_t {
    fn default() -> Self { rgba_t::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_color {
    pub name: * const std::os::raw::c_char,
    pub val: std::os::raw::c_uint,
}
impl named_color {
    pub const fn new() -> Self {
        named_color {
        name: (0 as * const std::os::raw::c_char),
        val: 0
        }
    }
}

impl std::default::Default for named_color {
    fn default() -> Self { named_color::new() }
}

static mut named_colors: [crate::src::src::rgba::named_color; 149] = [crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),crate::src::src::rgba::named_color::new(),]; unsafe fn laertes_init_named_colors() {
named_colors = [
    {
        let mut init = named_color {
            name: b"transparent\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffffff00 as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aliceblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf0f8ffff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"antiquewhite\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfaebd7ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aqua\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffffff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"aquamarine\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x7fffd4ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"azure\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf0ffffff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"beige\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf5f5dcff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"bisque\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffe4c4ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"black\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blanchedalmond\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffebcdff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"blueviolet\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8a2be2ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"brown\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xa52a2aff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"burlywood\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xdeb887ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cadetblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x5f9ea0ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"chartreuse\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x7fff00ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"chocolate\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xd2691eff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"coral\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff7f50ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cornflowerblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x6495edff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cornsilk\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfff8dcff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"crimson\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xdc143cff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"cyan\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffffff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8bff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkcyan\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8b8bff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgoldenrod\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xb8860bff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgray\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xa9a9a9ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x6400ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkgrey\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xa9a9a9ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkkhaki\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xbdb76bff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkmagenta\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8b008bff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkolivegreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x556b2fff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkorange\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff8c00ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkorchid\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x9932ccff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkred\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8b0000ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darksalmon\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xe9967aff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkseagreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8fbc8fff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslateblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x483d8bff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslategray\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x2f4f4fff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkslategrey\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x2f4f4fff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkturquoise\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xced1ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"darkviolet\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x9400d3ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"deeppink\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff1493ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"deepskyblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xbfffff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dimgray\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x696969ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dimgrey\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x696969ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"dodgerblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x1e90ffff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"firebrick\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xb22222ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"floralwhite\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfffaf0ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"forestgreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x228b22ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"fuchsia\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff00ffff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gainsboro\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xdcdcdcff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"ghostwhite\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf8f8ffff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gold\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffd700ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"goldenrod\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xdaa520ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"gray\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x808080ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"green\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8000ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"greenyellow\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xadff2fff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"grey\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x808080ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"honeydew\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf0fff0ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"hotpink\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff69b4ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"indianred\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xcd5c5cff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"indigo\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x4b0082ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"ivory\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfffff0ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"khaki\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf0e68cff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lavender\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xe6e6faff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lavenderblush\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfff0f5ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lawngreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x7cfc00ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lemonchiffon\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfffacdff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xadd8e6ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightcoral\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf08080ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightcyan\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xe0ffffff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgoldenrodyellow\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfafad2ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgray\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xd3d3d3ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x90ee90ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightgrey\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xd3d3d3ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightpink\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffb6c1ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightsalmon\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffa07aff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightseagreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x20b2aaff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightskyblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x87cefaff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightslategray\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x778899ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightslategrey\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x778899ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightsteelblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xb0c4deff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lightyellow\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffffe0ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"lime\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff00ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"limegreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x32cd32ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"linen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfaf0e6ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"magenta\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff00ffff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"maroon\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x800000ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumaquamarine\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x66cdaaff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xcdff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumorchid\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xba55d3ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumpurple\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x9370dbff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumseagreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x3cb371ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumslateblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x7b68eeff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumspringgreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfa9aff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumturquoise\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x48d1ccff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mediumvioletred\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xc71585ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"midnightblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x191970ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mintcream\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf5fffaff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"mistyrose\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffe4e1ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"moccasin\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffe4b5ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"navajowhite\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffdeadff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"navy\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x80ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"oldlace\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfdf5e6ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"olive\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x808000ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"olivedrab\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x6b8e23ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orange\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffa500ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orangered\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff4500ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"orchid\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xda70d6ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palegoldenrod\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xeee8aaff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palegreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x98fb98ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"paleturquoise\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xafeeeeff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"palevioletred\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xdb7093ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"papayawhip\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffefd5ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"peachpuff\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffdab9ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"peru\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xcd853fff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"pink\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffc0cbff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"plum\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xdda0ddff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"powderblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xb0e0e6ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"purple\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x800080ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"red\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff0000ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"rosybrown\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xbc8f8fff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"royalblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x4169e1ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"saddlebrown\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8b4513ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"salmon\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfa8072ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"sandybrown\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf4a460ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"seagreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x2e8b57ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"seashell\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfff5eeff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"sienna\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xa0522dff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"silver\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xc0c0c0ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"skyblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x87ceebff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slateblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x6a5acdff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slategray\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x708090ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"slategrey\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x708090ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"snow\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xfffafaff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"springgreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff7fff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"steelblue\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x4682b4ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"tan\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xd2b48cff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"teal\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x8080ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"thistle\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xd8bfd8ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"tomato\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xff6347ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"turquoise\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x40e0d0ff as std::os::raw::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"violet\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xee82eeff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"wheat\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf5deb3ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"white\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffffffff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"whitesmoke\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xf5f5f5ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"yellow\0" as *const u8 as *const std::os::raw::c_char,
            val: 0xffff00ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: b"yellowgreen\0" as *const u8 as *const std::os::raw::c_char,
            val: 0x9acd32ff as std::os::raw::c_uint,
        };
        init
    },
    {
        let mut init = named_color {
            name: 0 as *const std::os::raw::c_char,
            val: 0 as std::os::raw::c_int as uint32_t,
        };
        init
    },
];}//;
 extern "C" fn h(mut c: std::os::raw::c_char) -> std::os::raw::c_int {
    match c as std::os::raw::c_int {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            return c as std::os::raw::c_int - '0' as i32;
        }
        97 | 98 | 99 | 100 | 101 | 102 => {
            return c as std::os::raw::c_int - 'a' as i32 + 10 as std::os::raw::c_int;
        }
        65 | 66 | 67 | 68 | 69 | 70 => {
            return c as std::os::raw::c_int - 'A' as i32 + 10 as std::os::raw::c_int;
        }
        _ => {}
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn rgba_new(mut rgba: std::os::raw::c_uint) -> crate::src::src::rgba::rgba_t {
    let mut color = rgba_t {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    color
        .r = (rgba >> 24 as std::os::raw::c_int) as std::os::raw::c_double
        / 255 as std::os::raw::c_int as std::os::raw::c_double;
    color
        .g = (rgba >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_double / 255 as std::os::raw::c_int as std::os::raw::c_double;
    color
        .b = (rgba >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_double / 255 as std::os::raw::c_int as std::os::raw::c_double;
    color
        .a = (rgba & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_double
        / 255 as std::os::raw::c_int as std::os::raw::c_double;
    return color;
}
#[no_mangle]
pub unsafe extern "C" fn rgba_to_string(
    mut rgba: crate::src::src::rgba::rgba_t,
    mut buf: * mut std::os::raw::c_char,
    mut len: std::os::raw::c_ulong,
) {
    if 1 as std::os::raw::c_int as std::os::raw::c_double == rgba.a {
        snprintf(
            buf,
            len,
            b"#%.2x%.2x%.2x\0" as *const u8 as *const std::os::raw::c_char,
            (rgba.r * 255 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int,
            (rgba.g * 255 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int,
            (rgba.b * 255 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int,
        );
    } else {
        snprintf(
            buf,
            len,
            b"rgba(%d, %d, %d, %.2f)\0" as *const u8 as *const std::os::raw::c_char,
            (rgba.r * 255 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int,
            (rgba.g * 255 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int,
            (rgba.b * 255 as std::os::raw::c_int as std::os::raw::c_double) as std::os::raw::c_int,
            rgba.a,
        );
    };
}
#[inline]
 extern "C" fn rgba_from_rgba(
    mut r: std::os::raw::c_uchar,
    mut g: std::os::raw::c_uchar,
    mut b: std::os::raw::c_uchar,
    mut a: std::os::raw::c_uchar,
) -> std::os::raw::c_uint {
    return ((r as std::os::raw::c_int) << 24 as std::os::raw::c_int
        | (g as std::os::raw::c_int) << 16 as std::os::raw::c_int
        | (b as std::os::raw::c_int) << 8 as std::os::raw::c_int | a as std::os::raw::c_int) as uint32_t;
}
 extern "C" fn rgba_from_rgb(
    mut r: std::os::raw::c_uchar,
    mut g: std::os::raw::c_uchar,
    mut b: std::os::raw::c_uchar,
) -> std::os::raw::c_int {
    return rgba_from_rgba(r, g, b, 255 as std::os::raw::c_int as uint8_t) as int32_t;
}
unsafe extern "C" fn rgba_from_hex6_string(mut str: * const std::os::raw::c_char) -> std::os::raw::c_uint {
    return rgba_from_rgb(
        ((h(*str.offset(0 as std::os::raw::c_int as isize)) << 4 as std::os::raw::c_int)
            + h(*str.offset(1 as std::os::raw::c_int as isize))) as uint8_t,
        ((h(*str.offset(2 as std::os::raw::c_int as isize)) << 4 as std::os::raw::c_int)
            + h(*str.offset(3 as std::os::raw::c_int as isize))) as uint8_t,
        ((h(*str.offset(4 as std::os::raw::c_int as isize)) << 4 as std::os::raw::c_int)
            + h(*str.offset(5 as std::os::raw::c_int as isize))) as uint8_t,
    ) as uint32_t;
}
unsafe extern "C" fn rgba_from_hex3_string(mut str: * const std::os::raw::c_char) -> std::os::raw::c_int {
    return rgba_from_rgb(
        ((h(*str.offset(0 as std::os::raw::c_int as isize)) << 4 as std::os::raw::c_int)
            + h(*str.offset(0 as std::os::raw::c_int as isize))) as uint8_t,
        ((h(*str.offset(1 as std::os::raw::c_int as isize)) << 4 as std::os::raw::c_int)
            + h(*str.offset(1 as std::os::raw::c_int as isize))) as uint8_t,
        ((h(*str.offset(2 as std::os::raw::c_int as isize)) << 4 as std::os::raw::c_int)
            + h(*str.offset(2 as std::os::raw::c_int as isize))) as uint8_t,
    );
}
unsafe extern "C" fn rgba_from_rgb_string<'a1>(
    mut str: * const std::os::raw::c_char,
    mut ok: Option<&'a1 mut std::os::raw::c_short>,
) -> std::os::raw::c_int {
    if str
        == strstr(str, b"rgb(\0" as *const u8 as *const std::os::raw::c_char)
            as *const std::os::raw::c_char
    {
        str = str.offset(4 as std::os::raw::c_int as isize);
        while ' ' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        let mut r = 0 as std::os::raw::c_int as uint8_t;
        let mut g = 0 as std::os::raw::c_int as uint8_t;
        let mut b = 0 as std::os::raw::c_int as uint8_t;
        let mut c: i32 = 0;
        c = 0 as std::os::raw::c_int;
        if *str as std::os::raw::c_int >= '0' as i32 && *str as std::os::raw::c_int <= '9' as i32 {
            loop {
                c *= 10 as std::os::raw::c_int;
                let mut fresh0 = str;
                str = str.offset(1);
                c += *fresh0 as std::os::raw::c_int - '0' as i32;
                if !(*str as std::os::raw::c_int >= '0' as i32
                    && *str as std::os::raw::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as std::os::raw::c_int
        }
        if c > 255 as std::os::raw::c_int {
            c = 255 as std::os::raw::c_int;
        }
        r = c as uint8_t;
        while ' ' as i32 == *str as std::os::raw::c_int || ',' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        c = 0 as std::os::raw::c_int;
        if *str as std::os::raw::c_int >= '0' as i32 && *str as std::os::raw::c_int <= '9' as i32 {
            loop {
                c *= 10 as std::os::raw::c_int;
                let mut fresh1 = str;
                str = str.offset(1);
                c += *fresh1 as std::os::raw::c_int - '0' as i32;
                if !(*str as std::os::raw::c_int >= '0' as i32
                    && *str as std::os::raw::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as std::os::raw::c_int
        }
        if c > 255 as std::os::raw::c_int {
            c = 255 as std::os::raw::c_int;
        }
        g = c as uint8_t;
        while ' ' as i32 == *str as std::os::raw::c_int || ',' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        c = 0 as std::os::raw::c_int;
        if *str as std::os::raw::c_int >= '0' as i32 && *str as std::os::raw::c_int <= '9' as i32 {
            loop {
                c *= 10 as std::os::raw::c_int;
                let mut fresh2 = str;
                str = str.offset(1);
                c += *fresh2 as std::os::raw::c_int - '0' as i32;
                if !(*str as std::os::raw::c_int >= '0' as i32
                    && *str as std::os::raw::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as std::os::raw::c_int
        }
        if c > 255 as std::os::raw::c_int {
            c = 255 as std::os::raw::c_int;
        }
        b = c as uint8_t;
        while ' ' as i32 == *str as std::os::raw::c_int || ',' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        *(borrow_mut(&mut ok)).unwrap() = 1 as std::os::raw::c_int as std::os::raw::c_short;
        return rgba_from_rgb(r, g, b);
    }
    *(borrow_mut(&mut ok)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_short;
    return *(borrow(& ok)).unwrap() as int32_t;
}
unsafe extern "C" fn rgba_from_rgba_string<'a1>(
    mut str: * const std::os::raw::c_char,
    mut ok: Option<&'a1 mut std::os::raw::c_short>,
) -> std::os::raw::c_int {
    if str
        == strstr(str, b"rgba(\0" as *const u8 as *const std::os::raw::c_char)
            as *const std::os::raw::c_char
    {
        str = str.offset(5 as std::os::raw::c_int as isize);
        while ' ' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        let mut r = 0 as std::os::raw::c_int as uint8_t;
        let mut g = 0 as std::os::raw::c_int as uint8_t;
        let mut b = 0 as std::os::raw::c_int as uint8_t;
        let mut c: i32 = 0;
        let mut a = 0 as std::os::raw::c_int as std::os::raw::c_float;
        c = 0 as std::os::raw::c_int;
        if *str as std::os::raw::c_int >= '0' as i32 && *str as std::os::raw::c_int <= '9' as i32 {
            loop {
                c *= 10 as std::os::raw::c_int;
                let mut fresh3 = str;
                str = str.offset(1);
                c += *fresh3 as std::os::raw::c_int - '0' as i32;
                if !(*str as std::os::raw::c_int >= '0' as i32
                    && *str as std::os::raw::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as std::os::raw::c_int
        }
        if c > 255 as std::os::raw::c_int {
            c = 255 as std::os::raw::c_int;
        }
        r = c as uint8_t;
        while ' ' as i32 == *str as std::os::raw::c_int || ',' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        c = 0 as std::os::raw::c_int;
        if *str as std::os::raw::c_int >= '0' as i32 && *str as std::os::raw::c_int <= '9' as i32 {
            loop {
                c *= 10 as std::os::raw::c_int;
                let mut fresh4 = str;
                str = str.offset(1);
                c += *fresh4 as std::os::raw::c_int - '0' as i32;
                if !(*str as std::os::raw::c_int >= '0' as i32
                    && *str as std::os::raw::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as std::os::raw::c_int
        }
        if c > 255 as std::os::raw::c_int {
            c = 255 as std::os::raw::c_int;
        }
        g = c as uint8_t;
        while ' ' as i32 == *str as std::os::raw::c_int || ',' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        c = 0 as std::os::raw::c_int;
        if *str as std::os::raw::c_int >= '0' as i32 && *str as std::os::raw::c_int <= '9' as i32 {
            loop {
                c *= 10 as std::os::raw::c_int;
                let mut fresh5 = str;
                str = str.offset(1);
                c += *fresh5 as std::os::raw::c_int - '0' as i32;
                if !(*str as std::os::raw::c_int >= '0' as i32
                    && *str as std::os::raw::c_int <= '9' as i32)
                {
                    break;
                }
            }
        } else {
            return 0 as std::os::raw::c_int
        }
        if c > 255 as std::os::raw::c_int {
            c = 255 as std::os::raw::c_int;
        }
        b = c as uint8_t;
        while ' ' as i32 == *str as std::os::raw::c_int || ',' as i32 == *str as std::os::raw::c_int {
            str = str.offset(1);
        }
        if *str as std::os::raw::c_int >= '1' as i32 && *str as std::os::raw::c_int <= '9' as i32 {
            a = 1 as std::os::raw::c_int as std::os::raw::c_float;
        } else {
            if '0' as i32 == *str as std::os::raw::c_int {
                str = str.offset(1);
            }
            if '.' as i32 == *str as std::os::raw::c_int {
                str = str.offset(1);
                let mut n = 0.1f64 as std::os::raw::c_float;
                while *str as std::os::raw::c_int >= '0' as i32
                    && *str as std::os::raw::c_int <= '9' as i32
                {
                    let mut fresh6 = str;
                    str = str.offset(1);
                    a += (*fresh6 as std::os::raw::c_int - '0' as i32) as std::os::raw::c_float * n;
                    n = (n as std::os::raw::c_double * 0.1f64) as std::os::raw::c_float;
                }
            }
        }
        *(borrow_mut(&mut ok)).unwrap() = 1 as std::os::raw::c_int as std::os::raw::c_short;
        return rgba_from_rgba(
            r,
            g,
            b,
            (a * 255 as std::os::raw::c_int as std::os::raw::c_float) as std::os::raw::c_int as uint8_t,
        ) as int32_t;
    }
    *(borrow_mut(&mut ok)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_short;
    return *(borrow(& ok)).unwrap() as int32_t;
}
unsafe extern "C" fn rgba_from_hex_string<'a1>(
    mut str: * const std::os::raw::c_char,
    mut ok: Option<&'a1 mut std::os::raw::c_short>,
) -> std::os::raw::c_int {
    let mut len = strlen(str);
    *(borrow_mut(&mut ok)).unwrap() = 1 as std::os::raw::c_int as std::os::raw::c_short;
    if 6 as std::os::raw::c_int as std::os::raw::c_ulong == len {
        return rgba_from_hex6_string(str) as int32_t;
    }
    if 3 as std::os::raw::c_int as std::os::raw::c_ulong == len {
        return rgba_from_hex3_string(str);
    }
    *(borrow_mut(&mut ok)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_short;
    return *(borrow(& ok)).unwrap() as int32_t;
}
unsafe extern "C" fn rgba_from_name_string<'a1>(
    mut str: * const std::os::raw::c_char,
    mut ok: Option<&'a1 mut std::os::raw::c_short>,
) -> std::os::raw::c_int {
    let mut i = 0 as std::os::raw::c_int;
    let mut color = named_color {
        name: 0 as *const std::os::raw::c_char,
        val: 0,
    };
    loop {
        let mut fresh7 = i;
        i = i + 1;
        color = named_colors[fresh7 as usize];
        if (color.name).is_null() {
            break;
        }
        if *str as std::os::raw::c_int == *color.name as std::os::raw::c_int
            && 0 as std::os::raw::c_int == strcmp(str, color.name)
        {
            *(borrow_mut(&mut ok)).unwrap() = 1 as std::os::raw::c_int as std::os::raw::c_short;
            return color.val as int32_t;
        }
    }
    *(borrow_mut(&mut ok)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_short;
    return *(borrow(& ok)).unwrap() as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn rgba_from_string<'a1>(
    mut str: * const std::os::raw::c_char,
    mut ok: Option<&'a1 mut std::os::raw::c_short>,
) -> std::os::raw::c_uint {
    if '#' as i32 == *str.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int {
        str = str.offset(1);
        return rgba_from_hex_string(str, borrow_mut(&mut ok)) as uint32_t;
    }
    if str
        == strstr(str, b"rgba\0" as *const u8 as *const std::os::raw::c_char)
            as *const std::os::raw::c_char
    {
        return rgba_from_rgba_string(str, borrow_mut(&mut ok)) as uint32_t;
    }
    if str
        == strstr(str, b"rgb\0" as *const u8 as *const std::os::raw::c_char)
            as *const std::os::raw::c_char
    {
        return rgba_from_rgb_string(str, borrow_mut(&mut ok)) as uint32_t;
    }
    return rgba_from_name_string(str, borrow_mut(&mut ok)) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn rgba_inspect(mut rgba: std::os::raw::c_uint) {
    printf(
        b"rgba(%d,%d,%d,%d)\n\0" as *const u8 as *const std::os::raw::c_char,
        rgba >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint,
        rgba >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint,
        rgba >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint,
        rgba & 0xff as std::os::raw::c_int as std::os::raw::c_uint,
    );
}
use crate::laertes_rt::*;
