use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type exclist;
    pub type mode_change;
    pub type directory;
    pub type hash_table;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn safer_name_suffix(
        file_name: *const libc::c_char,
        link_target: bool,
        absolute_names: bool,
    ) -> *mut libc::c_char;
    fn utime_error(_: *const libc::c_char);
    static mut exit_status: libc::c_int;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn streamsavedir(_: *mut DIR, _: savedir_option) -> *mut libc::c_char;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, pn: *mut size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn abort() -> !;
    fn areadlinkat_with_size(
        fd: libc::c_int,
        filename: *const libc::c_char,
        size_hint: size_t,
    ) -> *mut libc::c_char;
    fn quotearg_n(n: libc::c_int, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    static mut archive_format: archive_format;
    static mut absolute_names_option: bool;
    static mut after_date_option: libc::c_int;
    static mut atime_preserve_option: atime_preserve;
    static mut dereference_option: bool;
    static mut hard_dereference_option: bool;
    static mut ignore_failed_read_option: bool;
    static mut incremental_option: bool;
    static mut interactive_option: bool;
    static mut listed_incremental_option: *const libc::c_char;
    static mut mode_option: *mut mode_change;
    static mut initial_umask: mode_t;
    static mut newer_mtime_option: timespec;
    static mut set_mtime_option: set_mtime_option_mode;
    static mut mtime_option: timespec;
    static mut recursion_option: libc::c_int;
    static mut numeric_owner_option: bool;
    static mut one_file_system_option: bool;
    static mut remove_files_option: bool;
    static mut selinux_context_option: libc::c_int;
    static mut acls_option: libc::c_int;
    static mut xattrs_option: libc::c_int;
    static mut sparse_option: bool;
    static mut totals_option: bool;
    static mut verbose_option: libc::c_int;
    static mut dev_null_output: bool;
    static mut start_time: timespec;
    static mut open_read_flags: libc::c_int;
    static mut open_searchdir_flags: libc::c_int;
    static mut fstatat_flags: libc::c_int;
    static mut savedir_sort_order: libc::c_int;
    fn available_space_after(pointer: *mut block) -> size_t;
    fn current_block_ordinal() -> off_t;
    fn close_archive();
    fn find_next_block() -> *mut block;
    fn open_archive(mode: access_mode);
    fn set_next_block_after(block: *mut block);
    fn mv_begin_write(file_name: *const libc::c_char, totsize: off_t, sizeleft: off_t);
    fn buffer_write_global_xheader();
    fn directory_contents(dir: *mut directory) -> *const libc::c_char;
    fn safe_directory_contents(dir: *mut directory) -> *const libc::c_char;
    fn write_directory_file();
    fn update_parent_directory(st: *mut tar_stat_info);
    fn dumpdir_size(p: *const libc::c_char) -> size_t;
    static mut current_format: archive_format;
    fn print_header(st: *mut tar_stat_info, blk: *mut block, block_ordinal: off_t);
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    fn blocking_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    static mut chdir_fd: libc::c_int;
    fn open_diag(name: *const libc::c_char);
    fn read_diag_details(name: *const libc::c_char, offset: off_t, size: size_t);
    fn readlink_diag(name: *const libc::c_char);
    fn savedir_diag(name: *const libc::c_char);
    fn stat_diag(name: *const libc::c_char);
    fn file_removed_diag(
        name: *const libc::c_char,
        top_level: bool,
        diagfn: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    );
    fn set_file_atime(
        fd: libc::c_int,
        parentfd: libc::c_int,
        file: *const libc::c_char,
        atime: timespec,
    ) -> libc::c_int;
    static mut filename_args: files_count;
    static mut gnu_list_name: *mut name;
    fn gid_to_gname(gid: gid_t, gname: *mut *mut libc::c_char);
    fn gname_to_gid(gname: *const libc::c_char, pgid: *mut gid_t) -> libc::c_int;
    fn uid_to_uname(uid: uid_t, uname: *mut *mut libc::c_char);
    fn uname_to_uid(uname: *const libc::c_char, puid: *mut uid_t) -> libc::c_int;
    fn name_next(change_dirs: libc::c_int) -> *const libc::c_char;
    fn collect_and_sort_names();
    fn name_from_list() -> *const name;
    fn blank_name_list();
    fn confirm(
        message_action: *const libc::c_char,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn tar_stat_init(st: *mut tar_stat_info);
    fn tar_stat_close(st: *mut tar_stat_info) -> bool;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn set_exit_status(val: libc::c_int);
    fn xheader_store(
        keyword: *const libc::c_char,
        st: *mut tar_stat_info,
        data: *const libc::c_void,
    );
    fn xheader_write(
        type_0: libc::c_char,
        name: *mut libc::c_char,
        t: time_t,
        xhdr: *mut xheader,
    );
    fn xheader_finish(hdr: *mut xheader);
    fn xheader_xhdr_name(st: *mut tar_stat_info) -> *mut libc::c_char;
    fn xheader_ghdr_name() -> *mut libc::c_char;
    fn sys_file_is_archive(p: *mut tar_stat_info) -> bool;
    fn sparse_dump_file(_: libc::c_int, st: *mut tar_stat_info) -> dump_status;
    fn excluded_name(name: *const libc::c_char, st: *mut tar_stat_info) -> bool;
    fn mode_adjust(
        _: mode_t,
        _: bool,
        _: mode_t,
        _: *const mode_change,
        _: *mut mode_t,
    ) -> mode_t;
    fn info_attach_exclist(dir: *mut tar_stat_info);
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn string_ascii_p(str: *const libc::c_char) -> bool;
    fn finish_deferred_unlinks();
    fn transform_name(pinput: *mut *mut libc::c_char, type_0: libc::c_int) -> bool;
    fn queue_deferred_unlink(name: *const libc::c_char, is_dir: bool);
    fn xattrs_acls_get(
        parentfd: libc::c_int,
        file_name: *const libc::c_char,
        st: *mut tar_stat_info,
        fd: libc::c_int,
        xisfile: libc::c_int,
    );
    fn xattrs_selinux_get(
        parentfd: libc::c_int,
        file_name: *const libc::c_char,
        st: *mut tar_stat_info,
        fd: libc::c_int,
    );
    fn xattrs_xattrs_get(
        parentfd: libc::c_int,
        file_name: *const libc::c_char,
        st: *mut tar_stat_info,
        fd: libc::c_int,
    );
    static mut warning_option: libc::c_int;
    fn group_map_translate(
        gid: gid_t,
        new_gid: *mut gid_t,
        new_name: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn owner_map_translate(
        uid: uid_t,
        new_uid: *mut uid_t,
        new_name: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_get_first(table: *const Hash_table) -> *mut libc::c_void;
    fn hash_get_next(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
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
pub type savedir_option = libc::c_uint;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 2;
pub const SAVEDIR_SORT_INODE: savedir_option = 2;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
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
pub type archive_format = libc::c_uint;
pub const GNU_FORMAT: archive_format = 6;
pub const STAR_FORMAT: archive_format = 5;
pub const POSIX_FORMAT: archive_format = 4;
pub const USTAR_FORMAT: archive_format = 3;
pub const OLDGNU_FORMAT: archive_format = 2;
pub const V7_FORMAT: archive_format = 1;
pub const DEFAULT_FORMAT: archive_format = 0;
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
pub type atime_preserve = libc::c_uint;
pub const system_atime_preserve: atime_preserve = 2;
pub const replace_atime_preserve: atime_preserve = 1;
pub const no_atime_preserve: atime_preserve = 0;
pub type exclusion_tag_type = libc::c_uint;
pub const exclusion_tag_all: exclusion_tag_type = 3;
pub const exclusion_tag_under: exclusion_tag_type = 2;
pub const exclusion_tag_contents: exclusion_tag_type = 1;
pub const exclusion_tag_none: exclusion_tag_type = 0;
pub type set_mtime_option_mode = libc::c_uint;
pub const CLAMP_MTIME: set_mtime_option_mode = 2;
pub const FORCE_MTIME: set_mtime_option_mode = 1;
pub const USE_FILE_MTIME: set_mtime_option_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name {
    pub next: *mut name,
    pub prev: *mut name,
    pub name: *mut libc::c_char,
    pub length: size_t,
    pub matching_flags: libc::c_int,
    pub cmdline: bool,
    pub change_dir: libc::c_int,
    pub found_count: uintmax_t,
    pub directory: *mut directory,
    pub parent: *mut name,
    pub child: *mut name,
    pub sibling: *mut name,
    pub caname: *mut libc::c_char,
}
pub type access_mode = libc::c_uint;
pub const ACCESS_UPDATE: access_mode = 2;
pub const ACCESS_WRITE: access_mode = 1;
pub const ACCESS_READ: access_mode = 0;
pub type dump_status = libc::c_uint;
pub const dump_status_not_implemented: dump_status = 3;
pub const dump_status_fail: dump_status = 2;
pub const dump_status_short: dump_status = 1;
pub const dump_status_ok: dump_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exclusion_tag {
    pub name: *const libc::c_char,
    pub length: size_t,
    pub type_0: exclusion_tag_type,
    pub predicate: Option::<unsafe extern "C" fn(libc::c_int) -> bool>,
    pub next: *mut exclusion_tag,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub dev: dev_t,
    pub ino: ino_t,
    pub nlink: nlink_t,
    pub name: [libc::c_char; 1],
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
pub const IMPOSTOR_ERRNO: C2RustUnnamed_2 = 2;
pub const FILES_MANY: files_count = 2;
pub type files_count = libc::c_uint;
pub const FILES_ONE: files_count = 1;
pub const FILES_NONE: files_count = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
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
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> libc::c_uint {
    let mut __major: libc::c_uint = 0;
    __major = ((__dev & 0xfff00 as libc::c_uint as __dev_t) >> 8 as libc::c_int)
        as libc::c_uint;
    __major = (__major as libc::c_ulong
        | (__dev & 0xfffff00000000000 as libc::c_ulong) >> 32 as libc::c_int)
        as libc::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_ctime(mut st: *const stat) -> timespec {
    return (*st).st_ctim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
static mut exclusion_tags: *mut exclusion_tag = 0 as *const exclusion_tag
    as *mut exclusion_tag;
pub unsafe extern "C" fn add_exclusion_tag(
    mut name: *const libc::c_char,
    mut type_0: exclusion_tag_type,
    mut predicate: Option::<unsafe extern "C" fn(libc::c_int) -> bool>,
) {
    let mut tag: *mut exclusion_tag = xmalloc(
        ::std::mem::size_of::<exclusion_tag>() as libc::c_ulong,
    ) as *mut exclusion_tag;
    (*tag).next = exclusion_tags;
    (*tag).name = name;
    (*tag).type_0 = type_0;
    (*tag).predicate = predicate;
    (*tag).length = strlen(name);
    exclusion_tags = tag;
}
pub unsafe extern "C" fn exclusion_tag_warning(
    mut dirname: *const libc::c_char,
    mut tagname: *const libc::c_char,
    mut message: *const libc::c_char,
) {
    if verbose_option != 0 {
        if warning_option & 0x4 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: contains a cache directory tag %s; %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(dirname),
                quotearg_n(1 as libc::c_int, tagname),
                message,
            );
        }
    }
}
pub unsafe extern "C" fn check_exclusion_tags(
    mut st: *const tar_stat_info,
    mut tag_file_name: *mut *const libc::c_char,
) -> exclusion_tag_type {
    let mut tag: *mut exclusion_tag = 0 as *mut exclusion_tag;
    tag = exclusion_tags;
    while !tag.is_null() {
        let mut tagfd: libc::c_int = subfile_open(st, (*tag).name, open_read_flags);
        if 0 as libc::c_int <= tagfd {
            let mut satisfied: bool = ((*tag).predicate).is_none()
                || ((*tag).predicate).unwrap()(tagfd) as libc::c_int != 0;
            close(tagfd);
            if satisfied {
                if !tag_file_name.is_null() {
                    *tag_file_name = (*tag).name;
                }
                return (*tag).type_0;
            }
        }
        tag = (*tag).next;
    }
    return exclusion_tag_none;
}
pub unsafe extern "C" fn cachedir_file_p(mut fd: libc::c_int) -> bool {
    let mut tagbuf: [libc::c_char; 43] = [0; 43];
    return read(
        fd,
        tagbuf.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 44]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as libc::c_ulong
        == (::std::mem::size_of::<[libc::c_char; 44]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && memcmp(
            tagbuf.as_mut_ptr() as *const libc::c_void,
            b"Signature: 8a477f597d28d172789f06886806bc55\0" as *const u8
                as *const libc::c_char as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 44]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int;
}
unsafe extern "C" fn to_octal(
    mut value: uintmax_t,
    mut where_0: *mut libc::c_char,
    mut size: size_t,
) {
    let mut v: uintmax_t = value;
    let mut i: size_t = size;
    loop {
        i = i.wrapping_sub(1);
        *where_0
            .offset(
                i as isize,
            ) = ('0' as i32 as libc::c_ulong)
            .wrapping_add(
                v
                    & (((1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_ulong,
            ) as libc::c_char;
        v >>= 3 as libc::c_int;
        if !(i != 0) {
            break;
        }
    };
}
unsafe extern "C" fn tar_copy_str(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        let ref mut fresh0 = *dst.offset(i as isize);
        *fresh0 = *src.offset(i as isize);
        if *fresh0 == 0 {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn tar_name_copy_str(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
) {
    tar_copy_str(dst, src, len);
    if archive_format as libc::c_uint == OLDGNU_FORMAT as libc::c_int as libc::c_uint {
        *dst
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
}
unsafe extern "C" fn to_base256(
    mut negative: libc::c_int,
    mut value: uintmax_t,
    mut where_0: *mut libc::c_char,
    mut size: size_t,
) {
    let mut v: uintmax_t = value;
    let mut propagated_sign_bits: uintmax_t = (-negative as uintmax_t)
        << (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong);
    let mut i: size_t = size;
    loop {
        i = i.wrapping_sub(1);
        *where_0
            .offset(
                i as isize,
            ) = (v
            & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                as libc::c_ulong) as libc::c_char;
        v = propagated_sign_bits | v >> 8 as libc::c_int;
        if !(i != 0) {
            break;
        }
    };
}
unsafe extern "C" fn to_chars_subst(
    mut negative: libc::c_int,
    mut gnu_format: libc::c_int,
    mut value: uintmax_t,
    mut valsize: size_t,
    mut substitute: Option::<unsafe extern "C" fn(*mut libc::c_int) -> uintmax_t>,
    mut where_0: *mut libc::c_char,
    mut size: size_t,
    mut type_0: *const libc::c_char,
) -> bool {
    let mut maxval: uintmax_t = if gnu_format != 0 {
        if size
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            ((1 as libc::c_int as uintmax_t)
                << size
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            -(1 as libc::c_int) as uintmax_t
        }
    } else if size
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
        < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        ((1 as libc::c_int as uintmax_t)
            << size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    } else {
        -(1 as libc::c_int) as uintmax_t
    };
    let mut valbuf: [libc::c_char; 22] = [0; 22];
    let mut maxbuf: [libc::c_char; 21] = [0; 21];
    let mut minbuf: [libc::c_char; 22] = [0; 22];
    let mut minval_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut maxval_string: *const libc::c_char = umaxtostr(maxval, maxbuf.as_mut_ptr());
    let mut value_string: *const libc::c_char = 0 as *const libc::c_char;
    if gnu_format != 0 {
        let mut m: uintmax_t = if maxval.wrapping_add(1 as libc::c_int as libc::c_ulong)
            != 0
        {
            maxval.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            maxval
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        };
        let mut p: *mut libc::c_char = umaxtostr(
            m,
            minbuf.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
        minval_string = p;
    } else {
        minval_string = b"0\0" as *const u8 as *const libc::c_char;
    }
    if negative != 0 {
        let mut p_0: *mut libc::c_char = umaxtostr(
            value.wrapping_neg(),
            valbuf.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        p_0 = p_0.offset(-1);
        *p_0 = '-' as i32 as libc::c_char;
        value_string = p_0;
    } else {
        value_string = umaxtostr(value, valbuf.as_mut_ptr());
    }
    if substitute.is_some() {
        let mut negsub: libc::c_int = 0;
        let mut sub: uintmax_t = substitute.unwrap()(&mut negsub) & maxval;
        negsub
            &= (archive_format as libc::c_uint
                == GNU_FORMAT as libc::c_int as libc::c_uint) as libc::c_int;
        let mut s: uintmax_t = if negsub != 0 { sub.wrapping_neg() } else { sub };
        let mut subbuf: [libc::c_char; 22] = [0; 22];
        let mut sub_string: *mut libc::c_char = umaxtostr(
            s,
            subbuf.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        if negsub != 0 {
            sub_string = sub_string.offset(-1);
            *sub_string = '-' as i32 as libc::c_char;
        }
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"value %s out of %s range %s..%s; substituting %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            value_string,
            type_0,
            minval_string,
            maxval_string,
            sub_string,
        );
        return to_chars(negsub, s, valsize, None, where_0, size, type_0);
    } else {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"value %s out of %s range %s..%s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            value_string,
            type_0,
            minval_string,
            maxval_string,
        );
        exit_status = 2 as libc::c_int;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn to_chars(
    mut negative: libc::c_int,
    mut value: uintmax_t,
    mut valsize: size_t,
    mut substitute: Option::<unsafe extern "C" fn(*mut libc::c_int) -> uintmax_t>,
    mut where_0: *mut libc::c_char,
    mut size: size_t,
    mut type_0: *const libc::c_char,
) -> bool {
    let mut gnu_format: libc::c_int = (archive_format as libc::c_uint
        == GNU_FORMAT as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint
            == OLDGNU_FORMAT as libc::c_int as libc::c_uint) as libc::c_int;
    if negative == 0
        && value
            <= (if size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                ((1 as libc::c_int as uintmax_t)
                    << size
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                -(1 as libc::c_int) as uintmax_t
            })
    {
        *where_0
            .offset(
                size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        to_octal(value, where_0, size.wrapping_sub(1 as libc::c_int as libc::c_ulong));
        return 1 as libc::c_int != 0;
    } else if gnu_format != 0 {
        if (if negative != 0 {
            (-(1 as libc::c_int) as libc::c_ulong).wrapping_sub(value)
        } else {
            value
        })
            <= (if size
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                ((1 as libc::c_int as uintmax_t)
                    << size
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                -(1 as libc::c_int) as uintmax_t
            })
        {
            *where_0
                .offset(
                    0 as libc::c_int as isize,
                ) = (if negative != 0 {
                -(1 as libc::c_int)
            } else {
                (1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int
            }) as libc::c_char;
            to_base256(
                negative,
                value,
                where_0.offset(1 as libc::c_int as isize),
                size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            return 1 as libc::c_int != 0;
        } else if negative != 0
            && valsize.wrapping_mul(8 as libc::c_int as libc::c_ulong)
                <= size
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong)
        {
            static mut warned_once: libc::c_int = 0;
            if warned_once == 0 {
                warned_once = 1 as libc::c_int;
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Generating negative octal headers\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            *where_0
                .offset(
                    size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            to_octal(
                value
                    & (if valsize
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                        < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    {
                        ((1 as libc::c_int as uintmax_t)
                            << valsize
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(1 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) as uintmax_t
                    }),
                where_0,
                size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            return 1 as libc::c_int != 0;
        }
    } else {
        substitute = None;
    }
    return to_chars_subst(
        negative,
        gnu_format,
        value,
        valsize,
        substitute,
        where_0,
        size,
        type_0,
    );
}
unsafe extern "C" fn gid_substitute(mut negative: *mut libc::c_int) -> uintmax_t {
    let mut r: gid_t = 0;
    static mut gid_nobody: gid_t = 0;
    if gid_nobody == 0
        && gname_to_gid(b"nobody\0" as *const u8 as *const libc::c_char, &mut gid_nobody)
            == 0
    {
        gid_nobody = -(2 as libc::c_int) as gid_t;
    }
    r = gid_nobody;
    *negative = (r < 0 as libc::c_int as libc::c_uint) as libc::c_int;
    return r as uintmax_t;
}
unsafe extern "C" fn gid_to_chars(
    mut v: gid_t,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    return to_chars(
        (v < 0 as libc::c_int as libc::c_uint) as libc::c_int,
        v as uintmax_t,
        ::std::mem::size_of::<gid_t>() as libc::c_ulong,
        Some(gid_substitute as unsafe extern "C" fn(*mut libc::c_int) -> uintmax_t),
        p,
        s,
        b"gid_t\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn major_to_chars(
    mut v: libc::c_int,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    return to_chars(
        (v < 0 as libc::c_int) as libc::c_int,
        v as uintmax_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        None,
        p,
        s,
        b"major_t\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn minor_to_chars(
    mut v: libc::c_int,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    return to_chars(
        (v < 0 as libc::c_int) as libc::c_int,
        v as uintmax_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        None,
        p,
        s,
        b"minor_t\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn mode_to_chars(
    mut v: mode_t,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    let mut negative: libc::c_int = 0;
    let mut u: uintmax_t = 0;
    if 0o4000 as libc::c_int == 0o4000 as libc::c_int
        && 0o2000 as libc::c_int == 0o2000 as libc::c_int
        && 0o1000 as libc::c_int == 0o1000 as libc::c_int
        && 0o400 as libc::c_int == 0o400 as libc::c_int
        && 0o200 as libc::c_int == 0o200 as libc::c_int
        && 0o100 as libc::c_int == 0o100 as libc::c_int
        && 0o400 as libc::c_int >> 3 as libc::c_int == 0o40 as libc::c_int
        && 0o200 as libc::c_int >> 3 as libc::c_int == 0o20 as libc::c_int
        && 0o100 as libc::c_int >> 3 as libc::c_int == 0o10 as libc::c_int
        && 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            == 0o4 as libc::c_int
        && 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            == 0o2 as libc::c_int
        && 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            == 0o1 as libc::c_int
        && archive_format as libc::c_uint != POSIX_FORMAT as libc::c_int as libc::c_uint
        && archive_format as libc::c_uint != USTAR_FORMAT as libc::c_int as libc::c_uint
        && archive_format as libc::c_uint != GNU_FORMAT as libc::c_int as libc::c_uint
    {
        negative = (v < 0 as libc::c_int as libc::c_uint) as libc::c_int;
        u = v as uintmax_t;
    } else {
        negative = 0 as libc::c_int;
        u = ((if v & 0o4000 as libc::c_int as libc::c_uint != 0 {
            0o4000 as libc::c_int
        } else {
            0 as libc::c_int
        })
            | (if v & 0o2000 as libc::c_int as libc::c_uint != 0 {
                0o2000 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v & 0o1000 as libc::c_int as libc::c_uint != 0 {
                0o1000 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v & 0o400 as libc::c_int as libc::c_uint != 0 {
                0o400 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v & 0o200 as libc::c_int as libc::c_uint != 0 {
                0o200 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v & 0o100 as libc::c_int as libc::c_uint != 0 {
                0o100 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v & (0o400 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
                0o40 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
                0o20 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v & (0o100 as libc::c_int >> 3 as libc::c_int) as libc::c_uint != 0 {
                0o10 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v
                & (0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint != 0
            {
                0o4 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v
                & (0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint != 0
            {
                0o2 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if v
                & (0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint != 0
            {
                0o1 as libc::c_int
            } else {
                0 as libc::c_int
            })) as uintmax_t;
    }
    return to_chars(
        negative,
        u,
        ::std::mem::size_of::<mode_t>() as libc::c_ulong,
        None,
        p,
        s,
        b"mode_t\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn off_to_chars(
    mut v: off_t,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    return to_chars(
        (v < 0 as libc::c_int as libc::c_long) as libc::c_int,
        v as uintmax_t,
        ::std::mem::size_of::<off_t>() as libc::c_ulong,
        None,
        p,
        s,
        b"off_t\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn time_to_chars(
    mut v: time_t,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    return to_chars(
        (v < 0 as libc::c_int as libc::c_long) as libc::c_int,
        v as uintmax_t,
        ::std::mem::size_of::<time_t>() as libc::c_ulong,
        None,
        p,
        s,
        b"time_t\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn uid_substitute(mut negative: *mut libc::c_int) -> uintmax_t {
    let mut r: uid_t = 0;
    static mut uid_nobody: uid_t = 0;
    if uid_nobody == 0
        && uname_to_uid(b"nobody\0" as *const u8 as *const libc::c_char, &mut uid_nobody)
            == 0
    {
        uid_nobody = -(2 as libc::c_int) as uid_t;
    }
    r = uid_nobody;
    *negative = (r < 0 as libc::c_int as libc::c_uint) as libc::c_int;
    return r as uintmax_t;
}
unsafe extern "C" fn uid_to_chars(
    mut v: uid_t,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    return to_chars(
        (v < 0 as libc::c_int as libc::c_uint) as libc::c_int,
        v as uintmax_t,
        ::std::mem::size_of::<uid_t>() as libc::c_ulong,
        Some(uid_substitute as unsafe extern "C" fn(*mut libc::c_int) -> uintmax_t),
        p,
        s,
        b"uid_t\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn uintmax_to_chars(
    mut v: uintmax_t,
    mut p: *mut libc::c_char,
    mut s: size_t,
) -> bool {
    return to_chars(
        0 as libc::c_int,
        v,
        ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
        None,
        p,
        s,
        b"uintmax_t\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn string_to_chars(
    mut str: *const libc::c_char,
    mut p: *mut libc::c_char,
    mut s: size_t,
) {
    tar_copy_str(p, str, s);
    *p
        .offset(
            s.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn file_dumpable_p(mut st: *const stat) -> bool {
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    if !((*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint || 0 as libc::c_int != 0)
    {
        return 0 as libc::c_int != 0;
    }
    if dev_null_output {
        return totals_option as libc::c_int != 0 && sparse_option as libc::c_int != 0
            && (*st).st_blocks
                < (*st).st_size / 512 as libc::c_int as libc::c_long
                    + ((*st).st_size % 512 as libc::c_int as libc::c_long
                        != 0 as libc::c_int as libc::c_long
                        && (*st).st_size / 512 as libc::c_int as libc::c_long
                            != 0 as libc::c_int as libc::c_long) as libc::c_int
                        as libc::c_long;
    }
    return !((*st).st_size == 0 as libc::c_int as libc::c_long
        && (*st).st_mode
            & (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint
            == (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint);
}
pub unsafe extern "C" fn write_eot() {
    let mut pointer: *mut block = find_next_block();
    memset(
        ((*pointer).buffer).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        512 as libc::c_int as libc::c_ulong,
    );
    set_next_block_after(pointer);
    pointer = find_next_block();
    memset(
        ((*pointer).buffer).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        available_space_after(pointer),
    );
    set_next_block_after(pointer);
}
pub unsafe extern "C" fn start_private_header(
    mut name: *const libc::c_char,
    mut size: size_t,
    mut t: time_t,
) -> *mut block {
    let mut header: *mut block = find_next_block();
    memset(
        ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<block>() as libc::c_ulong,
    );
    tar_name_copy_str(
        ((*header).header.name).as_mut_ptr(),
        name,
        100 as libc::c_int as size_t,
    );
    off_to_chars(
        size as off_t,
        ((*header).header.size).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    time_to_chars(
        (if t < 0 as libc::c_int as libc::c_long {
            0 as libc::c_int as libc::c_ulong
        } else if (t as libc::c_ulong)
            < (if (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                ((1 as libc::c_int as uintmax_t)
                    << (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                -(1 as libc::c_int) as uintmax_t
            })
        {
            t as libc::c_ulong
        } else if (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            -(1 as libc::c_int) as uintmax_t
        }) as time_t,
        ((*header).header.mtime).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    mode_to_chars(
        (0o100000 as libc::c_int | 0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
        ((*header).header.mode).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    uid_to_chars(
        0 as libc::c_int as uid_t,
        ((*header).header.uid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    gid_to_chars(
        0 as libc::c_int as gid_t,
        ((*header).header.gid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    memcpy(
        ((*header).header.magic).as_mut_ptr() as *mut libc::c_void,
        b"ustar\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        ((*header).header.version).as_mut_ptr() as *mut libc::c_void,
        b"00\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    return header;
}
unsafe extern "C" fn write_short_name(mut st: *mut tar_stat_info) -> *mut block {
    let mut header: *mut block = find_next_block();
    memset(
        ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<block>() as libc::c_ulong,
    );
    tar_name_copy_str(
        ((*header).header.name).as_mut_ptr(),
        (*st).file_name,
        100 as libc::c_int as size_t,
    );
    return header;
}
unsafe extern "C" fn write_gnu_long_link(
    mut st: *mut tar_stat_info,
    mut p: *const libc::c_char,
    mut type_0: libc::c_char,
) {
    let mut size: size_t = (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut bufsize: size_t = 0;
    let mut header: *mut block = 0 as *mut block;
    header = start_private_header(
        b"././@LongLink\0" as *const u8 as *const libc::c_char,
        size,
        0 as libc::c_int as time_t,
    );
    if !numeric_owner_option {
        static mut uname: *mut libc::c_char = 0 as *const libc::c_char
            as *mut libc::c_char;
        static mut gname: *mut libc::c_char = 0 as *const libc::c_char
            as *mut libc::c_char;
        if uname.is_null() {
            uid_to_uname(0 as libc::c_int as uid_t, &mut uname);
            gid_to_gname(0 as libc::c_int as gid_t, &mut gname);
        }
        string_to_chars(
            uname,
            ((*header).header.uname).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
        string_to_chars(
            gname,
            ((*header).header.gname).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    strcpy(
        ((*header).buffer).as_mut_ptr().offset(257 as libc::c_ulong as isize),
        b"ustar  \0" as *const u8 as *const libc::c_char,
    );
    (*header).header.typeflag = type_0;
    finish_header(st, header, -(1 as libc::c_int) as off_t);
    header = find_next_block();
    bufsize = available_space_after(header);
    while bufsize < size {
        memcpy(
            ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            bufsize,
        );
        p = p.offset(bufsize as isize);
        size = (size as libc::c_ulong).wrapping_sub(bufsize) as size_t as size_t;
        set_next_block_after(
            header
                .offset(
                    bufsize
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
                ),
        );
        header = find_next_block();
        bufsize = available_space_after(header);
    }
    memcpy(
        ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        size,
    );
    memset(
        ((*header).buffer).as_mut_ptr().offset(size as isize) as *mut libc::c_void,
        0 as libc::c_int,
        bufsize.wrapping_sub(size),
    );
    set_next_block_after(
        header
            .offset(
                size
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
            ),
    );
}
unsafe extern "C" fn split_long_name(
    mut name: *const libc::c_char,
    mut length: size_t,
) -> size_t {
    let mut i: size_t = 0;
    if length > (155 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        length = (155 as libc::c_int + 1 as libc::c_int) as size_t;
    } else if *name
        .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '/' as i32
    {
        length = length.wrapping_sub(1);
        length;
    }
    i = length.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > 0 as libc::c_int as libc::c_ulong {
        if *name.offset(i as isize) as libc::c_int == '/' as i32 {
            break;
        }
        i = i.wrapping_sub(1);
        i;
    }
    return i;
}
unsafe extern "C" fn write_ustar_long_name(mut name: *const libc::c_char) -> *mut block {
    let mut length: size_t = strlen(name);
    let mut i: size_t = 0;
    let mut nlen: size_t = 0;
    let mut header: *mut block = 0 as *mut block;
    if length
        > (155 as libc::c_int + 100 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: file name is too long (max %d); not dumped\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_colon(name),
            155 as libc::c_int + 100 as libc::c_int + 1 as libc::c_int,
        );
        exit_status = 2 as libc::c_int;
        return 0 as *mut block;
    }
    i = split_long_name(name, length);
    if i == 0 as libc::c_int as libc::c_ulong
        || {
            nlen = length
                .wrapping_sub(i)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            nlen > 100 as libc::c_int as libc::c_ulong
        } || nlen == 0 as libc::c_int as libc::c_ulong
    {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: file name is too long (cannot be split); not dumped\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_colon(name),
        );
        exit_status = 2 as libc::c_int;
        return 0 as *mut block;
    }
    header = find_next_block();
    memset(
        ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    memcpy(
        ((*header).header.prefix).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        i,
    );
    memcpy(
        ((*header).header.name).as_mut_ptr() as *mut libc::c_void,
        name.offset(i as isize).offset(1 as libc::c_int as isize) as *const libc::c_void,
        length.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    return header;
}
unsafe extern "C" fn write_long_link(mut st: *mut tar_stat_info) {
    match archive_format as libc::c_uint {
        4 => {
            xheader_store(
                b"linkpath\0" as *const u8 as *const libc::c_char,
                st,
                0 as *const libc::c_void,
            );
        }
        1 | 3 | 5 => {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: link name is too long; not dumped\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon((*st).link_name),
            );
            exit_status = 2 as libc::c_int;
        }
        2 | 6 => {
            write_gnu_long_link(st, (*st).link_name, 'K' as i32 as libc::c_char);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn write_long_name(mut st: *mut tar_stat_info) -> *mut block {
    match archive_format as libc::c_uint {
        4 => {
            xheader_store(
                b"path\0" as *const u8 as *const libc::c_char,
                st,
                0 as *const libc::c_void,
            );
        }
        1 => {
            if strlen((*st).file_name)
                > (100 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: file name is too long (max %d); not dumped\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_colon((*st).file_name),
                    100 as libc::c_int - 1 as libc::c_int,
                );
                exit_status = 2 as libc::c_int;
                return 0 as *mut block;
            }
        }
        3 | 5 => return write_ustar_long_name((*st).file_name),
        2 | 6 => {
            write_gnu_long_link(st, (*st).file_name, 'L' as i32 as libc::c_char);
        }
        _ => {
            abort();
        }
    }
    return write_short_name(st);
}
pub unsafe extern "C" fn write_extended(
    mut global: bool,
    mut st: *mut tar_stat_info,
    mut old_header: *mut block,
) -> *mut block {
    let mut header: *mut block = 0 as *mut block;
    let mut hp: block = block { buffer: [0; 512] };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    let mut t: time_t = 0;
    if !((*st).xhdr.buffer).is_null() || ((*st).xhdr.stk).is_null() {
        return old_header;
    }
    xheader_finish(&mut (*st).xhdr);
    memcpy(
        (hp.buffer).as_mut_ptr() as *mut libc::c_void,
        old_header as *const libc::c_void,
        ::std::mem::size_of::<block>() as libc::c_ulong,
    );
    if global {
        type_0 = 'g' as i32;
        p = xheader_ghdr_name();
        t = start_time.tv_sec;
    } else {
        type_0 = 'x' as i32;
        p = xheader_xhdr_name(st);
        t = if set_mtime_option as libc::c_uint != 0 {
            mtime_option.tv_sec
        } else {
            (*st).stat.st_mtim.tv_sec
        };
    }
    xheader_write(type_0 as libc::c_char, p, t, &mut (*st).xhdr);
    rpl_free(p as *mut libc::c_void);
    header = find_next_block();
    memcpy(
        header as *mut libc::c_void,
        &mut hp.buffer as *mut [libc::c_char; 512] as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
    );
    return header;
}
unsafe extern "C" fn write_header_name(mut st: *mut tar_stat_info) -> *mut block {
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
        && !string_ascii_p((*st).file_name)
    {
        xheader_store(
            b"path\0" as *const u8 as *const libc::c_char,
            st,
            0 as *const libc::c_void,
        );
        return write_short_name(st);
    } else if ((100 as libc::c_int
        - (archive_format as libc::c_uint
            == OLDGNU_FORMAT as libc::c_int as libc::c_uint) as libc::c_int)
        as libc::c_ulong) < strlen((*st).file_name)
    {
        return write_long_name(st)
    } else {
        return write_short_name(st)
    };
}
pub unsafe extern "C" fn start_header(mut st: *mut tar_stat_info) -> *mut block {
    let mut header: *mut block = 0 as *mut block;
    let mut uname: *const libc::c_char = 0 as *const libc::c_char;
    let mut gname: *const libc::c_char = 0 as *const libc::c_char;
    header = write_header_name(st);
    if header.is_null() {
        return 0 as *mut block;
    }
    owner_map_translate((*st).stat.st_uid, &mut (*st).stat.st_uid, &mut uname);
    group_map_translate((*st).stat.st_gid, &mut (*st).stat.st_gid, &mut gname);
    if !mode_option.is_null() {
        (*st)
            .stat
            .st_mode = (*st).stat.st_mode
            & !(0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int)))) as libc::c_uint
            | mode_adjust(
                (*st).stat.st_mode,
                ((*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int
                    != 0 as libc::c_int,
                initial_umask,
                mode_option,
                0 as *mut mode_t,
            );
    }
    if archive_format as libc::c_uint == V7_FORMAT as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == USTAR_FORMAT as libc::c_int as libc::c_uint
    {
        mode_to_chars(
            (*st).stat.st_mode
                & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                    | (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o200 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                            | 0o200 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int
                            | (0o400 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                | 0o400 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int)))) as libc::c_uint,
            ((*header).header.mode).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
    } else {
        mode_to_chars(
            (*st).stat.st_mode,
            ((*header).header.mode).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
    }
    let mut uid: uid_t = (*st).stat.st_uid;
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
        && (if (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            -(1 as libc::c_int) as uintmax_t
        }) < uid as libc::c_ulong
    {
        xheader_store(
            b"uid\0" as *const u8 as *const libc::c_char,
            st,
            0 as *const libc::c_void,
        );
        uid = 0 as libc::c_int as uid_t;
    }
    if !uid_to_chars(
        uid,
        ((*header).header.uid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    ) {
        return 0 as *mut block;
    }
    let mut gid: gid_t = (*st).stat.st_gid;
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
        && (if (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            -(1 as libc::c_int) as uintmax_t
        }) < gid as libc::c_ulong
    {
        xheader_store(
            b"gid\0" as *const u8 as *const libc::c_char,
            st,
            0 as *const libc::c_void,
        );
        gid = 0 as libc::c_int as gid_t;
    }
    if !gid_to_chars(
        gid,
        ((*header).header.gid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    ) {
        return 0 as *mut block;
    }
    let mut size: off_t = (*st).stat.st_size;
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
        && (if (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            -(1 as libc::c_int) as uintmax_t
        }) < size as libc::c_ulong
    {
        xheader_store(
            b"size\0" as *const u8 as *const libc::c_char,
            st,
            0 as *const libc::c_void,
        );
        size = 0 as libc::c_int as off_t;
    }
    if !off_to_chars(
        size,
        ((*header).header.size).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    ) {
        return 0 as *mut block;
    }
    let mut mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    match set_mtime_option as libc::c_uint {
        0 => {
            mtime = (*st).mtime;
        }
        1 => {
            mtime = mtime_option;
        }
        2 => {
            mtime = if timespec_cmp((*st).mtime, mtime_option) > 0 as libc::c_int {
                mtime_option
            } else {
                (*st).mtime
            };
        }
        _ => {}
    }
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint {
        if (if (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            -(1 as libc::c_int) as uintmax_t
        }) < mtime.tv_sec as libc::c_ulong
            || mtime.tv_nsec != 0 as libc::c_int as libc::c_long
        {
            xheader_store(
                b"mtime\0" as *const u8 as *const libc::c_char,
                st,
                &mut mtime as *mut timespec as *const libc::c_void,
            );
        }
        if (if (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            -(1 as libc::c_int) as uintmax_t
        }) < mtime.tv_sec as libc::c_ulong
        {
            mtime.tv_sec = 0 as libc::c_int as __time_t;
        }
    }
    if !time_to_chars(
        mtime.tv_sec,
        ((*header).header.mtime).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    ) {
        return 0 as *mut block;
    }
    if (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
        || (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint
    {
        let mut devmajor: libc::c_int = gnu_dev_major((*st).stat.st_rdev) as libc::c_int;
        let mut devminor: libc::c_int = gnu_dev_minor((*st).stat.st_rdev) as libc::c_int;
        if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
            && (if (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                ((1 as libc::c_int as uintmax_t)
                    << (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                -(1 as libc::c_int) as uintmax_t
            }) < devmajor as libc::c_ulong
        {
            xheader_store(
                b"devmajor\0" as *const u8 as *const libc::c_char,
                st,
                0 as *const libc::c_void,
            );
            devmajor = 0 as libc::c_int;
        }
        if !major_to_chars(
            devmajor,
            ((*header).header.devmajor).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        ) {
            return 0 as *mut block;
        }
        if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
            && (if (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                ((1 as libc::c_int as uintmax_t)
                    << (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                -(1 as libc::c_int) as uintmax_t
            }) < devminor as libc::c_ulong
        {
            xheader_store(
                b"devminor\0" as *const u8 as *const libc::c_char,
                st,
                0 as *const libc::c_void,
            );
            devminor = 0 as libc::c_int;
        }
        if !minor_to_chars(
            devminor,
            ((*header).header.devminor).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        ) {
            return 0 as *mut block;
        }
    } else if archive_format as libc::c_uint != GNU_FORMAT as libc::c_int as libc::c_uint
        && archive_format as libc::c_uint != OLDGNU_FORMAT as libc::c_int as libc::c_uint
    {
        if !(major_to_chars(
            0 as libc::c_int,
            ((*header).header.devmajor).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        ) as libc::c_int != 0
            && minor_to_chars(
                0 as libc::c_int,
                ((*header).header.devminor).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) as libc::c_int != 0)
        {
            return 0 as *mut block;
        }
    }
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint {
        xheader_store(
            b"atime\0" as *const u8 as *const libc::c_char,
            st,
            0 as *const libc::c_void,
        );
        xheader_store(
            b"ctime\0" as *const u8 as *const libc::c_char,
            st,
            0 as *const libc::c_void,
        );
    } else if incremental_option {
        if archive_format as libc::c_uint == OLDGNU_FORMAT as libc::c_int as libc::c_uint
            || archive_format as libc::c_uint
                == GNU_FORMAT as libc::c_int as libc::c_uint
        {
            time_to_chars(
                (*st).atime.tv_sec,
                ((*header).oldgnu_header.atime).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            );
            time_to_chars(
                (*st).ctime.tv_sec,
                ((*header).oldgnu_header.ctime).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            );
        }
    }
    (*header)
        .header
        .typeflag = (if archive_format as libc::c_uint
        == V7_FORMAT as libc::c_int as libc::c_uint
    {
        '\0' as i32
    } else {
        '0' as i32
    }) as libc::c_char;
    match archive_format as libc::c_uint {
        1 => {}
        2 | 6 => {
            strcpy(
                ((*header).buffer).as_mut_ptr().offset(257 as libc::c_ulong as isize),
                b"ustar  \0" as *const u8 as *const libc::c_char,
            );
        }
        4 | 3 => {
            memcpy(
                ((*header).header.magic).as_mut_ptr() as *mut libc::c_void,
                b"ustar\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            );
            memcpy(
                ((*header).header.version).as_mut_ptr() as *mut libc::c_void,
                b"00\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
        }
        _ => {
            abort();
        }
    }
    if !(archive_format as libc::c_uint == V7_FORMAT as libc::c_int as libc::c_uint
        || numeric_owner_option as libc::c_int != 0)
    {
        if !uname.is_null() {
            (*st).uname = xstrdup(uname);
        } else {
            uid_to_uname((*st).stat.st_uid, &mut (*st).uname);
        }
        if !gname.is_null() {
            (*st).gname = xstrdup(gname);
        } else {
            gid_to_gname((*st).stat.st_gid, &mut (*st).gname);
        }
        if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
            && (strlen((*st).uname) > 32 as libc::c_int as libc::c_ulong
                || !string_ascii_p((*st).uname))
        {
            xheader_store(
                b"uname\0" as *const u8 as *const libc::c_char,
                st,
                0 as *const libc::c_void,
            );
        }
        string_to_chars(
            (*st).uname,
            ((*header).header.uname).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
        if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
            && (strlen((*st).gname) > 32 as libc::c_int as libc::c_ulong
                || !string_ascii_p((*st).gname))
        {
            xheader_store(
                b"gname\0" as *const u8 as *const libc::c_char,
                st,
                0 as *const libc::c_void,
            );
        }
        string_to_chars(
            (*st).gname,
            ((*header).header.gname).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint {
        if acls_option > 0 as libc::c_int {
            if !((*st).acls_a_ptr).is_null() {
                xheader_store(
                    b"SCHILY.acl.access\0" as *const u8 as *const libc::c_char,
                    st,
                    0 as *const libc::c_void,
                );
            }
            if !((*st).acls_d_ptr).is_null() {
                xheader_store(
                    b"SCHILY.acl.default\0" as *const u8 as *const libc::c_char,
                    st,
                    0 as *const libc::c_void,
                );
            }
        }
        if selinux_context_option > 0 as libc::c_int && !((*st).cntx_name).is_null() {
            xheader_store(
                b"RHT.security.selinux\0" as *const u8 as *const libc::c_char,
                st,
                0 as *const libc::c_void,
            );
        }
        if xattrs_option > 0 as libc::c_int {
            let mut scan_xattr: size_t = 0 as libc::c_int as size_t;
            let mut xattr_map: *mut xattr_array = (*st).xattr_map;
            while scan_xattr < (*st).xattr_map_size {
                xheader_store(
                    (*xattr_map.offset(scan_xattr as isize)).xkey,
                    st,
                    &mut scan_xattr as *mut size_t as *const libc::c_void,
                );
                scan_xattr = scan_xattr.wrapping_add(1);
                scan_xattr;
            }
        }
    }
    return header;
}
pub unsafe extern "C" fn simple_finish_header(mut header: *mut block) {
    let mut i: size_t = 0;
    let mut sum: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    memcpy(
        ((*header).header.chksum).as_mut_ptr() as *mut libc::c_void,
        b"        \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    sum = 0 as libc::c_int;
    p = ((*header).buffer).as_mut_ptr();
    i = ::std::mem::size_of::<block>() as libc::c_ulong;
    loop {
        let fresh1 = i;
        i = i.wrapping_sub(1);
        if !(fresh1 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let fresh2 = p;
        p = p.offset(1);
        sum += 0xff as libc::c_int & *fresh2 as libc::c_int;
    }
    uintmax_to_chars(
        sum as uintmax_t,
        ((*header).header.chksum).as_mut_ptr(),
        7 as libc::c_int as size_t,
    );
    set_next_block_after(header);
}
pub unsafe extern "C" fn finish_header(
    mut st: *mut tar_stat_info,
    mut header: *mut block,
    mut block_ordinal: off_t,
) {
    if verbose_option != 0 && (*header).header.typeflag as libc::c_int != 'K' as i32
        && (*header).header.typeflag as libc::c_int != 'L' as i32
        && (*header).header.typeflag as libc::c_int != 'x' as i32
        && (*header).header.typeflag as libc::c_int != 'g' as i32
    {
        current_format = archive_format;
        print_header(st, header, block_ordinal);
    }
    header = write_extended(0 as libc::c_int != 0, st, header);
    simple_finish_header(header);
}
pub unsafe extern "C" fn pad_archive(mut size_left: off_t) {
    let mut blk: *mut block = 0 as *mut block;
    while size_left > 0 as libc::c_int as libc::c_long {
        blk = find_next_block();
        memset(
            ((*blk).buffer).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            512 as libc::c_int as libc::c_ulong,
        );
        set_next_block_after(blk);
        size_left -= 512 as libc::c_int as libc::c_long;
    }
}
unsafe extern "C" fn dump_regular_file(
    mut fd: libc::c_int,
    mut st: *mut tar_stat_info,
) -> dump_status {
    let mut size_left: off_t = (*st).stat.st_size;
    let mut block_ordinal: off_t = 0;
    let mut blk: *mut block = 0 as *mut block;
    block_ordinal = current_block_ordinal();
    blk = start_header(st);
    if blk.is_null() {
        return dump_status_fail;
    }
    if archive_format as libc::c_uint != V7_FORMAT as libc::c_int as libc::c_uint
        && 0 as libc::c_int != 0
    {
        (*blk).header.typeflag = '7' as i32 as libc::c_char;
    }
    finish_header(st, blk, block_ordinal);
    mv_begin_write((*st).file_name, (*st).stat.st_size, (*st).stat.st_size);
    while size_left > 0 as libc::c_int as libc::c_long {
        let mut bufsize: size_t = 0;
        let mut count: size_t = 0;
        blk = find_next_block();
        bufsize = available_space_after(blk);
        if (size_left as libc::c_ulong) < bufsize {
            bufsize = size_left as size_t;
            count = bufsize.wrapping_rem(512 as libc::c_int as libc::c_ulong);
            if count != 0 {
                memset(
                    ((*blk).buffer).as_mut_ptr().offset(size_left as isize)
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    (512 as libc::c_int as libc::c_ulong).wrapping_sub(count),
                );
            }
        }
        count = if fd <= 0 as libc::c_int {
            bufsize
        } else {
            blocking_read(fd, ((*blk).buffer).as_mut_ptr() as *mut libc::c_void, bufsize)
        };
        if count == -(1 as libc::c_int) as size_t {
            read_diag_details(
                (*st).orig_file_name,
                (*st).stat.st_size - size_left,
                bufsize,
            );
            pad_archive(size_left);
            return dump_status_short;
        }
        size_left = (size_left as libc::c_ulong).wrapping_sub(count) as off_t as off_t;
        set_next_block_after(
            blk
                .offset(
                    bufsize
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
                ),
        );
        if count != bufsize {
            let mut buf: [libc::c_char; 21] = [0; 21];
            memset(
                ((*blk).buffer).as_mut_ptr().offset(count as isize) as *mut libc::c_void,
                0 as libc::c_int,
                bufsize.wrapping_sub(count),
            );
            if warning_option & 0x80 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcngettext(
                        0 as *const libc::c_char,
                        b"%s: File shrank by %s byte; padding with zeros\0" as *const u8
                            as *const libc::c_char,
                        b"%s: File shrank by %s bytes; padding with zeros\0" as *const u8
                            as *const libc::c_char,
                        size_left as libc::c_ulong,
                        5 as libc::c_int,
                    ),
                    quotearg_colon((*st).orig_file_name),
                    umaxtostr(size_left as uintmax_t, buf.as_mut_ptr()),
                );
            }
            if !ignore_failed_read_option {
                set_exit_status(1 as libc::c_int);
            }
            pad_archive(
                (size_left as libc::c_ulong).wrapping_sub(bufsize.wrapping_sub(count))
                    as off_t,
            );
            return dump_status_short;
        }
    }
    return dump_status_ok;
}
unsafe extern "C" fn dump_dir0(
    mut st: *mut tar_stat_info,
    mut directory: *const libc::c_char,
) {
    let mut top_level: bool = ((*st).parent).is_null();
    let mut tag_file_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut blk: *mut block = 0 as *mut block;
    let mut block_ordinal: off_t = current_block_ordinal();
    (*st).stat.st_size = 0 as libc::c_int as __off_t;
    blk = start_header(st);
    if blk.is_null() {
        return;
    }
    info_attach_exclist(st);
    if incremental_option as libc::c_int != 0
        && archive_format as libc::c_uint != POSIX_FORMAT as libc::c_int as libc::c_uint
    {
        (*blk).header.typeflag = 'D' as i32 as libc::c_char;
    } else {
        (*blk).header.typeflag = '5' as i32 as libc::c_char;
    }
    if !incremental_option {
        finish_header(st, blk, block_ordinal);
    } else if !((*gnu_list_name).directory).is_null() {
        if archive_format as libc::c_uint == POSIX_FORMAT as libc::c_int as libc::c_uint
        {
            xheader_store(
                b"GNU.dumpdir\0" as *const u8 as *const libc::c_char,
                st,
                safe_directory_contents((*gnu_list_name).directory)
                    as *const libc::c_void,
            );
            finish_header(st, blk, block_ordinal);
        } else {
            let mut size_left: off_t = 0;
            let mut totsize: off_t = 0;
            let mut bufsize: size_t = 0;
            let mut count: ssize_t = 0;
            let mut buffer: *const libc::c_char = 0 as *const libc::c_char;
            let mut p_buffer: *const libc::c_char = 0 as *const libc::c_char;
            block_ordinal = current_block_ordinal();
            buffer = safe_directory_contents((*gnu_list_name).directory);
            totsize = dumpdir_size(buffer) as off_t;
            off_to_chars(
                totsize,
                ((*blk).header.size).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            );
            finish_header(st, blk, block_ordinal);
            p_buffer = buffer;
            size_left = totsize;
            mv_begin_write((*st).file_name, totsize, totsize);
            while size_left > 0 as libc::c_int as libc::c_long {
                blk = find_next_block();
                bufsize = available_space_after(blk);
                if (size_left as libc::c_ulong) < bufsize {
                    bufsize = size_left as size_t;
                    count = bufsize.wrapping_rem(512 as libc::c_int as libc::c_ulong)
                        as ssize_t;
                    if count != 0 {
                        memset(
                            ((*blk).buffer).as_mut_ptr().offset(size_left as isize)
                                as *mut libc::c_void,
                            0 as libc::c_int,
                            (512 as libc::c_int as libc::c_long - count) as libc::c_ulong,
                        );
                    }
                }
                memcpy(
                    ((*blk).buffer).as_mut_ptr() as *mut libc::c_void,
                    p_buffer as *const libc::c_void,
                    bufsize,
                );
                size_left = (size_left as libc::c_ulong).wrapping_sub(bufsize) as off_t
                    as off_t;
                p_buffer = p_buffer.offset(bufsize as isize);
                set_next_block_after(
                    blk
                        .offset(
                            bufsize
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_div(512 as libc::c_int as libc::c_ulong) as isize,
                        ),
                );
            }
        }
        return;
    }
    if recursion_option == 0 {
        return;
    }
    if one_file_system_option as libc::c_int != 0 && !top_level
        && (*(*st).parent).stat.st_dev != (*st).stat.st_dev
    {
        if verbose_option != 0 {
            if warning_option & 0x40000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: file is on a different filesystem; not dumped\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_colon((*st).orig_file_name),
                );
            }
        }
    } else {
        let mut name_buf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name_size: size_t = 0;
        match check_exclusion_tags(st, &mut tag_file_name) as libc::c_uint {
            0 => {
                let mut entry: *const libc::c_char = 0 as *const libc::c_char;
                let mut entry_len: size_t = 0;
                let mut name_len: size_t = 0;
                name_buf = xstrdup((*st).orig_file_name);
                name_len = strlen(name_buf);
                name_size = name_len;
                entry = directory;
                loop {
                    entry_len = strlen(entry);
                    if !(entry_len != 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    if name_size < name_len.wrapping_add(entry_len) {
                        name_size = name_len.wrapping_add(entry_len);
                        name_buf = xrealloc(
                            name_buf as *mut libc::c_void,
                            name_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as *mut libc::c_char;
                    }
                    strcpy(name_buf.offset(name_len as isize), entry);
                    if !excluded_name(name_buf, st) {
                        dump_file(st, entry, name_buf);
                    }
                    entry = entry
                        .offset(
                            entry_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        );
                }
                rpl_free(name_buf as *mut libc::c_void);
            }
            1 => {
                exclusion_tag_warning(
                    (*st).orig_file_name,
                    tag_file_name,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"contents not dumped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                name_size = (strlen((*st).orig_file_name))
                    .wrapping_add(strlen(tag_file_name))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                name_buf = xmalloc(name_size) as *mut libc::c_char;
                strcpy(name_buf, (*st).orig_file_name);
                strcat(name_buf, tag_file_name);
                dump_file(st, tag_file_name, name_buf);
                rpl_free(name_buf as *mut libc::c_void);
            }
            2 => {
                exclusion_tag_warning(
                    (*st).orig_file_name,
                    tag_file_name,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"contents not dumped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            3 | _ => {}
        }
    };
}
unsafe extern "C" fn ensure_slash(mut pstr: *mut *mut libc::c_char) {
    let mut len: size_t = strlen(*pstr);
    while len >= 1 as libc::c_int as libc::c_ulong
        && *(*pstr).offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
        len;
    }
    if !(*(*pstr).offset(len as isize) as libc::c_int == '/' as i32) {
        *pstr = xrealloc(
            *pstr as *mut libc::c_void,
            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    let fresh3 = len;
    len = len.wrapping_add(1);
    *(*pstr).offset(fresh3 as isize) = '/' as i32 as libc::c_char;
    *(*pstr).offset(len as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn open_failure_recover(mut dir: *const tar_stat_info) -> bool {
    if *__errno_location() == 24 as libc::c_int && !dir.is_null()
        && !((*dir).parent).is_null()
    {
        let mut p: *mut tar_stat_info = 0 as *mut tar_stat_info;
        p = (*(*dir).parent).parent;
        while !p.is_null() {
            if (0 as libc::c_int) < (*p).fd
                && (((*p).parent).is_null() || (*(*p).parent).fd <= 0 as libc::c_int)
            {
                tar_stat_close(p);
                return 1 as libc::c_int != 0;
            }
            p = (*p).parent;
        }
        *__errno_location() = 24 as libc::c_int;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn get_directory_entries(
    mut st: *mut tar_stat_info,
) -> *mut libc::c_char {
    loop {
        (*st).dirstream = fdopendir((*st).fd);
        if !((*st).dirstream).is_null() {
            break;
        }
        if !open_failure_recover(st) {
            return 0 as *mut libc::c_char;
        }
    }
    return streamsavedir((*st).dirstream, savedir_sort_order as savedir_option);
}
unsafe extern "C" fn dump_dir(mut st: *mut tar_stat_info) -> bool {
    let mut directory: *mut libc::c_char = get_directory_entries(st);
    if directory.is_null() {
        savedir_diag((*st).orig_file_name);
        return 0 as libc::c_int != 0;
    }
    dump_dir0(st, directory);
    restore_parent_fd(st);
    rpl_free(directory as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
static mut trivial_link_count: nlink_t = 0;
pub unsafe extern "C" fn create_archive() {
    let mut p: *const name = 0 as *const name;
    trivial_link_count = (filename_args as libc::c_uint
        != FILES_MANY as libc::c_int as libc::c_uint && !dereference_option)
        as libc::c_int as nlink_t;
    open_archive(ACCESS_WRITE);
    buffer_write_global_xheader();
    if incremental_option {
        let mut buffer_size: size_t = 0 as libc::c_int as size_t;
        let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut q: *const libc::c_char = 0 as *const libc::c_char;
        collect_and_sort_names();
        loop {
            p = name_from_list();
            if p.is_null() {
                break;
            }
            if !excluded_name((*p).name, 0 as *mut tar_stat_info) {
                dump_file(0 as *mut tar_stat_info, (*p).name, (*p).name);
            }
        }
        blank_name_list();
        loop {
            p = name_from_list();
            if p.is_null() {
                break;
            }
            if !excluded_name((*p).name, 0 as *mut tar_stat_info) {
                let mut st: tar_stat_info = tar_stat_info {
                    orig_file_name: 0 as *mut libc::c_char,
                    file_name: 0 as *mut libc::c_char,
                    had_trailing_slash: false,
                    link_name: 0 as *mut libc::c_char,
                    uname: 0 as *mut libc::c_char,
                    gname: 0 as *mut libc::c_char,
                    cntx_name: 0 as *mut libc::c_char,
                    acls_a_ptr: 0 as *mut libc::c_char,
                    acls_a_len: 0,
                    acls_d_ptr: 0 as *mut libc::c_char,
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
                    sparse_map: 0 as *mut sp_array,
                    real_size: 0,
                    real_size_set: false,
                    sparse_name_done: false,
                    xattr_map_size: 0,
                    xattr_map: 0 as *mut xattr_array,
                    xhdr: xheader {
                        stk: 0 as *mut obstack,
                        size: 0,
                        buffer: 0 as *mut libc::c_char,
                        string_length: 0,
                    },
                    is_dumpdir: false,
                    skipped: false,
                    dumpdir: 0 as *mut libc::c_char,
                    parent: 0 as *mut tar_stat_info,
                    dirstream: 0 as *mut DIR,
                    fd: 0,
                    exclude_list: 0 as *mut exclist,
                };
                let mut plen: size_t = strlen((*p).name);
                while buffer_size <= plen {
                    buffer = x2realloc(buffer as *mut libc::c_void, &mut buffer_size)
                        as *mut libc::c_char;
                }
                memcpy(
                    buffer as *mut libc::c_void,
                    (*p).name as *const libc::c_void,
                    plen,
                );
                if !(*buffer
                    .offset(
                        plen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '/' as i32)
                {
                    let fresh4 = plen;
                    plen = plen.wrapping_add(1);
                    *buffer.offset(fresh4 as isize) = '/' as i32 as libc::c_char;
                }
                tar_stat_init(&mut st);
                q = directory_contents((*p).directory);
                if !q.is_null() {
                    while *q != 0 {
                        let mut qlen: size_t = strlen(q);
                        if *q as libc::c_int == 'Y' as i32 {
                            if (st.orig_file_name).is_null() {
                                let mut fd: libc::c_int = openat(
                                    chdir_fd,
                                    (*p).name,
                                    open_searchdir_flags,
                                );
                                if fd < 0 as libc::c_int {
                                    file_removed_diag(
                                        (*p).name,
                                        ((*p).parent).is_null(),
                                        Some(
                                            open_diag as unsafe extern "C" fn(*const libc::c_char) -> (),
                                        ),
                                    );
                                    break;
                                } else {
                                    st.fd = fd;
                                    if fstat(fd, &mut st.stat) != 0 as libc::c_int {
                                        file_removed_diag(
                                            (*p).name,
                                            ((*p).parent).is_null(),
                                            Some(
                                                stat_diag as unsafe extern "C" fn(*const libc::c_char) -> (),
                                            ),
                                        );
                                        break;
                                    } else {
                                        st.orig_file_name = xstrdup((*p).name);
                                    }
                                }
                            }
                            while buffer_size < plen.wrapping_add(qlen) {
                                buffer = x2realloc(
                                    buffer as *mut libc::c_void,
                                    &mut buffer_size,
                                ) as *mut libc::c_char;
                            }
                            strcpy(
                                buffer.offset(plen as isize),
                                q.offset(1 as libc::c_int as isize),
                            );
                            dump_file(
                                &mut st,
                                q.offset(1 as libc::c_int as isize),
                                buffer,
                            );
                        }
                        q = q
                            .offset(
                                qlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            );
                    }
                }
                tar_stat_destroy(&mut st);
            }
        }
        rpl_free(buffer as *mut libc::c_void);
    } else {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        loop {
            name = name_next(1 as libc::c_int);
            if name.is_null() {
                break;
            }
            if !excluded_name(name, 0 as *mut tar_stat_info) {
                dump_file(0 as *mut tar_stat_info, name, name);
            }
        }
    }
    write_eot();
    close_archive();
    finish_deferred_unlinks();
    if !listed_incremental_option.is_null() {
        write_directory_file();
    }
}
unsafe extern "C" fn hash_link(
    mut entry: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut l: *const link = entry as *const link;
    let mut num: uintmax_t = (*l).dev ^ (*l).ino;
    return num.wrapping_rem(n_buckets);
}
unsafe extern "C" fn compare_links(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut link1: *const link = entry1 as *const link;
    let mut link2: *const link = entry2 as *const link;
    return (*link1).dev ^ (*link2).dev | (*link1).ino ^ (*link2).ino
        == 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn unknown_file_error(mut p: *const libc::c_char) {
    if warning_option & 0x20 as libc::c_int != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unknown file type; file ignored\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_colon(p),
        );
    }
    if !ignore_failed_read_option {
        set_exit_status(2 as libc::c_int);
    }
}
static mut link_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn dump_hard_link(mut st: *mut tar_stat_info) -> bool {
    if !link_table.is_null()
        && (trivial_link_count < (*st).stat.st_nlink
            || remove_files_option as libc::c_int != 0)
    {
        let mut lp: link = link {
            dev: 0,
            ino: 0,
            nlink: 0,
            name: [0; 1],
        };
        let mut duplicate: *mut link = 0 as *mut link;
        let mut block_ordinal: off_t = 0;
        let mut blk: *mut block = 0 as *mut block;
        lp.ino = (*st).stat.st_ino;
        lp.dev = (*st).stat.st_dev;
        duplicate = hash_lookup(link_table, &mut lp as *mut link as *const libc::c_void)
            as *mut link;
        if !duplicate.is_null() {
            let mut link_name: *const libc::c_char = safer_name_suffix(
                ((*duplicate).name).as_mut_ptr(),
                1 as libc::c_int != 0,
                absolute_names_option,
            );
            if (*duplicate).nlink != 0 {
                (*duplicate).nlink = ((*duplicate).nlink).wrapping_sub(1);
                (*duplicate).nlink;
            }
            block_ordinal = current_block_ordinal();
            assign_string(&mut (*st).link_name, link_name);
            if ((100 as libc::c_int
                - (archive_format as libc::c_uint
                    == OLDGNU_FORMAT as libc::c_int as libc::c_uint) as libc::c_int)
                as libc::c_ulong) < strlen(link_name)
            {
                write_long_link(st);
            }
            (*st).stat.st_size = 0 as libc::c_int as __off_t;
            blk = start_header(st);
            if blk.is_null() {
                return 0 as libc::c_int != 0;
            }
            tar_copy_str(
                ((*blk).header.linkname).as_mut_ptr(),
                link_name,
                100 as libc::c_int as size_t,
            );
            (*blk).header.typeflag = '1' as i32 as libc::c_char;
            finish_header(st, blk, block_ordinal);
            if remove_files_option {
                queue_deferred_unlink((*st).orig_file_name, 0 as libc::c_int != 0);
            }
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn file_count_links(mut st: *mut tar_stat_info) {
    if hard_dereference_option {
        return;
    }
    if trivial_link_count < (*st).stat.st_nlink {
        let mut duplicate: *mut link = 0 as *mut link;
        let mut linkname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut lp: *mut link = 0 as *mut link;
        assign_string(
            &mut linkname,
            safer_name_suffix(
                (*st).orig_file_name,
                1 as libc::c_int != 0,
                absolute_names_option,
            ),
        );
        transform_name(&mut linkname, 0x2 as libc::c_int);
        lp = xmalloc(
            (24 as libc::c_ulong)
                .wrapping_add(strlen(linkname))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut link;
        (*lp).ino = (*st).stat.st_ino;
        (*lp).dev = (*st).stat.st_dev;
        (*lp).nlink = (*st).stat.st_nlink;
        strcpy(((*lp).name).as_mut_ptr(), linkname);
        rpl_free(linkname as *mut libc::c_void);
        if !((!link_table.is_null()
            || {
                link_table = hash_initialize(
                    0 as libc::c_int as size_t,
                    0 as *const Hash_tuning,
                    Some(
                        hash_link
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                size_t,
                            ) -> size_t,
                    ),
                    Some(
                        compare_links
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> bool,
                    ),
                    None,
                );
                !link_table.is_null()
            })
            && {
                duplicate = hash_insert(link_table, lp as *const libc::c_void)
                    as *mut link;
                !duplicate.is_null()
            })
        {
            xalloc_die();
        }
        if duplicate != lp {
            abort();
        }
        (*lp).nlink = ((*lp).nlink).wrapping_sub(1);
        (*lp).nlink;
    }
}
pub unsafe extern "C" fn check_links() {
    let mut lp: *mut link = 0 as *mut link;
    if link_table.is_null() {
        return;
    }
    lp = hash_get_first(link_table) as *mut link;
    while !lp.is_null() {
        if (*lp).nlink != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Missing links to %s.\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(((*lp).name).as_mut_ptr()),
            );
        }
        lp = hash_get_next(link_table, lp as *const libc::c_void) as *mut link;
    }
}
pub unsafe extern "C" fn subfile_open(
    mut dir: *const tar_stat_info,
    mut file: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    static mut initialized: bool = false;
    if !initialized {
        initialized = 1 as libc::c_int != 0;
        strerror(2 as libc::c_int);
        dcgettext(
            0 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    loop {
        fd = openat(if !dir.is_null() { (*dir).fd } else { chdir_fd }, file, flags);
        if !(fd < 0 as libc::c_int && open_failure_recover(dir) as libc::c_int != 0) {
            break;
        }
    }
    return fd;
}
pub unsafe extern "C" fn restore_parent_fd(mut st: *const tar_stat_info) {
    let mut parent: *mut tar_stat_info = (*st).parent;
    if !parent.is_null() && (*parent).fd == 0 {
        let mut parentfd: libc::c_int = openat(
            (*st).fd,
            b"..\0" as *const u8 as *const libc::c_char,
            open_searchdir_flags,
        );
        let mut parentstat: stat = stat {
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
        if parentfd < 0 as libc::c_int {
            parentfd = -*__errno_location();
        } else if !(fstat(parentfd, &mut parentstat) == 0 as libc::c_int
            && (*parent).stat.st_ino == parentstat.st_ino
            && (*parent).stat.st_dev == parentstat.st_dev)
        {
            close(parentfd);
            parentfd = IMPOSTOR_ERRNO as libc::c_int;
        }
        if parentfd < 0 as libc::c_int {
            let mut origfd: libc::c_int = openat(
                chdir_fd,
                (*parent).orig_file_name,
                open_searchdir_flags,
            );
            if 0 as libc::c_int <= origfd {
                if fstat(parentfd, &mut parentstat) == 0 as libc::c_int
                    && (*parent).stat.st_ino == parentstat.st_ino
                    && (*parent).stat.st_dev == parentstat.st_dev
                {
                    parentfd = origfd;
                } else {
                    close(origfd);
                }
            }
        }
        (*parent).fd = parentfd;
    }
}
unsafe extern "C" fn dump_file0(
    mut st: *mut tar_stat_info,
    mut name: *const libc::c_char,
    mut p: *const libc::c_char,
) {
    let mut header: *mut block = 0 as *mut block;
    let mut type_0: libc::c_char = 0;
    let mut original_size: off_t = 0;
    let mut original_ctime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut block_ordinal: off_t = -(1 as libc::c_int) as off_t;
    let mut fd: libc::c_int = 0 as libc::c_int;
    let mut is_dir: bool = false;
    let mut parent: *const tar_stat_info = (*st).parent;
    let mut top_level: bool = parent.is_null();
    let mut parentfd: libc::c_int = if top_level as libc::c_int != 0 {
        chdir_fd
    } else {
        (*parent).fd
    };
    let mut diag: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()> = None;
    if interactive_option as libc::c_int != 0
        && confirm(b"add\0" as *const u8 as *const libc::c_char, p) == 0
    {
        return;
    }
    assign_string(&mut (*st).orig_file_name, p);
    assign_string(
        &mut (*st).file_name,
        safer_name_suffix(p, 0 as libc::c_int != 0, absolute_names_option),
    );
    transform_name(&mut (*st).file_name, 0x1 as libc::c_int);
    if parentfd < 0 as libc::c_int && !top_level {
        *__errno_location() = -parentfd;
        diag = Some(open_diag as unsafe extern "C" fn(*const libc::c_char) -> ());
    } else if fstatat(parentfd, name, &mut (*st).stat, fstatat_flags) != 0 as libc::c_int
    {
        diag = Some(stat_diag as unsafe extern "C" fn(*const libc::c_char) -> ());
    } else if file_dumpable_p(&mut (*st).stat) {
        fd = subfile_open(parent, name, open_read_flags);
        if fd < 0 as libc::c_int {
            diag = Some(open_diag as unsafe extern "C" fn(*const libc::c_char) -> ());
        } else {
            (*st).fd = fd;
            if fstat(fd, &mut (*st).stat) != 0 as libc::c_int {
                diag = Some(
                    stat_diag as unsafe extern "C" fn(*const libc::c_char) -> (),
                );
            }
        }
    }
    if diag.is_some() {
        file_removed_diag(p, top_level, diag);
        return;
    }
    original_size = (*st).stat.st_size;
    (*st).archive_file_size = original_size;
    (*st).atime = get_stat_atime(&mut (*st).stat);
    (*st).mtime = get_stat_mtime(&mut (*st).stat);
    original_ctime = get_stat_ctime(&mut (*st).stat);
    (*st).ctime = original_ctime;
    if !(incremental_option as libc::c_int != 0 && !top_level)
        && !((*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
        && timespec_cmp((*st).mtime, newer_mtime_option) < 0 as libc::c_int
        && (after_date_option == 0
            || timespec_cmp((*st).ctime, newer_mtime_option) < 0 as libc::c_int)
    {
        if !incremental_option && verbose_option != 0 {
            if warning_option & 0x100 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: file is unchanged; not dumped\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_colon(p),
                );
            }
        }
        return;
    }
    if sys_file_is_archive(st) {
        if warning_option & 0x400 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: file is the archive; not dumped\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(p),
            );
        }
        return;
    }
    is_dir = ((*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int;
    if !is_dir && dump_hard_link(st) as libc::c_int != 0 {
        return;
    }
    if is_dir as libc::c_int != 0
        || (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint || 0 as libc::c_int != 0
    {
        let mut ok: bool = false;
        let mut final_stat: stat = stat {
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
        xattrs_acls_get(parentfd, name, st, 0 as libc::c_int, !is_dir as libc::c_int);
        xattrs_selinux_get(parentfd, name, st, fd);
        xattrs_xattrs_get(parentfd, name, st, fd);
        if is_dir {
            let mut tag_file_name: *const libc::c_char = 0 as *const libc::c_char;
            ensure_slash(&mut (*st).orig_file_name);
            ensure_slash(&mut (*st).file_name);
            if check_exclusion_tags(st, &mut tag_file_name) as libc::c_uint
                == exclusion_tag_all as libc::c_int as libc::c_uint
            {
                exclusion_tag_warning(
                    (*st).orig_file_name,
                    tag_file_name,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"directory not dumped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                return;
            }
            ok = dump_dir(st);
            fd = (*st).fd;
            parentfd = if top_level as libc::c_int != 0 {
                chdir_fd
            } else {
                (*parent).fd
            };
        } else {
            let mut status: dump_status = dump_status_ok;
            if fd != 0 && sparse_option as libc::c_int != 0
                && (*st).stat.st_blocks
                    < (*st).stat.st_size / 512 as libc::c_int as libc::c_long
                        + ((*st).stat.st_size % 512 as libc::c_int as libc::c_long
                            != 0 as libc::c_int as libc::c_long
                            && (*st).stat.st_size / 512 as libc::c_int as libc::c_long
                                != 0 as libc::c_int as libc::c_long) as libc::c_int
                            as libc::c_long
            {
                status = sparse_dump_file(fd, st);
                if status as libc::c_uint
                    == dump_status_not_implemented as libc::c_int as libc::c_uint
                {
                    status = dump_regular_file(fd, st);
                }
            } else {
                status = dump_regular_file(fd, st);
            }
            match status as libc::c_uint {
                0 | 1 => {
                    file_count_links(st);
                }
                3 => {
                    abort();
                }
                2 | _ => {}
            }
            ok = status as libc::c_uint == dump_status_ok as libc::c_int as libc::c_uint;
        }
        if ok {
            if fd < 0 as libc::c_int {
                *__errno_location() = -fd;
                ok = 0 as libc::c_int != 0;
            } else if fd == 0 as libc::c_int {
                if parentfd < 0 as libc::c_int && !top_level {
                    *__errno_location() = -parentfd;
                    ok = 0 as libc::c_int != 0;
                } else {
                    ok = fstatat(parentfd, name, &mut final_stat, fstatat_flags)
                        == 0 as libc::c_int;
                }
            } else {
                ok = fstat(fd, &mut final_stat) == 0 as libc::c_int;
            }
            if !ok {
                file_removed_diag(
                    p,
                    top_level,
                    Some(stat_diag as unsafe extern "C" fn(*const libc::c_char) -> ()),
                );
            }
        }
        if ok {
            if timespec_cmp(get_stat_ctime(&mut final_stat), original_ctime)
                != 0 as libc::c_int
                && !(remove_files_option as libc::c_int != 0
                    && is_dir as libc::c_int != 0) || original_size < final_stat.st_size
            {
                if warning_option & 0x10 as libc::c_int != 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: file changed as we read it\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_colon(p),
                    );
                }
                set_exit_status(1 as libc::c_int);
            } else if atime_preserve_option as libc::c_uint
                == replace_atime_preserve as libc::c_int as libc::c_uint && fd != 0
                && (is_dir as libc::c_int != 0
                    || original_size != 0 as libc::c_int as libc::c_long)
                && set_file_atime(fd, parentfd, name, (*st).atime) != 0 as libc::c_int
            {
                utime_error(p);
            }
        }
        ok = (ok as libc::c_int & tar_stat_close(st) as libc::c_int) != 0;
        if ok as libc::c_int != 0 && remove_files_option as libc::c_int != 0 {
            queue_deferred_unlink(p, is_dir);
        }
        return;
    } else if (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        (*st)
            .link_name = areadlinkat_with_size(
            parentfd,
            name,
            (*st).stat.st_size as size_t,
        );
        if ((*st).link_name).is_null() {
            if *__errno_location() == 12 as libc::c_int {
                xalloc_die();
            }
            file_removed_diag(
                p,
                top_level,
                Some(readlink_diag as unsafe extern "C" fn(*const libc::c_char) -> ()),
            );
            return;
        }
        transform_name(&mut (*st).link_name, 0x4 as libc::c_int);
        if ((100 as libc::c_int
            - (archive_format as libc::c_uint
                == OLDGNU_FORMAT as libc::c_int as libc::c_uint) as libc::c_int)
            as libc::c_ulong) < strlen((*st).link_name)
        {
            write_long_link(st);
        }
        xattrs_selinux_get(parentfd, name, st, 0 as libc::c_int);
        xattrs_xattrs_get(parentfd, name, st, 0 as libc::c_int);
        block_ordinal = current_block_ordinal();
        (*st).stat.st_size = 0 as libc::c_int as __off_t;
        header = start_header(st);
        if header.is_null() {
            return;
        }
        tar_copy_str(
            ((*header).header.linkname).as_mut_ptr(),
            (*st).link_name,
            100 as libc::c_int as size_t,
        );
        (*header).header.typeflag = '2' as i32 as libc::c_char;
        finish_header(st, header, block_ordinal);
        if remove_files_option {
            queue_deferred_unlink(p, 0 as libc::c_int != 0);
        }
        file_count_links(st);
        return;
    } else if (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        type_0 = '3' as i32 as libc::c_char;
        xattrs_acls_get(parentfd, name, st, 0 as libc::c_int, 1 as libc::c_int);
        xattrs_selinux_get(parentfd, name, st, 0 as libc::c_int);
        xattrs_xattrs_get(parentfd, name, st, 0 as libc::c_int);
    } else if (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        type_0 = '4' as i32 as libc::c_char;
        xattrs_acls_get(parentfd, name, st, 0 as libc::c_int, 1 as libc::c_int);
        xattrs_selinux_get(parentfd, name, st, 0 as libc::c_int);
        xattrs_xattrs_get(parentfd, name, st, 0 as libc::c_int);
    } else if (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        type_0 = '6' as i32 as libc::c_char;
        xattrs_acls_get(parentfd, name, st, 0 as libc::c_int, 1 as libc::c_int);
        xattrs_selinux_get(parentfd, name, st, 0 as libc::c_int);
        xattrs_xattrs_get(parentfd, name, st, 0 as libc::c_int);
    } else if (*st).stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint
    {
        if warning_option & 0x20 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: socket ignored\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(p),
            );
        }
        return;
    } else {
        unknown_file_error(p);
        return;
    }
    if archive_format as libc::c_uint == V7_FORMAT as libc::c_int as libc::c_uint {
        unknown_file_error(p);
        return;
    }
    block_ordinal = current_block_ordinal();
    (*st).stat.st_size = 0 as libc::c_int as __off_t;
    header = start_header(st);
    if header.is_null() {
        return;
    }
    (*header).header.typeflag = type_0;
    if type_0 as libc::c_int != '6' as i32 {
        major_to_chars(
            gnu_dev_major((*st).stat.st_rdev) as libc::c_int,
            ((*header).header.devmajor).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
        minor_to_chars(
            gnu_dev_minor((*st).stat.st_rdev) as libc::c_int,
            ((*header).header.devminor).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
    }
    finish_header(st, header, block_ordinal);
    if remove_files_option {
        queue_deferred_unlink(p, 0 as libc::c_int != 0);
    }
}
pub unsafe extern "C" fn dump_file(
    mut parent: *mut tar_stat_info,
    mut name: *const libc::c_char,
    mut fullname: *const libc::c_char,
) {
    let mut st: tar_stat_info = tar_stat_info {
        orig_file_name: 0 as *mut libc::c_char,
        file_name: 0 as *mut libc::c_char,
        had_trailing_slash: false,
        link_name: 0 as *mut libc::c_char,
        uname: 0 as *mut libc::c_char,
        gname: 0 as *mut libc::c_char,
        cntx_name: 0 as *mut libc::c_char,
        acls_a_ptr: 0 as *mut libc::c_char,
        acls_a_len: 0,
        acls_d_ptr: 0 as *mut libc::c_char,
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
        sparse_map: 0 as *mut sp_array,
        real_size: 0,
        real_size_set: false,
        sparse_name_done: false,
        xattr_map_size: 0,
        xattr_map: 0 as *mut xattr_array,
        xhdr: xheader {
            stk: 0 as *mut obstack,
            size: 0,
            buffer: 0 as *mut libc::c_char,
            string_length: 0,
        },
        is_dumpdir: false,
        skipped: false,
        dumpdir: 0 as *mut libc::c_char,
        parent: 0 as *mut tar_stat_info,
        dirstream: 0 as *mut DIR,
        fd: 0,
        exclude_list: 0 as *mut exclist,
    };
    tar_stat_init(&mut st);
    st.parent = parent;
    dump_file0(&mut st, name, fullname);
    if !parent.is_null() && !listed_incremental_option.is_null() {
        update_parent_directory(parent);
    }
    tar_stat_destroy(&mut st);
}
