use ::libc;
extern "C" {
    pub type __dirstream;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
    fn xmalloc(_: size_t) -> pointer;
    fn xfree(_: pointer);
    fn zsysdep_login_name() -> *const libc::c_char;
    fn zsysdep_local_file_cwd(
        zname: *const libc::c_char,
        zpubdir: *const libc::c_char,
        pfbadname: *mut boolean,
    ) -> *mut libc::c_char;
    fn zsysdep_in_dir(
        zdir: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn zsappend3(
        zdir1: *const libc::c_char,
        zdir2: *const libc::c_char,
        zfile: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn getuid() -> __uid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type size_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub static mut picksb_rcsid: [libc::c_char; 51] = unsafe {
    *::std::mem::transmute::<
        &[u8; 51],
        &[libc::c_char; 51],
    >(b"$Id: picksb.c,v 1.13 2002/03/05 19:10:42 ian Rel $\0")
};
static mut qStopdir: *mut DIR = 0 as *const DIR as *mut DIR;
static mut zStopdir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut qSsysdir: *mut DIR = 0 as *const DIR as *mut DIR;
static mut zSsysdir: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn fsysdep_uupick_init(
    mut zsystem: *const libc::c_char,
    mut zpubdir: *const libc::c_char,
) -> boolean {
    let mut zuser: *const libc::c_char = 0 as *const libc::c_char;
    zuser = zsysdep_login_name();
    zStopdir = xmalloc(
        (strlen(zpubdir))
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_add(strlen(zuser)),
    ) as *mut libc::c_char;
    sprintf(
        zStopdir,
        b"%s/receive/%s\0" as *const u8 as *const libc::c_char,
        zpubdir,
        zuser,
    );
    qStopdir = opendir(zStopdir);
    if qStopdir.is_null() && *__errno_location() != 2 as libc::c_int {
        ulog(
            LOG_ERROR,
            b"opendir (%s): %s\0" as *const u8 as *const libc::c_char,
            zStopdir,
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    qSsysdir = 0 as *mut DIR;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_uupick(
    mut zsysarg: *const libc::c_char,
    mut zpubdir: *const libc::c_char,
    mut pzfrom: *mut *mut libc::c_char,
    mut pzfull: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut qentry: *mut dirent = 0 as *mut dirent;
    loop {
        while qSsysdir.is_null() {
            let mut zsystem: *const libc::c_char = 0 as *const libc::c_char;
            let mut zdir: *mut libc::c_char = 0 as *mut libc::c_char;
            if qStopdir.is_null() {
                return 0 as *mut libc::c_char;
            }
            if !zsysarg.is_null() {
                closedir(qStopdir);
                qStopdir = 0 as *mut DIR;
                zsystem = zsysarg;
            } else {
                loop {
                    qentry = readdir(qStopdir);
                    if qentry.is_null() {
                        closedir(qStopdir);
                        qStopdir = 0 as *mut DIR;
                        return 0 as *mut libc::c_char;
                    }
                    if !(strcmp(
                        ((*qentry).d_name).as_mut_ptr(),
                        b".\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                        || strcmp(
                            ((*qentry).d_name).as_mut_ptr(),
                            b"..\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int)
                    {
                        break;
                    }
                }
                zsystem = ((*qentry).d_name).as_mut_ptr();
            }
            zdir = zbufalc(
                (strlen(zStopdir))
                    .wrapping_add(strlen(zsystem))
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                    ),
            );
            sprintf(
                zdir,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                zStopdir,
                zsystem,
            );
            qSsysdir = opendir(zdir);
            if qSsysdir.is_null() {
                if *__errno_location() != 2 as libc::c_int
                    && *__errno_location() != 20 as libc::c_int
                {
                    ulog(
                        LOG_ERROR,
                        b"opendir (%s): %s\0" as *const u8 as *const libc::c_char,
                        zdir,
                        strerror(*__errno_location()),
                    );
                }
            } else {
                ubuffree(zSsysdir);
                zSsysdir = zbufcpy(zsystem);
            }
            ubuffree(zdir);
        }
        qentry = readdir(qSsysdir);
        if qentry.is_null() {
            closedir(qSsysdir);
            qSsysdir = 0 as *mut DIR;
        } else {
            if strcmp(
                ((*qentry).d_name).as_mut_ptr(),
                b".\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    ((*qentry).d_name).as_mut_ptr(),
                    b"..\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                continue;
            }
            *pzfrom = zbufcpy(zSsysdir);
            *pzfull = zsappend3(zStopdir, zSsysdir, ((*qentry).d_name).as_mut_ptr());
            return zbufcpy(((*qentry).d_name).as_mut_ptr());
        }
    };
}
pub unsafe extern "C" fn fsysdep_uupick_free(
    mut zsystem: *const libc::c_char,
    mut zpubdir: *const libc::c_char,
) -> boolean {
    xfree(zStopdir as pointer);
    if !qStopdir.is_null() {
        closedir(qStopdir);
        qStopdir = 0 as *mut DIR;
    }
    ubuffree(zSsysdir);
    zSsysdir = 0 as *mut libc::c_char;
    if !qSsysdir.is_null() {
        closedir(qSsysdir);
        qSsysdir = 0 as *mut DIR;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn zsysdep_uupick_local_file(
    mut zfile: *const libc::c_char,
    mut pfbadname: *mut boolean,
) -> *mut libc::c_char {
    let mut q: *mut passwd = 0 as *mut passwd;
    if !pfbadname.is_null() {
        *pfbadname = 0 as libc::c_int;
    }
    if *zfile.offset(0 as libc::c_int as isize) as libc::c_int != '~' as i32
        || *zfile.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32
            && *zfile.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        return zsysdep_local_file_cwd(
            zfile,
            0 as *mut libc::c_void as *const libc::c_char,
            pfbadname,
        );
    }
    q = getpwuid(getuid());
    if q.is_null() {
        ulog(
            LOG_ERROR,
            b"Can't get home directory\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    if *zfile.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return zbufcpy((*q).pw_dir);
    }
    return zsysdep_in_dir((*q).pw_dir, zfile.offset(2 as libc::c_int as isize));
}
