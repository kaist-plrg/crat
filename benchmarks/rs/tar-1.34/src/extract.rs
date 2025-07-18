use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type exclist;
    fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstatat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkdirat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: __mode_t,
    ) -> libc::c_int;
    fn __xmknodat(
        __ver: libc::c_int,
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> libc::c_int;
    fn mkfifoat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: __mode_t,
    ) -> libc::c_int;
    fn rpl_fchmodat(
        fd: libc::c_int,
        file: *const libc::c_char,
        mode: mode_t,
        flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn fchownat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn geteuid() -> __uid_t;
    fn linkat(
        __fromfd: libc::c_int,
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn symlinkat(
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
    ) -> libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn chmod_error_details(name: *const libc::c_char, mode: mode_t);
    fn chown_error_details(name: *const libc::c_char, uid: uid_t, gid: gid_t);
    fn close_error(_: *const libc::c_char);
    fn link_error(_: *const libc::c_char, _: *const libc::c_char);
    fn mkdir_error(_: *const libc::c_char);
    fn mkfifo_error(_: *const libc::c_char);
    fn mknod_error(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn stat_error(_: *const libc::c_char);
    fn stat_warn(_: *const libc::c_char);
    fn symlink_error(_: *const libc::c_char, _: *const libc::c_char);
    fn unlink_error(_: *const libc::c_char);
    fn utime_error(_: *const libc::c_char);
    fn write_error_details(_: *const libc::c_char, _: size_t, _: size_t);
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn fdutimensat(
        fd: libc::c_int,
        dir: libc::c_int,
        name: *const libc::c_char,
        _: *const timespec,
        atflag: libc::c_int,
    ) -> libc::c_int;
    static mut absolute_names_option: bool;
    static mut backup_option: bool;
    static mut dereference_option: bool;
    static mut incremental_option: bool;
    static mut interactive_option: bool;
    static mut old_files_option: old_files;
    static mut keep_directory_symlink_option: bool;
    static mut one_file_system_option: bool;
    static mut recursive_unlink_option: bool;
    static mut same_owner_option: libc::c_int;
    static mut same_permissions_option: libc::c_int;
    static mut xattrs_option: libc::c_int;
    static mut to_stdout_option: bool;
    static mut touch_option: bool;
    static mut to_command_option: *mut libc::c_char;
    static mut verbose_option: libc::c_int;
    static mut volume_start_time: timespec;
    static mut current_stat_info: tar_stat_info;
    static mut root_device: dev_t;
    static mut delay_directory_restore_option: bool;
    fn available_space_after(pointer: *mut block) -> size_t;
    fn find_next_block() -> *mut block;
    fn set_next_block_after(block: *mut block);
    fn mv_begin_read(st: *mut tar_stat_info);
    fn mv_end();
    fn mv_size_left(size: off_t);
    fn purge_directory(directory_name: *const libc::c_char);
    static mut current_header: *mut block;
    fn tartime(t: timespec, full_time: bool) -> *const libc::c_char;
    fn print_for_mkdir(dirname: *mut libc::c_char, length: libc::c_int, mode: mode_t);
    fn print_header(st: *mut tar_stat_info, blk: *mut block, block_ordinal: off_t);
    fn skip_file(size: off_t);
    fn skip_member();
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    static mut chdir_fd: libc::c_int;
    fn code_timespec(ts: timespec, sbuf: *mut libc::c_char) -> *const libc::c_char;
    fn blocking_write(
        fd: libc::c_int,
        buf: *const libc::c_void,
        count: size_t,
    ) -> size_t;
    fn must_be_dot_or_slash(_: *const libc::c_char) -> bool;
    fn xheader_xattr_copy(
        st: *const tar_stat_info,
        vals: *mut *mut xattr_array,
        sz: *mut size_t,
    );
    static mut chdir_current: libc::c_int;
    fn sparse_extract_file(
        fd: libc::c_int,
        st: *mut tar_stat_info,
        size: *mut off_t,
    ) -> dump_status;
    fn xattrs_xattrs_set(
        st: *const tar_stat_info,
        file_name: *const libc::c_char,
        typeflag: libc::c_char,
        later_run: libc::c_int,
    );
    fn xattrs_acls_set(
        st: *const tar_stat_info,
        file_name: *const libc::c_char,
        typeflag: libc::c_char,
    );
    fn xattrs_selinux_set(
        st: *const tar_stat_info,
        file_name: *const libc::c_char,
        typeflag: libc::c_char,
    );
    fn sys_wait_command();
    fn remove_any_file(
        file_name: *const libc::c_char,
        option: remove_option,
    ) -> libc::c_int;
    fn deref_stat(name: *const libc::c_char, buf: *mut stat) -> libc::c_int;
    fn sys_exec_command(
        file_name: *mut libc::c_char,
        typechar: libc::c_int,
        st: *mut tar_stat_info,
    ) -> libc::c_int;
    fn tar_timespec_cmp(a: timespec, b: timespec) -> libc::c_int;
    fn stat_diag(name: *const libc::c_char);
    static mut warning_option: libc::c_int;
    fn contains_dot_dot(name: *const libc::c_char) -> bool;
    fn undo_last_backup();
    fn sparse_member_p(st: *mut tar_stat_info) -> bool;
    fn maybe_backup_file(
        file_name: *const libc::c_char,
        this_is_the_archive: bool,
    ) -> bool;
    fn chdir_do(dir: libc::c_int);
    fn xheader_xattr_free(vals: *mut xattr_array, sz: size_t);
    fn confirm(
        message_action: *const libc::c_char,
        name: *const libc::c_char,
    ) -> libc::c_int;
    static mut fatal_exit_hook: Option::<unsafe extern "C" fn() -> ()>;
    fn gettime(_: *mut timespec);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
}
pub type __uintmax_t = libc::c_ulong;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
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
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_header {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub chksum: [libc::c_char; 8],
    pub typeflag: libc::c_char,
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub devmajor: [libc::c_char; 8],
    pub devminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse {
    pub offset: [libc::c_char; 12],
    pub numbytes: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_header {
    pub sp: [sparse; 21],
    pub isextended: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldgnu_header {
    pub unused_pad1: [libc::c_char; 345],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
    pub offset: [libc::c_char; 12],
    pub longnames: [libc::c_char; 4],
    pub unused_pad2: libc::c_char,
    pub sp: [sparse; 4],
    pub isextended: libc::c_char,
    pub realsize: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_header {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub chksum: [libc::c_char; 8],
    pub typeflag: libc::c_char,
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub devmajor: [libc::c_char; 8],
    pub devminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 131],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_in_header {
    pub fill: [libc::c_char; 345],
    pub prefix: [libc::c_char; 1],
    pub fill2: libc::c_char,
    pub fill3: [libc::c_char; 8],
    pub isextended: libc::c_char,
    pub sp: [sparse; 4],
    pub realsize: [libc::c_char; 12],
    pub offset: [libc::c_char; 12],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
    pub mfill: [libc::c_char; 8],
    pub xmagic: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_ext_header {
    pub sp: [sparse; 21],
    pub isextended: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sp_array {
    pub offset: off_t,
    pub numbytes: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xheader {
    pub stk: *mut obstack,
    pub size: size_t,
    pub buffer: *mut libc::c_char,
    pub string_length: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_array {
    pub xkey: *mut libc::c_char,
    pub xval_ptr: *mut libc::c_char,
    pub xval_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_stat_info {
    pub orig_file_name: *mut libc::c_char,
    pub file_name: *mut libc::c_char,
    pub had_trailing_slash: bool,
    pub link_name: *mut libc::c_char,
    pub uname: *mut libc::c_char,
    pub gname: *mut libc::c_char,
    pub cntx_name: *mut libc::c_char,
    pub acls_a_ptr: *mut libc::c_char,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut libc::c_char,
    pub acls_d_len: size_t,
    pub stat: stat,
    pub atime: timespec,
    pub mtime: timespec,
    pub ctime: timespec,
    pub archive_file_size: off_t,
    pub is_sparse: bool,
    pub sparse_major: libc::c_uint,
    pub sparse_minor: libc::c_uint,
    pub sparse_map_avail: size_t,
    pub sparse_map_size: size_t,
    pub sparse_map: *mut sp_array,
    pub real_size: off_t,
    pub real_size_set: bool,
    pub sparse_name_done: bool,
    pub xattr_map_size: size_t,
    pub xattr_map: *mut xattr_array,
    pub xhdr: xheader,
    pub is_dumpdir: bool,
    pub skipped: bool,
    pub dumpdir: *mut libc::c_char,
    pub parent: *mut tar_stat_info,
    pub dirstream: *mut DIR,
    pub fd: libc::c_int,
    pub exclude_list: *mut exclist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union block {
    pub buffer: [libc::c_char; 512],
    pub header: posix_header,
    pub star_header: star_header,
    pub oldgnu_header: oldgnu_header,
    pub sparse_header: sparse_header,
    pub star_in_header: star_in_header,
    pub star_ext_header: star_ext_header,
}
pub type old_files = libc::c_uint;
pub const KEEP_NEWER_FILES: old_files = 6;
pub const SKIP_OLD_FILES: old_files = 5;
pub const KEEP_OLD_FILES: old_files = 4;
pub const UNLINK_FIRST_OLD_FILES: old_files = 3;
pub const OVERWRITE_OLD_FILES: old_files = 2;
pub const NO_OVERWRITE_DIR_OLD_FILES: old_files = 1;
pub const DEFAULT_OLD_FILES: old_files = 0;
pub type dump_status = libc::c_uint;
pub const dump_status_not_implemented: dump_status = 3;
pub const dump_status_fail: dump_status = 2;
pub const dump_status_short: dump_status = 1;
pub const dump_status_ok: dump_status = 0;
pub type tar_extractor_t = Option::<
    unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
>;
pub type remove_option = libc::c_uint;
pub const WANT_DIRECTORY_REMOVE_OPTION: remove_option = 2;
pub const RECURSIVE_REMOVE_OPTION: remove_option = 1;
pub const ORDINARY_REMOVE_OPTION: remove_option = 0;
pub const BILLION: C2RustUnnamed_2 = 1000000000;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_set_stat {
    pub next: *mut delayed_set_stat,
    pub dev: dev_t,
    pub ino: ino_t,
    pub mode: mode_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub atime: timespec,
    pub mtime: timespec,
    pub current_mode: mode_t,
    pub current_mode_mask: mode_t,
    pub interdir: bool,
    pub atflag: libc::c_int,
    pub after_links: bool,
    pub change_dir: libc::c_int,
    pub cntx_name: *mut libc::c_char,
    pub acls_a_ptr: *mut libc::c_char,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut libc::c_char,
    pub acls_d_len: size_t,
    pub xattr_map_size: size_t,
    pub xattr_map: *mut xattr_array,
    pub file_name_len: size_t,
    pub file_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_list {
    pub next: *mut string_list,
    pub string: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_link {
    pub next: *mut delayed_link,
    pub dev: dev_t,
    pub ino: ino_t,
    pub birthtime: timespec,
    pub is_symlink: bool,
    pub mode: mode_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub atime: timespec,
    pub mtime: timespec,
    pub change_dir: libc::c_int,
    pub sources: *mut string_list,
    pub cntx_name: *mut libc::c_char,
    pub acls_a_ptr: *mut libc::c_char,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut libc::c_char,
    pub acls_d_len: size_t,
    pub xattr_map_size: size_t,
    pub xattr_map: *mut xattr_array,
    pub target: [libc::c_char; 1],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const LOG10_BILLION: C2RustUnnamed_2 = 9;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
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
unsafe extern "C" fn mknodat(
    mut __fd: libc::c_int,
    mut __path: *const libc::c_char,
    mut __mode: __mode_t,
    mut __dev: __dev_t,
) -> libc::c_int {
    return __xmknodat(0 as libc::c_int, __fd, __path, __mode, &mut __dev);
}
#[inline]
unsafe extern "C" fn priv_set_remove_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn priv_set_restore_linkdir() -> libc::c_int {
    return -(1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn get_stat_birthtime(mut st: *const stat) -> timespec {
    let mut t: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    t.tv_sec = -(1 as libc::c_int) as __time_t;
    t.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
    return t;
}
static mut we_are_root: bool = false;
static mut newdir_umask: mode_t = 0;
static mut current_umask: mode_t = 0;
unsafe extern "C" fn implemented(mut err: libc::c_int) -> bool {
    return !(err == 38 as libc::c_int || err == 95 as libc::c_int
        || 95 as libc::c_int != 95 as libc::c_int && err == 95 as libc::c_int);
}
static mut delayed_set_stat_head: *mut delayed_set_stat = 0 as *const delayed_set_stat
    as *mut delayed_set_stat;
static mut delayed_link_head: *mut delayed_link = 0 as *const delayed_link
    as *mut delayed_link;
pub unsafe extern "C" fn extr_init() {
    we_are_root = geteuid() == 0 as libc::c_int as libc::c_uint;
    same_permissions_option += we_are_root as libc::c_int;
    same_owner_option += we_are_root as libc::c_int;
    newdir_umask = umask(0 as libc::c_int as __mode_t);
    if (0 as libc::c_int) < same_permissions_option {
        current_umask = 0 as libc::c_int as mode_t;
    } else {
        umask(newdir_umask);
        current_umask = newdir_umask;
    };
}
unsafe extern "C" fn fd_i_chmod(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut mode: mode_t,
    mut atflag: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int <= fd {
        let mut result: libc::c_int = fchmod(fd, mode);
        if result == 0 as libc::c_int
            || implemented(*__errno_location()) as libc::c_int != 0
        {
            return result;
        }
    }
    return rpl_fchmodat(chdir_fd, file, mode, atflag);
}
unsafe extern "C" fn fd_chmod(
    mut fd: libc::c_int,
    mut file_name: *const libc::c_char,
    mut mode: libc::c_int,
    mut atflag: libc::c_int,
    mut typeflag: libc::c_int,
) -> libc::c_int {
    let mut chmod_errno: libc::c_int = if fd_i_chmod(
        fd,
        file_name,
        mode as mode_t,
        atflag,
    ) == 0 as libc::c_int
    {
        0 as libc::c_int
    } else {
        *__errno_location()
    };
    if chmod_errno == 1 as libc::c_int && mode & 0o4000 as libc::c_int != 0
        && priv_set_restore_linkdir() == 0 as libc::c_int
    {
        chmod_errno = if fd_i_chmod(fd, file_name, mode as mode_t, atflag)
            == 0 as libc::c_int
        {
            0 as libc::c_int
        } else {
            *__errno_location()
        };
        priv_set_remove_linkdir();
    }
    if atflag != 0 && typeflag != '2' as i32 && !implemented(chmod_errno) {
        chmod_errno = if fd_i_chmod(fd, file_name, mode as mode_t, 0 as libc::c_int)
            == 0 as libc::c_int
        {
            0 as libc::c_int
        } else {
            *__errno_location()
        };
    }
    if chmod_errno != 0
        && (typeflag != '2' as i32 || implemented(chmod_errno) as libc::c_int != 0)
    {
        *__errno_location() = chmod_errno;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fd_chown(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
    mut atflag: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int <= fd {
        let mut result: libc::c_int = fchown(fd, uid, gid);
        if result == 0 as libc::c_int
            || implemented(*__errno_location()) as libc::c_int != 0
        {
            return result;
        }
    }
    return fchownat(chdir_fd, file, uid, gid, atflag);
}
unsafe extern "C" fn fd_stat(
    mut fd: libc::c_int,
    mut file: *const libc::c_char,
    mut st: *mut stat,
    mut atflag: libc::c_int,
) -> libc::c_int {
    return if 0 as libc::c_int <= fd {
        fstat(fd, st)
    } else {
        fstatat(chdir_fd, file, st, atflag)
    };
}
unsafe extern "C" fn set_mode(
    mut file_name: *const libc::c_char,
    mut mode: mode_t,
    mut mode_mask: mode_t,
    mut fd: libc::c_int,
    mut current_mode: mode_t,
    mut current_mode_mask: mode_t,
    mut typeflag: libc::c_char,
    mut atflag: libc::c_int,
) {
    if (current_mode ^ mode | !current_mode_mask) & mode_mask != 0 {
        if (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int
                            >> 3 as libc::c_int)))) as libc::c_uint & !mode_mask
            & !current_mode_mask != 0
        {
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
            if fd_stat(fd, file_name, &mut st, atflag) != 0 as libc::c_int {
                stat_error(file_name);
                return;
            }
            current_mode = st.st_mode;
        }
        current_mode
            &= (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int)))) as libc::c_uint;
        mode = current_mode & !mode_mask | mode & mode_mask;
        if current_mode != mode {
            if fd_chmod(
                fd,
                file_name,
                mode as libc::c_int,
                atflag,
                typeflag as libc::c_int,
            ) != 0
            {
                chmod_error_details(file_name, mode);
            }
        }
    }
}
unsafe extern "C" fn check_time(mut file_name: *const libc::c_char, mut t: timespec) {
    if t.tv_sec < 0 as libc::c_int as libc::c_long {
        if warning_option & 0x8000 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: implausibly old time stamp %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
                tartime(t, 1 as libc::c_int != 0),
            );
        }
    } else if timespec_cmp(volume_start_time, t) < 0 as libc::c_int {
        let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        gettime(&mut now);
        if timespec_cmp(now, t) < 0 as libc::c_int {
            let mut buf: [libc::c_char; 32] = [0; 32];
            let mut diff: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            diff.tv_sec = t.tv_sec - now.tv_sec;
            diff.tv_nsec = t.tv_nsec - now.tv_nsec;
            if diff.tv_nsec < 0 as libc::c_int as libc::c_long {
                diff.tv_nsec += BILLION as libc::c_int as libc::c_long;
                diff.tv_sec -= 1;
                diff.tv_sec;
            }
            if warning_option & 0x8000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: time stamp %s is %s s in the future\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    file_name,
                    tartime(t, 1 as libc::c_int != 0),
                    code_timespec(diff, buf.as_mut_ptr()),
                );
            }
        }
    }
}
unsafe extern "C" fn set_stat(
    mut file_name: *const libc::c_char,
    mut st: *const tar_stat_info,
    mut fd: libc::c_int,
    mut current_mode: mode_t,
    mut current_mode_mask: mode_t,
    mut typeflag: libc::c_char,
    mut interdir: bool,
    mut atflag: libc::c_int,
) {
    if !touch_option && !interdir {
        let mut ts: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
        if incremental_option {
            ts[0 as libc::c_int as usize] = (*st).atime;
        } else {
            ts[0 as libc::c_int as usize]
                .tv_nsec = ((1 as libc::c_long) << 30 as libc::c_int)
                - 2 as libc::c_long;
        }
        ts[1 as libc::c_int as usize] = (*st).mtime;
        if fdutimensat(
            fd,
            chdir_fd,
            file_name,
            ts.as_mut_ptr() as *const timespec,
            atflag,
        ) == 0 as libc::c_int
        {
            if incremental_option {
                check_time(file_name, ts[0 as libc::c_int as usize]);
            }
            check_time(file_name, ts[1 as libc::c_int as usize]);
        } else if typeflag as libc::c_int != '2' as i32
            || implemented(*__errno_location()) as libc::c_int != 0
        {
            utime_error(file_name);
        }
    }
    if (0 as libc::c_int) < same_owner_option && !interdir {
        let mut uid: uid_t = (*st).stat.st_uid;
        let mut gid: gid_t = (*st).stat.st_gid;
        if fd_chown(fd, file_name, uid, gid, atflag) == 0 as libc::c_int {
            if (current_mode | !current_mode_mask)
                & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint != 0
            {
                current_mode_mask
                    &= !(current_mode
                        & (0o4000 as libc::c_int | 0o2000 as libc::c_int)
                            as libc::c_uint);
            }
        } else if typeflag as libc::c_int != '2' as i32
            || implemented(*__errno_location()) as libc::c_int != 0
        {
            chown_error_details(file_name, uid, gid);
        }
    }
    set_mode(
        file_name,
        (*st).stat.st_mode & !current_umask,
        (if (0 as libc::c_int) < same_permissions_option && !interdir {
            0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int)))
        } else {
            0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int))
        }) as mode_t,
        fd,
        current_mode,
        current_mode_mask,
        typeflag,
        atflag,
    );
    xattrs_xattrs_set(st, file_name, typeflag, 1 as libc::c_int);
    xattrs_acls_set(st, file_name, typeflag);
    xattrs_selinux_set(st, file_name, typeflag);
}
unsafe extern "C" fn find_direct_ancestor(
    mut file_name: *const libc::c_char,
) -> *mut delayed_set_stat {
    let mut h: *mut delayed_set_stat = delayed_set_stat_head;
    while !h.is_null() {
        if !(*h).after_links
            && strncmp(file_name, (*h).file_name, (*h).file_name_len) == 0 as libc::c_int
            && *file_name.offset((*h).file_name_len as isize) as libc::c_int
                == '/' as i32
            && last_component(file_name)
                == file_name
                    .offset((*h).file_name_len as isize)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_char
        {
            break;
        }
        h = (*h).next;
    }
    return h;
}
unsafe extern "C" fn mark_after_links(mut head: *mut delayed_set_stat) {
    let mut h: *mut delayed_set_stat = head;
    loop {
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
        (*h).after_links = 1 as libc::c_int != 0;
        if deref_stat((*h).file_name, &mut st) != 0 as libc::c_int {
            stat_error((*h).file_name);
        } else {
            (*h).dev = st.st_dev;
            (*h).ino = st.st_ino;
        }
        h = (*h).next;
        if !(!h.is_null() && !(*h).after_links) {
            break;
        }
    };
}
unsafe extern "C" fn delay_set_stat(
    mut file_name: *const libc::c_char,
    mut st: *const tar_stat_info,
    mut current_mode: mode_t,
    mut current_mode_mask: mode_t,
    mut mode: mode_t,
    mut atflag: libc::c_int,
) {
    let mut file_name_len: size_t = strlen(file_name);
    let mut data: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    data = delayed_set_stat_head;
    while !data.is_null() {
        if strcmp((*data).file_name, file_name) == 0 as libc::c_int {
            break;
        }
        data = (*data).next;
    }
    if !data.is_null() {
        if (*data).interdir {
            let mut real_st: stat = stat {
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
            if fstatat(chdir_fd, (*data).file_name, &mut real_st, (*data).atflag)
                != 0 as libc::c_int
            {
                stat_error((*data).file_name);
            } else {
                (*data).dev = real_st.st_dev;
                (*data).ino = real_st.st_ino;
            }
        }
    } else {
        data = xmalloc(::std::mem::size_of::<delayed_set_stat>() as libc::c_ulong)
            as *mut delayed_set_stat;
        (*data).next = delayed_set_stat_head;
        delayed_set_stat_head = data;
        (*data).file_name_len = file_name_len;
        (*data).file_name = xstrdup(file_name);
        (*data).after_links = 0 as libc::c_int != 0;
        if !st.is_null() {
            (*data).dev = (*st).stat.st_dev;
            (*data).ino = (*st).stat.st_ino;
        }
    }
    (*data).mode = mode;
    if !st.is_null() {
        (*data).uid = (*st).stat.st_uid;
        (*data).gid = (*st).stat.st_gid;
        (*data).atime = (*st).atime;
        (*data).mtime = (*st).mtime;
    }
    (*data).current_mode = current_mode;
    (*data).current_mode_mask = current_mode_mask;
    (*data).interdir = st.is_null();
    (*data).atflag = atflag;
    (*data).change_dir = chdir_current;
    (*data).cntx_name = 0 as *mut libc::c_char;
    if !st.is_null() {
        assign_string(&mut (*data).cntx_name, (*st).cntx_name);
    }
    if !st.is_null() && !((*st).acls_a_ptr).is_null() {
        (*data)
            .acls_a_ptr = xmemdup(
            (*st).acls_a_ptr as *const libc::c_void,
            ((*st).acls_a_len).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        (*data).acls_a_len = (*st).acls_a_len;
    } else {
        (*data).acls_a_ptr = 0 as *mut libc::c_char;
        (*data).acls_a_len = 0 as libc::c_int as size_t;
    }
    if !st.is_null() && !((*st).acls_d_ptr).is_null() {
        (*data)
            .acls_d_ptr = xmemdup(
            (*st).acls_d_ptr as *const libc::c_void,
            ((*st).acls_d_len).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        (*data).acls_d_len = (*st).acls_d_len;
    } else {
        (*data).acls_d_ptr = 0 as *mut libc::c_char;
        (*data).acls_d_len = 0 as libc::c_int as size_t;
    }
    if !st.is_null() {
        xheader_xattr_copy(st, &mut (*data).xattr_map, &mut (*data).xattr_map_size);
    } else {
        (*data).xattr_map = 0 as *mut xattr_array;
        (*data).xattr_map_size = 0 as libc::c_int as size_t;
    }
    if must_be_dot_or_slash(file_name) {
        mark_after_links(data);
    }
}
unsafe extern "C" fn repair_delayed_set_stat(
    mut dir: *const libc::c_char,
    mut dir_stat_info: *const stat,
) {
    let mut data: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    data = delayed_set_stat_head;
    while !data.is_null() {
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
        if fstatat(chdir_fd, (*data).file_name, &mut st, (*data).atflag)
            != 0 as libc::c_int
        {
            stat_error((*data).file_name);
            return;
        }
        if st.st_dev == (*dir_stat_info).st_dev && st.st_ino == (*dir_stat_info).st_ino {
            (*data).dev = current_stat_info.stat.st_dev;
            (*data).ino = current_stat_info.stat.st_ino;
            (*data).mode = current_stat_info.stat.st_mode;
            (*data).uid = current_stat_info.stat.st_uid;
            (*data).gid = current_stat_info.stat.st_gid;
            (*data).atime = current_stat_info.atime;
            (*data).mtime = current_stat_info.mtime;
            (*data).current_mode = st.st_mode;
            (*data).current_mode_mask = !(0 as libc::c_int as mode_t);
            (*data).interdir = 0 as libc::c_int != 0;
            return;
        }
        data = (*data).next;
    }
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: Unexpected inconsistency when making directory\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        quotearg_colon(dir),
    );
    exit_status = 2 as libc::c_int;
}
unsafe extern "C" fn free_delayed_set_stat(mut data: *mut delayed_set_stat) {
    rpl_free((*data).file_name as *mut libc::c_void);
    xheader_xattr_free((*data).xattr_map, (*data).xattr_map_size);
    rpl_free((*data).cntx_name as *mut libc::c_void);
    rpl_free((*data).acls_a_ptr as *mut libc::c_void);
    rpl_free((*data).acls_d_ptr as *mut libc::c_void);
    rpl_free(data as *mut libc::c_void);
}
pub unsafe extern "C" fn remove_delayed_set_stat(mut fname: *const libc::c_char) {
    let mut data: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    let mut next: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    let mut prev: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    data = delayed_set_stat_head;
    while !data.is_null() {
        next = (*data).next;
        if chdir_current == (*data).change_dir
            && strcmp((*data).file_name, fname) == 0 as libc::c_int
        {
            free_delayed_set_stat(data);
            if !prev.is_null() {
                (*prev).next = next;
            } else {
                delayed_set_stat_head = next;
            }
            return;
        } else {
            prev = data;
        }
        data = next;
    }
}
unsafe extern "C" fn fixup_delayed_set_stat(
    mut src: *const libc::c_char,
    mut dst: *const libc::c_char,
) {
    let mut data: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    data = delayed_set_stat_head;
    while !data.is_null() {
        if chdir_current == (*data).change_dir
            && strcmp((*data).file_name, src) == 0 as libc::c_int
        {
            rpl_free((*data).file_name as *mut libc::c_void);
            (*data).file_name = xstrdup(dst);
            (*data).file_name_len = strlen(dst);
            return;
        }
        data = (*data).next;
    }
}
unsafe extern "C" fn make_directories(
    mut file_name: *mut libc::c_char,
    mut interdir_made: *mut bool,
) -> libc::c_int {
    let mut cursor0: *mut libc::c_char = file_name.offset(0 as libc::c_int as isize);
    let mut cursor: *mut libc::c_char = 0 as *mut libc::c_char;
    cursor = cursor0;
    while *cursor != 0 {
        let mut mode: mode_t = 0;
        let mut desired_mode: mode_t = 0;
        let mut status: libc::c_int = 0;
        if *cursor as libc::c_int == '/' as i32 {
            if !(cursor == cursor0
                || *cursor.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32)
            {
                if !(*cursor.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '.' as i32
                    && (cursor == cursor0.offset(1 as libc::c_int as isize)
                        || *cursor.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '/' as i32
                        || *cursor.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '.' as i32
                            && (cursor == cursor0.offset(2 as libc::c_int as isize)
                                || *cursor.offset(-(3 as libc::c_int) as isize)
                                    as libc::c_int == '/' as i32)))
                {
                    *cursor = '\0' as i32 as libc::c_char;
                    desired_mode = (0o100 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o200 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int
                            | (0o400 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int))) as libc::c_uint & !newdir_umask;
                    mode = desired_mode
                        | (if we_are_root as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            0o200 as libc::c_int | 0o100 as libc::c_int
                        }) as libc::c_uint;
                    status = mkdirat(chdir_fd, file_name, mode);
                    if status == 0 as libc::c_int {
                        delay_set_stat(
                            file_name,
                            0 as *const tar_stat_info,
                            mode & !current_umask,
                            (0o100 as libc::c_int
                                | 0o100 as libc::c_int >> 3 as libc::c_int
                                | 0o100 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                                | (0o200 as libc::c_int
                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                    | 0o200 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int
                                    | (0o400 as libc::c_int
                                        | 0o400 as libc::c_int >> 3 as libc::c_int
                                        | 0o400 as libc::c_int >> 3 as libc::c_int
                                            >> 3 as libc::c_int))) as mode_t,
                            desired_mode,
                            0x100 as libc::c_int,
                        );
                        print_for_mkdir(
                            file_name,
                            cursor.offset_from(file_name) as libc::c_long as libc::c_int,
                            desired_mode,
                        );
                        *interdir_made = 1 as libc::c_int != 0;
                    } else if *__errno_location() == 17 as libc::c_int {
                        status = 0 as libc::c_int;
                    } else {
                        let mut e: libc::c_int = *__errno_location();
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
                        status = fstatat(chdir_fd, file_name, &mut st, 0 as libc::c_int);
                        if status != 0 {
                            *__errno_location() = e;
                            mkdir_error(file_name);
                        }
                    }
                    *cursor = '/' as i32 as libc::c_char;
                    if status != 0 {
                        return status;
                    }
                }
            }
        }
        cursor = cursor.offset(1);
        cursor;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_newer_p(
    mut file_name: *const libc::c_char,
    mut stp: *const stat,
    mut tar_stat: *mut tar_stat_info,
) -> bool {
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
    if stp.is_null() {
        if deref_stat(file_name, &mut st) != 0 as libc::c_int {
            if *__errno_location() != 2 as libc::c_int {
                stat_warn(file_name);
                return 1 as libc::c_int != 0;
            }
            return 0 as libc::c_int != 0;
        }
        stp = &mut st;
    }
    return !((*stp).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint)
        && tar_timespec_cmp((*tar_stat).mtime, get_stat_mtime(stp)) <= 0 as libc::c_int;
}
unsafe extern "C" fn maybe_recoverable(
    mut file_name: *mut libc::c_char,
    mut regular: bool,
    mut interdir_made: *mut bool,
) -> libc::c_int {
    let mut e: libc::c_int = *__errno_location();
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
    let mut stp: *const stat = 0 as *const stat;
    if *interdir_made {
        return 0 as libc::c_int;
    }
    let mut current_block_20: u64;
    match e {
        31 => {
            current_block_20 = 9382937993335388323;
        }
        40 | 95 => {
            current_block_20 = 9382937993335388323;
        }
        17 => {
            current_block_20 = 4203005064024670655;
        }
        2 => {
            current_block_20 = 17182852842979701788;
        }
        _ => {
            current_block_20 = 1608152415753874203;
        }
    }
    match current_block_20 {
        9382937993335388323 => {
            if !regular
                || old_files_option as libc::c_uint
                    != OVERWRITE_OLD_FILES as libc::c_int as libc::c_uint
                || dereference_option as libc::c_int != 0
            {
                current_block_20 = 1608152415753874203;
            } else if !(strchr(file_name, '/' as i32)).is_null() {
                if deref_stat(file_name, &mut st) != 0 as libc::c_int {
                    current_block_20 = 1608152415753874203;
                } else {
                    stp = &mut st;
                    current_block_20 = 4203005064024670655;
                }
            } else {
                current_block_20 = 4203005064024670655;
            }
        }
        _ => {}
    }
    match current_block_20 {
        4203005064024670655 => {
            let mut current_block_16: u64;
            match old_files_option as libc::c_uint {
                5 => {
                    if warning_option & 0x100000 as libc::c_int != 0 {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: skipping existing file\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            file_name,
                        );
                    }
                    return 2 as libc::c_int;
                }
                4 => return 0 as libc::c_int,
                6 => {
                    if file_newer_p(file_name, stp, &mut current_stat_info) {
                        current_block_16 = 10043043949733653460;
                    } else {
                        current_block_16 = 2809122975166620242;
                    }
                }
                0 | 1 | 2 => {
                    current_block_16 = 2809122975166620242;
                }
                3 | _ => {
                    current_block_16 = 10043043949733653460;
                }
            }
            match current_block_16 {
                2809122975166620242 => {
                    if (0 as libc::c_int)
                        < remove_any_file(file_name, ORDINARY_REMOVE_OPTION)
                    {
                        return 1 as libc::c_int;
                    }
                }
                _ => {}
            }
            current_block_20 = 17182852842979701788;
        }
        _ => {}
    }
    match current_block_20 {
        17182852842979701788 => {
            if make_directories(file_name, interdir_made) == 0 as libc::c_int
                && *interdir_made as libc::c_int != 0
            {
                return 1 as libc::c_int;
            }
        }
        _ => {}
    }
    *__errno_location() = e;
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_xattr(
    mut file_name: *const libc::c_char,
    mut st: *const tar_stat_info,
    mut invert_permissions: mode_t,
    mut typeflag: libc::c_char,
    mut file_created: *mut libc::c_int,
) -> libc::c_int {
    let mut interdir_made: bool = 0 as libc::c_int != 0;
    if xattrs_option > 0 as libc::c_int && (*st).xattr_map_size != 0 {
        let mut mode: mode_t = current_stat_info.stat.st_mode
            & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
                as libc::c_uint & !current_umask;
        loop {
            if mknodat(
                chdir_fd,
                file_name,
                mode ^ invert_permissions,
                0 as libc::c_int as __dev_t,
            ) == 0
            {
                xattrs_xattrs_set(st, file_name, typeflag, 0 as libc::c_int);
                *file_created = 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            match maybe_recoverable(
                file_name as *mut libc::c_char,
                0 as libc::c_int != 0,
                &mut interdir_made,
            ) {
                0 => {
                    skip_member();
                    open_error(file_name);
                    return 1 as libc::c_int;
                }
                2 => return 0 as libc::c_int,
                1 | _ => {}
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn apply_nonancestor_delayed_set_stat(
    mut file_name: *const libc::c_char,
    mut after_links: bool,
) {
    let mut file_name_len: size_t = strlen(file_name);
    let mut check_for_renamed_directories: bool = 0 as libc::c_int != 0;
    while !delayed_set_stat_head.is_null() {
        let mut data: *mut delayed_set_stat = delayed_set_stat_head;
        let mut skip_this_one: bool = 0 as libc::c_int != 0;
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
        let mut current_mode: mode_t = (*data).current_mode;
        let mut current_mode_mask: mode_t = (*data).current_mode_mask;
        check_for_renamed_directories = (check_for_renamed_directories as libc::c_int
            | (*data).after_links as libc::c_int) != 0;
        if (after_links as libc::c_int) < (*data).after_links as libc::c_int
            || (*data).file_name_len < file_name_len
                && *file_name.offset((*data).file_name_len as isize) as libc::c_int != 0
                && (*file_name.offset((*data).file_name_len as isize) as libc::c_int
                    == '/' as i32
                    || *file_name
                        .offset(
                            ((*data).file_name_len)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                        ) as libc::c_int == '/' as i32)
                && memcmp(
                    file_name as *const libc::c_void,
                    (*data).file_name as *const libc::c_void,
                    (*data).file_name_len,
                ) == 0 as libc::c_int
        {
            break;
        }
        chdir_do((*data).change_dir);
        if check_for_renamed_directories {
            if fstatat(chdir_fd, (*data).file_name, &mut st, (*data).atflag)
                != 0 as libc::c_int
            {
                stat_error((*data).file_name);
                skip_this_one = 1 as libc::c_int != 0;
            } else {
                current_mode = st.st_mode;
                current_mode_mask = !(0 as libc::c_int as mode_t);
                if !(st.st_dev == (*data).dev && st.st_ino == (*data).ino) {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: Directory renamed before its status could be extracted\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_colon((*data).file_name),
                    );
                    exit_status = 2 as libc::c_int;
                    skip_this_one = 1 as libc::c_int != 0;
                }
            }
        }
        if !skip_this_one {
            let mut sb: tar_stat_info = tar_stat_info {
                orig_file_name: 0 as *const libc::c_char as *mut libc::c_char,
                file_name: 0 as *const libc::c_char as *mut libc::c_char,
                had_trailing_slash: false,
                link_name: 0 as *const libc::c_char as *mut libc::c_char,
                uname: 0 as *const libc::c_char as *mut libc::c_char,
                gname: 0 as *const libc::c_char as *mut libc::c_char,
                cntx_name: 0 as *const libc::c_char as *mut libc::c_char,
                acls_a_ptr: 0 as *const libc::c_char as *mut libc::c_char,
                acls_a_len: 0,
                acls_d_ptr: 0 as *const libc::c_char as *mut libc::c_char,
                acls_d_len: 0,
                stat: stat {
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
                },
                atime: timespec { tv_sec: 0, tv_nsec: 0 },
                mtime: timespec { tv_sec: 0, tv_nsec: 0 },
                ctime: timespec { tv_sec: 0, tv_nsec: 0 },
                archive_file_size: 0,
                is_sparse: false,
                sparse_major: 0,
                sparse_minor: 0,
                sparse_map_avail: 0,
                sparse_map_size: 0,
                sparse_map: 0 as *const sp_array as *mut sp_array,
                real_size: 0,
                real_size_set: false,
                sparse_name_done: false,
                xattr_map_size: 0,
                xattr_map: 0 as *const xattr_array as *mut xattr_array,
                xhdr: xheader {
                    stk: 0 as *const obstack as *mut obstack,
                    size: 0,
                    buffer: 0 as *const libc::c_char as *mut libc::c_char,
                    string_length: 0,
                },
                is_dumpdir: false,
                skipped: false,
                dumpdir: 0 as *const libc::c_char as *mut libc::c_char,
                parent: 0 as *const tar_stat_info as *mut tar_stat_info,
                dirstream: 0 as *const DIR as *mut DIR,
                fd: 0,
                exclude_list: 0 as *const exclist as *mut exclist,
            };
            sb.stat.st_mode = (*data).mode;
            sb.stat.st_uid = (*data).uid;
            sb.stat.st_gid = (*data).gid;
            sb.atime = (*data).atime;
            sb.mtime = (*data).mtime;
            sb.cntx_name = (*data).cntx_name;
            sb.acls_a_ptr = (*data).acls_a_ptr;
            sb.acls_a_len = (*data).acls_a_len;
            sb.acls_d_ptr = (*data).acls_d_ptr;
            sb.acls_d_len = (*data).acls_d_len;
            sb.xattr_map = (*data).xattr_map;
            sb.xattr_map_size = (*data).xattr_map_size;
            set_stat(
                (*data).file_name,
                &mut sb,
                -(1 as libc::c_int),
                current_mode,
                current_mode_mask,
                '5' as i32 as libc::c_char,
                (*data).interdir,
                (*data).atflag,
            );
        }
        delayed_set_stat_head = (*data).next;
        free_delayed_set_stat(data);
    }
}
unsafe extern "C" fn is_directory_link(mut file_name: *const libc::c_char) -> bool {
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
    let mut e: libc::c_int = *__errno_location();
    let mut res: libc::c_int = 0;
    res = (fstatat(chdir_fd, file_name, &mut st, 0x100 as libc::c_int)
        == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        && fstatat(chdir_fd, file_name, &mut st, 0 as libc::c_int) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
    *__errno_location() = e;
    return res != 0;
}
#[inline]
unsafe extern "C" fn safe_dir_mode(mut st: *const stat) -> libc::c_int {
    return ((*st).st_mode
        & (if (0 as libc::c_int) < same_owner_option
            || (0 as libc::c_int) < same_permissions_option
        {
            0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
        } else {
            0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int))
        }) as libc::c_uint
        | (if we_are_root as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            0o200 as libc::c_int | 0o100 as libc::c_int
        }) as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn extract_dir(
    mut file_name: *mut libc::c_char,
    mut typeflag: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut mode: mode_t = 0;
    let mut current_mode: mode_t = 0 as libc::c_int as mode_t;
    let mut current_mode_mask: mode_t = 0 as libc::c_int as mode_t;
    let mut atflag: libc::c_int = 0 as libc::c_int;
    let mut interdir_made: bool = 0 as libc::c_int != 0;
    if one_file_system_option as libc::c_int != 0
        && root_device == 0 as libc::c_int as libc::c_ulong
    {
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
        if fstatat(
            chdir_fd,
            b".\0" as *const u8 as *const libc::c_char,
            &mut st,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            stat_diag(b".\0" as *const u8 as *const libc::c_char);
        } else {
            root_device = st.st_dev;
        }
    }
    if incremental_option {
        purge_directory(file_name);
    } else if typeflag == 'D' as i32 {
        skip_member();
    }
    mode = safe_dir_mode(&mut current_stat_info.stat) as mode_t;
    loop {
        status = mkdirat(chdir_fd, file_name, mode);
        if status == 0 as libc::c_int {
            current_mode = mode & !current_umask;
            current_mode_mask = (0o100 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
                as mode_t;
            atflag = 0x100 as libc::c_int;
            break;
        } else {
            if *__errno_location() == 17 as libc::c_int
                && (interdir_made as libc::c_int != 0
                    || keep_directory_symlink_option as libc::c_int != 0
                    || old_files_option as libc::c_uint
                        == NO_OVERWRITE_DIR_OLD_FILES as libc::c_int as libc::c_uint
                    || old_files_option as libc::c_uint
                        == DEFAULT_OLD_FILES as libc::c_int as libc::c_uint
                    || old_files_option as libc::c_uint
                        == OVERWRITE_OLD_FILES as libc::c_int as libc::c_uint)
            {
                let mut st_0: stat = stat {
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
                if keep_directory_symlink_option as libc::c_int != 0
                    && is_directory_link(file_name) as libc::c_int != 0
                {
                    return 0 as libc::c_int;
                }
                if deref_stat(file_name, &mut st_0) == 0 as libc::c_int {
                    current_mode = st_0.st_mode;
                    current_mode_mask = !(0 as libc::c_int as mode_t);
                    if current_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    {
                        if interdir_made {
                            repair_delayed_set_stat(file_name, &mut st_0);
                            return 0 as libc::c_int;
                        } else {
                            if !(old_files_option as libc::c_uint
                                == NO_OVERWRITE_DIR_OLD_FILES as libc::c_int
                                    as libc::c_uint)
                            {
                                break;
                            }
                            mode = safe_dir_mode(&mut st_0) as mode_t;
                            status = fd_chmod(
                                -(1 as libc::c_int),
                                file_name,
                                mode as libc::c_int,
                                0x100 as libc::c_int,
                                '5' as i32,
                            );
                            if status == 0 as libc::c_int {
                                current_stat_info.stat = st_0;
                                current_mode = mode & !current_umask;
                                current_mode_mask = (0o100 as libc::c_int
                                    | 0o100 as libc::c_int >> 3 as libc::c_int
                                    | 0o100 as libc::c_int >> 3 as libc::c_int
                                        >> 3 as libc::c_int
                                    | (0o200 as libc::c_int
                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                            >> 3 as libc::c_int
                                        | (0o400 as libc::c_int
                                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                                >> 3 as libc::c_int))) as mode_t;
                                atflag = 0x100 as libc::c_int;
                                break;
                            } else {
                                chmod_error_details(file_name, mode);
                                break;
                            }
                        }
                    }
                }
                *__errno_location() = 17 as libc::c_int;
            }
            match maybe_recoverable(
                file_name,
                0 as libc::c_int != 0,
                &mut interdir_made,
            ) {
                1 => {
                    continue;
                }
                0 => {}
                2 | _ => {
                    break;
                }
            }
            if *__errno_location() != 17 as libc::c_int {
                mkdir_error(file_name);
                return 1 as libc::c_int;
            }
            break;
        }
    }
    if status == 0 as libc::c_int
        || old_files_option as libc::c_uint
            == DEFAULT_OLD_FILES as libc::c_int as libc::c_uint
        || old_files_option as libc::c_uint
            == OVERWRITE_OLD_FILES as libc::c_int as libc::c_uint
    {
        delay_set_stat(
            file_name,
            &mut current_stat_info,
            current_mode,
            current_mode_mask,
            current_stat_info.stat.st_mode,
            atflag,
        );
    }
    return status;
}
unsafe extern "C" fn open_output_file(
    mut file_name: *const libc::c_char,
    mut typeflag: libc::c_int,
    mut mode: mode_t,
    mut file_created: libc::c_int,
    mut current_mode: *mut mode_t,
    mut current_mode_mask: *mut mode_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut overwriting_old_files: bool = old_files_option as libc::c_uint
        == OVERWRITE_OLD_FILES as libc::c_int as libc::c_uint;
    let mut openflag: libc::c_int = 0o1 as libc::c_int | 0 as libc::c_int
        | 0o2000000 as libc::c_int | 0o400 as libc::c_int | 0o4000 as libc::c_int
        | 0o100 as libc::c_int
        | (if overwriting_old_files as libc::c_int != 0 {
            0o1000 as libc::c_int
                | (if dereference_option as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    0o400000 as libc::c_int
                })
        } else {
            0o200 as libc::c_int
        });
    if file_created != 0 {
        openflag = openflag & !(0o200 as libc::c_int);
    }
    if typeflag == '7' as i32 {
        static mut conttype_diagnosed: libc::c_int = 0;
        if conttype_diagnosed == 0 {
            conttype_diagnosed = 1 as libc::c_int;
            if warning_option & 0x8 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Extracting contiguous files as regular files\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        }
    }
    if 1 as libc::c_int == 0 && overwriting_old_files as libc::c_int != 0
        && !dereference_option
    {
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
        if fstatat(chdir_fd, file_name, &mut st, 0x100 as libc::c_int)
            == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
        {
            *__errno_location() = 40 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    fd = openat(chdir_fd, file_name, openflag, mode);
    if 0 as libc::c_int <= fd {
        if overwriting_old_files {
            let mut st_0: stat = stat {
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
            if fstat(fd, &mut st_0) != 0 as libc::c_int {
                let mut e: libc::c_int = *__errno_location();
                close(fd);
                *__errno_location() = e;
                return -(1 as libc::c_int);
            }
            if !(st_0.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
            {
                close(fd);
                *__errno_location() = 17 as libc::c_int;
                return -(1 as libc::c_int);
            }
            *current_mode = st_0.st_mode;
            *current_mode_mask = !(0 as libc::c_int as mode_t);
        } else {
            *current_mode = mode & !current_umask;
            *current_mode_mask = (0o100 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
                as mode_t;
        }
    }
    return fd;
}
unsafe extern "C" fn extract_file(
    mut file_name: *mut libc::c_char,
    mut typeflag: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut size: off_t = 0;
    let mut data_block: *mut block = 0 as *mut block;
    let mut status: libc::c_int = 0;
    let mut count: size_t = 0;
    let mut written: size_t = 0;
    let mut interdir_made: bool = 0 as libc::c_int != 0;
    let mut mode: mode_t = current_stat_info.stat.st_mode
        & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
            as libc::c_uint
        & !(if (0 as libc::c_int) < same_owner_option {
            (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    let mut invert_permissions: mode_t = if (0 as libc::c_int) < same_owner_option {
        mode
            & ((0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
    } else {
        0 as libc::c_int as libc::c_uint
    };
    let mut current_mode: mode_t = 0 as libc::c_int as mode_t;
    let mut current_mode_mask: mode_t = 0 as libc::c_int as mode_t;
    if to_stdout_option {
        fd = 1 as libc::c_int;
    } else if !to_command_option.is_null() {
        fd = sys_exec_command(file_name, 'f' as i32, &mut current_stat_info);
        if fd < 0 as libc::c_int {
            skip_member();
            return 0 as libc::c_int;
        }
    } else {
        let mut file_created: libc::c_int = 0 as libc::c_int;
        if set_xattr(
            file_name,
            &mut current_stat_info,
            invert_permissions,
            typeflag as libc::c_char,
            &mut file_created,
        ) != 0
        {
            return 1 as libc::c_int;
        }
        loop {
            fd = open_output_file(
                file_name,
                typeflag,
                mode,
                file_created,
                &mut current_mode,
                &mut current_mode_mask,
            );
            if !(fd < 0 as libc::c_int) {
                break;
            }
            let mut recover: libc::c_int = maybe_recoverable(
                file_name,
                1 as libc::c_int != 0,
                &mut interdir_made,
            );
            if recover != 1 as libc::c_int {
                skip_member();
                if recover == 2 as libc::c_int {
                    return 0 as libc::c_int;
                }
                open_error(file_name);
                return 1 as libc::c_int;
            }
        }
    }
    mv_begin_read(&mut current_stat_info);
    if current_stat_info.is_sparse {
        sparse_extract_file(fd, &mut current_stat_info, &mut size);
    } else {
        size = current_stat_info.stat.st_size;
        while size > 0 as libc::c_int as libc::c_long {
            mv_size_left(size);
            data_block = find_next_block();
            if data_block.is_null() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unexpected EOF in archive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                exit_status = 2 as libc::c_int;
                break;
            } else {
                written = available_space_after(data_block);
                if written > size as libc::c_ulong {
                    written = size as size_t;
                }
                *__errno_location() = 0 as libc::c_int;
                count = blocking_write(
                    fd,
                    ((*data_block).buffer).as_mut_ptr() as *const libc::c_void,
                    written,
                );
                size = (size as libc::c_ulong).wrapping_sub(written) as off_t as off_t;
                set_next_block_after(
                    ((*data_block).buffer)
                        .as_mut_ptr()
                        .offset(written as isize)
                        .offset(-(1 as libc::c_int as isize)) as *mut block,
                );
                if !(count != written) {
                    continue;
                }
                if to_command_option.is_null() {
                    write_error_details(file_name, count, written);
                }
                break;
            }
        }
    }
    skip_file(size);
    mv_end();
    if to_stdout_option {
        return 0 as libc::c_int;
    }
    if to_command_option.is_null() {
        set_stat(
            file_name,
            &mut current_stat_info,
            fd,
            current_mode,
            current_mode_mask,
            typeflag as libc::c_char,
            0 as libc::c_int != 0,
            if old_files_option as libc::c_uint
                == OVERWRITE_OLD_FILES as libc::c_int as libc::c_uint
            {
                0 as libc::c_int
            } else {
                0x100 as libc::c_int
            },
        );
    }
    status = close(fd);
    if status < 0 as libc::c_int {
        close_error(file_name);
    }
    if !to_command_option.is_null() {
        sys_wait_command();
    }
    return status;
}
unsafe extern "C" fn find_delayed_link_source(
    mut name: *const libc::c_char,
) -> *mut delayed_link {
    let mut dl: *mut delayed_link = 0 as *mut delayed_link;
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
    if delayed_link_head.is_null() {
        return 0 as *mut delayed_link;
    }
    if fstatat(chdir_fd, name, &mut st, 0x100 as libc::c_int) != 0 {
        if *__errno_location() != 2 as libc::c_int {
            stat_error(name);
        }
        return 0 as *mut delayed_link;
    }
    dl = delayed_link_head;
    while !dl.is_null() {
        if (*dl).dev == st.st_dev && (*dl).ino == st.st_ino {
            break;
        }
        dl = (*dl).next;
    }
    return dl;
}
unsafe extern "C" fn create_placeholder_file(
    mut file_name: *mut libc::c_char,
    mut is_symlink: bool,
    mut interdir_made: *mut bool,
    mut prev: *mut delayed_link,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
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
    loop {
        fd = openat(
            chdir_fd,
            file_name,
            0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
            0 as libc::c_int,
        );
        if !(fd < 0 as libc::c_int) {
            break;
        }
        if *__errno_location() == 17 as libc::c_int
            && !(find_delayed_link_source(file_name)).is_null()
        {
            return 0 as libc::c_int;
        }
        match maybe_recoverable(file_name, 0 as libc::c_int != 0, interdir_made) {
            2 => return 0 as libc::c_int,
            0 => {
                open_error(file_name);
                return -(1 as libc::c_int);
            }
            1 | _ => {}
        }
    }
    if fstat(fd, &mut st) != 0 as libc::c_int {
        stat_error(file_name);
        close(fd);
    } else if close(fd) != 0 as libc::c_int {
        close_error(file_name);
    } else {
        let mut h: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
        let mut p: *mut delayed_link = xmalloc(
            (160 as libc::c_ulong)
                .wrapping_add(strlen(current_stat_info.link_name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut delayed_link;
        if !prev.is_null() {
            (*p).next = (*prev).next;
            (*prev).next = p;
        } else {
            (*p).next = delayed_link_head;
            delayed_link_head = p;
        }
        (*p).dev = st.st_dev;
        (*p).ino = st.st_ino;
        (*p).birthtime = get_stat_birthtime(&mut st);
        (*p).is_symlink = is_symlink;
        if is_symlink {
            (*p).mode = current_stat_info.stat.st_mode;
            (*p).uid = current_stat_info.stat.st_uid;
            (*p).gid = current_stat_info.stat.st_gid;
            (*p).atime = current_stat_info.atime;
            (*p).mtime = current_stat_info.mtime;
        }
        (*p).change_dir = chdir_current;
        (*p)
            .sources = xmalloc(
            (8 as libc::c_ulong)
                .wrapping_add(strlen(file_name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut string_list;
        (*(*p).sources).next = 0 as *mut string_list;
        strcpy(((*(*p).sources).string).as_mut_ptr(), file_name);
        (*p).cntx_name = 0 as *mut libc::c_char;
        assign_string(&mut (*p).cntx_name, current_stat_info.cntx_name);
        (*p).acls_a_ptr = 0 as *mut libc::c_char;
        (*p).acls_a_len = 0 as libc::c_int as size_t;
        (*p).acls_d_ptr = 0 as *mut libc::c_char;
        (*p).acls_d_len = 0 as libc::c_int as size_t;
        xheader_xattr_copy(
            &mut current_stat_info,
            &mut (*p).xattr_map,
            &mut (*p).xattr_map_size,
        );
        strcpy(((*p).target).as_mut_ptr(), current_stat_info.link_name);
        h = find_direct_ancestor(file_name);
        if !h.is_null() {
            mark_after_links(h);
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn extract_link(
    mut file_name: *mut libc::c_char,
    mut typeflag: libc::c_int,
) -> libc::c_int {
    let mut interdir_made: bool = 0 as libc::c_int != 0;
    let mut link_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut rc: libc::c_int = 0;
    let mut dl: *mut delayed_link = 0 as *mut delayed_link;
    link_name = current_stat_info.link_name;
    if !absolute_names_option && contains_dot_dot(link_name) as libc::c_int != 0 {
        return create_placeholder_file(
            file_name,
            0 as libc::c_int != 0,
            &mut interdir_made,
            0 as *mut delayed_link,
        );
    }
    dl = find_delayed_link_source(link_name);
    if !dl.is_null() {
        return create_placeholder_file(
            file_name,
            0 as libc::c_int != 0,
            &mut interdir_made,
            dl,
        );
    }
    loop {
        let mut st1: stat = stat {
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
        let mut st2: stat = stat {
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
        let mut e: libc::c_int = 0;
        let mut status: libc::c_int = linkat(
            chdir_fd,
            link_name,
            chdir_fd,
            file_name,
            0 as libc::c_int,
        );
        e = *__errno_location();
        if status == 0 as libc::c_int {
            let mut ds: *mut delayed_link = delayed_link_head;
            if !ds.is_null()
                && fstatat(chdir_fd, link_name, &mut st1, 0x100 as libc::c_int)
                    == 0 as libc::c_int
            {
                while !ds.is_null() {
                    if (*ds).change_dir == chdir_current && (*ds).dev == st1.st_dev
                        && (*ds).ino == st1.st_ino
                        && timespec_cmp((*ds).birthtime, get_stat_birthtime(&mut st1))
                            == 0 as libc::c_int
                    {
                        let mut p: *mut string_list = xmalloc(
                            (8 as libc::c_ulong)
                                .wrapping_add(strlen(file_name))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as *mut string_list;
                        strcpy(((*p).string).as_mut_ptr(), file_name);
                        (*p).next = (*ds).sources;
                        (*ds).sources = p;
                        break;
                    } else {
                        ds = (*ds).next;
                    }
                }
            }
            return 0 as libc::c_int;
        } else if e == 17 as libc::c_int
            && strcmp(link_name, file_name) == 0 as libc::c_int
            || fstatat(chdir_fd, link_name, &mut st1, 0x100 as libc::c_int)
                == 0 as libc::c_int
                && fstatat(chdir_fd, file_name, &mut st2, 0x100 as libc::c_int)
                    == 0 as libc::c_int && st1.st_dev == st2.st_dev
                && st1.st_ino == st2.st_ino
        {
            return 0 as libc::c_int
        }
        *__errno_location() = e;
        rc = maybe_recoverable(file_name, 0 as libc::c_int != 0, &mut interdir_made);
        if !(rc == 1 as libc::c_int) {
            break;
        }
    }
    if rc == 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !(incremental_option as libc::c_int != 0
        && *__errno_location() == 17 as libc::c_int)
    {
        link_error(link_name, file_name);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn extract_symlink(
    mut file_name: *mut libc::c_char,
    mut typeflag: libc::c_int,
) -> libc::c_int {
    let mut interdir_made: bool = 0 as libc::c_int != 0;
    if !absolute_names_option
        && (*(current_stat_info.link_name).offset(0 as libc::c_int as isize)
            as libc::c_int == '/' as i32
            || contains_dot_dot(current_stat_info.link_name) as libc::c_int != 0)
    {
        return create_placeholder_file(
            file_name,
            1 as libc::c_int != 0,
            &mut interdir_made,
            0 as *mut delayed_link,
        );
    }
    while symlinkat(current_stat_info.link_name, chdir_fd, file_name) != 0 as libc::c_int
    {
        match maybe_recoverable(file_name, 0 as libc::c_int != 0, &mut interdir_made) {
            2 => return 0 as libc::c_int,
            0 => {
                symlink_error(current_stat_info.link_name, file_name);
                return -(1 as libc::c_int);
            }
            1 | _ => {}
        }
    }
    set_stat(
        file_name,
        &mut current_stat_info,
        -(1 as libc::c_int),
        0 as libc::c_int as mode_t,
        0 as libc::c_int as mode_t,
        '2' as i32 as libc::c_char,
        0 as libc::c_int != 0,
        0x100 as libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn extract_node(
    mut file_name: *mut libc::c_char,
    mut typeflag: libc::c_int,
) -> libc::c_int {
    let mut interdir_made: bool = 0 as libc::c_int != 0;
    let mut mode: mode_t = current_stat_info.stat.st_mode
        & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int))
            | 0o60000 as libc::c_int | 0o20000 as libc::c_int) as libc::c_uint
        & !(if (0 as libc::c_int) < same_owner_option {
            (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    while mknodat(chdir_fd, file_name, mode, current_stat_info.stat.st_rdev)
        != 0 as libc::c_int
    {
        match maybe_recoverable(file_name, 0 as libc::c_int != 0, &mut interdir_made) {
            2 => return 0 as libc::c_int,
            0 => {
                mknod_error(file_name);
                return -(1 as libc::c_int);
            }
            1 | _ => {}
        }
    }
    set_stat(
        file_name,
        &mut current_stat_info,
        -(1 as libc::c_int),
        mode & !current_umask,
        (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
            as mode_t,
        typeflag as libc::c_char,
        0 as libc::c_int != 0,
        0x100 as libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn extract_fifo(
    mut file_name: *mut libc::c_char,
    mut typeflag: libc::c_int,
) -> libc::c_int {
    let mut interdir_made: bool = 0 as libc::c_int != 0;
    let mut mode: mode_t = current_stat_info.stat.st_mode
        & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
            as libc::c_uint
        & !(if (0 as libc::c_int) < same_owner_option {
            (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    while mkfifoat(chdir_fd, file_name, mode) != 0 as libc::c_int {
        match maybe_recoverable(file_name, 0 as libc::c_int != 0, &mut interdir_made) {
            2 => return 0 as libc::c_int,
            0 => {
                mkfifo_error(file_name);
                return -(1 as libc::c_int);
            }
            1 | _ => {}
        }
    }
    set_stat(
        file_name,
        &mut current_stat_info,
        -(1 as libc::c_int),
        mode & !current_umask,
        (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
            | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)))
            as mode_t,
        typeflag as libc::c_char,
        0 as libc::c_int != 0,
        0x100 as libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn prepare_to_extract(
    mut file_name: *const libc::c_char,
    mut typeflag: libc::c_int,
    mut fun: *mut tar_extractor_t,
) -> bool {
    let mut extractor: tar_extractor_t = None;
    match typeflag {
        83 => {
            extractor = Some(
                extract_file
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        0 | 48 | 55 => {
            if current_stat_info.had_trailing_slash {
                extractor = Some(
                    extract_dir
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                );
            } else {
                extractor = Some(
                    extract_file
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            libc::c_int,
                        ) -> libc::c_int,
                );
            }
        }
        50 => {
            extractor = Some(
                extract_symlink
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        49 => {
            extractor = Some(
                extract_link
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        51 => {
            current_stat_info.stat.st_mode |= 0o20000 as libc::c_int as libc::c_uint;
            extractor = Some(
                extract_node
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        52 => {
            current_stat_info.stat.st_mode |= 0o60000 as libc::c_int as libc::c_uint;
            extractor = Some(
                extract_node
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        54 => {
            extractor = Some(
                extract_fifo
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
        53 | 68 => {
            extractor = Some(
                extract_dir
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
            if current_stat_info.is_dumpdir {
                delay_directory_restore_option = 1 as libc::c_int != 0;
            }
        }
        86 => return 0 as libc::c_int != 0,
        77 => {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Cannot extract -- file is continued from another volume\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(current_stat_info.file_name),
            );
            exit_status = 2 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        76 | 75 => {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unexpected long name header\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit_status = 2 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        _ => {
            if warning_option & 0x10000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Unknown file type '%c', extracted as normal file\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_colon(file_name),
                    typeflag,
                );
            }
            extractor = Some(
                extract_file
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            );
        }
    }
    if to_stdout_option as libc::c_int != 0 || !to_command_option.is_null() {
        if extractor
            != Some(
                extract_file
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            )
        {
            return 0 as libc::c_int != 0;
        }
    } else {
        match old_files_option as libc::c_uint {
            3 => {
                if remove_any_file(
                    file_name,
                    (if recursive_unlink_option as libc::c_int != 0 {
                        RECURSIVE_REMOVE_OPTION as libc::c_int
                    } else {
                        ORDINARY_REMOVE_OPTION as libc::c_int
                    }) as remove_option,
                ) == 0 && *__errno_location() != 0
                    && *__errno_location() != 2 as libc::c_int
                {
                    unlink_error(file_name);
                    return 0 as libc::c_int != 0;
                }
            }
            6 => {
                if file_newer_p(file_name, 0 as *const stat, &mut current_stat_info) {
                    if warning_option & 0x800 as libc::c_int != 0 {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Current %s is newer or same age\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quote(file_name),
                        );
                    }
                    return 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
    }
    *fun = extractor;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn extract_archive() {
    let mut typeflag: libc::c_char = 0;
    let mut fun: tar_extractor_t = None;
    let mut skip_dotdot_name: bool = false;
    fatal_exit_hook = Some(extract_finish as unsafe extern "C" fn() -> ());
    set_next_block_after(current_header);
    skip_dotdot_name = !absolute_names_option
        && contains_dot_dot(current_stat_info.orig_file_name) as libc::c_int != 0;
    if skip_dotdot_name {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Member name contains '..'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_colon(current_stat_info.orig_file_name),
        );
        exit_status = 2 as libc::c_int;
    }
    if *(current_stat_info.file_name).offset(0 as libc::c_int as isize) == 0
        || skip_dotdot_name as libc::c_int != 0
        || interactive_option as libc::c_int != 0
            && confirm(
                b"extract\0" as *const u8 as *const libc::c_char,
                current_stat_info.file_name,
            ) == 0
    {
        skip_member();
        return;
    }
    if verbose_option != 0 {
        print_header(
            &mut current_stat_info,
            current_header,
            -(1 as libc::c_int) as off_t,
        );
    }
    if !delay_directory_restore_option {
        let mut dir: libc::c_int = chdir_current;
        apply_nonancestor_delayed_set_stat(
            current_stat_info.file_name,
            0 as libc::c_int != 0,
        );
        chdir_do(dir);
    }
    if backup_option {
        if !maybe_backup_file(current_stat_info.file_name, 0 as libc::c_int != 0) {
            let mut e: libc::c_int = *__errno_location();
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                e,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Was unable to backup this file\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(current_stat_info.file_name),
            );
            exit_status = 2 as libc::c_int;
            skip_member();
            return;
        }
    }
    typeflag = (if sparse_member_p(&mut current_stat_info) as libc::c_int != 0 {
        'S' as i32
    } else {
        (*current_header).header.typeflag as libc::c_int
    }) as libc::c_char;
    if prepare_to_extract(
        current_stat_info.file_name,
        typeflag as libc::c_int,
        &mut fun,
    ) {
        if fun.unwrap()(current_stat_info.file_name, typeflag as libc::c_int)
            == 0 as libc::c_int
        {
            return;
        }
    } else {
        skip_member();
    }
    if backup_option {
        undo_last_backup();
    }
}
unsafe extern "C" fn apply_delayed_links() {
    let mut ds: *mut delayed_link = 0 as *mut delayed_link;
    ds = delayed_link_head;
    while !ds.is_null() {
        let mut sources: *mut string_list = (*ds).sources;
        let mut valid_source: *const libc::c_char = 0 as *const libc::c_char;
        chdir_do((*ds).change_dir);
        sources = (*ds).sources;
        while !sources.is_null() {
            let mut source: *const libc::c_char = ((*sources).string).as_mut_ptr();
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
            if fstatat(chdir_fd, source, &mut st, 0x100 as libc::c_int)
                == 0 as libc::c_int && st.st_dev == (*ds).dev && st.st_ino == (*ds).ino
                && timespec_cmp(get_stat_birthtime(&mut st), (*ds).birthtime)
                    == 0 as libc::c_int
            {
                if unlinkat(chdir_fd, source, 0 as libc::c_int) != 0 as libc::c_int {
                    unlink_error(source);
                } else if !(!valid_source.is_null()
                    && linkat(chdir_fd, valid_source, chdir_fd, source, 0 as libc::c_int)
                        == 0 as libc::c_int)
                {
                    if !(*ds).is_symlink {
                        if linkat(
                            chdir_fd,
                            ((*ds).target).as_mut_ptr(),
                            chdir_fd,
                            source,
                            0 as libc::c_int,
                        ) != 0 as libc::c_int
                        {
                            link_error(((*ds).target).as_mut_ptr(), source);
                        }
                    } else if symlinkat(((*ds).target).as_mut_ptr(), chdir_fd, source)
                        != 0 as libc::c_int
                    {
                        symlink_error(((*ds).target).as_mut_ptr(), source);
                    } else {
                        let mut st1: tar_stat_info = tar_stat_info {
                            orig_file_name: 0 as *const libc::c_char
                                as *mut libc::c_char,
                            file_name: 0 as *const libc::c_char as *mut libc::c_char,
                            had_trailing_slash: false,
                            link_name: 0 as *const libc::c_char as *mut libc::c_char,
                            uname: 0 as *const libc::c_char as *mut libc::c_char,
                            gname: 0 as *const libc::c_char as *mut libc::c_char,
                            cntx_name: 0 as *const libc::c_char as *mut libc::c_char,
                            acls_a_ptr: 0 as *const libc::c_char as *mut libc::c_char,
                            acls_a_len: 0,
                            acls_d_ptr: 0 as *const libc::c_char as *mut libc::c_char,
                            acls_d_len: 0,
                            stat: stat {
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
                            },
                            atime: timespec { tv_sec: 0, tv_nsec: 0 },
                            mtime: timespec { tv_sec: 0, tv_nsec: 0 },
                            ctime: timespec { tv_sec: 0, tv_nsec: 0 },
                            archive_file_size: 0,
                            is_sparse: false,
                            sparse_major: 0,
                            sparse_minor: 0,
                            sparse_map_avail: 0,
                            sparse_map_size: 0,
                            sparse_map: 0 as *const sp_array as *mut sp_array,
                            real_size: 0,
                            real_size_set: false,
                            sparse_name_done: false,
                            xattr_map_size: 0,
                            xattr_map: 0 as *const xattr_array as *mut xattr_array,
                            xhdr: xheader {
                                stk: 0 as *const obstack as *mut obstack,
                                size: 0,
                                buffer: 0 as *const libc::c_char as *mut libc::c_char,
                                string_length: 0,
                            },
                            is_dumpdir: false,
                            skipped: false,
                            dumpdir: 0 as *const libc::c_char as *mut libc::c_char,
                            parent: 0 as *const tar_stat_info as *mut tar_stat_info,
                            dirstream: 0 as *const DIR as *mut DIR,
                            fd: 0,
                            exclude_list: 0 as *const exclist as *mut exclist,
                        };
                        st1.stat.st_mode = (*ds).mode;
                        st1.stat.st_uid = (*ds).uid;
                        st1.stat.st_gid = (*ds).gid;
                        st1.atime = (*ds).atime;
                        st1.mtime = (*ds).mtime;
                        st1.cntx_name = (*ds).cntx_name;
                        st1.acls_a_ptr = (*ds).acls_a_ptr;
                        st1.acls_a_len = (*ds).acls_a_len;
                        st1.acls_d_ptr = (*ds).acls_d_ptr;
                        st1.acls_d_len = (*ds).acls_d_len;
                        st1.xattr_map = (*ds).xattr_map;
                        st1.xattr_map_size = (*ds).xattr_map_size;
                        set_stat(
                            source,
                            &mut st1,
                            -(1 as libc::c_int),
                            0 as libc::c_int as mode_t,
                            0 as libc::c_int as mode_t,
                            '2' as i32 as libc::c_char,
                            0 as libc::c_int != 0,
                            0x100 as libc::c_int,
                        );
                        valid_source = source;
                    }
                }
            }
            sources = (*sources).next;
        }
        sources = (*ds).sources;
        while !sources.is_null() {
            let mut next: *mut string_list = (*sources).next;
            rpl_free(sources as *mut libc::c_void);
            sources = next;
        }
        xheader_xattr_free((*ds).xattr_map, (*ds).xattr_map_size);
        rpl_free((*ds).cntx_name as *mut libc::c_void);
        let mut next_0: *mut delayed_link = (*ds).next;
        rpl_free(ds as *mut libc::c_void);
        ds = next_0;
    }
    delayed_link_head = 0 as *mut delayed_link;
}
pub unsafe extern "C" fn extract_finish() {
    apply_nonancestor_delayed_set_stat(
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int != 0,
    );
    apply_delayed_links();
    apply_nonancestor_delayed_set_stat(
        b"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int != 0,
    );
}
pub unsafe extern "C" fn rename_directory(
    mut src: *mut libc::c_char,
    mut dst: *mut libc::c_char,
) -> bool {
    if renameat(chdir_fd, src, chdir_fd, dst) == 0 as libc::c_int {
        fixup_delayed_set_stat(src, dst);
    } else {
        let mut e: libc::c_int = *__errno_location();
        let mut interdir_made: bool = false;
        match e {
            2 => {
                if make_directories(dst, &mut interdir_made) == 0 as libc::c_int {
                    if renameat(chdir_fd, src, chdir_fd, dst) == 0 as libc::c_int {
                        return 1 as libc::c_int != 0;
                    }
                    e = *__errno_location();
                }
            }
            18 | _ => {}
        }
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            e,
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot rename %s to %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote_n(0 as libc::c_int, src),
            quote_n(1 as libc::c_int, dst),
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
