use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    pub type Fs_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn towlower(__wc: wint_t) -> wint_t;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn getfree(Stream: *mut Stream_t) -> mt_off_t;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    static mut mtools_ignore_short_case: libc::c_uint;
    static mut mtools_dotted_dir: libc::c_uint;
    static mut mtools_twenty_four_hour_clock: libc::c_uint;
    static mut mtools_date_string: *const libc::c_char;
    fn set_cmd_line_image(img: *mut libc::c_char);
    static mut progname: *const libc::c_char;
    fn getDrive(Stream: *mut Stream_t) -> libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut mversion: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    fn OpenRoot(Dir: *mut Stream_t) -> *mut Stream_t;
    fn vfat_lookup(
        entry: *mut direntry_t,
        filename: *const libc::c_char,
        length: size_t,
        flags: libc::c_int,
        shortname: *mut libc::c_char,
        shortname_len: size_t,
        longname: *mut libc::c_char,
        longname_len: size_t,
    ) -> libc::c_int;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn getPwd(entry: *mut direntry_t) -> *mut libc::c_char;
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(
        MainParam: *mut MainParam_t,
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
    ) -> libc::c_int;
    fn getSerialized(File: *mut Fs_t) -> bool;
    fn getSerialNumber(File: *mut Fs_t) -> libc::c_ulong;
    fn dos_to_wchar(
        fromDos: *mut doscp_t,
        dos: *const libc::c_char,
        wchar: *mut wchar_t,
        len: size_t,
    ) -> size_t;
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut libc::c_char,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
    fn isSpecialW(name: *const wchar_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type wchar_t = libc::c_int;
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
pub type wint_t = libc::c_uint;
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
pub struct directory {
    pub name: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub attr: libc::c_uchar,
    pub Case: libc::c_uchar,
    pub ctime_ms: libc::c_uchar,
    pub ctime: [libc::c_uchar; 2],
    pub cdate: [libc::c_uchar; 2],
    pub adate: [libc::c_uchar; 2],
    pub startHi: [libc::c_uchar; 2],
    pub time: [libc::c_uchar; 2],
    pub date: [libc::c_uchar; 2],
    pub start: [libc::c_uchar; 2],
    pub size: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MainParam_t {
    pub loop_0: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut MainParam_t,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub dirCallback: Option::<
        unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    >,
    pub callback: Option::<
        unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    >,
    pub unixcallback: Option::<unsafe extern "C" fn(*mut MainParam_t) -> libc::c_int>,
    pub arg: *mut libc::c_void,
    pub openflags: libc::c_int,
    pub lookupflags: libc::c_int,
    pub fast_quit: libc::c_int,
    pub shortname: bounded_string,
    pub longname: bounded_string,
    pub File: *mut Stream_t,
    pub direntry: *mut direntry_t,
    pub unixSourceName: *mut libc::c_char,
    pub targetDir: *mut Stream_t,
    pub targetName: *const libc::c_char,
    pub originalArg: *mut libc::c_char,
    pub basenameHasWildcard: libc::c_int,
    pub mcwd: [libc::c_char; 132],
    pub targetBuffer: [libc::c_char; 1021],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: libc::c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bounded_string {
    pub data: *mut libc::c_char,
    pub len: size_t,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn ch_towlower(mut ch: wchar_t) -> wchar_t {
    return towlower(ch as wint_t) as wchar_t;
}
static mut recursive: libc::c_int = 0;
static mut wide: libc::c_int = 0;
static mut all: libc::c_int = 0;
static mut concise: libc::c_int = 0;
static mut fast: libc::c_int = 0 as libc::c_int;
static mut dirPath: *const libc::c_char = 0 as *const libc::c_char;
static mut dynDirPath: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut currentDrive: libc::c_char = 0;
static mut currentDir: *mut Stream_t = 0 as *const Stream_t as *mut Stream_t;
static mut filesInDir: libc::c_int = 0;
static mut filesOnDrive: libc::c_int = 0;
static mut dirsOnDrive: libc::c_int = 0;
static mut debug: libc::c_int = 0 as libc::c_int;
static mut bytesInDir: mt_off_t = 0;
static mut bytesOnDrive: mt_off_t = 0;
static mut RootDir: *mut Stream_t = 0 as *const Stream_t as *mut Stream_t;
static mut mdir_shortname: [libc::c_char; 49] = [0; 49];
static mut mdir_longname: [libc::c_char; 1021] = [0; 1021];
#[inline]
unsafe extern "C" fn print_date(mut dir: *mut directory) {
    let mut year: [libc::c_char; 5] = [0; 5];
    let mut day: [libc::c_char; 3] = [0; 3];
    let mut month: [libc::c_char; 3] = [0; 3];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    sprintf(
        year.as_mut_ptr(),
        b"%04d\0" as *const u8 as *const libc::c_char,
        ((*dir).date[1 as libc::c_int as usize] as libc::c_int >> 1 as libc::c_int)
            + 1980 as libc::c_int,
    );
    sprintf(
        day.as_mut_ptr(),
        b"%02d\0" as *const u8 as *const libc::c_char,
        (*dir).date[0 as libc::c_int as usize] as libc::c_int & 0x1f as libc::c_int,
    );
    sprintf(
        month.as_mut_ptr(),
        b"%02d\0" as *const u8 as *const libc::c_char,
        (((*dir).date[1 as libc::c_int as usize] as libc::c_int & 0x1 as libc::c_int)
            << 3 as libc::c_int)
            + ((*dir).date[0 as libc::c_int as usize] as libc::c_int >> 5 as libc::c_int),
    );
    p = mtools_date_string;
    while *p != 0 {
        if strncasecmp(
            p,
            b"yyyy\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            printf(
                b"%04d\0" as *const u8 as *const libc::c_char,
                ((*dir).date[1 as libc::c_int as usize] as libc::c_int
                    >> 1 as libc::c_int) + 1980 as libc::c_int,
            );
            p = p.offset(3 as libc::c_int as isize);
        } else if strncasecmp(
            p,
            b"yy\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            printf(
                b"%02d\0" as *const u8 as *const libc::c_char,
                (((*dir).date[1 as libc::c_int as usize] as libc::c_int
                    >> 1 as libc::c_int) + 1980 as libc::c_int) % 100 as libc::c_int,
            );
            p = p.offset(1);
            p;
        } else if strncasecmp(
            p,
            b"dd\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            printf(
                b"%02d\0" as *const u8 as *const libc::c_char,
                (*dir).date[0 as libc::c_int as usize] as libc::c_int
                    & 0x1f as libc::c_int,
            );
            p = p.offset(1);
            p;
        } else if strncasecmp(
            p,
            b"mm\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            printf(
                b"%02d\0" as *const u8 as *const libc::c_char,
                (((*dir).date[1 as libc::c_int as usize] as libc::c_int
                    & 0x1 as libc::c_int) << 3 as libc::c_int)
                    + ((*dir).date[0 as libc::c_int as usize] as libc::c_int
                        >> 5 as libc::c_int),
            );
            p = p.offset(1);
            p;
        } else {
            putchar(*p as libc::c_int);
        }
        p = p.offset(1);
        p;
    }
}
#[inline]
unsafe extern "C" fn print_time(mut dir: *mut directory) {
    let mut am_pm: libc::c_char = 0;
    let mut hour: libc::c_int = (*dir).time[1 as libc::c_int as usize] as libc::c_int
        >> 3 as libc::c_int;
    if mtools_twenty_four_hour_clock == 0 {
        am_pm = (if hour >= 12 as libc::c_int { 'p' as i32 } else { 'a' as i32 })
            as libc::c_char;
        if hour > 12 as libc::c_int {
            hour = hour - 12 as libc::c_int;
        }
        if hour == 0 as libc::c_int {
            hour = 12 as libc::c_int;
        }
    } else {
        am_pm = ' ' as i32 as libc::c_char;
    }
    printf(
        b"%2d:%02d%c\0" as *const u8 as *const libc::c_char,
        hour,
        (((*dir).time[1 as libc::c_int as usize] as libc::c_int & 0x7 as libc::c_int)
            << 3 as libc::c_int)
            + ((*dir).time[0 as libc::c_int as usize] as libc::c_int
                >> 5 as libc::c_int),
        am_pm as libc::c_int,
    );
}
unsafe extern "C" fn dotted_num(
    mut num: mt_off_t,
    mut width: size_t,
    mut buf: *mut *mut libc::c_char,
) -> *const libc::c_char {
    let mut len: size_t = 0;
    let mut srcp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dstp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut numlo: libc::c_ulong = 0;
    let mut numhi: libc::c_ulong = 0;
    size = width.wrapping_add(width);
    *buf = malloc(size.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if (*buf).is_null() {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    numlo = (num % 1000000000 as libc::c_int as libc::c_long) as libc::c_ulong;
    numhi = (num / 1000000000 as libc::c_int as libc::c_long) as libc::c_ulong;
    if numhi != 0 && size > 9 as libc::c_int as libc::c_ulong {
        sprintf(
            *buf,
            b"%.*lu%09lu\0" as *const u8 as *const libc::c_char,
            size.wrapping_sub(9 as libc::c_int as libc::c_ulong) as libc::c_int,
            numhi,
            numlo,
        );
    } else {
        sprintf(
            *buf,
            b"%.*lu\0" as *const u8 as *const libc::c_char,
            size as libc::c_int,
            numlo,
        );
    }
    srcp = *buf;
    while *srcp.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        if !(*srcp.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32) {
            break;
        }
        *srcp.offset(0 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
        srcp = srcp.offset(1);
        srcp;
    }
    len = strlen(*buf);
    srcp = (*buf).offset(len as isize);
    dstp = (*buf).offset(len as isize).offset(1 as libc::c_int as isize);
    while dstp >= (*buf).offset(4 as libc::c_int as isize)
        && *(*__ctype_b_loc())
            .offset(
                *srcp.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                    as libc::c_int as isize,
            ) as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        srcp = srcp.offset(-(3 as libc::c_int as isize));
        dstp = dstp.offset(-(4 as libc::c_int as isize));
    }
    while dstp < (*buf).offset(len as isize) {
        *dstp
            .offset(0 as libc::c_int as isize) = *srcp.offset(0 as libc::c_int as isize);
        *dstp
            .offset(1 as libc::c_int as isize) = *srcp.offset(1 as libc::c_int as isize);
        *dstp
            .offset(2 as libc::c_int as isize) = *srcp.offset(2 as libc::c_int as isize);
        if dstp.offset(3 as libc::c_int as isize) < (*buf).offset(len as isize) {
            *dstp.offset(3 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
        }
        srcp = srcp.offset(3 as libc::c_int as isize);
        dstp = dstp.offset(4 as libc::c_int as isize);
    }
    return (*buf).offset(len as isize).offset(-(width as isize));
}
#[inline]
unsafe extern "C" fn print_volume_label(
    mut Dir: *mut Stream_t,
    mut drive: libc::c_char,
) -> libc::c_int {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut shortname: [libc::c_char; 13] = [0; 13];
    let mut longname: [libc::c_char; 261] = [0; 261];
    let mut r: libc::c_int = 0;
    RootDir = OpenRoot(Stream);
    if concise != 0 {
        return 0 as libc::c_int;
    }
    initializeDirentry(&mut entry, RootDir);
    r = vfat_lookup(
        &mut entry,
        0 as *const libc::c_char,
        0 as libc::c_int as size_t,
        0x8 as libc::c_int | 0x40 as libc::c_int,
        shortname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
        longname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 261]>() as libc::c_ulong,
    );
    if r != 0 {
        if r == -(2 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        printf(
            b" Volume in drive %c has no label\0" as *const u8 as *const libc::c_char,
            drive as libc::c_int,
        );
    } else if *longname.as_mut_ptr() != 0 {
        printf(
            b" Volume in drive %c is %s (abbr=%s)\0" as *const u8 as *const libc::c_char,
            drive as libc::c_int,
            longname.as_mut_ptr(),
            shortname.as_mut_ptr(),
        );
    } else {
        printf(
            b" Volume in drive %c is %s\0" as *const u8 as *const libc::c_char,
            drive as libc::c_int,
            shortname.as_mut_ptr(),
        );
    }
    if getSerialized(This) {
        let mut serial_number: libc::c_ulong = getSerialNumber(This);
        printf(
            b"\n Volume Serial Number is %04lX-%04lX\0" as *const u8
                as *const libc::c_char,
            serial_number >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong,
            serial_number & 0xffff as libc::c_int as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn printSummary(mut files: libc::c_int, mut bytes: mt_off_t) {
    if filesInDir == 0 {
        printf(b"No files\n\0" as *const u8 as *const libc::c_char);
    } else {
        let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
        printf(b"      %3d file\0" as *const u8 as *const libc::c_char, files);
        if files == 1 as libc::c_int {
            putchar(' ' as i32);
        } else {
            putchar('s' as i32);
        }
        printf(
            b"       %s bytes\n\0" as *const u8 as *const libc::c_char,
            dotted_num(bytes, 13 as libc::c_int as size_t, &mut s1),
        );
        if !s1.is_null() {
            free(s1 as *mut libc::c_void);
        }
    };
}
unsafe extern "C" fn leaveDrive(mut haveError: libc::c_int) {
    if currentDrive == 0 {
        return;
    }
    leaveDirectory(haveError);
    if concise == 0 && haveError == 0 {
        if dirsOnDrive > 1 as libc::c_int {
            printf(b"\nTotal files listed:\n\0" as *const u8 as *const libc::c_char);
            printSummary(filesOnDrive, bytesOnDrive);
        }
        if !RootDir.is_null() && fast == 0 {
            let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut bytes: mt_off_t = getfree(RootDir);
            if bytes == -(1 as libc::c_int) as libc::c_long {
                fprintf(stderr, b"Fat error\n\0" as *const u8 as *const libc::c_char);
            } else {
                printf(
                    b"                  %s bytes free\n\n\0" as *const u8
                        as *const libc::c_char,
                    dotted_num(bytes, 17 as libc::c_int as size_t, &mut s1),
                );
                if !s1.is_null() {
                    free(s1 as *mut libc::c_void);
                }
            }
        }
    }
    free_stream(&mut RootDir);
    currentDrive = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn enterDrive(
    mut Dir: *mut Stream_t,
    mut drive: libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if currentDrive as libc::c_int == drive as libc::c_int {
        return 0 as libc::c_int;
    }
    leaveDrive(0 as libc::c_int);
    currentDrive = drive;
    r = print_volume_label(Dir, drive);
    if r != 0 {
        return r;
    }
    bytesOnDrive = 0 as libc::c_int as mt_off_t;
    filesOnDrive = 0 as libc::c_int;
    dirsOnDrive = 0 as libc::c_int;
    return 0 as libc::c_int;
}
static mut emptyString: *const libc::c_char = b"<out-of-memory>\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn leaveDirectory(mut haveError: libc::c_int) {
    if currentDir.is_null() {
        return;
    }
    if haveError == 0 {
        if !dirPath.is_null() && dirPath != emptyString {
            free(dynDirPath as *mut libc::c_void);
        }
        if wide != 0 {
            putchar('\n' as i32);
        }
        if concise == 0 {
            printSummary(filesInDir, bytesInDir);
        }
    }
    free_stream(&mut currentDir);
}
unsafe extern "C" fn enterDirectory(mut Dir: *mut Stream_t) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut drive: libc::c_char = 0;
    if currentDir == Dir {
        return 0 as libc::c_int;
    }
    leaveDirectory(0 as libc::c_int);
    drive = getDrive(Dir);
    r = enterDrive(Dir, drive);
    if r != 0 {
        return r;
    }
    currentDir = copy_stream(Dir);
    dynDirPath = getPwd(getDirentry(Dir));
    if dynDirPath.is_null() {
        dirPath = emptyString;
    } else {
        if *dynDirPath.offset(3 as libc::c_int as isize) == 0 && concise != 0 {
            *dynDirPath.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        dirPath = dynDirPath;
    }
    if concise == 0 {
        printf(b"\nDirectory for %s\n\0" as *const u8 as *const libc::c_char, dirPath);
    }
    if wide == 0 && concise == 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    dirsOnDrive += 1;
    dirsOnDrive;
    bytesInDir = 0 as libc::c_int as mt_off_t;
    filesInDir = 0 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn list_file(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut size: libc::c_ulong = 0;
    let mut i: libc::c_int = 0;
    let mut Case: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ext: [wchar_t; 4] = [0; 4];
    let mut name: [wchar_t; 9] = [0; 9];
    let mut cp: *mut doscp_t = 0 as *mut doscp_t;
    if all == 0 && (*entry).dir.attr as libc::c_int & 0x6 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if concise != 0 && isSpecialW(((*entry).name).as_mut_ptr()) != 0 {
        return 0 as libc::c_int;
    }
    r = enterDirectory((*entry).Dir);
    if r != 0 {
        return 16 as libc::c_int;
    }
    if wide != 0 {
        if filesInDir % 5 as libc::c_int != 0 {
            putchar(' ' as i32);
        } else {
            putchar('\n' as i32);
        }
    }
    if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        size = 0 as libc::c_int as libc::c_ulong;
    } else {
        size = (((*entry).dir.size[0 as libc::c_int as usize] as libc::c_int
            + (((*entry).dir.size[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int
            + (((*((*entry).dir.size)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int
                + ((*((*entry).dir.size)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                << 16 as libc::c_int)) as uint32_t as libc::c_ulong;
    }
    Case = (*entry).dir.Case as libc::c_int;
    if Case & (0x8 as libc::c_int | 0x10 as libc::c_int) == 0
        && mtools_ignore_short_case != 0
    {
        Case |= 0x8 as libc::c_int | 0x10 as libc::c_int;
    }
    cp = ((*(*(*entry).Dir).Class).get_dosConvert).unwrap()((*entry).Dir);
    dos_to_wchar(
        cp,
        ((*entry).dir.ext).as_mut_ptr(),
        ext.as_mut_ptr(),
        3 as libc::c_int as size_t,
    );
    if Case & 0x10 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            ext[i as usize] = ch_towlower(ext[i as usize]);
            i += 1;
            i;
        }
    }
    ext[3 as libc::c_int as usize] = '\0' as i32;
    if (*entry).dir.name[0 as libc::c_int as usize] as libc::c_int == '\u{5}' as i32 {
        dos_to_wchar(
            cp,
            b"\xE5\0" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            1 as libc::c_int as size_t,
        );
        dos_to_wchar(
            cp,
            ((*entry).dir.name).as_mut_ptr().offset(1 as libc::c_int as isize),
            name.as_mut_ptr().offset(1 as libc::c_int as isize),
            7 as libc::c_int as size_t,
        );
    } else {
        dos_to_wchar(
            cp,
            ((*entry).dir.name).as_mut_ptr(),
            name.as_mut_ptr(),
            8 as libc::c_int as size_t,
        );
    }
    if Case & 0x8 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            name[i as usize] = ch_towlower(name[i as usize]);
            i += 1;
            i;
        }
    }
    name[8 as libc::c_int as usize] = '\0' as i32;
    if wide != 0 {
        if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
            printf(
                b"[%s]%*s\0" as *const u8 as *const libc::c_char,
                mdir_shortname.as_mut_ptr(),
                ((15 as libc::c_int - 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_sub(strlen(mdir_shortname.as_mut_ptr())) as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            printf(
                b"%-15s\0" as *const u8 as *const libc::c_char,
                mdir_shortname.as_mut_ptr(),
            );
        }
    } else if concise == 0 {
        let mut tmpBasename: [libc::c_char; 33] = [0; 33];
        let mut tmpExt: [libc::c_char; 13] = [0; 13];
        wchar_to_native(
            name.as_mut_ptr(),
            tmpBasename.as_mut_ptr(),
            8 as libc::c_int as size_t,
            ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
        );
        wchar_to_native(
            ext.as_mut_ptr(),
            tmpExt.as_mut_ptr(),
            3 as libc::c_int as size_t,
            ::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
        );
        if name[0 as libc::c_int as usize] == ' ' as i32 {
            printf(b"             \0" as *const u8 as *const libc::c_char);
        } else if mtools_dotted_dir != 0 {
            printf(
                b"%-12s \0" as *const u8 as *const libc::c_char,
                mdir_shortname.as_mut_ptr(),
            );
        } else {
            printf(
                b"%s %s \0" as *const u8 as *const libc::c_char,
                tmpBasename.as_mut_ptr(),
                tmpExt.as_mut_ptr(),
            );
        }
        if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
            printf(b"<DIR>    \0" as *const u8 as *const libc::c_char);
        } else {
            printf(b" %8ld\0" as *const u8 as *const libc::c_char, size as libc::c_long);
        }
        printf(b" \0" as *const u8 as *const libc::c_char);
        print_date(&mut (*entry).dir);
        printf(b"  \0" as *const u8 as *const libc::c_char);
        print_time(&mut (*entry).dir);
        if debug != 0 {
            printf(
                b" %s %d \0" as *const u8 as *const libc::c_char,
                tmpBasename.as_mut_ptr(),
                ((*entry).dir.start[0 as libc::c_int as usize] as libc::c_int
                    + (((*entry).dir.start[1 as libc::c_int as usize] as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int,
            );
        }
        if *mdir_longname.as_mut_ptr() != 0 {
            printf(
                b" %s\0" as *const u8 as *const libc::c_char,
                mdir_longname.as_mut_ptr(),
            );
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        let mut tmp: [libc::c_char; 1021] = [0; 1021];
        wchar_to_native(
            ((*entry).name).as_mut_ptr(),
            tmp.as_mut_ptr(),
            255 as libc::c_int as size_t,
            ::std::mem::size_of::<[libc::c_char; 1021]>() as libc::c_ulong,
        );
        printf(
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            dirPath,
            tmp.as_mut_ptr(),
        );
        if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
            putchar('/' as i32);
        }
        putchar('\n' as i32);
    }
    filesOnDrive += 1;
    filesOnDrive;
    filesInDir += 1;
    filesInDir;
    bytesOnDrive = (bytesOnDrive as libc::c_ulong).wrapping_add(size) as mt_off_t
        as mt_off_t;
    bytesInDir = (bytesInDir as libc::c_ulong).wrapping_add(size) as mt_off_t
        as mt_off_t;
    return 4 as libc::c_int;
}
unsafe extern "C" fn list_non_recurs_directory(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*mp).basenameHasWildcard != 0 {
        return list_file(entry, mp)
    } else {
        let mut subMp: MainParam_t = MainParam_t {
            loop_0: None,
            dirCallback: None,
            callback: None,
            unixcallback: None,
            arg: 0 as *mut libc::c_void,
            openflags: 0,
            lookupflags: 0,
            fast_quit: 0,
            shortname: bounded_string {
                data: 0 as *mut libc::c_char,
                len: 0,
            },
            longname: bounded_string {
                data: 0 as *mut libc::c_char,
                len: 0,
            },
            File: 0 as *mut Stream_t,
            direntry: 0 as *mut direntry_t,
            unixSourceName: 0 as *mut libc::c_char,
            targetDir: 0 as *mut Stream_t,
            targetName: 0 as *const libc::c_char,
            originalArg: 0 as *mut libc::c_char,
            basenameHasWildcard: 0,
            mcwd: [0; 132],
            targetBuffer: [0; 1021],
        };
        r = enterDirectory((*mp).File);
        if r != 0 {
            return 16 as libc::c_int;
        }
        subMp = *mp;
        subMp.dirCallback = subMp.callback;
        return ((*mp).loop_0)
            .unwrap()((*mp).File, &mut subMp, b"*\0" as *const u8 as *const libc::c_char)
            | 4 as libc::c_int;
    };
}
unsafe extern "C" fn list_recurs_directory(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut subMp: MainParam_t = MainParam_t {
        loop_0: None,
        dirCallback: None,
        callback: None,
        unixcallback: None,
        arg: 0 as *mut libc::c_void,
        openflags: 0,
        lookupflags: 0,
        fast_quit: 0,
        shortname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        longname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        File: 0 as *mut Stream_t,
        direntry: 0 as *mut direntry_t,
        unixSourceName: 0 as *mut libc::c_char,
        targetDir: 0 as *mut Stream_t,
        targetName: 0 as *const libc::c_char,
        originalArg: 0 as *mut libc::c_char,
        basenameHasWildcard: 0,
        mcwd: [0; 132],
        targetBuffer: [0; 1021],
    };
    let mut ret: libc::c_int = 0;
    subMp = *mp;
    subMp.lookupflags = 0x10 as libc::c_int | 0x20 as libc::c_int;
    subMp
        .dirCallback = Some(
        list_file
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    subMp
        .callback = Some(
        list_file
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    ret = ((*mp).loop_0)
        .unwrap()((*mp).File, &mut subMp, b"*\0" as *const u8 as *const libc::c_char);
    subMp = *mp;
    subMp
        .lookupflags = 0x10 as libc::c_int | 0x100 as libc::c_int | 0x80 as libc::c_int
        | 1 as libc::c_int;
    return ret
        | ((*mp).loop_0)
            .unwrap()((*mp).File, &mut subMp, b"*\0" as *const u8 as *const libc::c_char)
        | 4 as libc::c_int;
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
        b"Usage: %s: [-V] [-w] [-a] [-b] [-s] [-f] msdosdirectory\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    fprintf(
        stderr,
        b"       %s: [-V] [-w] [-a] [-b] [-s] [-f] msdosfile [msdosfiles...]\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn mdir(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut ret: libc::c_int = 0;
    let mut mp: MainParam_t = MainParam_t {
        loop_0: None,
        dirCallback: None,
        callback: None,
        unixcallback: None,
        arg: 0 as *mut libc::c_void,
        openflags: 0,
        lookupflags: 0,
        fast_quit: 0,
        shortname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        longname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        File: 0 as *mut Stream_t,
        direntry: 0 as *mut direntry_t,
        unixSourceName: 0 as *mut libc::c_char,
        targetDir: 0 as *mut Stream_t,
        targetName: 0 as *const libc::c_char,
        originalArg: 0 as *mut libc::c_char,
        basenameHasWildcard: 0,
        mcwd: [0; 132],
        targetBuffer: [0; 1021],
    };
    let mut c: libc::c_int = 0;
    let mut fakedArgv: [*const libc::c_char; 1] = [
        b".\0" as *const u8 as *const libc::c_char,
    ];
    concise = 0 as libc::c_int;
    recursive = 0 as libc::c_int;
    all = 0 as libc::c_int;
    wide = all;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:waXbfds/h\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            119 => {
                wide = 1 as libc::c_int;
            }
            97 => {
                all = 1 as libc::c_int;
            }
            98 | 88 => {
                concise = 1 as libc::c_int;
            }
            115 | 47 => {
                recursive = 1 as libc::c_int;
            }
            102 => {
                fast = 1 as libc::c_int;
            }
            100 => {
                debug = 1 as libc::c_int;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if optind == argc {
        argv = fakedArgv.as_mut_ptr() as *mut *mut libc::c_char;
        argc = 1 as libc::c_int;
        optind = 0 as libc::c_int;
    }
    init_mp(&mut mp);
    currentDrive = '\0' as i32 as libc::c_char;
    currentDir = 0 as *mut Stream_t;
    RootDir = 0 as *mut Stream_t;
    dirPath = 0 as *const libc::c_char;
    if recursive != 0 {
        mp
            .lookupflags = 0x10 as libc::c_int | 0x20 as libc::c_int
            | 0x400 as libc::c_int | 0x100 as libc::c_int;
        mp
            .dirCallback = Some(
            list_recurs_directory
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
        mp
            .callback = Some(
            list_file
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
    } else {
        mp
            .lookupflags = 0x10 as libc::c_int | 0x20 as libc::c_int
            | 0x400 as libc::c_int;
        mp
            .dirCallback = Some(
            list_non_recurs_directory
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
        mp
            .callback = Some(
            list_file
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
    }
    mp.longname.data = mdir_longname.as_mut_ptr();
    mp.longname.len = ::std::mem::size_of::<[libc::c_char; 1021]>() as libc::c_ulong;
    mp.shortname.data = mdir_shortname.as_mut_ptr();
    mp.shortname.len = ::std::mem::size_of::<[libc::c_char; 49]>() as libc::c_ulong;
    ret = main_loop(&mut mp, argv.offset(optind as isize), argc - optind);
    leaveDirectory(ret);
    leaveDrive(ret);
    exit(ret);
}
