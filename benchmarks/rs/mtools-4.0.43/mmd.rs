use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dos_name_t;
    pub type doscp_t;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn mk_entry(
        filename: *const dos_name_t,
        attr: libc::c_uchar,
        fat: libc::c_uint,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn mk_entry_from_base(
        base: *const libc::c_char,
        attr: libc::c_uchar,
        fat: libc::c_uint,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn getfreeMinClusters(Stream: *mut Stream_t, ref_0: uint32_t) -> libc::c_int;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn flush_stream(Stream: *mut Stream_t) -> libc::c_int;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn getTimeNow(now: *mut time_t) -> time_t;
    static mut progname: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    static mut mversion: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    fn init_mp(MainParam: *mut MainParam_t);
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn dir_write(entry: *mut direntry_t);
    fn main_loop(
        MainParam: *mut MainParam_t,
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
    ) -> libc::c_int;
    fn mwrite_one(
        Dir: *mut Stream_t,
        argname: *const libc::c_char,
        shortname: *const libc::c_char,
        cb: Option::<write_data_callback>,
        arg: *mut libc::c_void,
        ch: *mut ClashHandling_t,
    ) -> libc::c_int;
    fn handle_clash_options(ch: *mut ClashHandling_t, c: libc::c_int) -> libc::c_int;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn OpenFileByDirentry(entry: *mut direntry_t) -> *mut Stream_t;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
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
    pub target: *mut libc::c_char,
    pub mp: MainParam_t,
    pub SrcDir: *mut Stream_t,
    pub entry: libc::c_int,
    pub ch: ClashHandling_t,
    pub targetDir: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClashHandling_t {
    pub action: [clash_action; 2],
    pub namematch_default: [clash_action; 2],
    pub nowarn: libc::c_int,
    pub got_slots: libc::c_int,
    pub mod_time: libc::c_int,
    pub myname: *mut libc::c_char,
    pub dosname: *mut libc::c_uchar,
    pub single: libc::c_int,
    pub use_longname: libc::c_int,
    pub ignore_entry: libc::c_int,
    pub source: libc::c_int,
    pub source_entry: libc::c_int,
    pub name_converter: Option::<
        unsafe extern "C" fn(
            *mut doscp_t,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_int,
            *mut dos_name_t,
        ) -> (),
    >,
    pub is_label: libc::c_int,
}
pub type clash_action = libc::c_uint;
pub const NAMEMATCH_GREW: clash_action = 9;
pub const NAMEMATCH_SUCCESS: clash_action = 8;
pub const NAMEMATCH_ERROR: clash_action = 7;
pub const NAMEMATCH_OVERWRITE: clash_action = 6;
pub const NAMEMATCH_PRENAME: clash_action = 5;
pub const NAMEMATCH_RENAME: clash_action = 4;
pub const NAMEMATCH_SKIP: clash_action = 3;
pub const NAMEMATCH_QUIT: clash_action = 2;
pub const NAMEMATCH_AUTORENAME: clash_action = 1;
pub const NAMEMATCH_NONE: clash_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateArg_t {
    pub Dir: *mut Stream_t,
    pub NewDir: *mut Stream_t,
    pub attr: libc::c_uchar,
    pub mtime: time_t,
}
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut libc::c_char,
    *mut libc::c_void,
    *mut direntry_t,
) -> libc::c_int;
unsafe extern "C" fn makeit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut arg0: *mut libc::c_void,
    mut targetEntry: *mut direntry_t,
) -> libc::c_int {
    let mut Target: *mut Stream_t = 0 as *mut Stream_t;
    let mut arg: *mut CreateArg_t = arg0 as *mut CreateArg_t;
    let mut fat: uint32_t = 0;
    let mut subEntry: direntry_t = direntry_t {
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
    if getfreeMinClusters((*targetEntry).Dir, 1 as libc::c_int as uint32_t) == 0 {
        return -(1 as libc::c_int);
    }
    mk_entry(
        dosname,
        0x10 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int as uint32_t,
        (*arg).mtime,
        &mut (*targetEntry).dir,
    );
    Target = OpenFileByDirentry(targetEntry);
    if Target.is_null() {
        fprintf(
            stderr,
            b"Could not open Target\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    initializeDirentry(&mut subEntry, Target);
    subEntry.entry = 1 as libc::c_int;
    ((*(*(*targetEntry).Dir).Class).get_data)
        .unwrap()(
        (*targetEntry).Dir,
        0 as *mut time_t,
        0 as *mut mt_off_t,
        0 as *mut libc::c_int,
        &mut fat,
    );
    if fat == fat32RootCluster((*targetEntry).Dir) {
        fat = 0 as libc::c_int as uint32_t;
    }
    mk_entry_from_base(
        b"..      \0" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int as libc::c_uchar,
        fat,
        0 as libc::c_int as uint32_t,
        (*arg).mtime,
        &mut subEntry.dir,
    );
    dir_write(&mut subEntry);
    flush_stream(Target);
    subEntry.entry = 0 as libc::c_int;
    ((*(*Target).Class).get_data)
        .unwrap()(
        Target,
        0 as *mut time_t,
        0 as *mut mt_off_t,
        0 as *mut libc::c_int,
        &mut fat,
    );
    mk_entry_from_base(
        b".       \0" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int as libc::c_uchar,
        fat,
        0 as libc::c_int as uint32_t,
        (*arg).mtime,
        &mut subEntry.dir,
    );
    dir_write(&mut subEntry);
    mk_entry(
        dosname,
        (0x10 as libc::c_int | (*arg).attr as libc::c_int) as libc::c_uchar,
        fat,
        0 as libc::c_int as uint32_t,
        (*arg).mtime,
        &mut (*targetEntry).dir,
    );
    (*arg).NewDir = Target;
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
        b"Usage: %s [-D clash_option] file targetfile\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    fprintf(
        stderr,
        b"       %s [-D clash_option] file [files...] target_directory\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn createDir(
    mut Dir: *mut Stream_t,
    mut filename: *const libc::c_char,
    mut ch: *mut ClashHandling_t,
    mut attr: libc::c_uchar,
    mut mtime: time_t,
) -> *mut Stream_t {
    let mut arg: CreateArg_t = CreateArg_t {
        Dir: 0 as *mut Stream_t,
        NewDir: 0 as *mut Stream_t,
        attr: 0,
        mtime: 0,
    };
    let mut ret: libc::c_int = 0;
    arg.Dir = Dir;
    arg.attr = attr;
    arg.mtime = mtime;
    if getfreeMinClusters(Dir, 1 as libc::c_int as uint32_t) == 0 {
        return 0 as *mut Stream_t;
    }
    ret = mwrite_one(
        Dir,
        filename,
        0 as *const libc::c_char,
        Some(
            makeit
                as unsafe extern "C" fn(
                    *mut dos_name_t,
                    *mut libc::c_char,
                    *mut libc::c_void,
                    *mut direntry_t,
                ) -> libc::c_int,
        ),
        &mut arg as *mut CreateArg_t as *mut libc::c_void,
        ch,
    );
    if ret < 1 as libc::c_int { return 0 as *mut Stream_t } else { return arg.NewDir };
}
unsafe extern "C" fn createDirCallback(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut ret: *mut Stream_t = 0 as *mut Stream_t;
    let mut now: time_t = 0;
    ret = createDir(
        (*mp).File,
        (*mp).targetName,
        &mut (*((*mp).arg as *mut Arg_t)).ch,
        0x10 as libc::c_int as libc::c_uchar,
        getTimeNow(&mut now),
    );
    if ret.is_null() {
        return 16 as libc::c_int
    } else {
        free_stream(&mut ret);
        return 4 as libc::c_int;
    };
}
pub unsafe extern "C" fn mmd(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut arg: Arg_t = Arg_t {
        target: 0 as *mut libc::c_char,
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
        SrcDir: 0 as *mut Stream_t,
        entry: 0,
        ch: ClashHandling_t {
            action: [NAMEMATCH_NONE; 2],
            namematch_default: [NAMEMATCH_NONE; 2],
            nowarn: 0,
            got_slots: 0,
            mod_time: 0,
            myname: 0 as *mut libc::c_char,
            dosname: 0 as *mut libc::c_uchar,
            single: 0,
            use_longname: 0,
            ignore_entry: 0,
            source: 0,
            source_entry: 0,
            name_converter: None,
            is_label: 0,
        },
        targetDir: 0 as *mut Stream_t,
    };
    let mut c: libc::c_int = 0;
    init_clash_handling(&mut arg.ch);
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:D:oh\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            63 => {
                usage(1 as libc::c_int);
            }
            111 => {
                handle_clash_options(&mut arg.ch, c);
            }
            68 => {
                if handle_clash_options(&mut arg.ch, *optarg as libc::c_int) != 0 {
                    usage(1 as libc::c_int);
                }
            }
            104 => {
                usage(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if argc - optind < 1 as libc::c_int {
        usage(1 as libc::c_int);
    }
    init_mp(&mut arg.mp);
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.openflags = 0o2 as libc::c_int;
    arg
        .mp
        .callback = Some(
        createDirCallback
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    arg.mp.lookupflags = 0x1000 as libc::c_int | 0x400 as libc::c_int;
    exit(main_loop(&mut arg.mp, argv.offset(optind as isize), argc - optind));
}
