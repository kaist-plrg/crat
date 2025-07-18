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
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn _basename(filename: *const libc::c_char) -> *const libc::c_char;
    fn set_cmd_line_image(img: *mut libc::c_char);
    static mut progname: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    static mut mversion: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
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
    fn isRootEntry(entry: *mut direntry_t) -> libc::c_int;
    fn dosnameToDirentry(n: *const dos_name_t, dir: *mut directory);
    fn mpPickTargetName(mp: *mut MainParam_t) -> *const libc::c_char;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: libc::c_int);
    fn isSubdirOf(inside: *mut Stream_t, outside: *mut Stream_t) -> libc::c_int;
    fn dos_target_lookup(mp: *mut MainParam_t, arg: *const libc::c_char) -> libc::c_int;
    fn init_mp(MainParam: *mut MainParam_t);
    fn dir_write(entry: *mut direntry_t);
    fn wipeEntry(entry: *mut direntry_t);
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
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
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
    pub fromname: *const libc::c_char,
    pub verbose: libc::c_int,
    pub mp: MainParam_t,
    pub entry: *mut direntry_t,
    pub ch: ClashHandling_t,
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
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut libc::c_char,
    *mut libc::c_void,
    *mut direntry_t,
) -> libc::c_int;
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
unsafe extern "C" fn renameit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut arg0: *mut libc::c_void,
    mut targetEntry: *mut direntry_t,
) -> libc::c_int {
    let mut arg: *mut Arg_t = arg0 as *mut Arg_t;
    let mut fat: uint32_t = 0;
    (*targetEntry).dir = (*(*arg).entry).dir;
    dosnameToDirentry(dosname, &mut (*targetEntry).dir);
    if (*targetEntry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        let mut movedEntry: *mut direntry_t = 0 as *mut direntry_t;
        movedEntry = getDirentry((*arg).mp.File);
        if (*movedEntry).Dir != (*targetEntry).Dir {
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
            let mut oldDir: *mut Stream_t = 0 as *mut Stream_t;
            initializeDirentry(&mut subEntry, (*arg).mp.File);
            match vfat_lookup(
                &mut subEntry,
                b"..\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as size_t,
                0x10 as libc::c_int,
                0 as *mut libc::c_char,
                0 as libc::c_int as size_t,
                0 as *mut libc::c_char,
                0 as libc::c_int as size_t,
            ) {
                -1 => {
                    fprintf(
                        stderr,
                        b" Directory has no parent entry\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                -2 => return 16 as libc::c_int,
                0 => {
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
                    subEntry
                        .dir
                        .start[1 as libc::c_int
                        as usize] = (fat >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
                    subEntry
                        .dir
                        .start[0 as libc::c_int
                        as usize] = (fat & 0xff as libc::c_int as libc::c_uint)
                        as libc::c_uchar;
                    dir_write(&mut subEntry);
                    if (*arg).verbose != 0 {
                        fprintf(
                            stderr,
                            b"Easy, isn't it? I wonder why DOS can't do this.\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                }
                _ => {}
            }
            wipeEntry(movedEntry);
            oldDir = (*movedEntry).Dir;
            *movedEntry = *targetEntry;
            copy_stream((*targetEntry).Dir);
            free_stream(&mut oldDir);
            return 0 as libc::c_int;
        }
    }
    wipeEntry((*arg).mp.direntry);
    return 0 as libc::c_int;
}
unsafe extern "C" fn rename_file(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut targetDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut shortname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut longname: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    (*arg).entry = entry;
    targetDir = (*mp).targetDir;
    if targetDir == (*entry).Dir {
        (*arg).ch.ignore_entry = -(1 as libc::c_int);
        (*arg).ch.source = (*entry).entry;
        (*arg).ch.source_entry = (*entry).entry;
    } else {
        (*arg).ch.ignore_entry = -(1 as libc::c_int);
        (*arg).ch.source = -(2 as libc::c_int);
    }
    longname = mpPickTargetName(mp);
    shortname = 0 as *mut libc::c_char;
    result = mwrite_one(
        targetDir,
        longname,
        shortname,
        Some(
            renameit
                as unsafe extern "C" fn(
                    *mut dos_name_t,
                    *mut libc::c_char,
                    *mut libc::c_void,
                    *mut direntry_t,
                ) -> libc::c_int,
        ),
        arg as *mut libc::c_void,
        &mut (*arg).ch,
    );
    if result == 1 as libc::c_int {
        return 4 as libc::c_int
    } else {
        return 16 as libc::c_int
    };
}
unsafe extern "C" fn rename_directory(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if isSubdirOf((*mp).targetDir, (*mp).File) != 0 {
        fprintf(stderr, b"Cannot move directory \0" as *const u8 as *const libc::c_char);
        fprintPwd(stderr, entry, 0 as libc::c_int);
        fprintf(
            stderr,
            b" into one of its own subdirectories (\0" as *const u8
                as *const libc::c_char,
        );
        fprintPwd(stderr, getDirentry((*mp).targetDir), 0 as libc::c_int);
        fprintf(stderr, b")\n\0" as *const u8 as *const libc::c_char);
        return 16 as libc::c_int;
    }
    if isRootEntry(entry) != 0 {
        fprintf(
            stderr,
            b"Cannot move a root directory: \0" as *const u8 as *const libc::c_char,
        );
        fprintPwd(stderr, entry, 0 as libc::c_int);
        return 16 as libc::c_int;
    }
    ret = rename_file(entry, mp);
    if ret & 16 as libc::c_int != 0 {
        return ret;
    }
    return ret;
}
unsafe extern "C" fn rename_oldsyntax(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut targetDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut shortname: *const libc::c_char = 0 as *const libc::c_char;
    let mut longname: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    (*arg).entry = entry;
    targetDir = (*entry).Dir;
    (*arg).ch.ignore_entry = -(1 as libc::c_int);
    (*arg).ch.source = (*entry).entry;
    (*arg).ch.source_entry = (*entry).entry;
    longname = (*mp).targetName;
    shortname = 0 as *const libc::c_char;
    result = mwrite_one(
        targetDir,
        longname,
        shortname,
        Some(
            renameit
                as unsafe extern "C" fn(
                    *mut dos_name_t,
                    *mut libc::c_char,
                    *mut libc::c_void,
                    *mut direntry_t,
                ) -> libc::c_int,
        ),
        arg as *mut libc::c_void,
        &mut (*arg).ch,
    );
    if result == 1 as libc::c_int {
        return 4 as libc::c_int
    } else {
        return 16 as libc::c_int
    };
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
        b"Usage: %s [-vV] [-D clash_option] file targetfile\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    fprintf(
        stderr,
        b"       %s [-vV] [-D clash_option] file [files...] target_directory\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn mmove(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut oldsyntax: libc::c_int,
) {
    let mut arg: Arg_t = Arg_t {
        fromname: 0 as *const libc::c_char,
        verbose: 0,
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
        entry: 0 as *mut direntry_t,
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
    };
    let mut c: libc::c_int = 0;
    let mut shortname: [libc::c_char; 49] = [0; 49];
    let mut longname: [libc::c_char; 1021] = [0; 1021];
    let mut def_drive: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    init_clash_handling(&mut arg.ch);
    arg.verbose = 0 as libc::c_int;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:vD:oh\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            118 => {
                arg.verbose = 1 as libc::c_int;
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
            63 => {
                usage(1 as libc::c_int);
            }
            _ => {}
        }
    }
    if argc - optind < 2 as libc::c_int {
        usage(1 as libc::c_int);
    }
    init_mp(&mut arg.mp);
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.openflags = 0o2 as libc::c_int;
    def_drive = '\0' as i32 as libc::c_char;
    i = optind;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            != 0
            && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == ':' as i32
        {
            if def_drive == 0 {
                def_drive = ch_toupper(
                    *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize),
                );
            } else if def_drive as libc::c_int
                != ch_toupper(
                    *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize),
                ) as libc::c_int
            {
                fprintf(
                    stderr,
                    b"Cannot move files across different drives\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        i += 1;
        i;
    }
    if def_drive != 0 {
        *(arg.mp.mcwd).as_mut_ptr() = def_drive;
    }
    if oldsyntax != 0
        && (argc - optind != 2 as libc::c_int
            || !(strpbrk(
                b":/\0" as *const u8 as *const libc::c_char,
                *argv.offset((argc - 1 as libc::c_int) as isize),
            ))
                .is_null())
    {
        oldsyntax = 0 as libc::c_int;
    }
    arg
        .mp
        .lookupflags = 0x20 as libc::c_int | 0x10 as libc::c_int | 0x400 as libc::c_int
        | 0x100 as libc::c_int;
    if oldsyntax == 0 {
        dos_target_lookup(&mut arg.mp, *argv.offset((argc - 1 as libc::c_int) as isize));
        arg
            .mp
            .callback = Some(
            rename_file
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
        arg
            .mp
            .dirCallback = Some(
            rename_directory
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
    } else {
        arg.fromname = *argv.offset(optind as isize);
        if *(arg.fromname).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && *(arg.fromname).offset(1 as libc::c_int as isize) as libc::c_int
                == ':' as i32
        {
            arg.fromname = (arg.fromname).offset(2 as libc::c_int as isize);
        }
        arg.fromname = _basename(arg.fromname);
        arg.mp.targetName = strdup(*argv.offset((argc - 1 as libc::c_int) as isize));
        arg
            .mp
            .callback = Some(
            rename_oldsyntax
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
    }
    arg.mp.longname.data = longname.as_mut_ptr();
    arg.mp.longname.len = ::std::mem::size_of::<[libc::c_char; 1021]>() as libc::c_ulong;
    longname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    arg.mp.shortname.data = shortname.as_mut_ptr();
    arg.mp.shortname.len = ::std::mem::size_of::<[libc::c_char; 49]>() as libc::c_ulong;
    shortname[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    exit(
        main_loop(
            &mut arg.mp,
            argv.offset(optind as isize),
            argc - optind - 1 as libc::c_int,
        ),
    );
}
