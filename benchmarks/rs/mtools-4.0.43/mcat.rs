use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn get_default_drive() -> libc::c_char;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn chs_to_totsectors(dev: *mut device, errmsg: *mut libc::c_char) -> libc::c_int;
    fn check_if_sectors_fit(
        tot_sectors: uint32_t,
        maxBytes: mt_off_t,
        sectorSize: uint32_t,
        errmsg: *mut libc::c_char,
    ) -> libc::c_int;
    static mut devices: *mut device;
    fn expand(_: *const libc::c_char, _: *mut libc::c_char) -> *const libc::c_char;
    static mut mdate: *const libc::c_char;
    static mut mversion: *const libc::c_char;
    static mut noPrivileges: libc::c_int;
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
unsafe extern "C" fn usage() -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: mcat [-V] [-w] device\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       -w write on device else read\n\0" as *const u8 as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn bufLen(
    mut blocksize: size_t,
    mut totalSize: mt_off_t,
    mut address: mt_off_t,
) -> size_t {
    if totalSize == 0 as libc::c_int as libc::c_long {
        return blocksize;
    }
    if blocksize as mt_off_t > totalSize - address {
        return (totalSize - address) as size_t;
    }
    return blocksize;
}
pub unsafe extern "C" fn mcat(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut current_block: u64;
    let mut dev: *mut device = 0 as *mut device;
    let mut out_dev: device = device {
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
    let mut drive: libc::c_char = 0;
    let mut name: [libc::c_char; 2048] = [0; 2048];
    let mut errmsg: [libc::c_char; 200] = [0; 200];
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut buf: [libc::c_char; 16000] = [0; 16000];
    let mut address: mt_off_t = 0 as libc::c_int as mt_off_t;
    let mut maxSize: mt_off_t = 0 as libc::c_int as mt_off_t;
    let mut mode: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut c: libc::c_int = 0;
    noPrivileges = 1 as libc::c_int;
    if argc < 2 as libc::c_int {
        usage();
    }
    loop {
        c = getopt(argc, argv, b"wi:\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            119 => {
                mode = 0o1 as libc::c_int as libc::c_char;
            }
            105 => {
                set_cmd_line_image(optarg);
            }
            _ => {
                usage();
            }
        }
    }
    if argc - optind > 1 as libc::c_int {
        usage();
    }
    if argc - optind == 1 as libc::c_int {
        if *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize) == 0
            || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != ':' as i32
        {
            usage();
        }
        drive = ch_toupper(
            *(*argv.offset((argc - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize),
        );
    } else {
        drive = get_default_drive();
    }
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const libc::c_char,
        drive as libc::c_int,
    );
    Stream = 0 as *mut Stream_t;
    dev = devices;
    while !((*dev).name).is_null() {
        free_stream(&mut Stream);
        if !((*dev).drive as libc::c_int != drive as libc::c_int) {
            out_dev = *dev;
            expand((*dev).name, name.as_mut_ptr());
            Stream = OpenImage(
                &mut out_dev,
                dev,
                name.as_mut_ptr(),
                mode as libc::c_int,
                errmsg.as_mut_ptr(),
                4 as libc::c_int,
                mode as libc::c_int,
                &mut maxSize,
                0 as *mut libc::c_int,
                0 as *mut xdf_info,
            );
            if !Stream.is_null() {
                break;
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if !((*dev).drive as libc::c_int == 0 as libc::c_int) {
        if mode as libc::c_int == 0o1 as libc::c_int {
            let mut len: size_t = 0;
            let mut size: mt_off_t = 0 as libc::c_int as mt_off_t;
            if chs_to_totsectors(&mut out_dev, errmsg.as_mut_ptr()) < 0 as libc::c_int
                || check_if_sectors_fit(
                    out_dev.tot_sectors,
                    maxSize,
                    512 as libc::c_int as uint32_t,
                    errmsg.as_mut_ptr(),
                ) != 0
            {
                current_block = 14227524731022472268;
            } else {
                size = 512 as libc::c_int as libc::c_long
                    * out_dev.tot_sectors as mt_off_t;
                loop {
                    len = fread(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        bufLen(16000 as libc::c_uint as size_t, size, address),
                        stdin,
                    );
                    if !(len > 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    let mut r: ssize_t = ((*(*Stream).Class).pwrite)
                        .unwrap()(Stream, buf.as_mut_ptr(), address, len);
                    fprintf(
                        stderr,
                        b"Wrote to %d\n\0" as *const u8 as *const libc::c_char,
                        address as libc::c_int,
                    );
                    if r < 0 as libc::c_int as libc::c_long {
                        break;
                    }
                    address = (address as libc::c_ulong).wrapping_add(len) as mt_off_t
                        as mt_off_t;
                }
                current_block = 5330834795799507926;
            }
        } else {
            let mut len_0: ssize_t = 0;
            loop {
                len_0 = ((*(*Stream).Class).pread)
                    .unwrap()(
                    Stream,
                    buf.as_mut_ptr(),
                    address,
                    16000 as libc::c_uint as size_t,
                );
                if !(len_0 > 0 as libc::c_int as libc::c_long) {
                    break;
                }
                fwrite(
                    buf.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    len_0 as size_t,
                    stdout,
                );
                address = (address as libc::c_ulong).wrapping_add(len_0 as size_t)
                    as mt_off_t as mt_off_t;
            }
            current_block = 5330834795799507926;
        }
        match current_block {
            14227524731022472268 => {}
            _ => {
                free_stream(&mut Stream);
                exit(0 as libc::c_int);
            }
        }
    }
    free_stream(&mut Stream);
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, errmsg.as_mut_ptr());
    exit(1 as libc::c_int);
}
