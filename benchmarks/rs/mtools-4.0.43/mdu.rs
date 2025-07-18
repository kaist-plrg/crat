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
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn getStart(Dir: *mut Stream_t, dir: *mut directory) -> libc::c_uint;
    fn countBlocks(Dir: *mut Stream_t, block: libc::c_uint) -> libc::c_uint;
    fn isRootDir(Stream: *mut Stream_t) -> libc::c_int;
    static mut progname: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    static mut mversion: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: libc::c_int);
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(
        MainParam: *mut MainParam_t,
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
    ) -> libc::c_int;
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
    pub all: libc::c_int,
    pub inDir: libc::c_int,
    pub summary: libc::c_int,
    pub parent: *mut Arg_t,
    pub target: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub blocks: libc::c_uint,
    pub mp: MainParam_t,
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
        b"Usage: %s: msdosdirectory\n\0" as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn file_mdu(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut blocks: libc::c_uint = 0;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    blocks = countBlocks((*entry).Dir, getStart((*entry).Dir, &mut (*entry).dir));
    if (*arg).all != 0 || (*arg).inDir == 0 {
        fprintPwd(stdout, entry, 0 as libc::c_int);
        printf(b" %d\n\0" as *const u8 as *const libc::c_char, blocks);
    }
    (*arg).blocks = ((*arg).blocks).wrapping_add(blocks);
    return 4 as libc::c_int;
}
unsafe extern "C" fn dir_mdu(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut parentArg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut arg: Arg_t = Arg_t {
        all: 0,
        inDir: 0,
        summary: 0,
        parent: 0 as *mut Arg_t,
        target: 0 as *mut libc::c_char,
        path: 0 as *mut libc::c_char,
        blocks: 0,
        mp: MainParam_t {
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
        },
    };
    let mut ret: libc::c_int = 0;
    arg = *parentArg;
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.parent = parentArg;
    arg.inDir = 1 as libc::c_int;
    if isRootDir((*entry).Dir) == 0 {
        arg
            .blocks = countBlocks(
            (*entry).Dir,
            getStart((*entry).Dir, &mut (*entry).dir),
        );
    } else {
        arg.blocks = 0 as libc::c_int as libc::c_uint;
    }
    ret = ((*mp).loop_0)
        .unwrap()((*mp).File, &mut arg.mp, b"*\0" as *const u8 as *const libc::c_char);
    if arg.summary == 0 || (*parentArg).inDir == 0 {
        fprintPwd(stdout, entry, 0 as libc::c_int);
        printf(b" %d\n\0" as *const u8 as *const libc::c_char, arg.blocks);
    }
    (*arg.parent).blocks = ((*arg.parent).blocks).wrapping_add(arg.blocks);
    return ret;
}
pub unsafe extern "C" fn mdu(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut arg: Arg_t = Arg_t {
        all: 0,
        inDir: 0,
        summary: 0,
        parent: 0 as *mut Arg_t,
        target: 0 as *mut libc::c_char,
        path: 0 as *mut libc::c_char,
        blocks: 0,
        mp: MainParam_t {
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
        },
    };
    let mut c: libc::c_int = 0;
    arg.all = 0 as libc::c_int;
    arg.inDir = 0 as libc::c_int;
    arg.summary = 0 as libc::c_int;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:ash\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            97 => {
                arg.all = 1 as libc::c_int;
            }
            115 => {
                arg.summary = 1 as libc::c_int;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            63 => {
                usage(1 as libc::c_int);
            }
            _ => {}
        }
    }
    if optind >= argc {
        usage(1 as libc::c_int);
    }
    if arg.summary != 0 && arg.all != 0 {
        fprintf(
            stderr,
            b"-a and -s options are mutually exclusive\n\0" as *const u8
                as *const libc::c_char,
        );
        usage(1 as libc::c_int);
    }
    init_mp(&mut arg.mp);
    arg
        .mp
        .callback = Some(
        file_mdu
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    arg.mp.openflags = 0 as libc::c_int;
    arg
        .mp
        .dirCallback = Some(
        dir_mdu as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg
        .mp
        .lookupflags = 0x20 as libc::c_int | 0x10 as libc::c_int | 0x400 as libc::c_int
        | 0x100 as libc::c_int;
    exit(main_loop(&mut arg.mp, argv.offset(optind as isize), argc - optind));
}
