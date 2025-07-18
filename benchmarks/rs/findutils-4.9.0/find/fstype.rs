use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn read_file_system_list(need_fs_type: bool) -> *mut mount_entry;
    fn free_mount_entry(entry: *mut mount_entry);
    fn set_stat_placeholders(p: *mut stat);
    static mut options: options;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn extendbuf(
        existing: *mut libc::c_void,
        wanted: size_t,
        allocated: *mut size_t,
    ) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type dev_t = __dev_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mount_entry {
    pub me_devname: *mut libc::c_char,
    pub me_mountdir: *mut libc::c_char,
    pub me_mntroot: *mut libc::c_char,
    pub me_type: *mut libc::c_char,
    pub me_dev: dev_t,
    #[bitfield(name = "me_dummy", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "me_remote", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "me_type_malloced", ty = "libc::c_uint", bits = "2..=2")]
    pub me_dummy_me_remote_me_type_malloced: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub me_next: *mut mount_entry,
}
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub do_dir_first: bool,
    pub explicit_depth: bool,
    pub maxdepth: libc::c_int,
    pub mindepth: libc::c_int,
    pub no_leaf_check: bool,
    pub stay_on_filesystem: bool,
    pub ignore_readdir_race: bool,
    pub literal_control_chars: bool,
    pub warnings: bool,
    pub posixly_correct: bool,
    pub start_time: timespec,
    pub cur_day_start: timespec,
    pub full_days: bool,
    pub output_block_size: libc::c_int,
    pub debug_options: libc::c_ulong,
    pub symlink_handling: SymlinkOption,
    pub xstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub open_nofollow_available: bool,
    pub regex_options: libc::c_int,
    pub x_getfilecon: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub optimisation_level: libc::c_ushort,
    pub err_quoting_style: quoting_style,
    pub files0_from: *const libc::c_char,
    pub ok_prompt_stdin: bool,
}
pub type SymlinkOption = libc::c_uint;
pub const SYMLINK_DEREF_ARGSONLY: SymlinkOption = 2;
pub const SYMLINK_ALWAYS_DEREF: SymlinkOption = 1;
pub const SYMLINK_NEVER_DEREF: SymlinkOption = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: libc::c_int,
}
unsafe extern "C" fn free_file_system_list(mut p: *mut mount_entry) {
    while !p.is_null() {
        let mut pnext: *mut mount_entry = (*p).me_next;
        free_mount_entry(p);
        p = pnext;
    }
}
unsafe extern "C" fn get_file_system_list(mut need_fs_type: bool) -> *mut mount_entry {
    static mut mount_list: *mut mount_entry = 0 as *const mount_entry
        as *mut mount_entry;
    static mut has_fstype: bool = 0 as libc::c_int != 0;
    if !mount_list.is_null() && !has_fstype && need_fs_type as libc::c_int != 0 {
        free_file_system_list(mount_list);
        mount_list = 0 as *mut mount_entry;
    }
    if mount_list.is_null() {
        mount_list = read_file_system_list(need_fs_type);
        has_fstype = need_fs_type;
    }
    return mount_list;
}
pub unsafe extern "C" fn filesystem_type(
    mut statp: *const stat,
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    static mut fstype_known: bool = 0 as libc::c_int != 0;
    static mut current_fstype: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut current_dev: dev_t = 0;
    if !current_fstype.is_null() {
        if fstype_known as libc::c_int != 0 && (*statp).st_dev == current_dev {
            return current_fstype;
        }
        rpl_free(current_fstype as *mut libc::c_void);
    }
    current_dev = (*statp).st_dev;
    current_fstype = file_system_type_uncached(statp, path, &mut fstype_known);
    return current_fstype;
}
pub unsafe extern "C" fn is_used_fs_type(mut name: *const libc::c_char) -> bool {
    if 0 as libc::c_int == strcmp(b"afs\0" as *const u8 as *const libc::c_char, name) {
        return 1 as libc::c_int != 0
    } else {
        let mut entries: *const mount_entry = get_file_system_list(
            0 as libc::c_int != 0,
        );
        if !entries.is_null() {
            let mut entry: *const mount_entry = 0 as *const mount_entry;
            entry = entries;
            while !entry.is_null() {
                if 0 as libc::c_int == strcmp(name, (*entry).me_type) {
                    return 1 as libc::c_int != 0;
                }
                entry = (*entry).me_next;
            }
        } else {
            return 1 as libc::c_int != 0
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn set_fstype_devno(mut p: *mut mount_entry) -> libc::c_int {
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
    if (*p).me_dev == -(1 as libc::c_int) as dev_t {
        set_stat_placeholders(&mut stbuf);
        if 0 as libc::c_int == (options.xstat).unwrap()((*p).me_mountdir, &mut stbuf) {
            (*p).me_dev = stbuf.st_dev;
            return 0 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_system_type_uncached(
    mut statp: *const stat,
    mut path: *const libc::c_char,
    mut fstype_known: *mut bool,
) -> *mut libc::c_char {
    let mut entries: *mut mount_entry = 0 as *mut mount_entry;
    let mut entry: *mut mount_entry = 0 as *mut mount_entry;
    let mut best: *mut mount_entry = 0 as *mut mount_entry;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    best = 0 as *mut mount_entry;
    entries = get_file_system_list(1 as libc::c_int != 0);
    if entries.is_null() {
        if ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot read mounted file system list\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot read mounted file system list\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            if 0 as libc::c_int != 0 {} else {
                unreachable!();
            };
        };
    }
    type_0 = 0 as *mut libc::c_char;
    entry = entries;
    while !entry.is_null() {
        if !(strcmp((*entry).me_type, b"ignore\0" as *const u8 as *const libc::c_char)
            == 0)
        {
            if 0 as libc::c_int == set_fstype_devno(entry) {
                if (*entry).me_dev == (*statp).st_dev {
                    best = entry;
                }
            }
        }
        entry = (*entry).me_next;
    }
    if !best.is_null() {
        type_0 = xstrdup((*best).me_type);
    }
    *fstype_known = !type_0.is_null();
    return if !type_0.is_null() {
        type_0
    } else {
        xstrdup(
            dcgettext(
                0 as *const libc::c_char,
                b"unknown\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        )
    };
}
pub unsafe extern "C" fn get_mounted_devices(mut n: *mut size_t) -> *mut dev_t {
    let mut alloc_size: size_t = 0 as libc::c_uint as size_t;
    let mut used: size_t = 0 as libc::c_uint as size_t;
    let mut entries: *mut mount_entry = 0 as *mut mount_entry;
    let mut entry: *mut mount_entry = 0 as *mut mount_entry;
    let mut result: *mut dev_t = 0 as *mut dev_t;
    entries = read_file_system_list(0 as libc::c_int != 0);
    entry = entries;
    while !entry.is_null() {
        let mut p: *mut libc::c_void = extendbuf(
            result as *mut libc::c_void,
            (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                .wrapping_mul(used.wrapping_add(1 as libc::c_int as libc::c_ulong)),
            &mut alloc_size,
        );
        if !p.is_null() {
            result = p as *mut dev_t;
            if 0 as libc::c_int == set_fstype_devno(entry) {
                *result.offset(used as isize) = (*entry).me_dev;
                used = used.wrapping_add(1);
                used;
            }
        } else {
            rpl_free(result as *mut libc::c_void);
            result = 0 as *mut dev_t;
        }
        entry = (*entry).me_next;
    }
    free_file_system_list(entries);
    if !result.is_null() {
        *n = used;
    }
    return result;
}
