#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, main, register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn sscanf(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url_data {
    pub href: * mut std::os::raw::c_char,
    pub protocol: * mut std::os::raw::c_char,
    pub host: * mut std::os::raw::c_char,
    pub auth: * mut std::os::raw::c_char,
    pub hostname: * mut std::os::raw::c_char,
    pub pathname: * mut std::os::raw::c_char,
    pub search: * mut std::os::raw::c_char,
    pub path: * mut std::os::raw::c_char,
    pub hash: * mut std::os::raw::c_char,
    pub query: * mut std::os::raw::c_char,
    pub port: * mut std::os::raw::c_char,
}
impl std::default::Default for url_data {
    fn default() -> Self {
        url_data {
        href: 0 as * mut std::os::raw::c_char,
        protocol: 0 as * mut std::os::raw::c_char,
        host: 0 as * mut std::os::raw::c_char,
        auth: 0 as * mut std::os::raw::c_char,
        hostname: 0 as * mut std::os::raw::c_char,
        pathname: 0 as * mut std::os::raw::c_char,
        search: 0 as * mut std::os::raw::c_char,
        path: 0 as * mut std::os::raw::c_char,
        hash: 0 as * mut std::os::raw::c_char,
        query: 0 as * mut std::os::raw::c_char,
        port: 0 as * mut std::os::raw::c_char
        }
    }
}

pub type url_data_t = crate::url_data;
#[no_mangle]
pub unsafe extern "C" fn strdup(mut str: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut n: std::os::raw::c_int =
        strlen(str).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    let mut dup: *mut std::os::raw::c_char =
        malloc(n as std::os::raw::c_ulong) as *mut std::os::raw::c_char;
    if !dup.is_null() { strcpy(dup, str); }
    return dup;
}
#[no_mangle]
pub static mut URL_SCHEMES: [*mut std::os::raw::c_char; 177] =
    [b"aaa\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"aaas\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"about\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"acap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"acct\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"adiumxtra\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"afp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"afs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"aim\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"apt\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"attachment\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"aw\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"beshare\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"bitcoin\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"bolo\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"callto\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"cap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"chrome\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"crome-extension\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"com-evenbrite-attendee\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"cid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"coap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"coaps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"content\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"crid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"cvs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"data\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dav\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dict\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"lna-playsingle\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"dln-playcontainer\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"dns\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dtn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"dvb\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ed2k\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"facetime\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"fax\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"feed\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"file\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"finger\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"fish\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ftp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"geo\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gg\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"git\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gizmoproject\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"go\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gopher\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"gtalk\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"h323\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"hcp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"http\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"https\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iax\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"icap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"icon\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"im\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"imap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"info\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ipn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ipp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"irc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"irc6\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ircs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iris\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iris.beep\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"iris.xpc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"iris.xpcs\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"iris.lws\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"itms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"jabber\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"jar\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"jms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"keyparc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"lastfm\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ldap\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ldaps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"magnet\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mailserver\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"mailto\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"maps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"market\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"message\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"modem\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ms-help\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mssettings-power\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"msnim\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"msrp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"msrps\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mtqp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mumble\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mupdate\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"mvn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"news\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"nfs\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ni\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"nih\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"nntp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"notes\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"oid\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"paquelocktoken\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"pack\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"palm\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"paparazzi\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"pkcs11\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"platform\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"pop\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"pres\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"prospero\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"proxy\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"psyc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"query\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"reload\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"res\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"resource\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rmi\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rsync\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rtmp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"rtsp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"secondlife\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"service\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"session\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sftp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sgn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"shttp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sieve\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sip\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sips\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"skype\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"smb\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"sms\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"snews\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"snmp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"soap.beep\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"soap.beeps\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"soldat\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"spotify\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ssh\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"steam\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"svn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tag\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"teamspeak\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"tel\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"telnet\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tftp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"things\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"thismessage\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"tn3270\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tip\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"tv\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"udp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"unreal\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"urn\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ut2004\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"vemmi\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ventrilo\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"videotex\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"view-source\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"wais\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"webcal\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ws\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"wss\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"wtai\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"wyciwyg\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xcon\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xcon-userid\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"xfire\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xmlrpc.beep\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"xmlrpc.beeps\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"xmpp\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"xri\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"ymsgr\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"javascript\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"jdbc\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"doi\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn url_parse(mut url: *mut std::os::raw::c_char)
 -> *mut url_data_t {
    let mut data: *mut url_data_t =
        malloc(::std::mem::size_of::<url_data_t>() as std::os::raw::c_ulong) as
            *mut url_data_t;
    if data.is_null() { return core::ptr::null_mut() }
    (*data).href = url;
    let mut tmp: *mut std::os::raw::c_char = core::ptr::null_mut();
    let mut tmp_url: *mut std::os::raw::c_char = strdup(url);
    let mut is_ssh: bool = 0 as std::os::raw::c_int != 0;
    let mut protocol: *mut std::os::raw::c_char = url_get_protocol(tmp_url);
    if protocol.is_null() { return core::ptr::null_mut() }
    let mut protocol_len: std::os::raw::c_int =
        strlen(protocol) as std::os::raw::c_int + 3 as std::os::raw::c_int;
    (*data).protocol = protocol;
    is_ssh = url_is_ssh(protocol);
    let mut auth: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut auth_len: std::os::raw::c_int = 0 as std::os::raw::c_int;
    tmp = strstr(tmp_url, b"@\x00" as *const u8 as *const std::os::raw::c_char);
    if !tmp.is_null() {
        auth =
            get_part(tmp_url,
                     b"%[^@]\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len);
        auth_len = strlen(auth) as std::os::raw::c_int;
        if !auth.is_null() { auth_len += 1 }
    }
    (*data).auth = auth;
    let mut hostname: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    hostname =
        if is_ssh as std::os::raw::c_int != 0 {
            get_part(tmp_url,
                     b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len)
        } else {
            get_part(tmp_url,
                     b"%[^/]\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len)
        };
    if hostname.is_null() { return core::ptr::null_mut() }
    let mut hostname_len: std::os::raw::c_int = strlen(hostname) as std::os::raw::c_int;
    let mut tmp_hostname: *mut std::os::raw::c_char = strdup(hostname);
    (*data).hostname = hostname;
    let mut host: *mut std::os::raw::c_char =
        malloc(strlen(tmp_hostname).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                     as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    sscanf(tmp_hostname, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char,
           host);
    if host.is_null() { return core::ptr::null_mut() }
    let mut host_len: std::os::raw::c_int = strlen(host) as std::os::raw::c_int;
    (*data).host = host;
    let mut tmp_path: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    tmp_path =
        if is_ssh as std::os::raw::c_int != 0 {
            get_part(tmp_url, b":%s\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len + hostname_len)
        } else {
            get_part(tmp_url, b"/%s\x00" as *const u8 as *const std::os::raw::c_char,
                     protocol_len + auth_len + hostname_len)
        };
    let mut path: *mut std::os::raw::c_char =
        malloc(strlen(tmp_path).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                 as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if path.is_null() { return core::ptr::null_mut() }
    let mut fmt: *mut std::os::raw::c_char =
        if is_ssh as std::os::raw::c_int != 0 {
            b"%s\x00" as *const u8 as *const std::os::raw::c_char
        } else { b"/%s\x00" as *const u8 as *const std::os::raw::c_char } as
            *mut std::os::raw::c_char;
    sprintf(path, fmt, tmp_path);
    (*data).path = path;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut pathname: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if pathname.is_null() { return core::ptr::null_mut() }
    strcat(pathname, b"\x00" as *const u8 as *const std::os::raw::c_char);
    tmp_path = strdup(path);
    sscanf(tmp_path, b"%[^? | ^#]\x00" as *const u8 as *const std::os::raw::c_char,
           pathname);
    let mut pathname_len: std::os::raw::c_int = strlen(pathname) as std::os::raw::c_int;
    (*data).pathname = pathname;
    let mut search: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<*mut std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if search.is_null() { return core::ptr::null_mut() }
    tmp_path = strff(tmp_path, pathname_len);
    strcat(search, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%[^#]\x00" as *const u8 as *const std::os::raw::c_char,
           search);
    (*data).search = search;
    let mut search_len: std::os::raw::c_int = strlen(search) as std::os::raw::c_int;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut query: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if query.is_null() { return core::ptr::null_mut() }
    sscanf(search, b"?%s\x00" as *const u8 as *const std::os::raw::c_char, query);
    (*data).query = query;
    let mut hash: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if hash.is_null() { return core::ptr::null_mut() }
    tmp_path = strff(path, pathname_len + search_len);
    strcat(hash, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%s\x00" as *const u8 as *const std::os::raw::c_char, hash);
    (*data).hash = hash;
    free(tmp_path as *mut std::os::raw::c_void);
    let mut port: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if port.is_null() { return core::ptr::null_mut() }
    tmp_hostname = strff(hostname, host_len + 1 as std::os::raw::c_int);
    sscanf(tmp_hostname, b"%s\x00" as *const u8 as *const std::os::raw::c_char, port);
    (*data).port = port;
    free(tmp_hostname as *mut std::os::raw::c_void);
    return data;
}
unsafe extern "C" fn strff(mut ptr: *mut std::os::raw::c_char, mut n: std::os::raw::c_int)
 -> *mut std::os::raw::c_char {
    let mut y: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < n {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        y = *fresh0 as std::os::raw::c_int;
        i += 1
    }
    return strdup(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn url_get_protocol(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut protocol: *mut std::os::raw::c_char =
        malloc((16 as std::os::raw::c_int as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                    as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    if protocol.is_null() { return 0 as *mut std::os::raw::c_char }
    sscanf(url, b"%[^://]\x00" as *const u8 as *const std::os::raw::c_char, protocol);
    if url_is_protocol(protocol) { return protocol }
    return 0 as *mut std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn url_is_protocol(mut str: *mut std::os::raw::c_char) -> bool {
    let mut count: std::os::raw::c_int =
        (::std::mem::size_of::<[*mut std::os::raw::c_char; 177]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*mut std::os::raw::c_char>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < count {
        if 0 as std::os::raw::c_int == strcmp(URL_SCHEMES[i as usize], str) {
            return 1 as std::os::raw::c_int != 0
        }
        i += 1
    }
    return 0 as std::os::raw::c_int != 0;
}
unsafe extern "C" fn get_part(mut url: *mut std::os::raw::c_char,
                              mut format: *const std::os::raw::c_char,
                              mut l: std::os::raw::c_int) -> *mut std::os::raw::c_char {
    let mut has: bool = 0 as std::os::raw::c_int != 0;
    let mut tmp: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut tmp_url: *mut std::os::raw::c_char = strdup(url);
    let mut fmt_url: *mut std::os::raw::c_char = strdup(url);
    let mut ret: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() ||
           ret.is_null() {
        return 0 as *mut std::os::raw::c_char
    }
    strcpy(tmp, b"\x00" as *const u8 as *const std::os::raw::c_char);
    strcpy(fmt_url, b"\x00" as *const u8 as *const std::os::raw::c_char);
    fmt_url = strff(fmt_url, l);
    sscanf(fmt_url, format, tmp);
    if 0 as std::os::raw::c_int != strcmp(tmp, tmp_url) {
        has = 1 as std::os::raw::c_int != 0;
        ret = strdup(tmp)
    }
    fmt_url = strrwd(fmt_url, l);
    free(tmp as *mut std::os::raw::c_void);
    free(tmp_url as *mut std::os::raw::c_void);
    free(fmt_url as *mut std::os::raw::c_void);
    return if has as std::os::raw::c_int != 0 { ret } else { 0 as *mut std::os::raw::c_char };
}
unsafe extern "C" fn strrwd(mut ptr: *mut std::os::raw::c_char, mut n: std::os::raw::c_int)
 -> *mut std::os::raw::c_char {
    let mut y: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i < n {
        let fresh1 = ptr;
        ptr = ptr.offset(-1);
        y = *fresh1 as std::os::raw::c_int;
        i += 1
    }
    return strdup(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn url_is_ssh(mut str: *mut std::os::raw::c_char) -> bool {
    str = strdup(str);
    if 0 as std::os::raw::c_int ==
           strcmp(str, b"ssh\x00" as *const u8 as *const std::os::raw::c_char) ||
           0 as std::os::raw::c_int ==
               strcmp(str, b"git\x00" as *const u8 as *const std::os::raw::c_char) {
        free(str as *mut std::os::raw::c_void);
        return 1 as std::os::raw::c_int != 0
    }
    return 0 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_auth(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut protocol: *mut std::os::raw::c_char = url_get_protocol(url);
    if protocol.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut l: std::os::raw::c_int =
        strlen(protocol) as std::os::raw::c_int + 3 as std::os::raw::c_int;
    return get_part(url, b"%[^@]\x00" as *const u8 as *const std::os::raw::c_char, l);
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hostname(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut l: std::os::raw::c_int = 3 as std::os::raw::c_int;
    let mut protocol: *mut std::os::raw::c_char = url_get_protocol(url);
    let mut tmp_protocol: *mut std::os::raw::c_char = strdup(protocol);
    let mut auth: *mut std::os::raw::c_char = url_get_auth(url);
    if protocol.is_null() { return 0 as *mut std::os::raw::c_char }
    if !auth.is_null() {
        l =
            (l as
                 std::os::raw::c_ulong).wrapping_add(strlen(auth).wrapping_add(1 as
                                                                           std::os::raw::c_int
                                                                           as
                                                                           std::os::raw::c_ulong))
                as std::os::raw::c_int as std::os::raw::c_int
    }
    if !auth.is_null() { free(auth as *mut std::os::raw::c_void); }
    l += strlen(protocol) as std::os::raw::c_int;
    free(protocol as *mut std::os::raw::c_void);
    let mut hostname: *mut std::os::raw::c_char =
        if url_is_ssh(tmp_protocol) as std::os::raw::c_int != 0 {
            get_part(url, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char, l)
        } else {
            get_part(url, b"%[^/]\x00" as *const u8 as *const std::os::raw::c_char, l)
        };
    free(tmp_protocol as *mut std::os::raw::c_void);
    return hostname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_host(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut host: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut hostname: *mut std::os::raw::c_char = url_get_hostname(url);
    if host.is_null() || hostname.is_null() { return 0 as *mut std::os::raw::c_char }
    sscanf(hostname, b"%[^:]\x00" as *const u8 as *const std::os::raw::c_char, host);
    free(hostname as *mut std::os::raw::c_void);
    return host;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_pathname(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut path: *mut std::os::raw::c_char = url_get_path(url);
    let mut pathname: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if path.is_null() || pathname.is_null() { return 0 as *mut std::os::raw::c_char }
    strcat(pathname, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(path, b"%[^?]\x00" as *const u8 as *const std::os::raw::c_char, pathname);
    free(path as *mut std::os::raw::c_void);
    return pathname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_path(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut l: std::os::raw::c_int = 3 as std::os::raw::c_int;
    let mut tmp_path: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut protocol: *mut std::os::raw::c_char = url_get_protocol(url);
    let mut auth: *mut std::os::raw::c_char = url_get_auth(url);
    let mut hostname: *mut std::os::raw::c_char = url_get_hostname(url);
    if protocol.is_null() || hostname.is_null() {
        return 0 as *mut std::os::raw::c_char
    }
    let mut is_ssh: bool = url_is_ssh(protocol);
    l += strlen(protocol) as std::os::raw::c_int + strlen(hostname) as std::os::raw::c_int;
    if !auth.is_null() { l += strlen(auth) as std::os::raw::c_int + 1 as std::os::raw::c_int }
    tmp_path =
        if is_ssh as std::os::raw::c_int != 0 {
            get_part(url, b":%s\x00" as *const u8 as *const std::os::raw::c_char, l)
        } else {
            get_part(url, b"/%s\x00" as *const u8 as *const std::os::raw::c_char, l)
        };
    let mut fmt: *mut std::os::raw::c_char =
        if is_ssh as std::os::raw::c_int != 0 {
            b"%s\x00" as *const u8 as *const std::os::raw::c_char
        } else { b"/%s\x00" as *const u8 as *const std::os::raw::c_char } as
            *mut std::os::raw::c_char;
    let mut path: *mut std::os::raw::c_char =
        malloc(strlen(tmp_path).wrapping_mul(::std::mem::size_of::<std::os::raw::c_char>()
                                                 as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_char;
    sprintf(path, fmt, tmp_path);
    if !auth.is_null() { free(auth as *mut std::os::raw::c_void); }
    free(protocol as *mut std::os::raw::c_void);
    free(hostname as *mut std::os::raw::c_void);
    free(tmp_path as *mut std::os::raw::c_void);
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_search(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut path: *mut std::os::raw::c_char = url_get_path(url);
    let mut pathname: *mut std::os::raw::c_char = url_get_pathname(url);
    let mut search: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if path.is_null() || search.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut tmp_path: *mut std::os::raw::c_char =
        strff(path, strlen(pathname) as std::os::raw::c_int);
    strcat(search, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%[^#]\x00" as *const u8 as *const std::os::raw::c_char,
           search);
    tmp_path = strrwd(tmp_path, strlen(pathname) as std::os::raw::c_int);
    free(path as *mut std::os::raw::c_void);
    free(pathname as *mut std::os::raw::c_void);
    return search;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_query(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut search: *mut std::os::raw::c_char = url_get_search(url);
    let mut query: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if search.is_null() { return 0 as *mut std::os::raw::c_char }
    sscanf(search, b"?%s\x00" as *const u8 as *const std::os::raw::c_char, query);
    free(search as *mut std::os::raw::c_void);
    return query;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hash(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut hash: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if hash.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut path: *mut std::os::raw::c_char = url_get_path(url);
    if path.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut pathname: *mut std::os::raw::c_char = url_get_pathname(url);
    if pathname.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut search: *mut std::os::raw::c_char = url_get_search(url);
    let mut pathname_len: std::os::raw::c_int = strlen(pathname) as std::os::raw::c_int;
    let mut search_len: std::os::raw::c_int = strlen(search) as std::os::raw::c_int;
    let mut tmp_path: *mut std::os::raw::c_char =
        strff(path, pathname_len + search_len);
    strcat(hash, b"\x00" as *const u8 as *const std::os::raw::c_char);
    sscanf(tmp_path, b"%s\x00" as *const u8 as *const std::os::raw::c_char, hash);
    tmp_path = strrwd(tmp_path, pathname_len + search_len);
    free(tmp_path as *mut std::os::raw::c_void);
    free(pathname as *mut std::os::raw::c_void);
    free(path as *mut std::os::raw::c_void);
    if !search.is_null() { free(search as *mut std::os::raw::c_void); }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_port(mut url: *mut std::os::raw::c_char)
 -> *mut std::os::raw::c_char {
    let mut port: *mut std::os::raw::c_char =
        malloc(::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    let mut hostname: *mut std::os::raw::c_char = url_get_hostname(url);
    let mut host: *mut std::os::raw::c_char = url_get_host(url);
    if port.is_null() || hostname.is_null() { return 0 as *mut std::os::raw::c_char }
    let mut tmp_hostname: *mut std::os::raw::c_char =
        strff(hostname,
              strlen(host).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
                  std::os::raw::c_int);
    sscanf(tmp_hostname, b"%s\x00" as *const u8 as *const std::os::raw::c_char, port);
    free(hostname as *mut std::os::raw::c_void);
    free(tmp_hostname as *mut std::os::raw::c_void);
    return port;
}
#[no_mangle]
pub unsafe extern "C" fn url_free(mut data: *mut url_data_t) {
    if data.is_null() { return }
    if !(*data).auth.is_null() { free((*data).auth as *mut std::os::raw::c_void); }
    if !(*data).protocol.is_null() {
        free((*data).protocol as *mut std::os::raw::c_void);
    }
    if !(*data).hostname.is_null() {
        free((*data).hostname as *mut std::os::raw::c_void);
    }
    if !(*data).host.is_null() { free((*data).host as *mut std::os::raw::c_void); }
    if !(*data).pathname.is_null() {
        free((*data).pathname as *mut std::os::raw::c_void);
    }
    if !(*data).path.is_null() { free((*data).path as *mut std::os::raw::c_void); }
    if !(*data).hash.is_null() { free((*data).hash as *mut std::os::raw::c_void); }
    if !(*data).search.is_null() {
        free((*data).search as *mut std::os::raw::c_void);
    }
    if !(*data).query.is_null() { free((*data).query as *mut std::os::raw::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn url_inspect(mut url: *mut std::os::raw::c_char) {
    url_data_inspect(url_parse(url));
}
#[no_mangle]
pub unsafe extern "C" fn url_data_inspect(mut data: *mut url_data_t) {
    printf(b"#url =>\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"    .href: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).href);
    printf(b"    .protocol: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).protocol);
    printf(b"    .host: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).host);
    printf(b"    .auth: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).auth);
    printf(b"    .hostname: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).hostname);
    printf(b"    .pathname: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).pathname);
    printf(b"    .search: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).search);
    printf(b"    .path: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).path);
    printf(b"    .hash: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).hash);
    printf(b"    .query: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).query);
    printf(b"    .port: \"%s\"\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*data).port);
}
unsafe fn main_0() -> std::os::raw::c_int {
    //url_inspect("https://google.com/search?q=github");
    let mut gh_url: *mut std::os::raw::c_char =
        b"git://git@github.com:jwerle/url.h.git\x00" as *const u8 as
            *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut url: *mut std::os::raw::c_char =
        b"http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash\x00"
            as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    let mut parsed: *mut url_data_t = url_parse(url);
    let mut gh_parsed: *mut url_data_t = url_parse(gh_url);
    if parsed.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     15 as std::os::raw::c_int,
                     b"parsed\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if gh_parsed.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     16 as std::os::raw::c_int,
                     b"gh_parsed\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    url_data_inspect(parsed);
    url_data_inspect(gh_parsed);
    if (*parsed).href.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     21 as std::os::raw::c_int,
                     b"parsed->href\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).auth.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     22 as std::os::raw::c_int,
                     b"parsed->auth\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).protocol.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     23 as std::os::raw::c_int,
                     b"parsed->protocol\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).port.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     24 as std::os::raw::c_int,
                     b"parsed->port\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).hostname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     25 as std::os::raw::c_int,
                     b"parsed->hostname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).host.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     26 as std::os::raw::c_int,
                     b"parsed->host\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).pathname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     27 as std::os::raw::c_int,
                     b"parsed->pathname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).path.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     28 as std::os::raw::c_int,
                     b"parsed->path\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).hash.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     29 as std::os::raw::c_int,
                     b"parsed->hash\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    if (*parsed).search.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     30 as std::os::raw::c_int,
                     b"parsed->search\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*parsed).query.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     31 as std::os::raw::c_int,
                     b"parsed->query\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).href.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     33 as std::os::raw::c_int,
                     b"gh_parsed->href\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).protocol.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     34 as std::os::raw::c_int,
                     b"gh_parsed->protocol\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).host.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     35 as std::os::raw::c_int,
                     b"gh_parsed->host\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).auth.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     36 as std::os::raw::c_int,
                     b"gh_parsed->auth\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).hostname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     37 as std::os::raw::c_int,
                     b"gh_parsed->hostname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).pathname.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     38 as std::os::raw::c_int,
                     b"gh_parsed->pathname\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if (*gh_parsed).path.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     39 as std::os::raw::c_int,
                     b"gh_parsed->path\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"http\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     41 as std::os::raw::c_int,
                     b"url_is_protocol(\"http\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"https\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     42 as std::os::raw::c_int,
                     b"url_is_protocol(\"https\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"git\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     43 as std::os::raw::c_int,
                     b"url_is_protocol(\"git\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"ssh\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     44 as std::os::raw::c_int,
                     b"url_is_protocol(\"ssh\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"sftp\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     45 as std::os::raw::c_int,
                     b"url_is_protocol(\"sftp\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"ftp\x00" as *const u8 as *const std::os::raw::c_char as
                            *mut std::os::raw::c_char) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     46 as std::os::raw::c_int,
                     b"url_is_protocol(\"ftp\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !url_is_protocol(b"javascript\x00" as *const u8 as *const std::os::raw::c_char
                            as *mut std::os::raw::c_char) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     47 as std::os::raw::c_int,
                     b"url_is_protocol(\"javascript\")\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"http\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_protocol(url))) as std::os::raw::c_int as std::os::raw::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     49 as std::os::raw::c_int,
                     b"0 == strcmp(\"http\", url_get_protocol(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"user:pass\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_auth(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     50 as std::os::raw::c_int,
                     b"0 == strcmp(\"user:pass\", url_get_auth(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"subdomain.host.com:8080\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_hostname(url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     51 as std::os::raw::c_int,
                     b"0 == strcmp(\"subdomain.host.com:8080\", url_get_hostname(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"subdomain.host.com\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_host(url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     52 as std::os::raw::c_int,
                     b"0 == strcmp(\"subdomain.host.com\", url_get_host(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"/p/a/t/h\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_pathname(url))) as std::os::raw::c_int as std::os::raw::c_long !=
           0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     53 as std::os::raw::c_int,
                     b"0 == strcmp(\"/p/a/t/h\", url_get_pathname(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"/p/a/t/h?query=string#hash\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_path(url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     54 as std::os::raw::c_int,
                     b"0 == strcmp(\"/p/a/t/h?query=string#hash\", url_get_path(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"?query=string\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_search(url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     55 as std::os::raw::c_int,
                     b"0 == strcmp(\"?query=string\", url_get_search(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"query=string\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_query(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     56 as std::os::raw::c_int,
                     b"0 == strcmp(\"query=string\", url_get_query(url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"#hash\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_hash(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     57 as std::os::raw::c_int,
                     b"0 == strcmp(\"#hash\", url_get_hash(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"8080\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_port(url))) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     58 as std::os::raw::c_int,
                     b"0 == strcmp(\"8080\", url_get_port(url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"git\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_protocol(gh_url))) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     60 as std::os::raw::c_int,
                     b"0 == strcmp(\"git\", url_get_protocol(gh_url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"github.com\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_host(gh_url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     61 as std::os::raw::c_int,
                     b"0 == strcmp(\"github.com\", url_get_host(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"github.com\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_hostname(gh_url))) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     62 as std::os::raw::c_int,
                     b"0 == strcmp(\"github.com\", url_get_hostname(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"git\x00" as *const u8 as *const std::os::raw::c_char,
                    url_get_auth(gh_url))) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     63 as std::os::raw::c_int,
                     b"0 == strcmp(\"git\", url_get_auth(gh_url))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"jwerle/url.h.git\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_pathname(gh_url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     64 as std::os::raw::c_int,
                     b"0 == strcmp(\"jwerle/url.h.git\", url_get_pathname(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(0 as std::os::raw::c_int ==
             strcmp(b"jwerle/url.h.git\x00" as *const u8 as
                        *const std::os::raw::c_char, url_get_path(gh_url))) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"test.c\x00" as *const u8 as *const std::os::raw::c_char,
                     65 as std::os::raw::c_int,
                     b"0 == strcmp(\"jwerle/url.h.git\", url_get_path(gh_url))\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    url_free(parsed);
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
pub fn borrow<'a, 'b: 'a, T>(p: &'a Option<&'b mut T>) -> Option<&'a T> {
    p.as_ref().map(|x| &**x)
}

pub fn borrow_mut<'a, 'b : 'a, T>(p: &'a mut Option<&'b mut T>) -> Option<&'a mut T> {
    p.as_mut().map(|x| &mut **x)
}

pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
    p.as_ref().map(|x| x.as_ref())
}

pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
    p.as_mut().map(|x| x.as_mut())
}

pub fn option_to_raw<T>(p: Option<&T>) -> * const T {
    p.map_or(core::ptr::null(), |p| p as * const T)
}

pub fn _ref_eq<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) == option_to_raw(q)
}

pub fn _ref_ne<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) != option_to_raw(q)
}

