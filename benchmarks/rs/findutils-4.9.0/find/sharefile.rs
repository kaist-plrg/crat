use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    pub type Hash_tuning_0;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn set_cloexec_flag(desc: libc::c_int, value: bool) -> libc::c_int;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_free(table: *mut Hash_table);
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fatal_nontarget_file_error(
        errno_value: libc::c_int,
        name: *const libc::c_char,
    ) -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type sharefile_handle = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sharefile {
    pub mode: *mut libc::c_char,
    pub table: *mut Hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharefileEntry {
    pub device: dev_t,
    pub inode: ino_t,
    pub name: *mut libc::c_char,
    pub fp: *mut FILE,
}
pub const DefaultHashTableSize: C2RustUnnamed = 11;
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
unsafe extern "C" fn entry_comparator(
    mut av: *const libc::c_void,
    mut bv: *const libc::c_void,
) -> bool {
    let mut a: *const SharefileEntry = av as *const SharefileEntry;
    let mut b: *const SharefileEntry = bv as *const SharefileEntry;
    return (*a).inode == (*b).inode && (*a).device == (*b).device;
}
unsafe extern "C" fn entry_free(mut pv: *mut libc::c_void) {
    let mut p: *mut SharefileEntry = pv as *mut SharefileEntry;
    if !((*p).fp).is_null() {
        if 0 as libc::c_int != rpl_fclose((*p).fp) {
            fatal_nontarget_file_error(*__errno_location(), (*p).name);
        }
    }
    rpl_free((*p).name as *mut libc::c_void);
    rpl_free(p as *mut libc::c_void);
}
unsafe extern "C" fn entry_hashfunc(
    mut pv: *const libc::c_void,
    mut buckets: size_t,
) -> size_t {
    let mut p: *const SharefileEntry = pv as *const SharefileEntry;
    return ((*p).device ^ (*p).inode).wrapping_rem(buckets);
}
pub unsafe extern "C" fn sharefile_init(
    mut mode: *const libc::c_char,
) -> sharefile_handle {
    let mut p: *mut sharefile = malloc(
        ::std::mem::size_of::<sharefile>() as libc::c_ulong,
    ) as *mut sharefile;
    if !p.is_null() {
        (*p).mode = strdup(mode);
        if !((*p).mode).is_null() {
            (*p)
                .table = hash_initialize(
                DefaultHashTableSize as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    entry_hashfunc
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    entry_comparator
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                Some(entry_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            if !((*p).table).is_null() {
                return p as sharefile_handle
            } else {
                rpl_free((*p).mode as *mut libc::c_void);
                rpl_free(p as *mut libc::c_void);
            }
        } else {
            rpl_free(p as *mut libc::c_void);
        }
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn sharefile_destroy(mut pv: sharefile_handle) {
    let mut p: *mut sharefile = pv as *mut sharefile;
    rpl_free((*p).mode as *mut libc::c_void);
    hash_free((*p).table);
}
pub unsafe extern "C" fn sharefile_fopen(
    mut h: sharefile_handle,
    mut filename: *const libc::c_char,
) -> *mut FILE {
    let mut p: *mut sharefile = h as *mut sharefile;
    let mut new_entry: *mut SharefileEntry = 0 as *mut SharefileEntry;
    new_entry = malloc(::std::mem::size_of::<SharefileEntry>() as libc::c_ulong)
        as *mut SharefileEntry;
    if new_entry.is_null() {
        return 0 as *mut FILE;
    }
    (*new_entry).name = strdup(filename);
    if ((*new_entry).name).is_null() {
        rpl_free(new_entry as *mut libc::c_void);
        return 0 as *mut FILE;
    }
    (*new_entry).fp = fopen_safer(filename, (*p).mode);
    if ((*new_entry).fp).is_null() {
        entry_free(new_entry as *mut libc::c_void);
        return 0 as *mut FILE;
    } else {
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
        let fd: libc::c_int = fileno((*new_entry).fp);
        if fd >= 0 as libc::c_int {} else {
            __assert_fail(
                b"fd >= 0\0" as *const u8 as *const libc::c_char,
                b"sharefile.c\0" as *const u8 as *const libc::c_char,
                166 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"FILE *sharefile_fopen(sharefile_handle, const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_3793: {
            if fd >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"fd >= 0\0" as *const u8 as *const libc::c_char,
                    b"sharefile.c\0" as *const u8 as *const libc::c_char,
                    166 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 54],
                        &[libc::c_char; 54],
                    >(b"FILE *sharefile_fopen(sharefile_handle, const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        set_cloexec_flag(fd, 1 as libc::c_int != 0);
        if fstat(fd, &mut st) < 0 as libc::c_int {
            entry_free(new_entry as *mut libc::c_void);
            return 0 as *mut FILE;
        } else {
            let mut existing: *mut libc::c_void = 0 as *mut libc::c_void;
            (*new_entry).device = st.st_dev;
            (*new_entry).inode = st.st_ino;
            existing = hash_lookup((*p).table, new_entry as *const libc::c_void);
            if !existing.is_null() {
                entry_free(new_entry as *mut libc::c_void);
                return (*(existing as *const SharefileEntry)).fp;
            } else if !(hash_insert((*p).table, new_entry as *const libc::c_void))
                .is_null()
            {
                return (*new_entry).fp
            } else {
                let save_errno: libc::c_int = *__errno_location();
                entry_free(new_entry as *mut libc::c_void);
                *__errno_location() = save_errno;
                return 0 as *mut FILE;
            }
        }
    };
}
