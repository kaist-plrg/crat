use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dos_name_t;
    pub type doscp_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn copyfile(Source: *mut Stream_t, Target: *mut Stream_t) -> mt_off_t;
    fn getfreeMinBytes(Stream: *mut Stream_t, ref_0: mt_off_t) -> libc::c_int;
    fn mk_entry(
        filename: *const dos_name_t,
        attr: libc::c_uchar,
        fat: libc::c_uint,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn open_dos2unix(Next: *mut Stream_t, convertCharset: libc::c_int) -> *mut Stream_t;
    fn open_unix2dos(Next: *mut Stream_t, convertCharset: libc::c_int) -> *mut Stream_t;
    static mut got_signal: libc::c_int;
    fn fileTooBig(off: mt_off_t) -> libc::c_int;
    static mut batchmode: libc::c_int;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn getTimeNow(now: *mut time_t) -> time_t;
    static mut progname: *const libc::c_char;
    fn printOom();
    fn ask_confirmation(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut mversion: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(
        MainParam: *mut MainParam_t,
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
    ) -> libc::c_int;
    fn dos_target_lookup(mp: *mut MainParam_t, arg: *const libc::c_char) -> libc::c_int;
    fn mpGetBasename(mp: *mut MainParam_t) -> *const libc::c_char;
    fn mpPrintFilename(file: *mut FILE, mp: *mut MainParam_t);
    fn mpPickTargetName(mp: *mut MainParam_t) -> *const libc::c_char;
    fn isSubdirOf(inside: *mut Stream_t, outside: *mut Stream_t) -> libc::c_int;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: libc::c_int);
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn vfat_lookup_zt(
        entry: *mut direntry_t,
        filename: *const libc::c_char,
        flags: libc::c_int,
        shortname: *mut libc::c_char,
        shortname_len: size_t,
        longname: *mut libc::c_char,
        longname_len: size_t,
    ) -> libc::c_int;
    fn SimpleFileOpen(
        dev: *mut device,
        orig_dev: *mut device,
        name: *const libc::c_char,
        mode: libc::c_int,
        errmsg: *mut libc::c_char,
        mode2: libc::c_int,
        locked: libc::c_int,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn get_fd(Stream: *mut Stream_t) -> libc::c_int;
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
    fn createDir(
        Dir: *mut Stream_t,
        filename: *const libc::c_char,
        ch: *mut ClashHandling_t,
        attr: libc::c_uchar,
        mtime: time_t,
    ) -> *mut Stream_t;
    fn OpenFileByDirentry(entry: *mut direntry_t) -> *mut Stream_t;
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn fat_free(Dir: *mut Stream_t, fat: libc::c_uint) -> libc::c_int;
    fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
    pub preserveAttributes: libc::c_int,
    pub preserveTime: libc::c_int,
    pub attr: libc::c_uchar,
    pub path: *mut libc::c_char,
    pub textmode: libc::c_int,
    pub needfilter: libc::c_int,
    pub nowarn: libc::c_int,
    pub verbose: libc::c_int,
    pub type_0: libc::c_int,
    pub convertCharset: libc::c_int,
    pub mp: MainParam_t,
    pub ch: ClashHandling_t,
    pub noClobber: libc::c_int,
    pub unixTarget: *const libc::c_char,
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
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn ptrdiff(
    mut end: *const libc::c_char,
    mut begin: *const libc::c_char,
) -> size_t {
    return end.offset_from(begin) as libc::c_long as size_t;
}
unsafe extern "C" fn set_mtime(mut target: *const libc::c_char, mut mtime: time_t) {
    if !target.is_null()
        && strcmp(target, b"-\0" as *const u8 as *const libc::c_char) != 0
        && mtime != 0 as libc::c_long
    {
        let mut tv: [timeval; 2] = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
        tv[0 as libc::c_int as usize].tv_sec = mtime;
        tv[0 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
        tv[1 as libc::c_int as usize].tv_sec = mtime;
        tv[1 as libc::c_int as usize].tv_usec = 0 as libc::c_int as __suseconds_t;
        utimes(target, tv.as_mut_ptr() as *const timeval);
    }
}
unsafe extern "C" fn buildUnixFilename(mut arg: *mut Arg_t) -> *mut libc::c_char {
    let mut target: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    target = mpPickTargetName(&mut (*arg).mp);
    ret = malloc(
        (strlen((*arg).unixTarget))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(target)),
    ) as *mut libc::c_char;
    if ret.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcpy(ret, (*arg).unixTarget);
    strcat(ret, b"/\0" as *const u8 as *const libc::c_char);
    if *target != 0 {
        if strcmp(target, b".\0" as *const u8 as *const libc::c_char) == 0 {
            target = b"DOT\0" as *const u8 as *const libc::c_char;
        } else if strcmp(target, b"..\0" as *const u8 as *const libc::c_char) == 0 {
            target = b"DOTDOT\0" as *const u8 as *const libc::c_char;
        }
        loop {
            tmp = strchr(target, '/' as i32);
            if tmp.is_null() {
                break;
            }
            strncat(ret, target, ptrdiff(tmp, target));
            strcat(ret, b"\\\0" as *const u8 as *const libc::c_char);
            target = tmp.offset(1 as libc::c_int as isize);
        }
        strcat(ret, target);
    }
    return ret;
}
unsafe extern "C" fn unix_is_dir(mut name: *const libc::c_char) -> libc::c_int {
    let mut buf: stat = stat {
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
    if stat(name, &mut buf) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        return (1 as libc::c_int != 0
            && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
    };
}
unsafe extern "C" fn unix_target_lookup(
    mut arg: *mut Arg_t,
    mut in_0: *const libc::c_char,
) -> libc::c_int {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    (*arg).unixTarget = strdup(in_0);
    if access((*arg).unixTarget, 0 as libc::c_int) == 0 as libc::c_int {
        match unix_is_dir((*arg).unixTarget) {
            -1 => return 16 as libc::c_int,
            0 => {}
            _ => return 4 as libc::c_int,
        }
    } else if *__errno_location() != 2 as libc::c_int {
        return 16 as libc::c_int
    }
    ptr = strrchr((*arg).unixTarget, '/' as i32);
    if ptr.is_null() {
        (*arg).mp.targetName = (*arg).unixTarget;
        (*arg).unixTarget = strdup(b".\0" as *const u8 as *const libc::c_char);
        return 4 as libc::c_int;
    } else {
        *ptr = '\0' as i32 as libc::c_char;
        (*arg).mp.targetName = ptr.offset(1 as libc::c_int as isize);
        return 4 as libc::c_int;
    };
}
pub unsafe extern "C" fn target_lookup(
    mut arg: *mut Arg_t,
    mut in_0: *const libc::c_char,
) -> libc::c_int {
    if *in_0.offset(0 as libc::c_int as isize) as libc::c_int != 0
        && *in_0.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
    {
        return dos_target_lookup(&mut (*arg).mp, in_0)
    } else {
        return unix_target_lookup(arg, in_0)
    };
}
unsafe extern "C" fn unix_write(
    mut mp: *mut MainParam_t,
    mut needfilter: libc::c_int,
) -> libc::c_int {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    if (*arg).type_0 != 0 {
        return _unix_write(mp, needfilter, b"-\0" as *const u8 as *const libc::c_char)
    } else {
        let mut unixFile: *mut libc::c_char = buildUnixFilename(arg);
        let mut ret: libc::c_int = 0;
        if unixFile.is_null() {
            printOom();
            return 16 as libc::c_int;
        }
        ret = _unix_write(mp, needfilter, unixFile);
        free(unixFile as *mut libc::c_void);
        return ret;
    };
}
unsafe extern "C" fn _unix_write(
    mut mp: *mut MainParam_t,
    mut needfilter: libc::c_int,
    mut unixFile: *const libc::c_char,
) -> libc::c_int {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut mtime: time_t = 0;
    let mut File: *mut Stream_t = (*mp).File;
    let mut Target: *mut Stream_t = 0 as *mut Stream_t;
    let mut Source: *mut Stream_t = 0 as *mut Stream_t;
    let mut stbuf: stat = stat {
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
    let mut errmsg: [libc::c_char; 80] = [0; 80];
    ((*(*File).Class).get_data)
        .unwrap()(
        File,
        &mut mtime,
        0 as *mut mt_off_t,
        0 as *mut libc::c_int,
        0 as *mut uint32_t,
    );
    if (*arg).preserveTime == 0 {
        mtime = 0 as libc::c_long;
    }
    if (*arg).type_0 == 0 {
        if (*arg).nowarn == 0 && access(unixFile, 0 as libc::c_int) == 0 {
            if (*arg).noClobber != 0 {
                fprintf(
                    stderr,
                    b"File \"%s\" exists. To overwrite, try again, and explicitly specify target directory\n\0"
                        as *const u8 as *const libc::c_char,
                    unixFile,
                );
                return 16 as libc::c_int;
            }
            if stat(unixFile, &mut stbuf) == 0 {
                let mut srcStbuf: stat = stat {
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
                let mut sFd: libc::c_int = 0;
                if !(stbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint)
                {
                    fprintf(
                        stderr,
                        b"\"%s\" is not a regular file\n\0" as *const u8
                            as *const libc::c_char,
                        unixFile,
                    );
                    return 16 as libc::c_int;
                }
                sFd = get_fd(File);
                if !(sFd == -(1 as libc::c_int)) {
                    if fstat(sFd, &mut srcStbuf) == 0 && stbuf.st_dev == srcStbuf.st_dev
                        && stbuf.st_ino == srcStbuf.st_ino
                    {
                        fprintf(
                            stderr,
                            b"Attempt to copy file on itself\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        return 16 as libc::c_int;
                    }
                }
            }
            if ask_confirmation(
                b"File \"%s\" exists, overwrite (y/n) ? \0" as *const u8
                    as *const libc::c_char,
                unixFile,
            ) != 0
            {
                return 16 as libc::c_int;
            }
        }
    }
    if (*arg).type_0 == 0 && (*arg).verbose != 0 {
        fprintf(stderr, b"Copying \0" as *const u8 as *const libc::c_char);
        mpPrintFilename(stderr, mp);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if got_signal != 0 {
        return 16 as libc::c_int;
    }
    Target = SimpleFileOpen(
        0 as *mut device,
        0 as *mut device,
        unixFile,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        errmsg.as_mut_ptr(),
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut mt_off_t,
    );
    if !Target.is_null() {
        let mut ret: mt_off_t = 0;
        Source = copy_stream(File);
        if needfilter != 0 && (*arg).textmode != 0 {
            Source = open_dos2unix(Source, (*arg).convertCharset);
        }
        if !Source.is_null() {
            ret = copyfile(Source, Target);
        } else {
            ret = -(1 as libc::c_int) as mt_off_t;
        }
        free_stream(&mut Source);
        free_stream(&mut Target);
        if ret < 0 as libc::c_int as libc::c_long {
            if (*arg).type_0 == 0 {
                unlink(unixFile);
            }
            return 16 as libc::c_int;
        }
        if (*arg).type_0 == 0 {
            set_mtime(unixFile, mtime);
        }
        return 4 as libc::c_int;
    } else {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            errmsg.as_mut_ptr(),
        );
        return 16 as libc::c_int;
    };
}
unsafe extern "C" fn makeUnixDir(mut filename: *mut libc::c_char) -> libc::c_int {
    if mkdir(filename, 0o777 as libc::c_int as __mode_t) == 0 {
        return 0 as libc::c_int;
    }
    if *__errno_location() == 17 as libc::c_int {
        let mut buf: stat = stat {
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
        if stat(filename, &mut buf) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        *__errno_location() = 20 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn unix_copydir(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut mtime: time_t = 0;
    let mut File: *mut Stream_t = (*mp).File;
    let mut ret: libc::c_int = 0;
    let mut unixFile: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*arg).recursive == 0 && (*mp).basenameHasWildcard != 0 {
        return 0 as libc::c_int;
    }
    ((*(*File).Class).get_data)
        .unwrap()(
        File,
        &mut mtime,
        0 as *mut mt_off_t,
        0 as *mut libc::c_int,
        0 as *mut uint32_t,
    );
    if (*arg).preserveTime == 0 {
        mtime = 0 as libc::c_long;
    }
    if (*arg).type_0 == 0 && (*arg).verbose != 0 {
        fprintf(stderr, b"Copying \0" as *const u8 as *const libc::c_char);
        fprintPwd(stderr, entry, 0 as libc::c_int);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if got_signal != 0 {
        return 16 as libc::c_int;
    }
    unixFile = buildUnixFilename(arg);
    if unixFile.is_null() {
        printOom();
        return 16 as libc::c_int;
    }
    if (*arg).type_0 != 0 || makeUnixDir(unixFile) == 0 {
        let mut newArg: Arg_t = Arg_t {
            recursive: 0,
            preserveAttributes: 0,
            preserveTime: 0,
            attr: 0,
            path: 0 as *mut libc::c_char,
            textmode: 0,
            needfilter: 0,
            nowarn: 0,
            verbose: 0,
            type_0: 0,
            convertCharset: 0,
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
            noClobber: 0,
            unixTarget: 0 as *const libc::c_char,
        };
        newArg = *arg;
        newArg.mp.arg = &mut newArg as *mut Arg_t as *mut libc::c_void;
        newArg.unixTarget = unixFile;
        newArg.mp.targetName = 0 as *const libc::c_char;
        newArg.mp.basenameHasWildcard = 1 as libc::c_int;
        ret = ((*mp).loop_0)
            .unwrap()(File, &mut newArg.mp, b"*\0" as *const u8 as *const libc::c_char);
        set_mtime(unixFile, mtime);
        free(unixFile as *mut libc::c_void);
        return ret | 4 as libc::c_int;
    } else {
        fprintf(
            stderr,
            b"Failure to make directory %s: %s\n\0" as *const u8 as *const libc::c_char,
            unixFile,
            strerror(*__errno_location()),
        );
        free(unixFile as *mut libc::c_void);
        return 16 as libc::c_int;
    };
}
unsafe extern "C" fn dos_to_unix(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    return unix_write(mp, 1 as libc::c_int);
}
unsafe extern "C" fn unix_to_unix(mut mp: *mut MainParam_t) -> libc::c_int {
    return unix_write(mp, 0 as libc::c_int);
}
unsafe extern "C" fn directory_dos_to_unix(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    return unix_copydir(entry, mp);
}
unsafe extern "C" fn writeit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut arg0: *mut libc::c_void,
    mut entry: *mut direntry_t,
) -> libc::c_int {
    let mut Target: *mut Stream_t = 0 as *mut Stream_t;
    let mut now: time_t = 0;
    let mut type_0: libc::c_int = 0;
    let mut ret: mt_off_t = 0;
    let mut fat: uint32_t = 0;
    let mut date: time_t = 0;
    let mut filesize: mt_off_t = 0;
    let mut arg: *mut Arg_t = arg0 as *mut Arg_t;
    let mut Source: *mut Stream_t = copy_stream((*arg).mp.File);
    if ((*(*Source).Class).get_data)
        .unwrap()(Source, &mut date, &mut filesize, &mut type_0, 0 as *mut uint32_t)
        < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Can't stat source file\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if fileTooBig(filesize) != 0 {
        fprintf(
            stderr,
            b"File \"%s\" too big\n\0" as *const u8 as *const libc::c_char,
            longname,
        );
        return 1 as libc::c_int;
    }
    if type_0 != 0 {
        if (*arg).verbose != 0 {
            fprintf(
                stderr,
                b"\"%s\" is a directory\n\0" as *const u8 as *const libc::c_char,
                longname,
            );
        }
        return -(1 as libc::c_int);
    }
    if (*arg).verbose != 0 {
        fprintf(stderr, b"Copying %s\n\0" as *const u8 as *const libc::c_char, longname);
    }
    if got_signal != 0 {
        return -(1 as libc::c_int);
    }
    if getfreeMinBytes((*arg).mp.targetDir, filesize) == 0 {
        return -(1 as libc::c_int);
    }
    if (*arg).preserveTime != 0 {
        now = date;
    } else {
        getTimeNow(&mut now);
    }
    mk_entry(
        dosname,
        (*arg).attr,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int as uint32_t,
        now,
        &mut (*entry).dir,
    );
    Target = OpenFileByDirentry(entry);
    if Target.is_null() {
        fprintf(
            stderr,
            b"Could not open Target\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if (*arg).needfilter & (*arg).textmode != 0 {
        Source = open_unix2dos(Source, (*arg).convertCharset);
    }
    ret = copyfile(Source, Target);
    ((*(*Target).Class).get_data)
        .unwrap()(
        Target,
        0 as *mut time_t,
        0 as *mut mt_off_t,
        0 as *mut libc::c_int,
        &mut fat,
    );
    free_stream(&mut Source);
    free_stream(&mut Target);
    if ret < 0 as libc::c_int as libc::c_long {
        fat_free((*arg).mp.targetDir, fat);
        return -(1 as libc::c_int);
    } else {
        mk_entry(dosname, (*arg).attr, fat, ret as uint32_t, now, &mut (*entry).dir);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn dos_write(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
    mut needfilter: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut targetName: *const libc::c_char = mpPickTargetName(mp);
    if !entry.is_null() && (*arg).preserveAttributes != 0 {
        (*arg).attr = (*entry).dir.attr;
    } else {
        (*arg).attr = 0x20 as libc::c_int as libc::c_uchar;
    }
    (*arg).needfilter = needfilter;
    if !entry.is_null() && (*mp).targetDir == (*entry).Dir {
        (*arg).ch.ignore_entry = -(1 as libc::c_int);
        (*arg).ch.source = (*entry).entry;
    } else {
        (*arg).ch.ignore_entry = -(1 as libc::c_int);
        (*arg).ch.source = -(2 as libc::c_int);
    }
    result = mwrite_one(
        (*mp).targetDir,
        targetName,
        0 as *const libc::c_char,
        Some(
            writeit
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
unsafe extern "C" fn subDir(
    mut parent: *mut Stream_t,
    mut filename: *const libc::c_char,
) -> *mut Stream_t {
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
    initializeDirentry(&mut entry, parent);
    match vfat_lookup_zt(
        &mut entry,
        filename,
        0x10 as libc::c_int,
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
        0 as *mut libc::c_char,
        0 as libc::c_int as size_t,
    ) {
        0 => return OpenFileByDirentry(&mut entry),
        -1 => return 0 as *mut Stream_t,
        _ => return 0 as *mut Stream_t,
    };
}
unsafe extern "C" fn dos_copydir(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut newArg: Arg_t = Arg_t {
        recursive: 0,
        preserveAttributes: 0,
        preserveTime: 0,
        attr: 0,
        path: 0 as *mut libc::c_char,
        textmode: 0,
        needfilter: 0,
        nowarn: 0,
        verbose: 0,
        type_0: 0,
        convertCharset: 0,
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
        noClobber: 0,
        unixTarget: 0 as *const libc::c_char,
    };
    let mut now: time_t = 0;
    let mut date: time_t = 0;
    let mut ret: libc::c_int = 0;
    let mut targetName: *const libc::c_char = mpPickTargetName(mp);
    if (*arg).recursive == 0 && (*mp).basenameHasWildcard != 0 {
        return 0 as libc::c_int;
    }
    if !entry.is_null() && isSubdirOf((*mp).targetDir, (*mp).File) != 0 {
        fprintf(
            stderr,
            b"Cannot recursively copy directory \0" as *const u8 as *const libc::c_char,
        );
        fprintPwd(stderr, entry, 0 as libc::c_int);
        fprintf(
            stderr,
            b" into one of its own subdirectories \0" as *const u8 as *const libc::c_char,
        );
        fprintPwd(stderr, getDirentry((*mp).targetDir), 0 as libc::c_int);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        return 16 as libc::c_int;
    }
    if ((*(*(*arg).mp.File).Class).get_data)
        .unwrap()(
        (*arg).mp.File,
        &mut date,
        0 as *mut mt_off_t,
        0 as *mut libc::c_int,
        0 as *mut uint32_t,
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Can't stat source file\n\0" as *const u8 as *const libc::c_char,
        );
        return 16 as libc::c_int;
    }
    if (*arg).type_0 == 0 && (*arg).verbose != 0 {
        fprintf(
            stderr,
            b"Copying %s\n\0" as *const u8 as *const libc::c_char,
            mpGetBasename(mp),
        );
    }
    if !entry.is_null() && (*arg).preserveAttributes != 0 {
        (*arg).attr = (*entry).dir.attr;
    } else {
        (*arg).attr = 0 as libc::c_int as libc::c_uchar;
    }
    if !entry.is_null() && (*mp).targetDir == (*entry).Dir {
        (*arg).ch.ignore_entry = -(1 as libc::c_int);
        (*arg).ch.source = (*entry).entry;
    } else {
        (*arg).ch.ignore_entry = -(1 as libc::c_int);
        (*arg).ch.source = -(2 as libc::c_int);
    }
    if (*arg).preserveTime != 0 {
        now = date;
    } else {
        getTimeNow(&mut now);
    }
    newArg = *arg;
    newArg.mp.arg = &mut newArg as *mut Arg_t as *mut libc::c_void;
    newArg.mp.targetName = 0 as *const libc::c_char;
    newArg.mp.basenameHasWildcard = 1 as libc::c_int;
    if *targetName != 0 {
        newArg.mp.targetDir = subDir((*mp).targetDir, targetName);
        if (newArg.mp.targetDir).is_null() {
            newArg
                .mp
                .targetDir = createDir(
                (*mp).targetDir,
                targetName,
                &mut (*arg).ch,
                (*arg).attr,
                now,
            );
        }
    } else {
        newArg.mp.targetDir = (*mp).targetDir;
    }
    if (newArg.mp.targetDir).is_null() {
        return 16 as libc::c_int;
    }
    ret = ((*mp).loop_0)
        .unwrap()(
        (*mp).File,
        &mut newArg.mp,
        b"*\0" as *const u8 as *const libc::c_char,
    );
    if *targetName != 0 {
        free_stream(&mut newArg.mp.targetDir);
    }
    return ret | 4 as libc::c_int;
}
unsafe extern "C" fn dos_to_dos(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    return dos_write(entry, mp, 0 as libc::c_int);
}
unsafe extern "C" fn unix_to_dos(mut mp: *mut MainParam_t) -> libc::c_int {
    return dos_write(0 as *mut direntry_t, mp, 1 as libc::c_int);
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
        b"Usage: %s [-spatnmQVBT] [-D clash_option] sourcefile targetfile\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    fprintf(
        stderr,
        b"       %s [-spatnmQVBT] [-D clash_option] sourcefile [sourcefiles...] targetdirectory\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn mcopy(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut mtype: libc::c_int,
) {
    let mut arg: Arg_t = Arg_t {
        recursive: 0,
        preserveAttributes: 0,
        preserveTime: 0,
        attr: 0,
        path: 0 as *mut libc::c_char,
        textmode: 0,
        needfilter: 0,
        nowarn: 0,
        verbose: 0,
        type_0: 0,
        convertCharset: 0,
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
        noClobber: 0,
        unixTarget: 0 as *const libc::c_char,
    };
    let mut c: libc::c_int = 0;
    let mut fastquit: libc::c_int = 0;
    init_clash_handling(&mut arg.ch);
    arg.recursive = 0 as libc::c_int;
    arg.preserveTime = 0 as libc::c_int;
    arg.preserveAttributes = 0 as libc::c_int;
    arg.nowarn = 0 as libc::c_int;
    arg.textmode = 0 as libc::c_int;
    arg.verbose = 0 as libc::c_int;
    arg.convertCharset = 0 as libc::c_int;
    arg.type_0 = mtype;
    fastquit = 0 as libc::c_int;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(
            argc,
            argv,
            b"i:abB/sptTnmvQD:oh\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_26: u64;
        match c {
            105 => {
                set_cmd_line_image(optarg);
                current_block_26 = 11932355480408055363;
            }
            115 | 47 => {
                arg.recursive = 1 as libc::c_int;
                current_block_26 = 11932355480408055363;
            }
            112 => {
                arg.preserveAttributes = 1 as libc::c_int;
                current_block_26 = 11932355480408055363;
            }
            84 => {
                arg.convertCharset = 1 as libc::c_int;
                current_block_26 = 162301573416510823;
            }
            97 | 116 => {
                current_block_26 = 162301573416510823;
            }
            110 => {
                arg.nowarn = 1 as libc::c_int;
                current_block_26 = 11932355480408055363;
            }
            109 => {
                arg.preserveTime = 1 as libc::c_int;
                current_block_26 = 11932355480408055363;
            }
            118 => {
                arg.verbose = 1 as libc::c_int;
                current_block_26 = 11932355480408055363;
            }
            81 => {
                fastquit = 1 as libc::c_int;
                current_block_26 = 11932355480408055363;
            }
            66 | 98 => {
                batchmode = 1 as libc::c_int;
                current_block_26 = 11932355480408055363;
            }
            111 => {
                handle_clash_options(&mut arg.ch, c);
                current_block_26 = 11932355480408055363;
            }
            68 => {
                if handle_clash_options(&mut arg.ch, *optarg as libc::c_int) != 0 {
                    usage(1 as libc::c_int);
                }
                current_block_26 = 11932355480408055363;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            63 => {
                usage(1 as libc::c_int);
            }
            _ => {
                current_block_26 = 11932355480408055363;
            }
        }
        match current_block_26 {
            162301573416510823 => {
                arg.textmode = 1 as libc::c_int;
            }
            _ => {}
        }
    }
    if argc - optind < 1 as libc::c_int {
        usage(1 as libc::c_int);
    }
    init_mp(&mut arg.mp);
    arg.unixTarget = 0 as *const libc::c_char;
    arg
        .mp
        .lookupflags = 0x20 as libc::c_int | 0x10 as libc::c_int | 1 as libc::c_int
        | 0x100 as libc::c_int | 0x2000 as libc::c_int;
    arg.mp.fast_quit = fastquit;
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.openflags = 0 as libc::c_int;
    arg.noClobber = 0 as libc::c_int;
    if mtype == 0
        && strcmp(
            *argv.offset((argc - 1 as libc::c_int) as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        mtype = 1 as libc::c_int;
        arg.type_0 = mtype;
        argc -= 1;
        argc;
    }
    if mtype != 0 {
        arg.mp.targetName = strdup(b"-\0" as *const u8 as *const libc::c_char);
        arg.unixTarget = strdup(b"\0" as *const u8 as *const libc::c_char);
        arg
            .mp
            .callback = Some(
            dos_to_unix
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
        arg
            .mp
            .dirCallback = Some(
            unix_copydir
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
        );
        arg
            .mp
            .unixcallback = Some(
            unix_to_unix as unsafe extern "C" fn(*mut MainParam_t) -> libc::c_int,
        );
    } else {
        let mut target: *const libc::c_char = 0 as *const libc::c_char;
        if argc - optind == 1 as libc::c_int {
            target = b".\0" as *const u8 as *const libc::c_char;
            arg.noClobber = 1 as libc::c_int;
        } else {
            argc -= 1;
            argc;
            target = *argv.offset(argc as isize);
        }
        if target_lookup(&mut arg, target) == 16 as libc::c_int {
            fprintf(
                stderr,
                b"%s: %s\n\0" as *const u8 as *const libc::c_char,
                target,
                strerror(*__errno_location()),
            );
            exit(1 as libc::c_int);
        }
        if (arg.mp.targetDir).is_null() && (arg.unixTarget).is_null() {
            fprintf(
                stderr,
                b"Bad target %s\n\0" as *const u8 as *const libc::c_char,
                target,
            );
            exit(1 as libc::c_int);
        }
        if !(arg.unixTarget).is_null() {
            arg
                .mp
                .callback = Some(
                dos_to_unix
                    as unsafe extern "C" fn(
                        *mut direntry_t,
                        *mut MainParam_t,
                    ) -> libc::c_int,
            );
            arg
                .mp
                .dirCallback = Some(
                directory_dos_to_unix
                    as unsafe extern "C" fn(
                        *mut direntry_t,
                        *mut MainParam_t,
                    ) -> libc::c_int,
            );
            arg
                .mp
                .unixcallback = Some(
                unix_to_unix as unsafe extern "C" fn(*mut MainParam_t) -> libc::c_int,
            );
        } else {
            arg
                .mp
                .dirCallback = Some(
                dos_copydir
                    as unsafe extern "C" fn(
                        *mut direntry_t,
                        *mut MainParam_t,
                    ) -> libc::c_int,
            );
            arg
                .mp
                .callback = Some(
                dos_to_dos
                    as unsafe extern "C" fn(
                        *mut direntry_t,
                        *mut MainParam_t,
                    ) -> libc::c_int,
            );
            arg
                .mp
                .unixcallback = Some(
                unix_to_dos as unsafe extern "C" fn(*mut MainParam_t) -> libc::c_int,
            );
        }
    }
    exit(main_loop(&mut arg.mp, argv.offset(optind as isize), argc - optind));
}
