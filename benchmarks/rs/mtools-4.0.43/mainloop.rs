use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _basename(filename: *const libc::c_char) -> *const libc::c_char;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn open_root_dir(
        drivename: libc::c_char,
        flags: libc::c_int,
        isRop: *mut libc::c_int,
    ) -> *mut Stream_t;
    static mut got_signal: libc::c_int;
    fn get_default_drive() -> libc::c_char;
    static mut progname: *const libc::c_char;
    fn OpenDir(filename: *const libc::c_char) -> *mut Stream_t;
    fn open_mcwd(mode: *const libc::c_char) -> *mut FILE;
    fn unlink_mcwd();
    fn fat_error(Dir: *mut Stream_t) -> libc::c_int;
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
    fn vfat_lookup_zt(
        entry: *mut direntry_t,
        filename: *const libc::c_char,
        flags: libc::c_int,
        shortname: *mut libc::c_char,
        shortname_len: size_t,
        longname: *mut libc::c_char,
        longname_len: size_t,
    ) -> libc::c_int;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: libc::c_int);
    fn getParent(entry: *mut direntry_t) -> *mut direntry_t;
    fn unix_dir_loop(Stream: *mut Stream_t, mp: *mut MainParam_t) -> libc::c_int;
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
    fn OpenFileByDirentry(entry: *mut direntry_t) -> *mut Stream_t;
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
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
pub type __int32_t = libc::c_int;
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
pub struct lookupState_t {
    pub container: *mut Stream_t,
    pub nbContainers: libc::c_int,
    pub Dir: *mut Stream_t,
    pub nbDirs: libc::c_int,
    pub filename: *const libc::c_char,
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
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
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
unsafe extern "C" fn ptrdiff(
    mut end: *const libc::c_char,
    mut begin: *const libc::c_char,
) -> size_t {
    return end.offset_from(begin) as libc::c_long as size_t;
}
unsafe extern "C" fn fix_mcwd(mut ans: *mut libc::c_char) -> *const libc::c_char {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    fp = open_mcwd(b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() || (fgets(buf.as_mut_ptr(), 128 as libc::c_int, fp)).is_null() {
        if !fp.is_null() {
            fclose(fp);
        }
        *ans.offset(0 as libc::c_int as isize) = get_default_drive();
        strcpy(
            ans.offset(1 as libc::c_int as isize),
            b":/\0" as *const u8 as *const libc::c_char,
        );
        return ans;
    }
    buf[(strlen(buf.as_mut_ptr())).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    fclose(fp);
    s = buf.as_mut_ptr();
    if buf[0 as libc::c_int as usize] as libc::c_int != 0
        && buf[1 as libc::c_int as usize] as libc::c_int == ':' as i32
    {
        memcpy(
            ans as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        );
        *ans.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        s = &mut *buf.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut libc::c_char;
    } else {
        *ans.offset(0 as libc::c_int as isize) = get_default_drive();
        strcpy(
            ans.offset(1 as libc::c_int as isize),
            b":\0" as *const u8 as *const libc::c_char,
        );
    }
    if *s as libc::c_int != '/' as i32 && *s as libc::c_int != '\\' as i32 {
        strcat(ans, b"/\0" as *const u8 as *const libc::c_char);
        strcat(ans, s);
    } else {
        strcat(ans, s);
    }
    if strlen(ans) == 3 as libc::c_int as libc::c_ulong {
        return ans;
    }
    s = s.offset(-1);
    if *s as libc::c_int == '/' as i32 {
        *s = '\0' as i32 as libc::c_char;
    }
    return ans;
}
unsafe extern "C" fn _unix_loop(
    mut Dir: *mut Stream_t,
    mut mp: *mut MainParam_t,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    return unix_dir_loop(Dir, mp);
}
pub unsafe extern "C" fn unix_loop(
    mut Stream: *mut Stream_t,
    mut mp: *mut MainParam_t,
    mut arg: *mut libc::c_char,
    mut follow_dir_link: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut isdir: libc::c_int = 0 as libc::c_int;
    let mut unixNameLength: size_t = 0;
    (*mp).File = 0 as *mut Stream_t;
    (*mp).direntry = 0 as *mut direntry_t;
    unixNameLength = strlen(arg);
    if unixNameLength > 1 as libc::c_int as libc::c_ulong
        && *arg
            .offset(
                unixNameLength.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == '/' as i32
    {
        let mut name: *mut libc::c_char = strdup(arg);
        *name
            .offset(
                unixNameLength.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        (*mp).unixSourceName = name;
    } else {
        (*mp).unixSourceName = arg;
    }
    (*mp)
        .loop_0 = Some(
        _unix_loop
            as unsafe extern "C" fn(
                *mut Stream_t,
                *mut MainParam_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    if (*mp).lookupflags & 1 as libc::c_int != 0 {
        (*mp)
            .File = SimpleFileOpen(
            0 as *mut device,
            0 as *mut device,
            arg,
            0 as libc::c_int,
            0 as *mut libc::c_char,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as *mut mt_off_t,
        );
        if ((*mp).File).is_null() {
            perror(arg);
            return 16 as libc::c_int;
        }
        ((*(*(*mp).File).Class).get_data)
            .unwrap()(
            (*mp).File,
            0 as *mut time_t,
            0 as *mut mt_off_t,
            &mut isdir,
            0 as *mut uint32_t,
        );
        if isdir != 0 {
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
            free_stream(&mut (*mp).File);
            if follow_dir_link == 0 && lstat(arg, &mut buf) == 0 as libc::c_int
                && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"skipping directory symlink %s\n\0" as *const u8
                        as *const libc::c_char,
                    arg,
                );
                return 0 as libc::c_int;
            }
            if (*mp).lookupflags & 0x10 as libc::c_int == 0 {
                return 0 as libc::c_int;
            }
            (*mp).File = OpenDir(arg);
        }
    }
    if isdir != 0 {
        ret = ((*mp).dirCallback).unwrap()(0 as *mut direntry_t, mp);
    } else {
        ret = ((*mp).unixcallback).unwrap()(mp);
    }
    free_stream(&mut (*mp).File);
    return ret;
}
unsafe extern "C" fn checkForDot(
    mut lookupflags: libc::c_int,
    mut name: *const wchar_t,
) -> libc::c_int {
    return (lookupflags & 0x100 as libc::c_int != 0 && isSpecialW(name) != 0)
        as libc::c_int;
}
unsafe extern "C" fn isUniqueTarget(mut name: *const libc::c_char) -> libc::c_int {
    return (!name.is_null()
        && strcmp(name, b"-\0" as *const u8 as *const libc::c_char) != 0) as libc::c_int;
}
unsafe extern "C" fn handle_leaf(
    mut direntry: *mut direntry_t,
    mut mp: *mut MainParam_t,
    mut lookupState: *mut lookupState_t,
    mut DeferredFileP: *mut *mut Stream_t,
) -> libc::c_int {
    let mut MyFile: *mut Stream_t = 0 as *mut Stream_t;
    let mut ret: libc::c_int = 0;
    if got_signal != 0 {
        return 16 as libc::c_int;
    }
    if !lookupState.is_null() {
        match (*lookupState).nbDirs {
            0 => {
                (*lookupState).Dir = OpenFileByDirentry(direntry);
                (*lookupState).nbDirs += 1;
                (*lookupState).nbDirs;
                free_stream(&mut (*lookupState).container);
                return 0 as libc::c_int;
            }
            1 => {
                free_stream(&mut (*lookupState).Dir);
                fprintf(stderr, b"Ambiguous\n\0" as *const u8 as *const libc::c_char);
                return 32 as libc::c_int | 16 as libc::c_int;
            }
            _ => return 32 as libc::c_int | 16 as libc::c_int,
        }
    }
    (*mp).direntry = direntry;
    if (*direntry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        if (*mp).lookupflags & (1 as libc::c_int | 0x400 as libc::c_int) != 0 {
            (*mp).File = OpenFileByDirentry(direntry);
            MyFile = (*mp).File;
        }
        ret = ((*mp).dirCallback).unwrap()(direntry, mp);
    } else {
        if (*mp).lookupflags & 1 as libc::c_int != 0 {
            if !DeferredFileP.is_null() && !(*DeferredFileP).is_null() {
                free_stream(DeferredFileP);
                fprintf(
                    stderr,
                    b"Attempt to copy multiple files to non-directory\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 32 as libc::c_int | 16 as libc::c_int;
            }
            (*mp).File = OpenFileByDirentry(direntry);
            MyFile = (*mp).File;
            if !DeferredFileP.is_null() {
                *DeferredFileP = MyFile;
                return 0 as libc::c_int;
            }
        }
        ret = ((*mp).callback).unwrap()(direntry, mp);
    }
    free_stream(&mut MyFile);
    return ret;
}
unsafe extern "C" fn _dos_loop(
    mut Dir: *mut Stream_t,
    mut mp: *mut MainParam_t,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut MyFile: *mut Stream_t = 0 as *mut Stream_t;
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
    let mut ret: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    ret = 0 as libc::c_int;
    r = 0 as libc::c_int;
    initializeDirentry(&mut entry, Dir);
    while got_signal == 0
        && {
            r = vfat_lookup_zt(
                &mut entry,
                filename,
                (*mp).lookupflags,
                (*mp).shortname.data,
                (*mp).shortname.len,
                (*mp).longname.data,
                (*mp).longname.len,
            );
            r == 0 as libc::c_int
        }
    {
        (*mp).File = 0 as *mut Stream_t;
        if checkForDot((*mp).lookupflags, (entry.name).as_mut_ptr()) == 0 {
            MyFile = 0 as *mut Stream_t;
            if (*mp).lookupflags & 1 as libc::c_int != 0
                || entry.dir.attr as libc::c_int & 0x10 as libc::c_int != 0
                    && (*mp).lookupflags & 0x400 as libc::c_int != 0
            {
                (*mp).File = OpenFileByDirentry(&mut entry);
                MyFile = (*mp).File;
            }
            if got_signal != 0 {
                break;
            }
            (*mp).direntry = &mut entry;
            if entry.dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
                ret |= ((*mp).dirCallback).unwrap()(&mut entry, mp);
            } else {
                ret |= ((*mp).callback).unwrap()(&mut entry, mp);
            }
            free_stream(&mut MyFile);
        }
        if fat_error(Dir) != 0 {
            ret |= 16 as libc::c_int;
        }
        if (*mp).fast_quit != 0 && ret & 16 as libc::c_int != 0 {
            break;
        }
    }
    if r == -(2 as libc::c_int) {
        return 16 as libc::c_int;
    }
    if got_signal != 0 {
        ret |= 16 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn recurs_dos_loop(
    mut mp: *mut MainParam_t,
    mut filename0: *const libc::c_char,
    mut filename1: *const libc::c_char,
    mut lookupState: *mut lookupState_t,
    mut DeferredFileP: *mut *mut Stream_t,
) -> libc::c_int {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
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
    let mut length: size_t = 0;
    let mut lookupflags: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut have_one: libc::c_int = 0;
    let mut doing_mcwd: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    loop {
        if strncmp(
            filename0,
            b"./\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            filename0 = filename0.offset(2 as libc::c_int as isize);
        } else if strcmp(filename0, b".\0" as *const u8 as *const libc::c_char) == 0
            && !filename1.is_null()
        {
            filename0 = filename0.offset(1);
            filename0;
        } else if *filename0.offset(0 as libc::c_int as isize) as libc::c_int
            == '/' as i32
        {
            filename0 = filename0.offset(1);
            filename0;
        } else {
            if !(*filename0.offset(0 as libc::c_int as isize) == 0) {
                break;
            }
            if filename1.is_null() {
                break;
            }
            filename0 = filename1;
            filename1 = 0 as *const libc::c_char;
        }
    }
    if strncmp(
        filename0,
        b"../\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
        || strcmp(filename0, b"..\0" as *const u8 as *const libc::c_char) == 0
            && !filename1.is_null()
    {
        (*mp).File = (*getDirentry((*mp).File)).Dir;
        return recurs_dos_loop(
            mp,
            filename0.offset(2 as libc::c_int as isize),
            filename1,
            lookupState,
            DeferredFileP,
        );
    }
    doing_mcwd = !filename1.is_null() as libc::c_int;
    ptr = strchr(filename0, '/' as i32);
    if ptr.is_null() {
        length = strlen(filename0);
        ptr = filename1;
        filename1 = 0 as *const libc::c_char;
    } else {
        length = ptrdiff(ptr, filename0);
        ptr = ptr.offset(1);
        ptr;
    }
    if ptr.is_null() {
        if (*mp).lookupflags & 0x1000 as libc::c_int != 0 {
            (*mp).targetName = filename0;
            ret = handle_leaf(
                getDirentry((*mp).File),
                mp,
                lookupState,
                0 as *mut *mut Stream_t,
            );
            (*mp).targetName = 0 as *const libc::c_char;
            return ret;
        }
        if strcmp(filename0, b".\0" as *const u8 as *const libc::c_char) == 0
            || *filename0.offset(0 as libc::c_int as isize) == 0
        {
            return handle_leaf(
                getDirentry((*mp).File),
                mp,
                lookupState,
                0 as *mut *mut Stream_t,
            );
        }
        if strcmp(filename0, b"..\0" as *const u8 as *const libc::c_char) == 0 {
            return handle_leaf(
                getParent(getDirentry((*mp).File)),
                mp,
                lookupState,
                0 as *mut *mut Stream_t,
            );
        }
        lookupflags = (*mp).lookupflags;
        if !lookupState.is_null() {
            (*lookupState).filename = filename0;
            if (*lookupState).nbContainers + (*lookupState).nbDirs > 0 as libc::c_int {
                free_stream(&mut (*lookupState).container);
            } else {
                (*lookupState).container = copy_stream((*mp).File);
            }
            (*lookupState).nbContainers += 1;
            (*lookupState).nbContainers;
        }
    } else {
        lookupflags = 0x10 as libc::c_int | 1 as libc::c_int | 0x100 as libc::c_int;
    }
    ret = 0 as libc::c_int;
    r = 0 as libc::c_int;
    have_one = 0 as libc::c_int;
    initializeDirentry(&mut entry, (*mp).File);
    while ret & 32 as libc::c_int == 0 && got_signal == 0
        && {
            r = vfat_lookup(
                &mut entry,
                filename0,
                length,
                lookupflags | 0x80 as libc::c_int,
                (*mp).shortname.data,
                (*mp).shortname.len,
                (*mp).longname.data,
                (*mp).longname.len,
            );
            r == 0 as libc::c_int
        }
    {
        if checkForDot(lookupflags, (entry.name).as_mut_ptr()) != 0 {
            continue;
        }
        have_one = 1 as libc::c_int;
        if !ptr.is_null() {
            let mut SubDir: *mut Stream_t = 0 as *mut Stream_t;
            (*mp).File = OpenFileByDirentry(&mut entry);
            SubDir = (*mp).File;
            ret |= recurs_dos_loop(mp, ptr, filename1, lookupState, DeferredFileP);
            free_stream(&mut SubDir);
        } else {
            ret |= handle_leaf(&mut entry, mp, lookupState, DeferredFileP);
        }
        if doing_mcwd != 0 {
            break;
        }
    }
    if r == -(2 as libc::c_int) {
        return 16 as libc::c_int;
    }
    if got_signal != 0 {
        return ret | 16 as libc::c_int;
    }
    if doing_mcwd != 0 && have_one == 0 {
        return 8 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn common_dos_loop(
    mut mp: *mut MainParam_t,
    mut pathname: *const libc::c_char,
    mut lookupState: *mut lookupState_t,
    mut open_mode: libc::c_int,
) -> libc::c_int {
    let mut RootDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut drive: libc::c_char = 0;
    let mut DeferredFile: *mut Stream_t = 0 as *mut Stream_t;
    let mut DeferredFileP: *mut *mut Stream_t = 0 as *mut *mut Stream_t;
    let mut ret: libc::c_int = 0;
    (*mp)
        .loop_0 = Some(
        _dos_loop
            as unsafe extern "C" fn(
                *mut Stream_t,
                *mut MainParam_t,
                *const libc::c_char,
            ) -> libc::c_int,
    );
    drive = '\0' as i32 as libc::c_char;
    cwd = b"\0" as *const u8 as *const libc::c_char;
    if *pathname as libc::c_int != 0
        && *pathname.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
    {
        drive = ch_toupper(*pathname);
        pathname = pathname.offset(2 as libc::c_int as isize);
        if (*mp).mcwd[0 as libc::c_int as usize] as libc::c_int == drive as libc::c_int {
            cwd = ((*mp).mcwd).as_mut_ptr().offset(2 as libc::c_int as isize);
        }
    } else if (*mp).mcwd[0 as libc::c_int as usize] != 0 {
        drive = (*mp).mcwd[0 as libc::c_int as usize];
        cwd = ((*mp).mcwd).as_mut_ptr().offset(2 as libc::c_int as isize);
    } else {
        drive = get_default_drive();
    }
    if *pathname as libc::c_int == '/' as i32 {
        cwd = b"\0" as *const u8 as *const libc::c_char;
    }
    (*mp).File = open_root_dir(drive, open_mode, 0 as *mut libc::c_int);
    RootDir = (*mp).File;
    if ((*mp).File).is_null() {
        return 16 as libc::c_int;
    }
    if !((*mp).originalArg).is_null()
        && !(strpbrk((*mp).originalArg, b"*[?\0" as *const u8 as *const libc::c_char))
            .is_null() && (*mp).lookupflags & 0x2000 as libc::c_int != 0
        && isUniqueTarget((*mp).targetName) != 0
    {
        DeferredFileP = &mut DeferredFile;
    }
    ret = recurs_dos_loop(mp, cwd, pathname, lookupState, DeferredFileP);
    if ret & 8 as libc::c_int != 0 {
        *((*mp).mcwd).as_mut_ptr() = '\0' as i32 as libc::c_char;
        unlink_mcwd();
        ret = recurs_dos_loop(
            mp,
            b"\0" as *const u8 as *const libc::c_char,
            pathname,
            lookupState,
            DeferredFileP,
        );
    }
    if !DeferredFile.is_null() {
        (*mp).File = DeferredFile;
        ret = ((*mp).callback).unwrap()(0 as *mut direntry_t, mp);
        free_stream(&mut DeferredFile);
    }
    free_stream(&mut RootDir);
    return ret;
}
unsafe extern "C" fn dos_loop(
    mut mp: *mut MainParam_t,
    mut arg: *const libc::c_char,
) -> libc::c_int {
    return common_dos_loop(mp, arg, 0 as *mut lookupState_t, (*mp).openflags);
}
pub unsafe extern "C" fn dos_target_lookup(
    mut mp: *mut MainParam_t,
    mut arg: *const libc::c_char,
) -> libc::c_int {
    let mut lookupState: lookupState_t = lookupState_t {
        container: 0 as *mut Stream_t,
        nbContainers: 0,
        Dir: 0 as *mut Stream_t,
        nbDirs: 0,
        filename: 0 as *const libc::c_char,
    };
    let mut ret: libc::c_int = 0;
    let mut lookupflags: libc::c_int = 0;
    lookupState.nbDirs = 0 as libc::c_int;
    lookupState.Dir = 0 as *mut Stream_t;
    lookupState.nbContainers = 0 as libc::c_int;
    lookupState.container = 0 as *mut Stream_t;
    lookupflags = (*mp).lookupflags;
    (*mp).lookupflags = 1 as libc::c_int | 0x10 as libc::c_int;
    ret = common_dos_loop(mp, arg, &mut lookupState, 0o2 as libc::c_int);
    (*mp).lookupflags = lookupflags;
    if ret & 16 as libc::c_int != 0 {
        return ret;
    }
    if lookupState.nbDirs != 0 {
        (*mp).targetName = 0 as *const libc::c_char;
        (*mp).targetDir = lookupState.Dir;
        free_stream(&mut lookupState.container);
        return ret;
    }
    match lookupState.nbContainers {
        0 => {
            fprintf(
                stderr,
                b"%s: no match for target\n\0" as *const u8 as *const libc::c_char,
                arg,
            );
            return 2 as libc::c_int;
        }
        1 => {
            (*mp).targetName = strdup(lookupState.filename);
            (*mp).targetDir = lookupState.container;
            return ret;
        }
        _ => {
            fprintf(
                stderr,
                b"Ambiguous %s\n\0" as *const u8 as *const libc::c_char,
                arg,
            );
            return 16 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn main_loop(
    mut mp: *mut MainParam_t,
    mut argv: *mut *mut libc::c_char,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut Bret: libc::c_int = 0;
    Bret = 0 as libc::c_int;
    if argc != 1 as libc::c_int && !((*mp).targetName).is_null() {
        fprintf(
            stderr,
            b"Several file names given, but last argument (%s) not a directory\n\0"
                as *const u8 as *const libc::c_char,
            (*mp).targetName,
        );
        free_stream(&mut (*mp).targetDir);
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < argc {
        if got_signal != 0 {
            break;
        }
        (*mp).originalArg = *argv.offset(i as isize);
        (*mp)
            .basenameHasWildcard = (strpbrk(
            _basename((*mp).originalArg),
            b"*[?\0" as *const u8 as *const libc::c_char,
        ) != 0 as *mut libc::c_char) as libc::c_int;
        if ((*mp).unixcallback).is_some()
            && (*(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) == 0
                || *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int != ':' as i32)
        {
            ret = unix_loop(
                0 as *mut Stream_t,
                mp,
                *argv.offset(i as isize),
                1 as libc::c_int,
            );
        } else {
            ret = dos_loop(mp, *argv.offset(i as isize));
        }
        if ret & (4 as libc::c_int | 16 as libc::c_int) == 0 {
            fprintf(
                stderr,
                b"%s: File \"%s\" not found\n\0" as *const u8 as *const libc::c_char,
                progname,
                *argv.offset(i as isize),
            );
            ret |= 16 as libc::c_int;
        }
        Bret |= ret;
        if (*mp).fast_quit != 0 && Bret & (2 as libc::c_int | 16 as libc::c_int) != 0 {
            break;
        }
        i += 1;
        i;
    }
    free_stream(&mut (*mp).targetDir);
    if Bret & 16 as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    if Bret & 4 as libc::c_int != 0 && Bret & 2 as libc::c_int != 0 {
        return 2 as libc::c_int;
    }
    if Bret & 2 as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dispatchToFile(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    if !entry.is_null() {
        return ((*mp).callback).unwrap()(entry, mp)
    } else {
        return ((*mp).unixcallback).unwrap()(mp)
    };
}
pub unsafe extern "C" fn init_mp(mut mp: *mut MainParam_t) {
    fix_mcwd(((*mp).mcwd).as_mut_ptr());
    (*mp).openflags = 0 as libc::c_int;
    (*mp).lookupflags = 0 as libc::c_int;
    (*mp).targetName = 0 as *const libc::c_char;
    (*mp).targetDir = 0 as *mut Stream_t;
    (*mp)
        .dirCallback = Some(
        dispatchToFile
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    (*mp).unixcallback = None;
    (*mp).longname.data = 0 as *mut libc::c_char;
    (*mp).shortname.data = (*mp).longname.data;
    (*mp).longname.len = 0 as libc::c_int as size_t;
    (*mp).shortname.len = (*mp).longname.len;
    (*mp).File = 0 as *mut Stream_t;
    (*mp).fast_quit = 0 as libc::c_int;
    (*mp).originalArg = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn mpGetBasename(mut mp: *mut MainParam_t) -> *const libc::c_char {
    if !((*mp).direntry).is_null() {
        wchar_to_native(
            ((*(*mp).direntry).name).as_mut_ptr(),
            ((*mp).targetBuffer).as_mut_ptr(),
            (255 as libc::c_int + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<[libc::c_char; 1021]>() as libc::c_ulong,
        );
        return ((*mp).targetBuffer).as_mut_ptr();
    } else {
        return _basename((*mp).unixSourceName)
    };
}
pub unsafe extern "C" fn mpPrintFilename(mut fp: *mut FILE, mut mp: *mut MainParam_t) {
    if !((*mp).direntry).is_null() {
        fprintPwd(fp, (*mp).direntry, 0 as libc::c_int);
    } else {
        fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, (*mp).originalArg);
    };
}
pub unsafe extern "C" fn mpPickTargetName(
    mut mp: *mut MainParam_t,
) -> *const libc::c_char {
    if !((*mp).targetName).is_null() {
        return (*mp).targetName
    } else {
        return mpGetBasename(mp)
    };
}
