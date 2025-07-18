use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn ar_name(_: *const libc::c_char) -> libc::c_int;
    fn ar_member_date(_: *const libc::c_char) -> time_t;
    static mut command_count: libc::c_ulong;
    fn strcache_add_len(str: *const libc::c_char, len: size_t) -> *const libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn hash_init(
        ht: *mut hash_table,
        size: libc::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut *mut libc::c_void;
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert(ht: *mut hash_table, item: *const libc::c_void) -> *mut libc::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const libc::c_void,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(ht: *mut hash_table, free_items: libc::c_int);
    fn jhash_string(key: *const libc::c_uchar) -> libc::c_uint;
    static mut hash_deleted_item: *mut libc::c_void;
    static mut db_level: libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type time_t = __time_t;
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
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: *const libc::c_char,
    pub counter: libc::c_ulong,
    pub contents: *mut directory_contents,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory_contents {
    pub dev: dev_t,
    pub ino: ino_t,
    pub dirfiles: hash_table,
    pub counter: libc::c_ulong,
    pub dirstream: *mut DIR,
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: libc::c_ulong,
    pub ht_capacity: libc::c_ulong,
    pub ht_fill: libc::c_ulong,
    pub ht_empty_slots: libc::c_ulong,
    pub ht_collisions: libc::c_ulong,
    pub ht_lookups: libc::c_ulong,
    pub ht_rehashes: libc::c_uint,
}
pub type hash_cmp_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirfile {
    pub name: *const libc::c_char,
    pub length: size_t,
    pub impossible: libc::c_short,
    pub type_0: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirstream {
    pub contents: *mut directory_contents,
    pub dirfile_slot: *mut *mut dirfile,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
static mut open_directories: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe extern "C" fn clear_directory_contents(
    mut dc: *mut directory_contents,
) -> *mut directory_contents {
    (*dc).counter = 0 as libc::c_int as libc::c_ulong;
    if !((*dc).dirstream).is_null() {
        open_directories = open_directories.wrapping_sub(1);
        open_directories;
        closedir((*dc).dirstream);
        (*dc).dirstream = 0 as *mut DIR;
    }
    if !((*dc).dirfiles.ht_vec).is_null() {
        hash_free(&mut (*dc).dirfiles, 1 as libc::c_int);
    }
    return 0 as *mut directory_contents;
}
unsafe extern "C" fn directory_contents_hash_1(
    mut key_0: *const libc::c_void,
) -> libc::c_ulong {
    let mut key: *const directory_contents = key_0 as *const directory_contents;
    let mut hash: libc::c_ulong = 0;
    hash = (((*key).dev as libc::c_uint) << 4 as libc::c_int
        ^ (*key).ino as libc::c_uint) as libc::c_ulong;
    return hash;
}
unsafe extern "C" fn directory_contents_hash_2(
    mut key_0: *const libc::c_void,
) -> libc::c_ulong {
    let mut key: *const directory_contents = key_0 as *const directory_contents;
    let mut hash: libc::c_ulong = 0;
    hash = (((*key).dev as libc::c_uint) << 4 as libc::c_int
        ^ !(*key).ino as libc::c_uint) as libc::c_ulong;
    return hash;
}
unsafe extern "C" fn directory_contents_hash_cmp(
    mut xv: *const libc::c_void,
    mut yv: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const directory_contents = xv as *const directory_contents;
    let mut y: *const directory_contents = yv as *const directory_contents;
    let mut result: libc::c_int = 0;
    result = if (*x).ino < (*y).ino {
        -(1 as libc::c_int)
    } else if (*x).ino == (*y).ino {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if result != 0 {
        return result;
    }
    return if (*x).dev < (*y).dev {
        -(1 as libc::c_int)
    } else if (*x).dev == (*y).dev {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
static mut directory_contents: hash_table = hash_table {
    ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
};
unsafe extern "C" fn directory_hash_1(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = (*(key as *const directory)).name
        as *const libc::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn directory_hash_2(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    return _result_;
}
unsafe extern "C" fn directory_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    return if (*(x as *const directory)).name == (*(y as *const directory)).name {
        0 as libc::c_int
    } else {
        strcmp((*(x as *const directory)).name, (*(y as *const directory)).name)
    };
}
static mut directories: hash_table = hash_table {
    ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    ht_hash_1: None,
    ht_hash_2: None,
    ht_compare: None,
    ht_size: 0,
    ht_capacity: 0,
    ht_fill: 0,
    ht_empty_slots: 0,
    ht_collisions: 0,
    ht_lookups: 0,
    ht_rehashes: 0,
};
unsafe extern "C" fn dirfile_hash_1(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = (*(key as *const dirfile)).name
        as *const libc::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn dirfile_hash_2(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    return _result_;
}
unsafe extern "C" fn dirfile_hash_cmp(
    mut xv: *const libc::c_void,
    mut yv: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const dirfile = xv as *const dirfile;
    let mut y: *const dirfile = yv as *const dirfile;
    let mut result: libc::c_int = ((*x).length).wrapping_sub((*y).length) as libc::c_int;
    if result != 0 {
        return result;
    }
    return if (*x).name == (*y).name {
        0 as libc::c_int
    } else {
        strcmp((*x).name, (*y).name)
    };
}
unsafe extern "C" fn find_directory(mut name: *const libc::c_char) -> *mut directory {
    let mut dir: *mut directory = 0 as *mut directory;
    let mut dir_slot: *mut *mut directory = 0 as *mut *mut directory;
    let mut dir_key: directory = directory {
        name: 0 as *const libc::c_char,
        counter: 0,
        contents: 0 as *mut directory_contents,
    };
    let mut dc: *mut directory_contents = 0 as *mut directory_contents;
    let mut dc_slot: *mut *mut directory_contents = 0 as *mut *mut directory_contents;
    let mut dc_key: directory_contents = directory_contents {
        dev: 0,
        ino: 0,
        dirfiles: hash_table {
            ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
            ht_hash_1: None,
            ht_hash_2: None,
            ht_compare: None,
            ht_size: 0,
            ht_capacity: 0,
            ht_fill: 0,
            ht_empty_slots: 0,
            ht_collisions: 0,
            ht_lookups: 0,
            ht_rehashes: 0,
        },
        counter: 0,
        dirstream: 0 as *mut DIR,
    };
    let mut st: stat = stat {
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
    let mut r: libc::c_int = 0;
    dir_key.name = name;
    dir_slot = hash_find_slot(
        &mut directories,
        &mut dir_key as *mut directory as *const libc::c_void,
    ) as *mut *mut directory;
    dir = *dir_slot;
    if !(dir.is_null() || dir as *mut libc::c_void == hash_deleted_item) {
        let mut ctr: libc::c_ulong = if !((*dir).contents).is_null() {
            (*(*dir).contents).counter
        } else {
            (*dir).counter
        };
        if ctr == command_count {
            return dir;
        }
        if 0x2 as libc::c_int & db_level != 0 {
            printf(
                b"Directory %s cache invalidated (count %lu != command %lu)\n\0"
                    as *const u8 as *const libc::c_char,
                name,
                ctr,
                command_count,
            );
            fflush(stdout);
        }
        if !((*dir).contents).is_null() {
            clear_directory_contents((*dir).contents);
        }
    } else {
        let mut len: size_t = strlen(name);
        dir = xmalloc(::std::mem::size_of::<directory>() as libc::c_ulong)
            as *mut directory;
        (*dir).name = strcache_add_len(name, len);
        hash_insert_at(
            &mut directories,
            dir as *const libc::c_void,
            dir_slot as *const libc::c_void,
        );
    }
    (*dir).contents = 0 as *mut directory_contents;
    (*dir).counter = command_count;
    loop {
        r = stat(name, &mut st);
        if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if r < 0 as libc::c_int {
        return dir;
    }
    memset(
        &mut dc_key as *mut directory_contents as *mut libc::c_void,
        '\0' as i32,
        ::std::mem::size_of::<directory_contents>() as libc::c_ulong,
    );
    dc_key.dev = st.st_dev;
    dc_key.ino = st.st_ino;
    dc_slot = hash_find_slot(
        &mut directory_contents,
        &mut dc_key as *mut directory_contents as *const libc::c_void,
    ) as *mut *mut directory_contents;
    dc = *dc_slot;
    if dc.is_null() || dc as *mut libc::c_void == hash_deleted_item {
        dc = xcalloc(::std::mem::size_of::<directory_contents>() as libc::c_ulong)
            as *mut directory_contents;
        *dc = dc_key;
        hash_insert_at(
            &mut directory_contents,
            dc as *const libc::c_void,
            dc_slot as *const libc::c_void,
        );
    }
    (*dir).contents = dc;
    if (*dc).counter != command_count {
        if (*dc).counter != 0 {
            clear_directory_contents(dc);
        }
        (*dc).counter = command_count;
        loop {
            *__errno_location() = 0 as libc::c_int;
            (*dc).dirstream = opendir(name);
            if !(((*dc).dirstream).is_null() && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if ((*dc).dirstream).is_null() {
            (*dc).dirfiles.ht_vec = 0 as *mut *mut libc::c_void;
        } else {
            hash_init(
                &mut (*dc).dirfiles,
                107 as libc::c_int as libc::c_ulong,
                Some(
                    dirfile_hash_1
                        as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
                ),
                Some(
                    dirfile_hash_2
                        as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
                ),
                Some(
                    dirfile_hash_cmp
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            open_directories = open_directories.wrapping_add(1);
            open_directories;
            if open_directories == 10 as libc::c_int as libc::c_uint {
                dir_contents_file_exists_p(dir, 0 as *const libc::c_char);
            }
        }
    }
    return dir;
}
unsafe extern "C" fn dir_contents_file_exists_p(
    mut dir: *mut directory,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut df: *mut dirfile = 0 as *mut dirfile;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut dc: *mut directory_contents = (*dir).contents;
    if dc.is_null() || ((*dc).dirfiles.ht_vec).is_null() {
        return 0 as libc::c_int;
    }
    if !filename.is_null() {
        let mut dirfile_key: dirfile = dirfile {
            name: 0 as *const libc::c_char,
            length: 0,
            impossible: 0,
            type_0: 0,
        };
        if *filename as libc::c_int == '\0' as i32 {
            return 1 as libc::c_int;
        }
        dirfile_key.name = filename;
        dirfile_key.length = strlen(filename);
        df = hash_find_item(
            &mut (*dc).dirfiles,
            &mut dirfile_key as *mut dirfile as *const libc::c_void,
        ) as *mut dirfile;
        if !df.is_null() {
            return ((*df).impossible == 0) as libc::c_int;
        }
    }
    if ((*dc).dirstream).is_null() {
        return 0 as libc::c_int;
    }
    loop {
        let mut len: size_t = 0;
        let mut dirfile_key_0: dirfile = dirfile {
            name: 0 as *const libc::c_char,
            length: 0,
            impossible: 0,
            type_0: 0,
        };
        let mut dirfile_slot: *mut *mut dirfile = 0 as *mut *mut dirfile;
        loop {
            *__errno_location() = 0 as libc::c_int;
            d = readdir((*dc).dirstream);
            if !(d.is_null() && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if d.is_null() {
            if *__errno_location() != 0 {
                fatal(
                    0 as *mut floc,
                    (strlen((*dir).name))
                        .wrapping_add(strlen(strerror(*__errno_location()))),
                    b"readdir %s: %s\0" as *const u8 as *const libc::c_char,
                    (*dir).name,
                    strerror(*__errno_location()),
                );
            }
            break;
        } else {
            if !((*d).d_ino != 0 as libc::c_int as libc::c_ulong) {
                continue;
            }
            len = strlen(((*d).d_name).as_mut_ptr());
            dirfile_key_0.name = ((*d).d_name).as_mut_ptr();
            dirfile_key_0.length = len;
            dirfile_slot = hash_find_slot(
                &mut (*dc).dirfiles,
                &mut dirfile_key_0 as *mut dirfile as *const libc::c_void,
            ) as *mut *mut dirfile;
            df = xmalloc(::std::mem::size_of::<dirfile>() as libc::c_ulong)
                as *mut dirfile;
            (*df).name = strcache_add_len(((*d).d_name).as_mut_ptr(), len);
            (*df).type_0 = (*d).d_type;
            (*df).length = len;
            (*df).impossible = 0 as libc::c_int as libc::c_short;
            hash_insert_at(
                &mut (*dc).dirfiles,
                df as *const libc::c_void,
                dirfile_slot as *const libc::c_void,
            );
            if !filename.is_null()
                && (((*d).d_name).as_mut_ptr() == filename as *mut libc::c_char
                    || *((*d).d_name).as_mut_ptr() as libc::c_int
                        == *filename as libc::c_int
                        && (*((*d).d_name).as_mut_ptr() as libc::c_int == '\0' as i32
                            || strcmp(
                                ((*d).d_name)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize),
                                filename.offset(1 as libc::c_int as isize),
                            ) == 0))
            {
                return 1 as libc::c_int;
            }
        }
    }
    if d.is_null() {
        open_directories = open_directories.wrapping_sub(1);
        open_directories;
        closedir((*dc).dirstream);
        (*dc).dirstream = 0 as *mut DIR;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dir_file_exists_p(
    mut dirname: *const libc::c_char,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    return dir_contents_file_exists_p(find_directory(dirname), filename);
}
pub unsafe extern "C" fn file_exists_p(mut name: *const libc::c_char) -> libc::c_int {
    let mut dirend: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirname: *const libc::c_char = 0 as *const libc::c_char;
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    if ar_name(name) != 0 {
        return (ar_member_date(name) != -(1 as libc::c_int) as time_t) as libc::c_int;
    }
    dirend = strrchr(name, '/' as i32);
    if dirend.is_null() {
        return dir_file_exists_p(b".\0" as *const u8 as *const libc::c_char, name);
    }
    slash = dirend;
    if dirend == name {
        dirname = b"/\0" as *const u8 as *const libc::c_char;
    } else {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (dirend.offset_from(name) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong as usize,
        );
        p = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(
            p as *mut libc::c_void,
            name as *const libc::c_void,
            dirend.offset_from(name) as libc::c_long as libc::c_ulong,
        );
        *p
            .offset(
                dirend.offset_from(name) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        dirname = p;
    }
    slash = slash.offset(1);
    slash;
    return dir_file_exists_p(dirname, slash);
}
pub unsafe extern "C" fn file_impossible(mut filename: *const libc::c_char) {
    let mut dirend: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = filename;
    let mut dir: *mut directory = 0 as *mut directory;
    let mut new: *mut dirfile = 0 as *mut dirfile;
    dirend = strrchr(p, '/' as i32);
    if dirend.is_null() {
        dir = find_directory(b".\0" as *const u8 as *const libc::c_char);
    } else {
        let mut dirname: *const libc::c_char = 0 as *const libc::c_char;
        let mut slash: *const libc::c_char = dirend;
        if dirend == p {
            dirname = b"/\0" as *const u8 as *const libc::c_char;
        } else {
            let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut fresh1 = ::std::vec::from_elem(
                0,
                (dirend.offset_from(p) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong as usize,
            );
            cp = fresh1.leak().as_mut_ptr() as *mut libc::c_char;
            memcpy(
                cp as *mut libc::c_void,
                p as *const libc::c_void,
                dirend.offset_from(p) as libc::c_long as libc::c_ulong,
            );
            *cp
                .offset(
                    dirend.offset_from(p) as libc::c_long as isize,
                ) = '\0' as i32 as libc::c_char;
            dirname = cp;
        }
        dir = find_directory(dirname);
        p = slash.offset(1 as libc::c_int as isize);
        filename = p;
    }
    if ((*dir).contents).is_null() {
        (*dir)
            .contents = xcalloc(
            ::std::mem::size_of::<directory_contents>() as libc::c_ulong,
        ) as *mut directory_contents;
    }
    if ((*(*dir).contents).dirfiles.ht_vec).is_null() {
        hash_init(
            &mut (*(*dir).contents).dirfiles,
            107 as libc::c_int as libc::c_ulong,
            Some(
                dirfile_hash_1
                    as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
            ),
            Some(
                dirfile_hash_2
                    as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
            ),
            Some(
                dirfile_hash_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
    new = xmalloc(::std::mem::size_of::<dirfile>() as libc::c_ulong) as *mut dirfile;
    (*new).length = strlen(filename);
    (*new).name = strcache_add_len(filename, (*new).length);
    (*new).impossible = 1 as libc::c_int as libc::c_short;
    hash_insert(&mut (*(*dir).contents).dirfiles, new as *const libc::c_void);
}
pub unsafe extern "C" fn file_impossible_p(
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut dirend: *const libc::c_char = 0 as *const libc::c_char;
    let mut dir: *mut directory_contents = 0 as *mut directory_contents;
    let mut dirfile: *mut dirfile = 0 as *mut dirfile;
    let mut dirfile_key: dirfile = dirfile {
        name: 0 as *const libc::c_char,
        length: 0,
        impossible: 0,
        type_0: 0,
    };
    dirend = strrchr(filename, '/' as i32);
    if dirend.is_null() {
        dir = (*find_directory(b".\0" as *const u8 as *const libc::c_char)).contents;
    } else {
        let mut dirname: *const libc::c_char = 0 as *const libc::c_char;
        let mut slash: *const libc::c_char = dirend;
        if dirend == filename {
            dirname = b"/\0" as *const u8 as *const libc::c_char;
        } else {
            let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut fresh2 = ::std::vec::from_elem(
                0,
                (dirend.offset_from(filename) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as libc::c_ulong as usize,
            );
            cp = fresh2.leak().as_mut_ptr() as *mut libc::c_char;
            memcpy(
                cp as *mut libc::c_void,
                filename as *const libc::c_void,
                dirend.offset_from(filename) as libc::c_long as libc::c_ulong,
            );
            *cp
                .offset(
                    dirend.offset_from(filename) as libc::c_long as isize,
                ) = '\0' as i32 as libc::c_char;
            dirname = cp;
        }
        dir = (*find_directory(dirname)).contents;
        filename = slash.offset(1 as libc::c_int as isize);
    }
    if dir.is_null() || ((*dir).dirfiles.ht_vec).is_null() {
        return 0 as libc::c_int;
    }
    dirfile_key.name = filename;
    dirfile_key.length = strlen(filename);
    dirfile = hash_find_item(
        &mut (*dir).dirfiles,
        &mut dirfile_key as *mut dirfile as *const libc::c_void,
    ) as *mut dirfile;
    if !dirfile.is_null() {
        return (*dirfile).impossible as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dir_name(mut dir: *const libc::c_char) -> *const libc::c_char {
    return (*find_directory(dir)).name;
}
pub unsafe extern "C" fn print_dir_data_base() {
    let mut files: libc::c_uint = 0;
    let mut impossible: libc::c_uint = 0;
    let mut dir_slot: *mut *mut directory = 0 as *mut *mut directory;
    let mut dir_end: *mut *mut directory = 0 as *mut *mut directory;
    puts(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# Directories\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    impossible = 0 as libc::c_int as libc::c_uint;
    files = impossible;
    dir_slot = directories.ht_vec as *mut *mut directory;
    dir_end = dir_slot.offset(directories.ht_size as isize);
    while dir_slot < dir_end {
        let mut dir: *mut directory = *dir_slot;
        if !(dir.is_null() || dir as *mut libc::c_void == hash_deleted_item) {
            if ((*dir).contents).is_null() {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"# %s: could not be stat'd.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*dir).name,
                );
            } else if ((*(*dir).contents).dirfiles.ht_vec).is_null() {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"# %s (device %ld, inode %ld): could not be opened.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*dir).name,
                    (*(*dir).contents).dev as libc::c_long,
                    (*(*dir).contents).ino as libc::c_long,
                );
            } else {
                let mut f: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut im: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut files_slot: *mut *mut dirfile = 0 as *mut *mut dirfile;
                let mut files_end: *mut *mut dirfile = 0 as *mut *mut dirfile;
                files_slot = (*(*dir).contents).dirfiles.ht_vec as *mut *mut dirfile;
                files_end = files_slot
                    .offset((*(*dir).contents).dirfiles.ht_size as isize);
                while files_slot < files_end {
                    let mut df: *mut dirfile = *files_slot;
                    if !(df.is_null() || df as *mut libc::c_void == hash_deleted_item) {
                        if (*df).impossible != 0 {
                            im = im.wrapping_add(1);
                            im;
                        } else {
                            f = f.wrapping_add(1);
                            f;
                        }
                    }
                    files_slot = files_slot.offset(1);
                    files_slot;
                }
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"# %s (device %ld, inode %ld): \0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*dir).name,
                    (*(*dir).contents).dev as libc::c_long,
                    (*(*dir).contents).ino as libc::c_long,
                );
                if f == 0 as libc::c_int as libc::c_uint {
                    fputs(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"No\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        stdout,
                    );
                } else {
                    printf(b"%u\0" as *const u8 as *const libc::c_char, f);
                }
                fputs(
                    dcgettext(
                        0 as *const libc::c_char,
                        b" files, \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    stdout,
                );
                if im == 0 as libc::c_int as libc::c_uint {
                    fputs(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        stdout,
                    );
                } else {
                    printf(b"%u\0" as *const u8 as *const libc::c_char, im);
                }
                fputs(
                    dcgettext(
                        0 as *const libc::c_char,
                        b" impossibilities\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    stdout,
                );
                if ((*(*dir).contents).dirstream).is_null() {
                    puts(b".\0" as *const u8 as *const libc::c_char);
                } else {
                    puts(
                        dcgettext(
                            0 as *const libc::c_char,
                            b" so far.\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                files = files.wrapping_add(f);
                impossible = impossible.wrapping_add(im);
            }
        }
        dir_slot = dir_slot.offset(1);
        dir_slot;
    }
    fputs(b"\n# \0" as *const u8 as *const libc::c_char, stdout);
    if files == 0 as libc::c_int as libc::c_uint {
        fputs(
            dcgettext(
                0 as *const libc::c_char,
                b"No\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
    } else {
        printf(b"%u\0" as *const u8 as *const libc::c_char, files);
    }
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b" files, \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    if impossible == 0 as libc::c_int as libc::c_uint {
        fputs(
            dcgettext(
                0 as *const libc::c_char,
                b"no\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
    } else {
        printf(b"%u\0" as *const u8 as *const libc::c_char, impossible);
    }
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b" impossibilities in %lu directories.\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        directories.ht_fill,
    );
}
unsafe extern "C" fn open_dirstream(
    mut directory: *const libc::c_char,
) -> *mut libc::c_void {
    let mut new: *mut dirstream = 0 as *mut dirstream;
    let mut dir: *mut directory = find_directory(directory);
    if ((*dir).contents).is_null() || ((*(*dir).contents).dirfiles.ht_vec).is_null() {
        return 0 as *mut libc::c_void;
    }
    dir_contents_file_exists_p(dir, 0 as *const libc::c_char);
    new = xmalloc(::std::mem::size_of::<dirstream>() as libc::c_ulong) as *mut dirstream;
    (*new).contents = (*dir).contents;
    (*new).dirfile_slot = (*(*new).contents).dirfiles.ht_vec as *mut *mut dirfile;
    return new as *mut libc::c_void;
}
unsafe extern "C" fn read_dirstream(mut stream: *mut libc::c_void) -> *mut dirent {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: size_t = 0;
    let ds: *mut dirstream = stream as *mut dirstream;
    let mut dc: *mut directory_contents = (*ds).contents;
    let mut dirfile_end: *mut *mut dirfile = ((*dc).dirfiles.ht_vec as *mut *mut dirfile)
        .offset((*dc).dirfiles.ht_size as isize);
    while (*ds).dirfile_slot < dirfile_end {
        let fresh3 = (*ds).dirfile_slot;
        (*ds).dirfile_slot = ((*ds).dirfile_slot).offset(1);
        let mut df: *mut dirfile = *fresh3;
        if !(df.is_null() || df as *mut libc::c_void == hash_deleted_item)
            && (*df).impossible == 0
        {
            let mut d: *mut dirent = 0 as *mut dirent;
            let mut len: size_t = ((*df).length)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut sz: size_t = (::std::mem::size_of::<dirent>() as libc::c_ulong)
                .wrapping_sub(
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                )
                .wrapping_add(len);
            if sz > bufsz {
                bufsz = (bufsz as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                if sz > bufsz {
                    bufsz = sz;
                }
                buf = xrealloc(buf as *mut libc::c_void, bufsz) as *mut libc::c_char;
            }
            d = buf as *mut dirent;
            (*d).d_ino = 1 as libc::c_int as __ino_t;
            (*d).d_type = (*df).type_0;
            memcpy(
                ((*d).d_name).as_mut_ptr() as *mut libc::c_void,
                (*df).name as *const libc::c_void,
                len,
            );
            return d;
        }
    }
    return 0 as *mut dirent;
}
pub unsafe extern "C" fn dir_setup_glob(mut gl: *mut glob_t) {
    (*gl).gl_offs = 0 as libc::c_int as __size_t;
    (*gl)
        .gl_opendir = Some(
        open_dirstream as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    );
    (*gl)
        .gl_readdir = Some(
        read_dirstream as unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent,
    );
    (*gl).gl_closedir = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*gl)
        .gl_lstat = Some(
        lstat as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    );
    (*gl)
        .gl_stat = Some(
        stat as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    );
}
pub unsafe extern "C" fn hash_init_directories() {
    hash_init(
        &mut directories,
        199 as libc::c_int as libc::c_ulong,
        Some(
            directory_hash_1
                as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            directory_hash_2
                as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            directory_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    hash_init(
        &mut directory_contents,
        199 as libc::c_int as libc::c_ulong,
        Some(
            directory_contents_hash_1
                as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            directory_contents_hash_2
                as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            directory_contents_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
