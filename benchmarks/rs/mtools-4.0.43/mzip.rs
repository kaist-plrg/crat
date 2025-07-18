use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn getpass(__prompt: *const libc::c_char) -> *mut libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn setmntent(__file: *const libc::c_char, __mode: *const libc::c_char) -> *mut FILE;
    fn getmntent(__stream: *mut FILE) -> *mut mntent;
    fn endmntent(__stream: *mut FILE) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut devices: *mut device;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn drop_privs();
    fn reclaim_privs();
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    fn get_real_uid() -> uid_t;
    static mut mversion: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    fn expand(_: *const libc::c_char, _: *mut libc::c_char) -> *const libc::c_char;
    static mut progname: *const libc::c_char;
    fn closeExec(fd: libc::c_int);
    fn precmd(dev: *mut device);
    fn postcmd(cmd: *const libc::c_char);
    fn scsi_cmd(
        fd: libc::c_int,
        cdb: *mut libc::c_uchar,
        clen: uint8_t,
        mode: scsi_io_mode_t,
        data: *mut libc::c_void,
        len: uint32_t,
        extra_data: *mut libc::c_void,
    ) -> libc::c_int;
    fn scsi_open(
        name: *const libc::c_char,
        flags: libc::c_int,
        mode: libc::c_int,
        extra_data: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
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
pub type uid_t = __uid_t;
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
pub struct mntent {
    pub mnt_fsname: *mut libc::c_char,
    pub mnt_dir: *mut libc::c_char,
    pub mnt_type: *mut libc::c_char,
    pub mnt_opts: *mut libc::c_char,
    pub mnt_freq: libc::c_int,
    pub mnt_passno: libc::c_int,
}
pub type device_t = device;
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
pub type scsi_io_mode_t = libc::c_uint;
pub const SCSI_IO_WRITE: scsi_io_mode_t = 1;
pub const SCSI_IO_READ: scsi_io_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub type_0: libc::c_char,
    pub type_modifier: libc::c_char,
    pub scsi_version: libc::c_char,
    pub data_format: libc::c_char,
    pub length: libc::c_char,
    pub reserved1: [libc::c_char; 2],
    pub capabilities: libc::c_char,
    pub vendor: [libc::c_char; 8],
    pub product: [libc::c_char; 16],
    pub revision: [libc::c_char; 4],
    pub vendor_specific: [libc::c_char; 20],
    pub reserved2: [libc::c_char; 40],
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn ch_toupper(mut ch: libc::c_char) -> libc::c_char {
    return ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = ch as libc::c_uchar as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(ch as libc::c_uchar as libc::c_int);
            }
        } else {
            __res = *(*__ctype_toupper_loc())
                .offset(ch as libc::c_uchar as libc::c_int as isize);
        }
        __res
    }) as libc::c_char;
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
unsafe extern "C" fn zip_cmd(
    mut priv_0: libc::c_int,
    mut fd: libc::c_int,
    mut cdb: *mut libc::c_uchar,
    mut clen: uint8_t,
    mut mode: scsi_io_mode_t,
    mut data: *mut libc::c_void,
    mut len: uint32_t,
    mut extra_data: *mut libc::c_void,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if priv_0 != 0 {
        reclaim_privs();
    }
    r = scsi_cmd(fd, cdb, clen, mode, data, len, extra_data);
    if priv_0 != 0 {
        drop_privs();
    }
    return r;
}
unsafe extern "C" fn test_mounted(mut dev: *mut libc::c_char) -> libc::c_int {
    let mut mnt: *mut mntent = 0 as *mut mntent;
    let mut st_dev: stat = stat {
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
    let mut st_mnt: stat = stat {
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
    let mut mtab: *mut FILE = 0 as *mut FILE;
    if stat(dev, &mut st_dev) != 0 {
        fprintf(
            stderr,
            b"%s: stat(%s) failed: %s.\n\0" as *const u8 as *const libc::c_char,
            progname,
            dev,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    if !(st_dev.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int;
    }
    mtab = setmntent(
        b"/etc/mtab\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if mtab.is_null() {
        fprintf(
            stderr,
            b"%s: can't open %s.\n\0" as *const u8 as *const libc::c_char,
            progname,
            b"/etc/mtab\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    loop {
        mnt = getmntent(mtab);
        if mnt.is_null() {
            break;
        }
        if ((*mnt).mnt_fsname).is_null()
            || strcmp((*mnt).mnt_type, b"swap\0" as *const u8 as *const libc::c_char)
                == 0
            || strcmp((*mnt).mnt_type, b"nfs\0" as *const u8 as *const libc::c_char) == 0
            || strcmp((*mnt).mnt_type, b"proc\0" as *const u8 as *const libc::c_char)
                == 0
            || strcmp((*mnt).mnt_type, b"smbfs\0" as *const u8 as *const libc::c_char)
                == 0
            || strcmp((*mnt).mnt_type, b"ignore\0" as *const u8 as *const libc::c_char)
                == 0
        {
            continue;
        }
        if stat((*mnt).mnt_fsname, &mut st_mnt) != 0 {
            continue;
        }
        if st_mnt.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint
        {
            if gnu_dev_major(st_mnt.st_rdev) == gnu_dev_major(st_dev.st_rdev)
                && gnu_dev_minor(st_mnt.st_rdev) >= gnu_dev_minor(st_dev.st_rdev)
                && gnu_dev_minor(st_mnt.st_rdev)
                    <= (gnu_dev_minor(st_dev.st_rdev))
                        .wrapping_add(15 as libc::c_int as libc::c_uint)
            {
                fprintf(
                    stderr,
                    b"Device %s%d is mounted on %s.\n\0" as *const u8
                        as *const libc::c_char,
                    dev,
                    (gnu_dev_minor(st_mnt.st_rdev))
                        .wrapping_sub(gnu_dev_minor(st_dev.st_rdev)),
                    (*mnt).mnt_dir,
                );
                endmntent(mtab);
                return 1 as libc::c_int;
            }
        }
    }
    endmntent(mtab);
    return 0 as libc::c_int;
}
unsafe extern "C" fn usage(mut ret: libc::c_int) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-V] [-q] [-e] [-u] [-r|-w|-p|-x] [drive:]\n\t-q print status\n\t-e eject disk\n\t-f eject disk even when mounted\n\t-r write protected (read-only)\n\t-w not write-protected (read-write)\n\t-p password write protected\n\t-x password protected\n\t-u unprotect till disk ejecting\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn get_zip_status(
    mut priv_0: libc::c_int,
    mut fd: libc::c_int,
    mut extra_data: *mut libc::c_void,
) -> uint8_t {
    let mut status: [libc::c_uchar; 128] = [0; 128];
    let mut cdb: [libc::c_uchar; 6] = [
        0x6 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0x2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    if zip_cmd(
        priv_0,
        fd,
        cdb.as_mut_ptr(),
        6 as libc::c_int as uint8_t,
        SCSI_IO_READ,
        status.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong as uint32_t,
        extra_data,
    ) == -(1 as libc::c_int)
    {
        perror(b"status: \0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    return (status[21 as libc::c_int as usize] as libc::c_int & 0xf as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn short_command(
    mut priv_0: libc::c_int,
    mut fd: libc::c_int,
    mut cmd1: uint8_t,
    mut cmd2: uint8_t,
    mut cmd3: uint8_t,
    mut data: *const libc::c_char,
    mut extra_data: *mut libc::c_void,
) -> libc::c_int {
    let mut cdb: [uint8_t; 6] = [
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    cdb[0 as libc::c_int as usize] = cmd1;
    cdb[1 as libc::c_int as usize] = cmd2;
    cdb[4 as libc::c_int as usize] = cmd3;
    return zip_cmd(
        priv_0,
        fd,
        cdb.as_mut_ptr(),
        6 as libc::c_int as uint8_t,
        SCSI_IO_WRITE,
        data as *mut libc::c_char as *mut libc::c_void,
        if !data.is_null() {
            strlen(data) as uint32_t
        } else {
            0 as libc::c_int as libc::c_uint
        },
        extra_data,
    );
}
unsafe extern "C" fn iomega_command(
    mut priv_0: libc::c_int,
    mut fd: libc::c_int,
    mut mode: uint8_t,
    mut data: *const libc::c_char,
    mut extra_data: *mut libc::c_void,
) -> libc::c_int {
    return short_command(
        priv_0,
        fd,
        0xc as libc::c_int as uint8_t,
        mode,
        (if !data.is_null() {
            strlen(data) as uint8_t as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint8_t,
        data,
        extra_data,
    );
}
unsafe extern "C" fn door_command(
    mut priv_0: libc::c_int,
    mut fd: libc::c_int,
    mut cmd1: uint8_t,
    mut cmd2: uint8_t,
    mut extra_data: *mut libc::c_void,
) -> libc::c_int {
    return short_command(
        priv_0,
        fd,
        cmd1,
        0 as libc::c_int as uint8_t,
        cmd2,
        0 as *const libc::c_char,
        extra_data,
    );
}
pub unsafe extern "C" fn mzip(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut extra_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: libc::c_int = 0;
    let mut drive: libc::c_char = 0;
    let mut dev: *mut device_t = 0 as *mut device_t;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut name: [libc::c_char; 2048] = [0; 2048];
    let mut request: libc::c_int = 0 as libc::c_int;
    let mut newMode: uint8_t = 0 as libc::c_int as uint8_t;
    let mut oldMode: uint8_t = 0 as libc::c_int as uint8_t;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:efpqrwxuh\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            102 => {
                if get_real_uid() != 0 {
                    fprintf(
                        stderr,
                        b"Only root can use force. Sorry.\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                request |= (1 as libc::c_int) << 3 as libc::c_int;
            }
            101 => {
                request |= (1 as libc::c_int) << 1 as libc::c_int;
            }
            113 => {
                request |= (1 as libc::c_int) << 0 as libc::c_int;
            }
            112 => {
                if request & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    usage(1 as libc::c_int);
                }
                request |= (1 as libc::c_int) << 2 as libc::c_int;
                newMode = 3 as libc::c_int as uint8_t;
            }
            114 => {
                if request & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    usage(1 as libc::c_int);
                }
                request |= (1 as libc::c_int) << 2 as libc::c_int;
                newMode = 2 as libc::c_int as uint8_t;
            }
            119 => {
                if request & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    usage(1 as libc::c_int);
                }
                request |= (1 as libc::c_int) << 2 as libc::c_int;
                newMode = 0 as libc::c_int as uint8_t;
            }
            120 => {
                if request & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    usage(1 as libc::c_int);
                }
                request |= (1 as libc::c_int) << 2 as libc::c_int;
                newMode = 5 as libc::c_int as uint8_t;
            }
            117 => {
                if request & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    usage(1 as libc::c_int);
                }
                request |= (1 as libc::c_int) << 2 as libc::c_int;
                newMode = 8 as libc::c_int as uint8_t;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if request == 0 as libc::c_int {
        request = (1 as libc::c_int) << 0 as libc::c_int;
    }
    if argc - optind > 1 as libc::c_int
        || argc - optind == 1 as libc::c_int
            && (*(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize) == 0
                || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int != ':' as i32)
    {
        usage(1 as libc::c_int);
    }
    drive = ch_toupper(
        (if argc - optind == 1 as libc::c_int {
            *(*argv.offset((argc - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int
        } else {
            ':' as i32
        }) as libc::c_char,
    );
    dev = devices;
    while !((*dev).name).is_null() {
        let mut cdb: [libc::c_uchar; 6] = [
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
            0 as libc::c_int as libc::c_uchar,
        ];
        let mut inq_data: C2RustUnnamed = C2RustUnnamed {
            type_0: 0,
            type_modifier: 0,
            scsi_version: 0,
            data_format: 0,
            length: 0,
            reserved1: [0; 2],
            capabilities: 0,
            vendor: [0; 8],
            product: [0; 16],
            revision: [0; 4],
            vendor_specific: [0; 20],
            reserved2: [0; 40],
        };
        if !((*dev).drive as libc::c_int != drive as libc::c_int) {
            expand((*dev).name, name.as_mut_ptr());
            if request
                & ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) != 0
                && request & (1 as libc::c_int) << 3 as libc::c_int == 0
                && test_mounted(name.as_mut_ptr()) != 0
            {
                fprintf(
                    stderr,
                    b"Can't change status of/eject mounted device\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            precmd(dev);
            if !dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0 {
                reclaim_privs();
            }
            fd = scsi_open(
                name.as_mut_ptr(),
                0 as libc::c_int | 0o4000 as libc::c_int,
                0o644 as libc::c_int,
                &mut extra_data,
            );
            if !dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0 {
                drop_privs();
            }
            if !(fd == -(1 as libc::c_int)) {
                closeExec(fd);
                if request
                    & ((1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int) == 0
                {
                    break;
                }
                cdb[0 as libc::c_int as usize] = 0x12 as libc::c_int as libc::c_uchar;
                cdb[4 as libc::c_int
                    as usize] = ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong
                    as libc::c_uchar;
                if zip_cmd(
                    (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                        as libc::c_int,
                    fd,
                    cdb.as_mut_ptr(),
                    6 as libc::c_int as uint8_t,
                    SCSI_IO_READ,
                    &mut inq_data as *mut C2RustUnnamed as *mut libc::c_void,
                    ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong as uint32_t,
                    extra_data,
                ) != 0 as libc::c_int
                {
                    close(fd);
                } else {
                    if !(strncasecmp(
                        b"IOMEGA  \0" as *const u8 as *const libc::c_char,
                        (inq_data.vendor).as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    ) != 0
                        || strncasecmp(
                            b"ZIP 100         \0" as *const u8 as *const libc::c_char,
                            (inq_data.product).as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                        ) != 0
                            && strncasecmp(
                                b"ZIP 100 PLUS    \0" as *const u8 as *const libc::c_char,
                                (inq_data.product).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                            ) != 0
                            && strncasecmp(
                                b"ZIP 250         \0" as *const u8 as *const libc::c_char,
                                (inq_data.product).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                            ) != 0
                            && strncasecmp(
                                b"ZIP 750         \0" as *const u8 as *const libc::c_char,
                                (inq_data.product).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                            ) != 0
                            && strncasecmp(
                                b"JAZ 1GB         \0" as *const u8 as *const libc::c_char,
                                (inq_data.product).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                            ) != 0
                            && strncasecmp(
                                b"JAZ 2GB         \0" as *const u8 as *const libc::c_char,
                                (inq_data.product).as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                            ) != 0)
                    {
                        break;
                    }
                    fprintf(
                        stderr,
                        b"Skipping drive with vendor='\0" as *const u8
                            as *const libc::c_char,
                    );
                    fwrite(
                        (inq_data.vendor).as_mut_ptr() as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                        stderr,
                    );
                    fprintf(
                        stderr,
                        b"' product='\0" as *const u8 as *const libc::c_char,
                    );
                    fwrite(
                        (inq_data.product).as_mut_ptr() as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                        stderr,
                    );
                    fprintf(stderr, b"'\n\0" as *const u8 as *const libc::c_char);
                    close(fd);
                }
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if (*dev).drive as libc::c_int == 0 as libc::c_int {
        fprintf(
            stderr,
            b"%s: drive '%c:' is not a Zip or Jaz drive\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            drive as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    if request
        & ((1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int) != 0
    {
        oldMode = get_zip_status(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                as libc::c_int,
            fd,
            extra_data,
        );
    }
    if request & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        if newMode as libc::c_int == 8 as libc::c_int
            && oldMode as libc::c_int & 8 as libc::c_int != 0
        {
            request &= !((1 as libc::c_int) << 2 as libc::c_int);
        }
        if newMode as libc::c_int & 0x1 as libc::c_int == 0
            && newMode as libc::c_int == oldMode as libc::c_int
        {
            request &= !((1 as libc::c_int) << 2 as libc::c_int);
        }
    }
    if request & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        let mut ret: libc::c_int = 0;
        let mut unlockMode: uint8_t = 0;
        let mut unlockMask: uint8_t = 0;
        let mut passwd: *const libc::c_char = 0 as *const libc::c_char;
        let mut dummy: [libc::c_char; 1] = [0; 1];
        if newMode as libc::c_int == 8 as libc::c_int {
            unlockMode = (newMode as libc::c_int | oldMode as libc::c_int) as uint8_t;
            unlockMask = 9 as libc::c_int as uint8_t;
        } else {
            unlockMode = (newMode as libc::c_int & !(0x5 as libc::c_int)) as uint8_t;
            unlockMask = 1 as libc::c_int as uint8_t;
        }
        if oldMode as libc::c_int & unlockMask as libc::c_int == 1 as libc::c_int {
            let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
            passwd = b"APlaceForYourStuff\0" as *const u8 as *const libc::c_char;
            s = strchr(passwd, '\n' as i32);
            if !s.is_null() {
                *s = '\0' as i32 as libc::c_char;
            }
            iomega_command(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                    as libc::c_int,
                fd,
                unlockMode,
                passwd,
                extra_data,
            );
        }
        if get_zip_status(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                as libc::c_int,
            fd,
            extra_data,
        ) as libc::c_int & unlockMask as libc::c_int == 1 as libc::c_int
        {
            let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
            passwd = getpass(b"Password: \0" as *const u8 as *const libc::c_char);
            s_0 = strchr(passwd, '\n' as i32);
            if !s_0.is_null() {
                *s_0 = '\0' as i32 as libc::c_char;
            }
            ret = iomega_command(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                    as libc::c_int,
                fd,
                unlockMode,
                passwd,
                extra_data,
            );
            if ret != 0 {
                if ret == -(1 as libc::c_int) {
                    perror(b"passwd: \0" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(
                        stderr,
                        b"wrong password\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                exit(1 as libc::c_int);
            }
            if get_zip_status(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                    as libc::c_int,
                fd,
                extra_data,
            ) as libc::c_int & unlockMask as libc::c_int == 1 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"wrong password\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        if newMode as libc::c_int & 0x1 as libc::c_int != 0 {
            let mut first_try: [libc::c_char; 34] = [0; 34];
            passwd = getpass(
                b"Enter new password:\0" as *const u8 as *const libc::c_char,
            );
            strncpy(first_try.as_mut_ptr(), passwd, 33 as libc::c_int as libc::c_ulong);
            passwd = getpass(
                b"Re-type new password:\0" as *const u8 as *const libc::c_char,
            );
            if strncmp(
                first_try.as_mut_ptr(),
                passwd,
                33 as libc::c_int as libc::c_ulong,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"You misspelled it. Password not set.\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        } else {
            passwd = dummy.as_mut_ptr();
            dummy[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
        if newMode as libc::c_int == 8 as libc::c_int {
            newMode = (newMode as libc::c_int | oldMode as libc::c_int) as uint8_t;
        }
        ret = iomega_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                as libc::c_int,
            fd,
            newMode,
            passwd,
            extra_data,
        );
        if ret != 0 {
            if ret == -(1 as libc::c_int) {
                perror(b"set passwd: \0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(
                    stderr,
                    b"password not changed\n\0" as *const u8 as *const libc::c_char,
                );
            }
            exit(1 as libc::c_int);
        }
        ioctl(
            fd,
            ((0 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                    + 14 as libc::c_int
                | ((0x12 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint
                | ((95 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                | ((0 as libc::c_int)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint) as libc::c_ulong,
        );
    }
    if request & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        let mut unlocked: *const libc::c_char = 0 as *const libc::c_char;
        if oldMode as libc::c_int & 8 as libc::c_int != 0 {
            unlocked = b" and unlocked until eject\0" as *const u8
                as *const libc::c_char;
        } else {
            unlocked = b"\0" as *const u8 as *const libc::c_char;
        }
        match oldMode as libc::c_int & !(8 as libc::c_int) {
            0 => {
                printf(
                    b"Drive '%c:' is not write-protected\n\0" as *const u8
                        as *const libc::c_char,
                    drive as libc::c_int,
                );
            }
            2 => {
                printf(
                    b"Drive '%c:' is write-protected%s\n\0" as *const u8
                        as *const libc::c_char,
                    drive as libc::c_int,
                    unlocked,
                );
            }
            3 => {
                printf(
                    b"Drive '%c:' is password write-protected%s\n\0" as *const u8
                        as *const libc::c_char,
                    drive as libc::c_int,
                    unlocked,
                );
            }
            5 => {
                printf(
                    b"Drive '%c:' is password protected%s\n\0" as *const u8
                        as *const libc::c_char,
                    drive as libc::c_int,
                    unlocked,
                );
            }
            _ => {
                printf(
                    b"Unknown protection mode %d of drive '%c:'\n\0" as *const u8
                        as *const libc::c_char,
                    oldMode as libc::c_int,
                    drive as libc::c_int,
                );
            }
        }
    }
    if request & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if request & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            if door_command(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                    as libc::c_int,
                fd,
                0x1e as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                extra_data,
            ) < 0 as libc::c_int
            {
                perror(b"door unlock: \0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        }
        if door_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                as libc::c_int,
            fd,
            0x1b as libc::c_int as uint8_t,
            1 as libc::c_int as uint8_t,
            extra_data,
        ) < 0 as libc::c_int
        {
            perror(b"stop motor: \0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        if door_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                as libc::c_int,
            fd,
            0x1b as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            extra_data,
        ) < 0 as libc::c_int
        {
            perror(b"eject: \0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        if door_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0)
                as libc::c_int,
            fd,
            0x1b as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            extra_data,
        ) < 0 as libc::c_int
        {
            perror(b"second eject: \0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    close(fd);
    postcmd((*dev).postcmd);
    exit(0 as libc::c_int);
}
