use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(size: libc::c_uint) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn fatal(string: *const libc::c_char, a0: *const libc::c_char);
    fn message(
        kind: *mut libc::c_char,
        string: *mut libc::c_char,
        a0: *mut libc::c_char,
        a1: *mut libc::c_char,
    );
}
pub type size_t = libc::c_ulong;
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
pub type __syscall_slong_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type exit_values = libc::c_uint;
pub const system_error: exit_values = 5;
pub const indent_fatal: exit_values = 4;
pub const indent_punt: exit_values = 3;
pub const indent_error: exit_values = 2;
pub const invocation_error: exit_values = 64;
pub const total_success: exit_values = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_buffer {
    pub name: *mut libc::c_char,
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type file_buffer_ty = file_buffer;
pub type backup_mode_ty = libc::c_uint;
pub const numbered: backup_mode_ty = 4;
pub const numbered_existing: backup_mode_ty = 3;
pub const simple: backup_mode_ty = 2;
pub const none: backup_mode_ty = 1;
pub const unknown: backup_mode_ty = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct version_control_values_ty {
    pub value: backup_mode_ty,
    pub name: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut simple_backup_suffix: *mut libc::c_char = b"~\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut version_control: backup_mode_ty = unknown;
pub static mut version_width: libc::c_int = 1 as libc::c_int;
unsafe extern "C" fn simple_backup_name(
    mut pathname: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut backup_name: *mut libc::c_char = 0 as *mut libc::c_char;
    backup_name = xmalloc(
        (strlen(pathname))
            .wrapping_add(strlen(simple_backup_suffix))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_uint,
    ) as *mut libc::c_char;
    sprintf(
        backup_name,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        pathname,
        simple_backup_suffix,
    );
    return backup_name;
}
unsafe extern "C" fn version_number(
    mut base: *mut libc::c_char,
    mut direntry: *mut libc::c_char,
    mut base_length: libc::c_int,
) -> libc::c_int {
    let mut version: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    version = 0 as libc::c_int;
    if strncmp(base, direntry, base_length as libc::c_ulong) == 0
        && (*direntry.offset((base_length + 2 as libc::c_int) as isize) as libc::c_int
            & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc())
                .offset(
                    *direntry.offset((base_length + 2 as libc::c_int) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        p = &mut *direntry.offset((base_length + 2 as libc::c_int) as isize)
            as *mut libc::c_char;
        while *p as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            version = version * 10 as libc::c_int + *p as libc::c_int - '0' as i32;
            p = p.offset(1);
            p;
        }
        if *p.offset(0 as libc::c_int as isize) as libc::c_int != '~' as i32
            || *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            version = 0 as libc::c_int;
        }
    }
    return version;
}
unsafe extern "C" fn highest_version(
    mut filename: *mut libc::c_char,
    mut dirname: *mut libc::c_char,
) -> libc::c_int {
    let mut dirp: *mut DIR = opendir(dirname);
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut highestVersion: libc::c_int = 0;
    if dirp.is_null() {
        highestVersion = 0 as libc::c_int;
    } else {
        let mut this_version: libc::c_int = 0;
        let mut file_name_length: libc::c_uint = strlen(filename) as libc::c_uint;
        highestVersion = 0 as libc::c_int;
        loop {
            dp = readdir(dirp);
            if dp.is_null() {
                break;
            }
            if 1 as libc::c_int == 0
                || strlen(((*dp).d_name).as_mut_ptr())
                    <= file_name_length.wrapping_add(2 as libc::c_int as libc::c_uint)
                        as libc::c_ulong
            {
                continue;
            }
            this_version = version_number(
                filename,
                ((*dp).d_name).as_mut_ptr(),
                file_name_length as libc::c_int,
            );
            if this_version > highestVersion {
                highestVersion = this_version;
            }
        }
        closedir(dirp);
    }
    return highestVersion;
}
unsafe extern "C" fn max_version(mut pathname: *mut libc::c_char) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathlen: libc::c_int = strlen(pathname) as libc::c_int;
    let mut version: libc::c_int = 0;
    p = pathname.offset(pathlen as isize).offset(-(1 as libc::c_int as isize));
    while p > pathname && *p as libc::c_int != '/' as i32 {
        p = p.offset(-1);
        p;
    }
    if *p as libc::c_int == '/' as i32 {
        let mut dirlen: libc::c_int = p.offset_from(pathname) as libc::c_long
            as libc::c_int;
        let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
        filename = p.offset(1 as libc::c_int as isize);
        dirname = xmalloc((dirlen + 1 as libc::c_int) as libc::c_uint)
            as *mut libc::c_char;
        strncpy(dirname, pathname, dirlen as libc::c_ulong);
        *dirname.offset(dirlen as isize) = '\0' as i32 as libc::c_char;
        version = highest_version(filename, dirname);
        xfree(dirname as *mut libc::c_void);
    } else {
        filename = pathname;
        version = highest_version(
            filename,
            b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return version;
}
unsafe extern "C" fn generate_backup_filename(
    mut versionControl: backup_mode_ty,
    mut pathname: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut last_numbered_version: libc::c_int = 0;
    let mut backup_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if versionControl as libc::c_uint == none as libc::c_int as libc::c_uint {
        backup_name = 0 as *mut libc::c_char;
    } else if versionControl as libc::c_uint == simple as libc::c_int as libc::c_uint {
        backup_name = simple_backup_name(pathname);
    } else {
        last_numbered_version = max_version(pathname);
        if versionControl as libc::c_uint
            == numbered_existing as libc::c_int as libc::c_uint
            && last_numbered_version == 0 as libc::c_int
        {
            backup_name = simple_backup_name(pathname);
        } else {
            last_numbered_version += 1;
            last_numbered_version;
            backup_name = xmalloc(
                (strlen(pathname)).wrapping_add(16 as libc::c_int as libc::c_ulong)
                    as libc::c_uint,
            ) as *mut libc::c_char;
            if !backup_name.is_null() {
                sprintf(
                    backup_name,
                    b"%s.~%0*d~\0" as *const u8 as *const libc::c_char,
                    pathname,
                    version_width,
                    last_numbered_version,
                );
            }
        }
    }
    return backup_name;
}
static mut values: [version_control_values_ty; 8] = [
    {
        let mut init = version_control_values_ty {
            value: none,
            name: b"never\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = version_control_values_ty {
            value: none,
            name: b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = version_control_values_ty {
            value: simple,
            name: b"simple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = version_control_values_ty {
            value: numbered_existing,
            name: b"existing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = version_control_values_ty {
            value: numbered_existing,
            name: b"nil\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = version_control_values_ty {
            value: numbered,
            name: b"numbered\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = version_control_values_ty {
            value: numbered,
            name: b"t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = version_control_values_ty {
            value: unknown,
            name: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
pub unsafe extern "C" fn version_control_value() -> backup_mode_ty {
    let mut version: *mut libc::c_char = getenv(
        b"VERSION_CONTROL\0" as *const u8 as *const libc::c_char,
    );
    let mut v: *mut version_control_values_ty = 0 as *mut version_control_values_ty;
    let mut ret: backup_mode_ty = unknown;
    if version.is_null() || *version as libc::c_int == 0 as libc::c_int {
        ret = numbered_existing;
    } else {
        v = &mut *values.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut version_control_values_ty;
        while !((*v).name).is_null() {
            if strcmp(version, (*v).name) == 0 as libc::c_int {
                ret = (*v).value;
                break;
            } else {
                v = v.offset(1);
                v;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn set_version_width() {
    let mut v: *mut libc::c_char = getenv(
        b"VERSION_WIDTH\0" as *const u8 as *const libc::c_char,
    );
    if !v.is_null()
        && (*v as libc::c_int & !(0x7f as libc::c_int) == 0 as libc::c_int
            && *(*__ctype_b_loc()).offset(*v as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        version_width = atoi(v);
    }
    if version_width > 16 as libc::c_int {
        version_width = 16 as libc::c_int;
    }
}
pub unsafe extern "C" fn initialize_backups() {
    let mut v: *mut libc::c_char = getenv(
        b"SIMPLE_BACKUP_SUFFIX\0" as *const u8 as *const libc::c_char,
    );
    if !v.is_null() && *v as libc::c_int != 0 {
        simple_backup_suffix = v;
    }
    version_control = version_control_value();
    if version_control as libc::c_uint == unknown as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"indent:  Strange version-control value\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"indent:  Using numbered-existing\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        version_control = numbered_existing;
    }
    set_version_width();
}
pub unsafe extern "C" fn make_backup(
    mut file: *mut file_buffer_ty,
    mut file_stats: *const stat,
) {
    let mut bf: *mut FILE = 0 as *mut FILE;
    let mut backup_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_uint = 0;
    if !(version_control as libc::c_uint == none as libc::c_int as libc::c_uint) {
        backup_filename = generate_backup_filename(version_control, (*file).name);
        if backup_filename.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"indent: Can't make backup filename of %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            exit(system_error as libc::c_int);
        }
        bf = fopen(backup_filename, b"w\0" as *const u8 as *const libc::c_char);
        if bf.is_null() {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Can't open backup file %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                backup_filename,
            );
        }
        size = fwrite(
            (*file).data as *const libc::c_void,
            (*file).size,
            1 as libc::c_int as libc::c_ulong,
            bf,
        ) as libc::c_uint;
        if size != 1 as libc::c_int as libc::c_uint {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Can't write to backup file %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                backup_filename,
            );
        }
        fclose(bf);
        let mut buf: utimbuf = utimbuf { actime: 0, modtime: 0 };
        buf.actime = time(0 as *mut time_t);
        buf.modtime = (*file_stats).st_mtim.tv_sec;
        if utime(backup_filename, &mut buf) != 0 as libc::c_int {
            message(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Warning\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Can't preserve modification time on backup file %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                backup_filename,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
        xfree(backup_filename as *mut libc::c_void);
    }
}
