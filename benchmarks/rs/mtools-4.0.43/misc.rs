use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn geteuid() -> __uid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getlogin() -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type mt_off_t = off_t;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub unsafe extern "C" fn printOom() {
    fprintf(stderr, b"Out of memory error\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn get_homedir() -> *mut libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut uid: uid_t = 0;
    let mut homedir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    homedir = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    if !homedir.is_null() {
        return homedir;
    }
    pw = 0 as *mut passwd;
    username = getenv(b"LOGNAME\0" as *const u8 as *const libc::c_char);
    if username.is_null() {
        username = getlogin();
    }
    if !username.is_null() {
        pw = getpwnam(username);
    }
    if pw.is_null() {
        uid = geteuid();
        pw = getpwuid(uid);
    }
    if !pw.is_null() {
        return (*pw).pw_dir;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn get_mcwd_file_name(mut file: *mut libc::c_char) {
    let mut mcwd_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut homedir: *const libc::c_char = 0 as *const libc::c_char;
    mcwd_path = getenv(b"MCWD\0" as *const u8 as *const libc::c_char);
    if mcwd_path.is_null() || *mcwd_path as libc::c_int == '\0' as i32 {
        homedir = get_homedir();
        if homedir.is_null() {
            homedir = b"/tmp\0" as *const u8 as *const libc::c_char;
        }
        strncpy(
            file,
            homedir,
            (4096 as libc::c_int - 6 as libc::c_int) as libc::c_ulong,
        );
        *file
            .offset(
                (4096 as libc::c_int - 6 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        strcat(file, b"/.mcwd\0" as *const u8 as *const libc::c_char);
    } else {
        strncpy(file, mcwd_path, 4096 as libc::c_int as libc::c_ulong);
        *file.offset(4096 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    };
}
pub unsafe extern "C" fn unlink_mcwd() {
    let mut file: [libc::c_char; 4097] = [0; 4097];
    get_mcwd_file_name(file.as_mut_ptr());
    unlink(file.as_mut_ptr());
}
pub unsafe extern "C" fn open_mcwd(mut mode: *const libc::c_char) -> *mut FILE {
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut file: [libc::c_char; 4097] = [0; 4097];
    let mut now: time_t = 0;
    get_mcwd_file_name(file.as_mut_ptr());
    if *mode as libc::c_int == 'r' as i32 {
        if stat(file.as_mut_ptr(), &mut sbuf) < 0 as libc::c_int {
            return 0 as *mut FILE;
        }
        getTimeNow(&mut now);
        if now - sbuf.st_mtim.tv_sec
            > (6 as libc::c_int * 60 as libc::c_int * 60 as libc::c_int) as libc::c_long
        {
            fprintf(
                stderr,
                b"Warning: \"%s\" is out of date, removing it\n\0" as *const u8
                    as *const libc::c_char,
                file.as_mut_ptr(),
            );
            unlink(file.as_mut_ptr());
            return 0 as *mut FILE;
        }
    }
    return fopen(file.as_mut_ptr(), mode);
}
pub unsafe extern "C" fn safe_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = malloc(size);
    if p.is_null() {
        printOom();
        exit(1 as libc::c_int);
    }
    return p;
}
pub unsafe extern "C" fn print_sector(
    mut message: *const libc::c_char,
    mut data: *mut libc::c_uchar,
    mut size: libc::c_int,
) {
    let mut col: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    printf(b"%s:\n\0" as *const u8 as *const libc::c_char, message);
    row = 0 as libc::c_int;
    while (row * 16 as libc::c_int) < size {
        printf(b"%03x  \0" as *const u8 as *const libc::c_char, row * 16 as libc::c_int);
        col = 0 as libc::c_int;
        while col < 16 as libc::c_int {
            printf(
                b"%02x \0" as *const u8 as *const libc::c_char,
                *data.offset((row * 16 as libc::c_int + col) as isize) as libc::c_int,
            );
            col += 1;
            col;
        }
        col = 0 as libc::c_int;
        while col < 16 as libc::c_int {
            if *(*__ctype_b_loc())
                .offset(
                    *data.offset((row * 16 as libc::c_int + col) as isize) as libc::c_int
                        as isize,
                ) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                printf(
                    b"%c\0" as *const u8 as *const libc::c_char,
                    *data.offset((row * 16 as libc::c_int + col) as isize) as libc::c_int,
                );
            } else {
                printf(b".\0" as *const u8 as *const libc::c_char);
            }
            col += 1;
            col;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        row += 1;
        row;
    }
}
pub unsafe extern "C" fn getTimeNow(mut now: *mut time_t) -> time_t {
    static mut haveTime: libc::c_int = 0 as libc::c_int;
    static mut sharedNow: time_t = 0;
    if haveTime == 0 {
        let mut source_date_epoch: *const libc::c_char = getenv(
            b"SOURCE_DATE_EPOCH\0" as *const u8 as *const libc::c_char,
        );
        if !source_date_epoch.is_null() {
            let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut epoch: time_t = strtol(
                source_date_epoch,
                &mut endptr,
                10 as libc::c_int,
            );
            *__errno_location() = 0 as libc::c_int;
            if endptr == source_date_epoch as *mut libc::c_char {
                fprintf(
                    stderr,
                    b"SOURCE_DATE_EPOCH \"%s\" invalid\n\0" as *const u8
                        as *const libc::c_char,
                    source_date_epoch,
                );
            } else if *__errno_location() != 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"SOURCE_DATE_EPOCH: strtoll: %s: %s\n\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                    source_date_epoch,
                );
            } else if *endptr as libc::c_int != '\0' as i32 {
                fprintf(
                    stderr,
                    b"SOURCE_DATE_EPOCH has trailing garbage \"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    endptr,
                );
            } else {
                sharedNow = epoch;
                haveTime = 1 as libc::c_int;
            }
        }
    }
    if haveTime == 0 {
        time(&mut sharedNow);
        haveTime = 1 as libc::c_int;
    }
    if !now.is_null() {
        *now = sharedNow;
    }
    return sharedNow;
}
pub unsafe extern "C" fn str_to_offset_with_end(
    mut str: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
) -> off_t {
    let mut s: libc::c_char = 0;
    let mut ofs: off_t = 0;
    *endp = 0 as *mut libc::c_char;
    ofs = strtol(str, endp, 0 as libc::c_int);
    s = **endp;
    if s as libc::c_int == 's' as i32 || s as libc::c_int == 'S' as i32 {
        ofs <<= 9 as libc::c_int;
    } else if s as libc::c_int == 'k' as i32 || s as libc::c_int == 'K' as i32 {
        ofs <<= 10 as libc::c_int;
    } else if s as libc::c_int == 'm' as i32 || s as libc::c_int == 'M' as i32 {
        ofs <<= 20 as libc::c_int;
    } else if s as libc::c_int == 'g' as i32 || s as libc::c_int == 'G' as i32 {
        ofs <<= 30 as libc::c_int;
    } else {
        return ofs
    }
    *endp = (*endp).offset(1);
    *endp;
    return ofs;
}
pub unsafe extern "C" fn str_to_off_with_end(
    mut str: *const libc::c_char,
    mut endp: *mut *mut libc::c_char,
) -> mt_off_t {
    let mut s: libc::c_char = 0;
    let mut siz: mt_off_t = 0;
    *endp = 0 as *mut libc::c_char;
    siz = strtol(str, endp, 0 as libc::c_int);
    s = **endp;
    if s as libc::c_int == 's' as i32 || s as libc::c_int == 'S' as i32 {
        siz <<= 9 as libc::c_int;
    } else if s as libc::c_int == 'k' as i32 || s as libc::c_int == 'K' as i32 {
        siz <<= 10 as libc::c_int;
    } else if s as libc::c_int == 'm' as i32 || s as libc::c_int == 'M' as i32 {
        siz <<= 20 as libc::c_int;
    } else if s as libc::c_int == 'g' as i32 || s as libc::c_int == 'G' as i32 {
        siz <<= 30 as libc::c_int;
    } else {
        return siz
    }
    *endp = (*endp).offset(1);
    *endp;
    return siz;
}
pub unsafe extern "C" fn str_to_offset(mut str: *mut libc::c_char) -> off_t {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ofs: off_t = str_to_offset_with_end(str, &mut end);
    if ofs <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as off_t;
    }
    if *end != 0 {
        return 0 as libc::c_int as off_t;
    }
    return ofs;
}
