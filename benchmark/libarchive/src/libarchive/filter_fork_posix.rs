use ::libc;
extern "C" {
    pub type __spawn_action;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn posix_spawnp(
        __pid: *mut pid_t,
        __file: *const libc::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn posix_spawn_file_actions_addclose(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
        __newfd: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dup(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __archive_cmdline_allocate() -> *mut archive_cmdline;
    #[no_mangle]
    fn __archive_cmdline_parse(_: *mut archive_cmdline, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __archive_cmdline_free(_: *mut archive_cmdline) -> libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: libc::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: libc::c_int,
    pub __pad: [libc::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: libc::c_int,
    pub __used: libc::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
/*-
 * Copyright (c) 2012 Michihiro NAKAJIMA
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * $FreeBSD$
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_cmdline {
    pub path: *mut libc::c_char,
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const POLLIN: libc::c_int = 0x1 as libc::c_int;
pub const POLLOUT: libc::c_int = 0x4 as libc::c_int;
pub const O_NONBLOCK: libc::c_int = 0o4000 as libc::c_int;
pub const F_SETFL: libc::c_int = 4 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
/*-
 * Copyright (c) 2007 Joerg Sonnenberger
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * $FreeBSD: head/lib/libarchive/filter_fork.h 201087 2009-12-28 02:18:26Z kientzle $
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_create_child(
    mut cmd: *const libc::c_char,
    mut child_stdin: *mut libc::c_int,
    mut child_stdout: *mut libc::c_int,
    mut out_child: *mut pid_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut child: pid_t = 0;
    let mut stdin_pipe: [libc::c_int; 2] = [0; 2];
    let mut stdout_pipe: [libc::c_int; 2] = [0; 2];
    let mut tmp: libc::c_int = 0;
    let mut actions: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    let mut r: libc::c_int = 0;
    let mut cmdline: *mut archive_cmdline = 0 as *mut archive_cmdline;
    cmdline = __archive_cmdline_allocate();
    if !cmdline.is_null() {
        if !(__archive_cmdline_parse(cmdline, cmd) != ARCHIVE_OK) {
            if !(pipe(stdin_pipe.as_mut_ptr()) == -(1 as libc::c_int)) {
                if stdin_pipe[0 as libc::c_int as usize] == 1 as libc::c_int {
                    /* stdout */
                    tmp = dup(stdin_pipe[0 as libc::c_int as usize]);
                    if tmp == -(1 as libc::c_int) {
                        current_block = 6682144097986572766;
                    } else {
                        close(stdin_pipe[0 as libc::c_int as usize]);
                        stdin_pipe[0 as libc::c_int as usize] = tmp;
                        current_block = 2968425633554183086;
                    }
                } else {
                    current_block = 2968425633554183086;
                }
                match current_block {
                    2968425633554183086 => {
                        if !(pipe(stdout_pipe.as_mut_ptr()) == -(1 as libc::c_int)) {
                            if stdout_pipe[1 as libc::c_int as usize] == 0 as libc::c_int {
                                /* stdin */
                                tmp = dup(stdout_pipe[1 as libc::c_int as usize]);
                                if tmp == -(1 as libc::c_int) {
                                    current_block = 11649451133945740713;
                                } else {
                                    close(stdout_pipe[1 as libc::c_int as usize]);
                                    stdout_pipe[1 as libc::c_int as usize] = tmp;
                                    current_block = 4956146061682418353;
                                }
                            } else {
                                current_block = 4956146061682418353;
                            }
                            match current_block {
                                4956146061682418353 => {
                                    r = posix_spawn_file_actions_init(&mut actions);
                                    if r != 0 as libc::c_int {
                                        errno = r
                                    } else {
                                        r = posix_spawn_file_actions_addclose(
                                            &mut actions,
                                            stdin_pipe[1 as libc::c_int as usize],
                                        );
                                        if !(r != 0 as libc::c_int) {
                                            r = posix_spawn_file_actions_addclose(
                                                &mut actions,
                                                stdout_pipe[0 as libc::c_int as usize],
                                            );
                                            if !(r != 0 as libc::c_int) {
                                                /* Setup for stdin. */
                                                r = posix_spawn_file_actions_adddup2(
                                                    &mut actions,
                                                    stdin_pipe[0 as libc::c_int as usize],
                                                    0 as libc::c_int,
                                                );
                                                if !(r != 0 as libc::c_int) {
                                                    if stdin_pipe[0 as libc::c_int as usize]
                                                        != 0 as libc::c_int
                                                    {
                                                        /* stdin */
                                                        r = posix_spawn_file_actions_addclose(
                                                            &mut actions,
                                                            stdin_pipe[0 as libc::c_int as usize],
                                                        );
                                                        if r != 0 as libc::c_int {
                                                            current_block = 16705220375951505417;
                                                        } else {
                                                            current_block = 11932355480408055363;
                                                        }
                                                    } else {
                                                        current_block = 11932355480408055363;
                                                    }
                                                    match current_block {
                                                        16705220375951505417 => {}
                                                        _ => {
                                                            /* Setup for stdout. */
                                                            r = posix_spawn_file_actions_adddup2(
                                                                &mut actions,
                                                                stdout_pipe
                                                                    [1 as libc::c_int as usize],
                                                                1 as libc::c_int,
                                                            );
                                                            if !(r != 0 as libc::c_int) {
                                                                if stdout_pipe
                                                                    [1 as libc::c_int as usize]
                                                                    != 1 as libc::c_int
                                                                {
                                                                    /* stdout */
                                                                    r =
                                                                        posix_spawn_file_actions_addclose(&mut actions,
                                                                                                          stdout_pipe[1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          usize]);
                                                                    if r != 0 as libc::c_int {
                                                                        current_block =
                                                                            16705220375951505417;
                                                                    } else {
                                                                        current_block =
                                                                            17184638872671510253;
                                                                    }
                                                                } else {
                                                                    current_block =
                                                                        17184638872671510253;
                                                                }
                                                                match current_block {
                                                                    16705220375951505417 => {}
                                                                    _ => {
                                                                        r =
                                                                            posix_spawnp(&mut child,
                                                                                         (*cmdline).path,
                                                                                         &mut actions,
                                                                                         NULL
                                                                                             as
                                                                                             *const posix_spawnattr_t,
                                                                                         (*cmdline).argv
                                                                                             as
                                                                                             *const *mut libc::c_char,
                                                                                         NULL
                                                                                             as
                                                                                             *const *mut libc::c_char);
                                                                        if !(r != 0 as libc::c_int)
                                                                        {
                                                                            posix_spawn_file_actions_destroy(&mut actions);
                                                                            /* HAVE_POSIX_SPAWNP */
                                                                            /* HAVE_POSIX_SPAWNP */
                                                                            close(
                                                                                stdin_pipe[0
                                                                                    as libc::c_int
                                                                                    as usize],
                                                                            );
                                                                            close(
                                                                                stdout_pipe[1
                                                                                    as libc::c_int
                                                                                    as usize],
                                                                            );
                                                                            *child_stdin =
                                                                                stdin_pipe[1
                                                                                    as libc::c_int
                                                                                    as usize];
                                                                            fcntl(
                                                                                *child_stdin,
                                                                                F_SETFL,
                                                                                O_NONBLOCK,
                                                                            );
                                                                            *child_stdout =
                                                                                stdout_pipe[0
                                                                                    as libc::c_int
                                                                                    as usize];
                                                                            fcntl(
                                                                                *child_stdout,
                                                                                F_SETFL,
                                                                                O_NONBLOCK,
                                                                            );
                                                                            __archive_cmdline_free(
                                                                                cmdline,
                                                                            );
                                                                            *out_child = child;
                                                                            return ARCHIVE_OK;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        errno = r;
                                        posix_spawn_file_actions_destroy(&mut actions);
                                    }
                                }
                                _ => {}
                            }
                            close(stdout_pipe[0 as libc::c_int as usize]);
                            close(stdout_pipe[1 as libc::c_int as usize]);
                        }
                    }
                    _ => {}
                }
                close(stdin_pipe[0 as libc::c_int as usize]);
                close(stdin_pipe[1 as libc::c_int as usize]);
            }
        }
    }
    __archive_cmdline_free(cmdline);
    return ARCHIVE_FAILED;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_check_child(mut in_0: libc::c_int, mut out: libc::c_int) {
    let mut fds: [pollfd; 2] = [pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    }; 2];
    let mut idx: libc::c_int = 0;
    idx = 0 as libc::c_int;
    if in_0 != -(1 as libc::c_int) {
        fds[idx as usize].fd = in_0;
        fds[idx as usize].events = POLLOUT as libc::c_short;
        idx += 1
    }
    if out != -(1 as libc::c_int) {
        fds[idx as usize].fd = out;
        fds[idx as usize].events = POLLIN as libc::c_short;
        idx += 1
    }
    poll(fds.as_mut_ptr(), idx as nfds_t, -(1 as libc::c_int));
    /* -1 == INFTIM, wait forever */
}
/* defined(HAVE_PIPE) && defined(HAVE_VFORK) && defined(HAVE_FCNTL) */
