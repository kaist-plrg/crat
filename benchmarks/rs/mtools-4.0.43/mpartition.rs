use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn check_number_parse_errno(
        c: libc::c_char,
        optarg_0: *const libc::c_char,
        endptr: *mut libc::c_char,
    );
    fn parseSize(sizeStr: *mut libc::c_char) -> uint32_t;
    fn strtoui(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_uint;
    fn atoui(nptr: *const libc::c_char) -> libc::c_uint;
    fn strtou8(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint8_t;
    fn atou16(str: *const libc::c_char) -> uint16_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn print_sector(
        message: *const libc::c_char,
        data: *mut libc::c_uchar,
        size: libc::c_int,
    );
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut mversion: *const libc::c_char;
    fn expand(_: *const libc::c_char, _: *mut libc::c_char) -> *const libc::c_char;
    static mut mdate: *const libc::c_char;
    static mut progname: *const libc::c_char;
    static mut devices: *mut device;
    fn consistencyCheck(
        partTable: *mut partition,
        doprint: libc::c_int,
        verbose: libc::c_int,
        has_activated: *mut libc::c_int,
        tot_sectors: uint32_t,
        used_dev: *mut device,
        target_partition: libc::c_uint,
    ) -> libc::c_int;
    fn findOverlap(
        partTable: *mut partition,
        until: libc::c_uint,
        start: uint32_t,
        end: uint32_t,
    ) -> libc::c_uint;
    fn OpenImage(
        out_dev: *mut device,
        dev: *mut device,
        name: *const libc::c_char,
        mode: libc::c_int,
        errmsg: *mut libc::c_char,
        flags: libc::c_int,
        lockMode: libc::c_int,
        maxSize: *mut mt_off_t,
        geomFailureP: *mut libc::c_int,
        xdf_info: *mut xdf_info,
    ) -> *mut Stream_t;
    fn compute_lba_geom_from_tot_sectors(dev: *mut device) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct partition {
    pub start: hsc,
    pub end: hsc,
    pub start_sect: [libc::c_uchar; 4],
    pub nr_sects: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsc {
    pub byte0: libc::c_uchar,
    pub head: libc::c_uchar,
    pub sector: libc::c_uchar,
    pub cyl: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: libc::c_uint,
    pub RootDirSize: uint16_t,
    pub BadSectors: libc::c_uint,
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
#[inline]
unsafe extern "C" fn set_dword(mut data: *mut libc::c_uchar, mut value: uint32_t) {
    *data
        .offset(
            3 as libc::c_int as isize,
        ) = (value >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            2 as libc::c_int as isize,
        ) = (value >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (value >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn set_word(mut data: *mut libc::c_uchar, mut value: libc::c_ushort) {
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (value as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (value as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
}
unsafe extern "C" fn set_offset(
    mut h: *mut hsc,
    mut offset: libc::c_ulong,
    mut heads: uint16_t,
    mut sectors: uint16_t,
) {
    let mut head: uint16_t = 0;
    let mut sector: uint16_t = 0;
    let mut cyl: libc::c_uint = 0;
    if heads == 0 || sectors == 0 {
        cyl = 0 as libc::c_int as libc::c_uint;
        sector = cyl as uint16_t;
        head = sector;
    } else {
        sector = offset.wrapping_rem(sectors as libc::c_ulong) as uint16_t;
        offset = offset.wrapping_div(sectors as libc::c_ulong);
        head = offset.wrapping_rem(heads as libc::c_ulong) as uint16_t;
        offset = offset.wrapping_div(heads as libc::c_ulong);
        if offset > 1023 as libc::c_int as libc::c_ulong {
            cyl = 1023 as libc::c_int as libc::c_uint;
        } else {
            cyl = offset as uint16_t as libc::c_uint;
        }
    }
    if head as libc::c_int > 255 as libc::c_int {
        cyl = 0 as libc::c_int as libc::c_uint;
        sector = cyl as uint16_t;
        head = sector;
    }
    (*h).head = head as uint8_t;
    (*h)
        .sector = ((sector as libc::c_int + 1 as libc::c_int & 0x3f as libc::c_int)
        as libc::c_uint
        | (cyl & 0x300 as libc::c_int as libc::c_uint) >> 2 as libc::c_int)
        as libc::c_uchar;
    (*h).cyl = (cyl & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
pub unsafe extern "C" fn setBeginEnd(
    mut partTable: *mut partition,
    mut begin: uint32_t,
    mut end: uint32_t,
    mut iheads: uint16_t,
    mut isectors: uint16_t,
    mut activate: libc::c_int,
    mut type_0: uint8_t,
    mut fat_bits: libc::c_uint,
) {
    let mut heads: uint8_t = 0;
    let mut sectors: uint8_t = 0;
    if iheads as libc::c_int > 255 as libc::c_int {
        fprintf(
            stderr,
            b"Too many heads for partition: %d\n\0" as *const u8 as *const libc::c_char,
            iheads as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    heads = iheads as uint8_t;
    if isectors as libc::c_int > 255 as libc::c_int {
        fprintf(
            stderr,
            b"Too many sectors for partition: %d\n\0" as *const u8
                as *const libc::c_char,
            isectors as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    sectors = isectors as uint8_t;
    set_offset(
        &mut (*partTable).start,
        begin as libc::c_ulong,
        heads as uint16_t,
        sectors as uint16_t,
    );
    set_offset(
        &mut (*partTable).end,
        end.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        heads as uint16_t,
        sectors as uint16_t,
    );
    set_dword(((*partTable).start_sect).as_mut_ptr(), begin);
    set_dword(((*partTable).nr_sects).as_mut_ptr(), end.wrapping_sub(begin));
    if activate != 0 {
        (*partTable).start.byte0 = 0x80 as libc::c_int as libc::c_uchar;
    } else {
        (*partTable).start.byte0 = 0 as libc::c_int as libc::c_uchar;
    }
    if type_0 == 0 {
        if fat_bits == 0 as libc::c_int as libc::c_uint {
            if end.wrapping_sub(begin) < 4096 as libc::c_int as libc::c_uint {
                fat_bits = 12 as libc::c_int as libc::c_uint;
            } else {
                fat_bits = 16 as libc::c_int as libc::c_uint;
            }
        }
        if fat_bits == 32 as libc::c_int as libc::c_uint {
            type_0 = 0xc as libc::c_int as uint8_t;
        } else if end < 65536 as libc::c_int as libc::c_uint {
            if fat_bits == 12 as libc::c_int as libc::c_uint {
                type_0 = 0x1 as libc::c_int as uint8_t;
            } else if fat_bits == 16 as libc::c_int as libc::c_uint {
                type_0 = 0x4 as libc::c_int as uint8_t;
            }
        } else if end
            < ((sectors as libc::c_int * heads as libc::c_int) as libc::c_uint)
                .wrapping_mul(1024 as libc::c_uint)
        {
            type_0 = 0x6 as libc::c_int as uint8_t;
        } else {
            type_0 = 0xe as libc::c_int as uint8_t;
        }
    }
    (*partTable).end.byte0 = type_0;
}
unsafe extern "C" fn setsize(
    mut capacity: libc::c_ulong,
    mut cyls: *mut libc::c_uint,
    mut hds: *mut uint16_t,
    mut secs: *mut uint16_t,
) -> libc::c_int {
    let mut rv: libc::c_int = 0 as libc::c_int;
    let mut heads: libc::c_ulong = 0;
    let mut sectors: libc::c_ulong = 0;
    let mut cylinders: libc::c_ulong = 0;
    let mut temp: libc::c_ulong = 0;
    cylinders = 1024 as libc::c_long as libc::c_ulong;
    sectors = 62 as libc::c_long as libc::c_ulong;
    temp = cylinders.wrapping_mul(sectors);
    heads = capacity.wrapping_div(temp);
    if capacity.wrapping_rem(temp) != 0 {
        heads = heads.wrapping_add(1);
        heads;
        temp = cylinders.wrapping_mul(heads);
        sectors = capacity.wrapping_div(temp);
        if capacity.wrapping_rem(temp) != 0 {
            sectors = sectors.wrapping_add(1);
            sectors;
            temp = heads.wrapping_mul(sectors);
            cylinders = capacity.wrapping_div(temp);
        }
    }
    if cylinders == 0 as libc::c_int as libc::c_ulong {
        rv = -(1 as libc::c_int);
    }
    *cyls = cylinders as libc::c_uint;
    *secs = sectors as uint16_t;
    *hds = heads as uint16_t;
    return rv;
}
unsafe extern "C" fn setsize0(
    mut capacity: uint32_t,
    mut cyls: *mut libc::c_uint,
    mut hds: *mut uint16_t,
    mut secs: *mut uint16_t,
) {
    let mut r: libc::c_int = 0;
    if capacity < (1024 as libc::c_int * 2048 as libc::c_int) as libc::c_uint
        && capacity.wrapping_rem(1024 as libc::c_int as libc::c_uint) == 0
    {
        *cyls = capacity >> 11 as libc::c_int;
        *hds = 64 as libc::c_int as uint16_t;
        *secs = 32 as libc::c_int as uint16_t;
        return;
    }
    r = setsize(capacity as libc::c_ulong, cyls, hds, secs);
    if r != 0 || *hds as libc::c_int > 255 as libc::c_int
        || *secs as libc::c_int > 63 as libc::c_int
    {
        *cyls = capacity >> 11 as libc::c_int;
        *hds = 64 as libc::c_int as uint16_t;
        *secs = 32 as libc::c_int as uint16_t;
        return;
    }
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
        b"Usage: %s [-pradcv] [-I] [-B bootsect-template] [-s sectors] [-t cylinders] [-h heads] [-T type] [-b begin] [-l length] drive\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn mpartition(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut dummy: libc::c_int,
) {
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut dummy2: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut sec_per_cyl: uint16_t = 0;
    let mut doprint: libc::c_int = 0 as libc::c_int;
    let mut verbose: libc::c_int = 0 as libc::c_int;
    let mut create: libc::c_int = 0 as libc::c_int;
    let mut force: libc::c_int = 0 as libc::c_int;
    let mut length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut do_remove: libc::c_int = 0 as libc::c_int;
    let mut initialize: libc::c_int = 0 as libc::c_int;
    let mut tot_sectors: uint32_t = 0 as libc::c_int as uint32_t;
    let mut type_0: uint8_t = 0 as libc::c_int as uint8_t;
    let mut begin_set: libc::c_int = 0 as libc::c_int;
    let mut size_set: libc::c_int = 0 as libc::c_int;
    let mut end_set: libc::c_int = 0 as libc::c_int;
    let mut activate: libc::c_int = 0 as libc::c_int;
    let mut has_activated: libc::c_int = 0 as libc::c_int;
    let mut inconsistency: libc::c_int = 0 as libc::c_int;
    let mut begin: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut end: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dirty: libc::c_int = 0 as libc::c_int;
    let mut open2flags: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut used_dev: device = device {
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
    let mut argtracks: libc::c_uint = 0;
    let mut argheads: uint16_t = 0;
    let mut argsectors: uint16_t = 0;
    let mut drive: libc::c_char = 0;
    let mut name: [libc::c_char; 2048] = [0; 2048];
    let mut buf: [libc::c_uchar; 512] = [0; 512];
    let mut partTable: *mut partition = buf
        .as_mut_ptr()
        .offset(0x1ae as libc::c_int as isize) as *mut partition;
    let mut dev: *mut device = 0 as *mut device;
    let mut errmsg: [libc::c_char; 2100] = [0; 2100];
    let mut bootSector: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tpartition: *mut partition = 0 as *mut partition;
    argtracks = 0 as libc::c_int as libc::c_uint;
    argheads = 0 as libc::c_int as uint16_t;
    argsectors = 0 as libc::c_int as uint16_t;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(
            argc,
            argv,
            b"i:adprcIT:t:h:s:fvpb:l:S:B:\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            66 => {
                bootSector = optarg;
            }
            97 => {
                open2flags |= 1 as libc::c_int;
                activate = 1 as libc::c_int;
                dirty = 1 as libc::c_int;
            }
            100 => {
                activate = -(1 as libc::c_int);
                dirty = 1 as libc::c_int;
            }
            112 => {
                doprint = 1 as libc::c_int;
            }
            114 => {
                do_remove = 1 as libc::c_int;
                dirty = 1 as libc::c_int;
            }
            73 => {
                open2flags |= 1 as libc::c_int;
                initialize = 1 as libc::c_int;
                dirty = 1 as libc::c_int;
            }
            99 => {
                create = 1 as libc::c_int;
                dirty = 1 as libc::c_int;
            }
            84 => {
                open2flags |= 1 as libc::c_int;
                type_0 = strtou8(optarg, &mut endptr, 0 as libc::c_int);
            }
            116 => {
                argtracks = atoui(optarg);
            }
            104 => {
                argheads = atou16(optarg);
            }
            115 => {
                argsectors = atou16(optarg);
            }
            102 => {
                open2flags |= 1 as libc::c_int;
                force = 1 as libc::c_int;
            }
            118 => {
                verbose += 1;
                verbose;
            }
            98 => {
                begin_set = 1 as libc::c_int;
                begin = strtoui(optarg, &mut endptr, 0 as libc::c_int);
            }
            108 => {
                size_set = 1 as libc::c_int;
                length = parseSize(optarg);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
        check_number_parse_errno(c as libc::c_char, optarg, endptr);
    }
    if argc - optind != 1 as libc::c_int
        || *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize) == 0
        || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int != ':' as i32
    {
        usage(1 as libc::c_int);
    }
    drive = ch_toupper(
        *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize),
    );
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const libc::c_char,
        drive as libc::c_int,
    );
    Stream = 0 as *mut Stream_t;
    dev = devices;
    while (*dev).drive != 0 {
        let mut mode: libc::c_int = 0;
        free_stream(&mut Stream);
        if !((*dev).drive as libc::c_int != drive as libc::c_int) {
            if (*dev).partition < 1 as libc::c_int as libc::c_uint
                || (*dev).partition > 4 as libc::c_int as libc::c_uint
            {
                sprintf(
                    errmsg.as_mut_ptr(),
                    b"Drive '%c:' is not a partition\0" as *const u8
                        as *const libc::c_char,
                    drive as libc::c_int,
                );
            } else {
                used_dev = *dev;
                if argtracks != 0 {
                    used_dev.tracks = argtracks;
                }
                if argheads != 0 {
                    used_dev.heads = argheads;
                }
                if argsectors != 0 {
                    used_dev.sectors = argsectors;
                }
                expand((*dev).name, name.as_mut_ptr());
                mode = if dirty != 0 { 0o2 as libc::c_int } else { 0 as libc::c_int };
                if initialize != 0 {
                    mode |= 0o100 as libc::c_int;
                }
                Stream = OpenImage(
                    &mut used_dev,
                    dev,
                    name.as_mut_ptr(),
                    mode,
                    errmsg.as_mut_ptr(),
                    open2flags | 2 as libc::c_int | 4 as libc::c_int,
                    mode,
                    0 as *mut mt_off_t,
                    0 as *mut libc::c_int,
                    0 as *mut xdf_info,
                );
                if Stream.is_null() {
                    snprintf(
                        errmsg.as_mut_ptr(),
                        (::std::mem::size_of::<[libc::c_char; 2100]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"init: open: %s\0" as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                } else {
                    tot_sectors = used_dev.tot_sectors;
                    if ((*(*Stream).Class).pread)
                        .unwrap()(
                        Stream,
                        buf.as_mut_ptr() as *mut libc::c_char,
                        0 as libc::c_int as mt_off_t,
                        512 as libc::c_int as size_t,
                    ) != 512 as libc::c_int as libc::c_long && initialize == 0
                    {
                        snprintf(
                            errmsg.as_mut_ptr(),
                            (::std::mem::size_of::<[libc::c_char; 2100]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            b"Error reading from '%s', wrong parameters?\0" as *const u8
                                as *const libc::c_char,
                            name.as_mut_ptr(),
                        );
                    } else {
                        if verbose >= 2 as libc::c_int {
                            print_sector(
                                b"Read sector\0" as *const u8 as *const libc::c_char,
                                buf.as_mut_ptr(),
                                512 as libc::c_int,
                            );
                        }
                        break;
                    }
                }
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if (*dev).drive as libc::c_int == 0 as libc::c_int {
        free_stream(&mut Stream);
        fprintf(
            stderr,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            errmsg.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if (used_dev.sectors as libc::c_int != 0 || used_dev.heads as libc::c_int != 0)
        && (used_dev.sectors == 0 || used_dev.heads == 0)
    {
        fprintf(
            stderr,
            b"You should either indicate both the number of sectors and the number of heads,\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(stderr, b" or none of them\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if initialize != 0 {
        if !bootSector.is_null() {
            let mut fd: libc::c_int = 0;
            fd = open(
                bootSector,
                0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int,
            );
            if fd < 0 as libc::c_int {
                perror(b"open MBR\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            if read(
                fd,
                buf.as_mut_ptr() as *mut libc::c_char as *mut libc::c_void,
                512 as libc::c_int as size_t,
            ) < 512 as libc::c_int as libc::c_long
            {
                perror(b"read MBR\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        }
        memset(
            partTable.offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<partition>() as libc::c_ulong),
        );
        set_word(
            buf.as_mut_ptr().offset(510 as libc::c_int as isize),
            0xaa55 as libc::c_int as libc::c_ushort,
        );
    }
    if buf[510 as libc::c_int as usize] as libc::c_int != 0x55 as libc::c_int
        || buf[511 as libc::c_int as usize] as libc::c_int != 0xaa as libc::c_int
    {
        fprintf(
            stderr,
            b"Boot signature not set\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Use the -I flag to initialize the partition table, and set the boot signature\n\0"
                as *const u8 as *const libc::c_char,
        );
        inconsistency = 1 as libc::c_int;
    }
    tpartition = &mut *partTable.offset((*dev).partition as isize) as *mut partition;
    if do_remove != 0 {
        if (*tpartition).end.byte0 == 0 {
            fprintf(
                stderr,
                b"Partition for drive %c: does not exist\n\0" as *const u8
                    as *const libc::c_char,
                drive as libc::c_int,
            );
        }
        if (*tpartition).end.byte0 as libc::c_int & 0x3f as libc::c_int
            == 5 as libc::c_int
        {
            fprintf(
                stderr,
                b"Partition for drive %c: may be an extended partition\n\0" as *const u8
                    as *const libc::c_char,
                drive as libc::c_int,
            );
            fprintf(
                stderr,
                b"Use the -f flag to remove it anyways\n\0" as *const u8
                    as *const libc::c_char,
            );
            inconsistency = 1 as libc::c_int;
        }
        memset(
            tpartition as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<partition>() as libc::c_ulong,
        );
    }
    if create != 0 && (*tpartition).end.byte0 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"Partition for drive %c: already exists\n\0" as *const u8
                as *const libc::c_char,
            drive as libc::c_int,
        );
        fprintf(
            stderr,
            b"Use the -r flag to remove it before attempting to recreate it\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    compute_lba_geom_from_tot_sectors(&mut used_dev);
    has_activated = 0 as libc::c_int;
    i = 1 as libc::c_int as libc::c_uint;
    while i < 5 as libc::c_int as libc::c_uint {
        let mut partition: *mut partition = &mut *partTable.offset(i as isize)
            as *mut partition;
        if !((*partition).end.byte0 == 0) {
            if (*partition).start.byte0 != 0 {
                has_activated += 1;
                has_activated;
            }
            if i < (*dev).partition && begin_set == 0 {
                begin = ((((*partition).start_sect[0 as libc::c_int as usize]
                    as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t)
                    .wrapping_add(
                        (((*partition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                            + (((*partition).nr_sects[1 as libc::c_int as usize]
                                as libc::c_int) << 8 as libc::c_int)) as uint16_t
                            as libc::c_int
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as isize)
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as libc::c_int as isize)
                                    .offset(1 as libc::c_int as isize) as libc::c_int)
                                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                                << 16 as libc::c_int)) as uint32_t,
                    );
            }
            if i > (*dev).partition && end_set == 0 && size_set == 0 {
                end = (((*partition).start_sect[0 as libc::c_int as usize] as libc::c_int
                    + (((*partition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t;
                end_set = 1 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if used_dev.sectors == 0 && used_dev.heads == 0 {
        if tot_sectors != 0 {
            setsize0(
                tot_sectors,
                &mut dummy2,
                &mut used_dev.heads,
                &mut used_dev.sectors,
            );
        } else {
            used_dev.heads = 64 as libc::c_int as uint16_t;
            used_dev.sectors = 32 as libc::c_int as uint16_t;
        }
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"sectors: %d heads: %d %u\n\0" as *const u8 as *const libc::c_char,
            used_dev.sectors as libc::c_int,
            used_dev.heads as libc::c_int,
            tot_sectors,
        );
    }
    sec_per_cyl = (used_dev.sectors as libc::c_int * used_dev.heads as libc::c_int)
        as uint16_t;
    if create != 0 {
        let mut overlap: libc::c_uint = 0;
        if end_set == 0 && size_set == 0 && tot_sectors != 0 {
            end = tot_sectors
                .wrapping_sub(tot_sectors.wrapping_rem(sec_per_cyl as libc::c_uint));
            end_set = 1 as libc::c_int;
        }
        if begin == 0 && begin_set == 0 {
            begin = (if used_dev.sectors as libc::c_int != 0 {
                used_dev.sectors as libc::c_int
            } else {
                2048 as libc::c_int
            }) as libc::c_uint;
        }
        if size_set != 0 {
            end = begin.wrapping_add(length);
        } else if end_set == 0 {
            fprintf(stderr, b"Unknown size\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        if begin >= end {
            fprintf(
                stderr,
                b"Begin larger than end\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        overlap = findOverlap(partTable, 4 as libc::c_int as libc::c_uint, begin, end);
        if overlap != 0 {
            fprintf(
                stderr,
                b"Partition would overlap with partition %d\n\0" as *const u8
                    as *const libc::c_char,
                overlap,
            );
            exit(1 as libc::c_int);
        }
        setBeginEnd(
            tpartition,
            begin,
            end,
            used_dev.heads,
            used_dev.sectors,
            (has_activated == 0) as libc::c_int,
            type_0,
            (if (*dev).fat_bits > 0 as libc::c_int {
                (*dev).fat_bits
            } else {
                -(*dev).fat_bits
            }) as libc::c_uint,
        );
    }
    if activate != 0 {
        if (*tpartition).end.byte0 == 0 {
            fprintf(
                stderr,
                b"Partition for drive %c: does not exist\n\0" as *const u8
                    as *const libc::c_char,
                drive as libc::c_int,
            );
        } else {
            match activate {
                1 => {
                    (*tpartition).start.byte0 = 0x80 as libc::c_int as libc::c_uchar;
                }
                -1 => {
                    (*tpartition).start.byte0 = 0 as libc::c_int as libc::c_uchar;
                }
                _ => {}
            }
        }
    }
    inconsistency
        |= consistencyCheck(
            partTable,
            doprint,
            verbose,
            &mut has_activated,
            tot_sectors,
            &mut used_dev,
            (*dev).partition,
        );
    match has_activated {
        0 => {
            fprintf(
                stderr,
                b"Warning: no active (bootable) partition present\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        1 => {}
        _ => {
            fprintf(
                stderr,
                b"Warning: %d active (bootable) partitions present\n\0" as *const u8
                    as *const libc::c_char,
                has_activated,
            );
            fprintf(
                stderr,
                b"Usually, a disk should have exactly one active partition\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    if inconsistency != 0 && force == 0 {
        fprintf(
            stderr,
            b"inconsistency detected!\n\0" as *const u8 as *const libc::c_char,
        );
        if dirty != 0 {
            fprintf(
                stderr,
                b"Retry with the -f switch to go ahead anyways\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if doprint != 0 && (*tpartition).end.byte0 as libc::c_int != 0 {
        printf(
            b"The following command will recreate the partition for drive %c:\n\0"
                as *const u8 as *const libc::c_char,
            drive as libc::c_int,
        );
        used_dev
            .tracks = ((((*tpartition).nr_sects[0 as libc::c_int as usize] as libc::c_int
            + (((*tpartition).nr_sects[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int
            + (((*((*tpartition).nr_sects)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int
                + ((*((*tpartition).nr_sects)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                << 16 as libc::c_int)) as uint32_t)
            .wrapping_add(
                ((((*tpartition).start_sect[0 as libc::c_int as usize] as libc::c_int
                    + (((*tpartition).start_sect[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                    + (((*((*tpartition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as libc::c_int
                        + ((*((*tpartition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int)) as uint16_t as libc::c_int)
                        << 16 as libc::c_int)) as uint32_t)
                    .wrapping_rem(sec_per_cyl as libc::c_uint),
            )
            .wrapping_div(sec_per_cyl as libc::c_uint);
        printf(
            b"mpartition -c -b %d -l %d -t %d -h %d -s %d -b %u %c:\n\0" as *const u8
                as *const libc::c_char,
            (((*tpartition).start_sect[0 as libc::c_int as usize] as libc::c_int
                + (((*tpartition).start_sect[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*tpartition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*tpartition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
            (((*tpartition).nr_sects[0 as libc::c_int as usize] as libc::c_int
                + (((*tpartition).nr_sects[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*tpartition).nr_sects)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*tpartition).nr_sects)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
            used_dev.tracks,
            used_dev.heads as libc::c_int,
            used_dev.sectors as libc::c_int,
            (((*tpartition).start_sect[0 as libc::c_int as usize] as libc::c_int
                + (((*tpartition).start_sect[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*tpartition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*tpartition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
            drive as libc::c_int,
        );
    }
    if dirty != 0 {
        if verbose >= 2 as libc::c_int {
            print_sector(
                b"Writing sector\0" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
                512 as libc::c_int,
            );
        }
        if ((*(*Stream).Class).pwrite)
            .unwrap()(
            Stream,
            buf.as_mut_ptr() as *mut libc::c_char,
            0 as libc::c_int as mt_off_t,
            512 as libc::c_int as size_t,
        ) != 512 as libc::c_int as libc::c_long
        {
            fprintf(
                stderr,
                b"Error writing partition table\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if verbose >= 3 as libc::c_int {
            print_sector(
                b"Sector written\0" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
                512 as libc::c_int,
            );
        }
    }
    free_stream(&mut Stream);
    exit(0 as libc::c_int);
}
