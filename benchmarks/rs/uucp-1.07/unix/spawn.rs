use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut iDebug: libc::c_int;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> libc::c_int;
    fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> libc::c_int;
    fn setgid(__gid: __gid_t) -> libc::c_int;
    fn setregid(__rgid: __gid_t, __egid: __gid_t) -> libc::c_int;
    fn ixsfork() -> pid_t;
    static mut zSspooldir: *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub unsafe extern "C" fn ixsspawn(
    mut pazargs: *mut *const libc::c_char,
    mut aidescs: *mut libc::c_int,
    mut fkeepuid: boolean,
    mut fkeepenv: boolean,
    mut zchdir: *const libc::c_char,
    mut fnosigs: boolean,
    mut fshell: boolean,
    mut zpath: *const libc::c_char,
    mut zuu_machine: *const libc::c_char,
    mut zuu_user: *const libc::c_char,
) -> pid_t {
    let mut zshcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut azenv: [*mut libc::c_char; 9] = [0 as *mut libc::c_char; 9];
    let mut pazenv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ferr: boolean = 0;
    let mut ierr: libc::c_int = 0 as libc::c_int;
    let mut onull: libc::c_int = 0;
    let mut aichild_descs: [libc::c_int; 3] = [0; 3];
    let mut cpar_close: libc::c_int = 0;
    let mut aipar_close: [libc::c_int; 4] = [0; 4];
    let mut cchild_close: libc::c_int = 0;
    let mut aichild_close: [libc::c_int; 3] = [0; 3];
    let mut iret: pid_t = 0 as libc::c_int;
    let mut zcmd: *const libc::c_char = 0 as *const libc::c_char;
    zshcmd = 0 as *mut libc::c_char;
    if fshell != 0 {
        let mut clen: size_t = 0;
        clen = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int;
        while !(*pazargs.offset(i as isize)).is_null() {
            clen = (clen as libc::c_ulong)
                .wrapping_add(strlen(*pazargs.offset(i as isize))) as size_t as size_t;
            i += 1;
            i;
        }
        zshcmd = zbufalc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(clen)
                .wrapping_add(i as libc::c_ulong),
        );
    }
    if fkeepenv != 0 {
        pazenv = environ;
    } else {
        let mut zterm: *const libc::c_char = 0 as *const libc::c_char;
        let mut ztz: *const libc::c_char = 0 as *const libc::c_char;
        let mut zspace: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ienv: libc::c_int = 0;
        if zpath.is_null() {
            zpath = b"/bin /usr/bin /usr/local/bin\0" as *const u8
                as *const libc::c_char;
        }
        azenv[0 as libc::c_int
            as usize] = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(zpath)),
        );
        sprintf(
            azenv[0 as libc::c_int as usize],
            b"PATH=%s\0" as *const u8 as *const libc::c_char,
            zpath,
        );
        zspace = (azenv[0 as libc::c_int as usize])
            .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
            .offset(-(1 as libc::c_int as isize));
        loop {
            zspace = strchr(zspace, ' ' as i32);
            if zspace.is_null() {
                break;
            }
            *zspace = ':' as i32 as libc::c_char;
        }
        azenv[1 as libc::c_int
            as usize] = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(zSspooldir)),
        );
        sprintf(
            azenv[1 as libc::c_int as usize],
            b"HOME=%s\0" as *const u8 as *const libc::c_char,
            zSspooldir,
        );
        zterm = getenv(b"TERM\0" as *const u8 as *const libc::c_char);
        if zterm.is_null() {
            zterm = b"unknown\0" as *const u8 as *const libc::c_char;
        }
        azenv[2 as libc::c_int
            as usize] = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(zterm)),
        );
        sprintf(
            azenv[2 as libc::c_int as usize],
            b"TERM=%s\0" as *const u8 as *const libc::c_char,
            zterm,
        );
        azenv[3 as libc::c_int
            as usize] = zbufcpy(b"SHELL=/bin/sh\0" as *const u8 as *const libc::c_char);
        azenv[4 as libc::c_int
            as usize] = zbufalc(
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_add(strlen(b"uucp\0" as *const u8 as *const libc::c_char)),
        );
        sprintf(
            azenv[4 as libc::c_int as usize],
            b"USER=%s\0" as *const u8 as *const libc::c_char,
            b"uucp\0" as *const u8 as *const libc::c_char,
        );
        ienv = 5 as libc::c_int;
        ztz = getenv(b"TZ\0" as *const u8 as *const libc::c_char);
        if !ztz.is_null() {
            azenv[ienv
                as usize] = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_add(strlen(ztz)),
            );
            sprintf(
                azenv[ienv as usize],
                b"TZ=%s\0" as *const u8 as *const libc::c_char,
                ztz,
            );
            ienv += 1;
            ienv;
        }
        if !zuu_machine.is_null() {
            azenv[ienv
                as usize] = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_add(strlen(zuu_machine)),
            );
            sprintf(
                azenv[ienv as usize],
                b"UU_MACHINE=%s\0" as *const u8 as *const libc::c_char,
                zuu_machine,
            );
            ienv += 1;
            ienv;
        }
        if !zuu_user.is_null() {
            azenv[ienv
                as usize] = zbufalc(
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_add(strlen(zuu_user)),
            );
            sprintf(
                azenv[ienv as usize],
                b"UU_USER=%s\0" as *const u8 as *const libc::c_char,
                zuu_user,
            );
            ienv += 1;
            ienv;
        }
        azenv[ienv as usize] = 0 as *mut libc::c_char;
        pazenv = azenv.as_mut_ptr();
    }
    ferr = 0 as libc::c_int;
    onull = -(1 as libc::c_int);
    cpar_close = 0 as libc::c_int;
    cchild_close = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *aidescs.offset(i as isize) == -(1 as libc::c_int) {
            if onull < 0 as libc::c_int {
                onull = open(
                    b"/dev/null\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0o2 as libc::c_int,
                );
                if onull < 0 as libc::c_int
                    || fcntl(
                        onull,
                        2 as libc::c_int,
                        fcntl(onull, 1 as libc::c_int, 0 as libc::c_int)
                            | 1 as libc::c_int,
                    ) < 0 as libc::c_int
                {
                    ierr = *__errno_location();
                    close(onull);
                    ferr = 1 as libc::c_int;
                    break;
                } else {
                    aipar_close[cpar_close as usize] = onull;
                    cpar_close += 1;
                    cpar_close;
                }
            }
            aichild_descs[i as usize] = onull;
        } else if *aidescs.offset(i as isize) != -(2 as libc::c_int)
            && *aidescs.offset(i as isize) != -(3 as libc::c_int)
        {
            aichild_descs[i as usize] = *aidescs.offset(i as isize);
        } else {
            let mut aipipe: [libc::c_int; 2] = [0; 2];
            if pipe(aipipe.as_mut_ptr()) < 0 as libc::c_int {
                ierr = *__errno_location();
                ferr = 1 as libc::c_int;
                break;
            } else {
                if *aidescs.offset(i as isize) == -(2 as libc::c_int) {
                    *aidescs.offset(i as isize) = aipipe[0 as libc::c_int as usize];
                    aichild_close[cchild_close
                        as usize] = aipipe[0 as libc::c_int as usize];
                    aichild_descs[i as usize] = aipipe[1 as libc::c_int as usize];
                    aipar_close[cpar_close as usize] = aipipe[1 as libc::c_int as usize];
                } else {
                    *aidescs.offset(i as isize) = aipipe[1 as libc::c_int as usize];
                    aichild_close[cchild_close
                        as usize] = aipipe[1 as libc::c_int as usize];
                    aichild_descs[i as usize] = aipipe[0 as libc::c_int as usize];
                    aipar_close[cpar_close as usize] = aipipe[0 as libc::c_int as usize];
                }
                cpar_close += 1;
                cpar_close;
                cchild_close += 1;
                cchild_close;
                if fcntl(
                    aipipe[0 as libc::c_int as usize],
                    2 as libc::c_int,
                    fcntl(
                        aipipe[0 as libc::c_int as usize],
                        1 as libc::c_int,
                        0 as libc::c_int,
                    ) | 1 as libc::c_int,
                ) < 0 as libc::c_int
                    || fcntl(
                        aipipe[1 as libc::c_int as usize],
                        2 as libc::c_int,
                        fcntl(
                            aipipe[1 as libc::c_int as usize],
                            1 as libc::c_int,
                            0 as libc::c_int,
                        ) | 1 as libc::c_int,
                    ) < 0 as libc::c_int
                {
                    ierr = *__errno_location();
                    ferr = 1 as libc::c_int;
                    break;
                }
            }
        }
        i += 1;
        i;
    }
    if ferr == 0 && iDebug & 0o400 as libc::c_int != 0 as libc::c_int {
        ulog(
            LOG_DEBUG_START,
            b"Forking %s\0" as *const u8 as *const libc::c_char,
            *pazargs.offset(0 as libc::c_int as isize),
        );
        i = 1 as libc::c_int;
        while !(*pazargs.offset(i as isize)).is_null() {
            ulog(
                LOG_DEBUG_CONTINUE,
                b" %s\0" as *const u8 as *const libc::c_char,
                *pazargs.offset(i as isize),
            );
            i += 1;
            i;
        }
        ulog(
            LOG_DEBUG_END,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        );
    }
    if ferr == 0 {
        iret = ixsfork();
        if iret < 0 as libc::c_int {
            ferr = 1 as libc::c_int;
            ierr = *__errno_location();
        }
    }
    if ferr != 0 {
        i = 0 as libc::c_int;
        while i < cchild_close {
            close(aichild_close[i as usize]);
            i += 1;
            i;
        }
        iret = -(1 as libc::c_int);
    }
    if iret != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < cpar_close {
            close(aipar_close[i as usize]);
            i += 1;
            i;
        }
        ubuffree(zshcmd);
        if fkeepenv == 0 {
            let mut pz: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            pz = azenv.as_mut_ptr();
            while !(*pz).is_null() {
                ubuffree(*pz);
                pz = pz.offset(1);
                pz;
            }
        }
        *__errno_location() = ierr;
        return iret;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if aichild_descs[i as usize] != i {
            dup2(aichild_descs[i as usize], i);
        }
        fcntl(
            i,
            2 as libc::c_int,
            fcntl(i, 1 as libc::c_int, 0 as libc::c_int) & !(1 as libc::c_int),
        );
        i += 1;
        i;
    }
    zcmd = *pazargs.offset(0 as libc::c_int as isize);
    let ref mut fresh0 = *pazargs.offset(0 as libc::c_int as isize);
    *fresh0 = strrchr(zcmd, '/' as i32);
    if (*pazargs.offset(0 as libc::c_int as isize)).is_null() {
        let ref mut fresh1 = *pazargs.offset(0 as libc::c_int as isize);
        *fresh1 = zcmd;
    } else {
        let ref mut fresh2 = *pazargs.offset(0 as libc::c_int as isize);
        *fresh2 = (*fresh2).offset(1);
        *fresh2;
    }
    if fkeepuid == 0 {
        setuid(getuid());
        setgid(getgid());
    } else {
        setreuid(geteuid(), -(1 as libc::c_int) as __uid_t);
        setregid(getegid(), -(1 as libc::c_int) as __gid_t);
    }
    if !zchdir.is_null() {
        chdir(zchdir);
    }
    if fnosigs != 0 {
        signal(
            1 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        signal(
            2 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
        signal(
            3 as libc::c_int,
            ::std::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    }
    execve(
        zcmd as *mut libc::c_char,
        pazargs as *mut *mut libc::c_char as *const *mut libc::c_char,
        pazenv as *const *mut libc::c_char,
    );
    if *__errno_location() == 8 as libc::c_int && fshell != 0 {
        let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut azshargs: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
        let ref mut fresh3 = *pazargs.offset(0 as libc::c_int as isize);
        *fresh3 = zcmd;
        zto = zshcmd;
        i = 0 as libc::c_int;
        while !(*pazargs.offset(i as isize)).is_null() {
            let mut zfrom: *const libc::c_char = 0 as *const libc::c_char;
            zfrom = *pazargs.offset(i as isize);
            while *zfrom as libc::c_int != '\0' as i32 {
                if *zfrom as libc::c_int != '/' as i32 {
                    let fresh4 = zto;
                    zto = zto.offset(1);
                    *fresh4 = '\\' as i32 as libc::c_char;
                }
                let fresh5 = zto;
                zto = zto.offset(1);
                *fresh5 = *zfrom;
                zfrom = zfrom.offset(1);
                zfrom;
            }
            let fresh6 = zto;
            zto = zto.offset(1);
            *fresh6 = ' ' as i32 as libc::c_char;
            i += 1;
            i;
        }
        *zto.offset(-(1 as libc::c_int as isize)) = '\0' as i32 as libc::c_char;
        azshargs[0 as libc::c_int
            as usize] = b"sh\0" as *const u8 as *const libc::c_char;
        azshargs[1 as libc::c_int
            as usize] = b"-c\0" as *const u8 as *const libc::c_char;
        azshargs[2 as libc::c_int as usize] = zshcmd;
        azshargs[3 as libc::c_int as usize] = 0 as *const libc::c_char;
        execve(
            b"/bin/sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            azshargs.as_mut_ptr() as *mut *mut libc::c_char as *const *mut libc::c_char,
            pazenv as *const *mut libc::c_char,
        );
    }
    _exit(1 as libc::c_int);
}
