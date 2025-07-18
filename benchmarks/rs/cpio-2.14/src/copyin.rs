use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn __xmknod(
        __ver: libc::c_int,
        __path: *const libc::c_char,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn lchown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn unlink_error(_: *const libc::c_char);
    fn symlink_error(_: *const libc::c_char, _: *const libc::c_char);
    fn stat_error(_: *const libc::c_char);
    fn open_fatal(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn mknod_error(_: *const libc::c_char);
    fn close_error(_: *const libc::c_char);
    fn chown_error_details(name: *const libc::c_char, uid: uid_t, gid: gid_t);
    fn chmod_error_details(name: *const libc::c_char, mode: mode_t);
    fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn cpio_file_stat_free(file_hdr: *mut cpio_file_stat);
    fn cpio_set_c_name(file_hdr: *mut cpio_file_stat, name: *mut libc::c_char);
    fn cpio_realloc_c_name(file_hdr: *mut cpio_file_stat, len: size_t);
    fn ds_init(string: *mut dynamic_string);
    fn ds_free(string: *mut dynamic_string);
    fn ds_fgets(f: *mut FILE, s: *mut dynamic_string) -> *mut libc::c_char;
    fn ds_fgetstr(
        f: *mut FILE,
        s: *mut dynamic_string,
        eos: libc::c_char,
    ) -> *mut libc::c_char;
    static mut archive_format: archive_format;
    fn quotearg(arg: *const libc::c_char) -> *mut libc::c_char;
    fn getgroup(gid: gid_t) -> *mut libc::c_char;
    static mut input_bytes: off_t;
    static mut save_patterns: *mut *mut libc::c_char;
    static mut num_patterns: libc::c_int;
    static mut name_end: libc::c_char;
    static mut input_is_special: libc::c_char;
    static mut input_is_seekable: libc::c_char;
    static mut output_is_seekable: libc::c_char;
    static mut in_buff: *mut libc::c_char;
    static mut input_buffer: *mut libc::c_char;
    static mut crc: uint32_t;
    static mut archive_des: libc::c_int;
    static mut pattern_file_name: *mut libc::c_char;
    static mut numeric_uid: libc::c_int;
    static mut copy_matching_files: libc::c_int;
    static mut last_header_start: off_t;
    static mut set_owner: uid_t;
    static mut set_group_flag: libc::c_int;
    static mut set_group: gid_t;
    static mut no_chown_flag: libc::c_int;
    static mut to_stdout_option: bool;
    static mut quiet_flag: libc::c_int;
    static mut only_verify_crc_flag: libc::c_int;
    static mut no_abs_paths_flag: libc::c_int;
    fn mode_string(mode: libc::c_uint, str: *mut libc::c_char);
    static mut swapping_halfwords: libc::c_int;
    static mut swapping_bytes: libc::c_int;
    static mut swap_halfwords_flag: libc::c_int;
    static mut newdir_umask: mode_t;
    fn link_to_maj_min_ino(
        file_name: *mut libc::c_char,
        st_dev_maj: libc::c_int,
        st_dev_min: libc::c_int,
        st_ino: ino_t,
    ) -> libc::c_int;
    static mut set_owner_flag: libc::c_int;
    static mut swap_bytes_flag: libc::c_int;
    static mut io_block_size: libc::c_int;
    static mut create_dir_flag: libc::c_int;
    static mut rename_flag: libc::c_int;
    static mut rename_batch_file: *mut libc::c_char;
    fn link_to_name(
        link_name: *const libc::c_char,
        link_target: *const libc::c_char,
    ) -> libc::c_int;
    static mut table_flag: libc::c_int;
    static mut append_flag: libc::c_int;
    static mut crc_i_flag: libc::c_int;
    static mut unconditional_flag: libc::c_int;
    static mut verbose_flag: libc::c_int;
    static mut dot_flag: libc::c_int;
    static mut retain_time_flag: libc::c_int;
    fn set_perms(fd: libc::c_int, header: *mut cpio_file_stat);
    fn tape_buffered_read(
        in_buf: *mut libc::c_char,
        in_des: libc::c_int,
        num_bytes: off_t,
    );
    fn tape_toss_input(in_des: libc::c_int, num_bytes: off_t);
    fn disk_empty_output_buffer(out_des: libc::c_int, flush: bool);
    fn copy_files_tape_to_disk(
        in_des: libc::c_int,
        out_des: libc::c_int,
        num_bytes: off_t,
    );
    fn read_in_tar_header(file_hdr: *mut cpio_file_stat, in_des: libc::c_int);
    fn is_tar_header(buf: *mut libc::c_char) -> libc::c_int;
    fn tape_buffered_peek(
        peek_buf: *mut libc::c_char,
        in_des: libc::c_int,
        num_bytes: libc::c_int,
    ) -> libc::c_int;
    fn getuser(uid: uid_t) -> *mut libc::c_char;
    fn create_all_directories(name: *const libc::c_char);
    fn set_file_times(
        fd: libc::c_int,
        name: *const libc::c_char,
        atime: libc::c_ulong,
        mtime: libc::c_ulong,
    );
    fn cpio_create_dir(
        file_hdr: *mut cpio_file_stat,
        existing_dir: libc::c_int,
    ) -> libc::c_int;
    fn change_dir();
    fn cpio_safer_name_suffix(
        name: *mut libc::c_char,
        link_target: bool,
        absolute_names: bool,
        strip_leading_dots: bool,
    );
    fn apply_delayed_set_stat();
    fn create_deferment(file_hdr: *mut cpio_file_stat) -> *mut deferment;
    fn free_deferment(d: *mut deferment);
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn hash_get_first(table: *const Hash_table) -> *mut libc::c_void;
    fn hash_get_next(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(table: *mut Hash_table);
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
    fn current_timespec() -> timespec;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
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
pub type uint32_t = __uint32_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct old_cpio_header {
    pub c_magic: libc::c_ushort,
    pub c_dev: libc::c_ushort,
    pub c_ino: libc::c_ushort,
    pub c_mode: libc::c_ushort,
    pub c_uid: libc::c_ushort,
    pub c_gid: libc::c_ushort,
    pub c_nlink: libc::c_ushort,
    pub c_rdev: libc::c_ushort,
    pub c_mtimes: [libc::c_ushort; 2],
    pub c_namesize: libc::c_ushort,
    pub c_filesizes: [libc::c_ushort; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct old_ascii_header {
    pub c_magic: [libc::c_char; 6],
    pub c_dev: [libc::c_char; 6],
    pub c_ino: [libc::c_char; 6],
    pub c_mode: [libc::c_char; 6],
    pub c_uid: [libc::c_char; 6],
    pub c_gid: [libc::c_char; 6],
    pub c_nlink: [libc::c_char; 6],
    pub c_rdev: [libc::c_char; 6],
    pub c_mtime: [libc::c_char; 11],
    pub c_namesize: [libc::c_char; 6],
    pub c_filesize: [libc::c_char; 11],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct new_ascii_header {
    pub c_magic: [libc::c_char; 6],
    pub c_ino: [libc::c_char; 8],
    pub c_mode: [libc::c_char; 8],
    pub c_uid: [libc::c_char; 8],
    pub c_gid: [libc::c_char; 8],
    pub c_nlink: [libc::c_char; 8],
    pub c_mtime: [libc::c_char; 8],
    pub c_filesize: [libc::c_char; 8],
    pub c_dev_maj: [libc::c_char; 8],
    pub c_dev_min: [libc::c_char; 8],
    pub c_rdev_maj: [libc::c_char; 8],
    pub c_rdev_min: [libc::c_char; 8],
    pub c_namesize: [libc::c_char; 8],
    pub c_chksum: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio_file_stat {
    pub c_magic: libc::c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: libc::c_uint,
    pub c_dev_min: libc::c_uint,
    pub c_rdev_maj: libc::c_uint,
    pub c_rdev_min: libc::c_uint,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: *mut libc::c_char,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynamic_string {
    pub ds_size: size_t,
    pub ds_idx: size_t,
    pub ds_string: *mut libc::c_char,
}
pub type archive_format = libc::c_uint;
pub const arf_hpbinary: archive_format = 8;
pub const arf_hpoldascii: archive_format = 7;
pub const arf_ustar: archive_format = 6;
pub const arf_tar: archive_format = 5;
pub const arf_crcascii: archive_format = 4;
pub const arf_newascii: archive_format = 3;
pub const arf_oldascii: archive_format = 2;
pub const arf_binary: archive_format = 1;
pub const arf_unknown: archive_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: [libc::c_char; 6],
    pub num: libc::c_ushort,
    pub old_header: old_cpio_header,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: [libc::c_char; 512],
    pub us: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deferment {
    pub next: *mut deferment,
    pub header: cpio_file_stat,
}
pub type Hash_table = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_link {
    pub dev: dev_t,
    pub ino: ino_t,
    pub mode: mode_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub mtime: time_t,
    pub source: *mut libc::c_char,
    pub target: [libc::c_char; 1],
}
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
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn mknod(
    mut __path: *const libc::c_char,
    mut __mode: __mode_t,
    mut __dev: __dev_t,
) -> libc::c_int {
    return __xmknod(0 as libc::c_int, __path, __mode, &mut __dev);
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
unsafe extern "C" fn gnu_dev_makedev(
    mut __major: libc::c_uint,
    mut __minor: libc::c_uint,
) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as libc::c_uint) as __dev_t) << 8 as libc::c_int;
    __dev |= ((__major & 0xfffff000 as libc::c_uint) as __dev_t) << 32 as libc::c_int;
    __dev |= ((__minor & 0xff as libc::c_uint) as __dev_t) << 0 as libc::c_int;
    __dev |= ((__minor & 0xffffff00 as libc::c_uint) as __dev_t) << 12 as libc::c_int;
    return __dev;
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
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as libc::c_char;
        *fresh0 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: libc::c_int) -> libc::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as libc::c_int
        as libc::c_long != 0
    {
        __overflow(stdout, __c as libc::c_uchar as libc::c_int)
    } else {
        let fresh1 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh1 = __c as libc::c_char;
        *fresh1 as libc::c_uchar as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
}
pub unsafe extern "C" fn warn_junk_bytes(mut bytes_skipped: libc::c_long) {
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcngettext(
            0 as *const libc::c_char,
            b"warning: skipped %ld byte of junk\0" as *const u8 as *const libc::c_char,
            b"warning: skipped %ld bytes of junk\0" as *const u8 as *const libc::c_char,
            bytes_skipped as libc::c_ulong,
            5 as libc::c_int,
        ),
        bytes_skipped,
    );
}
unsafe extern "C" fn query_rename(
    mut file_hdr: *mut cpio_file_stat,
    mut tty_in: *mut FILE,
    mut tty_out: *mut FILE,
    mut rename_in: *mut FILE,
) -> libc::c_int {
    let mut str_res: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut new_name: dynamic_string = dynamic_string {
        ds_size: 0,
        ds_idx: 0,
        ds_string: 0 as *const libc::c_char as *mut libc::c_char,
    };
    static mut initialized_new_name: libc::c_int = 0 as libc::c_int;
    if initialized_new_name == 0 {
        ds_init(&mut new_name);
        initialized_new_name = 1 as libc::c_int;
    }
    if rename_flag != 0 {
        fprintf(
            tty_out,
            dcgettext(
                0 as *const libc::c_char,
                b"rename %s -> \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file_hdr).c_name,
        );
        fflush_unlocked(tty_out);
        str_res = ds_fgets(tty_in, &mut new_name);
    } else {
        str_res = ds_fgetstr(rename_in, &mut new_name, '\n' as i32 as libc::c_char);
    }
    if str_res.is_null()
        || *str_res.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        return -(1 as libc::c_int)
    } else {
        cpio_set_c_name(file_hdr, new_name.ds_string);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tape_skip_padding(mut in_file_des: libc::c_int, mut offset: off_t) {
    let mut pad: off_t = 0;
    if archive_format as libc::c_uint == arf_crcascii as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_newascii as libc::c_int as libc::c_uint
    {
        pad = (4 as libc::c_int as libc::c_long
            - offset % 4 as libc::c_int as libc::c_long)
            % 4 as libc::c_int as libc::c_long;
    } else if archive_format as libc::c_uint == arf_binary as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_hpbinary as libc::c_int as libc::c_uint
    {
        pad = (2 as libc::c_int as libc::c_long
            - offset % 2 as libc::c_int as libc::c_long)
            % 2 as libc::c_int as libc::c_long;
    } else if archive_format as libc::c_uint == arf_tar as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint
    {
        pad = (512 as libc::c_int as libc::c_long
            - offset % 512 as libc::c_int as libc::c_long)
            % 512 as libc::c_int as libc::c_long;
    } else {
        pad = 0 as libc::c_int as off_t;
    }
    if pad != 0 as libc::c_int as libc::c_long {
        tape_toss_input(in_file_des, pad);
    }
}
unsafe extern "C" fn get_link_name(
    mut file_hdr: *mut cpio_file_stat,
    mut in_file_des: libc::c_int,
) -> *mut libc::c_char {
    let mut link_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*file_hdr).c_filesize < 0 as libc::c_int as libc::c_long
        || (*file_hdr).c_filesize as libc::c_ulong
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: stored filename length is out of range\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file_hdr).c_name,
        );
        link_name = 0 as *mut libc::c_char;
    } else {
        link_name = xmalloc(
            ((*file_hdr).c_filesize + 1 as libc::c_int as libc::c_long) as size_t,
        ) as *mut libc::c_char;
        tape_buffered_read(link_name, in_file_des, (*file_hdr).c_filesize);
        *link_name.offset((*file_hdr).c_filesize as isize) = '\0' as i32 as libc::c_char;
        tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
    }
    return link_name;
}
unsafe extern "C" fn list_file(
    mut file_hdr: *mut cpio_file_stat,
    mut in_file_des: libc::c_int,
) {
    if verbose_flag != 0 {
        if (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            if archive_format as libc::c_uint != arf_tar as libc::c_int as libc::c_uint
                && archive_format as libc::c_uint
                    != arf_ustar as libc::c_int as libc::c_uint
            {
                let mut link_name: *mut libc::c_char = get_link_name(
                    file_hdr,
                    in_file_des,
                );
                if !link_name.is_null() {
                    long_format(file_hdr, link_name);
                    rpl_free(link_name as *mut libc::c_void);
                }
            } else {
                long_format(file_hdr, (*file_hdr).c_tar_linkname);
            }
            return;
        } else {
            long_format(file_hdr, 0 as *mut libc::c_char);
        }
    } else {
        printf(
            b"%s%c\0" as *const u8 as *const libc::c_char,
            (*file_hdr).c_name,
            name_end as libc::c_int,
        );
    }
    crc = 0 as libc::c_int as uint32_t;
    tape_toss_input(in_file_des, (*file_hdr).c_filesize);
    tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
    if only_verify_crc_flag != 0 {
        if (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        {
            return;
        }
        if crc != (*file_hdr).c_chksum {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: checksum error (0x%x, should be 0x%x)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_name,
                crc,
                (*file_hdr).c_chksum,
            );
        }
    }
}
unsafe extern "C" fn try_existing_file(
    mut file_hdr: *mut cpio_file_stat,
    mut in_file_des: libc::c_int,
    mut existing_dir: *mut bool,
) -> libc::c_int {
    let mut file_stat: stat = stat {
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
    *existing_dir = 0 as libc::c_int != 0;
    if lstat((*file_hdr).c_name, &mut file_stat) == 0 as libc::c_int {
        if file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
            && (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            *existing_dir = 1 as libc::c_int != 0;
            return 0 as libc::c_int;
        } else if unconditional_flag == 0
            && (*file_hdr).c_mtime <= file_stat.st_mtim.tv_sec
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s not created: newer or same age version exists\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_name,
            );
            tape_toss_input(in_file_des, (*file_hdr).c_filesize);
            tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
            return -(1 as libc::c_int);
        } else if if file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            rmdir((*file_hdr).c_name)
        } else {
            unlink((*file_hdr).c_name)
        } != 0
        {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot remove current %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_name,
            );
            tape_toss_input(in_file_des, (*file_hdr).c_filesize);
            tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub static mut deferments: *mut deferment = 0 as *const deferment as *mut deferment;
unsafe extern "C" fn defer_copyin(mut file_hdr: *mut cpio_file_stat) {
    let mut d: *mut deferment = 0 as *mut deferment;
    d = create_deferment(file_hdr);
    (*d).next = deferments;
    deferments = d;
}
unsafe extern "C" fn create_defered_links(mut file_hdr: *mut cpio_file_stat) {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut d_prev: *mut deferment = 0 as *mut deferment;
    let mut ino: ino_t = 0;
    let mut maj: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut link_res: libc::c_int = 0;
    ino = (*file_hdr).c_ino;
    maj = (*file_hdr).c_dev_maj as libc::c_int;
    min = (*file_hdr).c_dev_min as libc::c_int;
    d = deferments;
    d_prev = 0 as *mut deferment;
    while !d.is_null() {
        if (*d).header.c_ino == ino && (*d).header.c_dev_maj == maj as libc::c_uint
            && (*d).header.c_dev_min == min as libc::c_uint
        {
            let mut d_free: *mut deferment = 0 as *mut deferment;
            link_res = link_to_name((*d).header.c_name, (*file_hdr).c_name);
            if link_res < 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot link %s to %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*d).header.c_name,
                    (*file_hdr).c_name,
                );
            }
            if !d_prev.is_null() {
                (*d_prev).next = (*d).next;
            } else {
                deferments = (*d).next;
            }
            d_free = d;
            d = (*d).next;
            free_deferment(d_free);
        } else {
            d_prev = d;
            d = (*d).next;
        }
    }
}
unsafe extern "C" fn create_defered_links_to_skipped(
    mut file_hdr: *mut cpio_file_stat,
    mut in_file_des: libc::c_int,
) -> libc::c_int {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut d_prev: *mut deferment = 0 as *mut deferment;
    let mut ino: ino_t = 0;
    let mut maj: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    if (*file_hdr).c_filesize == 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    ino = (*file_hdr).c_ino;
    maj = (*file_hdr).c_dev_maj as libc::c_int;
    min = (*file_hdr).c_dev_min as libc::c_int;
    d = deferments;
    d_prev = 0 as *mut deferment;
    while !d.is_null() {
        if (*d).header.c_ino == ino && (*d).header.c_dev_maj == maj as libc::c_uint
            && (*d).header.c_dev_min == min as libc::c_uint
        {
            if !d_prev.is_null() {
                (*d_prev).next = (*d).next;
            } else {
                deferments = (*d).next;
            }
            cpio_set_c_name(file_hdr, (*d).header.c_name);
            free_deferment(d);
            copyin_regular_file(file_hdr, in_file_des);
            return 0 as libc::c_int;
        } else {
            d_prev = d;
            d = (*d).next;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn create_final_defers() {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut link_res: libc::c_int = 0;
    let mut out_file_des: libc::c_int = 0;
    d = deferments;
    while !d.is_null() {
        link_res = link_to_maj_min_ino(
            (*d).header.c_name,
            (*d).header.c_dev_maj as libc::c_int,
            (*d).header.c_dev_min as libc::c_int,
            (*d).header.c_ino,
        );
        if !(link_res == 0 as libc::c_int) {
            out_file_des = open(
                (*d).header.c_name,
                0o100 as libc::c_int | 0o1 as libc::c_int | 0 as libc::c_int,
                0o600 as libc::c_int,
            );
            if out_file_des < 0 as libc::c_int && create_dir_flag != 0 {
                create_all_directories((*d).header.c_name);
                out_file_des = open(
                    (*d).header.c_name,
                    0o100 as libc::c_int | 0o1 as libc::c_int | 0 as libc::c_int,
                    0o600 as libc::c_int,
                );
            }
            if out_file_des < 0 as libc::c_int {
                open_error((*d).header.c_name);
            } else {
                set_perms(out_file_des, &mut (*d).header);
                if close(out_file_des) < 0 as libc::c_int {
                    close_error((*d).header.c_name);
                }
            }
        }
        d = (*d).next;
    }
}
unsafe extern "C" fn copyin_regular_file(
    mut file_hdr: *mut cpio_file_stat,
    mut in_file_des: libc::c_int,
) {
    let mut out_file_des: libc::c_int = 0;
    if to_stdout_option {
        out_file_des = 1 as libc::c_int;
    } else {
        if (*file_hdr).c_nlink > 1 as libc::c_int as libc::c_ulong
            && (archive_format as libc::c_uint
                == arf_newascii as libc::c_int as libc::c_uint
                || archive_format as libc::c_uint
                    == arf_crcascii as libc::c_int as libc::c_uint)
        {
            let mut link_res: libc::c_int = 0;
            if (*file_hdr).c_filesize == 0 as libc::c_int as libc::c_long {
                defer_copyin(file_hdr);
                tape_toss_input(in_file_des, (*file_hdr).c_filesize);
                tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
                return;
            }
            link_res = link_to_maj_min_ino(
                (*file_hdr).c_name,
                (*file_hdr).c_dev_maj as libc::c_int,
                (*file_hdr).c_dev_min as libc::c_int,
                (*file_hdr).c_ino,
            );
            if link_res == 0 as libc::c_int {
                tape_toss_input(in_file_des, (*file_hdr).c_filesize);
                tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
                return;
            }
        } else if (*file_hdr).c_nlink > 1 as libc::c_int as libc::c_ulong
            && archive_format as libc::c_uint != arf_tar as libc::c_int as libc::c_uint
            && archive_format as libc::c_uint != arf_ustar as libc::c_int as libc::c_uint
        {
            let mut link_res_0: libc::c_int = 0;
            link_res_0 = link_to_maj_min_ino(
                (*file_hdr).c_name,
                (*file_hdr).c_dev_maj as libc::c_int,
                (*file_hdr).c_dev_min as libc::c_int,
                (*file_hdr).c_ino,
            );
            if link_res_0 == 0 as libc::c_int {
                tape_toss_input(in_file_des, (*file_hdr).c_filesize);
                tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
                return;
            }
        } else if (archive_format as libc::c_uint
            == arf_tar as libc::c_int as libc::c_uint
            || archive_format as libc::c_uint
                == arf_ustar as libc::c_int as libc::c_uint)
            && !((*file_hdr).c_tar_linkname).is_null()
            && *((*file_hdr).c_tar_linkname).offset(0 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
        {
            let mut link_res_1: libc::c_int = 0;
            link_res_1 = link_to_name((*file_hdr).c_name, (*file_hdr).c_tar_linkname);
            if link_res_1 < 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"cannot link %s to %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file_hdr).c_tar_linkname,
                    (*file_hdr).c_name,
                );
            }
            return;
        }
        out_file_des = open(
            (*file_hdr).c_name,
            0o100 as libc::c_int | 0o1 as libc::c_int | 0 as libc::c_int,
            0o600 as libc::c_int,
        );
        if out_file_des < 0 as libc::c_int && create_dir_flag != 0 {
            create_all_directories((*file_hdr).c_name);
            out_file_des = open(
                (*file_hdr).c_name,
                0o100 as libc::c_int | 0o1 as libc::c_int | 0 as libc::c_int,
                0o600 as libc::c_int,
            );
        }
        if out_file_des < 0 as libc::c_int {
            open_error((*file_hdr).c_name);
            tape_toss_input(in_file_des, (*file_hdr).c_filesize);
            tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
            return;
        }
    }
    crc = 0 as libc::c_int as uint32_t;
    if swap_halfwords_flag != 0 {
        if (*file_hdr).c_filesize % 4 as libc::c_int as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
            swapping_halfwords = 1 as libc::c_int;
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot swap halfwords of %s: odd number of halfwords\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_name,
            );
        }
    }
    if swap_bytes_flag != 0 {
        if (*file_hdr).c_filesize % 2 as libc::c_int as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
            swapping_bytes = 1 as libc::c_int;
        } else {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot swap bytes of %s: odd number of bytes\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_name,
            );
        }
    }
    copy_files_tape_to_disk(in_file_des, out_file_des, (*file_hdr).c_filesize);
    disk_empty_output_buffer(out_file_des, 1 as libc::c_int != 0);
    if to_stdout_option {
        if archive_format as libc::c_uint == arf_crcascii as libc::c_int as libc::c_uint
        {
            if crc != (*file_hdr).c_chksum {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: checksum error (0x%x, should be 0x%x)\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file_hdr).c_name,
                    crc,
                    (*file_hdr).c_chksum,
                );
            }
        }
        tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
        return;
    }
    set_perms(out_file_des, file_hdr);
    if close(out_file_des) < 0 as libc::c_int {
        close_error((*file_hdr).c_name);
    }
    if archive_format as libc::c_uint == arf_crcascii as libc::c_int as libc::c_uint {
        if crc != (*file_hdr).c_chksum {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: checksum error (0x%x, should be 0x%x)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_name,
                crc,
                (*file_hdr).c_chksum,
            );
        }
    }
    tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
    if (*file_hdr).c_nlink > 1 as libc::c_int as libc::c_ulong
        && (archive_format as libc::c_uint == arf_newascii as libc::c_int as libc::c_uint
            || archive_format as libc::c_uint
                == arf_crcascii as libc::c_int as libc::c_uint)
    {
        create_defered_links(file_hdr);
    }
}
unsafe extern "C" fn copyin_device(mut file_hdr: *mut cpio_file_stat) {
    let mut res: libc::c_int = 0;
    if to_stdout_option {
        return;
    }
    if (*file_hdr).c_nlink > 1 as libc::c_int as libc::c_ulong
        && archive_format as libc::c_uint != arf_tar as libc::c_int as libc::c_uint
        && archive_format as libc::c_uint != arf_ustar as libc::c_int as libc::c_uint
    {
        let mut link_res: libc::c_int = 0;
        link_res = link_to_maj_min_ino(
            (*file_hdr).c_name,
            (*file_hdr).c_dev_maj as libc::c_int,
            (*file_hdr).c_dev_min as libc::c_int,
            (*file_hdr).c_ino,
        );
        if link_res == 0 as libc::c_int {
            return;
        }
    } else if archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint
        && !((*file_hdr).c_tar_linkname).is_null()
        && *((*file_hdr).c_tar_linkname).offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
    {
        let mut link_res_0: libc::c_int = 0;
        link_res_0 = link_to_name((*file_hdr).c_name, (*file_hdr).c_tar_linkname);
        if link_res_0 < 0 as libc::c_int {
            error(
                0 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot link %s to %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_tar_linkname,
                (*file_hdr).c_name,
            );
        }
        return;
    }
    res = mknod(
        (*file_hdr).c_name,
        (*file_hdr).c_mode,
        gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min),
    );
    if res < 0 as libc::c_int && create_dir_flag != 0 {
        create_all_directories((*file_hdr).c_name);
        res = mknod(
            (*file_hdr).c_name,
            (*file_hdr).c_mode,
            gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min),
        );
    }
    if res < 0 as libc::c_int {
        mknod_error((*file_hdr).c_name);
        return;
    }
    if no_chown_flag == 0 {
        let mut uid: uid_t = if set_owner_flag != 0 {
            set_owner
        } else {
            (*file_hdr).c_uid
        };
        let mut gid: gid_t = if set_group_flag != 0 {
            set_group
        } else {
            (*file_hdr).c_gid
        };
        if chown((*file_hdr).c_name, uid, gid) < 0 as libc::c_int
            && *__errno_location() != 1 as libc::c_int
        {
            chown_error_details((*file_hdr).c_name, uid, gid);
        }
    }
    if chmod((*file_hdr).c_name, (*file_hdr).c_mode) < 0 as libc::c_int {
        chmod_error_details((*file_hdr).c_name, (*file_hdr).c_mode);
    }
    if retain_time_flag != 0 {
        set_file_times(
            -(1 as libc::c_int),
            (*file_hdr).c_name,
            (*file_hdr).c_mtime as libc::c_ulong,
            (*file_hdr).c_mtime as libc::c_ulong,
        );
    }
}
static mut delayed_link_table: *mut Hash_table = 0 as *const Hash_table
    as *mut Hash_table;
unsafe extern "C" fn dl_hash(
    mut entry: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut dl: *const delayed_link = entry as *const delayed_link;
    let mut n: uintmax_t = (*dl).dev;
    let mut nshift: libc::c_int = (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<dev_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
    if (0 as libc::c_int) < nshift {
        n <<= nshift;
    }
    n ^= (*dl).ino;
    return n.wrapping_rem(table_size);
}
unsafe extern "C" fn dl_compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> bool {
    let mut da: *const delayed_link = a as *const delayed_link;
    let mut db: *const delayed_link = b as *const delayed_link;
    return ((*da).dev == (*db).dev) as libc::c_int
        & ((*da).ino == (*db).ino) as libc::c_int != 0;
}
unsafe extern "C" fn symlink_placeholder(
    mut oldpath: *mut libc::c_char,
    mut newpath: *mut libc::c_char,
    mut file_stat: *mut cpio_file_stat,
) -> libc::c_int {
    let mut fd: libc::c_int = open(
        newpath,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
        0 as libc::c_int,
    );
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
    let mut p: *mut delayed_link = 0 as *mut delayed_link;
    let mut newlen: size_t = strlen(newpath);
    if fd < 0 as libc::c_int {
        open_error(newpath);
        return -(1 as libc::c_int);
    }
    if fstat(fd, &mut st) != 0 as libc::c_int {
        stat_error(newpath);
        close(fd);
        return -(1 as libc::c_int);
    }
    close(fd);
    p = xmalloc(
        (::std::mem::size_of::<delayed_link>() as libc::c_ulong)
            .wrapping_add(strlen(oldpath))
            .wrapping_add(newlen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut delayed_link;
    (*p).dev = st.st_dev;
    (*p).ino = st.st_ino;
    (*p).mode = (*file_stat).c_mode;
    (*p).uid = (*file_stat).c_uid;
    (*p).gid = (*file_stat).c_gid;
    (*p).mtime = (*file_stat).c_mtime;
    strcpy(((*p).target).as_mut_ptr(), newpath);
    (*p)
        .source = ((*p).target)
        .as_mut_ptr()
        .offset(newlen as isize)
        .offset(1 as libc::c_int as isize);
    strcpy((*p).source, oldpath);
    if !((!delayed_link_table.is_null()
        || {
            delayed_link_table = hash_initialize(
                0 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    dl_hash
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    dl_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            !delayed_link_table.is_null()
        }) && !(hash_insert(delayed_link_table, p as *const libc::c_void)).is_null())
    {
        xalloc_die();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn replace_symlink_placeholders() {
    let mut dl: *mut delayed_link = 0 as *mut delayed_link;
    if delayed_link_table.is_null() {
        return;
    }
    dl = hash_get_first(delayed_link_table) as *mut delayed_link;
    while !dl.is_null() {
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
        if lstat(((*dl).target).as_mut_ptr(), &mut st) == 0 as libc::c_int
            && st.st_dev == (*dl).dev && st.st_ino == (*dl).ino
        {
            if unlink(((*dl).target).as_mut_ptr()) != 0 {
                unlink_error(((*dl).target).as_mut_ptr());
            } else {
                let mut res: libc::c_int = symlink(
                    (*dl).source,
                    ((*dl).target).as_mut_ptr(),
                );
                if res < 0 as libc::c_int && create_dir_flag != 0 {
                    create_all_directories(((*dl).target).as_mut_ptr());
                    res = symlink((*dl).source, ((*dl).target).as_mut_ptr());
                }
                if res < 0 as libc::c_int {
                    symlink_error((*dl).source, ((*dl).target).as_mut_ptr());
                } else if no_chown_flag == 0 {
                    let mut uid: uid_t = if set_owner_flag != 0 {
                        set_owner
                    } else {
                        (*dl).uid
                    };
                    let mut gid: gid_t = if set_group_flag != 0 {
                        set_group
                    } else {
                        (*dl).gid
                    };
                    if lchown(((*dl).target).as_mut_ptr(), uid, gid) < 0 as libc::c_int
                        && *__errno_location() != 1 as libc::c_int
                    {
                        chown_error_details(((*dl).target).as_mut_ptr(), uid, gid);
                    }
                }
            }
        }
        dl = hash_get_next(delayed_link_table, dl as *const libc::c_void)
            as *mut delayed_link;
    }
    hash_free(delayed_link_table);
    delayed_link_table = 0 as *mut Hash_table;
}
unsafe extern "C" fn copyin_link(
    mut file_hdr: *mut cpio_file_stat,
    mut in_file_des: libc::c_int,
) {
    let mut link_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: libc::c_int = 0;
    if archive_format as libc::c_uint != arf_tar as libc::c_int as libc::c_uint
        && archive_format as libc::c_uint != arf_ustar as libc::c_int as libc::c_uint
    {
        if to_stdout_option {
            tape_toss_input(in_file_des, (*file_hdr).c_filesize);
            tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
            return;
        }
        link_name = get_link_name(file_hdr, in_file_des);
        if link_name.is_null() {
            return;
        }
    } else {
        if to_stdout_option {
            return;
        }
        link_name = xstrdup((*file_hdr).c_tar_linkname);
    }
    if no_abs_paths_flag != 0 {
        symlink_placeholder(link_name, (*file_hdr).c_name, file_hdr);
    } else {
        res = symlink(link_name, (*file_hdr).c_name);
        if res < 0 as libc::c_int && create_dir_flag != 0 {
            create_all_directories((*file_hdr).c_name);
            res = symlink(link_name, (*file_hdr).c_name);
        }
        if res < 0 as libc::c_int {
            symlink_error(link_name, (*file_hdr).c_name);
        } else if no_chown_flag == 0 {
            let mut uid: uid_t = if set_owner_flag != 0 {
                set_owner
            } else {
                (*file_hdr).c_uid
            };
            let mut gid: gid_t = if set_group_flag != 0 {
                set_group
            } else {
                (*file_hdr).c_gid
            };
            if lchown((*file_hdr).c_name, uid, gid) < 0 as libc::c_int
                && *__errno_location() != 1 as libc::c_int
            {
                chown_error_details((*file_hdr).c_name, uid, gid);
            }
        }
    }
    rpl_free(link_name as *mut libc::c_void);
}
unsafe extern "C" fn copyin_file(
    mut file_hdr: *mut cpio_file_stat,
    mut in_file_des: libc::c_int,
) {
    let mut existing_dir: bool = 0 as libc::c_int != 0;
    if !to_stdout_option
        && try_existing_file(file_hdr, in_file_des, &mut existing_dir) < 0 as libc::c_int
    {
        return;
    }
    match (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint {
        32768 => {
            copyin_regular_file(file_hdr, in_file_des);
        }
        16384 => {
            cpio_create_dir(file_hdr, existing_dir as libc::c_int);
        }
        8192 | 24576 | 49152 | 4096 => {
            copyin_device(file_hdr);
        }
        40960 => {
            copyin_link(file_hdr, in_file_des);
        }
        _ => {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: unknown file type\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file_hdr).c_name,
            );
            tape_toss_input(in_file_des, (*file_hdr).c_filesize);
            tape_skip_padding(in_file_des, (*file_hdr).c_filesize);
        }
    };
}
static mut current_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
pub unsafe extern "C" fn long_format(
    mut file_hdr: *mut cpio_file_stat,
    mut link_name: *const libc::c_char,
) {
    let mut mbuf: [libc::c_char; 11] = [0; 11];
    let mut when: time_t = 0;
    let mut tbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut when_timespec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut six_months_ago: timespec = {
        let mut init = timespec {
            tv_sec: current_time.tv_sec
                - (31556952 as libc::c_int / 2 as libc::c_int) as libc::c_long,
            tv_nsec: current_time.tv_nsec,
        };
        init
    };
    mode_string((*file_hdr).c_mode, mbuf.as_mut_ptr());
    mbuf[10 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    printf(
        b"%s %3ju \0" as *const u8 as *const libc::c_char,
        mbuf.as_mut_ptr(),
        (*file_hdr).c_nlink,
    );
    if numeric_uid != 0 {
        printf(
            b"%-8ju %-8ju \0" as *const u8 as *const libc::c_char,
            (*file_hdr).c_uid as uintmax_t,
            (*file_hdr).c_gid as uintmax_t,
        );
    } else {
        printf(
            b"%-8.8s %-8.8s \0" as *const u8 as *const libc::c_char,
            getuser((*file_hdr).c_uid),
            getgroup((*file_hdr).c_gid),
        );
    }
    if (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
        || (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint
    {
        printf(
            b"%3ju, %3ju \0" as *const u8 as *const libc::c_char,
            (*file_hdr).c_rdev_maj as uintmax_t,
            (*file_hdr).c_rdev_min as uintmax_t,
        );
    } else {
        printf(
            b"%8ju \0" as *const u8 as *const libc::c_char,
            (*file_hdr).c_filesize as uintmax_t,
        );
    }
    when = (*file_hdr).c_mtime;
    when_timespec.tv_sec = when;
    when_timespec.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    tbuf = ctime(&mut when);
    if timespec_cmp(current_time, when_timespec) < 0 as libc::c_int {
        current_time = current_timespec();
    }
    if !(timespec_cmp(six_months_ago, when_timespec) < 0 as libc::c_int
        && timespec_cmp(when_timespec, current_time) < 0 as libc::c_int)
    {
        memcpy(
            tbuf.offset(11 as libc::c_int as isize) as *mut libc::c_void,
            tbuf.offset(19 as libc::c_int as isize) as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    *tbuf.offset(16 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
    *tbuf.offset(17 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        tbuf.offset(4 as libc::c_int as isize),
    );
    printf(b"%s\0" as *const u8 as *const libc::c_char, quotearg((*file_hdr).c_name));
    if !link_name.is_null() {
        printf(b" -> \0" as *const u8 as *const libc::c_char);
        printf(b"%s\0" as *const u8 as *const libc::c_char, quotearg(link_name));
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn read_pattern_file() {
    let mut new_save_patterns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut max_new_patterns: size_t = 0;
    let mut new_num_patterns: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut pattern_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as libc::c_int as size_t,
            ds_idx: 0 as libc::c_int as size_t,
            ds_string: 0 as *mut libc::c_char,
        };
        init
    };
    let mut pattern_fp: *mut FILE = 0 as *mut FILE;
    if num_patterns < 0 as libc::c_int {
        num_patterns = 0 as libc::c_int;
    }
    new_num_patterns = num_patterns as size_t;
    max_new_patterns = num_patterns as size_t;
    new_save_patterns = xcalloc(
        max_new_patterns,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    pattern_fp = fopen(pattern_file_name, b"r\0" as *const u8 as *const libc::c_char);
    if pattern_fp.is_null() {
        open_fatal(pattern_file_name);
    }
    while !(ds_fgetstr(pattern_fp, &mut pattern_name, '\n' as i32 as libc::c_char))
        .is_null()
    {
        if new_num_patterns == max_new_patterns {
            new_save_patterns = x2nrealloc(
                new_save_patterns as *mut libc::c_void,
                &mut max_new_patterns,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
        }
        let ref mut fresh2 = *new_save_patterns.offset(new_num_patterns as isize);
        *fresh2 = xstrdup(pattern_name.ds_string);
        new_num_patterns = new_num_patterns.wrapping_add(1);
        new_num_patterns;
    }
    ds_free(&mut pattern_name);
    if ferror_unlocked(pattern_fp) != 0 || fclose(pattern_fp) == -(1 as libc::c_int) {
        close_error(pattern_file_name);
    }
    i = 0 as libc::c_int;
    while i < num_patterns {
        let ref mut fresh3 = *new_save_patterns.offset(i as isize);
        *fresh3 = *save_patterns.offset(i as isize);
        i += 1;
        i;
    }
    save_patterns = new_save_patterns;
    num_patterns = new_num_patterns as libc::c_int;
}
pub unsafe extern "C" fn from_ascii(
    mut where_0: *const libc::c_char,
    mut digs: size_t,
    mut logbase: libc::c_uint,
) -> uintmax_t {
    let mut value: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut buf: *const libc::c_char = where_0;
    let mut end: *const libc::c_char = buf.offset(digs as isize);
    let mut overflow: libc::c_int = 0 as libc::c_int;
    static mut codetab: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<
            &[u8; 17],
            &mut [libc::c_char; 17],
        >(b"0123456789ABCDEF\0")
    };
    while *buf as libc::c_int == ' ' as i32 {
        if buf == end {
            return 0 as libc::c_int as uintmax_t;
        }
        buf = buf.offset(1);
        buf;
    }
    if buf == end || *buf as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int as uintmax_t;
    }
    loop {
        let mut d: libc::c_uint = 0;
        let mut p: *mut libc::c_char = strchr(
            codetab.as_mut_ptr(),
            {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *buf as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(*buf as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*buf as libc::c_int as isize);
                }
                __res
            },
        );
        if p.is_null() {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed number %.*s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                digs as libc::c_int,
                where_0,
            );
            break;
        } else {
            d = p.offset_from(codetab.as_mut_ptr()) as libc::c_long as libc::c_uint;
            if d >> logbase > 1 as libc::c_int as libc::c_uint {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Malformed number %.*s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    digs as libc::c_int,
                    where_0,
                );
                break;
            } else {
                value = (value as libc::c_ulong).wrapping_add(d as libc::c_ulong)
                    as uintmax_t as uintmax_t;
                buf = buf.offset(1);
                if buf == end || *buf as libc::c_int == 0 as libc::c_int {
                    break;
                }
                overflow = (overflow as libc::c_ulong
                    | value ^ value << logbase >> logbase) as libc::c_int;
                value <<= logbase;
            }
        }
    }
    if overflow != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Archive value %.*s is out of range\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            digs as libc::c_int,
            where_0,
        );
    }
    return value;
}
pub unsafe extern "C" fn read_in_header(
    mut file_hdr: *mut cpio_file_stat,
    mut in_des: libc::c_int,
) {
    let mut magic: C2RustUnnamed = C2RustUnnamed { str_0: [0; 6] };
    let mut bytes_skipped: libc::c_long = 0 as libc::c_int as libc::c_long;
    if archive_format as libc::c_uint == arf_unknown as libc::c_int as libc::c_uint {
        let mut tmpbuf: C2RustUnnamed_0 = C2RustUnnamed_0 { s: [0; 512] };
        let mut check_tar: libc::c_int = 0;
        let mut peeked_bytes: libc::c_int = 0;
        while archive_format as libc::c_uint
            == arf_unknown as libc::c_int as libc::c_uint
        {
            peeked_bytes = tape_buffered_peek(
                (tmpbuf.s).as_mut_ptr(),
                in_des,
                512 as libc::c_int,
            );
            if peeked_bytes < 6 as libc::c_int {
                error(
                    2 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"premature end of archive\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
            if strncmp(
                (tmpbuf.s).as_mut_ptr(),
                b"070701\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                archive_format = arf_newascii;
            } else if strncmp(
                (tmpbuf.s).as_mut_ptr(),
                b"070707\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                archive_format = arf_oldascii;
            } else if strncmp(
                (tmpbuf.s).as_mut_ptr(),
                b"070702\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                archive_format = arf_crcascii;
                crc_i_flag = 1 as libc::c_int;
            } else if tmpbuf.us as libc::c_int == 0o70707 as libc::c_int
                || tmpbuf.us as libc::c_int
                    == (0o70707 as libc::c_int as libc::c_ushort as libc::c_int)
                        << 8 as libc::c_int & 0xff00 as libc::c_int
                        | 0o70707 as libc::c_int as libc::c_ushort as libc::c_int
                            >> 8 as libc::c_int & 0xff as libc::c_int
            {
                archive_format = arf_binary;
            } else if peeked_bytes >= 512 as libc::c_int
                && {
                    check_tar = is_tar_header((tmpbuf.s).as_mut_ptr());
                    check_tar != 0
                }
            {
                if check_tar == 2 as libc::c_int {
                    archive_format = arf_ustar;
                } else {
                    archive_format = arf_tar;
                }
            } else {
                tape_buffered_read((tmpbuf.s).as_mut_ptr(), in_des, 1 as libc::c_long);
                bytes_skipped += 1;
                bytes_skipped;
            }
        }
    }
    if archive_format as libc::c_uint == arf_tar as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint
    {
        if append_flag != 0 {
            last_header_start = input_bytes - io_block_size as libc::c_long
                + in_buff.offset_from(input_buffer) as libc::c_long;
        }
        if bytes_skipped > 0 as libc::c_int as libc::c_long {
            warn_junk_bytes(bytes_skipped);
        }
        read_in_tar_header(file_hdr, in_des);
        return;
    }
    (*file_hdr).c_tar_linkname = 0 as *const libc::c_char;
    tape_buffered_read(
        (magic.str_0).as_mut_ptr(),
        in_des,
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as off_t,
    );
    loop {
        if append_flag != 0 {
            last_header_start = input_bytes - io_block_size as libc::c_long
                + in_buff.offset_from(input_buffer) as libc::c_long
                - 6 as libc::c_int as libc::c_long;
        }
        if archive_format as libc::c_uint == arf_newascii as libc::c_int as libc::c_uint
            && strncmp(
                (magic.str_0).as_mut_ptr(),
                b"070701\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            if bytes_skipped > 0 as libc::c_int as libc::c_long {
                warn_junk_bytes(bytes_skipped);
            }
            (*file_hdr).c_magic = 0o70701 as libc::c_int as libc::c_ushort;
            read_in_new_ascii(file_hdr, in_des);
            break;
        } else if archive_format as libc::c_uint
            == arf_crcascii as libc::c_int as libc::c_uint
            && strncmp(
                (magic.str_0).as_mut_ptr(),
                b"070702\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            if bytes_skipped > 0 as libc::c_int as libc::c_long {
                warn_junk_bytes(bytes_skipped);
            }
            (*file_hdr).c_magic = 0o70702 as libc::c_int as libc::c_ushort;
            read_in_new_ascii(file_hdr, in_des);
            break;
        } else if (archive_format as libc::c_uint
            == arf_oldascii as libc::c_int as libc::c_uint
            || archive_format as libc::c_uint
                == arf_hpoldascii as libc::c_int as libc::c_uint)
            && strncmp(
                (magic.str_0).as_mut_ptr(),
                b"070707\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            if bytes_skipped > 0 as libc::c_int as libc::c_long {
                warn_junk_bytes(bytes_skipped);
            }
            (*file_hdr).c_magic = 0o70707 as libc::c_int as libc::c_ushort;
            read_in_old_ascii(file_hdr, in_des);
            break;
        } else if (archive_format as libc::c_uint
            == arf_binary as libc::c_int as libc::c_uint
            || archive_format as libc::c_uint
                == arf_hpbinary as libc::c_int as libc::c_uint)
            && (magic.num as libc::c_int == 0o70707 as libc::c_int
                || magic.num as libc::c_int
                    == (0o70707 as libc::c_int as libc::c_ushort as libc::c_int)
                        << 8 as libc::c_int & 0xff00 as libc::c_int
                        | 0o70707 as libc::c_int as libc::c_ushort as libc::c_int
                            >> 8 as libc::c_int & 0xff as libc::c_int)
        {
            if bytes_skipped > 0 as libc::c_int as libc::c_long {
                warn_junk_bytes(bytes_skipped);
            }
            (*file_hdr).c_magic = 0o70707 as libc::c_int as libc::c_ushort;
            read_in_binary(file_hdr, &mut magic.old_header, in_des);
            break;
        } else {
            bytes_skipped += 1;
            bytes_skipped;
            memmove(
                (magic.str_0).as_mut_ptr() as *mut libc::c_void,
                (magic.str_0).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            tape_buffered_read(
                (magic.str_0)
                    .as_mut_ptr()
                    .offset(
                        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(1 as libc::c_int as isize)),
                in_des,
                1 as libc::c_long,
            );
        }
    };
}
unsafe extern "C" fn read_name_from_file(
    mut file_hdr: *mut cpio_file_stat,
    mut fd: libc::c_int,
    mut len: uintmax_t,
) {
    if len == 0 as libc::c_int as libc::c_ulong {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"malformed header: file name of zero length\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        cpio_realloc_c_name(file_hdr, len);
        tape_buffered_read((*file_hdr).c_name, fd, len as off_t);
        if *((*file_hdr).c_name)
            .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"malformed header: file name is not nul-terminated\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            len = 0 as libc::c_int as uintmax_t;
        }
    }
    (*file_hdr).c_namesize = len;
}
pub unsafe extern "C" fn read_in_old_ascii(
    mut file_hdr: *mut cpio_file_stat,
    mut in_des: libc::c_int,
) {
    let mut ascii_header: old_ascii_header = old_ascii_header {
        c_magic: [0; 6],
        c_dev: [0; 6],
        c_ino: [0; 6],
        c_mode: [0; 6],
        c_uid: [0; 6],
        c_gid: [0; 6],
        c_nlink: [0; 6],
        c_rdev: [0; 6],
        c_mtime: [0; 11],
        c_namesize: [0; 6],
        c_filesize: [0; 11],
    };
    let mut dev: libc::c_ulong = 0;
    tape_buffered_read(
        (ascii_header.c_dev).as_mut_ptr(),
        in_des,
        (::std::mem::size_of::<old_ascii_header>() as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            as off_t,
    );
    dev = from_ascii(
        (ascii_header.c_dev).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    );
    (*file_hdr).c_dev_maj = gnu_dev_major(dev);
    (*file_hdr).c_dev_min = gnu_dev_minor(dev);
    (*file_hdr)
        .c_ino = from_ascii(
        (ascii_header.c_ino).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    );
    (*file_hdr)
        .c_mode = from_ascii(
        (ascii_header.c_mode).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    ) as mode_t;
    (*file_hdr)
        .c_uid = from_ascii(
        (ascii_header.c_uid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    ) as uid_t;
    (*file_hdr)
        .c_gid = from_ascii(
        (ascii_header.c_gid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    ) as gid_t;
    (*file_hdr)
        .c_nlink = from_ascii(
        (ascii_header.c_nlink).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    );
    dev = from_ascii(
        (ascii_header.c_rdev).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    );
    (*file_hdr).c_rdev_maj = gnu_dev_major(dev);
    (*file_hdr).c_rdev_min = gnu_dev_minor(dev);
    (*file_hdr)
        .c_mtime = from_ascii(
        (ascii_header.c_mtime).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    ) as time_t;
    (*file_hdr)
        .c_filesize = from_ascii(
        (ascii_header.c_filesize).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
        3 as libc::c_int as libc::c_uint,
    ) as off_t;
    read_name_from_file(
        file_hdr,
        in_des,
        from_ascii(
            (ascii_header.c_namesize).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
            3 as libc::c_int as libc::c_uint,
        ),
    );
    match (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint {
        8192 | 24576 | 49152 | 4096 => {
            if (*file_hdr).c_filesize != 0 as libc::c_int as libc::c_long
                && (*file_hdr).c_rdev_maj == 0 as libc::c_int as libc::c_uint
                && (*file_hdr).c_rdev_min == 1 as libc::c_int as libc::c_uint
            {
                (*file_hdr)
                    .c_rdev_maj = gnu_dev_major((*file_hdr).c_filesize as __dev_t);
                (*file_hdr)
                    .c_rdev_min = gnu_dev_minor((*file_hdr).c_filesize as __dev_t);
                (*file_hdr).c_filesize = 0 as libc::c_int as off_t;
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn read_in_new_ascii(
    mut file_hdr: *mut cpio_file_stat,
    mut in_des: libc::c_int,
) {
    let mut ascii_header: new_ascii_header = new_ascii_header {
        c_magic: [0; 6],
        c_ino: [0; 8],
        c_mode: [0; 8],
        c_uid: [0; 8],
        c_gid: [0; 8],
        c_nlink: [0; 8],
        c_mtime: [0; 8],
        c_filesize: [0; 8],
        c_dev_maj: [0; 8],
        c_dev_min: [0; 8],
        c_rdev_maj: [0; 8],
        c_rdev_min: [0; 8],
        c_namesize: [0; 8],
        c_chksum: [0; 8],
    };
    tape_buffered_read(
        (ascii_header.c_ino).as_mut_ptr(),
        in_des,
        (::std::mem::size_of::<new_ascii_header>() as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            as off_t,
    );
    (*file_hdr)
        .c_ino = from_ascii(
        (ascii_header.c_ino).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    );
    (*file_hdr)
        .c_mode = from_ascii(
        (ascii_header.c_mode).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as mode_t;
    (*file_hdr)
        .c_uid = from_ascii(
        (ascii_header.c_uid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as uid_t;
    (*file_hdr)
        .c_gid = from_ascii(
        (ascii_header.c_gid).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as gid_t;
    (*file_hdr)
        .c_nlink = from_ascii(
        (ascii_header.c_nlink).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    );
    (*file_hdr)
        .c_mtime = from_ascii(
        (ascii_header.c_mtime).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as time_t;
    (*file_hdr)
        .c_filesize = from_ascii(
        (ascii_header.c_filesize).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as off_t;
    (*file_hdr)
        .c_dev_maj = from_ascii(
        (ascii_header.c_dev_maj).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as libc::c_uint;
    (*file_hdr)
        .c_dev_min = from_ascii(
        (ascii_header.c_dev_min).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as libc::c_uint;
    (*file_hdr)
        .c_rdev_maj = from_ascii(
        (ascii_header.c_rdev_maj).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as libc::c_uint;
    (*file_hdr)
        .c_rdev_min = from_ascii(
        (ascii_header.c_rdev_min).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as libc::c_uint;
    (*file_hdr)
        .c_chksum = from_ascii(
        (ascii_header.c_chksum).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        4 as libc::c_int as libc::c_uint,
    ) as uint32_t;
    read_name_from_file(
        file_hdr,
        in_des,
        from_ascii(
            (ascii_header.c_namesize).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            4 as libc::c_int as libc::c_uint,
        ),
    );
    tape_skip_padding(
        in_des,
        ((*file_hdr).c_namesize).wrapping_add(110 as libc::c_int as libc::c_ulong)
            as off_t,
    );
}
pub unsafe extern "C" fn read_in_binary(
    mut file_hdr: *mut cpio_file_stat,
    mut short_hdr: *mut old_cpio_header,
    mut in_des: libc::c_int,
) {
    (*file_hdr).c_magic = (*short_hdr).c_magic;
    tape_buffered_read(
        (short_hdr as *mut libc::c_char).offset(6 as libc::c_int as isize),
        in_des,
        (::std::mem::size_of::<old_cpio_header>() as libc::c_ulong)
            .wrapping_sub(6 as libc::c_int as libc::c_ulong) as off_t,
    );
    if (*file_hdr).c_magic as libc::c_int
        == (0o70707 as libc::c_int as libc::c_ushort as libc::c_int) << 8 as libc::c_int
            & 0xff00 as libc::c_int
            | 0o70707 as libc::c_int as libc::c_ushort as libc::c_int >> 8 as libc::c_int
                & 0xff as libc::c_int
    {
        static mut warned: libc::c_int = 0 as libc::c_int;
        if warned == 0 as libc::c_int {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: archive header has reverse byte-order\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            warned = 1 as libc::c_int;
        }
        swab_array(short_hdr as *mut libc::c_char, 13 as libc::c_int);
    }
    (*file_hdr).c_dev_maj = gnu_dev_major((*short_hdr).c_dev as __dev_t);
    (*file_hdr).c_dev_min = gnu_dev_minor((*short_hdr).c_dev as __dev_t);
    (*file_hdr).c_ino = (*short_hdr).c_ino as ino_t;
    (*file_hdr).c_mode = (*short_hdr).c_mode as mode_t;
    (*file_hdr).c_uid = (*short_hdr).c_uid as uid_t;
    (*file_hdr).c_gid = (*short_hdr).c_gid as gid_t;
    (*file_hdr).c_nlink = (*short_hdr).c_nlink as size_t;
    (*file_hdr).c_rdev_maj = gnu_dev_major((*short_hdr).c_rdev as __dev_t);
    (*file_hdr).c_rdev_min = gnu_dev_minor((*short_hdr).c_rdev as __dev_t);
    (*file_hdr)
        .c_mtime = (((*short_hdr).c_mtimes[0 as libc::c_int as usize] as libc::c_ulong)
        << 16 as libc::c_int
        | (*short_hdr).c_mtimes[1 as libc::c_int as usize] as libc::c_ulong) as time_t;
    (*file_hdr)
        .c_filesize = (((*short_hdr).c_filesizes[0 as libc::c_int as usize]
        as libc::c_ulong) << 16 as libc::c_int
        | (*short_hdr).c_filesizes[1 as libc::c_int as usize] as libc::c_ulong) as off_t;
    read_name_from_file(file_hdr, in_des, (*short_hdr).c_namesize as uintmax_t);
    if ((*file_hdr).c_namesize).wrapping_rem(2 as libc::c_int as libc::c_ulong) != 0 {
        tape_toss_input(in_des, 1 as libc::c_long);
    }
    match (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint {
        8192 | 24576 | 49152 | 4096 => {
            if (*file_hdr).c_filesize != 0 as libc::c_int as libc::c_long
                && (*file_hdr).c_rdev_maj == 0 as libc::c_int as libc::c_uint
                && (*file_hdr).c_rdev_min == 1 as libc::c_int as libc::c_uint
            {
                (*file_hdr)
                    .c_rdev_maj = gnu_dev_major((*file_hdr).c_filesize as __dev_t);
                (*file_hdr)
                    .c_rdev_min = gnu_dev_minor((*file_hdr).c_filesize as __dev_t);
                (*file_hdr).c_filesize = 0 as libc::c_int as off_t;
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn swab_array(mut ptr: *mut libc::c_char, mut count: libc::c_int) {
    let mut tmp: libc::c_char = 0;
    loop {
        let fresh4 = count;
        count = count - 1;
        if !(fresh4 > 0 as libc::c_int) {
            break;
        }
        tmp = *ptr;
        *ptr = *ptr.offset(1 as libc::c_int as isize);
        ptr = ptr.offset(1);
        ptr;
        *ptr = tmp;
        ptr = ptr.offset(1);
        ptr;
    };
}
pub unsafe extern "C" fn process_copy_in() {
    let mut tty_in: *mut FILE = 0 as *mut FILE;
    let mut tty_out: *mut FILE = 0 as *mut FILE;
    let mut rename_in: *mut FILE = 0 as *mut FILE;
    let mut file_stat: stat = stat {
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
    let mut file_hdr: cpio_file_stat = {
        let mut init = cpio_file_stat {
            c_magic: 0 as libc::c_int as libc::c_ushort,
            c_ino: 0 as libc::c_int as ino_t,
            c_mode: 0 as libc::c_int as mode_t,
            c_uid: 0 as libc::c_int as uid_t,
            c_gid: 0 as libc::c_int as gid_t,
            c_nlink: 0 as libc::c_int as size_t,
            c_mtime: 0 as libc::c_int as time_t,
            c_filesize: 0 as libc::c_int as off_t,
            c_dev_maj: 0 as libc::c_int as libc::c_uint,
            c_dev_min: 0 as libc::c_int as libc::c_uint,
            c_rdev_maj: 0 as libc::c_int as libc::c_uint,
            c_rdev_min: 0 as libc::c_int as libc::c_uint,
            c_namesize: 0 as libc::c_int as size_t,
            c_chksum: 0 as libc::c_int as uint32_t,
            c_name: 0 as *mut libc::c_char,
            c_name_buflen: 0 as libc::c_int as size_t,
            c_tar_linkname: 0 as *const libc::c_char,
        };
        init
    };
    let mut in_file_des: libc::c_int = 0;
    let mut skip_file: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    newdir_umask = umask(0 as libc::c_int as __mode_t);
    if !pattern_file_name.is_null() {
        read_pattern_file();
    }
    if !rename_batch_file.is_null() {
        rename_in = fopen(rename_batch_file, b"r\0" as *const u8 as *const libc::c_char);
        if rename_in.is_null() {
            error(
                2 as libc::c_int,
                *__errno_location(),
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if rename_flag != 0 {
        tty_in = fopen(
            b"/dev/tty\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if tty_in.is_null() {
            error(
                2 as libc::c_int,
                *__errno_location(),
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
            );
        }
        tty_out = fopen(
            b"/dev/tty\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if tty_out.is_null() {
            error(
                2 as libc::c_int,
                *__errno_location(),
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if table_flag != 0 && verbose_flag != 0 {
        current_time = current_timespec();
    }
    in_file_des = archive_des;
    if in_file_des >= (1 as libc::c_int) << 30 as libc::c_int {
        input_is_special = 1 as libc::c_int as libc::c_char;
        input_is_seekable = 0 as libc::c_int as libc::c_char;
    } else {
        if fstat(in_file_des, &mut file_stat) != 0 {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard input is closed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        input_is_special = (file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint
            || file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint) as libc::c_int
            as libc::c_char;
        input_is_seekable = (file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_char;
    }
    output_is_seekable = 1 as libc::c_int as libc::c_char;
    change_dir();
    loop {
        swapping_bytes = 0 as libc::c_int;
        swapping_halfwords = swapping_bytes;
        read_in_header(&mut file_hdr, in_file_des);
        if file_hdr.c_namesize == 0 as libc::c_int as libc::c_ulong {
            skip_file = 1 as libc::c_int as libc::c_char;
        } else {
            if strcmp(
                b"TRAILER!!!\0" as *const u8 as *const libc::c_char,
                file_hdr.c_name,
            ) == 0 as libc::c_int
            {
                break;
            }
            cpio_safer_name_suffix(
                file_hdr.c_name,
                0 as libc::c_int != 0,
                no_abs_paths_flag == 0,
                0 as libc::c_int != 0,
            );
            if num_patterns <= 0 as libc::c_int {
                skip_file = 0 as libc::c_int as libc::c_char;
            } else {
                skip_file = copy_matching_files as libc::c_char;
                i = 0 as libc::c_int;
                while i < num_patterns && skip_file as libc::c_int == copy_matching_files
                {
                    if fnmatch(
                        *save_patterns.offset(i as isize),
                        file_hdr.c_name,
                        0 as libc::c_int,
                    ) == 0 as libc::c_int
                    {
                        skip_file = (copy_matching_files == 0) as libc::c_int
                            as libc::c_char;
                    }
                    i += 1;
                    i;
                }
            }
        }
        if skip_file != 0 {
            if file_hdr.c_nlink > 1 as libc::c_int as libc::c_ulong
                && (archive_format as libc::c_uint
                    == arf_newascii as libc::c_int as libc::c_uint
                    || archive_format as libc::c_uint
                        == arf_crcascii as libc::c_int as libc::c_uint)
            {
                if create_defered_links_to_skipped(&mut file_hdr, in_file_des)
                    < 0 as libc::c_int
                {
                    tape_toss_input(in_file_des, file_hdr.c_filesize);
                    tape_skip_padding(in_file_des, file_hdr.c_filesize);
                }
            } else {
                tape_toss_input(in_file_des, file_hdr.c_filesize);
                tape_skip_padding(in_file_des, file_hdr.c_filesize);
            }
        } else if table_flag != 0 {
            list_file(&mut file_hdr, in_file_des);
        } else if append_flag != 0 {
            tape_toss_input(in_file_des, file_hdr.c_filesize);
            tape_skip_padding(in_file_des, file_hdr.c_filesize);
        } else if only_verify_crc_flag != 0 {
            if file_hdr.c_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {
                if archive_format as libc::c_uint
                    != arf_tar as libc::c_int as libc::c_uint
                    && archive_format as libc::c_uint
                        != arf_ustar as libc::c_int as libc::c_uint
                {
                    tape_toss_input(in_file_des, file_hdr.c_filesize);
                    tape_skip_padding(in_file_des, file_hdr.c_filesize);
                    continue;
                }
            }
            crc = 0 as libc::c_int as uint32_t;
            tape_toss_input(in_file_des, file_hdr.c_filesize);
            tape_skip_padding(in_file_des, file_hdr.c_filesize);
            if crc != file_hdr.c_chksum {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: checksum error (0x%x, should be 0x%x)\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    file_hdr.c_name,
                    crc,
                    file_hdr.c_chksum,
                );
            }
            if verbose_flag != 0 {
                fprintf(
                    stderr,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    file_hdr.c_name,
                );
            }
            if dot_flag != 0 {
                fputc_unlocked('.' as i32, stderr);
            }
        } else {
            if rename_flag != 0 || !rename_batch_file.is_null() {
                if query_rename(&mut file_hdr, tty_in, tty_out, rename_in)
                    < 0 as libc::c_int
                {
                    tape_toss_input(in_file_des, file_hdr.c_filesize);
                    tape_skip_padding(in_file_des, file_hdr.c_filesize);
                    continue;
                }
            }
            copyin_file(&mut file_hdr, in_file_des);
            if verbose_flag != 0 {
                fprintf(
                    stderr,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    file_hdr.c_name,
                );
            }
            if dot_flag != 0 {
                fputc_unlocked('.' as i32, stderr);
            }
        }
    }
    if dot_flag != 0 {
        fputc_unlocked('\n' as i32, stderr);
    }
    replace_symlink_placeholders();
    apply_delayed_set_stat();
    cpio_file_stat_free(&mut file_hdr);
    if append_flag != 0 {
        return;
    }
    if archive_format as libc::c_uint == arf_newascii as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_crcascii as libc::c_int as libc::c_uint
    {
        create_final_defers();
    }
    if quiet_flag == 0 {
        let mut blocks: size_t = 0;
        blocks = ((input_bytes + io_block_size as libc::c_long
            - 1 as libc::c_int as libc::c_long) / io_block_size as libc::c_long)
            as size_t;
        fprintf(
            stderr,
            dcngettext(
                0 as *const libc::c_char,
                b"%lu block\n\0" as *const u8 as *const libc::c_char,
                b"%lu blocks\n\0" as *const u8 as *const libc::c_char,
                blocks,
                5 as libc::c_int,
            ),
            blocks,
        );
    }
}
