use ::libc;
pub type XXH_errorcode = libc::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_xxhash {
    pub XXH32: Option<
        unsafe extern "C" fn(
            _: *const libc::c_void,
            _: libc::c_uint,
            _: libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub XXH32_init: Option<unsafe extern "C" fn(_: libc::c_uint) -> *mut libc::c_void>,
    pub XXH32_update: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_uint,
        ) -> XXH_errorcode,
    >,
    pub XXH32_digest: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_uint>,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
xxHash - Fast Hash algorithm
Copyright (C) 2012-2014, Yann Collet.
BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

* Redistributions of source code must retain the above copyright
notice, this list of conditions and the following disclaimer.
* Redistributions in binary form must reproduce the above
copyright notice, this list of conditions and the following disclaimer
in the documentation and/or other materials provided with the
distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

You can contact the author at :
- xxHash source repository : http://code.google.com/p/xxhash/
*/
/*
 * Define an empty version of the struct if we aren't using the LZ4 library.
 */
#[no_mangle]
pub static mut __archive_xxhash: archive_xxhash = unsafe {
    {
        let mut init = archive_xxhash {
            XXH32: ::std::mem::transmute::<
                libc::intptr_t,
                Option<
                    unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: libc::c_uint,
                        _: libc::c_uint,
                    ) -> libc::c_uint,
                >,
            >(NULL as libc::intptr_t),
            XXH32_init: ::std::mem::transmute::<
                libc::intptr_t,
                Option<unsafe extern "C" fn(_: libc::c_uint) -> *mut libc::c_void>,
            >(NULL as libc::intptr_t),
            XXH32_update: ::std::mem::transmute::<
                libc::intptr_t,
                Option<
                    unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: *const libc::c_void,
                        _: libc::c_uint,
                    ) -> XXH_errorcode,
                >,
            >(NULL as libc::intptr_t),
            XXH32_digest: ::std::mem::transmute::<
                libc::intptr_t,
                Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_uint>,
            >(NULL as libc::intptr_t),
        };
        init
    }
};
/* HAVE_LIBLZ4 */
