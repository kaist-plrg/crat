use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn perror(__s: *const libc::c_char);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn printOom();
    fn destroy_privs();
    fn find_device(
        drive: libc::c_char,
        mode: libc::c_int,
        out_dev: *mut device,
        boot: *mut bootsector,
        name: *mut libc::c_char,
        media: *mut libc::c_int,
        maxSize: *mut mt_off_t,
        isRop: *mut libc::c_int,
    ) -> *mut Stream_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: libc::c_int,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub pread: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub pwrite: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub freeFunc: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub set_geom: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> libc::c_int,
    >,
    pub get_data: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut libc::c_int,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub pre_allocate: Option::<
        unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
    >,
    pub get_dosConvert: Option::<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union bootsector {
    pub bytes: [libc::c_uchar; 4096],
    pub characters: [libc::c_char; 4096],
    pub boot: bootsector_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bootsector_s {
    pub jump: [libc::c_uchar; 3],
    pub banner: [libc::c_char; 8],
    pub secsiz: [libc::c_uchar; 2],
    pub clsiz: libc::c_uchar,
    pub nrsvsect: [libc::c_uchar; 2],
    pub nfat: libc::c_uchar,
    pub dirents: [libc::c_uchar; 2],
    pub psect: [libc::c_uchar; 2],
    pub descr: libc::c_uchar,
    pub fatlen: [libc::c_uchar; 2],
    pub nsect: [libc::c_uchar; 2],
    pub nheads: [libc::c_uchar; 2],
    pub nhs: [libc::c_uchar; 4],
    pub bigsect: [libc::c_uchar; 4],
    pub ext: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fat32: fat32_t,
    pub old: oldboot_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldboot_t {
    pub labelBlock: label_blk_t,
    pub res_2m: libc::c_uchar,
    pub CheckSum: libc::c_uchar,
    pub fmt_2mf: libc::c_uchar,
    pub wt: libc::c_uchar,
    pub rate_0: libc::c_uchar,
    pub rate_any: libc::c_uchar,
    pub BootP: [libc::c_uchar; 2],
    pub Infp0: [libc::c_uchar; 2],
    pub InfpX: [libc::c_uchar; 2],
    pub InfTm: [libc::c_uchar; 2],
    pub DateF: [libc::c_uchar; 2],
    pub TimeF: [libc::c_uchar; 2],
    pub junk: [libc::c_uchar; 944],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct label_blk_t {
    pub physdrive: libc::c_uchar,
    pub reserved: libc::c_uchar,
    pub dos4: libc::c_uchar,
    pub serial: [libc::c_uchar; 4],
    pub label: [libc::c_char; 11],
    pub fat_type: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fat32_t {
    pub bigFat: [libc::c_uchar; 4],
    pub extFlags: [libc::c_uchar; 2],
    pub fsVersion: [libc::c_uchar; 2],
    pub rootCluster: [libc::c_uchar; 4],
    pub infoSector: [libc::c_uchar; 2],
    pub backupBoot: [libc::c_uchar; 2],
    pub reserved: [libc::c_uchar; 6],
    pub reserved2: [libc::c_uchar; 6],
    pub labelBlock: label_blk_t,
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
pub unsafe extern "C" fn mmount(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut drive: libc::c_char = 0;
    let mut pid: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut dev: device = device {
        name: 0 as *const libc::c_char,
        drive: 0,
        fat_bits: 0,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0,
        ssize: 0,
        use_2m: 0,
        precmd: 0 as *mut libc::c_char,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: 0 as *const libc::c_char,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: 0 as *mut libc::c_char,
        cfg_filename: 0 as *const libc::c_char,
    };
    let mut name: [libc::c_char; 2048] = [0; 2048];
    let mut media: libc::c_int = 0;
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    if argc < 2 as libc::c_int
        || *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            == 0
        || *(*argv.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int != ':' as i32
        || *(*argv.offset(1 as libc::c_int as isize)).offset(2 as libc::c_int as isize)
            as libc::c_int != 0
    {
        fprintf(
            stderr,
            b"Usage: %s -V drive:\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    drive = ch_toupper(
        *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize),
    );
    Stream = find_device(
        drive,
        0 as libc::c_int,
        &mut dev,
        &mut boot,
        name.as_mut_ptr(),
        &mut media,
        0 as *mut mt_off_t,
        0 as *mut libc::c_int,
    );
    if Stream.is_null() {
        exit(1 as libc::c_int);
    }
    free_stream(&mut Stream);
    destroy_privs();
    if dev.partition != 0 {
        let mut part_name: [libc::c_char; 4] = [0; 4];
        sprintf(
            part_name.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (dev.partition).wrapping_rem(1000 as libc::c_int as libc::c_uint),
        );
        strcat(name.as_mut_ptr(), part_name.as_mut_ptr());
    }
    pid = fork();
    match pid {
        -1 => {
            fprintf(stderr, b"fork failed\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        0 => {
            close(2 as libc::c_int);
            open(
                b"/dev/null\0" as *const u8 as *const libc::c_char,
                0o2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int,
            );
            let ref mut fresh0 = *argv.offset(1 as libc::c_int as isize);
            *fresh0 = strdup(b"mount\0" as *const u8 as *const libc::c_char);
            if argc > 2 as libc::c_int {
                execvp(
                    b"mount\0" as *const u8 as *const libc::c_char,
                    argv.offset(1 as libc::c_int as isize) as *const *mut libc::c_char,
                );
            } else {
                execlp(
                    b"mount\0" as *const u8 as *const libc::c_char,
                    b"mount\0" as *const u8 as *const libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
            }
            perror(b"exec mount\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        _ => while wait(&mut status) != pid {}
    }
    if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int == 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    let ref mut fresh1 = *argv.offset(0 as libc::c_int as isize);
    *fresh1 = strdup(b"mount\0" as *const u8 as *const libc::c_char);
    let ref mut fresh2 = *argv.offset(1 as libc::c_int as isize);
    *fresh2 = strdup(b"-r\0" as *const u8 as *const libc::c_char);
    if (*argv.offset(0 as libc::c_int as isize)).is_null()
        || (*argv.offset(1 as libc::c_int as isize)).is_null()
    {
        printOom();
        exit(1 as libc::c_int);
    }
    if argc > 2 as libc::c_int {
        execvp(
            b"mount\0" as *const u8 as *const libc::c_char,
            argv as *const *mut libc::c_char,
        );
    } else {
        execlp(
            b"mount\0" as *const u8 as *const libc::c_char,
            b"mount\0" as *const u8 as *const libc::c_char,
            b"-r\0" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
    }
    exit(1 as libc::c_int);
}
