use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type hash_table;
    pub type exclist;
    pub type namebuf;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __strtoul_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_ulong;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn offtostr(_: off_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn close_error(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn read_error(_: *const libc::c_char);
    fn read_fatal(_: *const libc::c_char) -> !;
    fn savedir_error(_: *const libc::c_char);
    fn seek_error(_: *const libc::c_char);
    fn truncate_error(_: *const libc::c_char);
    fn write_error(_: *const libc::c_char);
    fn fatal_exit() -> !;
    fn safer_name_suffix(
        file_name: *const libc::c_char,
        link_target: bool,
        absolute_names: bool,
    ) -> *mut libc::c_char;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_do_for_each(
        table: *const Hash_table,
        processor: Hash_processor,
        processor_data: *mut libc::c_void,
    ) -> size_t;
    fn hash_string(string: *const libc::c_char, n_buckets: size_t) -> size_t;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_clear(table: *mut Hash_table);
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    static mut absolute_names_option: bool;
    static mut after_date_option: libc::c_int;
    static mut interactive_option: bool;
    static mut listed_incremental_option: *const libc::c_char;
    static mut incremental_level: libc::c_int;
    static mut check_device_option: bool;
    static mut newer_mtime_option: timespec;
    static mut recursion_option: libc::c_int;
    static mut one_file_system_option: bool;
    static mut verbose_option: libc::c_int;
    static mut start_time: timespec;
    static mut current_stat_info: tar_stat_info;
    static mut open_read_flags: libc::c_int;
    static mut fstatat_flags: libc::c_int;
    static mut root_device: dev_t;
    static mut stdlis: *mut FILE;
    fn available_space_after(pointer: *mut block) -> size_t;
    fn find_next_block() -> *mut block;
    fn set_next_block_after(block: *mut block);
    fn mv_begin_read(st: *mut tar_stat_info);
    fn mv_end();
    fn mv_size_left(size: off_t);
    fn get_directory_entries(st: *mut tar_stat_info) -> *mut libc::c_char;
    fn subfile_open(
        dir: *const tar_stat_info,
        file: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn restore_parent_fd(st: *const tar_stat_info);
    fn exclusion_tag_warning(
        dirname: *const libc::c_char,
        tagname: *const libc::c_char,
        message: *const libc::c_char,
    );
    fn check_exclusion_tags(
        st: *const tar_stat_info,
        tag_file_name: *mut *const libc::c_char,
    ) -> exclusion_tag_type;
    fn rename_directory(src: *mut libc::c_char, dst: *mut libc::c_char) -> bool;
    static mut current_header: *mut block;
    fn skip_member();
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    fn unquote_string(str: *mut libc::c_char) -> libc::c_int;
    fn zap_slashes(name: *mut libc::c_char) -> *mut libc::c_char;
    fn normalize_filename(
        cdidx: libc::c_int,
        name: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn replace_prefix(
        pname: *mut *mut libc::c_char,
        samp: *const libc::c_char,
        slen: size_t,
        repl: *const libc::c_char,
        rlen: size_t,
    );
    fn tar_savedir(
        name: *const libc::c_char,
        must_exist: libc::c_int,
    ) -> *mut libc::c_char;
    fn namebuf_create(dir: *const libc::c_char) -> namebuf_t;
    fn namebuf_free(buf: namebuf_t);
    fn namebuf_name(buf: namebuf_t, name: *const libc::c_char) -> *mut libc::c_char;
    fn sysinttostr(
        _: uintmax_t,
        _: intmax_t,
        _: uintmax_t,
        buf: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strtosysint(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: intmax_t,
        _: uintmax_t,
    ) -> intmax_t;
    fn decode_timespec(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: bool,
    ) -> timespec;
    fn remove_any_file(
        file_name: *const libc::c_char,
        option: remove_option,
    ) -> libc::c_int;
    fn deref_stat(name: *const libc::c_char, buf: *mut stat) -> libc::c_int;
    static mut chdir_current: libc::c_int;
    fn open_diag(name: *const libc::c_char);
    fn stat_diag(name: *const libc::c_char);
    fn file_removed_diag(
        name: *const libc::c_char,
        top_level: bool,
        diagfn: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()>,
    );
    fn name_from_list() -> *const name;
    fn blank_name_list();
    fn make_file_name(
        dir_name_0: *const libc::c_char,
        name: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn confirm(
        message_action: *const libc::c_char,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn tar_stat_init(st: *mut tar_stat_info);
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn sys_truncate(fd: libc::c_int) -> libc::c_int;
    fn info_attach_exclist(dir: *mut tar_stat_info);
    fn excluded_name(name: *const libc::c_char, st: *mut tar_stat_info) -> bool;
    static mut program_name: *const libc::c_char;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    static mut warning_option: libc::c_int;
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
}
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
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
pub type Hash_processor = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
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
pub type exclusion_tag_type = libc::c_uint;
pub const exclusion_tag_all: exclusion_tag_type = 3;
pub const exclusion_tag_under: exclusion_tag_type = 2;
pub const exclusion_tag_contents: exclusion_tag_type = 1;
pub const exclusion_tag_none: exclusion_tag_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub next: *mut directory,
    pub mtime: timespec,
    pub device_number: dev_t,
    pub inode_number: ino_t,
    pub dump: *mut dumpdir,
    pub idump: *mut dumpdir,
    pub children: children,
    pub flags: libc::c_uint,
    pub orig: *mut directory,
    pub tagfile: *const libc::c_char,
    pub caname: *mut libc::c_char,
    pub name: *mut libc::c_char,
}
pub type children = libc::c_uint;
pub const ALL_CHILDREN: children = 2;
pub const CHANGED_CHILDREN: children = 1;
pub const NO_CHILDREN: children = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dumpdir {
    pub contents: *mut libc::c_char,
    pub total: size_t,
    pub elc: size_t,
    pub elv: *mut *mut libc::c_char,
}
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
pub type namebuf_t = *mut namebuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dumpdir_iter {
    pub dump: *mut dumpdir,
    pub all: libc::c_int,
    pub next: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_range {
    pub fieldname: *const libc::c_char,
    pub min_val: intmax_t,
    pub max_val: uintmax_t,
}
pub const BILLION: C2RustUnnamed_2 = 1000000000;
pub type remove_option = libc::c_uint;
pub const WANT_DIRECTORY_REMOVE_OPTION: remove_option = 2;
pub const RECURSIVE_REMOVE_OPTION: remove_option = 1;
pub const ORDINARY_REMOVE_OPTION: remove_option = 0;
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
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *mut libc::c_void;
        __comparison = (Some(__compar.unwrap())).unwrap()(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int
        as libc::c_long != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn fputc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: libc::c_int,
    mut __stream: *mut FILE,
) -> libc::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(__stream, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh2 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh2 = __c as libc::c_char;
        *fresh2 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn valid_timespec(mut t: timespec) -> bool {
    return 0 as libc::c_int as libc::c_long <= t.tv_nsec;
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
unsafe extern "C" fn dumpdir_create0(
    mut contents: *const libc::c_char,
    mut cmask: *const libc::c_char,
) -> *mut dumpdir {
    let mut dump: *mut dumpdir = 0 as *mut dumpdir;
    let mut i: size_t = 0;
    let mut total: size_t = 0;
    let mut ctsize: size_t = 0;
    let mut len: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    i = 0 as libc::c_int as size_t;
    total = 0 as libc::c_int as size_t;
    ctsize = 1 as libc::c_int as size_t;
    q = contents;
    while *q != 0 {
        len = (strlen(q)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        ctsize = (ctsize as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        if cmask.is_null() || !(strchr(cmask, *q as libc::c_int)).is_null() {
            i = i.wrapping_add(1);
            i;
        }
        total = total.wrapping_add(1);
        total;
        q = q.offset(len as isize);
    }
    dump = xmalloc(
        (::std::mem::size_of::<dumpdir>() as libc::c_ulong).wrapping_add(ctsize),
    ) as *mut dumpdir;
    (*dump).contents = dump.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    memcpy(
        (*dump).contents as *mut libc::c_void,
        contents as *const libc::c_void,
        ctsize,
    );
    (*dump).total = total;
    (*dump).elc = i;
    (*dump)
        .elv = xcalloc(
        i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int as size_t;
    p = (*dump).contents;
    while *p != 0 {
        if cmask.is_null() || !(strchr(cmask, *p as libc::c_int)).is_null() {
            let fresh3 = i;
            i = i.wrapping_add(1);
            let ref mut fresh4 = *((*dump).elv).offset(fresh3 as isize);
            *fresh4 = p.offset(1 as libc::c_int as isize);
        }
        p = p
            .offset(
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    let ref mut fresh5 = *((*dump).elv).offset(i as isize);
    *fresh5 = 0 as *mut libc::c_char;
    return dump;
}
unsafe extern "C" fn dumpdir_create(mut contents: *const libc::c_char) -> *mut dumpdir {
    return dumpdir_create0(contents, b"YND\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn dumpdir_free(mut dump: *mut dumpdir) {
    rpl_free((*dump).elv as *mut libc::c_void);
    rpl_free(dump as *mut libc::c_void);
}
unsafe extern "C" fn compare_dirnames(
    mut first: *const libc::c_void,
    mut second: *const libc::c_void,
) -> libc::c_int {
    let mut name1: *const *const libc::c_char = first as *const *const libc::c_char;
    let mut name2: *const *const libc::c_char = second as *const *const libc::c_char;
    return strcmp(*name1, *name2);
}
unsafe extern "C" fn dumpdir_locate(
    mut dump: *mut dumpdir,
    mut name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if dump.is_null() {
        return 0 as *mut libc::c_char;
    }
    ptr = bsearch(
        &mut name as *mut *const libc::c_char as *const libc::c_void,
        (*dump).elv as *const libc::c_void,
        (*dump).elc,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            compare_dirnames
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut *mut libc::c_char;
    return if !ptr.is_null() {
        (*ptr).offset(-(1 as libc::c_int as isize))
    } else {
        0 as *mut libc::c_char
    };
}
unsafe extern "C" fn dumpdir_next(mut itr: *mut dumpdir_iter) -> *mut libc::c_char {
    let mut cur: size_t = (*itr).next;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*itr).all != 0 {
        ret = ((*(*itr).dump).contents).offset(cur as isize);
        if *ret as libc::c_int == 0 as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        (*itr)
            .next = ((*itr).next as libc::c_ulong)
            .wrapping_add((strlen(ret)).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    } else if cur < (*(*itr).dump).elc {
        ret = (*((*(*itr).dump).elv).offset(cur as isize))
            .offset(-(1 as libc::c_int as isize));
        (*itr).next = ((*itr).next).wrapping_add(1);
        (*itr).next;
    }
    return ret;
}
unsafe extern "C" fn dumpdir_first(
    mut dump: *mut dumpdir,
    mut all: libc::c_int,
    mut pitr: *mut *mut dumpdir_iter,
) -> *mut libc::c_char {
    let mut itr: *mut dumpdir_iter = xmalloc(
        ::std::mem::size_of::<dumpdir_iter>() as libc::c_ulong,
    ) as *mut dumpdir_iter;
    (*itr).dump = dump;
    (*itr).all = all;
    (*itr).next = 0 as libc::c_int as size_t;
    *pitr = itr;
    return dumpdir_next(itr);
}
pub unsafe extern "C" fn dumpdir_size(mut p: *const libc::c_char) -> size_t {
    let mut totsize: size_t = 0 as libc::c_int as size_t;
    while *p != 0 {
        let mut size: size_t = (strlen(p))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        totsize = (totsize as libc::c_ulong).wrapping_add(size) as size_t as size_t;
        p = p.offset(size as isize);
    }
    return totsize.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
static mut dirhead: *mut directory = 0 as *const directory as *mut directory;
static mut dirtail: *mut directory = 0 as *const directory as *mut directory;
static mut directory_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
static mut directory_meta_table: *mut Hash_table = 0 as *const Hash_table
    as *mut Hash_table;
unsafe extern "C" fn hash_directory_canonical_name(
    mut entry: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut directory: *const directory = entry as *const directory;
    return hash_string((*directory).caname, n_buckets);
}
unsafe extern "C" fn compare_directory_canonical_names(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut directory1: *const directory = entry1 as *const directory;
    let mut directory2: *const directory = entry2 as *const directory;
    return strcmp((*directory1).caname, (*directory2).caname) == 0 as libc::c_int;
}
unsafe extern "C" fn hash_directory_meta(
    mut entry: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut directory: *const directory = entry as *const directory;
    return ((*directory).device_number)
        .wrapping_add((*directory).inode_number)
        .wrapping_rem(n_buckets);
}
unsafe extern "C" fn compare_directory_meta(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut directory1: *const directory = entry1 as *const directory;
    let mut directory2: *const directory = entry2 as *const directory;
    return (*directory1).device_number == (*directory2).device_number
        && (*directory1).inode_number == (*directory2).inode_number;
}
unsafe extern "C" fn make_directory(
    mut name: *const libc::c_char,
    mut caname: *mut libc::c_char,
) -> *mut directory {
    let mut namelen: size_t = strlen(name);
    let mut directory: *mut directory = xmalloc(
        ::std::mem::size_of::<directory>() as libc::c_ulong,
    ) as *mut directory;
    (*directory).next = 0 as *mut directory;
    (*directory).idump = 0 as *mut dumpdir;
    (*directory).dump = (*directory).idump;
    (*directory).orig = 0 as *mut directory;
    (*directory).flags = 0 as libc::c_int as libc::c_uint;
    if namelen > 1 as libc::c_int as libc::c_ulong
        && *name.offset(namelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32
    {
        namelen = namelen.wrapping_sub(1);
        namelen;
    }
    (*directory)
        .name = xmalloc(namelen.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    memcpy((*directory).name as *mut libc::c_void, name as *const libc::c_void, namelen);
    *((*directory).name).offset(namelen as isize) = 0 as libc::c_int as libc::c_char;
    (*directory).caname = caname;
    (*directory).tagfile = 0 as *const libc::c_char;
    return directory;
}
unsafe extern "C" fn free_directory(mut dir: *mut directory) {
    rpl_free((*dir).caname as *mut libc::c_void);
    rpl_free((*dir).name as *mut libc::c_void);
    rpl_free(dir as *mut libc::c_void);
}
unsafe extern "C" fn attach_directory(mut name: *const libc::c_char) -> *mut directory {
    let mut cname: *mut libc::c_char = normalize_filename(chdir_current, name);
    let mut dir: *mut directory = make_directory(name, cname);
    if !dirtail.is_null() {
        (*dirtail).next = dir;
    } else {
        dirhead = dir;
    }
    dirtail = dir;
    return dir;
}
unsafe extern "C" fn dirlist_replace_prefix(
    mut pref: *const libc::c_char,
    mut repl: *const libc::c_char,
) {
    let mut dp: *mut directory = 0 as *mut directory;
    let mut pref_len: size_t = strlen(pref);
    let mut repl_len: size_t = strlen(repl);
    dp = dirhead;
    while !dp.is_null() {
        replace_prefix(&mut (*dp).name, pref, pref_len, repl, repl_len);
        dp = (*dp).next;
    }
}
pub unsafe extern "C" fn clear_directory_table() {
    let mut dp: *mut directory = 0 as *mut directory;
    if !directory_table.is_null() {
        hash_clear(directory_table);
    }
    if !directory_meta_table.is_null() {
        hash_clear(directory_meta_table);
    }
    dp = dirhead;
    while !dp.is_null() {
        let mut next: *mut directory = (*dp).next;
        free_directory(dp);
        dp = next;
    }
    dirtail = 0 as *mut directory;
    dirhead = dirtail;
}
unsafe extern "C" fn note_directory(
    mut name: *const libc::c_char,
    mut mtime: timespec,
    mut dev: dev_t,
    mut ino: ino_t,
    mut nfs: bool,
    mut found: bool,
    mut contents: *const libc::c_char,
) -> *mut directory {
    let mut directory: *mut directory = attach_directory(name);
    (*directory).mtime = mtime;
    (*directory).device_number = dev;
    (*directory).inode_number = ino;
    (*directory).children = CHANGED_CHILDREN;
    if nfs {
        (*directory).flags |= 0x2 as libc::c_int as libc::c_uint;
    }
    if found {
        (*directory).flags |= 0x4 as libc::c_int as libc::c_uint;
    }
    if !contents.is_null() {
        (*directory).dump = dumpdir_create(contents);
    } else {
        (*directory).dump = 0 as *mut dumpdir;
    }
    if !((!directory_table.is_null()
        || {
            directory_table = hash_initialize(
                0 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    hash_directory_canonical_name
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    compare_directory_canonical_names
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                None,
            );
            !directory_table.is_null()
        })
        && !(hash_insert(directory_table, directory as *const libc::c_void)).is_null())
    {
        xalloc_die();
    }
    if !((!directory_meta_table.is_null()
        || {
            directory_meta_table = hash_initialize(
                0 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    hash_directory_meta
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    compare_directory_meta
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                None,
            );
            !directory_meta_table.is_null()
        })
        && !(hash_insert(directory_meta_table, directory as *const libc::c_void))
            .is_null())
    {
        xalloc_die();
    }
    return directory;
}
unsafe extern "C" fn find_directory(mut name: *const libc::c_char) -> *mut directory {
    if directory_table.is_null() {
        return 0 as *mut directory
    } else {
        let mut caname: *mut libc::c_char = normalize_filename(chdir_current, name);
        let mut dir: *mut directory = make_directory(name, caname);
        let mut ret: *mut directory = hash_lookup(
            directory_table,
            dir as *const libc::c_void,
        ) as *mut directory;
        free_directory(dir);
        return ret;
    };
}
pub unsafe extern "C" fn rebase_directory(
    mut dir: *mut directory,
    mut old_prefix: *const libc::c_char,
    mut old_prefix_len: size_t,
    mut new_prefix: *const libc::c_char,
    mut new_prefix_len: size_t,
) {
    replace_prefix(
        &mut (*dir).name,
        old_prefix,
        old_prefix_len,
        new_prefix,
        new_prefix_len,
    );
}
unsafe extern "C" fn find_directory_meta(
    mut dev: dev_t,
    mut ino: ino_t,
) -> *mut directory {
    if directory_meta_table.is_null() {
        return 0 as *mut directory
    } else {
        let mut dir: *mut directory = make_directory(
            b"\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_char,
        );
        let mut ret: *mut directory = 0 as *mut directory;
        (*dir).device_number = dev;
        (*dir).inode_number = ino;
        ret = hash_lookup(directory_meta_table, dir as *const libc::c_void)
            as *mut directory;
        free_directory(dir);
        return ret;
    };
}
pub unsafe extern "C" fn update_parent_directory(mut parent: *mut tar_stat_info) {
    let mut directory: *mut directory = find_directory((*parent).orig_file_name);
    if !directory.is_null() {
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
        if fstat((*parent).fd, &mut st) != 0 as libc::c_int {
            stat_diag((*directory).name);
        } else {
            (*directory).mtime = get_stat_mtime(&mut st);
        }
    }
}
unsafe extern "C" fn procdir(
    mut name_buffer: *const libc::c_char,
    mut st: *mut tar_stat_info,
    mut flag: libc::c_int,
    mut entry: *mut libc::c_char,
) -> *mut directory {
    let mut directory: *mut directory = 0 as *mut directory;
    let mut stat_data: *mut stat = &mut (*st).stat;
    let mut nfs: bool = (*stat_data).st_dev
        & !(0 as libc::c_int as dev_t)
            << (::std::mem::size_of::<__dev_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        != 0 as libc::c_int as libc::c_ulong;
    let mut perhaps_renamed: bool = 0 as libc::c_int != 0;
    directory = find_directory(name_buffer);
    if !directory.is_null() {
        if (*directory).flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            if flag & 0x20 as libc::c_int != 0 {
                assign_string(&mut (*directory).name, name_buffer);
            } else {
                *entry = 'N' as i32 as libc::c_char;
                return directory;
            }
        }
        if strcmp((*directory).name, name_buffer) != 0 {
            *entry = 'N' as i32 as libc::c_char;
            return directory;
        }
        if !((!check_device_option
            || (*directory).flags & 0x2 as libc::c_int as libc::c_uint != 0
                && nfs as libc::c_int != 0
            || (*directory).device_number == (*stat_data).st_dev)
            && (*directory).inode_number == (*stat_data).st_ino)
        {
            let mut d: *mut directory = find_directory_meta(
                (*stat_data).st_dev,
                (*stat_data).st_ino,
            );
            if !d.is_null() {
                if strcmp((*d).name, name_buffer) != 0 {
                    if warning_option & 0x2000 as libc::c_int != 0 {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: Directory has been renamed from %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_colon(name_buffer),
                            quote_n(1 as libc::c_int, (*d).name),
                        );
                    }
                    (*directory).orig = d;
                    (*directory).flags |= 0x10 as libc::c_int as libc::c_uint;
                    (*d).flags &= !(0x10 as libc::c_int) as libc::c_uint;
                    dirlist_replace_prefix((*d).name, name_buffer);
                }
                (*directory).children = CHANGED_CHILDREN;
            } else {
                perhaps_renamed = 1 as libc::c_int != 0;
                (*directory).children = ALL_CHILDREN;
                (*directory).device_number = (*stat_data).st_dev;
                (*directory).inode_number = (*stat_data).st_ino;
            }
            if nfs {
                (*directory).flags |= 0x2 as libc::c_int as libc::c_uint;
            }
        } else {
            (*directory).children = CHANGED_CHILDREN;
        }
        (*directory).flags |= 0x4 as libc::c_int as libc::c_uint;
    } else {
        let mut d_0: *mut directory = find_directory_meta(
            (*stat_data).st_dev,
            (*stat_data).st_ino,
        );
        directory = note_directory(
            name_buffer,
            get_stat_mtime(stat_data),
            (*stat_data).st_dev,
            (*stat_data).st_ino,
            nfs,
            1 as libc::c_int != 0,
            0 as *const libc::c_char,
        );
        if !d_0.is_null() {
            if strcmp((*d_0).name, name_buffer) != 0 {
                if warning_option & 0x2000 as libc::c_int != 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: Directory has been renamed from %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_colon(name_buffer),
                        quote_n(1 as libc::c_int, (*d_0).name),
                    );
                }
                (*directory).orig = d_0;
                (*directory).flags |= 0x10 as libc::c_int as libc::c_uint;
                (*d_0).flags &= !(0x10 as libc::c_int) as libc::c_uint;
                dirlist_replace_prefix((*d_0).name, name_buffer);
            }
            (*directory).children = CHANGED_CHILDREN;
        } else {
            (*directory).flags |= 0x8 as libc::c_int as libc::c_uint;
            if warning_option & 0x1000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Directory is new\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_colon(name_buffer),
                );
            }
            (*directory)
                .children = (if !listed_incremental_option.is_null()
                || (timespec_cmp(get_stat_mtime(stat_data), newer_mtime_option)
                    < 0 as libc::c_int
                    || after_date_option != 0
                        && timespec_cmp(get_stat_ctime(stat_data), newer_mtime_option)
                            < 0 as libc::c_int)
            {
                ALL_CHILDREN as libc::c_int
            } else {
                CHANGED_CHILDREN as libc::c_int
            }) as children;
        }
    }
    if one_file_system_option as libc::c_int != 0 && !((*st).parent).is_null()
        && (*stat_data).st_dev != (*(*st).parent).stat.st_dev
    {
        if warning_option & 0x40000 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: directory is on a different filesystem; not dumped\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon((*directory).name),
            );
        }
        (*directory).children = NO_CHILDREN;
        if !((*directory).dump).is_null() {
            dumpdir_free((*directory).dump);
            (*directory).dump = 0 as *mut dumpdir;
        }
        perhaps_renamed = 0 as libc::c_int != 0;
    } else if flag & 0x10 as libc::c_int != 0 {
        (*directory).children = (flag & 3 as libc::c_int) as children;
        if (*directory).children as libc::c_uint
            == NO_CHILDREN as libc::c_int as libc::c_uint
        {
            *entry = 'N' as i32 as libc::c_char;
        }
    }
    if perhaps_renamed {
        if warning_option & 0x2000 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Directory has been renamed\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(name_buffer),
            );
        }
    }
    (*directory).flags |= 0x1 as libc::c_int as libc::c_uint;
    if (*directory).children as libc::c_uint
        != NO_CHILDREN as libc::c_int as libc::c_uint
    {
        let mut tag_file_name: *const libc::c_char = 0 as *const libc::c_char;
        match check_exclusion_tags(st, &mut tag_file_name) as libc::c_uint {
            3 => {
                exclusion_tag_warning(
                    name_buffer,
                    tag_file_name,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"directory not dumped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                *entry = 'N' as i32 as libc::c_char;
                (*directory).children = NO_CHILDREN;
            }
            1 => {
                exclusion_tag_warning(
                    name_buffer,
                    tag_file_name,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"contents not dumped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                (*directory).children = NO_CHILDREN;
                (*directory).tagfile = tag_file_name;
            }
            2 => {
                exclusion_tag_warning(
                    name_buffer,
                    tag_file_name,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"contents not dumped\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                (*directory).tagfile = tag_file_name;
            }
            0 | _ => {}
        }
    }
    return directory;
}
unsafe extern "C" fn makedumpdir(
    mut directory: *mut directory,
    mut dir: *const libc::c_char,
) {
    let mut i: size_t = 0;
    let mut dirsize: size_t = 0;
    let mut len: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut array: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut new_dump: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_dump_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dump: *mut dumpdir = 0 as *mut dumpdir;
    if (*directory).children as libc::c_uint
        == ALL_CHILDREN as libc::c_int as libc::c_uint
    {
        dump = 0 as *mut dumpdir;
    } else if !((*directory).orig).is_null() {
        dump = if !((*(*directory).orig).idump).is_null() {
            (*(*directory).orig).idump
        } else {
            (*(*directory).orig).dump
        };
    } else {
        dump = (*directory).dump;
    }
    dirsize = 0 as libc::c_int as size_t;
    len = 0 as libc::c_int as size_t;
    p = dir;
    while *p != 0 {
        len = (len as libc::c_ulong)
            .wrapping_add((strlen(p)).wrapping_add(2 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        p = p
            .offset(
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        dirsize = dirsize.wrapping_add(1);
        dirsize;
    }
    len = len.wrapping_add(1);
    len;
    array = xcalloc(
        dirsize,
        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    ) as *mut *const libc::c_char;
    i = 0 as libc::c_int as size_t;
    p = dir;
    while *p != 0 {
        let ref mut fresh6 = *array.offset(i as isize);
        *fresh6 = p;
        p = p
            .offset(
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    qsort(
        array as *mut libc::c_void,
        dirsize,
        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        Some(
            compare_dirnames
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    new_dump = xmalloc(len) as *mut libc::c_char;
    new_dump_ptr = new_dump;
    i = 0 as libc::c_int as size_t;
    while i < dirsize {
        let mut loc: *const libc::c_char = dumpdir_locate(
            dump,
            *array.offset(i as isize),
        );
        if !loc.is_null() {
            if !((*directory).tagfile).is_null() {
                *new_dump_ptr = 'I' as i32 as libc::c_char;
            } else {
                *new_dump_ptr = ' ' as i32 as libc::c_char;
            }
            new_dump_ptr = new_dump_ptr.offset(1);
            new_dump_ptr;
        } else if !((*directory).tagfile).is_null() {
            let fresh7 = new_dump_ptr;
            new_dump_ptr = new_dump_ptr.offset(1);
            *fresh7 = 'I' as i32 as libc::c_char;
        } else {
            let fresh8 = new_dump_ptr;
            new_dump_ptr = new_dump_ptr.offset(1);
            *fresh8 = 'Y' as i32 as libc::c_char;
        }
        p = *array.offset(i as isize);
        loop {
            let fresh9 = p;
            p = p.offset(1);
            let fresh10 = new_dump_ptr;
            new_dump_ptr = new_dump_ptr.offset(1);
            *fresh10 = *fresh9;
            if !(*fresh10 != 0) {
                break;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    *new_dump_ptr = 0 as libc::c_int as libc::c_char;
    (*directory).idump = (*directory).dump;
    (*directory).dump = dumpdir_create0(new_dump, 0 as *const libc::c_char);
    rpl_free(new_dump as *mut libc::c_void);
    rpl_free(array as *mut libc::c_void);
}
unsafe extern "C" fn maketagdumpdir(mut directory: *mut directory) {
    let mut len: size_t = (strlen((*directory).tagfile))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut new_dump: *mut libc::c_char = xmalloc(
        len.wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *new_dump.offset(0 as libc::c_int as isize) = 'Y' as i32 as libc::c_char;
    memcpy(
        new_dump.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (*directory).tagfile as *const libc::c_void,
        len,
    );
    *new_dump
        .offset(
            len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    (*directory).idump = (*directory).dump;
    (*directory).dump = dumpdir_create0(new_dump, 0 as *const libc::c_char);
    rpl_free(new_dump as *mut libc::c_void);
}
pub unsafe extern "C" fn scan_directory(mut st: *mut tar_stat_info) -> *mut directory {
    let mut dir: *const libc::c_char = (*st).orig_file_name;
    let mut dirp: *mut libc::c_char = get_directory_entries(st);
    let mut device: dev_t = (*st).stat.st_dev;
    let mut cmdline: bool = ((*st).parent).is_null();
    let mut nbuf: namebuf_t = 0 as *mut namebuf;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut directory: *mut directory = 0 as *mut directory;
    let mut ch: libc::c_char = 0;
    if dirp.is_null() {
        savedir_error(dir);
    }
    info_attach_exclist(st);
    tmp = xstrdup(dir);
    zap_slashes(tmp);
    directory = procdir(
        tmp,
        st,
        if cmdline as libc::c_int != 0 { 0x20 as libc::c_int } else { 0 as libc::c_int },
        &mut ch,
    );
    rpl_free(tmp as *mut libc::c_void);
    nbuf = namebuf_create(dir);
    if !dirp.is_null() {
        if (*directory).children as libc::c_uint
            != NO_CHILDREN as libc::c_int as libc::c_uint
        {
            let mut entry: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut itr: *mut dumpdir_iter = 0 as *mut dumpdir_iter;
            makedumpdir(directory, dirp);
            entry = dumpdir_first((*directory).dump, 1 as libc::c_int, &mut itr);
            while !entry.is_null() {
                let mut full_name: *mut libc::c_char = namebuf_name(
                    nbuf,
                    entry.offset(1 as libc::c_int as isize),
                );
                if *entry as libc::c_int == 'I' as i32 {
                    *entry = 'N' as i32 as libc::c_char;
                } else if excluded_name(full_name, st) {
                    *entry = 'N' as i32 as libc::c_char;
                } else {
                    let mut fd: libc::c_int = (*st).fd;
                    let mut diag: Option::<
                        unsafe extern "C" fn(*const libc::c_char) -> (),
                    > = None;
                    let mut stsub: tar_stat_info = tar_stat_info {
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
                    tar_stat_init(&mut stsub);
                    if fd < 0 as libc::c_int {
                        *__errno_location() = -fd;
                        diag = Some(
                            open_diag as unsafe extern "C" fn(*const libc::c_char) -> (),
                        );
                    } else if fstatat(
                        fd,
                        entry.offset(1 as libc::c_int as isize),
                        &mut stsub.stat,
                        fstatat_flags,
                    ) != 0 as libc::c_int
                    {
                        diag = Some(
                            stat_diag as unsafe extern "C" fn(*const libc::c_char) -> (),
                        );
                    } else if stsub.stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    {
                        let mut subfd: libc::c_int = subfile_open(
                            st,
                            entry.offset(1 as libc::c_int as isize),
                            open_read_flags,
                        );
                        if subfd < 0 as libc::c_int {
                            diag = Some(
                                open_diag as unsafe extern "C" fn(*const libc::c_char) -> (),
                            );
                        } else {
                            stsub.fd = subfd;
                            if fstat(subfd, &mut stsub.stat) != 0 as libc::c_int {
                                diag = Some(
                                    stat_diag as unsafe extern "C" fn(*const libc::c_char) -> (),
                                );
                            }
                        }
                    }
                    if diag.is_some() {
                        file_removed_diag(full_name, 0 as libc::c_int != 0, diag);
                        *entry = 'N' as i32 as libc::c_char;
                    } else if stsub.stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    {
                        let mut pd_flag: libc::c_int = 0 as libc::c_int;
                        if recursion_option == 0 {
                            pd_flag |= 0x10 as libc::c_int | NO_CHILDREN as libc::c_int;
                        } else if (*directory).children as libc::c_uint
                            == ALL_CHILDREN as libc::c_int as libc::c_uint
                        {
                            pd_flag |= 0x10 as libc::c_int | ALL_CHILDREN as libc::c_int;
                        }
                        *entry = 'D' as i32 as libc::c_char;
                        stsub.parent = st;
                        procdir(full_name, &mut stsub, pd_flag, entry);
                        restore_parent_fd(&mut stsub);
                    } else if one_file_system_option as libc::c_int != 0
                        && device != stsub.stat.st_dev
                    {
                        *entry = 'N' as i32 as libc::c_char;
                    } else if !(*entry as libc::c_int == 'Y' as i32) {
                        if timespec_cmp(
                            get_stat_mtime(&mut stsub.stat),
                            newer_mtime_option,
                        ) < 0 as libc::c_int
                            && (after_date_option == 0
                                || timespec_cmp(
                                    get_stat_ctime(&mut stsub.stat),
                                    newer_mtime_option,
                                ) < 0 as libc::c_int)
                        {
                            *entry = 'N' as i32 as libc::c_char;
                        } else {
                            *entry = 'Y' as i32 as libc::c_char;
                        }
                    }
                    tar_stat_destroy(&mut stsub);
                }
                entry = dumpdir_next(itr);
            }
            rpl_free(itr as *mut libc::c_void);
        } else if !((*directory).tagfile).is_null() {
            maketagdumpdir(directory);
        }
    }
    namebuf_free(nbuf);
    rpl_free(dirp as *mut libc::c_void);
    return directory;
}
pub unsafe extern "C" fn directory_contents(
    mut dir: *mut directory,
) -> *const libc::c_char {
    if dir.is_null() {
        return 0 as *const libc::c_char;
    }
    return if !((*dir).dump).is_null() {
        (*(*dir).dump).contents
    } else {
        0 as *mut libc::c_char
    };
}
pub unsafe extern "C" fn safe_directory_contents(
    mut dir: *mut directory,
) -> *const libc::c_char {
    let mut ret: *const libc::c_char = directory_contents(dir);
    return if !ret.is_null() {
        ret
    } else {
        b"\0\0\0\0\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn obstack_code_rename(
    mut stk: *mut obstack,
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = if *from.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        from
    } else {
        safer_name_suffix(from, 0 as libc::c_int != 0, absolute_names_option)
            as *const libc::c_char
    };
    let mut __o: *mut obstack = stk;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o, 1 as libc::c_int as size_t);
    }
    let fresh11 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh11 = 'R' as i32 as libc::c_char;
    let mut __o_0: *mut obstack = stk;
    let mut __len: size_t = (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if ({
        let mut __o1: *const obstack = __o_0;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o_0, __len);
    }
    memcpy((*__o_0).next_free as *mut libc::c_void, s as *const libc::c_void, __len);
    (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
    s = if *to.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        to
    } else {
        safer_name_suffix(to, 0 as libc::c_int != 0, absolute_names_option)
            as *const libc::c_char
    };
    let mut __o_1: *mut obstack = stk;
    if ({
        let mut __o1: *const obstack = __o_1;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o_1, 1 as libc::c_int as size_t);
    }
    let fresh12 = (*__o_1).next_free;
    (*__o_1).next_free = ((*__o_1).next_free).offset(1);
    *fresh12 = 'T' as i32 as libc::c_char;
    let mut __o_2: *mut obstack = stk;
    let mut __len_0: size_t = (strlen(s))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if ({
        let mut __o1: *const obstack = __o_2;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len_0
    {
        _obstack_newchunk(__o_2, __len_0);
    }
    memcpy((*__o_2).next_free as *mut libc::c_void, s as *const libc::c_void, __len_0);
    (*__o_2).next_free = ((*__o_2).next_free).offset(__len_0 as isize);
}
unsafe extern "C" fn store_rename(mut dir: *mut directory, mut stk: *mut obstack) {
    let mut prev: *mut directory = 0 as *mut directory;
    let mut p: *mut directory = 0 as *mut directory;
    prev = dir;
    while !prev.is_null() && (*prev).orig != dir {
        prev = (*prev).orig;
    }
    if prev.is_null() {
        p = dir;
        while !p.is_null() && !((*p).orig).is_null() {
            obstack_code_rename(stk, (*(*p).orig).name, (*p).name);
            p = (*p).orig;
        }
    } else {
        let mut temp_name: *mut libc::c_char = 0 as *mut libc::c_char;
        temp_name = dir_name((*dir).name);
        let mut __o: *mut obstack = stk;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < 1 as libc::c_int as libc::c_ulong
        {
            _obstack_newchunk(__o, 1 as libc::c_int as size_t);
        }
        let fresh13 = (*__o).next_free;
        (*__o).next_free = ((*__o).next_free).offset(1);
        *fresh13 = 'X' as i32 as libc::c_char;
        let mut __o_0: *mut obstack = stk;
        let mut __len: size_t = (strlen(temp_name))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if ({
            let mut __o1: *const obstack = __o_0;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o_0, __len);
        }
        memcpy(
            (*__o_0).next_free as *mut libc::c_void,
            temp_name as *const libc::c_void,
            __len,
        );
        (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
        obstack_code_rename(stk, (*dir).name, b"\0" as *const u8 as *const libc::c_char);
        p = dir;
        while p != prev {
            obstack_code_rename(stk, (*(*p).orig).name, (*p).name);
            p = (*p).orig;
        }
        obstack_code_rename(
            stk,
            b"\0" as *const u8 as *const libc::c_char,
            (*prev).name,
        );
        rpl_free(temp_name as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn append_incremental_renames(mut dir: *mut directory) {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: 0 as *mut _obstack_chunk,
        object_base: 0 as *mut libc::c_char,
        next_free: 0 as *mut libc::c_char,
        chunk_limit: 0 as *mut libc::c_char,
        temp: C2RustUnnamed_1 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2RustUnnamed_0 { plain: None },
        freefun: C2RustUnnamed { plain: None },
        extra_arg: 0 as *mut libc::c_void,
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut size: size_t = 0;
    let mut dp: *mut directory = 0 as *mut directory;
    let mut dump: *const libc::c_char = 0 as *const libc::c_char;
    if dirhead.is_null() {
        return;
    }
    _obstack_begin(
        &mut stk,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    dump = directory_contents(dir);
    if !dump.is_null() {
        size = (dumpdir_size(dump)).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut __o: *mut obstack = &mut stk;
        let mut __len: size_t = size;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        memcpy(
            (*__o).next_free as *mut libc::c_void,
            dump as *const libc::c_void,
            __len,
        );
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    } else {
        size = 0 as libc::c_int as size_t;
    }
    dp = dirhead;
    while !dp.is_null() {
        if (*dp).flags & 0x10 as libc::c_int as libc::c_uint != 0 {
            store_rename(dp, &mut stk);
        }
        dp = (*dp).next;
    }
    if !dir.is_null()
        && ({
            let mut __o_0: *const obstack = &mut stk as *mut obstack;
            ((*__o_0).next_free).offset_from((*__o_0).object_base) as libc::c_long
                as size_t
        }) != size
    {
        let mut __o_0: *mut obstack = &mut stk;
        if ({
            let mut __o1: *const obstack = __o_0;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < 1 as libc::c_int as libc::c_ulong
        {
            _obstack_newchunk(__o_0, 1 as libc::c_int as size_t);
        }
        let fresh14 = (*__o_0).next_free;
        (*__o_0).next_free = ((*__o_0).next_free).offset(1);
        *fresh14 = 0 as libc::c_int as libc::c_char;
        dumpdir_free((*dir).dump);
        (*dir)
            .dump = dumpdir_create(
            ({
                let mut __o1: *mut obstack = &mut stk as *mut obstack;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut libc::c_char {
                    (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                }
                (*__o1)
                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                    as libc::c_ulong)
                    < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                {
                    (*__o1).object_base
                } else {
                    0 as *mut libc::c_char
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                },
                            ) as libc::c_long as libc::c_ulong)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
                    > ((*__o1).chunk_limit)
                        .offset_from((*__o1).chunk as *mut libc::c_char) as libc::c_long
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            }) as *const libc::c_char,
        );
    }
    let mut __o_1: *mut obstack = &mut stk;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o_1).chunk as *mut libc::c_void
        && __obj < (*__o_1).chunk_limit as *mut libc::c_void
    {
        (*__o_1).object_base = __obj as *mut libc::c_char;
        (*__o_1).next_free = (*__o_1).object_base;
    } else {
        _obstack_free(__o_1, __obj);
    };
}
static mut listed_incremental_stream: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn read_incr_db_01(
    mut version: libc::c_int,
    mut initbuf: *const libc::c_char,
) {
    let mut n: libc::c_int = 0;
    let mut u: uintmax_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut ebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lineno: libc::c_long = 1 as libc::c_int as libc::c_long;
    if version == 1 as libc::c_int {
        if getline(&mut buf, &mut bufsize, listed_incremental_stream)
            <= 0 as libc::c_int as libc::c_long
        {
            read_error(listed_incremental_option);
            rpl_free(buf as *mut libc::c_void);
            return;
        }
        lineno += 1;
        lineno;
    } else {
        buf = strdup(initbuf);
        bufsize = (strlen(buf)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    newer_mtime_option = decode_timespec(buf, &mut ebuf, 0 as libc::c_int != 0);
    if !valid_timespec(newer_mtime_option) {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            *__errno_location(),
            b"%s:%ld: %s\0" as *const u8 as *const libc::c_char,
            quotearg_colon(listed_incremental_option),
            lineno,
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid time stamp\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    } else if version == 1 as libc::c_int && *ebuf as libc::c_int != 0 {
        let mut buf_ns: *const libc::c_char = ebuf.offset(1 as libc::c_int as isize);
        *__errno_location() = 0 as libc::c_int;
        u = strtoumax(buf_ns, &mut ebuf, 10 as libc::c_int);
        if *__errno_location() == 0 && BILLION as libc::c_int as libc::c_ulong <= u {
            *__errno_location() = 34 as libc::c_int;
        }
        if *__errno_location() != 0 || buf_ns == ebuf as *const libc::c_char {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s:%ld: %s\0" as *const u8 as *const libc::c_char,
                quotearg_colon(listed_incremental_option),
                lineno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid time stamp\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit_status = 2 as libc::c_int;
            newer_mtime_option
                .tv_sec = !if (0 as libc::c_int as time_t)
                < -(1 as libc::c_int) as time_t
            {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            };
            newer_mtime_option.tv_nsec = -(1 as libc::c_int) as __syscall_slong_t;
        } else {
            newer_mtime_option.tv_nsec = u as __syscall_slong_t;
        }
    }
    loop {
        n = getline(&mut buf, &mut bufsize, listed_incremental_stream) as libc::c_int;
        if !((0 as libc::c_int) < n) {
            break;
        }
        let mut dev: dev_t = 0;
        let mut ino: ino_t = 0;
        let mut nfs: bool = *buf.offset(0 as libc::c_int as isize) as libc::c_int
            == '+' as i32;
        let mut strp: *mut libc::c_char = buf.offset(nfs as libc::c_int as isize);
        let mut mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        lineno += 1;
        lineno;
        if *buf.offset((n - 1 as libc::c_int) as isize) as libc::c_int == '\n' as i32 {
            *buf.offset((n - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
        }
        if version == 1 as libc::c_int {
            mtime = decode_timespec(strp, &mut ebuf, 0 as libc::c_int != 0);
            strp = ebuf;
            if !valid_timespec(mtime) || *strp as libc::c_int != ' ' as i32 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s:%ld: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(listed_incremental_option),
                    lineno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid modification time\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fatal_exit();
            }
            *__errno_location() = 0 as libc::c_int;
            u = strtoumax(strp, &mut ebuf, 10 as libc::c_int);
            if *__errno_location() == 0 && BILLION as libc::c_int as libc::c_ulong <= u {
                *__errno_location() = 34 as libc::c_int;
            }
            if *__errno_location() != 0 || strp == ebuf
                || *ebuf as libc::c_int != ' ' as i32
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s:%ld: %s\0" as *const u8 as *const libc::c_char,
                    quotearg_colon(listed_incremental_option),
                    lineno,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid modification time (nanoseconds)\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fatal_exit();
            } else {
                mtime.tv_nsec = u as __syscall_slong_t;
            }
            strp = ebuf;
        } else {
            mtime.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            mtime.tv_sec = mtime.tv_nsec;
        }
        dev = strtosysint(
            strp,
            &mut ebuf,
            !if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                -(1 as libc::c_int) as dev_t
            } else {
                ((1 as libc::c_int as dev_t)
                    << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } as intmax_t,
            if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                -(1 as libc::c_int) as dev_t
            } else {
                ((1 as libc::c_int as dev_t)
                    << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
        ) as dev_t;
        strp = ebuf;
        if *__errno_location() != 0 || *strp as libc::c_int != ' ' as i32 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s:%ld: %s\0" as *const u8 as *const libc::c_char,
                quotearg_colon(listed_incremental_option),
                lineno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid device number\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fatal_exit();
        }
        ino = strtosysint(
            strp,
            &mut ebuf,
            !if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                -(1 as libc::c_int) as ino_t
            } else {
                ((1 as libc::c_int as ino_t)
                    << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } as intmax_t,
            if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                -(1 as libc::c_int) as ino_t
            } else {
                ((1 as libc::c_int as ino_t)
                    << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
        ) as ino_t;
        strp = ebuf;
        if *__errno_location() != 0 || *strp as libc::c_int != ' ' as i32 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s:%ld: %s\0" as *const u8 as *const libc::c_char,
                quotearg_colon(listed_incremental_option),
                lineno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid inode number\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fatal_exit();
        }
        strp = strp.offset(1);
        strp;
        unquote_string(strp);
        note_directory(
            strp,
            mtime,
            dev,
            ino,
            nfs,
            0 as libc::c_int != 0,
            0 as *const libc::c_char,
        );
    }
    rpl_free(buf as *mut libc::c_void);
}
unsafe extern "C" fn read_obstack(
    mut fp: *mut FILE,
    mut stk: *mut obstack,
    mut pcount: *mut size_t,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    c = getc_unlocked(fp);
    while c != -(1 as libc::c_int) && c != 0 as libc::c_int {
        let mut __o: *mut obstack = stk;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < 1 as libc::c_int as libc::c_ulong
        {
            _obstack_newchunk(__o, 1 as libc::c_int as size_t);
        }
        let fresh15 = (*__o).next_free;
        (*__o).next_free = ((*__o).next_free).offset(1);
        *fresh15 = c as libc::c_char;
        c = getc_unlocked(fp);
        i = i.wrapping_add(1);
        i;
    }
    let mut __o_0: *mut obstack = stk;
    if ({
        let mut __o1: *const obstack = __o_0;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o_0, 1 as libc::c_int as size_t);
    }
    let fresh16 = (*__o_0).next_free;
    (*__o_0).next_free = ((*__o_0).next_free).offset(1);
    *fresh16 = 0 as libc::c_int as libc::c_char;
    *pcount = i;
    return c;
}
unsafe extern "C" fn read_num(
    mut fp: *mut FILE,
    mut fieldname: *const libc::c_char,
    mut min_val: intmax_t,
    mut max_val: uintmax_t,
    mut pval: *mut intmax_t,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut offbuf: [libc::c_char; 21] = [0; 21];
    let mut minbuf: [libc::c_char; 21] = [0; 21];
    let mut maxbuf: [libc::c_char; 21] = [0; 21];
    let mut conversion_errno: libc::c_int = 0;
    let mut c: libc::c_int = getc_unlocked(fp);
    let mut negative: bool = c == '-' as i32;
    i = 0 as libc::c_int;
    while i == 0 as libc::c_int && negative as libc::c_int != 0
        || (c as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
    {
        buf[i as usize] = c as libc::c_char;
        if i as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: byte %s: %s %.*s... too long\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(listed_incremental_option),
                offtostr(ftello(fp), offbuf.as_mut_ptr()),
                fieldname,
                i + 1 as libc::c_int,
                buf.as_mut_ptr(),
            );
            fatal_exit();
        }
        c = getc_unlocked(fp);
        i += 1;
        i;
    }
    buf[i as usize] = 0 as libc::c_int as libc::c_char;
    if c < 0 as libc::c_int {
        if ferror_unlocked(fp) != 0 {
            read_fatal(listed_incremental_option);
        }
        if i != 0 as libc::c_int {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                quotearg_colon(listed_incremental_option),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unexpected EOF in snapshot file\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fatal_exit();
        }
        return 0 as libc::c_int != 0;
    }
    if c != 0 {
        let mut uc: libc::c_uint = c as libc::c_uint;
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: byte %s: %s %s followed by invalid byte 0x%02x\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            quotearg_colon(listed_incremental_option),
            offtostr(ftello(fp), offbuf.as_mut_ptr()),
            fieldname,
            buf.as_mut_ptr(),
            uc,
        );
        fatal_exit();
    }
    *pval = strtosysint(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char, min_val, max_val);
    conversion_errno = *__errno_location();
    match conversion_errno {
        34 => {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                conversion_errno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: byte %s: (valid range %s..%s)\n\t%s %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(listed_incremental_option),
                offtostr(ftello(fp), offbuf.as_mut_ptr()),
                imaxtostr(min_val, minbuf.as_mut_ptr()),
                umaxtostr(max_val, maxbuf.as_mut_ptr()),
                fieldname,
                buf.as_mut_ptr(),
            );
            fatal_exit();
        }
        0 => {}
        _ => {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                conversion_errno,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: byte %s: %s %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(listed_incremental_option),
                offtostr(ftello(fp), offbuf.as_mut_ptr()),
                fieldname,
                buf.as_mut_ptr(),
            );
            fatal_exit();
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn read_timespec(mut fp: *mut FILE, mut pval: *mut timespec) {
    let mut s: intmax_t = 0;
    let mut ns: intmax_t = 0;
    if read_num(
        fp,
        b"sec\0" as *const u8 as *const libc::c_char,
        !(if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }),
        (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
        &mut s,
    ) as libc::c_int != 0
        && read_num(
            fp,
            b"nsec\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as intmax_t,
            (BILLION as libc::c_int - 1 as libc::c_int) as uintmax_t,
            &mut ns,
        ) as libc::c_int != 0
    {
        (*pval).tv_sec = s;
        (*pval).tv_nsec = ns;
    } else {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            quotearg_colon(listed_incremental_option),
            dcgettext(
                0 as *const libc::c_char,
                b"Unexpected EOF in snapshot file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    };
}
unsafe extern "C" fn read_incr_db_2() {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: 0 as *mut _obstack_chunk,
        object_base: 0 as *mut libc::c_char,
        next_free: 0 as *mut libc::c_char,
        chunk_limit: 0 as *mut libc::c_char,
        temp: C2RustUnnamed_1 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2RustUnnamed_0 { plain: None },
        freefun: C2RustUnnamed { plain: None },
        extra_arg: 0 as *mut libc::c_void,
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut offbuf: [libc::c_char; 21] = [0; 21];
    _obstack_begin(
        &mut stk,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    read_timespec(listed_incremental_stream, &mut newer_mtime_option);
    loop {
        let mut i: intmax_t = 0;
        let mut mtime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut dev: dev_t = 0;
        let mut ino: ino_t = 0;
        let mut nfs: bool = false;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut content: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: size_t = 0;
        if !read_num(
            listed_incremental_stream,
            b"nfs\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as intmax_t,
            1 as libc::c_int as uintmax_t,
            &mut i,
        ) {
            return;
        }
        nfs = i != 0;
        read_timespec(listed_incremental_stream, &mut mtime);
        if !read_num(
            listed_incremental_stream,
            b"dev\0" as *const u8 as *const libc::c_char,
            !if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                -(1 as libc::c_int) as dev_t
            } else {
                ((1 as libc::c_int as dev_t)
                    << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } as intmax_t,
            if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                -(1 as libc::c_int) as dev_t
            } else {
                ((1 as libc::c_int as dev_t)
                    << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
            &mut i,
        ) {
            break;
        }
        dev = i as dev_t;
        if !read_num(
            listed_incremental_stream,
            b"ino\0" as *const u8 as *const libc::c_char,
            !if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                -(1 as libc::c_int) as ino_t
            } else {
                ((1 as libc::c_int as ino_t)
                    << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } as intmax_t,
            if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                -(1 as libc::c_int) as ino_t
            } else {
                ((1 as libc::c_int as ino_t)
                    << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
            &mut i,
        ) {
            break;
        }
        ino = i as ino_t;
        if read_obstack(listed_incremental_stream, &mut stk, &mut s) != 0 {
            break;
        }
        name = ({
            let mut __o1: *mut obstack = &mut stk as *mut obstack;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *mut libc::c_char;
        while read_obstack(listed_incremental_stream, &mut stk, &mut s)
            == 0 as libc::c_int && s > 1 as libc::c_int as libc::c_ulong
        {}
        if getc_unlocked(listed_incremental_stream) != 0 as libc::c_int {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: byte %s: %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(listed_incremental_option),
                offtostr(ftello(listed_incremental_stream), offbuf.as_mut_ptr()),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Missing record terminator\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fatal_exit();
        }
        content = ({
            let mut __o1: *mut obstack = &mut stk as *mut obstack;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *mut libc::c_char;
        note_directory(name, mtime, dev, ino, nfs, 0 as libc::c_int != 0, content);
        let mut __o: *mut obstack = &mut stk;
        let mut __obj: *mut libc::c_void = content as *mut libc::c_void;
        if __obj > (*__o).chunk as *mut libc::c_void
            && __obj < (*__o).chunk_limit as *mut libc::c_void
        {
            (*__o).object_base = __obj as *mut libc::c_char;
            (*__o).next_free = (*__o).object_base;
        } else {
            _obstack_free(__o, __obj);
        }
    }
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        b"%s: %s\0" as *const u8 as *const libc::c_char,
        quotearg_colon(listed_incremental_option),
        dcgettext(
            0 as *const libc::c_char,
            b"Unexpected EOF in snapshot file\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    fatal_exit();
}
static mut field_ranges: [field_range; 6] = [field_range {
    fieldname: 0 as *const libc::c_char,
    min_val: 0,
    max_val: 0,
}; 6];
pub unsafe extern "C" fn show_snapshot_field_ranges() {
    let mut p: *const field_range = 0 as *const field_range;
    let mut minbuf: [libc::c_char; 21] = [0; 21];
    let mut maxbuf: [libc::c_char; 21] = [0; 21];
    printf(
        b"This tar's snapshot file field ranges are\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"   (%-15s => [ %s, %s ]):\n\n\0" as *const u8 as *const libc::c_char,
        b"field name\0" as *const u8 as *const libc::c_char,
        b"min\0" as *const u8 as *const libc::c_char,
        b"max\0" as *const u8 as *const libc::c_char,
    );
    p = field_ranges.as_ptr();
    while !((*p).fieldname).is_null() {
        printf(
            b"    %-15s => [ %s, %s ],\n\0" as *const u8 as *const libc::c_char,
            (*p).fieldname,
            sysinttostr(
                (*p).min_val as uintmax_t,
                (*p).min_val,
                (*p).max_val,
                minbuf.as_mut_ptr(),
            ),
            sysinttostr((*p).max_val, (*p).min_val, (*p).max_val, maxbuf.as_mut_ptr()),
        );
        p = p.offset(1);
        p;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn read_directory_file() {
    let mut fd: libc::c_int = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut flags: libc::c_int = 0o2 as libc::c_int | 0o100 as libc::c_int;
    if incremental_level == 0 as libc::c_int {
        flags |= 0o1000 as libc::c_int;
    }
    fd = open(
        listed_incremental_option,
        flags,
        0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
            | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int),
    );
    if fd < 0 as libc::c_int {
        open_error(listed_incremental_option);
        return;
    }
    listed_incremental_stream = fdopen(fd, b"r+\0" as *const u8 as *const libc::c_char);
    if listed_incremental_stream.is_null() {
        open_error(listed_incremental_option);
        close(fd);
        return;
    }
    name_from_list();
    blank_name_list();
    if (0 as libc::c_int as libc::c_long)
        < getline(&mut buf, &mut bufsize, listed_incremental_stream)
    {
        let mut ebuf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut incremental_version: uintmax_t = 0;
        if strncmp(
            buf,
            b"GNU tar\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            ebuf = buf
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize));
            let fresh17 = ebuf;
            ebuf = ebuf.offset(1);
            if *fresh17 as libc::c_int != '-' as i32 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Bad incremental file format\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                exit_status = 2 as libc::c_int;
            }
            while *ebuf as libc::c_int != '-' as i32 {
                if *ebuf == 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Bad incremental file format\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit_status = 2 as libc::c_int;
                }
                ebuf = ebuf.offset(1);
                ebuf;
            }
            incremental_version = strtoumax(
                ebuf.offset(1 as libc::c_int as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            );
        } else {
            incremental_version = 0 as libc::c_int as uintmax_t;
        }
        match incremental_version {
            0 | 1 => {
                read_incr_db_01(incremental_version as libc::c_int, buf);
            }
            2 => {
                read_incr_db_2();
            }
            _ => {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unsupported incremental format version: %lu\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    incremental_version,
                );
                exit_status = 2 as libc::c_int;
            }
        }
    }
    if ferror_unlocked(listed_incremental_stream) != 0 {
        read_error(listed_incremental_option);
    }
    rpl_free(buf as *mut libc::c_void);
}
unsafe extern "C" fn write_directory_file_entry(
    mut entry: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> bool {
    let mut directory: *const directory = entry as *const directory;
    let mut fp: *mut FILE = data as *mut FILE;
    if (*directory).flags & 0x4 as libc::c_int as libc::c_uint != 0 {
        let mut buf: [libc::c_char; 21] = [0; 21];
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        s = if (*directory).flags & 0x2 as libc::c_int as libc::c_uint != 0 {
            b"1\0" as *const u8 as *const libc::c_char
        } else {
            b"0\0" as *const u8 as *const libc::c_char
        };
        if 0 != 0 && 0 != 0
            && (2 as libc::c_int as size_t).wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 2 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = s;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (2 as libc::c_int as size_t)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh18 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh18 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                0u8
            });
        } else {
            if 0 != 0 && 2 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {} else {
                fwrite_unlocked(
                    s as *const libc::c_void,
                    2 as libc::c_int as size_t,
                    1 as libc::c_int as size_t,
                    fp,
                );
            };
        };
        0u8;
        s = sysinttostr(
            (*directory).mtime.tv_sec as uintmax_t,
            !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            },
            (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }) as uintmax_t,
            buf.as_mut_ptr(),
        );
        if 0 != 0 && 0 != 0
            && (strlen(s))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = s;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (strlen(s))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh19 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh19 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                strlen(s);
                strlen(s);
                0u8
            });
        } else {
            if 0 != 0
                && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {
                strlen(s);
            } else {
                fwrite_unlocked(
                    s as *const libc::c_void,
                    (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as size_t,
                    fp,
                );
            };
        };
        0u8;
        s = imaxtostr((*directory).mtime.tv_nsec, buf.as_mut_ptr());
        if 0 != 0 && 0 != 0
            && (strlen(s))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = s;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (strlen(s))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh20 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh20 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                strlen(s);
                strlen(s);
                0u8
            });
        } else {
            if 0 != 0
                && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {
                strlen(s);
            } else {
                fwrite_unlocked(
                    s as *const libc::c_void,
                    (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as size_t,
                    fp,
                );
            };
        };
        0u8;
        s = sysinttostr(
            (*directory).device_number,
            !if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                -(1 as libc::c_int) as dev_t
            } else {
                ((1 as libc::c_int as dev_t)
                    << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } as intmax_t,
            if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                -(1 as libc::c_int) as dev_t
            } else {
                ((1 as libc::c_int as dev_t)
                    << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
            buf.as_mut_ptr(),
        );
        if 0 != 0 && 0 != 0
            && (strlen(s))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = s;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (strlen(s))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh21 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh21 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                strlen(s);
                strlen(s);
                0u8
            });
        } else {
            if 0 != 0
                && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {
                strlen(s);
            } else {
                fwrite_unlocked(
                    s as *const libc::c_void,
                    (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as size_t,
                    fp,
                );
            };
        };
        0u8;
        s = sysinttostr(
            (*directory).inode_number,
            !if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                -(1 as libc::c_int) as ino_t
            } else {
                ((1 as libc::c_int as ino_t)
                    << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } as intmax_t,
            if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                -(1 as libc::c_int) as ino_t
            } else {
                ((1 as libc::c_int as ino_t)
                    << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
            buf.as_mut_ptr(),
        );
        if 0 != 0 && 0 != 0
            && (strlen(s))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = s;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (strlen(s))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh22 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh22 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                strlen(s);
                strlen(s);
                0u8
            });
        } else {
            if 0 != 0
                && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {
                strlen(s);
            } else {
                fwrite_unlocked(
                    s as *const libc::c_void,
                    (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as size_t,
                    fp,
                );
            };
        };
        0u8;
        if 0 != 0 && 0 != 0
            && (strlen((*directory).name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && (strlen((*directory).name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = (*directory).name
                    as *const libc::c_char;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (strlen((*directory).name))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh23 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh23 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                strlen((*directory).name);
                strlen((*directory).name);
                0u8
            });
        } else {
            if 0 != 0
                && (strlen((*directory).name))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {
                strlen((*directory).name);
            } else {
                fwrite_unlocked(
                    (*directory).name as *const libc::c_void,
                    (strlen((*directory).name))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as size_t,
                    fp,
                );
            };
        };
        0u8;
        if !((*directory).dump).is_null() {
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            let mut itr: *mut dumpdir_iter = 0 as *mut dumpdir_iter;
            p = dumpdir_first((*directory).dump, 0 as libc::c_int, &mut itr);
            while !p.is_null() {
                if 0 != 0 && 0 != 0
                    && (strlen(p))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(1 as libc::c_int as size_t)
                        <= 8 as libc::c_int as libc::c_ulong
                    && (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        != 0 as libc::c_int as libc::c_ulong
                {
                    ({
                        let mut __ptr: *const libc::c_char = p;
                        let mut __stream: *mut FILE = fp;
                        let mut __cnt: size_t = 0;
                        __cnt = (strlen(p))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(1 as libc::c_int as size_t);
                        while __cnt > 0 as libc::c_int as libc::c_ulong {
                            let fresh24 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*fresh24 as libc::c_int, __stream)
                                == -(1 as libc::c_int)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        strlen(p);
                        strlen(p);
                        0u8
                    });
                } else {
                    if 0 != 0
                        && (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            == 0 as libc::c_int as libc::c_ulong
                        || 0 != 0
                            && 1 as libc::c_int as size_t
                                == 0 as libc::c_int as libc::c_ulong
                    {
                        strlen(p);
                    } else {
                        fwrite_unlocked(
                            p as *const libc::c_void,
                            (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                            1 as libc::c_int as size_t,
                            fp,
                        );
                    };
                };
                0u8;
                p = dumpdir_next(itr);
            }
            rpl_free(itr as *mut libc::c_void);
        }
        if 0 != 0 && 0 != 0
            && (2 as libc::c_int as size_t).wrapping_mul(1 as libc::c_int as size_t)
                <= 8 as libc::c_int as libc::c_ulong
            && 2 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
        {
            ({
                let mut __ptr: *const libc::c_char = b"\0\0\0" as *const u8
                    as *const libc::c_char;
                let mut __stream: *mut FILE = fp;
                let mut __cnt: size_t = 0;
                __cnt = (2 as libc::c_int as size_t)
                    .wrapping_mul(1 as libc::c_int as size_t);
                while __cnt > 0 as libc::c_int as libc::c_ulong {
                    let fresh25 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*fresh25 as libc::c_int, __stream)
                        == -(1 as libc::c_int)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                0u8
            });
        } else {
            if 0 != 0 && 2 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
            {} else {
                fwrite_unlocked(
                    b"\0\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                    1 as libc::c_int as size_t,
                    fp,
                );
            };
        };
        0u8;
    }
    return ferror_unlocked(fp) == 0;
}
pub unsafe extern "C" fn write_directory_file() {
    let mut fp: *mut FILE = listed_incremental_stream;
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if fp.is_null() {
        return;
    }
    if fseeko(fp, 0 as libc::c_long, 0 as libc::c_int) != 0 as libc::c_int {
        seek_error(listed_incremental_option);
    }
    if sys_truncate(fileno(fp)) != 0 as libc::c_int {
        truncate_error(listed_incremental_option);
    }
    fprintf(
        fp,
        b"%s-%s-%d\n\0" as *const u8 as *const libc::c_char,
        b"GNU tar\0" as *const u8 as *const libc::c_char,
        b"1.34\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    );
    s = if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        imaxtostr(start_time.tv_sec, buf.as_mut_ptr())
    } else {
        umaxtostr(start_time.tv_sec as uintmax_t, buf.as_mut_ptr())
    };
    if 0 != 0 && 0 != 0
        && (strlen(s))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as size_t)
            <= 8 as libc::c_int as libc::c_ulong
        && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = s as *const libc::c_char;
            let mut __stream: *mut FILE = fp;
            let mut __cnt: size_t = 0;
            __cnt = (strlen(s))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh26 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh26 as libc::c_int, __stream)
                    == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            strlen(s);
            strlen(s);
            0u8
        });
    } else {
        if 0 != 0
            && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
        {
            strlen(s);
        } else {
            fwrite_unlocked(
                s as *const libc::c_void,
                (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                1 as libc::c_int as size_t,
                fp,
            );
        };
    };
    0u8;
    s = umaxtostr(start_time.tv_nsec as uintmax_t, buf.as_mut_ptr());
    if 0 != 0 && 0 != 0
        && (strlen(s))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(1 as libc::c_int as size_t)
            <= 8 as libc::c_int as libc::c_ulong
        && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            != 0 as libc::c_int as libc::c_ulong
    {
        ({
            let mut __ptr: *const libc::c_char = s as *const libc::c_char;
            let mut __stream: *mut FILE = fp;
            let mut __cnt: size_t = 0;
            __cnt = (strlen(s))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(1 as libc::c_int as size_t);
            while __cnt > 0 as libc::c_int as libc::c_ulong {
                let fresh27 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*fresh27 as libc::c_int, __stream)
                    == -(1 as libc::c_int)
                {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
                __cnt;
            }
            strlen(s);
            strlen(s);
            0u8
        });
    } else {
        if 0 != 0
            && (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            || 0 != 0 && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
        {
            strlen(s);
        } else {
            fwrite_unlocked(
                s as *const libc::c_void,
                (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                1 as libc::c_int as size_t,
                fp,
            );
        };
    };
    0u8;
    if ferror_unlocked(fp) == 0 && !directory_table.is_null() {
        hash_do_for_each(
            directory_table,
            Some(
                write_directory_file_entry
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
            ),
            fp as *mut libc::c_void,
        );
    }
    if ferror_unlocked(fp) != 0 {
        write_error(listed_incremental_option);
    }
    if fclose(fp) != 0 as libc::c_int {
        close_error(listed_incremental_option);
    }
}
unsafe extern "C" fn get_gnu_dumpdir(mut stat_info: *mut tar_stat_info) {
    let mut size: size_t = 0;
    let mut copied: size_t = 0;
    let mut data_block: *mut block = 0 as *mut block;
    let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut archive_dir: *mut libc::c_char = 0 as *mut libc::c_char;
    size = (*stat_info).stat.st_size as size_t;
    archive_dir = xmalloc(size) as *mut libc::c_char;
    to = archive_dir;
    set_next_block_after(current_header);
    mv_begin_read(stat_info);
    while size > 0 as libc::c_int as libc::c_ulong {
        mv_size_left(size as off_t);
        data_block = find_next_block();
        if data_block.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unexpected EOF in archive\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit_status = 2 as libc::c_int;
        }
        copied = available_space_after(data_block);
        if copied > size {
            copied = size;
        }
        memcpy(
            to as *mut libc::c_void,
            ((*data_block).buffer).as_mut_ptr() as *const libc::c_void,
            copied,
        );
        to = to.offset(copied as isize);
        set_next_block_after(
            ((*data_block).buffer)
                .as_mut_ptr()
                .offset(copied as isize)
                .offset(-(1 as libc::c_int as isize)) as *mut block,
        );
        size = (size as libc::c_ulong).wrapping_sub(copied) as size_t as size_t;
    }
    mv_end();
    (*stat_info).dumpdir = archive_dir;
    (*stat_info).skipped = 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn is_dumpdir(mut stat_info: *mut tar_stat_info) -> bool {
    if (*stat_info).is_dumpdir as libc::c_int != 0 && ((*stat_info).dumpdir).is_null() {
        get_gnu_dumpdir(stat_info);
    }
    return (*stat_info).is_dumpdir;
}
unsafe extern "C" fn dumpdir_ok(mut dumpdir: *mut libc::c_char) -> bool {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut has_tempdir: libc::c_int = 0 as libc::c_int;
    let mut expect: libc::c_int = 0 as libc::c_int;
    p = dumpdir;
    while *p != 0 {
        if expect != 0 && *p as libc::c_int != expect {
            let mut uc: libc::c_uchar = *p as libc::c_uchar;
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed dumpdir: expected '%c' but found %#3o\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                expect,
                uc as libc::c_int,
            );
            exit_status = 2 as libc::c_int;
            return 0 as libc::c_int != 0;
        }
        match *p as libc::c_int {
            88 => {
                if has_tempdir != 0 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Malformed dumpdir: 'X' duplicated\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit_status = 2 as libc::c_int;
                    return 0 as libc::c_int != 0;
                } else {
                    has_tempdir = 1 as libc::c_int;
                }
            }
            82 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                {
                    if has_tempdir == 0 {
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Malformed dumpdir: empty name in 'R'\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        exit_status = 2 as libc::c_int;
                        return 0 as libc::c_int != 0;
                    } else {
                        has_tempdir = 0 as libc::c_int;
                    }
                }
                expect = 'T' as i32;
            }
            84 => {
                if expect != 'T' as i32 {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Malformed dumpdir: 'T' not preceded by 'R'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit_status = 2 as libc::c_int;
                    return 0 as libc::c_int != 0;
                }
                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int && has_tempdir == 0
                {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Malformed dumpdir: empty name in 'T'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit_status = 2 as libc::c_int;
                    return 0 as libc::c_int != 0;
                }
                expect = 0 as libc::c_int;
            }
            78 | 89 | 68 | _ => {}
        }
        p = p
            .offset(
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    if expect != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Malformed dumpdir: expected '%c' but found end of data\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            expect,
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    if has_tempdir != 0 {
        if warning_option & 0x2 as libc::c_int != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed dumpdir: 'X' never used\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn try_purge_directory(
    mut directory_name: *const libc::c_char,
) -> bool {
    let mut current_dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_stub: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dump: *mut dumpdir = 0 as *mut dumpdir;
    if !is_dumpdir(&mut current_stat_info) {
        return 0 as libc::c_int != 0;
    }
    current_dir = tar_savedir(directory_name, 0 as libc::c_int);
    if current_dir.is_null() {
        return 0 as libc::c_int != 0;
    }
    if !dumpdir_ok(current_stat_info.dumpdir) {
        return 0 as libc::c_int != 0;
    }
    arc = current_stat_info.dumpdir;
    while *arc != 0 {
        if *arc as libc::c_int == 'X' as i32 {
            let mut len: size_t = strlen(arc.offset(1 as libc::c_int as isize));
            temp_stub = xrealloc(
                temp_stub as *mut libc::c_void,
                len
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            memcpy(
                temp_stub as *mut libc::c_void,
                arc.offset(1 as libc::c_int as isize) as *const libc::c_void,
                len,
            );
            *temp_stub.offset(len as isize) = '/' as i32 as libc::c_char;
            memcpy(
                temp_stub.offset(len as isize).offset(1 as libc::c_int as isize)
                    as *mut libc::c_void,
                b"tar.XXXXXX\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
            );
            if (mkdtemp(temp_stub)).is_null() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cannot create temporary directory using template %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quote(temp_stub),
                );
                exit_status = 2 as libc::c_int;
                rpl_free(temp_stub as *mut libc::c_void);
                rpl_free(current_dir as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
        } else if *arc as libc::c_int == 'R' as i32 {
            let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
            src = arc.offset(1 as libc::c_int as isize);
            arc = arc
                .offset(
                    (strlen(arc)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                );
            dst = arc.offset(1 as libc::c_int as isize);
            if *src != 0 {
                src = safer_name_suffix(
                    src,
                    0 as libc::c_int != 0,
                    absolute_names_option,
                );
            }
            if *dst != 0 {
                dst = safer_name_suffix(
                    dst,
                    0 as libc::c_int != 0,
                    absolute_names_option,
                );
            }
            if *src as libc::c_int == 0 as libc::c_int {
                src = temp_stub;
            } else if *dst as libc::c_int == 0 as libc::c_int {
                dst = temp_stub;
            }
            if !rename_directory(src, dst) {
                rpl_free(temp_stub as *mut libc::c_void);
                rpl_free(current_dir as *mut libc::c_void);
                return 0 as libc::c_int != 0;
            }
        }
        arc = arc
            .offset(
                (strlen(arc)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    rpl_free(temp_stub as *mut libc::c_void);
    dump = dumpdir_create(current_stat_info.dumpdir);
    p = 0 as *mut libc::c_char;
    cur = current_dir;
    while *cur != 0 {
        let mut entry: *const libc::c_char = 0 as *const libc::c_char;
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
        rpl_free(p as *mut libc::c_void);
        p = make_file_name(directory_name, cur);
        if deref_stat(p, &mut st) != 0 as libc::c_int {
            if *__errno_location() != 2 as libc::c_int {
                stat_diag(p);
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Not purging directory: unable to stat\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quotearg_colon(p),
                );
            }
        } else {
            entry = dumpdir_locate(dump, cur);
            if entry.is_null()
                || *entry as libc::c_int == 'D' as i32
                    && !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
                || *entry as libc::c_int == 'Y' as i32
                    && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
            {
                if one_file_system_option as libc::c_int != 0 && st.st_dev != root_device
                {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: directory is on a different device: not purging\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        quotearg_colon(p),
                    );
                } else if !interactive_option
                    || confirm(b"delete\0" as *const u8 as *const libc::c_char, p) != 0
                {
                    if verbose_option != 0 {
                        fprintf(
                            stdlis,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: Deleting %s\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            program_name,
                            quote(p),
                        );
                    }
                    if remove_any_file(p, RECURSIVE_REMOVE_OPTION) == 0 {
                        let mut e: libc::c_int = *__errno_location();
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            e,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: Cannot remove\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            quotearg_colon(p),
                        );
                        exit_status = 2 as libc::c_int;
                    }
                }
            }
        }
        cur = cur
            .offset(
                (strlen(cur)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
    }
    rpl_free(p as *mut libc::c_void);
    dumpdir_free(dump);
    rpl_free(current_dir as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn purge_directory(mut directory_name: *const libc::c_char) {
    if !try_purge_directory(directory_name) {
        skip_member();
    }
}
pub unsafe extern "C" fn list_dumpdir(mut buffer: *mut libc::c_char, mut size: size_t) {
    let mut state: libc::c_int = 0 as libc::c_int;
    while size != 0 {
        match *buffer as libc::c_int {
            89 | 78 | 68 | 82 | 84 | 88 => {
                fprintf(
                    stdlis,
                    b"%c\0" as *const u8 as *const libc::c_char,
                    *buffer as libc::c_int,
                );
                if state == 0 as libc::c_int {
                    fprintf(stdlis, b" \0" as *const u8 as *const libc::c_char);
                    state = 1 as libc::c_int;
                }
                buffer = buffer.offset(1);
                buffer;
                size = size.wrapping_sub(1);
                size;
            }
            0 => {
                fputc_unlocked('\n' as i32, stdlis);
                buffer = buffer.offset(1);
                buffer;
                size = size.wrapping_sub(1);
                size;
                state = 0 as libc::c_int;
            }
            _ => {
                fputc_unlocked(*buffer as libc::c_int, stdlis);
                buffer = buffer.offset(1);
                buffer;
                size = size.wrapping_sub(1);
                size;
            }
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    field_ranges = [
        {
            let mut init = field_range {
                fieldname: b"nfs\0" as *const u8 as *const libc::c_char,
                min_val: 0 as libc::c_int as intmax_t,
                max_val: 1 as libc::c_int as uintmax_t,
            };
            init
        },
        {
            let mut init = field_range {
                fieldname: b"timestamp_sec\0" as *const u8 as *const libc::c_char,
                min_val: !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t
                {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                },
                max_val: (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t
                {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as uintmax_t,
            };
            init
        },
        {
            let mut init = field_range {
                fieldname: b"timestamp_nsec\0" as *const u8 as *const libc::c_char,
                min_val: 0 as libc::c_int as intmax_t,
                max_val: (BILLION as libc::c_int - 1 as libc::c_int) as uintmax_t,
            };
            init
        },
        {
            let mut init = field_range {
                fieldname: b"dev\0" as *const u8 as *const libc::c_char,
                min_val: !if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                    -(1 as libc::c_int) as dev_t
                } else {
                    ((1 as libc::c_int as dev_t)
                        << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                } as intmax_t,
                max_val: if (0 as libc::c_int as dev_t) < -(1 as libc::c_int) as dev_t {
                    -(1 as libc::c_int) as dev_t
                } else {
                    ((1 as libc::c_int as dev_t)
                        << (::std::mem::size_of::<dev_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                },
            };
            init
        },
        {
            let mut init = field_range {
                fieldname: b"ino\0" as *const u8 as *const libc::c_char,
                min_val: !if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                    -(1 as libc::c_int) as ino_t
                } else {
                    ((1 as libc::c_int as ino_t)
                        << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                } as intmax_t,
                max_val: if (0 as libc::c_int as ino_t) < -(1 as libc::c_int) as ino_t {
                    -(1 as libc::c_int) as ino_t
                } else {
                    ((1 as libc::c_int as ino_t)
                        << (::std::mem::size_of::<ino_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                },
            };
            init
        },
        {
            let mut init = field_range {
                fieldname: 0 as *const libc::c_char,
                min_val: 0 as libc::c_int as intmax_t,
                max_val: 0 as libc::c_int as uintmax_t,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
