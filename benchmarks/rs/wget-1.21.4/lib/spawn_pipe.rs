use ::libc;
extern "C" {
    pub type __spawn_action;
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn abort() -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn canonicalize_filename_mode(
        _: *const libc::c_char,
        _: canonicalize_mode_t,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn block_fatal_signals();
    fn unblock_fatal_signals();
    fn find_in_given_path(
        progname: *const libc::c_char,
        path: *const libc::c_char,
        directory: *const libc::c_char,
        optimize_for_exec: bool,
    ) -> *const libc::c_char;
    fn pipe2_safer(_: *mut libc::c_int, _: libc::c_int) -> libc::c_int;
    fn register_slave_subprocess(child: pid_t);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn posix_spawn(
        __pid: *mut pid_t,
        __path: *const libc::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn posix_spawnp(
        __pid: *mut pid_t,
        __file: *const libc::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn posix_spawnattr_init(__attr: *mut posix_spawnattr_t) -> libc::c_int;
    fn posix_spawnattr_destroy(__attr: *mut posix_spawnattr_t) -> libc::c_int;
    fn posix_spawnattr_setsigmask(
        __attr: *mut posix_spawnattr_t,
        __sigmask: *const sigset_t,
    ) -> libc::c_int;
    fn posix_spawnattr_setflags(
        _attr: *mut posix_spawnattr_t,
        __flags: libc::c_short,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
        __newfd: libc::c_int,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_addclose(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_addopen(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __oflag: libc::c_int,
        __mode: mode_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_addchdir(
        __file_actions: *mut posix_spawn_file_actions_t,
        __path: *const libc::c_char,
    ) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: libc::c_int,
    pub __used: libc::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
pub type canonicalize_mode_t = libc::c_uint;
pub const CAN_NOLINKS: canonicalize_mode_t = 4;
pub const CAN_MISSING: canonicalize_mode_t = 2;
pub const CAN_ALL_BUT_LAST: canonicalize_mode_t = 1;
pub const CAN_EXISTING: canonicalize_mode_t = 0;
unsafe extern "C" fn nonintr_close(mut fd: libc::c_int) -> libc::c_int {
    let mut retval: libc::c_int = 0;
    loop {
        retval = close(fd);
        if !(retval < 0 as libc::c_int && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return retval;
}
unsafe extern "C" fn create_pipe(
    mut progname: *const libc::c_char,
    mut prog_path: *const libc::c_char,
    mut prog_argv: *const *const libc::c_char,
    mut directory: *const libc::c_char,
    mut pipe_stdin: bool,
    mut pipe_stdout: bool,
    mut prog_stdin: *const libc::c_char,
    mut prog_stdout: *const libc::c_char,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut libc::c_int,
) -> pid_t {
    let mut ifd: [libc::c_int; 2] = [0; 2];
    let mut ofd: [libc::c_int; 2] = [0; 2];
    let mut blocked_signals: sigset_t = sigset_t { __val: [0; 16] };
    let mut actions: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    let mut actions_allocated: bool = false;
    let mut attrs: posix_spawnattr_t = posix_spawnattr_t {
        __flags: 0,
        __pgrp: 0,
        __sd: sigset_t { __val: [0; 16] },
        __ss: sigset_t { __val: [0; 16] },
        __sp: sched_param { sched_priority: 0 },
        __policy: 0,
        __pad: [0; 16],
    };
    let mut attrs_allocated: bool = false;
    let mut err: libc::c_int = 0;
    let mut child: pid_t = 0;
    let mut current_block: u64;
    let mut saved_errno: libc::c_int = 0;
    let mut prog_path_to_free: *mut libc::c_char = 0 as *mut libc::c_char;
    if !directory.is_null() {
        if !(*prog_path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
            let mut resolved_prog: *const libc::c_char = find_in_given_path(
                prog_path,
                getenv(b"PATH\0" as *const u8 as *const libc::c_char),
                0 as *const libc::c_char,
                0 as libc::c_int != 0,
            );
            if resolved_prog.is_null() {
                current_block = 4581708784584515963;
            } else {
                if resolved_prog != prog_path {
                    prog_path_to_free = resolved_prog as *mut libc::c_char;
                }
                prog_path = resolved_prog;
                if !(*prog_path.offset(0 as libc::c_int as isize) as libc::c_int
                    == '/' as i32)
                {
                    let mut absolute_prog: *mut libc::c_char = canonicalize_filename_mode(
                        prog_path,
                        (CAN_MISSING as libc::c_int | CAN_NOLINKS as libc::c_int)
                            as canonicalize_mode_t,
                    );
                    if absolute_prog.is_null() {
                        rpl_free(prog_path_to_free as *mut libc::c_void);
                        current_block = 4581708784584515963;
                    } else {
                        rpl_free(prog_path_to_free as *mut libc::c_void);
                        prog_path_to_free = absolute_prog;
                        prog_path = absolute_prog;
                        if !(*prog_path.offset(0 as libc::c_int as isize) as libc::c_int
                            == '/' as i32)
                        {
                            abort();
                        }
                        current_block = 3512920355445576850;
                    }
                } else {
                    current_block = 3512920355445576850;
                }
            }
            match current_block {
                3512920355445576850 => {}
                _ => {
                    saved_errno = *__errno_location();
                    current_block = 15043446999318232975;
                }
            }
        } else {
            current_block = 3512920355445576850;
        }
    } else {
        current_block = 3512920355445576850;
    }
    match current_block {
        3512920355445576850 => {
            ifd = [0; 2];
            ofd = [0; 2];
            if pipe_stdout {
                if pipe2_safer(
                    ifd.as_mut_ptr(),
                    0 as libc::c_int | 0o2000000 as libc::c_int,
                ) < 0 as libc::c_int
                {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            b"wget-gnulib\0" as *const u8 as *const libc::c_char,
                            b"cannot create pipe\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            if pipe_stdin {
                if pipe2_safer(
                    ofd.as_mut_ptr(),
                    0 as libc::c_int | 0o2000000 as libc::c_int,
                ) < 0 as libc::c_int
                {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            b"wget-gnulib\0" as *const u8 as *const libc::c_char,
                            b"cannot create pipe\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            blocked_signals = sigset_t { __val: [0; 16] };
            actions = posix_spawn_file_actions_t {
                __allocated: 0,
                __used: 0,
                __actions: 0 as *mut __spawn_action,
                __pad: [0; 16],
            };
            actions_allocated = false;
            attrs = posix_spawnattr_t {
                __flags: 0,
                __pgrp: 0,
                __sd: sigset_t { __val: [0; 16] },
                __ss: sigset_t { __val: [0; 16] },
                __sp: sched_param { sched_priority: 0 },
                __policy: 0,
                __pad: [0; 16],
            };
            attrs_allocated = false;
            err = 0;
            child = 0;
            if slave_process {
                sigprocmask(
                    2 as libc::c_int,
                    0 as *const sigset_t,
                    &mut blocked_signals,
                );
                block_fatal_signals();
            }
            actions_allocated = 0 as libc::c_int != 0;
            attrs_allocated = 0 as libc::c_int != 0;
            err = posix_spawn_file_actions_init(&mut actions);
            if err != 0 as libc::c_int
                || {
                    actions_allocated = 1 as libc::c_int != 0;
                    pipe_stdin as libc::c_int != 0
                        && {
                            err = posix_spawn_file_actions_adddup2(
                                &mut actions,
                                ofd[0 as libc::c_int as usize],
                                0 as libc::c_int,
                            );
                            err != 0 as libc::c_int
                        }
                        || pipe_stdout as libc::c_int != 0
                            && {
                                err = posix_spawn_file_actions_adddup2(
                                    &mut actions,
                                    ifd[1 as libc::c_int as usize],
                                    1 as libc::c_int,
                                );
                                err != 0 as libc::c_int
                            }
                        || pipe_stdin as libc::c_int != 0
                            && {
                                err = posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ofd[0 as libc::c_int as usize],
                                );
                                err != 0 as libc::c_int
                            }
                        || pipe_stdout as libc::c_int != 0
                            && {
                                err = posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ifd[1 as libc::c_int as usize],
                                );
                                err != 0 as libc::c_int
                            }
                        || pipe_stdin as libc::c_int != 0
                            && {
                                err = posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ofd[1 as libc::c_int as usize],
                                );
                                err != 0 as libc::c_int
                            }
                        || pipe_stdout as libc::c_int != 0
                            && {
                                err = posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ifd[0 as libc::c_int as usize],
                                );
                                err != 0 as libc::c_int
                            }
                        || null_stderr as libc::c_int != 0
                            && {
                                err = posix_spawn_file_actions_addopen(
                                    &mut actions,
                                    2 as libc::c_int,
                                    b"/dev/null\0" as *const u8 as *const libc::c_char,
                                    0o2 as libc::c_int,
                                    0 as libc::c_int as mode_t,
                                );
                                err != 0 as libc::c_int
                            }
                        || !pipe_stdin && !prog_stdin.is_null()
                            && {
                                err = posix_spawn_file_actions_addopen(
                                    &mut actions,
                                    0 as libc::c_int,
                                    prog_stdin,
                                    0 as libc::c_int,
                                    0 as libc::c_int as mode_t,
                                );
                                err != 0 as libc::c_int
                            }
                        || !pipe_stdout && !prog_stdout.is_null()
                            && {
                                err = posix_spawn_file_actions_addopen(
                                    &mut actions,
                                    1 as libc::c_int,
                                    prog_stdout,
                                    0o1 as libc::c_int,
                                    0 as libc::c_int as mode_t,
                                );
                                err != 0 as libc::c_int
                            }
                        || !directory.is_null()
                            && {
                                err = posix_spawn_file_actions_addchdir(
                                    &mut actions,
                                    directory,
                                );
                                err != 0
                            }
                        || slave_process as libc::c_int != 0
                            && {
                                err = posix_spawnattr_init(&mut attrs);
                                err != 0 as libc::c_int
                                    || {
                                        attrs_allocated = 1 as libc::c_int != 0;
                                        err = posix_spawnattr_setsigmask(
                                            &mut attrs,
                                            &mut blocked_signals,
                                        );
                                        err != 0 as libc::c_int
                                            || {
                                                err = posix_spawnattr_setflags(
                                                    &mut attrs,
                                                    0x8 as libc::c_int as libc::c_short,
                                                );
                                                err != 0 as libc::c_int
                                            }
                                    }
                            }
                        || {
                            err = if !directory.is_null() {
                                posix_spawn(
                                    &mut child,
                                    prog_path,
                                    &mut actions,
                                    if attrs_allocated as libc::c_int != 0 {
                                        &mut attrs
                                    } else {
                                        0 as *mut posix_spawnattr_t
                                    },
                                    prog_argv as *const *mut libc::c_char,
                                    environ,
                                )
                            } else {
                                posix_spawnp(
                                    &mut child,
                                    prog_path,
                                    &mut actions,
                                    if attrs_allocated as libc::c_int != 0 {
                                        &mut attrs
                                    } else {
                                        0 as *mut posix_spawnattr_t
                                    },
                                    prog_argv as *const *mut libc::c_char,
                                    environ as *const *mut libc::c_char,
                                )
                            };
                            err != 0 as libc::c_int
                        }
                }
            {
                if actions_allocated {
                    posix_spawn_file_actions_destroy(&mut actions);
                }
                if attrs_allocated {
                    posix_spawnattr_destroy(&mut attrs);
                }
                if slave_process {
                    unblock_fatal_signals();
                }
                if pipe_stdout {
                    nonintr_close(ifd[0 as libc::c_int as usize]);
                    nonintr_close(ifd[1 as libc::c_int as usize]);
                }
                if pipe_stdin {
                    nonintr_close(ofd[0 as libc::c_int as usize]);
                    nonintr_close(ofd[1 as libc::c_int as usize]);
                }
                rpl_free(prog_path_to_free as *mut libc::c_void);
                saved_errno = err;
            } else {
                posix_spawn_file_actions_destroy(&mut actions);
                if attrs_allocated {
                    posix_spawnattr_destroy(&mut attrs);
                }
                if slave_process {
                    register_slave_subprocess(child);
                    unblock_fatal_signals();
                }
                if pipe_stdin {
                    nonintr_close(ofd[0 as libc::c_int as usize]);
                }
                if pipe_stdout {
                    nonintr_close(ifd[1 as libc::c_int as usize]);
                }
                rpl_free(prog_path_to_free as *mut libc::c_void);
                if pipe_stdout {
                    *fd
                        .offset(
                            0 as libc::c_int as isize,
                        ) = ifd[0 as libc::c_int as usize];
                }
                if pipe_stdin {
                    *fd
                        .offset(
                            1 as libc::c_int as isize,
                        ) = ofd[1 as libc::c_int as usize];
                }
                return child;
            }
        }
        _ => {}
    }
    if exit_on_error as libc::c_int != 0 || !null_stderr {
        error(
            if exit_on_error as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            },
            saved_errno,
            dcgettext(
                b"wget-gnulib\0" as *const u8 as *const libc::c_char,
                b"%s subprocess failed\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            progname,
        );
    }
    *__errno_location() = saved_errno;
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn create_pipe_bidi(
    mut progname: *const libc::c_char,
    mut prog_path: *const libc::c_char,
    mut prog_argv: *const *const libc::c_char,
    mut directory: *const libc::c_char,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut libc::c_int,
) -> pid_t {
    let mut result: pid_t = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        null_stderr,
        slave_process,
        exit_on_error,
        fd,
    );
    return result;
}
pub unsafe extern "C" fn create_pipe_in(
    mut progname: *const libc::c_char,
    mut prog_path: *const libc::c_char,
    mut prog_argv: *const *const libc::c_char,
    mut directory: *const libc::c_char,
    mut prog_stdin: *const libc::c_char,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut libc::c_int,
) -> pid_t {
    let mut iofd: [libc::c_int; 2] = [0; 2];
    let mut result: pid_t = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
        prog_stdin,
        0 as *const libc::c_char,
        null_stderr,
        slave_process,
        exit_on_error,
        iofd.as_mut_ptr(),
    );
    if result != -(1 as libc::c_int) {
        *fd.offset(0 as libc::c_int as isize) = iofd[0 as libc::c_int as usize];
    }
    return result;
}
pub unsafe extern "C" fn create_pipe_out(
    mut progname: *const libc::c_char,
    mut prog_path: *const libc::c_char,
    mut prog_argv: *const *const libc::c_char,
    mut directory: *const libc::c_char,
    mut prog_stdout: *const libc::c_char,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut libc::c_int,
) -> pid_t {
    let mut iofd: [libc::c_int; 2] = [0; 2];
    let mut result: pid_t = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as *const libc::c_char,
        prog_stdout,
        null_stderr,
        slave_process,
        exit_on_error,
        iofd.as_mut_ptr(),
    );
    if result != -(1 as libc::c_int) {
        *fd.offset(0 as libc::c_int as isize) = iofd[1 as libc::c_int as usize];
    }
    return result;
}
