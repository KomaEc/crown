use ::libc;
extern "C" {
    pub type archive_entry;
    #[no_mangle]
    fn archive_entry_set_atime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_unset_birthtime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_dev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_ino(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_rdev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type la_int64_t = int64_t;
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_stat(
    mut entry: *mut archive_entry,
    mut st: *const stat,
) {
    archive_entry_set_atime(entry, (*st).st_atim.tv_sec, (*st).st_atim.tv_nsec);
    archive_entry_set_ctime(entry, (*st).st_ctim.tv_sec, (*st).st_ctim.tv_nsec);
    archive_entry_set_mtime(entry, (*st).st_mtim.tv_sec, (*st).st_mtim.tv_nsec);
    archive_entry_unset_birthtime(entry);
    archive_entry_set_dev(entry, (*st).st_dev);
    archive_entry_set_gid(entry, (*st).st_gid as la_int64_t);
    archive_entry_set_uid(entry, (*st).st_uid as la_int64_t);
    archive_entry_set_ino(entry, (*st).st_ino as la_int64_t);
    archive_entry_set_nlink(entry, (*st).st_nlink as libc::c_uint);
    archive_entry_set_rdev(entry, (*st).st_rdev);
    archive_entry_set_size(entry, (*st).st_size);
    archive_entry_set_mode(entry, (*st).st_mode);
}
