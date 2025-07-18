use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fchmodat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __mode: __mode_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkdirat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: __mode_t,
    ) -> libc::c_int;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn lchown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn fchownat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn symlinkat(
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
    ) -> libc::c_int;
    fn readlinkat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn hash_get_n_entries(_: *const Hash_table) -> size_t;
    fn hash_lookup(_: *const Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_string(_: *const libc::c_char, _: size_t) -> size_t;
    fn hash_initialize(
        _: size_t,
        _: *const Hash_tuning,
        _: Hash_hasher,
        _: Hash_comparator,
        _: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(_: *mut Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_delete(_: *mut Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn xalloc_die();
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut debug: libc::c_int;
    fn free(__ptr: *mut libc::c_void);
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
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
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
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
pub struct symlink {
    pub prev: *mut symlink,
    pub path: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cached_dirfd {
    pub lru_link: list_head,
    pub children_link: list_head,
    pub children: list_head,
    pub parent: *mut cached_dirfd,
    pub name: *mut libc::c_char,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
pub type Hash_table = hash_table;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstatat(
    mut __fd: libc::c_int,
    mut __filename: *const libc::c_char,
    mut __statbuf: *mut stat,
    mut __flag: libc::c_int,
) -> libc::c_int {
    return __fxstatat(1 as libc::c_int, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn INIT_LIST_HEAD(mut list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}
#[inline]
unsafe extern "C" fn list_add(mut entry: *mut list_head, mut head: *mut list_head) {
    let mut next: *mut list_head = (*head).next;
    (*entry).prev = head;
    (*entry).next = next;
    (*head).next = entry;
    (*next).prev = (*head).next;
}
#[inline]
unsafe extern "C" fn list_del(mut entry: *mut list_head) {
    let mut next: *mut list_head = (*entry).next;
    let mut prev: *mut list_head = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
}
#[inline]
unsafe extern "C" fn list_del_init(mut entry: *mut list_head) {
    list_del(entry);
    INIT_LIST_HEAD(entry);
}
#[inline]
unsafe extern "C" fn list_empty(mut head: *const list_head) -> bool {
    return (*head).next == head as *mut list_head;
}
static mut MAX_PATH_COMPONENTS: libc::c_uint = 1024 as libc::c_int as libc::c_uint;
#[export_name = "unsafe"]
pub static mut unsafe_0: bool = false;
pub static mut dirfd_cache_misses: libc::c_uint = 0;
static mut cached_dirfds: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
static mut max_cached_fds: size_t = 0;
pub static mut lru_list: list_head = unsafe {
    {
        let mut init = list_head {
            next: &lru_list as *const list_head as *mut list_head,
            prev: &lru_list as *const list_head as *mut list_head,
        };
        init
    }
};
unsafe extern "C" fn hash_cached_dirfd(
    mut entry: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut d: *const cached_dirfd = entry as *const cached_dirfd;
    let mut strhash: size_t = hash_string((*d).name, table_size);
    return strhash
        .wrapping_mul(31 as libc::c_int as libc::c_ulong)
        .wrapping_add((*(*d).parent).fd as libc::c_ulong)
        .wrapping_rem(table_size);
}
unsafe extern "C" fn compare_cached_dirfds(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> bool {
    let mut a: *const cached_dirfd = _a as *const cached_dirfd;
    let mut b: *const cached_dirfd = _b as *const cached_dirfd;
    return (*(*a).parent).fd == (*(*b).parent).fd && strcmp((*a).name, (*b).name) == 0;
}
unsafe extern "C" fn free_cached_dirfd(mut entry: *mut cached_dirfd) {
    list_del(&mut (*entry).children_link);
    free((*entry).name as *mut libc::c_void);
    free(entry as *mut libc::c_void);
}
unsafe extern "C" fn init_dirfd_cache() {
    let mut nofile: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    max_cached_fds = 8 as libc::c_int as size_t;
    if getrlimit(RLIMIT_NOFILE, &mut nofile) == 0 as libc::c_int {
        max_cached_fds = if (nofile.rlim_cur)
            .wrapping_div(4 as libc::c_int as libc::c_ulong) > max_cached_fds
        {
            (nofile.rlim_cur).wrapping_div(4 as libc::c_int as libc::c_ulong)
        } else {
            max_cached_fds
        };
    }
    cached_dirfds = hash_initialize(
        max_cached_fds,
        0 as *const Hash_tuning,
        Some(
            hash_cached_dirfd
                as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            compare_cached_dirfds
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    if cached_dirfds.is_null() {
        xalloc_die();
    }
}
unsafe extern "C" fn lookup_cached_dirfd(
    mut dir: *mut cached_dirfd,
    mut name: *const libc::c_char,
) -> *mut cached_dirfd {
    let mut entry: *mut cached_dirfd = 0 as *mut cached_dirfd;
    if !cached_dirfds.is_null() {
        let mut key: cached_dirfd = cached_dirfd {
            lru_link: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            children_link: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            children: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            parent: 0 as *mut cached_dirfd,
            name: 0 as *mut libc::c_char,
            fd: 0,
        };
        key.parent = dir;
        key.name = name as *mut libc::c_char;
        entry = hash_lookup(
            cached_dirfds,
            &mut key as *mut cached_dirfd as *const libc::c_void,
        ) as *mut cached_dirfd;
    }
    return entry;
}
unsafe extern "C" fn remove_cached_dirfd(mut entry: *mut cached_dirfd) {
    while !list_empty(&mut (*entry).children) {
        let mut child: *mut cached_dirfd = ((*entry).children.next as *mut libc::c_char)
            .offset(-(16 as libc::c_ulong as isize)) as *mut cached_dirfd;
        list_del_init(&mut (*child).children_link);
        hash_delete(cached_dirfds, child as *const libc::c_void);
    }
    list_del(&mut (*entry).lru_link);
    hash_delete(cached_dirfds, entry as *const libc::c_void);
    close((*entry).fd);
    free_cached_dirfd(entry);
}
unsafe extern "C" fn insert_cached_dirfd(
    mut entry: *mut cached_dirfd,
    mut keepfd: libc::c_int,
) {
    if cached_dirfds.is_null() {
        init_dirfd_cache();
    }
    while hash_get_n_entries(cached_dirfds) >= max_cached_fds {
        let mut last: *mut cached_dirfd = (lru_list.prev as *mut libc::c_char)
            .offset(-(0 as libc::c_ulong as isize)) as *mut cached_dirfd;
        if &mut (*last).lru_link as *mut list_head == &mut lru_list as *mut list_head {
            break;
        }
        if (*last).fd == keepfd {
            last = ((*last).lru_link.prev as *mut libc::c_char)
                .offset(-(0 as libc::c_ulong as isize)) as *mut cached_dirfd;
            if &mut (*last).lru_link as *mut list_head == &mut lru_list as *mut list_head
            {
                break;
            }
        }
        remove_cached_dirfd(last);
    }
    if !list_empty(&mut (*entry).children_link) {
        if hash_insert(cached_dirfds, entry as *const libc::c_void)
            == entry as *mut libc::c_void
        {} else {
            __assert_fail(
                b"hash_insert (cached_dirfds, entry) == entry\0" as *const u8
                    as *const libc::c_char,
                b"safe.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 53],
                    &[libc::c_char; 53],
                >(b"void insert_cached_dirfd(struct cached_dirfd *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_4067: {
            if hash_insert(cached_dirfds, entry as *const libc::c_void)
                == entry as *mut libc::c_void
            {} else {
                __assert_fail(
                    b"hash_insert (cached_dirfds, entry) == entry\0" as *const u8
                        as *const libc::c_char,
                    b"safe.c\0" as *const u8 as *const libc::c_char,
                    169 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 53],
                        &[libc::c_char; 53],
                    >(b"void insert_cached_dirfd(struct cached_dirfd *, int)\0"))
                        .as_ptr(),
                );
            }
        };
    }
}
unsafe extern "C" fn invalidate_cached_dirfd(
    mut dirfd: libc::c_int,
    mut name: *const libc::c_char,
) {
    let mut dir: cached_dirfd = cached_dirfd {
        lru_link: list_head {
            next: 0 as *mut list_head,
            prev: 0 as *mut list_head,
        },
        children_link: list_head {
            next: 0 as *mut list_head,
            prev: 0 as *mut list_head,
        },
        children: list_head {
            next: 0 as *mut list_head,
            prev: 0 as *mut list_head,
        },
        parent: 0 as *mut cached_dirfd,
        name: 0 as *mut libc::c_char,
        fd: 0,
    };
    let mut key: cached_dirfd = cached_dirfd {
        lru_link: list_head {
            next: 0 as *mut list_head,
            prev: 0 as *mut list_head,
        },
        children_link: list_head {
            next: 0 as *mut list_head,
            prev: 0 as *mut list_head,
        },
        children: list_head {
            next: 0 as *mut list_head,
            prev: 0 as *mut list_head,
        },
        parent: 0 as *mut cached_dirfd,
        name: 0 as *mut libc::c_char,
        fd: 0,
    };
    let mut entry: *mut cached_dirfd = 0 as *mut cached_dirfd;
    if cached_dirfds.is_null() {
        return;
    }
    dir.fd = dirfd;
    key.parent = &mut dir;
    key.name = name as *mut libc::c_char;
    entry = hash_lookup(
        cached_dirfds,
        &mut key as *mut cached_dirfd as *const libc::c_void,
    ) as *mut cached_dirfd;
    if !entry.is_null() {
        remove_cached_dirfd(entry);
    }
}
unsafe extern "C" fn put_path(mut entry: *mut cached_dirfd) -> libc::c_int {
    let mut fd: libc::c_int = (*entry).fd;
    while !entry.is_null() {
        let mut parent: *mut cached_dirfd = (*entry).parent;
        if parent.is_null() {
            break;
        }
        list_add(&mut (*entry).lru_link, &mut lru_list);
        entry = parent;
    }
    return fd;
}
unsafe extern "C" fn new_cached_dirfd(
    mut dir: *mut cached_dirfd,
    mut name: *const libc::c_char,
    mut fd: libc::c_int,
) -> *mut cached_dirfd {
    let mut entry: *mut cached_dirfd = xmalloc(
        ::std::mem::size_of::<cached_dirfd>() as libc::c_ulong,
    ) as *mut cached_dirfd;
    INIT_LIST_HEAD(&mut (*entry).lru_link);
    list_add(&mut (*entry).children_link, &mut (*dir).children);
    INIT_LIST_HEAD(&mut (*entry).children);
    (*entry).parent = dir;
    (*entry).name = xstrdup(name);
    (*entry).fd = fd;
    return entry;
}
unsafe extern "C" fn openat_cached(
    mut dir: *mut cached_dirfd,
    mut name: *const libc::c_char,
    mut keepfd: libc::c_int,
) -> *mut cached_dirfd {
    let mut fd: libc::c_int = 0;
    let mut entry: *mut cached_dirfd = lookup_cached_dirfd(dir, name);
    if !entry.is_null() {
        list_del_init(&mut (*entry).lru_link);
    } else {
        dirfd_cache_misses = dirfd_cache_misses.wrapping_add(1);
        dirfd_cache_misses;
        fd = openat((*dir).fd, name, 0o200000 as libc::c_int | 0o400000 as libc::c_int);
        if fd < 0 as libc::c_int {
            return 0 as *mut cached_dirfd;
        }
        entry = new_cached_dirfd(dir, name, fd);
        insert_cached_dirfd(entry, keepfd);
    }
    return entry;
}
unsafe extern "C" fn count_path_components(
    mut path: *const libc::c_char,
) -> libc::c_uint {
    let mut components: libc::c_uint = 0;
    while *path as libc::c_int == '/' as i32 {
        path = path.offset(1);
        path;
    }
    if *path == 0 {
        return 1 as libc::c_int as libc::c_uint;
    }
    components = 0 as libc::c_int as libc::c_uint;
    while *path != 0 {
        while *path as libc::c_int != 0 && !(*path as libc::c_int == '/' as i32) {
            path = path.offset(1);
            path;
        }
        while *path as libc::c_int == '/' as i32 {
            path = path.offset(1);
            path;
        }
        components = components.wrapping_add(1);
        components;
    }
    return components;
}
unsafe extern "C" fn push_symlink(
    mut stack: *mut *mut symlink,
    mut symlink_0: *mut symlink,
) {
    (*symlink_0).prev = *stack;
    *stack = symlink_0;
}
unsafe extern "C" fn pop_symlink(mut stack: *mut *mut symlink) {
    let mut top: *mut symlink = *stack;
    *stack = (*top).prev;
    free(top as *mut libc::c_void);
}
pub static mut cwd_stat_errno: libc::c_int = -(1 as libc::c_int);
pub static mut cwd_stat: stat = stat {
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
unsafe extern "C" fn read_symlink(
    mut dirfd: libc::c_int,
    mut name: *const libc::c_char,
) -> *mut symlink {
    let mut current_block: u64;
    let mut saved_errno: libc::c_int = *__errno_location();
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
    let mut symlink_0: *mut symlink = 0 as *mut symlink;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: ssize_t = 0;
    if fstatat(dirfd, name, &mut st, 0x100 as libc::c_int) != 0
        || !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
    {
        *__errno_location() = saved_errno;
        return 0 as *mut symlink;
    }
    symlink_0 = xmalloc(
        (::std::mem::size_of::<symlink>() as libc::c_ulong)
            .wrapping_add(st.st_size as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut symlink;
    buffer = symlink_0.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    ret = readlinkat(dirfd, name, buffer, st.st_size as size_t);
    if !(ret <= 0 as libc::c_int as libc::c_long) {
        *buffer.offset(ret as isize) = 0 as libc::c_int as libc::c_char;
        (*symlink_0).path = buffer;
        if *buffer as libc::c_int == '/' as i32 {
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            if cwd_stat_errno == -(1 as libc::c_int) {
                cwd_stat_errno = if stat(
                    b".\0" as *const u8 as *const libc::c_char,
                    &mut cwd_stat,
                ) == 0 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    *__errno_location()
                };
                if cwd_stat_errno != 0 {
                    current_block = 16477040363243130047;
                } else {
                    current_block = 1841672684692190573;
                }
            } else {
                current_block = 1841672684692190573;
            }
            match current_block {
                1841672684692190573 => {
                    end = buffer.offset(ret as isize);
                    loop {
                        let mut slash: libc::c_char = 0;
                        let mut rv: libc::c_int = 0;
                        slash = *end;
                        *end = 0 as libc::c_int as libc::c_char;
                        rv = stat((*symlink_0).path, &mut st);
                        *end = slash;
                        if rv == 0 as libc::c_int && st.st_dev == cwd_stat.st_dev
                            && st.st_ino == cwd_stat.st_ino
                        {
                            while *end as libc::c_int == '/' as i32 {
                                end = end.offset(1);
                                end;
                            }
                            (*symlink_0).path = end;
                            return symlink_0;
                        }
                        end = end.offset(-1);
                        end;
                        if end == (*symlink_0).path as *mut libc::c_char {
                            break;
                        }
                        while end
                            != ((*symlink_0).path).offset(1 as libc::c_int as isize)
                                as *mut libc::c_char && !(*end as libc::c_int == '/' as i32)
                        {
                            end = end.offset(-1);
                            end;
                        }
                        while end
                            != ((*symlink_0).path).offset(1 as libc::c_int as isize)
                                as *mut libc::c_char
                            && *end.offset(-(1 as libc::c_int as isize)) as libc::c_int
                                == '/' as i32
                        {
                            end = end.offset(-1);
                            end;
                        }
                    }
                }
                _ => {}
            }
            *__errno_location() = 18 as libc::c_int;
        } else {
            return symlink_0
        }
    }
    free(symlink_0 as *mut libc::c_void);
    return 0 as *mut symlink;
}
unsafe extern "C" fn traverse_next(
    mut dir: *mut cached_dirfd,
    mut path: *mut *const libc::c_char,
    mut keepfd: libc::c_int,
    mut symlink_0: *mut *mut symlink,
) -> *mut cached_dirfd {
    let mut current_block: u64;
    let mut p: *const libc::c_char = *path;
    let mut entry: *mut cached_dirfd = dir;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    while *p as libc::c_int != 0 && !(*p as libc::c_int == '/' as i32) {
        p = p.offset(1);
        p;
    }
    if **path as libc::c_int == '.' as i32
        && (*path).offset(1 as libc::c_int as isize) == p
    {
        current_block = 13452095205664998285;
    } else if **path as libc::c_int == '.' as i32
        && *(*path).offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
        && (*path).offset(2 as libc::c_int as isize) == p
    {
        entry = (*dir).parent;
        if entry.is_null() {
            *__errno_location() = 18 as libc::c_int;
            current_block = 10114013572737001381;
        } else {
            if list_empty(&mut (*dir).lru_link) {} else {
                __assert_fail(
                    b"list_empty (&dir->lru_link)\0" as *const u8 as *const libc::c_char,
                    b"safe.c\0" as *const u8 as *const libc::c_char,
                    379 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 97],
                        &[libc::c_char; 97],
                    >(
                        b"struct cached_dirfd *traverse_next(struct cached_dirfd *, const char **, int, struct symlink **)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_5740: {
                if list_empty(&mut (*dir).lru_link) {} else {
                    __assert_fail(
                        b"list_empty (&dir->lru_link)\0" as *const u8
                            as *const libc::c_char,
                        b"safe.c\0" as *const u8 as *const libc::c_char,
                        379 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 97],
                            &[libc::c_char; 97],
                        >(
                            b"struct cached_dirfd *traverse_next(struct cached_dirfd *, const char **, int, struct symlink **)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            list_add(&mut (*dir).lru_link, &mut lru_list);
            current_block = 13452095205664998285;
        }
    } else {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (p.offset_from(*path) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong as usize,
        );
        name = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(
            name as *mut libc::c_void,
            *path as *const libc::c_void,
            p.offset_from(*path) as libc::c_long as libc::c_ulong,
        );
        *name
            .offset(
                p.offset_from(*path) as libc::c_long as isize,
            ) = 0 as libc::c_int as libc::c_char;
        entry = openat_cached(dir, name, keepfd);
        if entry.is_null() {
            if *__errno_location() == 40 as libc::c_int
                || *__errno_location() == 31 as libc::c_int
                || *__errno_location() == 0 as libc::c_int
                || *__errno_location() == 20 as libc::c_int
            {
                *symlink_0 = read_symlink((*dir).fd, name);
                if !(*symlink_0).is_null() {
                    entry = dir;
                    current_block = 13452095205664998285;
                } else {
                    *__errno_location() = 40 as libc::c_int;
                    current_block = 10114013572737001381;
                }
            } else {
                current_block = 10114013572737001381;
            }
        } else {
            current_block = 13452095205664998285;
        }
    }
    match current_block {
        13452095205664998285 => {
            while *p as libc::c_int == '/' as i32 {
                p = p.offset(1);
                p;
            }
        }
        _ => {}
    }
    *path = p;
    return entry;
}
unsafe extern "C" fn traverse_another_path(
    mut pathname: *mut *const libc::c_char,
    mut keepfd: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    static mut cwd: cached_dirfd = {
        let mut init = cached_dirfd {
            lru_link: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            children_link: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            children: list_head {
                next: 0 as *mut list_head,
                prev: 0 as *mut list_head,
            },
            parent: 0 as *const cached_dirfd as *mut cached_dirfd,
            name: 0 as *const libc::c_char as *mut libc::c_char,
            fd: -(100 as libc::c_int),
        };
        init
    };
    let mut misses: libc::c_uint = dirfd_cache_misses;
    let mut path: *const libc::c_char = *pathname;
    let mut last: *const libc::c_char = 0 as *const libc::c_char;
    let mut dir: *mut cached_dirfd = &mut cwd;
    let mut stack: *mut symlink = 0 as *mut symlink;
    let mut steps: libc::c_uint = count_path_components(path);
    let mut traversed_symlink: *mut cached_dirfd = 0 as *mut cached_dirfd;
    INIT_LIST_HEAD(&mut cwd.children);
    if steps > MAX_PATH_COMPONENTS {
        *__errno_location() = 40 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if *path == 0
        || (*path.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            || 0 as libc::c_int != 0 as libc::c_int)
    {
        return -(100 as libc::c_int);
    }
    last = (strrchr(path, 0 as libc::c_int)).offset(-(1 as libc::c_int as isize));
    if *last as libc::c_int == '/' as i32 {
        while last != path {
            last = last.offset(-1);
            if !(*last as libc::c_int == '/' as i32) {
                break;
            }
        }
    }
    while last != path
        && !(*last.offset(-(1 as libc::c_int as isize)) as libc::c_int == '/' as i32)
    {
        last = last.offset(-1);
        last;
    }
    if last == path {
        return -(100 as libc::c_int);
    }
    if debug & 32 as libc::c_int != 0 {
        printf(
            b"Resolving path \"%.*s\"\0" as *const u8 as *const libc::c_char,
            last.offset_from(path) as libc::c_long as libc::c_int,
            path,
        );
    }
    loop {
        if !(!stack.is_null() || path != last) {
            current_block = 1345366029464561491;
            break;
        }
        let mut entry: *mut cached_dirfd = 0 as *mut cached_dirfd;
        let mut symlink_0: *mut symlink = 0 as *mut symlink;
        let mut prev: *const libc::c_char = path;
        entry = traverse_next(
            dir,
            if !stack.is_null() { &mut (*stack).path } else { &mut path },
            keepfd,
            &mut symlink_0,
        );
        if entry.is_null() {
            if debug & 32 as libc::c_int != 0 {
                printf(b" (failed)\n\0" as *const u8 as *const libc::c_char);
                fflush(stdout);
            }
            current_block = 6948114259746151983;
            break;
        } else {
            dir = entry;
            if stack.is_null() && !symlink_0.is_null() {
                let mut p: *const libc::c_char = prev;
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                while *p as libc::c_int != 0 && !(*p as libc::c_int == '/' as i32) {
                    p = p.offset(1);
                    p;
                }
                let mut fresh1 = ::std::vec::from_elem(
                    0,
                    (p.offset_from(prev) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as libc::c_ulong as usize,
                );
                name = fresh1.leak().as_mut_ptr() as *mut libc::c_char;
                memcpy(
                    name as *mut libc::c_void,
                    prev as *const libc::c_void,
                    p.offset_from(prev) as libc::c_long as libc::c_ulong,
                );
                *name
                    .offset(
                        p.offset_from(prev) as libc::c_long as isize,
                    ) = 0 as libc::c_int as libc::c_char;
                traversed_symlink = new_cached_dirfd(dir, name, -(1 as libc::c_int));
            }
            if !stack.is_null() && *(*stack).path == 0 {
                pop_symlink(&mut stack);
            }
            if !symlink_0.is_null() && *(*symlink_0).path as libc::c_int != 0 {
                push_symlink(&mut stack, symlink_0);
                steps = steps.wrapping_add(count_path_components((*symlink_0).path));
                if steps > MAX_PATH_COMPONENTS {
                    *__errno_location() = 40 as libc::c_int;
                    current_block = 6948114259746151983;
                    break;
                }
            } else if !symlink_0.is_null() {
                pop_symlink(&mut symlink_0);
            }
            if !traversed_symlink.is_null() && stack.is_null() {
                (*traversed_symlink)
                    .fd = if (*entry).fd == -(100 as libc::c_int) {
                    -(100 as libc::c_int)
                } else {
                    dup((*entry).fd)
                };
                if (*traversed_symlink).fd != -(1 as libc::c_int) {
                    insert_cached_dirfd(traversed_symlink, keepfd);
                    list_add(&mut (*traversed_symlink).lru_link, &mut lru_list);
                } else {
                    free_cached_dirfd(traversed_symlink);
                }
                traversed_symlink = 0 as *mut cached_dirfd;
            }
        }
    }
    match current_block {
        6948114259746151983 => {
            if !traversed_symlink.is_null() {
                free_cached_dirfd(traversed_symlink);
            }
            put_path(dir);
            while !stack.is_null() {
                pop_symlink(&mut stack);
            }
            return -(1 as libc::c_int);
        }
        _ => {
            *pathname = last;
            if debug & 32 as libc::c_int != 0 {
                misses = (dirfd_cache_misses as libc::c_int - misses as libc::c_int)
                    as libc::c_uint;
                if misses == 0 {
                    printf(b" (cached)\n\0" as *const u8 as *const libc::c_char);
                } else {
                    printf(
                        b" (%u miss%s)\n\0" as *const u8 as *const libc::c_char,
                        misses,
                        if misses == 1 as libc::c_int as libc::c_uint {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            b"es\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                fflush(stdout);
            }
            return put_path(dir);
        }
    };
}
unsafe extern "C" fn traverse_path(
    mut pathname: *mut *const libc::c_char,
) -> libc::c_int {
    return traverse_another_path(pathname, -(1 as libc::c_int));
}
unsafe extern "C" fn safe_xstat(
    mut pathname: *const libc::c_char,
    mut buf: *mut stat,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return fstatat(-(100 as libc::c_int), pathname, buf, flags);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return fstatat(dirfd, pathname, buf, flags);
}
pub unsafe extern "C" fn safe_stat(
    mut pathname: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    return safe_xstat(pathname, buf, 0 as libc::c_int);
}
pub unsafe extern "C" fn safe_lstat(
    mut pathname: *const libc::c_char,
    mut buf: *mut stat,
) -> libc::c_int {
    return safe_xstat(pathname, buf, 0x100 as libc::c_int);
}
pub unsafe extern "C" fn safe_open(
    mut pathname: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return open(pathname, flags, mode);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return openat(dirfd, pathname, flags, mode);
}
pub unsafe extern "C" fn safe_rename(
    mut oldpath: *const libc::c_char,
    mut newpath: *const libc::c_char,
) -> libc::c_int {
    let mut olddirfd: libc::c_int = 0;
    let mut newdirfd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if unsafe_0 {
        return rename(oldpath, newpath);
    }
    olddirfd = traverse_path(&mut oldpath);
    if olddirfd < 0 as libc::c_int && olddirfd != -(100 as libc::c_int) {
        return olddirfd;
    }
    newdirfd = traverse_another_path(&mut newpath, olddirfd);
    if newdirfd < 0 as libc::c_int && newdirfd != -(100 as libc::c_int) {
        return newdirfd;
    }
    ret = renameat(olddirfd, oldpath, newdirfd, newpath);
    if ret == 0 {
        invalidate_cached_dirfd(olddirfd, oldpath);
        invalidate_cached_dirfd(newdirfd, newpath);
    }
    return ret;
}
pub unsafe extern "C" fn safe_mkdir(
    mut pathname: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return mkdir(pathname, mode);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return mkdirat(dirfd, pathname, mode);
}
pub unsafe extern "C" fn safe_rmdir(mut pathname: *const libc::c_char) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if unsafe_0 {
        return rmdir(pathname);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    ret = unlinkat(dirfd, pathname, 0x200 as libc::c_int);
    if ret == 0 {
        invalidate_cached_dirfd(dirfd, pathname);
    }
    return ret;
}
pub unsafe extern "C" fn safe_unlink(mut pathname: *const libc::c_char) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return unlink(pathname);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return unlinkat(dirfd, pathname, 0 as libc::c_int);
}
pub unsafe extern "C" fn safe_symlink(
    mut target: *const libc::c_char,
    mut linkpath: *const libc::c_char,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return symlink(target, linkpath);
    }
    dirfd = traverse_path(&mut linkpath);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return symlinkat(target, dirfd, linkpath);
}
pub unsafe extern "C" fn safe_chmod(
    mut pathname: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return chmod(pathname, mode);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return fchmodat(dirfd, pathname, mode, 0 as libc::c_int);
}
pub unsafe extern "C" fn safe_lchown(
    mut pathname: *const libc::c_char,
    mut owner: uid_t,
    mut group: gid_t,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return lchown(pathname, owner, group);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return fchownat(dirfd, pathname, owner, group, 0x100 as libc::c_int);
}
pub unsafe extern "C" fn safe_lutimens(
    mut pathname: *const libc::c_char,
    mut times: *const timespec,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return utimensat(-(100 as libc::c_int), pathname, times, 0x100 as libc::c_int);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return utimensat(dirfd, pathname, times, 0x100 as libc::c_int);
}
pub unsafe extern "C" fn safe_readlink(
    mut pathname: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
) -> ssize_t {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return readlink(pathname, buf, bufsiz);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd as ssize_t;
    }
    return readlinkat(dirfd, pathname, buf, bufsiz);
}
pub unsafe extern "C" fn safe_access(
    mut pathname: *const libc::c_char,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut dirfd: libc::c_int = 0;
    if unsafe_0 {
        return access(pathname, mode);
    }
    dirfd = traverse_path(&mut pathname);
    if dirfd < 0 as libc::c_int && dirfd != -(100 as libc::c_int) {
        return dirfd;
    }
    return faccessat(dirfd, pathname, mode, 0 as libc::c_int);
}
