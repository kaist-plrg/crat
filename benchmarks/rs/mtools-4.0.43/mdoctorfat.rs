use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dos_name_t;
    pub type doscp_t;
    pub type FatMap_t;
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
    fn __errno_location() -> *mut libc::c_int;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn check_number_parse_errno(
        c: libc::c_char,
        optarg_0: *const libc::c_char,
        endptr: *mut libc::c_char,
    );
    fn strtoui(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_uint;
    fn strtou32(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint32_t;
    static mut progname: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut mversion: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(
        MainParam: *mut MainParam_t,
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
    ) -> libc::c_int;
    fn dir_write(entry: *mut direntry_t);
    fn isRootEntry(entry: *mut direntry_t) -> libc::c_int;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn getFs(Stream: *mut Stream_t) -> *mut Fs_t;
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
pub struct Fs_t {
    pub head: Stream_t,
    pub serialized: libc::c_int,
    pub serial_number: libc::c_ulong,
    pub cluster_size: uint8_t,
    pub sector_size: uint16_t,
    pub fat_error: libc::c_int,
    pub fat_decode: Option::<
        unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
    >,
    pub fat_encode: Option::<
        unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
    >,
    pub fat_dirty: libc::c_int,
    pub fat_start: uint16_t,
    pub fat_len: uint32_t,
    pub num_fat: uint8_t,
    pub end_fat: uint32_t,
    pub last_fat: uint32_t,
    pub fat_bits: libc::c_uint,
    pub FatMap: *mut FatMap_t,
    pub dir_start: uint32_t,
    pub dir_len: uint16_t,
    pub clus_start: uint32_t,
    pub num_clus: uint32_t,
    pub drive: libc::c_char,
    pub primaryFat: uint32_t,
    pub writeAllFats: uint32_t,
    pub rootCluster: uint32_t,
    pub infoSectorLoc: uint32_t,
    pub backupBoot: uint16_t,
    pub last: uint32_t,
    pub freeSpace: uint32_t,
    pub preallocatedClusters: libc::c_uint,
    pub lastFatSectorNr: uint32_t,
    pub lastFatSectorData: *mut libc::c_uchar,
    pub lastFatAccessMode: fatAccessMode_t,
    pub sectorMask: libc::c_uint,
    pub sectorShift: libc::c_uint,
    pub cp: *mut doscp_t,
}
pub type fatAccessMode_t = libc::c_uint;
pub const FAT_ACCESS_WRITE: fatAccessMode_t = 1;
pub const FAT_ACCESS_READ: fatAccessMode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_t {
    pub target: *mut libc::c_char,
    pub mp: MainParam_t,
    pub ch: ClashHandling_t,
    pub sourcefile: *mut Stream_t,
    pub fat: uint32_t,
    pub markbad: libc::c_int,
    pub setsize: libc::c_int,
    pub size: uint32_t,
    pub Fs: *mut Fs_t,
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
unsafe extern "C" fn dos_doctorfat(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut Fs: *mut Fs_t = getFs((*mp).File);
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    if (*arg).markbad == 0 && isRootEntry(entry) != 0 {
        set_word(
            ((*entry).dir.start).as_mut_ptr(),
            ((*arg).fat & 0xffff as libc::c_int as libc::c_uint) as libc::c_ushort,
        );
        set_word(
            ((*entry).dir.startHi).as_mut_ptr(),
            ((*arg).fat >> 16 as libc::c_int) as libc::c_ushort,
        );
        if (*arg).setsize != 0 {
            set_dword(((*entry).dir.size).as_mut_ptr(), (*arg).size);
        }
        dir_write(entry);
    }
    (*arg).Fs = Fs;
    return 4 as libc::c_int;
}
unsafe extern "C" fn unix_doctorfat(mut mp: *mut MainParam_t) -> libc::c_int {
    fprintf(
        stderr,
        b"File does not reside on a Dos fs\n\0" as *const u8 as *const libc::c_char,
    );
    return 16 as libc::c_int;
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
        b"Usage: [-b] %s file fat\n\0" as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn mdoctorfat(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut mtype: libc::c_int,
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
        sourcefile: 0 as *mut Stream_t,
        fat: 0,
        markbad: 0,
        setsize: 0,
        size: 0,
        Fs: 0 as *mut Fs_t,
    };
    let mut c: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut address: libc::c_uint = 0;
    let mut begin: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut number: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut offset: libc::c_uint = 0;
    init_clash_handling(&mut arg.ch);
    offset = 0 as libc::c_int as libc::c_uint;
    arg.markbad = 0 as libc::c_int;
    arg.setsize = 0 as libc::c_int;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:bo:s:h\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            98 => {
                arg.markbad = 1 as libc::c_int;
            }
            111 => {
                offset = strtoui(optarg, &mut endptr, 0 as libc::c_int);
            }
            115 => {
                arg.setsize = 1 as libc::c_int;
                arg.size = strtou32(optarg, &mut endptr, 0 as libc::c_int);
            }
            104 => {
                usage(0 as libc::c_int);
            }
            63 => {
                usage(1 as libc::c_int);
            }
            _ => {}
        }
        check_number_parse_errno(c as libc::c_char, optarg, endptr);
    }
    if argc - optind < 2 as libc::c_int {
        usage(1 as libc::c_int);
    }
    init_mp(&mut arg.mp);
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg
        .mp
        .callback = Some(
        dos_doctorfat
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    arg
        .mp
        .unixcallback = Some(
        unix_doctorfat as unsafe extern "C" fn(*mut MainParam_t) -> libc::c_int,
    );
    arg.mp.lookupflags = 0x20 as libc::c_int | 0x10 as libc::c_int | 1 as libc::c_int;
    arg.mp.openflags = 0o2 as libc::c_int;
    arg
        .fat = (strtoui(
        *argv.offset((optind + 1 as libc::c_int) as isize),
        0 as *mut *mut libc::c_char,
        0 as libc::c_int,
    ))
        .wrapping_add(offset);
    ret = main_loop(&mut arg.mp, argv.offset(optind as isize), 1 as libc::c_int);
    if ret != 0 {
        exit(ret);
    }
    address = 0 as libc::c_int as libc::c_uint;
    i = optind + 1 as libc::c_int;
    while i < argc {
        let mut j: libc::c_uint = 0;
        number = *argv.offset(i as isize);
        if *number as libc::c_int == '<' as i32 {
            number = number.offset(1);
            number;
        }
        begin = strtoui(number, &mut eptr, 0 as libc::c_int);
        if !eptr.is_null() && *eptr as libc::c_int == '-' as i32 {
            number = eptr.offset(1 as libc::c_int as isize);
            end = strtoui(number, &mut eptr, 0 as libc::c_int);
        } else {
            end = begin;
        }
        if eptr == number {
            fprintf(
                stderr,
                b"Not a number: %s\n\0" as *const u8 as *const libc::c_char,
                number,
            );
            exit(-(1 as libc::c_int));
        }
        if !eptr.is_null() && *eptr as libc::c_int == '>' as i32 {
            eptr = eptr.offset(1);
            eptr;
        }
        if !eptr.is_null() && *eptr as libc::c_int != 0 {
            fprintf(
                stderr,
                b"Not a number: %s\n\0" as *const u8 as *const libc::c_char,
                eptr,
            );
            exit(-(1 as libc::c_int));
        }
        j = begin;
        while j <= end {
            if arg.markbad != 0 {
                ((*arg.Fs).fat_encode)
                    .unwrap()(
                    arg.Fs,
                    j.wrapping_add(offset),
                    (*arg.Fs).last_fat ^ 6 as libc::c_int as libc::c_uint
                        ^ 8 as libc::c_int as libc::c_uint,
                );
            } else {
                if address != 0 {
                    ((*arg.Fs).fat_encode)
                        .unwrap()(arg.Fs, address, j.wrapping_add(offset));
                }
                address = j.wrapping_add(offset);
            }
            j = j.wrapping_add(1);
            j;
        }
        i += 1;
        i;
    }
    if address != 0 && arg.markbad == 0 {
        ((*arg.Fs).fat_encode).unwrap()(arg.Fs, address, (*arg.Fs).end_fat);
    }
    exit(ret);
}
