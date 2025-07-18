use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const libc::c_char,
    pub drive: libc::c_char,
    pub fat_bits: libc::c_int,
    pub mode: libc::c_int,
    pub tracks: libc::c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: libc::c_uint,
    pub offset: off_t,
    pub partition: libc::c_uint,
    pub misc_flags: libc::c_uint,
    pub ssize: uint8_t,
    pub use_2m: libc::c_uint,
    pub precmd: *mut libc::c_char,
    pub file_nr: libc::c_int,
    pub blocksize: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data_map: *const libc::c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut libc::c_char,
    pub cfg_filename: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floppy_struct {
    pub size: libc::c_uint,
    pub sect: libc::c_uint,
    pub head: libc::c_uint,
    pub track: libc::c_uint,
    pub stretch: libc::c_uint,
    pub gap: libc::c_uchar,
    pub rate: libc::c_uchar,
    pub spec1: libc::c_uchar,
    pub fmt_gap: libc::c_uchar,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floppy_raw_cmd {
    pub flags: libc::c_uint,
    pub data: *mut libc::c_void,
    pub kernel_data: *mut libc::c_char,
    pub next: *mut floppy_raw_cmd,
    pub length: libc::c_long,
    pub phys_length: libc::c_long,
    pub buffer_length: libc::c_int,
    pub rate: libc::c_uchar,
    pub cmd_count: libc::c_uchar,
    pub cmd: [libc::c_uchar; 16],
    pub reply_count: libc::c_uchar,
    pub reply: [libc::c_uchar; 16],
    pub track: libc::c_int,
    pub resultcode: libc::c_int,
    pub reserved1: libc::c_int,
    pub reserved2: libc::c_int,
}
pub type RawRequest_t = floppy_raw_cmd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hd_geometry {
    pub heads: libc::c_uchar,
    pub sectors: libc::c_uchar,
    pub cylinders: libc::c_ushort,
    pub start: libc::c_ulong,
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
unsafe extern "C" fn compare_geom(
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> libc::c_int {
    if !orig_dev.is_null() && (*orig_dev).misc_flags & 0x10 as libc::c_uint != 0 {
        return 0 as libc::c_int;
    }
    if orig_dev.is_null() || (*orig_dev).tracks == 0 || dev.is_null()
        || (*dev).tracks == 0
    {
        return 0 as libc::c_int;
    }
    return ((*orig_dev).tracks != (*dev).tracks
        || (*orig_dev).heads as libc::c_int != (*dev).heads as libc::c_int
        || (*orig_dev).sectors as libc::c_int != (*dev).sectors as libc::c_int)
        as libc::c_int;
}
static mut error_msg: [*const libc::c_char; 22] = [
    b"Missing Data Address Mark\0" as *const u8 as *const libc::c_char,
    b"Bad cylinder\0" as *const u8 as *const libc::c_char,
    b"Scan not satisfied\0" as *const u8 as *const libc::c_char,
    b"Scan equal hit\0" as *const u8 as *const libc::c_char,
    b"Wrong cylinder\0" as *const u8 as *const libc::c_char,
    b"CRC error in data field\0" as *const u8 as *const libc::c_char,
    b"Control Mark = deleted\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"Missing Address Mark\0" as *const u8 as *const libc::c_char,
    b"Write Protect\0" as *const u8 as *const libc::c_char,
    b"No Data - unreadable\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"OverRun\0" as *const u8 as *const libc::c_char,
    b"CRC error in data or address\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"End Of Cylinder\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"Not ready\0" as *const u8 as *const libc::c_char,
    b"Equipment check error\0" as *const u8 as *const libc::c_char,
    b"Seek end\0" as *const u8 as *const libc::c_char,
];
#[inline]
unsafe extern "C" fn print_message(
    mut raw_cmd: *mut RawRequest_t,
    mut message: *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    if message.is_null() {
        return;
    }
    fprintf(stderr, b"   \0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*raw_cmd).cmd_count as libc::c_int {
        fprintf(
            stderr,
            b"%2.2x \0" as *const u8 as *const libc::c_char,
            (*raw_cmd).cmd[i as usize] as libc::c_int,
        );
        i += 1;
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*raw_cmd).reply_count as libc::c_int {
        fprintf(
            stderr,
            b"%2.2x \0" as *const u8 as *const libc::c_char,
            (*raw_cmd).reply[i as usize] as libc::c_int,
        );
        i += 1;
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    code = (((*raw_cmd).reply[0 as libc::c_int as usize] as libc::c_int)
        << 16 as libc::c_int)
        + (((*raw_cmd).reply[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)
        + (*raw_cmd).reply[2 as libc::c_int as usize] as libc::c_int;
    i = 0 as libc::c_int;
    while i < 22 as libc::c_int {
        if code & (1 as libc::c_int) << i != 0 && !(error_msg[i as usize]).is_null() {
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                error_msg[i as usize],
            );
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn send_one_cmd(
    mut fd: libc::c_int,
    mut raw_cmd: *mut RawRequest_t,
    mut message: *const libc::c_char,
) -> libc::c_int {
    if ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((2 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x58 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        raw_cmd,
    ) >= 0 as libc::c_int
    {
        if ((*raw_cmd).reply_count as libc::c_int) < 7 as libc::c_int {
            fprintf(
                stderr,
                b"Short reply from FDC\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    match *__errno_location() {
        16 => {
            fprintf(
                stderr,
                b"FDC busy, sleeping for a second\n\0" as *const u8
                    as *const libc::c_char,
            );
            sleep(1 as libc::c_int as libc::c_uint);
            return 1 as libc::c_int;
        }
        5 => {
            fprintf(
                stderr,
                b"resetting controller\n\0" as *const u8 as *const libc::c_char,
            );
            if ioctl(
                fd,
                ((0 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                        + 14 as libc::c_int
                    | ((2 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                        as libc::c_uint
                    | ((0x54 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                    | ((0 as libc::c_int)
                        << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                        as libc::c_uint) as libc::c_ulong,
                2 as libc::c_int,
            ) < 0 as libc::c_int
            {
                perror(b"reset\0" as *const u8 as *const libc::c_char);
                return -(1 as libc::c_int);
            }
            return 1 as libc::c_int;
        }
        _ => {
            perror(message);
            return -(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn analyze_one_reply(
    mut raw_cmd: *mut RawRequest_t,
    mut bytes: *mut libc::c_int,
    mut do_print: libc::c_int,
) -> libc::c_int {
    if (*raw_cmd).reply_count as libc::c_int == 7 as libc::c_int {
        let mut end: libc::c_int = 0;
        if (*raw_cmd).reply[3 as libc::c_int as usize] as libc::c_int
            != (*raw_cmd).cmd[2 as libc::c_int as usize] as libc::c_int
        {
            end = (*raw_cmd).cmd[6 as libc::c_int as usize] as libc::c_int
                + 1 as libc::c_int;
        } else {
            end = (*raw_cmd).reply[5 as libc::c_int as usize] as libc::c_int;
        }
        *bytes = end - (*raw_cmd).cmd[4 as libc::c_int as usize] as libc::c_int;
        *bytes = *bytes
            << 7 as libc::c_int
                + (*raw_cmd).cmd[5 as libc::c_int as usize] as libc::c_int;
    } else {
        *bytes = 0 as libc::c_int;
    }
    match (*raw_cmd).reply[0 as libc::c_int as usize] as libc::c_int
        & 0xc0 as libc::c_int
    {
        64 => {
            if (*raw_cmd).reply[0 as libc::c_int as usize] as libc::c_int
                & 0x38 as libc::c_int == 0 as libc::c_int
                && (*raw_cmd).reply[1 as libc::c_int as usize] as libc::c_int
                    == 0x80 as libc::c_int
                && (*raw_cmd).reply[2 as libc::c_int as usize] as libc::c_int
                    == 0 as libc::c_int
            {
                *bytes
                    += (1 as libc::c_int)
                        << 7 as libc::c_int
                            + (*raw_cmd).cmd[5 as libc::c_int as usize] as libc::c_int;
            } else {
                if (*raw_cmd).reply[1 as libc::c_int as usize] as libc::c_int
                    & 0x2 as libc::c_int != 0
                {
                    *bytes = 0 as libc::c_int;
                    fprintf(
                        stderr,
                        b"This disk is write protected\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                if *bytes == 0 && do_print != 0 {
                    print_message(raw_cmd, b"\0" as *const u8 as *const libc::c_char);
                }
                return -(1 as libc::c_int);
            }
        }
        128 => {
            *bytes = 0 as libc::c_int;
            fprintf(
                stderr,
                b"invalid command given\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        192 => {
            *bytes = 0 as libc::c_int;
            fprintf(
                stderr,
                b"abnormal termination caused by polling\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        _ => {}
    }
    if (*raw_cmd).flags & 0x100 as libc::c_int as libc::c_uint != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub static mut const_devices: [device; 5] = [
    {
        let mut init = device {
            name: b"/dev/fd0\0" as *const u8 as *const libc::c_char,
            drive: 'A' as i32 as libc::c_char,
            fat_bits: 0 as libc::c_int,
            mode: 0 as libc::c_int,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as uint16_t,
            sectors: 18 as libc::c_int as uint16_t,
            hidden: 0 as libc::c_int as libc::c_uint,
            offset: 0 as libc::c_long,
            partition: 0 as libc::c_int as libc::c_uint,
            misc_flags: 0x10 as libc::c_uint,
            ssize: 0x2 as libc::c_int as uint8_t,
            use_2m: 0 as libc::c_int as libc::c_uint,
            precmd: 0 as *const libc::c_char as *mut libc::c_char,
            file_nr: 0 as libc::c_int,
            blocksize: 0 as libc::c_int as libc::c_uint,
            codepage: 0 as libc::c_int as libc::c_uint,
            data_map: 0 as *const libc::c_char,
            tot_sectors: 0 as libc::c_int as uint32_t,
            sector_size: 0 as libc::c_int as uint16_t,
            postcmd: 0 as *const libc::c_char as *mut libc::c_char,
            cfg_filename: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = device {
            name: b"/dev/fd1\0" as *const u8 as *const libc::c_char,
            drive: 'B' as i32 as libc::c_char,
            fat_bits: 0 as libc::c_int,
            mode: 0 as libc::c_int,
            tracks: 0 as libc::c_int as libc::c_uint,
            heads: 0 as libc::c_int as uint16_t,
            sectors: 0 as libc::c_int as uint16_t,
            hidden: 0 as libc::c_int as libc::c_uint,
            offset: 0 as libc::c_long,
            partition: 0 as libc::c_int as libc::c_uint,
            misc_flags: 0 as libc::c_int as libc::c_uint,
            ssize: 0x2 as libc::c_int as uint8_t,
            use_2m: 0 as libc::c_int as libc::c_uint,
            precmd: 0 as *const libc::c_char as *mut libc::c_char,
            file_nr: 0 as libc::c_int,
            blocksize: 0 as libc::c_int as libc::c_uint,
            codepage: 0 as libc::c_int as libc::c_uint,
            data_map: 0 as *const libc::c_char,
            tot_sectors: 0 as libc::c_int as uint32_t,
            sector_size: 0 as libc::c_int as uint16_t,
            postcmd: 0 as *const libc::c_char as *mut libc::c_char,
            cfg_filename: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = device {
            name: b"/dev/sdb4\0" as *const u8 as *const libc::c_char,
            drive: 'J' as i32 as libc::c_char,
            fat_bits: 16 as libc::c_int,
            mode: 0 as libc::c_int,
            tracks: 0 as libc::c_int as libc::c_uint,
            heads: 0 as libc::c_int as uint16_t,
            sectors: 0 as libc::c_int as uint16_t,
            hidden: 0 as libc::c_int as libc::c_uint,
            offset: 0 as libc::c_long,
            partition: 0 as libc::c_int as libc::c_uint,
            misc_flags: 0x10 as libc::c_uint,
            ssize: 0x2 as libc::c_int as uint8_t,
            use_2m: 0 as libc::c_int as libc::c_uint,
            precmd: 0 as *const libc::c_char as *mut libc::c_char,
            file_nr: 0 as libc::c_int,
            blocksize: 0 as libc::c_int as libc::c_uint,
            codepage: 0 as libc::c_int as libc::c_uint,
            data_map: 0 as *const libc::c_char,
            tot_sectors: 0 as libc::c_int as uint32_t,
            sector_size: 0 as libc::c_int as uint16_t,
            postcmd: 0 as *const libc::c_char as *mut libc::c_char,
            cfg_filename: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = device {
            name: b"/dev/sdb4\0" as *const u8 as *const libc::c_char,
            drive: 'Z' as i32 as libc::c_char,
            fat_bits: 16 as libc::c_int,
            mode: 0 as libc::c_int,
            tracks: 0 as libc::c_int as libc::c_uint,
            heads: 0 as libc::c_int as uint16_t,
            sectors: 0 as libc::c_int as uint16_t,
            hidden: 0 as libc::c_int as libc::c_uint,
            offset: 0 as libc::c_long,
            partition: 0 as libc::c_int as libc::c_uint,
            misc_flags: 0x10 as libc::c_uint,
            ssize: 0x2 as libc::c_int as uint8_t,
            use_2m: 0 as libc::c_int as libc::c_uint,
            precmd: 0 as *const libc::c_char as *mut libc::c_char,
            file_nr: 0 as libc::c_int,
            blocksize: 0 as libc::c_int as libc::c_uint,
            codepage: 0 as libc::c_int as libc::c_uint,
            data_map: 0 as *const libc::c_char,
            tot_sectors: 0 as libc::c_int as uint32_t,
            sector_size: 0 as libc::c_int as uint16_t,
            postcmd: 0 as *const libc::c_char as *mut libc::c_char,
            cfg_filename: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = device {
            name: b"$DISPLAY\0" as *const u8 as *const libc::c_char,
            drive: 'X' as i32 as libc::c_char,
            fat_bits: 0 as libc::c_int,
            mode: 0 as libc::c_int,
            tracks: 0 as libc::c_int as libc::c_uint,
            heads: 0 as libc::c_int as uint16_t,
            sectors: 0 as libc::c_int as uint16_t,
            hidden: 0 as libc::c_int as libc::c_uint,
            offset: 0 as libc::c_long,
            partition: 0 as libc::c_int as libc::c_uint,
            misc_flags: 0x40 as libc::c_uint,
            ssize: 0x2 as libc::c_int as uint8_t,
            use_2m: 0 as libc::c_int as libc::c_uint,
            precmd: 0 as *const libc::c_char as *mut libc::c_char,
            file_nr: 0 as libc::c_int,
            blocksize: 0 as libc::c_int as libc::c_uint,
            codepage: 0 as libc::c_int as libc::c_uint,
            data_map: 0 as *const libc::c_char,
            tot_sectors: 0 as libc::c_int as uint32_t,
            sector_size: 0 as libc::c_int as uint16_t,
            postcmd: 0 as *const libc::c_char as *mut libc::c_char,
            cfg_filename: 0 as *const libc::c_char,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn set_2m(mut floppy: *mut floppy_struct, mut value: libc::c_uint) {
    let mut v: uint8_t = 0;
    if value & 0x7f as libc::c_int as libc::c_uint != 0 {
        v = 0x4 as libc::c_int as uint8_t;
    } else {
        v = 0 as libc::c_int as uint8_t;
    }
    (*floppy)
        .rate = ((*floppy).rate as libc::c_int & !(0x4 as libc::c_int)
        | v as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn set_ssize(mut floppy: *mut floppy_struct, mut value: libc::c_int) {
    let mut v: uint8_t = ((((value & 7 as libc::c_int) + 6 as libc::c_int)
        % 8 as libc::c_int) << 3 as libc::c_int) as uint8_t;
    (*floppy)
        .rate = ((*floppy).rate as libc::c_int & !(0x38 as libc::c_int)
        | v as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn set_parameters(
    mut fd: libc::c_int,
    mut floppy: *mut floppy_struct,
    mut buf: *mut stat,
) -> libc::c_int {
    if gnu_dev_minor((*buf).st_rdev) & 0x7f as libc::c_int as libc::c_uint
        > 3 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return ioctl(
        fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((2 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x42 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
            as libc::c_ulong
            | (::std::mem::size_of::<floppy_struct>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        floppy,
    );
}
#[inline]
unsafe extern "C" fn get_parameters(
    mut fd: libc::c_int,
    mut floppy: *mut floppy_struct,
) -> libc::c_int {
    return ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((2 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0x4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
            as libc::c_ulong
            | (::std::mem::size_of::<floppy_struct>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        floppy,
    );
}
unsafe extern "C" fn ulong_to_sectors(mut raw_sect: libc::c_ulong) -> uint32_t {
    if raw_sect
        > (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
    {
        fprintf(
            stderr,
            b"Too many sectors for FAT %8lx\n\0" as *const u8 as *const libc::c_char,
            raw_sect,
        );
        exit(1 as libc::c_int);
    }
    return raw_sect as uint32_t;
}
pub unsafe extern "C" fn get_sector_size(mut fd: libc::c_int) -> libc::c_int {
    let mut sec_size: libc::c_int = 0;
    if ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((104 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        &mut sec_size as *mut libc::c_int,
    ) != 0 as libc::c_int || sec_size <= 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Could not get sector size of device (%s)\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if sec_size > 4096 as libc::c_int {
        sec_size = 4096 as libc::c_int;
    }
    return sec_size;
}
unsafe extern "C" fn get_block_geom(
    mut fd: libc::c_int,
    mut dev: *mut device,
) -> libc::c_int {
    let mut geom: hd_geometry = hd_geometry {
        heads: 0,
        sectors: 0,
        cylinders: 0,
        start: 0,
    };
    let mut sec_size: libc::c_int = 0;
    let mut size: libc::c_ulong = 0;
    let mut heads: uint16_t = (*dev).heads;
    let mut sectors: uint16_t = (*dev).sectors;
    let mut sect_per_track: uint32_t = 0;
    if ioctl(fd, 0x301 as libc::c_int as libc::c_ulong, &mut geom as *mut hd_geometry)
        < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Could not get geometry of device (%s)\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint
            | ((96 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        &mut size as *mut libc::c_ulong,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Could not get size of device (%s)\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    sec_size = get_sector_size(fd);
    if sec_size < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*dev).ssize = 0 as libc::c_int as uint8_t;
    while ((*dev).ssize as libc::c_int) < 0x7f as libc::c_int
        && ((128 as libc::c_int) << (*dev).ssize as libc::c_int) < sec_size
    {
        (*dev).ssize = ((*dev).ssize).wrapping_add(1);
        (*dev).ssize;
    }
    if heads == 0 {
        heads = geom.heads as uint16_t;
    }
    if sectors == 0 {
        sectors = geom.sectors as uint16_t;
    }
    sect_per_track = (heads as libc::c_int * sectors as libc::c_int) as uint32_t;
    if (*dev).hidden == 0 {
        let mut hidden: uint32_t = 0;
        hidden = (geom.start).wrapping_rem(sect_per_track as libc::c_ulong) as uint32_t;
        if hidden != 0 && hidden != sectors as libc::c_uint {
            fprintf(
                stderr,
                b"Hidden (%d) does not match sectors (%d)\n\0" as *const u8
                    as *const libc::c_char,
                hidden,
                sectors as libc::c_int,
            );
            return -(1 as libc::c_int);
        }
        (*dev).hidden = hidden;
    }
    (*dev).heads = heads;
    (*dev).sectors = sectors;
    if (*dev).tracks == 0 {
        (*dev)
            .tracks = ulong_to_sectors(
            size
                .wrapping_add(
                    ((*dev).hidden).wrapping_rem(sect_per_track) as libc::c_ulong,
                )
                .wrapping_div(sect_per_track as libc::c_ulong),
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn init_geom(
    mut fd: libc::c_int,
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut statbuf: *mut stat,
) -> libc::c_int {
    let mut floppy: floppy_struct = floppy_struct {
        size: 0,
        sect: 0,
        head: 0,
        track: 0,
        stretch: 0,
        gap: 0,
        rate: 0,
        spec1: 0,
        fmt_gap: 0,
        name: 0 as *const libc::c_char,
    };
    let mut change: libc::c_int = 0;
    if (*statbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
        && gnu_dev_major((*statbuf).st_rdev) != 2 as libc::c_int as libc::c_uint
    {
        get_block_geom(fd, dev);
        return compare_geom(dev, orig_dev);
    }
    if !((*statbuf).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
        && gnu_dev_major((*statbuf).st_rdev) == 2 as libc::c_int as libc::c_uint)
    {
        return compare_geom(dev, orig_dev);
    }
    if get_parameters(fd, &mut floppy) != 0 {
        return 1 as libc::c_int;
    }
    change = 0 as libc::c_int;
    if (*dev).sectors as libc::c_int != 0
        && (*dev).sectors as libc::c_uint != floppy.sect
    {
        floppy.sect = (*dev).sectors as libc::c_uint;
        change = 1 as libc::c_int;
    } else {
        (*dev).sectors = floppy.sect as uint16_t;
    }
    if (*dev).heads as libc::c_int != 0 && (*dev).heads as libc::c_uint != floppy.head {
        floppy.head = (*dev).heads as libc::c_uint;
        change = 1 as libc::c_int;
    } else {
        (*dev).heads = floppy.head as uint16_t;
    }
    if (*dev).tracks != 0 && (*dev).tracks != floppy.track {
        floppy.track = (*dev).tracks;
        change = 1 as libc::c_int;
    } else {
        (*dev).tracks = floppy.track;
    }
    if (*dev).use_2m != 0
        && (*dev).use_2m
            != (if floppy.rate as libc::c_int & 0x4 as libc::c_int != 0 {
                0xff as libc::c_int
            } else {
                0x80 as libc::c_int
            }) as libc::c_uint
    {
        set_2m(&mut floppy, (*dev).use_2m);
        change = 1 as libc::c_int;
    } else {
        (*dev)
            .use_2m = (if floppy.rate as libc::c_int & 0x4 as libc::c_int != 0 {
            0xff as libc::c_int
        } else {
            0x80 as libc::c_int
        }) as libc::c_uint;
    }
    if (*dev).ssize as libc::c_int & 0x80 as libc::c_int == 0 {
        (*dev).ssize = 0 as libc::c_int as uint8_t;
    }
    if (*dev).ssize as libc::c_int != 0
        && (*dev).ssize as libc::c_int
            != (((floppy.rate as libc::c_int & 0x38 as libc::c_int) >> 3 as libc::c_int)
                + 2 as libc::c_int) % 8 as libc::c_int + 128 as libc::c_int
    {
        set_ssize(&mut floppy, (*dev).ssize as libc::c_int);
        change = 1 as libc::c_int;
    } else {
        (*dev)
            .ssize = ((((floppy.rate as libc::c_int & 0x38 as libc::c_int)
            >> 3 as libc::c_int) + 2 as libc::c_int) % 8 as libc::c_int) as uint8_t;
    }
    if change == 0 {
        return 0 as libc::c_int;
    }
    floppy
        .size = (((*dev).sectors as libc::c_int * (*dev).heads as libc::c_int)
        as libc::c_uint)
        .wrapping_mul((*dev).tracks);
    if (*dev).tracks > 41 as libc::c_int as libc::c_uint {
        floppy.stretch = 0 as libc::c_int as libc::c_uint;
    } else {
        floppy.stretch = 1 as libc::c_int as libc::c_uint;
    }
    return set_parameters(fd, &mut floppy, statbuf);
}
pub static mut nr_const_devices: libc::c_uint = 0;
unsafe extern "C" fn run_static_initializers() {
    nr_const_devices = (::std::mem::size_of::<[device; 5]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<device>() as libc::c_ulong) as libc::c_uint;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
