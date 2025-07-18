use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type off_t = __off_t;
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
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
pub unsafe extern "C" fn fread_file(
    mut stream: *mut FILE,
    mut flags: libc::c_int,
    mut length: *mut size_t,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alloc: size_t = 8192 as libc::c_int as size_t;
    let mut st: stat = stat {
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
    if fstat(fileno(stream), &mut st) >= 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
    {
        let mut pos: off_t = ftello(stream);
        if pos >= 0 as libc::c_int as libc::c_long && pos < st.st_size {
            let mut alloc_off: off_t = st.st_size - pos;
            if (9223372036854775807 as libc::c_long - 1 as libc::c_int as libc::c_long)
                < alloc_off
            {
                *__errno_location() = 12 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            alloc = (alloc_off + 1 as libc::c_int as libc::c_long) as size_t;
        }
    }
    buf = malloc(alloc) as *mut libc::c_char;
    if buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut save_errno: libc::c_int = 0;
    loop {
        let mut requested: size_t = alloc.wrapping_sub(size);
        let mut count: size_t = fread(
            buf.offset(size as isize) as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            requested,
            stream,
        );
        size = (size as libc::c_ulong).wrapping_add(count) as size_t as size_t;
        if count != requested {
            save_errno = *__errno_location();
            if ferror(stream) != 0 {
                break;
            }
            if size < alloc.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                if flags & 0x2 as libc::c_int != 0 {
                    let mut smaller_buf: *mut libc::c_char = malloc(
                        size.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    if smaller_buf.is_null() {
                        explicit_bzero(
                            buf.offset(size as isize) as *mut libc::c_void,
                            alloc.wrapping_sub(size),
                        );
                    } else {
                        memcpy(
                            smaller_buf as *mut libc::c_void,
                            buf as *const libc::c_void,
                            size,
                        );
                        explicit_bzero(buf as *mut libc::c_void, alloc);
                        rpl_free(buf as *mut libc::c_void);
                        buf = smaller_buf;
                    }
                } else {
                    let mut smaller_buf_0: *mut libc::c_char = realloc(
                        buf as *mut libc::c_void,
                        size.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                    if !smaller_buf_0.is_null() {
                        buf = smaller_buf_0;
                    }
                }
            }
            *buf.offset(size as isize) = '\0' as i32 as libc::c_char;
            *length = size;
            return buf;
        } else {
            let mut new_buf: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut save_alloc: size_t = alloc;
            if alloc == 9223372036854775807 as libc::c_long as libc::c_ulong {
                save_errno = 12 as libc::c_int;
                break;
            } else {
                if alloc
                    < (9223372036854775807 as libc::c_long as libc::c_ulong)
                        .wrapping_sub(
                            alloc.wrapping_div(2 as libc::c_int as libc::c_ulong),
                        )
                {
                    alloc = alloc
                        .wrapping_add(
                            alloc.wrapping_div(2 as libc::c_int as libc::c_ulong),
                        );
                } else {
                    alloc = 9223372036854775807 as libc::c_long as size_t;
                }
                if flags & 0x2 as libc::c_int != 0 {
                    new_buf = malloc(alloc) as *mut libc::c_char;
                    if new_buf.is_null() {
                        save_errno = *__errno_location();
                        break;
                    } else {
                        memcpy(
                            new_buf as *mut libc::c_void,
                            buf as *const libc::c_void,
                            save_alloc,
                        );
                        explicit_bzero(buf as *mut libc::c_void, save_alloc);
                        rpl_free(buf as *mut libc::c_void);
                    }
                } else {
                    new_buf = realloc(buf as *mut libc::c_void, alloc)
                        as *mut libc::c_char;
                    if new_buf.is_null() {
                        save_errno = *__errno_location();
                        break;
                    }
                }
                buf = new_buf;
            }
        }
    }
    if flags & 0x2 as libc::c_int != 0 {
        explicit_bzero(buf as *mut libc::c_void, alloc);
    }
    rpl_free(buf as *mut libc::c_void);
    *__errno_location() = save_errno;
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn read_file(
    mut filename: *const libc::c_char,
    mut flags: libc::c_int,
    mut length: *mut size_t,
) -> *mut libc::c_char {
    let mut mode: *const libc::c_char = if flags & 0x1 as libc::c_int != 0 {
        b"rbe\0" as *const u8 as *const libc::c_char
    } else {
        b"re\0" as *const u8 as *const libc::c_char
    };
    let mut stream: *mut FILE = fopen(filename, mode);
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    if stream.is_null() {
        return 0 as *mut libc::c_char;
    }
    if flags & 0x2 as libc::c_int != 0 {
        setvbuf(
            stream,
            0 as *mut libc::c_char,
            2 as libc::c_int,
            0 as libc::c_int as size_t,
        );
    }
    out = fread_file(stream, flags, length);
    if fclose(stream) != 0 as libc::c_int {
        if !out.is_null() {
            if flags & 0x2 as libc::c_int != 0 {
                explicit_bzero(out as *mut libc::c_void, *length);
            }
            rpl_free(out as *mut libc::c_void);
        }
        return 0 as *mut libc::c_char;
    }
    return out;
}
