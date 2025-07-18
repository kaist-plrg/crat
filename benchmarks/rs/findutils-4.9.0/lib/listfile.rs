use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn areadlinkat(fd: libc::c_int, filename: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn strmode(mode: mode_t, str: *mut libc::c_char);
    fn human_readable(
        _: uintmax_t,
        _: *mut libc::c_char,
        _: libc::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut libc::c_char;
    fn gnu_mbswidth(string: *const libc::c_char, flags: libc::c_int) -> libc::c_int;
    fn getuser(uid: uid_t) -> *mut libc::c_char;
    fn getgroup(gid: gid_t) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uintmax_t = libc::c_ulong;
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = libc::c_uint;
pub const human_B: C2RustUnnamed = 256;
pub const human_SI: C2RustUnnamed = 128;
pub const human_space_before_unit: C2RustUnnamed = 64;
pub const human_base_1024: C2RustUnnamed = 32;
pub const human_autoscale: C2RustUnnamed = 16;
pub const human_suppress_point_zero: C2RustUnnamed = 8;
pub const human_group_digits: C2RustUnnamed = 4;
pub const human_floor: C2RustUnnamed = 2;
pub const human_round_to_nearest: C2RustUnnamed = 1;
pub const human_ceiling: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> libc::c_uint {
    let mut __major: libc::c_uint = 0;
    __major = ((__dev & 0xfff00 as libc::c_uint as __dev_t) >> 8 as libc::c_int)
        as libc::c_uint;
    __major = (__major as libc::c_ulong
        | (__dev & 0xfffff00000000000 as libc::c_ulong) >> 32 as libc::c_int)
        as libc::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
}
static mut inode_number_width: libc::c_int = 9 as libc::c_int;
static mut block_size_width: libc::c_int = 6 as libc::c_int;
static mut nlink_width: libc::c_int = 3 as libc::c_int;
static mut owner_width: libc::c_int = 8 as libc::c_int;
static mut group_width: libc::c_int = 8 as libc::c_int;
static mut major_device_number_width: libc::c_int = 3 as libc::c_int;
static mut minor_device_number_width: libc::c_int = 3 as libc::c_int;
static mut file_size_width: libc::c_int = 8 as libc::c_int;
unsafe extern "C" fn print_num(
    mut stream: *mut FILE,
    mut num: libc::c_ulong,
    mut width: *mut libc::c_int,
) -> bool {
    let chars_out: libc::c_int = fprintf(
        stream,
        b"%*lu\0" as *const u8 as *const libc::c_char,
        *width,
        num,
    );
    if chars_out >= 0 as libc::c_int {
        if *width < chars_out {
            *width = chars_out;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn list_file(
    mut name: *const libc::c_char,
    mut dir_fd: libc::c_int,
    mut relname: *const libc::c_char,
    mut statp: *const stat,
    mut current_time: time_t,
    mut output_block_size: libc::c_int,
    mut literal_control_chars: libc::c_int,
    mut stream: *mut FILE,
) {
    let mut modebuf: [libc::c_char; 12] = [0; 12];
    let mut when_local: *const tm = 0 as *const tm;
    let mut user_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut group_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut hbuf: [libc::c_char; 652] = [0; 652];
    let mut output_good: bool = 1 as libc::c_int != 0;
    let mut chars_out: libc::c_int = 0;
    let mut failed_at: libc::c_int = 0 as libc::c_int;
    strmode((*statp).st_mode, modebuf.as_mut_ptr());
    chars_out = fprintf(
        stream,
        b"%*s\0" as *const u8 as *const libc::c_char,
        inode_number_width,
        human_readable(
            (*statp).st_ino,
            hbuf.as_mut_ptr(),
            human_ceiling as libc::c_int,
            1 as libc::c_uint as uintmax_t,
            1 as libc::c_uint as uintmax_t,
        ),
    );
    if chars_out < 0 as libc::c_int {
        output_good = 0 as libc::c_int != 0;
        failed_at = 100 as libc::c_int;
    } else if chars_out > inode_number_width {
        inode_number_width = chars_out;
    }
    if output_good {
        if -(1 as libc::c_int) == putc(' ' as i32, stream) {
            output_good = 0 as libc::c_int != 0;
            failed_at = 150 as libc::c_int;
        }
        chars_out = fprintf(
            stream,
            b"%*s\0" as *const u8 as *const libc::c_char,
            block_size_width,
            human_readable(
                (*statp).st_blocks as uintmax_t,
                hbuf.as_mut_ptr(),
                human_ceiling as libc::c_int,
                512 as libc::c_int as uintmax_t,
                output_block_size as uintmax_t,
            ),
        );
        if chars_out < 0 as libc::c_int {
            output_good = 0 as libc::c_int != 0;
            failed_at = 200 as libc::c_int;
        } else if chars_out > block_size_width {
            block_size_width = chars_out;
        }
    }
    if output_good {
        if -(1 as libc::c_int) == putc(' ' as i32, stream) {
            output_good = 0 as libc::c_int != 0;
            failed_at = 250 as libc::c_int;
        }
    }
    if output_good {
        if fputs(modebuf.as_mut_ptr(), stream) < 0 as libc::c_int {
            output_good = 0 as libc::c_int != 0;
            failed_at = 275 as libc::c_int;
        }
    }
    if output_good {
        chars_out = fprintf(
            stream,
            b"%*lu\0" as *const u8 as *const libc::c_char,
            nlink_width,
            (*statp).st_nlink,
        );
        if chars_out < 0 as libc::c_int {
            output_good = 0 as libc::c_int != 0;
            failed_at = 300 as libc::c_int;
        } else if chars_out > nlink_width {
            nlink_width = chars_out;
        }
    }
    if output_good {
        if -(1 as libc::c_int) == putc(' ' as i32, stream) {
            output_good = 0 as libc::c_int != 0;
            failed_at = 250 as libc::c_int;
        }
        user_name = getuser((*statp).st_uid);
        if !user_name.is_null() {
            let mut len: libc::c_int = gnu_mbswidth(user_name, 0 as libc::c_int);
            if len > owner_width {
                owner_width = len;
            }
            output_good = fprintf(
                stream,
                b"%-*s \0" as *const u8 as *const libc::c_char,
                owner_width,
                user_name,
            ) >= 0 as libc::c_int;
            if !output_good {
                failed_at = 400 as libc::c_int;
            }
        } else {
            chars_out = fprintf(
                stream,
                b"%-8lu \0" as *const u8 as *const libc::c_char,
                (*statp).st_uid as libc::c_ulong,
            );
            if chars_out > owner_width {
                owner_width = chars_out;
            }
            output_good = chars_out > 0 as libc::c_int;
            if !output_good {
                failed_at = 450 as libc::c_int;
            }
        }
    }
    if output_good {
        group_name = getgroup((*statp).st_gid);
        if !group_name.is_null() {
            let mut len_0: libc::c_int = gnu_mbswidth(group_name, 0 as libc::c_int);
            if len_0 > group_width {
                group_width = len_0;
            }
            output_good = fprintf(
                stream,
                b"%-*s \0" as *const u8 as *const libc::c_char,
                group_width,
                group_name,
            ) >= 0 as libc::c_int;
            if !output_good {
                failed_at = 500 as libc::c_int;
            }
        } else {
            chars_out = fprintf(
                stream,
                b"%-*lu\0" as *const u8 as *const libc::c_char,
                group_width,
                (*statp).st_gid as libc::c_ulong,
            );
            if chars_out > group_width {
                group_width = chars_out;
            }
            output_good = chars_out >= 0 as libc::c_int;
            if output_good {
                if -(1 as libc::c_int) == putc(' ' as i32, stream) {
                    output_good = 0 as libc::c_int != 0;
                    failed_at = 525 as libc::c_int;
                }
            } else if !output_good {
                failed_at = 550 as libc::c_int;
            }
        }
    }
    if output_good {
        if (*statp).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
            || (*statp).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o60000 as libc::c_int as libc::c_uint
        {
            if !print_num(
                stream,
                gnu_dev_major((*statp).st_rdev) as libc::c_ulong,
                &mut major_device_number_width,
            ) {
                output_good = 0 as libc::c_int != 0;
                failed_at = 600 as libc::c_int;
            }
            if output_good {
                if fprintf(stream, b", \0" as *const u8 as *const libc::c_char)
                    < 0 as libc::c_int
                {
                    output_good = 0 as libc::c_int != 0;
                    failed_at = 625 as libc::c_int;
                }
            }
            if output_good {
                if !print_num(
                    stream,
                    gnu_dev_minor((*statp).st_rdev) as libc::c_ulong,
                    &mut minor_device_number_width,
                ) {
                    output_good = 0 as libc::c_int != 0;
                    failed_at = 650 as libc::c_int;
                }
            }
        } else {
            let blocksize: libc::c_int = if output_block_size < 0 as libc::c_int {
                output_block_size
            } else {
                1 as libc::c_int
            };
            chars_out = fprintf(
                stream,
                b"%*s\0" as *const u8 as *const libc::c_char,
                file_size_width,
                human_readable(
                    (*statp).st_size as uintmax_t,
                    hbuf.as_mut_ptr(),
                    human_ceiling as libc::c_int,
                    1 as libc::c_int as uintmax_t,
                    blocksize as uintmax_t,
                ),
            );
            if chars_out < 0 as libc::c_int {
                output_good = 0 as libc::c_int != 0;
                failed_at = 800 as libc::c_int;
            } else if chars_out > file_size_width {
                file_size_width = chars_out;
            }
        }
    }
    if output_good {
        if -(1 as libc::c_int) == putc(' ' as i32, stream) {
            output_good = 0 as libc::c_int != 0;
            failed_at = 850 as libc::c_int;
        }
    }
    if output_good {
        when_local = localtime(&(*statp).st_mtim.tv_sec);
        if !when_local.is_null() {
            let mut init_bigbuf: [libc::c_char; 256] = [0; 256];
            let mut buf: *mut libc::c_char = init_bigbuf.as_mut_ptr();
            let mut bufsize: size_t = ::std::mem::size_of::<[libc::c_char; 256]>()
                as libc::c_ulong;
            let mut fmt: *const libc::c_char = if current_time
                - (6 as libc::c_int * 30 as libc::c_int * 24 as libc::c_int
                    * 60 as libc::c_int * 60 as libc::c_int) as libc::c_long
                <= (*statp).st_mtim.tv_sec
                && (*statp).st_mtim.tv_sec
                    <= current_time
                        + (60 as libc::c_int * 60 as libc::c_int) as libc::c_long
            {
                b"%b %e %H:%M\0" as *const u8 as *const libc::c_char
            } else {
                b"%b %e  %Y\0" as *const u8 as *const libc::c_char
            };
            while strftime(buf, bufsize, fmt, when_local) == 0 {
                bufsize = (bufsize as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                let mut fresh0 = ::std::vec::from_elem(0, bufsize as usize);
                buf = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
            }
            if fprintf(stream, b"%s \0" as *const u8 as *const libc::c_char, buf)
                < 0 as libc::c_int
            {
                output_good = 0 as libc::c_int != 0;
                failed_at = 900 as libc::c_int;
            }
        } else {
            let mut width: libc::c_int = 12 as libc::c_int;
            if (*statp).st_mtim.tv_sec < 0 as libc::c_int as libc::c_long {
                let mut num: *const libc::c_char = human_readable(
                    ((*statp).st_mtim.tv_sec as uintmax_t).wrapping_neg(),
                    hbuf.as_mut_ptr(),
                    human_ceiling as libc::c_int,
                    1 as libc::c_int as uintmax_t,
                    1 as libc::c_int as uintmax_t,
                );
                let mut sign_width: libc::c_int = (width as libc::c_ulong)
                    .wrapping_sub(strlen(num)) as libc::c_int;
                if fprintf(
                    stream,
                    b"%*s%s \0" as *const u8 as *const libc::c_char,
                    if sign_width < 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        sign_width
                    },
                    b"-\0" as *const u8 as *const libc::c_char,
                    num,
                ) < 0 as libc::c_int
                {
                    output_good = 0 as libc::c_int != 0;
                    failed_at = 1000 as libc::c_int;
                }
            } else if fprintf(
                stream,
                b"%*s \0" as *const u8 as *const libc::c_char,
                width,
                human_readable(
                    (*statp).st_mtim.tv_sec as uintmax_t,
                    hbuf.as_mut_ptr(),
                    human_ceiling as libc::c_int,
                    1 as libc::c_int as uintmax_t,
                    1 as libc::c_int as uintmax_t,
                ),
            ) < 0 as libc::c_int
            {
                output_good = 0 as libc::c_int != 0;
                failed_at = 1100 as libc::c_int;
            }
        }
    }
    if output_good {
        output_good = print_name(name, stream, literal_control_chars);
        if !output_good {
            failed_at = 1200 as libc::c_int;
        }
    }
    if output_good {
        if (*statp).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            let mut linkname: *mut libc::c_char = areadlinkat(dir_fd, relname);
            if !linkname.is_null() {
                if fputs(b" -> \0" as *const u8 as *const libc::c_char, stream)
                    < 0 as libc::c_int
                {
                    output_good = 0 as libc::c_int != 0;
                    failed_at = 1300 as libc::c_int;
                }
                if output_good {
                    output_good = print_name(linkname, stream, literal_control_chars);
                    if !output_good {
                        failed_at = 1350 as libc::c_int;
                    }
                }
            } else {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
            rpl_free(linkname as *mut libc::c_void);
        }
        if output_good {
            if -(1 as libc::c_int) == putc('\n' as i32, stream) {
                output_good = 0 as libc::c_int != 0;
                if !output_good {
                    failed_at = 1400 as libc::c_int;
                }
            }
        }
    }
    if !output_good {
        if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to write output (at stage %d)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                failed_at,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to write output (at stage %d)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                failed_at,
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn print_name_without_quoting(
    mut p: *const libc::c_char,
    mut stream: *mut FILE,
) -> bool {
    return fprintf(stream, b"%s\0" as *const u8 as *const libc::c_char, p)
        >= 0 as libc::c_int;
}
unsafe extern "C" fn print_name_with_quoting(
    mut p: *const libc::c_char,
    mut stream: *mut FILE,
) -> bool {
    let mut c: libc::c_uchar = 0;
    loop {
        let fresh1 = p;
        p = p.offset(1);
        c = *fresh1 as libc::c_uchar;
        if !(c as libc::c_int != '\0' as i32) {
            break;
        }
        let mut fprintf_result: libc::c_int = -(1 as libc::c_int);
        match c as libc::c_int {
            92 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\\\\0" as *const u8 as *const libc::c_char,
                );
            }
            10 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\n\0" as *const u8 as *const libc::c_char,
                );
            }
            8 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\b\0" as *const u8 as *const libc::c_char,
                );
            }
            13 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\r\0" as *const u8 as *const libc::c_char,
                );
            }
            9 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\t\0" as *const u8 as *const libc::c_char,
                );
            }
            12 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\f\0" as *const u8 as *const libc::c_char,
                );
            }
            32 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\ \0" as *const u8 as *const libc::c_char,
                );
            }
            34 => {
                fprintf_result = fprintf(
                    stream,
                    b"\\\"\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                if c as libc::c_int > 0o40 as libc::c_int
                    && (c as libc::c_int) < 0o177 as libc::c_int
                {
                    if -(1 as libc::c_int) == putc(c as libc::c_int, stream) {
                        return 0 as libc::c_int != 0;
                    }
                    fprintf_result = 1 as libc::c_int;
                } else {
                    fprintf_result = fprintf(
                        stream,
                        b"\\%03o\0" as *const u8 as *const libc::c_char,
                        c as libc::c_uint,
                    );
                }
            }
        }
        if fprintf_result < 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn print_name(
    mut p: *const libc::c_char,
    mut stream: *mut FILE,
    mut literal_control_chars: libc::c_int,
) -> bool {
    if literal_control_chars != 0 {
        return print_name_without_quoting(p, stream)
    } else {
        return print_name_with_quoting(p, stream)
    };
}
