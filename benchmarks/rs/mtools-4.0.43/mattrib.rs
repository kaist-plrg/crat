use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn set_cmd_line_image(img: *mut libc::c_char);
    static mut progname: *const libc::c_char;
    static mut mversion: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut mdate: *const libc::c_char;
    fn main_loop(
        MainParam: *mut MainParam_t,
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
    ) -> libc::c_int;
    fn dir_write(entry: *mut direntry_t);
    fn isRootEntry(entry: *mut direntry_t) -> libc::c_int;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: libc::c_int);
    fn init_mp(MainParam: *mut MainParam_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_t {
    pub recursive: libc::c_int,
    pub doPrintName: libc::c_int,
    pub add: libc::c_uchar,
    pub remove: libc::c_uchar,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn attrib_file(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    if isRootEntry(entry) == 0 {
        (*entry)
            .dir
            .attr = ((*entry).dir.attr as libc::c_int & (*arg).remove as libc::c_int
            | (*arg).add as libc::c_int) as libc::c_uchar;
        dir_write(entry);
    }
    return 4 as libc::c_int;
}
unsafe extern "C" fn replay_attrib(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    if (*entry).dir.attr as libc::c_int & 0x20 as libc::c_int != 0
        && (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0
        || (*entry).dir.attr as libc::c_int & 0x20 as libc::c_int == 0
            && (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int == 0
        || (*entry).dir.attr as libc::c_int & 0x4 as libc::c_int != 0
        || (*entry).dir.attr as libc::c_int & 0x2 as libc::c_int != 0
    {
        printf(b"mattrib \0" as *const u8 as *const libc::c_char);
        if (*entry).dir.attr as libc::c_int & 0x20 as libc::c_int != 0
            && (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0
        {
            printf(b"+a \0" as *const u8 as *const libc::c_char);
        }
        if (*entry).dir.attr as libc::c_int & 0x20 as libc::c_int == 0
            && (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int == 0
        {
            printf(b"-a \0" as *const u8 as *const libc::c_char);
        }
        if (*entry).dir.attr as libc::c_int & 0x4 as libc::c_int != 0 {
            printf(b"+s \0" as *const u8 as *const libc::c_char);
        }
        if (*entry).dir.attr as libc::c_int & 0x2 as libc::c_int != 0 {
            printf(b"+h \0" as *const u8 as *const libc::c_char);
        }
        fprintPwd(stdout, entry, 1 as libc::c_int);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return 4 as libc::c_int;
}
unsafe extern "C" fn view_attrib(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    printf(b"  \0" as *const u8 as *const libc::c_char);
    if (*entry).dir.attr as libc::c_int & 0x20 as libc::c_int != 0 {
        putchar('A' as i32);
    } else {
        putchar(' ' as i32);
    }
    fputs(b"  \0" as *const u8 as *const libc::c_char, stdout);
    if (*entry).dir.attr as libc::c_int & 0x4 as libc::c_int != 0 {
        putchar('S' as i32);
    } else {
        putchar(' ' as i32);
    }
    if (*entry).dir.attr as libc::c_int & 0x2 as libc::c_int != 0 {
        putchar('H' as i32);
    } else {
        putchar(' ' as i32);
    }
    if (*entry).dir.attr as libc::c_int & 0x1 as libc::c_int != 0 {
        putchar('R' as i32);
    } else {
        putchar(' ' as i32);
    }
    printf(b"     \0" as *const u8 as *const libc::c_char);
    fprintPwd(stdout, entry, 0 as libc::c_int);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 4 as libc::c_int;
}
unsafe extern "C" fn concise_view_attrib(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    if (*entry).dir.attr as libc::c_int & 0x20 as libc::c_int != 0 {
        putchar('A' as i32);
    }
    if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        putchar('D' as i32);
    }
    if (*entry).dir.attr as libc::c_int & 0x4 as libc::c_int != 0 {
        putchar('S' as i32);
    }
    if (*entry).dir.attr as libc::c_int & 0x2 as libc::c_int != 0 {
        putchar('H' as i32);
    }
    if (*entry).dir.attr as libc::c_int & 0x1 as libc::c_int != 0 {
        putchar('R' as i32);
    }
    if (*arg).doPrintName != 0 {
        putchar(' ' as i32);
        fprintPwd(stdout, entry, 0 as libc::c_int);
    }
    putchar('\n' as i32);
    return 4 as libc::c_int;
}
unsafe extern "C" fn recursive_attrib(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    ((*mp).callback).unwrap()(entry, mp);
    return ((*mp).loop_0)
        .unwrap()((*mp).File, mp, b"*\0" as *const u8 as *const libc::c_char);
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
        b"Usage: %s [-p] [-a|+a] [-h|+h] [-r|+r] [-s|+s] msdosfile [msdosfiles...]\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn letterToCode(mut letter: libc::c_int) -> libc::c_int {
    match ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = letter;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(letter);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(letter as isize);
        }
        __res
    }) {
        65 => return 0x20 as libc::c_int,
        72 => return 0x2 as libc::c_int,
        82 => return 0x1 as libc::c_int,
        83 => return 0x4 as libc::c_int,
        _ => {
            usage(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn mattrib(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut arg: Arg_t = Arg_t {
        recursive: 0,
        doPrintName: 0,
        add: 0,
        remove: 0,
    };
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
    let mut view: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut concise: libc::c_int = 0;
    let mut replay: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wantUsage: libc::c_int = 0;
    arg.add = 0 as libc::c_int as libc::c_uchar;
    arg.remove = 0xff as libc::c_int as libc::c_uchar;
    arg.recursive = 0 as libc::c_int;
    arg.doPrintName = 1 as libc::c_int;
    view = 0 as libc::c_int;
    concise = 0 as libc::c_int;
    replay = 0 as libc::c_int;
    wantUsage = 0 as libc::c_int;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:/ahrsAHRSXp\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_16: u64;
        match c {
            104 => {
                wantUsage = 1 as libc::c_int;
                current_block_16 = 11351189237192080773;
            }
            105 => {
                set_cmd_line_image(optarg);
                current_block_16 = 12124785117276362961;
            }
            112 => {
                replay = 1 as libc::c_int;
                current_block_16 = 12124785117276362961;
            }
            47 => {
                arg.recursive = 1 as libc::c_int;
                current_block_16 = 12124785117276362961;
            }
            88 => {
                concise = 1 as libc::c_int;
                current_block_16 = 12124785117276362961;
            }
            63 => {
                usage(1 as libc::c_int);
            }
            _ => {
                current_block_16 = 11351189237192080773;
            }
        }
        match current_block_16 {
            11351189237192080773 => {
                arg
                    .remove = (arg.remove as libc::c_int & !letterToCode(c))
                    as libc::c_uchar;
            }
            _ => {}
        }
    }
    if optind == argc && wantUsage != 0 {
        usage(0 as libc::c_int);
    }
    while optind < argc {
        match *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int
        {
            43 => {
                ptr = (*argv.offset(optind as isize)).offset(1 as libc::c_int as isize);
                while *ptr != 0 {
                    arg
                        .add = (arg.add as libc::c_int
                        | letterToCode(*ptr as libc::c_int)) as libc::c_uchar;
                    ptr = ptr.offset(1);
                    ptr;
                }
            }
            45 => {
                ptr = (*argv.offset(optind as isize)).offset(1 as libc::c_int as isize);
                while *ptr != 0 {
                    arg
                        .remove = (arg.remove as libc::c_int
                        & !letterToCode(*ptr as libc::c_int)) as libc::c_uchar;
                    ptr = ptr.offset(1);
                    ptr;
                }
            }
            _ => {
                break;
            }
        }
        optind += 1;
        optind;
    }
    if arg.remove as libc::c_int == 0xff as libc::c_int && arg.add == 0 {
        view = 1 as libc::c_int;
    }
    if optind >= argc {
        usage(1 as libc::c_int);
    }
    init_mp(&mut mp);
    if view != 0 {
        if concise != 0 {
            mp
                .callback = Some(
                concise_view_attrib
                    as unsafe extern "C" fn(
                        *mut direntry_t,
                        *mut MainParam_t,
                    ) -> libc::c_int,
            );
            arg
                .doPrintName = (argc - optind > 1 as libc::c_int || arg.recursive != 0
                || !(strpbrk(
                    *argv.offset(optind as isize),
                    b"*[?\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()) as libc::c_int;
        } else if replay != 0 {
            mp
                .callback = Some(
                replay_attrib
                    as unsafe extern "C" fn(
                        *mut direntry_t,
                        *mut MainParam_t,
                    ) -> libc::c_int,
            );
        } else {
            mp
                .callback = Some(
                view_attrib
                    as unsafe extern "C" fn(
                        *mut direntry_t,
                        *mut MainParam_t,
                    ) -> libc::c_int,
            );
        }
        mp.openflags = 0 as libc::c_int;
    } else {
        mp
            .callback = Some(
            attrib_file
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
        mp.openflags = 0o2 as libc::c_int;
    }
    if arg.recursive != 0 {
        mp
            .dirCallback = Some(
            recursive_attrib
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
    }
    mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    mp.lookupflags = 0x20 as libc::c_int | 0x10 as libc::c_int;
    if arg.recursive != 0 {
        mp.lookupflags |= 0x400 as libc::c_int | 0x100 as libc::c_int;
    }
    exit(main_loop(&mut mp, argv.offset(optind as isize), argc - optind));
}
