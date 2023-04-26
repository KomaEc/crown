use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url_data {
    pub href: *mut libc::c_char,
    pub protocol: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub auth: *mut libc::c_char,
    pub hostname: *mut libc::c_char,
    pub pathname: *mut libc::c_char,
    pub search: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub hash: *mut libc::c_char,
    pub query: *mut libc::c_char,
    pub port: *mut libc::c_char,
}
pub type url_data_t = url_data;
#[no_mangle]
pub unsafe extern "C" fn url_data_inspect(mut data: *mut url_data_t) {
    printf(b"#url =>\n\0" as *const u8 as *const libc::c_char);
    printf(b"    .href: \"%s\"\n\0" as *const u8 as *const libc::c_char, (*data).href);
    printf(
        b"    .protocol: \"%s\"\n\0" as *const u8 as *const libc::c_char,
        (*data).protocol,
    );
    printf(b"    .host: \"%s\"\n\0" as *const u8 as *const libc::c_char, (*data).host);
    printf(b"    .auth: \"%s\"\n\0" as *const u8 as *const libc::c_char, (*data).auth);
    printf(
        b"    .hostname: \"%s\"\n\0" as *const u8 as *const libc::c_char,
        (*data).hostname,
    );
    printf(
        b"    .pathname: \"%s\"\n\0" as *const u8 as *const libc::c_char,
        (*data).pathname,
    );
    printf(
        b"    .search: \"%s\"\n\0" as *const u8 as *const libc::c_char,
        (*data).search,
    );
    printf(b"    .path: \"%s\"\n\0" as *const u8 as *const libc::c_char, (*data).path);
    printf(b"    .hash: \"%s\"\n\0" as *const u8 as *const libc::c_char, (*data).hash);
    printf(b"    .query: \"%s\"\n\0" as *const u8 as *const libc::c_char, (*data).query);
    printf(b"    .port: \"%s\"\n\0" as *const u8 as *const libc::c_char, (*data).port);
}
#[no_mangle]
pub unsafe extern "C" fn url_inspect(mut url: *mut libc::c_char) {
    url_data_inspect(url_parse(url));
}
#[no_mangle]
pub unsafe extern "C" fn url_free(mut data: *mut url_data_t) {
    if data.is_null() {
        return;
    }
    if !((*data).auth).is_null() {
        free((*data).auth as *mut libc::c_void);
    }
    if !((*data).protocol).is_null() {
        free((*data).protocol as *mut libc::c_void);
    }
    if !((*data).hostname).is_null() {
        free((*data).hostname as *mut libc::c_void);
    }
    if !((*data).host).is_null() {
        free((*data).host as *mut libc::c_void);
    }
    if !((*data).pathname).is_null() {
        free((*data).pathname as *mut libc::c_void);
    }
    if !((*data).path).is_null() {
        free((*data).path as *mut libc::c_void);
    }
    if !((*data).hash).is_null() {
        free((*data).hash as *mut libc::c_void);
    }
    if !((*data).search).is_null() {
        free((*data).search as *mut libc::c_void);
    }
    if !((*data).query).is_null() {
        free((*data).query as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn url_get_port(mut url: *mut libc::c_char) -> *mut libc::c_char {
    let mut port = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    let mut hostname = url_get_hostname(url);
    let mut host = url_get_host(url);
    if port.is_null() || hostname.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut tmp_hostname = strff(
        hostname,
        (strlen(host)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    sscanf(tmp_hostname, b"%s\0" as *const u8 as *const libc::c_char, port);
    free(hostname as *mut libc::c_void);
    free(tmp_hostname as *mut libc::c_void);
    return port;
}
#[no_mangle]
pub static mut URL_SCHEMES: [*mut libc::c_char; 177] = [
    b"aaa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aaas\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"about\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"acap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"acct\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"adiumxtra\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"afp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"afs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"apt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"attachment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beshare\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bitcoin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bolo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"callto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"chrome\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"crome-extension\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"com-evenbrite-attendee\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"coap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"coaps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"content\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"crid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cvs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dict\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lna-playsingle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dln-playcontainer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dns\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dtn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dvb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ed2k\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"facetime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"feed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"finger\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fish\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ftp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"geo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"git\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gizmoproject\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"go\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gopher\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gtalk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"h323\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hcp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"http\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"https\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"icap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"icon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"im\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"imap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ipn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ipp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"irc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"irc6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ircs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iris\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iris.beep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iris.xpc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iris.xpcs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iris.lws\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"itms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"jabber\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"jar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"jms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"keyparc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lastfm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ldap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ldaps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"magnet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mailserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mailto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"maps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"market\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"message\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"modem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ms-help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mssettings-power\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"msnim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"msrp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"msrps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mtqp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mumble\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mupdate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mvn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"news\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"nfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ni\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"nih\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"nntp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"notes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"oid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"paquelocktoken\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"palm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"paparazzi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pkcs11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"platform\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pres\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"prospero\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"proxy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"psyc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"query\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"reload\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"res\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"resource\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rmi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rsync\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rtmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rtsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"secondlife\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"service\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"session\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sftp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sgn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"shttp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sieve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sips\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"skype\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"smb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"snews\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"snmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"soap.beep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"soap.beeps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"soldat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"spotify\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ssh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"steam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"teamspeak\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"telnet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tftp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"things\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"thismessage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tn3270\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"udp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"unreal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"urn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ut2004\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"vemmi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ventrilo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"videotex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"view-source\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wais\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"webcal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ws\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wtai\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wyciwyg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xcon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xcon-userid\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xfire\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xmlrpc.beep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xmlrpc.beeps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xmpp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xri\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ymsgr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"javascript\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"jdbc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"doi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn url_parse(mut url: *mut libc::c_char) -> *mut url_data_t {
    let mut data = malloc(::std::mem::size_of::<url_data_t>() as libc::c_ulong)
        as *mut url_data_t;
    if data.is_null() {
        return 0 as *mut url_data_t;
    }
    let ref mut fresh0 = (*data).href;
    *fresh0 = url;
    let mut tmp = 0 as *mut libc::c_char;
    let mut tmp_url = strdup(url);
    let mut is_ssh = 0 as libc::c_int != 0;
    let mut protocol = url_get_protocol(tmp_url);
    if protocol.is_null() {
        return 0 as *mut url_data_t;
    }
    let mut protocol_len = strlen(protocol) as libc::c_int + 3 as libc::c_int;
    let ref mut fresh1 = (*data).protocol;
    *fresh1 = protocol;
    is_ssh = url_is_ssh(protocol);
    let mut auth = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    let mut auth_len = 0 as libc::c_int;
    tmp = strstr(tmp_url, b"@\0" as *const u8 as *const libc::c_char);
    if !tmp.is_null() {
        auth = get_part(
            tmp_url,
            b"%[^@]\0" as *const u8 as *const libc::c_char,
            protocol_len,
        );
        auth_len = strlen(auth) as libc::c_int;
        if !auth.is_null() {
            auth_len += 1;
        }
    }
    let ref mut fresh2 = (*data).auth;
    *fresh2 = auth;
    let mut hostname = 0 as *mut libc::c_char;
    hostname = if is_ssh as libc::c_int != 0 {
        get_part(
            tmp_url,
            b"%[^:]\0" as *const u8 as *const libc::c_char,
            protocol_len + auth_len,
        )
    } else {
        get_part(
            tmp_url,
            b"%[^/]\0" as *const u8 as *const libc::c_char,
            protocol_len + auth_len,
        )
    };
    if hostname.is_null() {
        return 0 as *mut url_data_t;
    }
    let mut hostname_len = strlen(hostname) as libc::c_int;
    let mut tmp_hostname = strdup(hostname);
    let ref mut fresh3 = (*data).hostname;
    *fresh3 = hostname;
    let mut host = malloc(
        (strlen(tmp_hostname))
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    sscanf(tmp_hostname, b"%[^:]\0" as *const u8 as *const libc::c_char, host);
    if host.is_null() {
        return 0 as *mut url_data_t;
    }
    let mut host_len = strlen(host) as libc::c_int;
    let ref mut fresh4 = (*data).host;
    *fresh4 = host;
    let mut tmp_path = 0 as *mut libc::c_char;
    tmp_path = if is_ssh as libc::c_int != 0 {
        get_part(
            tmp_url,
            b":%s\0" as *const u8 as *const libc::c_char,
            protocol_len + auth_len + hostname_len,
        )
    } else {
        get_part(
            tmp_url,
            b"/%s\0" as *const u8 as *const libc::c_char,
            protocol_len + auth_len + hostname_len,
        )
    };
    let mut path = malloc(
        (strlen(tmp_path))
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if path.is_null() {
        return 0 as *mut url_data_t;
    }
    let mut fmt = (if is_ssh as libc::c_int != 0 {
        b"%s\0" as *const u8 as *const libc::c_char
    } else {
        b"/%s\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    sprintf(path, fmt, tmp_path);
    let ref mut fresh5 = (*data).path;
    *fresh5 = path;
    free(tmp_path as *mut libc::c_void);
    let mut pathname = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if pathname.is_null() {
        return 0 as *mut url_data_t;
    }
    strcat(pathname, b"\0" as *const u8 as *const libc::c_char);
    tmp_path = strdup(path);
    sscanf(tmp_path, b"%[^? | ^#]\0" as *const u8 as *const libc::c_char, pathname);
    let mut pathname_len = strlen(pathname) as libc::c_int;
    let ref mut fresh6 = (*data).pathname;
    *fresh6 = pathname;
    let mut search = malloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if search.is_null() {
        return 0 as *mut url_data_t;
    }
    tmp_path = strff(tmp_path, pathname_len);
    strcat(search, b"\0" as *const u8 as *const libc::c_char);
    sscanf(tmp_path, b"%[^#]\0" as *const u8 as *const libc::c_char, search);
    let ref mut fresh7 = (*data).search;
    *fresh7 = search;
    let mut search_len = strlen(search) as libc::c_int;
    free(tmp_path as *mut libc::c_void);
    let mut query = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if query.is_null() {
        return 0 as *mut url_data_t;
    }
    sscanf(search, b"?%s\0" as *const u8 as *const libc::c_char, query);
    let ref mut fresh8 = (*data).query;
    *fresh8 = query;
    let mut hash = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if hash.is_null() {
        return 0 as *mut url_data_t;
    }
    tmp_path = strff(path, pathname_len + search_len);
    strcat(hash, b"\0" as *const u8 as *const libc::c_char);
    sscanf(tmp_path, b"%s\0" as *const u8 as *const libc::c_char, hash);
    let ref mut fresh9 = (*data).hash;
    *fresh9 = hash;
    free(tmp_path as *mut libc::c_void);
    let mut port = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if port.is_null() {
        return 0 as *mut url_data_t;
    }
    tmp_hostname = strff(hostname, host_len + 1 as libc::c_int);
    sscanf(tmp_hostname, b"%s\0" as *const u8 as *const libc::c_char, port);
    let ref mut fresh10 = (*data).port;
    *fresh10 = port;
    free(tmp_hostname as *mut libc::c_void);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut n = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let mut dup = malloc(n as libc::c_ulong) as *mut libc::c_char;
    if !dup.is_null() {
        strcpy(dup, str);
    }
    return dup;
}
unsafe extern "C" fn strff(
    mut ptr: *mut libc::c_char,
    mut n: libc::c_int,
) -> *mut libc::c_char {
    let mut y = 0 as libc::c_int;
    let mut i = 0 as libc::c_int;
    while i < n {
        let fresh11 = ptr;
        ptr = ptr.offset(1);
        y = *fresh11 as libc::c_int;
        i += 1;
    }
    return strdup(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn url_get_protocol(
    mut url: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut protocol = malloc(
        (16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if protocol.is_null() {
        return 0 as *mut libc::c_char;
    }
    sscanf(url, b"%[^://]\0" as *const u8 as *const libc::c_char, protocol);
    if url_is_protocol(protocol) {
        return protocol;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn url_is_protocol(mut str: *mut libc::c_char) -> bool {
    let mut count = (::std::mem::size_of::<[*mut libc::c_char; 177]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    let mut i = 0 as libc::c_int;
    while i < count {
        if 0 as libc::c_int == strcmp(URL_SCHEMES[i as usize], str) {
            return 1 as libc::c_int != 0;
        }
        i += 1;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn get_part(
    mut url: *mut libc::c_char,
    mut format: *const libc::c_char,
    mut l: libc::c_int,
) -> *mut libc::c_char {
    let mut has = 0 as libc::c_int != 0;
    let mut tmp = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    let mut tmp_url = strdup(url);
    let mut fmt_url = strdup(url);
    let mut ret = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() || ret.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcpy(tmp, b"\0" as *const u8 as *const libc::c_char);
    strcpy(fmt_url, b"\0" as *const u8 as *const libc::c_char);
    fmt_url = strff(fmt_url, l);
    sscanf(fmt_url, format, tmp);
    if 0 as libc::c_int != strcmp(tmp, tmp_url) {
        has = 1 as libc::c_int != 0;
        ret = strdup(tmp);
    }
    fmt_url = strrwd(fmt_url, l);
    free(tmp as *mut libc::c_void);
    free(tmp_url as *mut libc::c_void);
    free(fmt_url as *mut libc::c_void);
    return if has as libc::c_int != 0 { ret } else { 0 as *mut libc::c_char };
}
unsafe extern "C" fn strrwd(
    mut ptr: *mut libc::c_char,
    mut n: libc::c_int,
) -> *mut libc::c_char {
    let mut y = 0 as libc::c_int;
    let mut i = 0 as libc::c_int;
    while i < n {
        let fresh12 = ptr;
        ptr = ptr.offset(-1);
        y = *fresh12 as libc::c_int;
        i += 1;
    }
    return strdup(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn url_is_ssh(mut str: *mut libc::c_char) -> bool {
    str = strdup(str);
    if 0 as libc::c_int == strcmp(str, b"ssh\0" as *const u8 as *const libc::c_char)
        || 0 as libc::c_int == strcmp(str, b"git\0" as *const u8 as *const libc::c_char)
    {
        free(str as *mut libc::c_void);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_auth(mut url: *mut libc::c_char) -> *mut libc::c_char {
    let mut protocol = url_get_protocol(url);
    if protocol.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut l = strlen(protocol) as libc::c_int + 3 as libc::c_int;
    return get_part(url, b"%[^@]\0" as *const u8 as *const libc::c_char, l);
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hostname(
    mut url: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut l = 3 as libc::c_int;
    let mut protocol = url_get_protocol(url);
    let mut tmp_protocol = strdup(protocol);
    let mut auth = url_get_auth(url);
    if protocol.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !auth.is_null() {
        l = (l as libc::c_ulong)
            .wrapping_add((strlen(auth)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as libc::c_int as libc::c_int;
    }
    if !auth.is_null() {
        free(auth as *mut libc::c_void);
    }
    l += strlen(protocol) as libc::c_int;
    free(protocol as *mut libc::c_void);
    let mut hostname = if url_is_ssh(tmp_protocol) as libc::c_int != 0 {
        get_part(url, b"%[^:]\0" as *const u8 as *const libc::c_char, l)
    } else {
        get_part(url, b"%[^/]\0" as *const u8 as *const libc::c_char, l)
    };
    free(tmp_protocol as *mut libc::c_void);
    return hostname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_host(mut url: *mut libc::c_char) -> *mut libc::c_char {
    let mut host = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    let mut hostname = url_get_hostname(url);
    if host.is_null() || hostname.is_null() {
        return 0 as *mut libc::c_char;
    }
    sscanf(hostname, b"%[^:]\0" as *const u8 as *const libc::c_char, host);
    free(hostname as *mut libc::c_void);
    return host;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_pathname(
    mut url: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut path = url_get_path(url);
    let mut pathname = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if path.is_null() || pathname.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcat(pathname, b"\0" as *const u8 as *const libc::c_char);
    sscanf(path, b"%[^?]\0" as *const u8 as *const libc::c_char, pathname);
    free(path as *mut libc::c_void);
    return pathname;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_path(mut url: *mut libc::c_char) -> *mut libc::c_char {
    let mut l = 3 as libc::c_int;
    let mut tmp_path = 0 as *mut libc::c_char;
    let mut protocol = url_get_protocol(url);
    let mut auth = url_get_auth(url);
    let mut hostname = url_get_hostname(url);
    if protocol.is_null() || hostname.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut is_ssh = url_is_ssh(protocol);
    l += strlen(protocol) as libc::c_int + strlen(hostname) as libc::c_int;
    if !auth.is_null() {
        l += strlen(auth) as libc::c_int + 1 as libc::c_int;
    }
    tmp_path = if is_ssh as libc::c_int != 0 {
        get_part(url, b":%s\0" as *const u8 as *const libc::c_char, l)
    } else {
        get_part(url, b"/%s\0" as *const u8 as *const libc::c_char, l)
    };
    let mut fmt = (if is_ssh as libc::c_int != 0 {
        b"%s\0" as *const u8 as *const libc::c_char
    } else {
        b"/%s\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut path = malloc(
        (strlen(tmp_path))
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(path, fmt, tmp_path);
    if !auth.is_null() {
        free(auth as *mut libc::c_void);
    }
    free(protocol as *mut libc::c_void);
    free(hostname as *mut libc::c_void);
    free(tmp_path as *mut libc::c_void);
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_search(
    mut url: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut path = url_get_path(url);
    let mut pathname = url_get_pathname(url);
    let mut search = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if path.is_null() || search.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut tmp_path = strff(path, strlen(pathname) as libc::c_int);
    strcat(search, b"\0" as *const u8 as *const libc::c_char);
    sscanf(tmp_path, b"%[^#]\0" as *const u8 as *const libc::c_char, search);
    tmp_path = strrwd(tmp_path, strlen(pathname) as libc::c_int);
    free(path as *mut libc::c_void);
    free(pathname as *mut libc::c_void);
    return search;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_query(mut url: *mut libc::c_char) -> *mut libc::c_char {
    let mut search = url_get_search(url);
    let mut query = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if search.is_null() {
        return 0 as *mut libc::c_char;
    }
    sscanf(search, b"?%s\0" as *const u8 as *const libc::c_char, query);
    free(search as *mut libc::c_void);
    return query;
}
#[no_mangle]
pub unsafe extern "C" fn url_get_hash(mut url: *mut libc::c_char) -> *mut libc::c_char {
    let mut hash = malloc(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        as *mut libc::c_char;
    if hash.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut path = url_get_path(url);
    if path.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut pathname = url_get_pathname(url);
    if pathname.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut search = url_get_search(url);
    let mut pathname_len = strlen(pathname) as libc::c_int;
    let mut search_len = strlen(search) as libc::c_int;
    let mut tmp_path = strff(path, pathname_len + search_len);
    strcat(hash, b"\0" as *const u8 as *const libc::c_char);
    sscanf(tmp_path, b"%s\0" as *const u8 as *const libc::c_char, hash);
    tmp_path = strrwd(tmp_path, pathname_len + search_len);
    free(tmp_path as *mut libc::c_void);
    free(pathname as *mut libc::c_void);
    free(path as *mut libc::c_void);
    if !search.is_null() {
        free(search as *mut libc::c_void);
    }
    return hash;
}
unsafe fn main_0() -> libc::c_int {
    let mut gh_url = b"git://git@github.com:jwerle/url.h.git\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut url = b"http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut parsed = url_parse(url);
    let mut gh_parsed = url_parse(gh_url);
    if !parsed.is_null() {} else {
        __assert_fail(
            b"parsed\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !gh_parsed.is_null() {} else {
        __assert_fail(
            b"gh_parsed\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    url_data_inspect(parsed);
    url_data_inspect(gh_parsed);
    if !((*parsed).href).is_null() {} else {
        __assert_fail(
            b"parsed->href\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            21 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).auth).is_null() {} else {
        __assert_fail(
            b"parsed->auth\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            22 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).protocol).is_null() {} else {
        __assert_fail(
            b"parsed->protocol\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).port).is_null() {} else {
        __assert_fail(
            b"parsed->port\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            24 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).hostname).is_null() {} else {
        __assert_fail(
            b"parsed->hostname\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            25 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).host).is_null() {} else {
        __assert_fail(
            b"parsed->host\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).pathname).is_null() {} else {
        __assert_fail(
            b"parsed->pathname\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).path).is_null() {} else {
        __assert_fail(
            b"parsed->path\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).hash).is_null() {} else {
        __assert_fail(
            b"parsed->hash\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).search).is_null() {} else {
        __assert_fail(
            b"parsed->search\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*parsed).query).is_null() {} else {
        __assert_fail(
            b"parsed->query\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*gh_parsed).href).is_null() {} else {
        __assert_fail(
            b"gh_parsed->href\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*gh_parsed).protocol).is_null() {} else {
        __assert_fail(
            b"gh_parsed->protocol\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*gh_parsed).host).is_null() {} else {
        __assert_fail(
            b"gh_parsed->host\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*gh_parsed).auth).is_null() {} else {
        __assert_fail(
            b"gh_parsed->auth\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*gh_parsed).hostname).is_null() {} else {
        __assert_fail(
            b"gh_parsed->hostname\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*gh_parsed).pathname).is_null() {} else {
        __assert_fail(
            b"gh_parsed->pathname\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if !((*gh_parsed).path).is_null() {} else {
        __assert_fail(
            b"gh_parsed->path\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if url_is_protocol(
        b"http\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int != 0
    {} else {
        __assert_fail(
            b"url_is_protocol(\"http\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if url_is_protocol(
        b"https\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int != 0
    {} else {
        __assert_fail(
            b"url_is_protocol(\"https\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if url_is_protocol(b"git\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        as libc::c_int != 0
    {} else {
        __assert_fail(
            b"url_is_protocol(\"git\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if url_is_protocol(b"ssh\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        as libc::c_int != 0
    {} else {
        __assert_fail(
            b"url_is_protocol(\"ssh\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if url_is_protocol(
        b"sftp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int != 0
    {} else {
        __assert_fail(
            b"url_is_protocol(\"sftp\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if url_is_protocol(b"ftp\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        as libc::c_int != 0
    {} else {
        __assert_fail(
            b"url_is_protocol(\"ftp\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if url_is_protocol(
        b"javascript\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int != 0
    {} else {
        __assert_fail(
            b"url_is_protocol(\"javascript\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(b"http\0" as *const u8 as *const libc::c_char, url_get_protocol(url))
    {} else {
        __assert_fail(
            b"0 == strcmp(\"http\", url_get_protocol(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(b"user:pass\0" as *const u8 as *const libc::c_char, url_get_auth(url))
    {} else {
        __assert_fail(
            b"0 == strcmp(\"user:pass\", url_get_auth(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"subdomain.host.com:8080\0" as *const u8 as *const libc::c_char,
            url_get_hostname(url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"subdomain.host.com:8080\", url_get_hostname(url))\0"
                as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"subdomain.host.com\0" as *const u8 as *const libc::c_char,
            url_get_host(url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"subdomain.host.com\", url_get_host(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"/p/a/t/h\0" as *const u8 as *const libc::c_char,
            url_get_pathname(url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"/p/a/t/h\", url_get_pathname(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"/p/a/t/h?query=string#hash\0" as *const u8 as *const libc::c_char,
            url_get_path(url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"/p/a/t/h?query=string#hash\", url_get_path(url))\0"
                as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"?query=string\0" as *const u8 as *const libc::c_char,
            url_get_search(url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"?query=string\", url_get_search(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"query=string\0" as *const u8 as *const libc::c_char,
            url_get_query(url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"query=string\", url_get_query(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(b"#hash\0" as *const u8 as *const libc::c_char, url_get_hash(url))
    {} else {
        __assert_fail(
            b"0 == strcmp(\"#hash\", url_get_hash(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(b"8080\0" as *const u8 as *const libc::c_char, url_get_port(url))
    {} else {
        __assert_fail(
            b"0 == strcmp(\"8080\", url_get_port(url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(b"git\0" as *const u8 as *const libc::c_char, url_get_protocol(gh_url))
    {} else {
        __assert_fail(
            b"0 == strcmp(\"git\", url_get_protocol(gh_url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"github.com\0" as *const u8 as *const libc::c_char,
            url_get_host(gh_url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"github.com\", url_get_host(gh_url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"github.com\0" as *const u8 as *const libc::c_char,
            url_get_hostname(gh_url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"github.com\", url_get_hostname(gh_url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(b"git\0" as *const u8 as *const libc::c_char, url_get_auth(gh_url))
    {} else {
        __assert_fail(
            b"0 == strcmp(\"git\", url_get_auth(gh_url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"jwerle/url.h.git\0" as *const u8 as *const libc::c_char,
            url_get_pathname(gh_url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"jwerle/url.h.git\", url_get_pathname(gh_url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == strcmp(
            b"jwerle/url.h.git\0" as *const u8 as *const libc::c_char,
            url_get_path(gh_url),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"jwerle/url.h.git\", url_get_path(gh_url))\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 15],
                &[libc::c_char; 15],
            >(b"int main(void)\0"))
                .as_ptr(),
        );
    };
    url_free(parsed);
    return 0 as libc::c_int;
}
// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
