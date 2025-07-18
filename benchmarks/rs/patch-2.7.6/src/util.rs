use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut program_name: *const libc::c_char;
    static mut buf: *mut libc::c_char;
    static mut bufsize: size_t;
    static mut using_plan_a: bool;
    static mut dry_run: bool;
    static mut posixly_correct: bool;
    static mut origprae: *const libc::c_char;
    static mut origbase: *const libc::c_char;
    static mut origsuff: *const libc::c_char;
    static mut debug: libc::c_int;
    static mut force: bool;
    static mut batch: bool;
    static mut noreverse: bool;
    static mut reverse: bool;
    static mut verbosity: C2RustUnnamed_0;
    static mut skip_rest_of_patch: bool;
    static mut patch_get: libc::c_int;
    static mut set_time: bool;
    static mut set_utc: bool;
    static mut follow_symlinks: bool;
    fn fatal_exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn geteuid() -> __uid_t;
    fn getegid() -> __gid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn hash_lookup(_: *const Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn hash_initialize(
        _: size_t,
        _: *const Hash_tuning,
        _: Hash_hasher,
        _: Hash_comparator,
        _: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(_: *mut Hash_table, _: *const libc::c_void) -> *mut libc::c_void;
    fn quotearg_n(n: libc::c_int, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg(arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn gettime(_: *mut timespec);
    fn find_backup_file_name(
        _: *const libc::c_char,
        _: backup_type,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xmemdup0(p: *const libc::c_void, s: size_t) -> *mut libc::c_char;
    fn parse_datetime(
        _: *mut timespec,
        _: *const libc::c_char,
        _: *const timespec,
    ) -> bool;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn full_write(fd: libc::c_int, buf_0: *const libc::c_void, count: size_t) -> size_t;
    fn try_tempname(
        tmpl: *mut libc::c_char,
        suffixlen: libc::c_int,
        args: *mut libc::c_void,
        tryfunc: Option::<
            unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_void) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn safe_stat(pathname: *const libc::c_char, buf_0: *mut stat) -> libc::c_int;
    fn safe_lstat(pathname: *const libc::c_char, buf_0: *mut stat) -> libc::c_int;
    fn safe_open(
        pathname: *const libc::c_char,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn safe_rename(
        oldpath: *const libc::c_char,
        newpath: *const libc::c_char,
    ) -> libc::c_int;
    fn safe_mkdir(pathname: *const libc::c_char, mode: mode_t) -> libc::c_int;
    fn safe_rmdir(pathname: *const libc::c_char) -> libc::c_int;
    fn safe_unlink(pathname: *const libc::c_char) -> libc::c_int;
    fn safe_symlink(
        target: *const libc::c_char,
        linkpath: *const libc::c_char,
    ) -> libc::c_int;
    fn safe_chmod(pathname: *const libc::c_char, mode: mode_t) -> libc::c_int;
    fn safe_lchown(
        pathname: *const libc::c_char,
        owner: uid_t,
        group: gid_t,
    ) -> libc::c_int;
    fn safe_lutimens(
        pathname: *const libc::c_char,
        times: *const timespec,
    ) -> libc::c_int;
    fn safe_readlink(
        pathname: *const libc::c_char,
        buf_0: *mut libc::c_char,
        bufsiz: size_t,
    ) -> ssize_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
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
pub type lin = off_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const VERBOSE: C2RustUnnamed_0 = 2;
pub const SILENT: C2RustUnnamed_0 = 1;
pub const DEFAULT_VERBOSITY: C2RustUnnamed_0 = 0;
pub type file_offset = libc::c_long;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const TIMESPEC_RESOLUTION: C2RustUnnamed_1 = 1000000000;
pub type backup_type = libc::c_uint;
pub const numbered_backups: backup_type = 3;
pub const numbered_existing_backups: backup_type = 2;
pub const simple_backups: backup_type = 1;
pub const no_backups: backup_type = 0;
pub type file_id_type = libc::c_uint;
pub const OVERWRITTEN: file_id_type = 3;
pub const DELETE_LATER: file_id_type = 2;
pub const CREATED: file_id_type = 1;
pub const UNKNOWN: file_id_type = 0;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_id {
    pub dev: dev_t,
    pub ino: ino_t,
    pub type_0: file_id_type,
    pub queued_output: bool,
}
pub type file_attributes = libc::c_uint;
pub const FA_XATTRS: file_attributes = 8;
pub const FA_MODE: file_attributes = 4;
pub const FA_IDS: file_attributes = 2;
pub const FA_TIMES: file_attributes = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_2,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_12,
    pub _timer: C2RustUnnamed_11,
    pub _rt: C2RustUnnamed_10,
    pub _sigchld: C2RustUnnamed_9,
    pub _sigfault: C2RustUnnamed_6,
    pub _sigpoll: C2RustUnnamed_5,
    pub _sigsys: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub _addr_bnd: C2RustUnnamed_8,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct try_safe_open_args {
    pub flags: libc::c_int,
    pub mode: mode_t,
}
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
pub static mut backup_type: backup_type = no_backups;
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    if a.tv_sec < b.tv_sec {
        return -(1 as libc::c_int);
    }
    if a.tv_sec > b.tv_sec {
        return 1 as libc::c_int;
    }
    if -(1 as libc::c_int) as libc::c_long <= a.tv_nsec
        && a.tv_nsec
            <= (2 as libc::c_int * TIMESPEC_RESOLUTION as libc::c_int) as libc::c_long
    {} else {
        unreachable!();
    };
    if -(1 as libc::c_int) as libc::c_long <= b.tv_nsec
        && b.tv_nsec
            <= (2 as libc::c_int * TIMESPEC_RESOLUTION as libc::c_int) as libc::c_long
    {} else {
        unreachable!();
    };
    return (a.tv_nsec - b.tv_nsec) as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
unsafe extern "C" fn file_id_hasher(
    mut entry: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut e: *const file_id = entry as *const file_id;
    let mut i: size_t = ((*e).ino).wrapping_add((*e).dev);
    return i.wrapping_rem(table_size);
}
unsafe extern "C" fn file_id_comparator(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut e1: *const file_id = entry1 as *const file_id;
    let mut e2: *const file_id = entry2 as *const file_id;
    return (*e1).ino == (*e2).ino && (*e1).dev == (*e2).dev;
}
static mut file_id_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
pub unsafe extern "C" fn init_backup_hash_table() {
    file_id_table = hash_initialize(
        0 as libc::c_int as size_t,
        0 as *const Hash_tuning,
        Some(
            file_id_hasher as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
        ),
        Some(
            file_id_comparator
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if file_id_table.is_null() {
        xalloc_die();
    }
}
unsafe extern "C" fn __insert_file_id(
    mut st: *const stat,
    mut type_0: file_id_type,
) -> *mut file_id {
    let mut p: *mut file_id = 0 as *mut file_id;
    static mut next_slot: *mut file_id = 0 as *const file_id as *mut file_id;
    if next_slot.is_null() {
        next_slot = xmalloc(::std::mem::size_of::<file_id>() as libc::c_ulong)
            as *mut file_id;
    }
    (*next_slot).dev = (*st).st_dev;
    (*next_slot).ino = (*st).st_ino;
    (*next_slot).queued_output = 0 as libc::c_int != 0;
    p = hash_insert(file_id_table, next_slot as *const libc::c_void) as *mut file_id;
    if p.is_null() {
        xalloc_die();
    }
    if p == next_slot {
        next_slot = 0 as *mut file_id;
    }
    (*p).type_0 = type_0;
    return p;
}
unsafe extern "C" fn __lookup_file_id(mut st: *const stat) -> *mut file_id {
    let mut f: file_id = file_id {
        dev: 0,
        ino: 0,
        type_0: UNKNOWN,
        queued_output: false,
    };
    f.dev = (*st).st_dev;
    f.ino = (*st).st_ino;
    return hash_lookup(file_id_table, &mut f as *mut file_id as *const libc::c_void)
        as *mut file_id;
}
pub unsafe extern "C" fn insert_file_id(mut st: *const stat, mut type_0: file_id_type) {
    __insert_file_id(st, type_0);
}
pub unsafe extern "C" fn lookup_file_id(mut st: *const stat) -> file_id_type {
    let mut p: *mut file_id = __lookup_file_id(st);
    return (if !p.is_null() {
        (*p).type_0 as libc::c_uint
    } else {
        UNKNOWN as libc::c_int as libc::c_uint
    }) as file_id_type;
}
pub unsafe extern "C" fn set_queued_output(
    mut st: *const stat,
    mut queued_output: bool,
) {
    let mut p: *mut file_id = __lookup_file_id(st);
    if p.is_null() {
        p = __insert_file_id(st, UNKNOWN);
    }
    (*p).queued_output = queued_output;
}
pub unsafe extern "C" fn has_queued_output(mut st: *const stat) -> bool {
    let mut p: *mut file_id = __lookup_file_id(st);
    return !p.is_null() && (*p).queued_output as libc::c_int != 0;
}
unsafe extern "C" fn contains_slash(mut s: *const libc::c_char) -> bool {
    while *s != 0 {
        if *s as libc::c_int == '/' as i32 {
            return 1 as libc::c_int != 0;
        }
        s = s.offset(1);
        s;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn copy_attr(
    mut src_path: *const libc::c_char,
    mut dst_path: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn set_file_attributes(
    mut to: *const libc::c_char,
    mut attr: file_attributes,
    mut from: *const libc::c_char,
    mut st: *const stat,
    mut mode: mode_t,
    mut new_time: *mut timespec,
) {
    if attr as libc::c_uint & FA_TIMES as libc::c_int as libc::c_uint != 0 {
        let mut times: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
        if !new_time.is_null() {
            times[1 as libc::c_int as usize] = *new_time;
            times[0 as libc::c_int as usize] = times[1 as libc::c_int as usize];
        } else {
            times[0 as libc::c_int as usize] = get_stat_atime(st);
            times[1 as libc::c_int as usize] = get_stat_mtime(st);
        }
        if safe_lutimens(to, times.as_mut_ptr() as *const timespec) != 0 as libc::c_int {
            pfatal(
                b"Failed to set the timestamps of %s %s\0" as *const u8
                    as *const libc::c_char,
                if mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                {
                    b"symbolic link\0" as *const u8 as *const libc::c_char
                } else {
                    b"file\0" as *const u8 as *const libc::c_char
                },
                quotearg(to),
            );
        }
    }
    if attr as libc::c_uint & FA_IDS as libc::c_int as libc::c_uint != 0 {
        static mut euid: uid_t = -(1 as libc::c_int) as uid_t;
        static mut egid: gid_t = -(1 as libc::c_int) as gid_t;
        let mut uid: uid_t = 0;
        let mut gid: uid_t = 0;
        if euid == -(1 as libc::c_int) as libc::c_uint {
            euid = geteuid();
            egid = getegid();
        }
        uid = if euid == (*st).st_uid {
            -(1 as libc::c_int) as libc::c_uint
        } else {
            (*st).st_uid
        };
        gid = if egid == (*st).st_gid {
            -(1 as libc::c_int) as libc::c_uint
        } else {
            (*st).st_gid
        };
        if (uid != -(1 as libc::c_int) as libc::c_uint
            || gid != -(1 as libc::c_int) as libc::c_uint)
            && safe_lchown(to, uid, gid) != 0 as libc::c_int
            && (*__errno_location() != 1 as libc::c_int
                || uid != -(1 as libc::c_int) as libc::c_uint
                    && {
                        uid = -(1 as libc::c_int) as uid_t;
                        safe_lchown(to, uid, gid) != 0 as libc::c_int
                    } && *__errno_location() != 1 as libc::c_int)
        {
            pfatal(
                b"Failed to set the %s of %s %s\0" as *const u8 as *const libc::c_char,
                if uid == -(1 as libc::c_int) as libc::c_uint {
                    b"owner\0" as *const u8 as *const libc::c_char
                } else {
                    b"owning group\0" as *const u8 as *const libc::c_char
                },
                if mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                {
                    b"symbolic link\0" as *const u8 as *const libc::c_char
                } else {
                    b"file\0" as *const u8 as *const libc::c_char
                },
                quotearg(to),
            );
        }
    }
    if attr as libc::c_uint & FA_XATTRS as libc::c_int as libc::c_uint != 0 {
        if copy_attr(from, to) != 0 as libc::c_int
            && *__errno_location() != 38 as libc::c_int
            && *__errno_location() != 95 as libc::c_int
            && *__errno_location() != 1 as libc::c_int
        {
            fatal_exit(0 as libc::c_int);
        }
    }
    if attr as libc::c_uint & FA_MODE as libc::c_int as libc::c_uint != 0 {
        if !(mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint)
            && safe_chmod(to, mode) != 0 as libc::c_int
        {
            pfatal(
                b"Failed to set the permissions of %s %s\0" as *const u8
                    as *const libc::c_char,
                if mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                {
                    b"symbolic link\0" as *const u8 as *const libc::c_char
                } else {
                    b"file\0" as *const u8 as *const libc::c_char
                },
                quotearg(to),
            );
        }
    }
}
unsafe extern "C" fn create_backup_copy(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut st: *const stat,
    mut to_dir_known_to_exist: bool,
) {
    copy_file(
        from,
        to,
        0 as *mut stat,
        0 as libc::c_int,
        (*st).st_mode,
        to_dir_known_to_exist,
    );
    set_file_attributes(
        to,
        (FA_TIMES as libc::c_int | FA_IDS as libc::c_int | FA_MODE as libc::c_int)
            as file_attributes,
        from,
        st,
        (*st).st_mode,
        0 as *mut timespec,
    );
}
pub unsafe extern "C" fn create_backup(
    mut to: *const libc::c_char,
    mut to_st: *const stat,
    mut leave_original: bool,
) {
    if !to_st.is_null()
        && !((*to_st).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
            || (*to_st).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint)
    {
        fatal(
            b"File %s is not a %s -- refusing to create backup\0" as *const u8
                as *const libc::c_char,
            to,
            if (*to_st).st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {
                b"symbolic link\0" as *const u8 as *const libc::c_char
            } else {
                b"regular file\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !to_st.is_null()
        && lookup_file_id(to_st) as libc::c_uint
            == CREATED as libc::c_int as libc::c_uint
    {
        if debug & 4 as libc::c_int != 0 {
            say(
                b"File %s already seen\n\0" as *const u8 as *const libc::c_char,
                quotearg(to),
            );
        }
    } else {
        let mut try_makedirs_errno: libc::c_int = 0 as libc::c_int;
        let mut bakname: *mut libc::c_char = 0 as *mut libc::c_char;
        if !origprae.is_null() || !origbase.is_null() || !origsuff.is_null() {
            let mut p: *const libc::c_char = if !origprae.is_null() {
                origprae
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            let mut b: *const libc::c_char = if !origbase.is_null() {
                origbase
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            let mut s: *const libc::c_char = if !origsuff.is_null() {
                origsuff
            } else {
                b"\0" as *const u8 as *const libc::c_char
            };
            let mut t: *const libc::c_char = to;
            let mut plen: size_t = strlen(p);
            let mut blen: size_t = strlen(b);
            let mut slen: size_t = strlen(s);
            let mut tlen: size_t = strlen(t);
            let mut o: *const libc::c_char = 0 as *const libc::c_char;
            let mut olen: size_t = 0;
            o = t.offset(tlen as isize);
            olen = 0 as libc::c_int as size_t;
            while o > t
                && !(*o.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == '/' as i32)
            {
                o = o.offset(-1);
                o;
            }
            olen = t.offset(tlen as isize).offset_from(o) as libc::c_long as size_t;
            tlen = (tlen as libc::c_ulong).wrapping_sub(olen) as size_t as size_t;
            bakname = xmalloc(
                plen
                    .wrapping_add(tlen)
                    .wrapping_add(blen)
                    .wrapping_add(olen)
                    .wrapping_add(slen)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(bakname as *mut libc::c_void, p as *const libc::c_void, plen);
            memcpy(
                bakname.offset(plen as isize) as *mut libc::c_void,
                t as *const libc::c_void,
                tlen,
            );
            memcpy(
                bakname.offset(plen as isize).offset(tlen as isize) as *mut libc::c_void,
                b as *const libc::c_void,
                blen,
            );
            memcpy(
                bakname.offset(plen as isize).offset(tlen as isize).offset(blen as isize)
                    as *mut libc::c_void,
                o as *const libc::c_void,
                olen,
            );
            memcpy(
                bakname
                    .offset(plen as isize)
                    .offset(tlen as isize)
                    .offset(blen as isize)
                    .offset(olen as isize) as *mut libc::c_void,
                s as *const libc::c_void,
                slen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            if !origprae.is_null()
                && (contains_slash(origprae.offset(0 as libc::c_int as isize))
                    as libc::c_int != 0 || contains_slash(to) as libc::c_int != 0)
                || !origbase.is_null() && contains_slash(origbase) as libc::c_int != 0
            {
                try_makedirs_errno = 2 as libc::c_int;
            }
        } else {
            bakname = find_backup_file_name(to, backup_type);
            if bakname.is_null() {
                xalloc_die();
            }
        }
        if to_st.is_null() {
            let mut fd: libc::c_int = 0;
            if debug & 4 as libc::c_int != 0 {
                say(
                    b"Creating empty file %s\n\0" as *const u8 as *const libc::c_char,
                    quotearg(bakname),
                );
            }
            try_makedirs_errno = 2 as libc::c_int;
            safe_unlink(bakname);
            loop {
                fd = safe_open(
                    bakname,
                    0o100 as libc::c_int | 0o1 as libc::c_int | 0o1000 as libc::c_int,
                    0o666 as libc::c_int as mode_t,
                );
                if !(fd < 0 as libc::c_int) {
                    break;
                }
                if *__errno_location() != try_makedirs_errno {
                    pfatal(
                        b"Can't create file %s\0" as *const u8 as *const libc::c_char,
                        quotearg(bakname),
                    );
                }
                makedirs(bakname);
                try_makedirs_errno = 0 as libc::c_int;
            }
            if close(fd) != 0 as libc::c_int {
                pfatal(
                    b"Can't close file %s\0" as *const u8 as *const libc::c_char,
                    quotearg(bakname),
                );
            }
        } else if leave_original {
            create_backup_copy(
                to,
                bakname,
                to_st,
                try_makedirs_errno == 0 as libc::c_int,
            );
        } else {
            if debug & 4 as libc::c_int != 0 {
                say(
                    b"Renaming file %s to %s\n\0" as *const u8 as *const libc::c_char,
                    quotearg_n(0 as libc::c_int, to),
                    quotearg_n(1 as libc::c_int, bakname),
                );
            }
            while safe_rename(to, bakname) != 0 as libc::c_int {
                if *__errno_location() == try_makedirs_errno {
                    makedirs(bakname);
                    try_makedirs_errno = 0 as libc::c_int;
                } else if *__errno_location() == 18 as libc::c_int {
                    create_backup_copy(
                        to,
                        bakname,
                        to_st,
                        try_makedirs_errno == 0 as libc::c_int,
                    );
                    safe_unlink(to);
                    break;
                } else {
                    pfatal(
                        b"Can't rename file %s to %s\0" as *const u8
                            as *const libc::c_char,
                        quotearg_n(0 as libc::c_int, to),
                        quotearg_n(1 as libc::c_int, bakname),
                    );
                }
            }
        }
        free(bakname as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn move_file(
    mut from: *const libc::c_char,
    mut from_needs_removal: *mut bool,
    mut fromst: *const stat,
    mut to: *const libc::c_char,
    mut mode: mode_t,
    mut backup: bool,
) {
    let mut to_st: stat = stat {
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
    let mut to_errno: libc::c_int = 0;
    to_errno = stat_file(to, &mut to_st);
    if backup {
        create_backup(
            to,
            if to_errno != 0 { 0 as *mut stat } else { &mut to_st },
            0 as libc::c_int != 0,
        );
    }
    if to_errno == 0 {
        insert_file_id(&mut to_st, OVERWRITTEN);
    }
    if !from.is_null() {
        if mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            let mut to_dir_known_to_exist: bool = 0 as libc::c_int != 0;
            let mut buffer: *mut libc::c_char = xmalloc(4096 as libc::c_int as size_t)
                as *mut libc::c_char;
            let mut fd: libc::c_int = 0;
            let mut size: libc::c_int = 0 as libc::c_int;
            let mut i: libc::c_int = 0;
            fd = safe_open(
                from,
                0 as libc::c_int | 0 as libc::c_int,
                0 as libc::c_int as mode_t,
            );
            if fd < 0 as libc::c_int {
                pfatal(
                    b"Can't reopen file %s\0" as *const u8 as *const libc::c_char,
                    quotearg(from),
                );
            }
            loop {
                i = read(
                    fd,
                    buffer.offset(size as isize) as *mut libc::c_void,
                    (4096 as libc::c_int - size) as size_t,
                ) as libc::c_int;
                if !(i > 0 as libc::c_int) {
                    break;
                }
                size += i;
            }
            if i != 0 as libc::c_int || close(fd) != 0 as libc::c_int {
                read_fatal();
            }
            *buffer.offset(size as isize) = 0 as libc::c_int as libc::c_char;
            if !backup {
                if safe_unlink(to) == 0 as libc::c_int {
                    to_dir_known_to_exist = 1 as libc::c_int != 0;
                }
            }
            if safe_symlink(buffer, to) != 0 as libc::c_int {
                if *__errno_location() == 2 as libc::c_int && !to_dir_known_to_exist {
                    makedirs(to);
                }
                if safe_symlink(buffer, to) != 0 as libc::c_int {
                    pfatal(
                        b"Can't create %s %s\0" as *const u8 as *const libc::c_char,
                        b"symbolic link\0" as *const u8 as *const libc::c_char,
                        to,
                    );
                }
            }
            free(buffer as *mut libc::c_void);
            if safe_lstat(to, &mut to_st) != 0 as libc::c_int {
                pfatal(
                    b"Can't get file attributes of %s %s\0" as *const u8
                        as *const libc::c_char,
                    b"symbolic link\0" as *const u8 as *const libc::c_char,
                    to,
                );
            }
            insert_file_id(&mut to_st, CREATED);
        } else {
            let mut current_block_45: u64;
            if debug & 4 as libc::c_int != 0 {
                say(
                    b"Renaming file %s to %s\n\0" as *const u8 as *const libc::c_char,
                    quotearg_n(0 as libc::c_int, from),
                    quotearg_n(1 as libc::c_int, to),
                );
            }
            if safe_rename(from, to) != 0 as libc::c_int {
                let mut to_dir_known_to_exist_0: bool = 0 as libc::c_int != 0;
                if *__errno_location() == 2 as libc::c_int
                    && (to_errno == -(1 as libc::c_int) || to_errno == 2 as libc::c_int)
                {
                    makedirs(to);
                    to_dir_known_to_exist_0 = 1 as libc::c_int != 0;
                    if safe_rename(from, to) == 0 as libc::c_int {
                        current_block_45 = 3513533334412407323;
                    } else {
                        current_block_45 = 11932355480408055363;
                    }
                } else {
                    current_block_45 = 11932355480408055363;
                }
                match current_block_45 {
                    3513533334412407323 => {}
                    _ => {
                        if *__errno_location() == 18 as libc::c_int {
                            let mut tost: stat = stat {
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
                            if !backup {
                                if safe_unlink(to) == 0 as libc::c_int {
                                    to_dir_known_to_exist_0 = 1 as libc::c_int != 0;
                                } else if *__errno_location() != 2 as libc::c_int {
                                    pfatal(
                                        b"Can't remove file %s\0" as *const u8
                                            as *const libc::c_char,
                                        quotearg(to),
                                    );
                                }
                            }
                            copy_file(
                                from,
                                to,
                                &mut tost,
                                0 as libc::c_int,
                                mode,
                                to_dir_known_to_exist_0,
                            );
                            insert_file_id(&mut tost, CREATED);
                            return;
                        }
                        pfatal(
                            b"Can't rename file %s to %s\0" as *const u8
                                as *const libc::c_char,
                            quotearg_n(0 as libc::c_int, from),
                            quotearg_n(1 as libc::c_int, to),
                        );
                    }
                }
            }
            insert_file_id(fromst, CREATED);
            if ((0 as libc::c_int) < to_errno
                || to_errno == 0 as libc::c_int
                    && to_st.st_nlink <= 1 as libc::c_int as libc::c_ulong)
                && !from_needs_removal.is_null()
            {
                *from_needs_removal = 0 as libc::c_int != 0;
            }
        }
    } else if !backup {
        if debug & 4 as libc::c_int != 0 {
            say(
                b"Removing file %s\n\0" as *const u8 as *const libc::c_char,
                quotearg(to),
            );
        }
        if safe_unlink(to) != 0 as libc::c_int && *__errno_location() != 2 as libc::c_int
        {
            pfatal(
                b"Can't remove file %s\0" as *const u8 as *const libc::c_char,
                quotearg(to),
            );
        }
    }
}
pub unsafe extern "C" fn create_file(
    mut file: *const libc::c_char,
    mut open_flags: libc::c_int,
    mut mode: mode_t,
    mut to_dir_known_to_exist: bool,
) -> libc::c_int {
    let mut try_makedirs_errno: libc::c_int = if to_dir_known_to_exist as libc::c_int
        != 0
    {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut fd: libc::c_int = 0;
    mode |= (0o400 as libc::c_int | 0o200 as libc::c_int) as libc::c_uint;
    mode
        &= !(0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
            as libc::c_uint;
    loop {
        if !(0o100 as libc::c_int != 0 && 0o1000 as libc::c_int != 0) {
            close(
                safe_open(
                    file,
                    0o100 as libc::c_int | 0o1 as libc::c_int | 0o1000 as libc::c_int,
                    mode,
                ),
            );
        }
        fd = safe_open(
            file,
            0o100 as libc::c_int | 0o1000 as libc::c_int | open_flags,
            mode,
        );
        if fd < 0 as libc::c_int {
            let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
            if *__errno_location() != try_makedirs_errno {
                pfatal(
                    b"Can't create file %s\0" as *const u8 as *const libc::c_char,
                    quotearg(file),
                );
            }
            f = xstrdup(file);
            makedirs(f);
            free(f as *mut libc::c_void);
            try_makedirs_errno = 0 as libc::c_int;
        }
        if !(fd < 0 as libc::c_int) {
            break;
        }
    }
    return fd;
}
unsafe extern "C" fn copy_to_fd(mut from: *const libc::c_char, mut tofd: libc::c_int) {
    let mut fromfd: libc::c_int = 0;
    let mut i: ssize_t = 0;
    fromfd = safe_open(
        from,
        0 as libc::c_int | 0 as libc::c_int,
        0 as libc::c_int as mode_t,
    );
    if fromfd < 0 as libc::c_int {
        pfatal(
            b"Can't reopen file %s\0" as *const u8 as *const libc::c_char,
            quotearg(from),
        );
    }
    loop {
        i = read(fromfd, buf as *mut libc::c_void, bufsize);
        if !(i != 0 as libc::c_int as libc::c_long) {
            break;
        }
        if i == -(1 as libc::c_int) as ssize_t {
            read_fatal();
        }
        if full_write(tofd, buf as *const libc::c_void, i as size_t)
            != i as libc::c_ulong
        {
            write_fatal();
        }
    }
    if close(fromfd) != 0 as libc::c_int {
        read_fatal();
    }
}
pub unsafe extern "C" fn copy_file(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut tost: *mut stat,
    mut to_flags: libc::c_int,
    mut mode: mode_t,
    mut to_dir_known_to_exist: bool,
) {
    let mut tofd: libc::c_int = 0;
    if debug & 4 as libc::c_int != 0 {
        say(
            b"Copying %s %s to %s\n\0" as *const u8 as *const libc::c_char,
            if mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {
                b"symbolic link\0" as *const u8 as *const libc::c_char
            } else {
                b"file\0" as *const u8 as *const libc::c_char
            },
            quotearg_n(0 as libc::c_int, from),
            quotearg_n(1 as libc::c_int, to),
        );
    }
    if mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        let mut buffer: *mut libc::c_char = xmalloc(
            (4096 as libc::c_int + 1 as libc::c_int) as size_t,
        ) as *mut libc::c_char;
        let mut r: ssize_t = 0;
        r = safe_readlink(from, buffer, 4096 as libc::c_int as size_t);
        if r < 0 as libc::c_int as libc::c_long {
            pfatal(
                b"Can't read %s %s\0" as *const u8 as *const libc::c_char,
                b"symbolic link\0" as *const u8 as *const libc::c_char,
                from,
            );
        }
        *buffer.offset(r as isize) = '\0' as i32 as libc::c_char;
        if safe_symlink(buffer, to) != 0 as libc::c_int {
            pfatal(
                b"Can't create %s %s\0" as *const u8 as *const libc::c_char,
                b"symbolic link\0" as *const u8 as *const libc::c_char,
                to,
            );
        }
        if !tost.is_null() && safe_lstat(to, tost) != 0 as libc::c_int {
            pfatal(
                b"Can't get file attributes of %s %s\0" as *const u8
                    as *const libc::c_char,
                b"symbolic link\0" as *const u8 as *const libc::c_char,
                to,
            );
        }
        free(buffer as *mut libc::c_void);
    } else {
        if mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"S_ISREG (mode)\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                627 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void copy_file(const char *, const char *, struct stat *, int, mode_t, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8868: {
            if mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"S_ISREG (mode)\0" as *const u8 as *const libc::c_char,
                    b"util.c\0" as *const u8 as *const libc::c_char,
                    627 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void copy_file(const char *, const char *, struct stat *, int, mode_t, _Bool)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        tofd = create_file(
            to,
            0o1 as libc::c_int | 0 as libc::c_int | to_flags,
            mode,
            to_dir_known_to_exist,
        );
        copy_to_fd(from, tofd);
        if !tost.is_null() && fstat(tofd, tost) != 0 as libc::c_int {
            pfatal(
                b"Can't get file attributes of %s %s\0" as *const u8
                    as *const libc::c_char,
                b"file\0" as *const u8 as *const libc::c_char,
                to,
            );
        }
        if close(tofd) != 0 as libc::c_int {
            write_fatal();
        }
    };
}
pub unsafe extern "C" fn append_to_file(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    let mut tofd: libc::c_int = 0;
    tofd = safe_open(
        to,
        0o1 as libc::c_int | 0 as libc::c_int | 0o2000 as libc::c_int,
        0 as libc::c_int as mode_t,
    );
    if tofd < 0 as libc::c_int {
        pfatal(
            b"Can't reopen file %s\0" as *const u8 as *const libc::c_char,
            quotearg(to),
        );
    }
    copy_to_fd(from, tofd);
    if close(tofd) != 0 as libc::c_int {
        write_fatal();
    }
}
static mut DEV_NULL: [libc::c_char; 10] = unsafe {
    *::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"/dev/null\0")
};
static mut RCSSUFFIX: [libc::c_char; 3] = unsafe {
    *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b",v\0")
};
static mut CHECKOUT: [libc::c_char; 6] = unsafe {
    *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"co %s\0")
};
static mut CHECKOUT_LOCKED: [libc::c_char; 9] = unsafe {
    *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"co -l %s\0")
};
static mut RCSDIFF1: [libc::c_char; 11] = unsafe {
    *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"rcsdiff %s\0")
};
static mut SCCSPREFIX: [libc::c_char; 3] = unsafe {
    *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"s.\0")
};
static mut GET: [libc::c_char; 5] = unsafe {
    *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"get \0")
};
static mut GET_LOCKED: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"get -e \0")
};
static mut SCCSDIFF1: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"get -p \0")
};
static mut SCCSDIFF2: [libc::c_char; 11] = unsafe {
    *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"|diff - %s\0")
};
static mut CLEARTOOL_CO: [libc::c_char; 23] = unsafe {
    *::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"cleartool co -unr -nc \0")
};
static mut PERFORCE_CO: [libc::c_char; 9] = unsafe {
    *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"p4 edit \0")
};
unsafe extern "C" fn quote_system_arg(
    mut quoted: *mut libc::c_char,
    mut arg: *const libc::c_char,
) -> size_t {
    let mut q: *mut libc::c_char = quotearg_style(shell_quoting_style, arg);
    let mut len: size_t = strlen(q);
    if !quoted.is_null() {
        memcpy(
            quoted as *mut libc::c_void,
            q as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    return len;
}
pub unsafe extern "C" fn version_controller(
    mut filename: *const libc::c_char,
    mut readonly: bool,
    mut filestat: *const stat,
    mut getbuf: *mut *mut libc::c_char,
    mut diffbuf: *mut *mut libc::c_char,
) -> *const libc::c_char {
    let mut cstat: stat = stat {
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
    let mut dir: *mut libc::c_char = dir_name(filename);
    let mut filebase: *mut libc::c_char = base_name(filename);
    let mut dotslash: *const libc::c_char = if *filename as libc::c_int == '-' as i32 {
        b"./\0" as *const u8 as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    let mut dirlen: size_t = (strlen(dir))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut maxfixlen: size_t = (::std::mem::size_of::<[libc::c_char; 6]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut maxtrysize: size_t = dirlen
        .wrapping_add(strlen(filebase))
        .wrapping_add(maxfixlen)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut quotelen: size_t = (quote_system_arg(0 as *mut libc::c_char, dir))
        .wrapping_add(quote_system_arg(0 as *mut libc::c_char, filebase));
    let mut maxgetsize: size_t = (::std::mem::size_of::<[libc::c_char; 23]>()
        as libc::c_ulong)
        .wrapping_add(quotelen)
        .wrapping_add(maxfixlen);
    let mut maxdiffsize: size_t = (::std::mem::size_of::<[libc::c_char; 8]>()
        as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_add((2 as libc::c_int as libc::c_ulong).wrapping_mul(quotelen))
        .wrapping_add(maxfixlen);
    let mut trybuf: *mut libc::c_char = xmalloc(maxtrysize) as *mut libc::c_char;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    sprintf(trybuf, b"%s/\0" as *const u8 as *const libc::c_char, dir);
    sprintf(
        trybuf.offset(dirlen as isize),
        b"RCS/%s%s\0" as *const u8 as *const libc::c_char,
        filebase,
        RCSSUFFIX.as_ptr(),
    );
    if (safe_stat(trybuf, &mut cstat) == 0 as libc::c_int
        || {
            sprintf(
                trybuf.offset(dirlen as isize),
                b"RCS/%s\0" as *const u8 as *const libc::c_char,
                filebase,
            );
            safe_stat(trybuf, &mut cstat) == 0 as libc::c_int
        }
        || {
            sprintf(
                trybuf.offset(dirlen as isize),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                filebase,
                RCSSUFFIX.as_ptr(),
            );
            safe_stat(trybuf, &mut cstat) == 0 as libc::c_int
        })
        && !(!filestat.is_null() && (*filestat).st_dev == cstat.st_dev
            && (*filestat).st_ino == cstat.st_ino)
    {
        if !getbuf.is_null() {
            *getbuf = xmalloc(maxgetsize) as *mut libc::c_char;
            let mut p: *mut libc::c_char = *getbuf;
            sprintf(
                p,
                if readonly as libc::c_int != 0 {
                    CHECKOUT.as_ptr()
                } else {
                    CHECKOUT_LOCKED.as_ptr()
                },
                dotslash,
            );
            p = p.offset(strlen(p) as isize);
            p = p.offset(quote_system_arg(p, filename) as isize);
            *p = '\0' as i32 as libc::c_char;
        }
        if !diffbuf.is_null() {
            *diffbuf = xmalloc(maxdiffsize) as *mut libc::c_char;
            let mut p_0: *mut libc::c_char = *diffbuf;
            sprintf(p_0, RCSDIFF1.as_ptr(), dotslash);
            p_0 = p_0.offset(strlen(p_0) as isize);
            p_0 = p_0.offset(quote_system_arg(p_0, filename) as isize);
            let fresh0 = p_0;
            p_0 = p_0.offset(1);
            *fresh0 = '>' as i32 as libc::c_char;
            strcpy(p_0, DEV_NULL.as_ptr());
        }
        r = b"RCS\0" as *const u8 as *const libc::c_char;
    } else {
        sprintf(
            trybuf.offset(dirlen as isize),
            b"SCCS/%s%s\0" as *const u8 as *const libc::c_char,
            SCCSPREFIX.as_ptr(),
            filebase,
        );
        if safe_stat(trybuf, &mut cstat) == 0 as libc::c_int
            || {
                sprintf(
                    trybuf.offset(dirlen as isize),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    SCCSPREFIX.as_ptr(),
                    filebase,
                );
                safe_stat(trybuf, &mut cstat) == 0 as libc::c_int
            }
        {
            if !getbuf.is_null() {
                *getbuf = xmalloc(maxgetsize) as *mut libc::c_char;
                let mut p_1: *mut libc::c_char = *getbuf;
                sprintf(
                    p_1,
                    if readonly as libc::c_int != 0 {
                        GET.as_ptr()
                    } else {
                        GET_LOCKED.as_ptr()
                    },
                );
                p_1 = p_1.offset(strlen(p_1) as isize);
                p_1 = p_1.offset(quote_system_arg(p_1, trybuf) as isize);
                *p_1 = '\0' as i32 as libc::c_char;
            }
            if !diffbuf.is_null() {
                *diffbuf = xmalloc(maxdiffsize) as *mut libc::c_char;
                let mut p_2: *mut libc::c_char = *diffbuf;
                strcpy(p_2, SCCSDIFF1.as_ptr());
                p_2 = p_2
                    .offset(
                        (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                p_2 = p_2.offset(quote_system_arg(p_2, trybuf) as isize);
                sprintf(p_2, SCCSDIFF2.as_ptr(), dotslash);
                p_2 = p_2.offset(strlen(p_2) as isize);
                p_2 = p_2.offset(quote_system_arg(p_2, filename) as isize);
                let fresh1 = p_2;
                p_2 = p_2.offset(1);
                *fresh1 = '>' as i32 as libc::c_char;
                strcpy(p_2, DEV_NULL.as_ptr());
            }
            r = b"SCCS\0" as *const u8 as *const libc::c_char;
        } else if !readonly && !filestat.is_null()
            && {
                sprintf(
                    trybuf.offset(dirlen as isize),
                    b"%s@@\0" as *const u8 as *const libc::c_char,
                    filebase,
                );
                safe_stat(trybuf, &mut cstat) == 0 as libc::c_int
            }
            && cstat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            if !getbuf.is_null() {
                *getbuf = xmalloc(maxgetsize) as *mut libc::c_char;
                let mut p_3: *mut libc::c_char = *getbuf;
                strcpy(p_3, CLEARTOOL_CO.as_ptr());
                p_3 = p_3
                    .offset(
                        (::std::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                p_3 = p_3.offset(quote_system_arg(p_3, filename) as isize);
                *p_3 = '\0' as i32 as libc::c_char;
            }
            if !diffbuf.is_null() {
                *diffbuf = 0 as *mut libc::c_char;
            }
            r = b"ClearCase\0" as *const u8 as *const libc::c_char;
        } else if !readonly && !filestat.is_null()
            && (!(getenv(b"P4PORT\0" as *const u8 as *const libc::c_char)).is_null()
                || !(getenv(b"P4USER\0" as *const u8 as *const libc::c_char)).is_null()
                || !(getenv(b"P4CONFIG\0" as *const u8 as *const libc::c_char))
                    .is_null())
        {
            if !getbuf.is_null() {
                *getbuf = xmalloc(maxgetsize) as *mut libc::c_char;
                let mut p_4: *mut libc::c_char = *getbuf;
                strcpy(p_4, PERFORCE_CO.as_ptr());
                p_4 = p_4
                    .offset(
                        (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                p_4 = p_4.offset(quote_system_arg(p_4, filename) as isize);
                *p_4 = '\0' as i32 as libc::c_char;
            }
            if !diffbuf.is_null() {
                *diffbuf = 0 as *mut libc::c_char;
            }
            r = b"Perforce\0" as *const u8 as *const libc::c_char;
        }
    }
    free(trybuf as *mut libc::c_void);
    free(filebase as *mut libc::c_void);
    free(dir as *mut libc::c_void);
    return r;
}
pub unsafe extern "C" fn version_get(
    mut filename: *const libc::c_char,
    mut cs: *const libc::c_char,
    mut exists: bool,
    mut readonly: bool,
    mut getbuf: *const libc::c_char,
    mut filestat: *mut stat,
) -> bool {
    if patch_get < 0 as libc::c_int {
        ask(
            b"Get file %s from %s%s? [y] \0" as *const u8 as *const libc::c_char,
            quotearg(filename),
            cs,
            if readonly as libc::c_int != 0 {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b" with lock\0" as *const u8 as *const libc::c_char
            },
        );
        if *buf as libc::c_int == 'n' as i32 {
            return 0 as libc::c_int != 0;
        }
    }
    if dry_run {
        if !exists {
            fatal(
                b"can't do dry run on nonexistent version-controlled file %s; invoke '%s' and try again\0"
                    as *const u8 as *const libc::c_char,
                quotearg(filename),
                getbuf,
            );
        }
    } else {
        if verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint {
            say(
                b"Getting file %s from %s%s...\n\0" as *const u8 as *const libc::c_char,
                quotearg(filename),
                cs,
                if readonly as libc::c_int != 0 {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b" with lock\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if systemic(getbuf) != 0 as libc::c_int {
            fatal(
                b"Can't get file %s from %s\0" as *const u8 as *const libc::c_char,
                quotearg(filename),
                cs,
            );
        }
        if safe_stat(filename, filestat) != 0 as libc::c_int {
            pfatal(b"%s\0" as *const u8 as *const libc::c_char, quotearg(filename));
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn savebuf(
    mut s: *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    if size == 0 {
        return 0 as *mut libc::c_char;
    }
    rv = malloc(size) as *mut libc::c_char;
    if rv.is_null() {
        if !using_plan_a {
            xalloc_die();
        }
    } else {
        memcpy(rv as *mut libc::c_void, s as *const libc::c_void, size);
    }
    return rv;
}
pub unsafe extern "C" fn savestr(mut s: *const libc::c_char) -> *mut libc::c_char {
    return savebuf(s, (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong));
}
pub unsafe extern "C" fn remove_prefix(mut p: *mut libc::c_char, mut prefixlen: size_t) {
    let mut s: *const libc::c_char = p.offset(prefixlen as isize);
    loop {
        let fresh2 = s;
        s = s.offset(1);
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = *fresh2;
        if !(*fresh3 != 0) {
            break;
        }
    };
}
pub unsafe extern "C" fn format_linenum(
    mut numbuf: *mut libc::c_char,
    mut n: lin,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = numbuf
        .offset(
            (::std::mem::size_of::<lin>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_div(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        );
    *p = '\0' as i32 as libc::c_char;
    if n < 0 as libc::c_int as libc::c_long {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 - (n % 10 as libc::c_int as libc::c_long) as libc::c_int)
                as libc::c_char;
            n /= 10 as libc::c_int as libc::c_long;
            if !(n != 0 as libc::c_int as libc::c_long) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 + (n % 10 as libc::c_int as libc::c_long) as libc::c_int)
                as libc::c_char;
            n /= 10 as libc::c_int as libc::c_long;
            if !(n != 0 as libc::c_int as libc::c_long) {
                break;
            }
        }
    }
    return p;
}
pub unsafe extern "C" fn fatal(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(stderr, b"%s: **** \0" as *const u8 as *const libc::c_char, program_name);
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    putc('\n' as i32, stderr);
    fflush(stderr);
    fatal_exit(0 as libc::c_int);
}
pub unsafe extern "C" fn xalloc_die() -> ! {
    fatal(b"out of memory\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn read_fatal() -> ! {
    pfatal(b"read error\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn write_fatal() -> ! {
    pfatal(b"write error\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn pfatal(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut errnum: libc::c_int = *__errno_location();
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(stderr, b"%s: **** \0" as *const u8 as *const libc::c_char, program_name);
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    fflush(stderr);
    *__errno_location() = errnum;
    perror(b" \0" as *const u8 as *const libc::c_char);
    fflush(stderr);
    fatal_exit(0 as libc::c_int);
}
unsafe extern "C" fn vsay(
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    vfprintf(stdout, format, args.as_va_list());
    fflush(stdout);
}
pub unsafe extern "C" fn say(mut format: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vsay(format, args_0.as_va_list());
}
pub unsafe extern "C" fn ask(mut format: *const libc::c_char, mut args: ...) {
    static mut ttyfd: libc::c_int = -(2 as libc::c_int);
    let mut r: ssize_t = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stdout, format, args_0.as_va_list());
    fflush(stdout);
    if ttyfd == -(2 as libc::c_int) {
        ttyfd = if posixly_correct as libc::c_int != 0 || isatty(1 as libc::c_int) != 0 {
            open(b"/dev/tty\0" as *const u8 as *const libc::c_char, 0 as libc::c_int)
        } else {
            -(1 as libc::c_int)
        };
    }
    if ttyfd < 0 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        *buf.offset(0 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
        *buf.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        let mut s: size_t = 0 as libc::c_int as size_t;
        loop {
            r = read(
                ttyfd,
                buf.offset(s as isize) as *mut libc::c_void,
                bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_sub(s),
            );
            if !(r as libc::c_ulong
                == bufsize
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(s)
                && *buf
                    .offset(
                        bufsize.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int != '\n' as i32)
            {
                break;
            }
            s = bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            bufsize = (bufsize as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            buf = realloc(buf as *mut libc::c_void, bufsize) as *mut libc::c_char;
            if buf.is_null() {
                xalloc_die();
            }
        }
        if r == 0 as libc::c_int as libc::c_long {
            printf(b"EOF\n\0" as *const u8 as *const libc::c_char);
        } else if r < 0 as libc::c_int as libc::c_long {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"tty read failed\0" as *const u8 as *const libc::c_char,
            );
            let mut __x: libc::c_int = close(ttyfd);
            ttyfd = -(1 as libc::c_int);
            r = 0 as libc::c_int as ssize_t;
        }
        *buf
            .offset(
                s.wrapping_add(r as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    };
}
pub unsafe extern "C" fn ok_to_reverse(
    mut format: *const libc::c_char,
    mut args: ...
) -> bool {
    let mut r: bool = 0 as libc::c_int != 0;
    if noreverse as libc::c_int != 0
        || !(force as libc::c_int != 0
            && verbosity as libc::c_uint == SILENT as libc::c_int as libc::c_uint)
    {
        let mut args_0: ::std::ffi::VaListImpl;
        args_0 = args.clone();
        vsay(format, args_0.as_va_list());
    }
    if noreverse {
        say(b"  Skipping patch.\n\0" as *const u8 as *const libc::c_char);
        skip_rest_of_patch = 1 as libc::c_int != 0;
    } else if force {
        if verbosity as libc::c_uint != SILENT as libc::c_int as libc::c_uint {
            say(b"  Applying it anyway.\n\0" as *const u8 as *const libc::c_char);
        }
    } else if batch {
        say(
            if reverse as libc::c_int != 0 {
                b"  Ignoring -R.\n\0" as *const u8 as *const libc::c_char
            } else {
                b"  Assuming -R.\n\0" as *const u8 as *const libc::c_char
            },
        );
        r = 1 as libc::c_int != 0;
    } else {
        ask(
            if reverse as libc::c_int != 0 {
                b"  Ignore -R? [n] \0" as *const u8 as *const libc::c_char
            } else {
                b"  Assume -R? [n] \0" as *const u8 as *const libc::c_char
            },
        );
        r = *buf as libc::c_int == 'y' as i32;
        if !r {
            ask(b"Apply anyway? [n] \0" as *const u8 as *const libc::c_char);
            if *buf as libc::c_int != 'y' as i32 {
                if verbosity as libc::c_uint != SILENT as libc::c_int as libc::c_uint {
                    say(b"Skipping patch.\n\0" as *const u8 as *const libc::c_char);
                }
                skip_rest_of_patch = 1 as libc::c_int != 0;
            }
        }
    }
    return r;
}
static mut sigs: [libc::c_int; 6] = [
    1 as libc::c_int,
    13 as libc::c_int,
    15 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    2 as libc::c_int,
];
static mut initial_signal_mask: sigset_t = sigset_t { __val: [0; 16] };
static mut signals_to_block: sigset_t = sigset_t { __val: [0; 16] };
pub unsafe extern "C" fn set_signals(mut reset: bool) {
    let mut i: libc::c_int = 0;
    let mut initial_act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_2 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut fatal_act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_2 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    fatal_act
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(libc::c_int) -> !>,
        __sighandler_t,
    >(Some(fatal_exit as unsafe extern "C" fn(libc::c_int) -> !));
    sigemptyset(&mut fatal_act.sa_mask);
    fatal_act.sa_flags = 0 as libc::c_int;
    if !reset {
        signal(17 as libc::c_int, None);
        sigemptyset(&mut signals_to_block);
        i = 0 as libc::c_int;
        while i
            < (::std::mem::size_of::<[libc::c_int; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int
        {
            let mut ignoring_signal: bool = false;
            if !(sigaction(sigs[i as usize], 0 as *mut sigaction, &mut initial_act)
                != 0 as libc::c_int)
            {
                ignoring_signal = initial_act.__sigaction_handler.sa_handler
                    == ::std::mem::transmute::<
                        libc::intptr_t,
                        __sighandler_t,
                    >(1 as libc::c_int as libc::intptr_t);
                if !ignoring_signal {
                    sigaddset(&mut signals_to_block, sigs[i as usize]);
                    sigaction(sigs[i as usize], &mut fatal_act, 0 as *mut sigaction);
                }
            }
            i += 1;
            i;
        }
    } else {
        sigprocmask(2 as libc::c_int, &mut initial_signal_mask, 0 as *mut sigset_t);
    };
}
pub unsafe extern "C" fn ignore_signals() {
    sigprocmask(0 as libc::c_int, &mut signals_to_block, &mut initial_signal_mask);
}
pub unsafe extern "C" fn exit_with_signal(mut sig: libc::c_int) -> ! {
    let mut s: sigset_t = sigset_t { __val: [0; 16] };
    signal(sig, None);
    sigemptyset(&mut s);
    sigaddset(&mut s, sig);
    sigprocmask(1 as libc::c_int, &mut s, 0 as *mut sigset_t);
    raise(sig);
    exit(2 as libc::c_int);
}
pub unsafe extern "C" fn systemic(mut command: *const libc::c_char) -> libc::c_int {
    if debug & 8 as libc::c_int != 0 {
        say(b"+ %s\n\0" as *const u8 as *const libc::c_char, command);
    }
    fflush(stdout);
    return system(command);
}
unsafe extern "C" fn replace_slashes(
    mut filename: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_location_replaced: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut component_start: *const libc::c_char = 0 as *const libc::c_char;
    f = filename.offset(0 as libc::c_int as isize);
    while *f as libc::c_int == '/' as i32 {
        f = f.offset(1);
        f;
    }
    component_start = f;
    while *f != 0 {
        if *f as libc::c_int == '/' as i32 {
            let mut slash: *mut libc::c_char = f;
            while *f.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                f = f.offset(1);
                f;
            }
            if *f.offset(1 as libc::c_int as isize) == 0 {
                break;
            }
            if !(slash.offset_from(component_start) as libc::c_long
                <= 2 as libc::c_int as libc::c_long
                && *component_start.offset(0 as libc::c_int as isize) as libc::c_int
                    == '.' as i32
                && *slash.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '.' as i32)
            {
                *slash = '\0' as i32 as libc::c_char;
                last_location_replaced = slash;
            }
            component_start = f.offset(1 as libc::c_int as isize);
        }
        f = f.offset(1);
        f;
    }
    return last_location_replaced;
}
unsafe extern "C" fn makedirs(mut name: *const libc::c_char) {
    let mut filename: *mut libc::c_char = xstrdup(name);
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flim: *mut libc::c_char = replace_slashes(filename);
    if !flim.is_null() {
        f = filename;
        while f <= flim {
            if *f == 0 {
                safe_mkdir(
                    filename,
                    (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as mode_t,
                );
                *f = '/' as i32 as libc::c_char;
            }
            f = f.offset(1);
            f;
        }
    }
    free(filename as *mut libc::c_void);
}
pub unsafe extern "C" fn removedirs(mut name: *const libc::c_char) {
    let mut filename: *mut libc::c_char = xstrdup(name);
    let mut i: size_t = 0;
    i = strlen(filename);
    while i != 0 as libc::c_int as libc::c_ulong {
        if *filename.offset(i as isize) as libc::c_int == '/' as i32
            && !(*filename
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '/' as i32
                || *filename
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int == '.' as i32
                    && (i == 1 as libc::c_int as libc::c_ulong
                        || *filename
                            .offset(
                                i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '/' as i32
                        || *filename
                            .offset(
                                i.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
                            ) as libc::c_int == '.' as i32
                            && (i == 2 as libc::c_int as libc::c_ulong
                                || *filename
                                    .offset(
                                        i.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize,
                                    ) as libc::c_int == '/' as i32)))
        {
            *filename.offset(i as isize) = '\0' as i32 as libc::c_char;
            if safe_rmdir(filename) == 0 as libc::c_int
                && verbosity as libc::c_uint == VERBOSE as libc::c_int as libc::c_uint
            {
                say(
                    b"Removed empty directory %s\n\0" as *const u8
                        as *const libc::c_char,
                    quotearg(filename),
                );
            }
            *filename.offset(i as isize) = '/' as i32 as libc::c_char;
        }
        i = i.wrapping_sub(1);
        i;
    }
    free(filename as *mut libc::c_void);
}
static mut initial_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
pub unsafe extern "C" fn init_time() {
    gettime(&mut initial_time);
}
unsafe extern "C" fn parse_c_string(
    mut s: *const libc::c_char,
    mut endp: *mut *const libc::c_char,
) -> *mut libc::c_char {
    let mut u: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if *s as libc::c_int == '"' as i32 {} else {
        __assert_fail(
            b"*s == '\"'\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            1346 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"char *parse_c_string(const char *, const char **)\0"))
                .as_ptr(),
        );
    }
    'c_6708: {
        if *s as libc::c_int == '"' as i32 {} else {
            __assert_fail(
                b"*s == '\"'\0" as *const u8 as *const libc::c_char,
                b"util.c\0" as *const u8 as *const libc::c_char,
                1346 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"char *parse_c_string(const char *, const char **)\0"))
                    .as_ptr(),
            );
        }
    };
    s = s.offset(1);
    s;
    v = xmalloc(strlen(s)) as *mut libc::c_char;
    u = v;
    loop {
        let fresh4 = s;
        s = s.offset(1);
        let mut c: libc::c_char = *fresh4;
        match c as libc::c_int {
            0 => {
                break;
            }
            34 => {
                let fresh5 = v;
                v = v.offset(1);
                *fresh5 = 0 as libc::c_int as libc::c_char;
                v = realloc(
                    u as *mut libc::c_void,
                    v.offset_from(u) as libc::c_long as libc::c_ulong,
                ) as *mut libc::c_char;
                if !v.is_null() {
                    u = v;
                }
                if !endp.is_null() {
                    *endp = s;
                }
                return u;
            }
            92 => {
                let fresh7 = s;
                s = s.offset(1);
                c = *fresh7;
                match c as libc::c_int {
                    97 => {
                        c = '\u{7}' as i32 as libc::c_char;
                    }
                    98 => {
                        c = '\u{8}' as i32 as libc::c_char;
                    }
                    102 => {
                        c = '\u{c}' as i32 as libc::c_char;
                    }
                    110 => {
                        c = '\n' as i32 as libc::c_char;
                    }
                    114 => {
                        c = '\r' as i32 as libc::c_char;
                    }
                    116 => {
                        c = '\t' as i32 as libc::c_char;
                    }
                    118 => {
                        c = '\u{b}' as i32 as libc::c_char;
                    }
                    92 | 34 => {}
                    48 | 49 | 50 | 51 => {
                        let mut acc: libc::c_int = (c as libc::c_int - '0' as i32)
                            << 6 as libc::c_int;
                        let fresh8 = s;
                        s = s.offset(1);
                        c = *fresh8;
                        if (c as libc::c_int) < '0' as i32
                            || c as libc::c_int > '7' as i32
                        {
                            break;
                        }
                        acc |= (c as libc::c_int - '0' as i32) << 3 as libc::c_int;
                        let fresh9 = s;
                        s = s.offset(1);
                        c = *fresh9;
                        if (c as libc::c_int) < '0' as i32
                            || c as libc::c_int > '7' as i32
                        {
                            break;
                        }
                        acc |= c as libc::c_int - '0' as i32;
                        c = acc as libc::c_char;
                    }
                    _ => {
                        break;
                    }
                }
                let fresh10 = v;
                v = v.offset(1);
                *fresh10 = c;
            }
            _ => {
                let fresh6 = v;
                v = v.offset(1);
                *fresh6 = c;
            }
        }
    }
    free(u as *mut libc::c_void);
    if !endp.is_null() {
        *endp = s;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn strip_leading_slashes(
    mut name: *mut libc::c_char,
    mut strip_leading: libc::c_int,
) -> bool {
    let mut s: libc::c_int = strip_leading;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    n = name;
    p = n;
    while *p != 0 {
        if *p as libc::c_int == '/' as i32 {
            while *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                p = p.offset(1);
                p;
            }
            if strip_leading < 0 as libc::c_int
                || {
                    s -= 1;
                    s >= 0 as libc::c_int
                }
            {
                n = p.offset(1 as libc::c_int as isize);
            }
        }
        p = p.offset(1);
        p;
    }
    if (strip_leading < 0 as libc::c_int || s <= 0 as libc::c_int)
        && *n as libc::c_int != 0
    {
        memmove(
            name as *mut libc::c_void,
            n as *const libc::c_void,
            (strlen(n)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0
    };
}
pub unsafe extern "C" fn fetchname(
    mut at: *const libc::c_char,
    mut strip_leading: libc::c_int,
    mut pname: *mut *mut libc::c_char,
    mut ptimestr: *mut *mut libc::c_char,
    mut pstamp: *mut timespec,
) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut timestr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stamp: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    stamp.tv_sec = -(1 as libc::c_int) as __time_t;
    stamp.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    while 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*at as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        at = at.offset(1);
        at;
    }
    if debug & 128 as libc::c_int != 0 {
        say(
            b"fetchname %s %d\n\0" as *const u8 as *const libc::c_char,
            at,
            strip_leading,
        );
    }
    if *at as libc::c_int == '"' as i32 {
        name = parse_c_string(at, &mut t);
        if name.is_null() {
            if debug & 128 as libc::c_int != 0 {
                say(
                    b"ignoring malformed filename %s\n\0" as *const u8
                        as *const libc::c_char,
                    quotearg(at),
                );
            }
            return;
        }
    } else {
        t = at;
        while *t != 0 {
            if 1 as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*t as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut u: *const libc::c_char = t;
                while *u as libc::c_int != '\t' as i32
                    && (1 as libc::c_int != 0
                        && *(*__ctype_b_loc())
                            .offset(
                                *u.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
                {
                    u = u.offset(1);
                    u;
                }
                if !(*u as libc::c_int != '\t' as i32
                    && !(strchr(
                        u.offset(1 as libc::c_int as isize),
                        if !pstamp.is_null() { '\t' as i32 } else { '\n' as i32 },
                    ))
                        .is_null())
                {
                    break;
                }
            }
            t = t.offset(1);
            t;
        }
        name = xmemdup0(
            at as *const libc::c_void,
            t.offset_from(at) as libc::c_long as size_t,
        );
    }
    if strcmp(name, b"/dev/null\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        free(name as *mut libc::c_void);
        if !pstamp.is_null() {
            (*pstamp).tv_sec = 0 as libc::c_int as __time_t;
            (*pstamp).tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        }
        return;
    }
    if !strip_leading_slashes(name, strip_leading) {
        free(name as *mut libc::c_void);
        return;
    }
    if !ptimestr.is_null() {
        let mut u_0: *const libc::c_char = t.offset(strlen(t) as isize);
        if u_0 != t
            && *u_0.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\n' as i32
        {
            u_0 = u_0.offset(-1);
            u_0;
        }
        if u_0 != t
            && *u_0.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\r' as i32
        {
            u_0 = u_0.offset(-1);
            u_0;
        }
        timestr = xmemdup0(
            t as *const libc::c_void,
            u_0.offset_from(t) as libc::c_long as size_t,
        );
    }
    if *t as libc::c_int != '\n' as i32 {
        if pstamp.is_null() {
            free(name as *mut libc::c_void);
            free(timestr as *mut libc::c_void);
            return;
        }
        if set_time as libc::c_int | set_utc as libc::c_int != 0 {
            parse_datetime(&mut stamp, t, &mut initial_time);
        } else {
            let lower: timespec = {
                let mut init = timespec {
                    tv_sec: -(25 as libc::c_long) * 60 as libc::c_int as libc::c_long
                        * 60 as libc::c_int as libc::c_long,
                    tv_nsec: 0,
                };
                init
            };
            let upper: timespec = {
                let mut init = timespec {
                    tv_sec: 26 as libc::c_long * 60 as libc::c_int as libc::c_long
                        * 60 as libc::c_int as libc::c_long,
                    tv_nsec: 0,
                };
                init
            };
            if parse_datetime(&mut stamp, t, &mut initial_time) as libc::c_int != 0
                && timespec_cmp(stamp, lower) > 0 as libc::c_int
                && timespec_cmp(stamp, upper) < 0 as libc::c_int
            {
                stamp.tv_sec = 0 as libc::c_int as __time_t;
                stamp.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            }
        }
    }
    free(*pname as *mut libc::c_void);
    *pname = name;
    if !ptimestr.is_null() {
        free(*ptimestr as *mut libc::c_void);
        *ptimestr = timestr;
    }
    if !pstamp.is_null() {
        *pstamp = stamp;
    }
}
pub unsafe extern "C" fn parse_name(
    mut s: *const libc::c_char,
    mut strip_leading: libc::c_int,
    mut endp: *mut *const libc::c_char,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    while 1 as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*s as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == '"' as i32 {
        ret = parse_c_string(s, endp);
        if ret.is_null() {
            return 0 as *mut libc::c_char;
        }
    } else {
        let mut t: *const libc::c_char = 0 as *const libc::c_char;
        t = s;
        while *t as libc::c_int != 0
            && !(1 as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*t as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            t = t.offset(1);
            t;
        }
        ret = xmemdup0(
            s as *const libc::c_void,
            t.offset_from(s) as libc::c_long as size_t,
        );
        if !endp.is_null() {
            *endp = t;
        }
    }
    if !strip_leading_slashes(ret, strip_leading) {
        free(ret as *mut libc::c_void);
        ret = 0 as *mut libc::c_char;
    }
    return ret;
}
pub unsafe extern "C" fn Fseek(
    mut stream: *mut FILE,
    mut offset: file_offset,
    mut ptrname: libc::c_int,
) {
    if fseek(stream, offset, ptrname) != 0 as libc::c_int {
        pfatal(b"fseek\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn try_safe_open(
    mut template: *mut libc::c_char,
    mut __args: *mut libc::c_void,
) -> libc::c_int {
    let mut args: *mut try_safe_open_args = __args as *mut try_safe_open_args;
    let mut try_makedirs_errno: libc::c_int = 2 as libc::c_int;
    let mut fd: libc::c_int = 0;
    loop {
        fd = safe_open(
            template,
            0o100 as libc::c_int | 0o200 as libc::c_int | (*args).flags,
            (*args).mode,
        );
        if !(fd < 0 as libc::c_int && *__errno_location() == try_makedirs_errno) {
            break;
        }
        makedirs(template);
        try_makedirs_errno = 0 as libc::c_int;
    }
    return fd;
}
pub unsafe extern "C" fn make_tempfile(
    mut name: *mut *const libc::c_char,
    mut letter: libc::c_char,
    mut real_name: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut template: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args: try_safe_open_args = {
        let mut init = try_safe_open_args {
            flags: flags,
            mode: mode,
        };
        init
    };
    let mut fd: libc::c_int = 0;
    if !real_name.is_null() && !dry_run {
        let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut basename: *mut libc::c_char = 0 as *mut libc::c_char;
        dirname = dir_name(real_name);
        basename = base_name(real_name);
        template = xmalloc(
            (strlen(dirname))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(basename))
                .wrapping_add(9 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            template,
            b"%s/%s.%cXXXXXX\0" as *const u8 as *const libc::c_char,
            dirname,
            basename,
            letter as libc::c_int,
        );
        free(dirname as *mut libc::c_void);
        free(basename as *mut libc::c_void);
    } else {
        let mut tmpdir: *const libc::c_char = 0 as *const libc::c_char;
        tmpdir = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
        if tmpdir.is_null() {
            tmpdir = getenv(b"TMP\0" as *const u8 as *const libc::c_char);
        }
        if tmpdir.is_null() {
            tmpdir = getenv(b"TEMP\0" as *const u8 as *const libc::c_char);
        }
        if tmpdir.is_null() {
            tmpdir = b"/tmp\0" as *const u8 as *const libc::c_char;
        }
        template = xmalloc(
            (strlen(tmpdir)).wrapping_add(10 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            template,
            b"%s/p%cXXXXXX\0" as *const u8 as *const libc::c_char,
            tmpdir,
            letter as libc::c_int,
        );
    }
    fd = try_tempname(
        template,
        0 as libc::c_int,
        &mut args as *mut try_safe_open_args as *mut libc::c_void,
        Some(
            try_safe_open
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    *name = template;
    return fd;
}
pub unsafe extern "C" fn stat_file(
    mut filename: *const libc::c_char,
    mut st: *mut stat,
) -> libc::c_int {
    let mut xstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    > = if follow_symlinks as libc::c_int != 0 {
        Some(
            safe_stat
                as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        )
    } else {
        Some(
            safe_lstat
                as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        )
    };
    return if xstat.unwrap()(filename, st) == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        *__errno_location()
    };
}
pub unsafe extern "C" fn filename_is_safe(mut name: *const libc::c_char) -> bool {
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || 0 as libc::c_int != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    while *name != 0 {
        if *name as libc::c_int == '.' as i32
            && {
                name = name.offset(1);
                *name as libc::c_int == '.' as i32
            }
            && {
                name = name.offset(1);
                *name == 0 || *name as libc::c_int == '/' as i32
            }
        {
            return 0 as libc::c_int != 0;
        }
        while *name as libc::c_int != 0 && !(*name as libc::c_int == '/' as i32) {
            name = name.offset(1);
            name;
        }
        while *name as libc::c_int == '/' as i32 {
            name = name.offset(1);
            name;
        }
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn cwd_is_root(mut name: *const libc::c_char) -> bool {
    let mut prefix_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let vla = prefix_len.wrapping_add(2 as libc::c_int as libc::c_uint) as usize;
    let mut root: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
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
    let mut root_dev: dev_t = 0;
    let mut root_ino: ino_t = 0;
    memcpy(
        root.as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        prefix_len as libc::c_ulong,
    );
    *root.as_mut_ptr().offset(prefix_len as isize) = '/' as i32 as libc::c_char;
    *root
        .as_mut_ptr()
        .offset(
            prefix_len.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    if stat(root.as_mut_ptr(), &mut st) != 0 {
        return 0 as libc::c_int != 0;
    }
    root_dev = st.st_dev;
    root_ino = st.st_ino;
    if stat(b".\0" as *const u8 as *const libc::c_char, &mut st) != 0 {
        return 0 as libc::c_int != 0;
    }
    return root_dev == st.st_dev && root_ino == st.st_ino;
}
