use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __uflow(_: *mut FILE) -> libc::c_int;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, ps: *mut size_t) -> *mut libc::c_void;
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
    fn chmod_error_details(name: *const libc::c_char, mode: mode_t);
    fn chown_error_details(name: *const libc::c_char, uid: uid_t, gid: gid_t);
    fn mkdir_error(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn stat_error(_: *const libc::c_char);
    fn utime_error(_: *const libc::c_char);
    fn safer_name_suffix(
        file_name: *const libc::c_char,
        link_target: bool,
        absolute_names: bool,
    ) -> *mut libc::c_char;
    fn ds_free(string: *mut dynamic_string);
    fn ds_fgets(f: *mut FILE, s: *mut dynamic_string) -> *mut libc::c_char;
    static mut io_block_size: libc::c_int;
    static mut create_dir_flag: libc::c_int;
    static mut retain_time_flag: libc::c_int;
    static mut crc_i_flag: libc::c_int;
    static mut append_flag: libc::c_int;
    static mut swapping_bytes: libc::c_int;
    static mut swapping_halfwords: libc::c_int;
    static mut set_owner_flag: libc::c_int;
    static mut set_owner: uid_t;
    static mut set_group_flag: libc::c_int;
    static mut set_group: gid_t;
    static mut no_chown_flag: libc::c_int;
    static mut sparse_flag: libc::c_int;
    static mut only_verify_crc_flag: libc::c_int;
    static mut warn_option: libc::c_uint;
    static mut newdir_umask: mode_t;
    static mut renumber_inodes_option: libc::c_int;
    static mut ignore_devno_option: libc::c_int;
    static mut to_stdout_option: bool;
    static mut last_header_start: off_t;
    static mut new_media_message: *mut libc::c_char;
    static mut new_media_message_with_number: *mut libc::c_char;
    static mut new_media_message_after_number: *mut libc::c_char;
    static mut archive_name: *mut libc::c_char;
    static mut rsh_command_option: *mut libc::c_char;
    static mut crc: uint32_t;
    static mut input_buffer: *mut libc::c_char;
    static mut output_buffer: *mut libc::c_char;
    static mut in_buff: *mut libc::c_char;
    static mut out_buff: *mut libc::c_char;
    static mut input_buffer_size: size_t;
    static mut input_size: size_t;
    static mut output_size: size_t;
    static mut input_bytes: off_t;
    static mut output_bytes: off_t;
    static mut input_is_special: libc::c_char;
    static mut output_is_special: libc::c_char;
    static mut xstat: Option::<unsafe extern "C" fn() -> libc::c_int>;
    static mut copy_function: Option::<unsafe extern "C" fn() -> ()>;
    static mut change_directory_option: *mut libc::c_char;
    fn swab_array(arg: *mut libc::c_char, count: libc::c_int);
    fn process_copy_in();
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn make_path(
        argpath: *const libc::c_char,
        owner: uid_t,
        group: gid_t,
        verbose_fmt_string: *const libc::c_char,
    ) -> libc::c_int;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn full_write(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> size_t;
    static mut rmt_dev_name__: *const libc::c_char;
    fn rmt_open__(
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn rmt_close__(_: libc::c_int) -> libc::c_int;
    fn rmt_read__(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
    fn rmt_write__(_: libc::c_int, _: *mut libc::c_char, _: size_t) -> size_t;
    fn rmt_ioctl__(
        _: libc::c_int,
        _: libc::c_ulong,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    static mut force_local_option: bool;
    fn hash_lookup(
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
    fn fdutimens(
        _: libc::c_int,
        _: *const libc::c_char,
        _: *const timespec,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
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
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: libc::c_int,
}
pub const in_zeros: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const not_in_zeros: C2RustUnnamed = 2;
pub const begin: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inode_val {
    pub inode: ino_t,
    pub major_num: libc::c_ulong,
    pub minor_num: libc::c_ulong,
    pub trans_inode: ino_t,
    pub file_name: *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_set_stat {
    pub next: *mut delayed_set_stat,
    pub stat: cpio_file_stat,
    pub invert_permissions: mode_t,
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
pub unsafe extern "C" fn tape_empty_output_buffer(mut out_des: libc::c_int) {
    let mut bytes_written: libc::c_int = 0;
    bytes_written = (if out_des >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_write__(
            out_des - ((1 as libc::c_int) << 30 as libc::c_int),
            output_buffer,
            output_size,
        )
    } else {
        full_write(out_des, output_buffer as *const libc::c_void, output_size)
    }) as libc::c_int;
    if bytes_written as libc::c_ulong != output_size {
        let mut rest_bytes_written: libc::c_int = 0;
        let mut rest_output_size: libc::c_int = 0;
        if output_is_special as libc::c_int != 0
            && (bytes_written >= 0 as libc::c_int
                || (*__errno_location() == 28 as libc::c_int
                    || *__errno_location() == 5 as libc::c_int
                    || *__errno_location() == 6 as libc::c_int))
        {
            get_next_reel(out_des);
            if bytes_written > 0 as libc::c_int {
                rest_output_size = output_size
                    .wrapping_sub(bytes_written as libc::c_ulong) as libc::c_int;
            } else {
                rest_output_size = output_size as libc::c_int;
            }
            rest_bytes_written = (if out_des >= (1 as libc::c_int) << 30 as libc::c_int {
                rmt_write__(
                    out_des - ((1 as libc::c_int) << 30 as libc::c_int),
                    output_buffer,
                    rest_output_size as size_t,
                )
            } else {
                full_write(
                    out_des,
                    output_buffer as *const libc::c_void,
                    rest_output_size as size_t,
                )
            }) as libc::c_int;
            if rest_bytes_written != rest_output_size {
                error(
                    2 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"write error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            }
        } else {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    output_bytes = (output_bytes as libc::c_ulong).wrapping_add(output_size) as off_t
        as off_t;
    out_buff = output_buffer;
    output_size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn disk_empty_output_buffer(
    mut out_des: libc::c_int,
    mut flush: bool,
) {
    let mut bytes_written: ssize_t = 0;
    if swapping_halfwords != 0 || swapping_bytes != 0 {
        if swapping_halfwords != 0 {
            let mut complete_words: libc::c_int = 0;
            complete_words = output_size.wrapping_div(4 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            swahw_array(output_buffer, complete_words);
            if swapping_bytes != 0 {
                swab_array(output_buffer, 2 as libc::c_int * complete_words);
            }
        } else {
            let mut complete_halfwords: libc::c_int = 0;
            complete_halfwords = output_size
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
            swab_array(output_buffer, complete_halfwords);
        }
    }
    if sparse_flag != 0 {
        bytes_written = sparse_write(out_des, output_buffer, output_size, flush);
    } else {
        bytes_written = write(
            out_des,
            output_buffer as *const libc::c_void,
            output_size,
        );
    }
    if bytes_written as libc::c_ulong != output_size {
        if bytes_written == -(1 as libc::c_int) as libc::c_long {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            error(
                2 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"write error: partial write\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    output_bytes = (output_bytes as libc::c_ulong).wrapping_add(output_size) as off_t
        as off_t;
    out_buff = output_buffer;
    output_size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn swahw_array(
    mut ptr: *mut libc::c_char,
    mut count: libc::c_int,
) {
    let mut tmp: libc::c_char = 0;
    while count > 0 as libc::c_int {
        tmp = *ptr;
        *ptr = *ptr.offset(2 as libc::c_int as isize);
        *ptr.offset(2 as libc::c_int as isize) = tmp;
        ptr = ptr.offset(1);
        ptr;
        tmp = *ptr;
        *ptr = *ptr.offset(2 as libc::c_int as isize);
        *ptr.offset(2 as libc::c_int as isize) = tmp;
        ptr = ptr.offset(3 as libc::c_int as isize);
        count -= 1;
        count;
    }
}
unsafe extern "C" fn tape_fill_input_buffer(
    mut in_des: libc::c_int,
    mut num_bytes: libc::c_int,
) {
    in_buff = input_buffer;
    num_bytes = if num_bytes < io_block_size { num_bytes } else { io_block_size };
    input_size = if in_des >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_read__(
            in_des - ((1 as libc::c_int) << 30 as libc::c_int),
            input_buffer,
            num_bytes as size_t,
        )
    } else {
        safe_read(in_des, input_buffer as *mut libc::c_void, num_bytes as size_t)
    };
    if input_size == 0 as libc::c_int as libc::c_ulong
        && input_is_special as libc::c_int != 0
    {
        get_next_reel(in_des);
        input_size = if in_des >= (1 as libc::c_int) << 30 as libc::c_int {
            rmt_read__(
                in_des - ((1 as libc::c_int) << 30 as libc::c_int),
                input_buffer,
                num_bytes as size_t,
            )
        } else {
            safe_read(in_des, input_buffer as *mut libc::c_void, num_bytes as size_t)
        };
    }
    if input_size == -(1 as libc::c_int) as size_t {
        error(
            2 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"read error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if input_size == 0 as libc::c_int as libc::c_ulong {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"premature end of file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    input_bytes = (input_bytes as libc::c_ulong).wrapping_add(input_size) as off_t
        as off_t;
}
unsafe extern "C" fn disk_fill_input_buffer(
    mut in_des: libc::c_int,
    mut num_bytes: off_t,
) -> libc::c_int {
    in_buff = input_buffer;
    num_bytes = if num_bytes < 512 as libc::c_int as libc::c_long {
        num_bytes
    } else {
        512 as libc::c_int as libc::c_long
    };
    input_size = read(in_des, input_buffer as *mut libc::c_void, num_bytes as size_t)
        as size_t;
    if input_size == -(1 as libc::c_int) as size_t {
        input_size = 0 as libc::c_int as size_t;
        return -(1 as libc::c_int);
    } else if input_size == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int
    }
    input_bytes = (input_bytes as libc::c_ulong).wrapping_add(input_size) as off_t
        as off_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn tape_buffered_write(
    mut in_buf: *mut libc::c_char,
    mut out_des: libc::c_int,
    mut num_bytes: off_t,
) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as libc::c_int as libc::c_long {
        space_left = (io_block_size as libc::c_ulong).wrapping_sub(output_size) as off_t;
        if space_left == 0 as libc::c_int as libc::c_long {
            tape_empty_output_buffer(out_des);
        } else {
            if bytes_left < space_left {
                space_left = bytes_left;
            }
            memcpy(
                out_buff as *mut libc::c_void,
                in_buf as *const libc::c_void,
                space_left as libc::c_uint as libc::c_ulong,
            );
            out_buff = out_buff.offset(space_left as isize);
            output_size = (output_size as libc::c_ulong)
                .wrapping_add(space_left as libc::c_ulong) as size_t as size_t;
            in_buf = in_buf.offset(space_left as isize);
            bytes_left -= space_left;
        }
    }
}
pub unsafe extern "C" fn disk_buffered_write(
    mut in_buf: *mut libc::c_char,
    mut out_des: libc::c_int,
    mut num_bytes: off_t,
) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as libc::c_int as libc::c_long {
        space_left = (512 as libc::c_int as libc::c_ulong).wrapping_sub(output_size)
            as off_t;
        if space_left == 0 as libc::c_int as libc::c_long {
            disk_empty_output_buffer(out_des, 0 as libc::c_int != 0);
        } else {
            if bytes_left < space_left {
                space_left = bytes_left;
            }
            memcpy(
                out_buff as *mut libc::c_void,
                in_buf as *const libc::c_void,
                space_left as libc::c_uint as libc::c_ulong,
            );
            out_buff = out_buff.offset(space_left as isize);
            output_size = (output_size as libc::c_ulong)
                .wrapping_add(space_left as libc::c_ulong) as size_t as size_t;
            in_buf = in_buf.offset(space_left as isize);
            bytes_left -= space_left;
        }
    }
}
pub unsafe extern "C" fn tape_buffered_read(
    mut in_buf: *mut libc::c_char,
    mut in_des: libc::c_int,
    mut num_bytes: off_t,
) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as libc::c_int as libc::c_long {
        if input_size == 0 as libc::c_int as libc::c_ulong {
            tape_fill_input_buffer(in_des, io_block_size);
        }
        if (bytes_left as libc::c_ulong) < input_size {
            space_left = bytes_left;
        } else {
            space_left = input_size as off_t;
        }
        memcpy(
            in_buf as *mut libc::c_void,
            in_buff as *const libc::c_void,
            space_left as libc::c_uint as libc::c_ulong,
        );
        in_buff = in_buff.offset(space_left as isize);
        in_buf = in_buf.offset(space_left as isize);
        input_size = (input_size as libc::c_ulong)
            .wrapping_sub(space_left as libc::c_ulong) as size_t as size_t;
        bytes_left -= space_left;
    }
}
pub unsafe extern "C" fn tape_buffered_peek(
    mut peek_buf: *mut libc::c_char,
    mut in_des: libc::c_int,
    mut num_bytes: libc::c_int,
) -> libc::c_int {
    let mut tmp_input_size: libc::c_long = 0;
    let mut got_bytes: libc::c_long = 0;
    let mut append_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    while input_size < num_bytes as libc::c_ulong {
        append_buf = in_buff.offset(input_size as isize);
        if append_buf.offset_from(input_buffer) as libc::c_long as libc::c_ulong
            >= input_buffer_size
        {
            let mut half: libc::c_int = 0;
            half = input_buffer_size.wrapping_div(2 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            memmove(
                input_buffer as *mut libc::c_void,
                input_buffer.offset(half as isize) as *const libc::c_void,
                half as libc::c_ulong,
            );
            in_buff = in_buff.offset(-(half as isize));
            append_buf = append_buf.offset(-(half as isize));
        }
        tmp_input_size = (if in_des >= (1 as libc::c_int) << 30 as libc::c_int {
            rmt_read__(
                in_des - ((1 as libc::c_int) << 30 as libc::c_int),
                append_buf,
                io_block_size as size_t,
            )
        } else {
            safe_read(in_des, append_buf as *mut libc::c_void, io_block_size as size_t)
        }) as libc::c_long;
        if tmp_input_size == 0 as libc::c_int as libc::c_long {
            if !(input_is_special != 0) {
                break;
            }
            get_next_reel(in_des);
            tmp_input_size = (if in_des >= (1 as libc::c_int) << 30 as libc::c_int {
                rmt_read__(
                    in_des - ((1 as libc::c_int) << 30 as libc::c_int),
                    append_buf,
                    io_block_size as size_t,
                )
            } else {
                safe_read(
                    in_des,
                    append_buf as *mut libc::c_void,
                    io_block_size as size_t,
                )
            }) as libc::c_long;
        }
        if tmp_input_size < 0 as libc::c_int as libc::c_long {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"read error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        input_bytes += tmp_input_size;
        input_size = (input_size as libc::c_ulong)
            .wrapping_add(tmp_input_size as libc::c_ulong) as size_t as size_t;
    }
    if num_bytes as libc::c_ulong <= input_size {
        got_bytes = num_bytes as libc::c_long;
    } else {
        got_bytes = input_size as libc::c_long;
    }
    memcpy(
        peek_buf as *mut libc::c_void,
        in_buff as *const libc::c_void,
        got_bytes as libc::c_uint as libc::c_ulong,
    );
    return got_bytes as libc::c_int;
}
pub unsafe extern "C" fn tape_toss_input(mut in_des: libc::c_int, mut num_bytes: off_t) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as libc::c_int as libc::c_long {
        if input_size == 0 as libc::c_int as libc::c_ulong {
            tape_fill_input_buffer(in_des, io_block_size);
        }
        if (bytes_left as libc::c_ulong) < input_size {
            space_left = bytes_left;
        } else {
            space_left = input_size as off_t;
        }
        if crc_i_flag != 0 && only_verify_crc_flag != 0 {
            let mut k: libc::c_int = 0;
            k = 0 as libc::c_int;
            while (k as libc::c_long) < space_left {
                crc = (crc as libc::c_uint)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as libc::c_int
                            & 0xff as libc::c_int) as libc::c_uint,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        in_buff = in_buff.offset(space_left as isize);
        input_size = (input_size as libc::c_ulong)
            .wrapping_sub(space_left as libc::c_ulong) as size_t as size_t;
        bytes_left -= space_left;
    }
}
pub unsafe extern "C" fn write_nuls_to_file(
    mut num_bytes: off_t,
    mut out_des: libc::c_int,
    mut writer: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int, off_t) -> (),
    >,
) {
    let mut blocks: off_t = 0;
    let mut extra_bytes: off_t = 0;
    let mut i: off_t = 0;
    static mut zeros_512: [libc::c_char; 512] = [0; 512];
    blocks = (num_bytes as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
        as off_t;
    extra_bytes = (num_bytes as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
        as off_t;
    i = 0 as libc::c_int as off_t;
    while i < blocks {
        writer
            .unwrap()(
            zeros_512.as_mut_ptr(),
            out_des,
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as off_t,
        );
        i += 1;
        i;
    }
    if extra_bytes != 0 {
        writer.unwrap()(zeros_512.as_mut_ptr(), out_des, extra_bytes);
    }
}
pub unsafe extern "C" fn copy_files_tape_to_disk(
    mut in_des: libc::c_int,
    mut out_des: libc::c_int,
    mut num_bytes: off_t,
) {
    let mut size: off_t = 0;
    let mut k: off_t = 0;
    while num_bytes > 0 as libc::c_int as libc::c_long {
        if input_size == 0 as libc::c_int as libc::c_ulong {
            tape_fill_input_buffer(in_des, io_block_size);
        }
        size = (if input_size < num_bytes as libc::c_ulong {
            input_size
        } else {
            num_bytes as libc::c_ulong
        }) as off_t;
        if crc_i_flag != 0 {
            k = 0 as libc::c_int as off_t;
            while k < size {
                crc = (crc as libc::c_uint)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as libc::c_int
                            & 0xff as libc::c_int) as libc::c_uint,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        disk_buffered_write(in_buff, out_des, size);
        num_bytes -= size;
        input_size = (input_size as libc::c_ulong).wrapping_sub(size as libc::c_ulong)
            as size_t as size_t;
        in_buff = in_buff.offset(size as isize);
    }
}
pub unsafe extern "C" fn copy_files_disk_to_tape(
    mut in_des: libc::c_int,
    mut out_des: libc::c_int,
    mut num_bytes: off_t,
    mut filename: *mut libc::c_char,
) {
    let mut size: off_t = 0;
    let mut k: off_t = 0;
    let mut rc: libc::c_int = 0;
    let mut original_num_bytes: off_t = 0;
    original_num_bytes = num_bytes;
    while num_bytes > 0 as libc::c_int as libc::c_long {
        if input_size == 0 as libc::c_int as libc::c_ulong {
            rc = disk_fill_input_buffer(
                in_des,
                if num_bytes < 512 as libc::c_int as libc::c_long {
                    num_bytes
                } else {
                    512 as libc::c_int as libc::c_long
                },
            );
            if rc != 0 {
                if rc > 0 as libc::c_int {
                    let mut buf: [libc::c_char; 21] = [0; 21];
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcngettext(
                            0 as *const libc::c_char,
                            b"File %s shrunk by %s byte, padding with zeros\0"
                                as *const u8 as *const libc::c_char,
                            b"File %s shrunk by %s bytes, padding with zeros\0"
                                as *const u8 as *const libc::c_char,
                            num_bytes as libc::c_ulong,
                            5 as libc::c_int,
                        ),
                        filename,
                        umaxtostr(num_bytes as uintmax_t, buf.as_mut_ptr()),
                    );
                } else {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Read error at byte %lld in file %s, padding with zeros\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (original_num_bytes - num_bytes) as libc::c_longlong,
                        filename,
                    );
                }
                write_nuls_to_file(
                    num_bytes,
                    out_des,
                    Some(
                        tape_buffered_write
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                off_t,
                            ) -> (),
                    ),
                );
                break;
            }
        }
        size = (if input_size < num_bytes as libc::c_ulong {
            input_size
        } else {
            num_bytes as libc::c_ulong
        }) as off_t;
        if crc_i_flag != 0 {
            k = 0 as libc::c_int as off_t;
            while k < size {
                crc = (crc as libc::c_uint)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as libc::c_int
                            & 0xff as libc::c_int) as libc::c_uint,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        tape_buffered_write(in_buff, out_des, size);
        num_bytes -= size;
        input_size = (input_size as libc::c_ulong).wrapping_sub(size as libc::c_ulong)
            as size_t as size_t;
        in_buff = in_buff.offset(size as isize);
    }
}
pub unsafe extern "C" fn copy_files_disk_to_disk(
    mut in_des: libc::c_int,
    mut out_des: libc::c_int,
    mut num_bytes: off_t,
    mut filename: *mut libc::c_char,
) {
    let mut size: off_t = 0;
    let mut k: off_t = 0;
    let mut original_num_bytes: off_t = 0;
    let mut rc: libc::c_int = 0;
    original_num_bytes = num_bytes;
    while num_bytes > 0 as libc::c_int as libc::c_long {
        if input_size == 0 as libc::c_int as libc::c_ulong {
            rc = disk_fill_input_buffer(in_des, num_bytes);
            if rc != 0 {
                if rc > 0 as libc::c_int {
                    let mut buf: [libc::c_char; 21] = [0; 21];
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcngettext(
                            0 as *const libc::c_char,
                            b"File %s shrunk by %s byte, padding with zeros\0"
                                as *const u8 as *const libc::c_char,
                            b"File %s shrunk by %s bytes, padding with zeros\0"
                                as *const u8 as *const libc::c_char,
                            num_bytes as libc::c_ulong,
                            5 as libc::c_int,
                        ),
                        filename,
                        umaxtostr(num_bytes as uintmax_t, buf.as_mut_ptr()),
                    );
                } else {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Read error at byte %lld in file %s, padding with zeros\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (original_num_bytes - num_bytes) as libc::c_longlong,
                        filename,
                    );
                }
                write_nuls_to_file(
                    num_bytes,
                    out_des,
                    Some(
                        disk_buffered_write
                            as unsafe extern "C" fn(
                                *mut libc::c_char,
                                libc::c_int,
                                off_t,
                            ) -> (),
                    ),
                );
                break;
            }
        }
        size = (if input_size < num_bytes as libc::c_ulong {
            input_size
        } else {
            num_bytes as libc::c_ulong
        }) as off_t;
        if crc_i_flag != 0 {
            k = 0 as libc::c_int as off_t;
            while k < size {
                crc = (crc as libc::c_uint)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as libc::c_int
                            & 0xff as libc::c_int) as libc::c_uint,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        disk_buffered_write(in_buff, out_des, size);
        num_bytes -= size;
        input_size = (input_size as libc::c_ulong).wrapping_sub(size as libc::c_ulong)
            as size_t as size_t;
        in_buff = in_buff.offset(size as isize);
    }
}
pub unsafe extern "C" fn warn_if_file_changed(
    mut file_name: *mut libc::c_char,
    mut old_file_size: off_t,
    mut old_file_mtime: time_t,
) {
    let mut new_file_stat: stat = stat {
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
    if ::std::mem::transmute::<
        _,
        fn(_, _) -> libc::c_int,
    >((Some(xstat.unwrap())).unwrap())(file_name, &mut new_file_stat) < 0 as libc::c_int
    {
        stat_error(file_name);
        return;
    }
    if new_file_stat.st_size > old_file_size {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcngettext(
                0 as *const libc::c_char,
                b"File %s grew, %lu new byte not copied\0" as *const u8
                    as *const libc::c_char,
                b"File %s grew, %lu new bytes not copied\0" as *const u8
                    as *const libc::c_char,
                (new_file_stat.st_size - old_file_size) as libc::c_ulong,
                5 as libc::c_int,
            ),
            file_name,
            (new_file_stat.st_size - old_file_size) as uintmax_t,
        );
    } else if new_file_stat.st_mtim.tv_sec != old_file_mtime {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"File %s was modified while being copied\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
    }
}
pub unsafe extern "C" fn create_all_directories(mut name: *const libc::c_char) {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    dir = dir_name(name);
    if dir.is_null() {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"virtual memory exhausted\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if *dir.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
        || *dir.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
        if warn_option & 0x2 as libc::c_int as libc::c_uint != 0 {
            fmt = dcgettext(
                0 as *const libc::c_char,
                b"Creating intermediate directory `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            fmt = 0 as *const libc::c_char;
        }
        make_path(dir, -(1 as libc::c_int) as uid_t, -(1 as libc::c_int) as gid_t, fmt);
    }
    rpl_free(dir as *mut libc::c_void);
}
pub unsafe extern "C" fn prepare_append(mut out_file_des: libc::c_int) {
    let mut start_of_header: off_t = 0;
    let mut start_of_block: off_t = 0;
    let mut useful_bytes_in_block: size_t = 0;
    let mut tmp_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    start_of_header = last_header_start;
    useful_bytes_in_block = (start_of_header % io_block_size as libc::c_long) as size_t;
    start_of_block = (start_of_header as libc::c_ulong)
        .wrapping_sub(useful_bytes_in_block) as off_t;
    if lseek(out_file_des, start_of_block, 0 as libc::c_int)
        < 0 as libc::c_int as libc::c_long
    {
        error(
            2 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot seek on output\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if useful_bytes_in_block > 0 as libc::c_int as libc::c_ulong {
        tmp_buf = xmalloc(useful_bytes_in_block) as *mut libc::c_char;
        read(out_file_des, tmp_buf as *mut libc::c_void, useful_bytes_in_block);
        if lseek(out_file_des, start_of_block, 0 as libc::c_int)
            < 0 as libc::c_int as libc::c_long
        {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot seek on output\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        tape_buffered_write(tmp_buf, out_file_des, useful_bytes_in_block as off_t);
        rpl_free(tmp_buf as *mut libc::c_void);
    }
    input_size = 0 as libc::c_int as size_t;
    input_bytes = 0 as libc::c_int as off_t;
    in_buff = input_buffer;
}
static mut hash_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn inode_val_hasher(
    mut val: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut ival: *const inode_val = val as *const inode_val;
    return ((*ival).inode).wrapping_rem(n_buckets);
}
unsafe extern "C" fn inode_val_compare(
    mut val1: *const libc::c_void,
    mut val2: *const libc::c_void,
) -> bool {
    let mut ival1: *const inode_val = val1 as *const inode_val;
    let mut ival2: *const inode_val = val2 as *const inode_val;
    return (*ival1).inode == (*ival2).inode && (*ival1).major_num == (*ival2).major_num
        && (*ival1).minor_num == (*ival2).minor_num;
}
unsafe extern "C" fn find_inode_val(
    mut node_num: ino_t,
    mut major_num: libc::c_ulong,
    mut minor_num: libc::c_ulong,
) -> *mut inode_val {
    let mut sample: inode_val = inode_val {
        inode: 0,
        major_num: 0,
        minor_num: 0,
        trans_inode: 0,
        file_name: 0 as *mut libc::c_char,
    };
    if hash_table.is_null() {
        return 0 as *mut inode_val;
    }
    sample.inode = node_num;
    sample.major_num = major_num;
    sample.minor_num = minor_num;
    return hash_lookup(hash_table, &mut sample as *mut inode_val as *const libc::c_void)
        as *mut inode_val;
}
pub unsafe extern "C" fn find_inode_file(
    mut node_num: ino_t,
    mut major_num: libc::c_ulong,
    mut minor_num: libc::c_ulong,
) -> *mut libc::c_char {
    let mut ival: *mut inode_val = find_inode_val(node_num, major_num, minor_num);
    return if !ival.is_null() { (*ival).file_name } else { 0 as *mut libc::c_char };
}
static mut next_inode: ino_t = 0;
pub unsafe extern "C" fn add_inode(
    mut node_num: ino_t,
    mut file_name: *mut libc::c_char,
    mut major_num: libc::c_ulong,
    mut minor_num: libc::c_ulong,
) -> *mut inode_val {
    let mut temp: *mut inode_val = 0 as *mut inode_val;
    let mut e: *mut inode_val = 0 as *mut inode_val;
    temp = xmalloc(::std::mem::size_of::<inode_val>() as libc::c_ulong)
        as *mut inode_val;
    (*temp).inode = node_num;
    (*temp).major_num = major_num;
    (*temp).minor_num = minor_num;
    (*temp)
        .file_name = if !file_name.is_null() {
        xstrdup(file_name)
    } else {
        0 as *mut libc::c_char
    };
    if renumber_inodes_option != 0 {
        let fresh1 = next_inode;
        next_inode = next_inode.wrapping_add(1);
        (*temp).trans_inode = fresh1;
    } else {
        (*temp).trans_inode = (*temp).inode;
    }
    if !((!hash_table.is_null()
        || {
            hash_table = hash_initialize(
                0 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    inode_val_hasher
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    inode_val_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                None,
            );
            !hash_table.is_null()
        })
        && {
            e = hash_insert(hash_table, temp as *const libc::c_void) as *mut inode_val;
            !e.is_null()
        })
    {
        xalloc_die();
    }
    return e;
}
unsafe extern "C" fn get_inode_and_dev(mut hdr: *mut cpio_file_stat, mut st: *mut stat) {
    if renumber_inodes_option != 0 {
        if (*st).st_nlink > 1 as libc::c_int as libc::c_ulong {
            let mut ival: *mut inode_val = find_inode_val(
                (*st).st_ino,
                gnu_dev_major((*st).st_dev) as libc::c_ulong,
                gnu_dev_minor((*st).st_dev) as libc::c_ulong,
            );
            if ival.is_null() {
                ival = add_inode(
                    (*st).st_ino,
                    0 as *mut libc::c_char,
                    gnu_dev_major((*st).st_dev) as libc::c_ulong,
                    gnu_dev_minor((*st).st_dev) as libc::c_ulong,
                );
            }
            (*hdr).c_ino = (*ival).trans_inode;
        } else {
            let fresh2 = next_inode;
            next_inode = next_inode.wrapping_add(1);
            (*hdr).c_ino = fresh2;
        }
    } else {
        (*hdr).c_ino = (*st).st_ino;
    }
    if ignore_devno_option != 0 {
        (*hdr).c_dev_maj = 0 as libc::c_int as libc::c_uint;
        (*hdr).c_dev_min = 0 as libc::c_int as libc::c_uint;
    } else {
        (*hdr).c_dev_maj = gnu_dev_major((*st).st_dev);
        (*hdr).c_dev_min = gnu_dev_minor((*st).st_dev);
    };
}
pub unsafe extern "C" fn open_archive(mut file: *mut libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut copy_in: Option::<unsafe extern "C" fn() -> ()> = None;
    copy_in = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        Option::<unsafe extern "C" fn() -> ()>,
    >(Some(process_copy_in as unsafe extern "C" fn() -> ()));
    if copy_function == copy_in {
        fd = if !force_local_option
            && {
                rmt_dev_name__ = strchr(file, ':' as i32);
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > file as *const libc::c_char
            && (memchr(
                file as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(file) as libc::c_long as libc::c_ulong,
            ))
                .is_null()
        {
            rmt_open__(
                file,
                0 as libc::c_int | 0 as libc::c_int,
                (1 as libc::c_int) << 30 as libc::c_int,
                rsh_command_option,
            )
        } else {
            open(
                file,
                0 as libc::c_int | 0 as libc::c_int,
                0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int),
            )
        };
    } else if append_flag == 0 {
        fd = if !force_local_option
            && {
                rmt_dev_name__ = strchr(file, ':' as i32);
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > file as *const libc::c_char
            && (memchr(
                file as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(file) as libc::c_long as libc::c_ulong,
            ))
                .is_null()
        {
            rmt_open__(
                file,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int
                    | 0 as libc::c_int,
                (1 as libc::c_int) << 30 as libc::c_int,
                rsh_command_option,
            )
        } else {
            open(
                file,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int
                    | 0 as libc::c_int,
                0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int),
            )
        };
    } else {
        fd = if !force_local_option
            && {
                rmt_dev_name__ = strchr(file, ':' as i32);
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > file as *const libc::c_char
            && (memchr(
                file as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(file) as libc::c_long as libc::c_ulong,
            ))
                .is_null()
        {
            rmt_open__(
                file,
                0o2 as libc::c_int | 0 as libc::c_int,
                (1 as libc::c_int) << 30 as libc::c_int,
                rsh_command_option,
            )
        } else {
            open(
                file,
                0o2 as libc::c_int | 0 as libc::c_int,
                0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                    | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int),
            )
        };
    }
    return fd;
}
pub unsafe extern "C" fn tape_offline(mut tape_des: libc::c_int) {
    let mut control: mtop = mtop { mt_op: 0, mt_count: 0 };
    control.mt_op = 7 as libc::c_int as libc::c_short;
    control.mt_count = 1 as libc::c_int;
    if tape_des >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_ioctl__(
            tape_des - ((1 as libc::c_int) << 30 as libc::c_int),
            ((1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                    + 14 as libc::c_int
                | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<mtop>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut control as *mut mtop as *mut libc::c_char as *mut libc::c_void,
        );
    } else {
        ioctl(
            tape_des,
            ((1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int
                    + 14 as libc::c_int
                | (('m' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<mtop>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut control as *mut mtop as *mut libc::c_char,
        );
    };
}
pub unsafe extern "C" fn get_next_reel(mut tape_des: libc::c_int) {
    static mut reel_number: libc::c_int = 1 as libc::c_int;
    let mut tty_in: *mut FILE = 0 as *mut FILE;
    let mut tty_out: *mut FILE = 0 as *mut FILE;
    let mut old_tape_des: libc::c_int = 0;
    let mut next_archive_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as libc::c_int as size_t,
            ds_idx: 0 as libc::c_int as size_t,
            ds_string: 0 as *mut libc::c_char,
        };
        init
    };
    let mut str_res: *mut libc::c_char = 0 as *mut libc::c_char;
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
    old_tape_des = tape_des;
    tape_offline(tape_des);
    if tape_des >= (1 as libc::c_int) << 30 as libc::c_int {
        rmt_close__(tape_des - ((1 as libc::c_int) << 30 as libc::c_int));
    } else {
        close(tape_des);
    };
    reel_number += 1;
    reel_number;
    if !new_media_message.is_null() {
        fprintf(tty_out, b"%s\0" as *const u8 as *const libc::c_char, new_media_message);
    } else if !new_media_message_with_number.is_null() {
        fprintf(
            tty_out,
            b"%s%d%s\0" as *const u8 as *const libc::c_char,
            new_media_message_with_number,
            reel_number,
            new_media_message_after_number,
        );
    } else if !archive_name.is_null() {
        fprintf(
            tty_out,
            dcgettext(
                0 as *const libc::c_char,
                b"Found end of tape.  Load next tape and press RETURN. \0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        fprintf(
            tty_out,
            dcgettext(
                0 as *const libc::c_char,
                b"Found end of tape.  To continue, type device/file name when ready.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    fflush_unlocked(tty_out);
    if !archive_name.is_null() {
        let mut c: libc::c_int = 0;
        loop {
            c = getc_unlocked(tty_in);
            if !(c != -(1 as libc::c_int) && c != '\n' as i32) {
                break;
            }
        }
        tape_des = open_archive(archive_name);
        if tape_des == -(1 as libc::c_int) {
            open_error(archive_name);
        }
    } else {
        loop {
            if tape_des < 0 as libc::c_int {
                fprintf(
                    tty_out,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"To continue, type device/file name when ready.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                fflush_unlocked(tty_out);
            }
            str_res = ds_fgets(tty_in, &mut new_name);
            if str_res.is_null()
                || *str_res.offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
            {
                exit(2 as libc::c_int);
            }
            next_archive_name = str_res;
            tape_des = open_archive(next_archive_name);
            if tape_des == -(1 as libc::c_int) {
                open_error(next_archive_name);
            }
            if !(tape_des < 0 as libc::c_int) {
                break;
            }
        }
    }
    if tape_des != old_tape_des {
        error(
            2 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"internal error: tape descriptor changed from %d to %d\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            old_tape_des,
            tape_des,
        );
    }
    ds_free(&mut new_name);
    fclose(tty_in);
    fclose(tty_out);
}
pub unsafe extern "C" fn set_new_media_message(mut message: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prev_was_percent: libc::c_int = 0;
    p = message;
    prev_was_percent = 0 as libc::c_int;
    while *p as libc::c_int != '\0' as i32 {
        if *p as libc::c_int == 'd' as i32 && prev_was_percent != 0 {
            break;
        }
        prev_was_percent = (*p as libc::c_int == '%' as i32) as libc::c_int;
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '\0' as i32 {
        new_media_message = xstrdup(message);
    } else {
        let mut length: libc::c_int = (p.offset_from(message) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as libc::c_int;
        new_media_message_with_number = xmalloc((length + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        strncpy(new_media_message_with_number, message, length as libc::c_ulong);
        *new_media_message_with_number
            .offset(length as isize) = '\0' as i32 as libc::c_char;
        length = strlen(p.offset(1 as libc::c_int as isize)) as libc::c_int;
        new_media_message_after_number = xmalloc((length + 1 as libc::c_int) as size_t)
            as *mut libc::c_char;
        strcpy(new_media_message_after_number, p.offset(1 as libc::c_int as isize));
    };
}
unsafe extern "C" fn buf_all_zeros(
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < bufsize {
        let fresh3 = buf;
        buf = buf.offset(1);
        if *fresh3 as libc::c_int != '\0' as i32 {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn sparse_write(
    mut fildes: libc::c_int,
    mut buf: *mut libc::c_char,
    mut nbytes: size_t,
    mut flush: bool,
) -> ssize_t {
    let mut nwritten: size_t = 0 as libc::c_int as size_t;
    let mut n: ssize_t = 0;
    let mut start_ptr: *mut libc::c_char = buf;
    static mut delayed_seek_count: off_t = 0 as libc::c_int as off_t;
    let mut seek_count: off_t = 0 as libc::c_int as off_t;
    let mut state: C2RustUnnamed = (if delayed_seek_count != 0 {
        in_zeros as libc::c_int
    } else {
        begin as libc::c_int
    }) as C2RustUnnamed;
    while nbytes != 0 {
        let mut rest: size_t = nbytes;
        if rest < 512 as libc::c_int as libc::c_ulong {
            state = not_in_zeros;
        } else if buf_all_zeros(buf, rest as libc::c_int) != 0 {
            if state as libc::c_uint == not_in_zeros as libc::c_int as libc::c_uint {
                let mut bytes: ssize_t = (buf.offset_from(start_ptr) as libc::c_long
                    as libc::c_ulong)
                    .wrapping_add(rest) as ssize_t;
                n = write(fildes, start_ptr as *const libc::c_void, bytes as size_t);
                if n == -(1 as libc::c_int) as libc::c_long {
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten = (nwritten as libc::c_ulong).wrapping_add(n as libc::c_ulong)
                    as size_t as size_t;
                if n < bytes {
                    return nwritten.wrapping_add(seek_count as libc::c_ulong) as ssize_t;
                }
                start_ptr = 0 as *mut libc::c_char;
            } else {
                seek_count = (seek_count as libc::c_ulong).wrapping_add(rest) as off_t
                    as off_t;
            }
            state = in_zeros;
        } else {
            seek_count += delayed_seek_count;
            if lseek(fildes, seek_count, 1 as libc::c_int)
                == -(1 as libc::c_int) as libc::c_long
            {
                return -(1 as libc::c_int) as ssize_t;
            }
            seek_count = 0 as libc::c_int as off_t;
            delayed_seek_count = seek_count;
            state = not_in_zeros;
            start_ptr = buf;
        }
        buf = buf.offset(rest as isize);
        nbytes = (nbytes as libc::c_ulong).wrapping_sub(rest) as size_t as size_t;
    }
    if state as libc::c_uint != in_zeros as libc::c_int as libc::c_uint {
        seek_count += delayed_seek_count;
        if seek_count != 0
            && lseek(fildes, seek_count, 1 as libc::c_int)
                == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        seek_count = 0 as libc::c_int as off_t;
        delayed_seek_count = seek_count;
        n = write(
            fildes,
            start_ptr as *const libc::c_void,
            buf.offset_from(start_ptr) as libc::c_long as size_t,
        );
        if n == -(1 as libc::c_int) as libc::c_long {
            return n;
        }
        nwritten = (nwritten as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
            as size_t;
    }
    delayed_seek_count += seek_count;
    if flush as libc::c_int != 0 && delayed_seek_count != 0 {
        if lseek(
            fildes,
            delayed_seek_count - 1 as libc::c_int as libc::c_long,
            1 as libc::c_int,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        n = write(
            fildes,
            b"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        if n != 1 as libc::c_int as libc::c_long {
            return n;
        }
        delayed_seek_count = 0 as libc::c_int as off_t;
    }
    return nwritten.wrapping_add(seek_count as libc::c_ulong) as ssize_t;
}
pub unsafe extern "C" fn stat_to_cpio(mut hdr: *mut cpio_file_stat, mut st: *mut stat) {
    get_inode_and_dev(hdr, st);
    (*hdr).c_mode = (*st).st_mode & 0o7777 as libc::c_int as libc::c_uint;
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_mode |= 0o100000 as libc::c_int as libc::c_uint;
    } else if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_mode |= 0o40000 as libc::c_int as libc::c_uint;
    } else if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_mode |= 0o60000 as libc::c_int as libc::c_uint;
    } else if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o20000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_mode |= 0o20000 as libc::c_int as libc::c_uint;
    } else if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o10000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_mode |= 0o10000 as libc::c_int as libc::c_uint;
    } else if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o120000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_mode |= 0o120000 as libc::c_int as libc::c_uint;
    } else if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_mode |= 0o140000 as libc::c_int as libc::c_uint;
    }
    (*hdr).c_nlink = (*st).st_nlink;
    (*hdr).c_uid = if set_owner_flag != 0 { set_owner } else { (*st).st_uid };
    (*hdr).c_gid = if set_group_flag != 0 { set_group } else { (*st).st_gid };
    if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint
        || (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint
    {
        (*hdr).c_rdev_maj = gnu_dev_major((*st).st_rdev);
        (*hdr).c_rdev_min = gnu_dev_minor((*st).st_rdev);
    } else {
        (*hdr).c_rdev_maj = 0 as libc::c_int as libc::c_uint;
        (*hdr).c_rdev_min = 0 as libc::c_int as libc::c_uint;
    }
    (*hdr).c_mtime = (*st).st_mtim.tv_sec;
    (*hdr).c_filesize = (*st).st_size;
    (*hdr).c_chksum = 0 as libc::c_int as uint32_t;
    (*hdr).c_tar_linkname = 0 as *const libc::c_char;
}
pub unsafe extern "C" fn cpio_to_stat(mut st: *mut stat, mut hdr: *mut cpio_file_stat) {
    memset(
        st as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<stat>() as libc::c_ulong,
    );
    (*st).st_dev = gnu_dev_makedev((*hdr).c_dev_maj, (*hdr).c_dev_min);
    (*st).st_ino = (*hdr).c_ino;
    (*st).st_mode = (*hdr).c_mode & 0o777 as libc::c_int as libc::c_uint;
    if (*hdr).c_mode & 0o100000 as libc::c_int as libc::c_uint != 0 {
        (*st).st_mode |= 0o100000 as libc::c_int as libc::c_uint;
    } else if (*hdr).c_mode & 0o40000 as libc::c_int as libc::c_uint != 0 {
        (*st).st_mode |= 0o40000 as libc::c_int as libc::c_uint;
    } else if (*hdr).c_mode & 0o60000 as libc::c_int as libc::c_uint != 0 {
        (*st).st_mode |= 0o60000 as libc::c_int as libc::c_uint;
    } else if (*hdr).c_mode & 0o20000 as libc::c_int as libc::c_uint != 0 {
        (*st).st_mode |= 0o20000 as libc::c_int as libc::c_uint;
    } else if (*hdr).c_mode & 0o120000 as libc::c_int as libc::c_uint != 0 {
        (*st).st_mode |= 0o120000 as libc::c_int as libc::c_uint;
    } else if (*hdr).c_mode & 0o140000 as libc::c_int as libc::c_uint != 0 {
        (*st).st_mode |= 0o140000 as libc::c_int as libc::c_uint;
    }
    (*st).st_nlink = (*hdr).c_nlink;
    (*st).st_uid = if set_owner_flag != 0 { set_owner } else { (*hdr).c_uid };
    (*st).st_gid = if set_group_flag != 0 { set_group } else { (*hdr).c_gid };
    (*st).st_rdev = gnu_dev_makedev((*hdr).c_rdev_maj, (*hdr).c_rdev_min);
    (*st).st_mtim.tv_sec = (*hdr).c_mtime;
    (*st).st_size = (*hdr).c_filesize;
}
pub unsafe extern "C" fn fchown_or_chown(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut uid: uid_t,
    mut gid: uid_t,
) -> libc::c_int {
    if 1 as libc::c_int != 0 && fd != -(1 as libc::c_int) {
        return fchown(fd, uid, gid)
    } else {
        return chown(name, uid, gid)
    };
}
pub unsafe extern "C" fn fchmod_or_chmod(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    if 1 as libc::c_int != 0 && fd != -(1 as libc::c_int) {
        return fchmod(fd, mode)
    } else {
        return chmod(name, mode)
    };
}
pub unsafe extern "C" fn set_perms(
    mut fd: libc::c_int,
    mut header: *mut cpio_file_stat,
) {
    if no_chown_flag == 0 {
        let mut uid: uid_t = if set_owner_flag != 0 {
            set_owner
        } else {
            (*header).c_uid
        };
        let mut gid: gid_t = if set_group_flag != 0 {
            set_group
        } else {
            (*header).c_gid
        };
        if fchown_or_chown(fd, (*header).c_name, uid, gid) < 0 as libc::c_int
            && *__errno_location() != 1 as libc::c_int
        {
            chown_error_details((*header).c_name, uid, gid);
        }
    }
    if fchmod_or_chmod(fd, (*header).c_name, (*header).c_mode) < 0 as libc::c_int {
        chmod_error_details((*header).c_name, (*header).c_mode);
    }
    if retain_time_flag != 0 {
        set_file_times(
            fd,
            (*header).c_name,
            (*header).c_mtime as libc::c_ulong,
            (*header).c_mtime as libc::c_ulong,
        );
    }
}
pub unsafe extern "C" fn set_file_times(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut atime: libc::c_ulong,
    mut mtime: libc::c_ulong,
) {
    let mut ts: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    memset(
        &mut ts as *mut [timespec; 2] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[timespec; 2]>() as libc::c_ulong,
    );
    ts[0 as libc::c_int as usize].tv_sec = atime as __time_t;
    ts[1 as libc::c_int as usize].tv_sec = mtime as __time_t;
    if fdutimens(fd, name, ts.as_mut_ptr() as *const timespec) < 0 as libc::c_int
        && *__errno_location() != 30 as libc::c_int
    {
        utime_error(name);
    }
}
pub unsafe extern "C" fn cpio_realloc_c_name(
    mut file_hdr: *mut cpio_file_stat,
    mut len: size_t,
) {
    while (*file_hdr).c_name_buflen < len {
        (*file_hdr)
            .c_name = x2realloc(
            (*file_hdr).c_name as *mut libc::c_void,
            &mut (*file_hdr).c_name_buflen,
        ) as *mut libc::c_char;
    }
}
pub unsafe extern "C" fn cpio_set_c_name(
    mut file_hdr: *mut cpio_file_stat,
    mut name: *mut libc::c_char,
) {
    let mut len: size_t = (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    cpio_realloc_c_name(file_hdr, len);
    (*file_hdr).c_namesize = len;
    memmove((*file_hdr).c_name as *mut libc::c_void, name as *const libc::c_void, len);
}
pub unsafe extern "C" fn cpio_safer_name_suffix(
    mut name: *mut libc::c_char,
    mut link_target: bool,
    mut absolute_names: bool,
    mut strip_leading_dots: bool,
) {
    let mut p: *mut libc::c_char = safer_name_suffix(name, link_target, absolute_names);
    if strip_leading_dots as libc::c_int != 0
        && strcmp(p, b"./\0" as *const u8 as *const libc::c_char) != 0
    {
        while *p as libc::c_int == '.' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            p = p.offset(1);
            p;
            while *p as libc::c_int == '/' as i32 {
                p = p.offset(1);
                p;
            }
        }
    }
    if p != name {
        memmove(
            name as *mut libc::c_void,
            p as *const libc::c_void,
            (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
}
static mut delayed_set_stat_head: *mut delayed_set_stat = 0 as *const delayed_set_stat
    as *mut delayed_set_stat;
pub unsafe extern "C" fn delay_cpio_set_stat(
    mut file_stat: *mut cpio_file_stat,
    mut invert_permissions: mode_t,
) {
    let mut file_name_len: size_t = strlen((*file_stat).c_name);
    let mut data: *mut delayed_set_stat = xmalloc(
        (::std::mem::size_of::<delayed_set_stat>() as libc::c_ulong)
            .wrapping_add(file_name_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut delayed_set_stat;
    (*data).next = delayed_set_stat_head;
    memcpy(
        &mut (*data).stat as *mut cpio_file_stat as *mut libc::c_void,
        file_stat as *const libc::c_void,
        ::std::mem::size_of::<cpio_file_stat>() as libc::c_ulong,
    );
    (*data).stat.c_name = data.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    strcpy((*data).stat.c_name, (*file_stat).c_name);
    (*data).invert_permissions = invert_permissions;
    delayed_set_stat_head = data;
}
pub unsafe extern "C" fn delay_set_stat(
    mut file_name: *const libc::c_char,
    mut st: *mut stat,
    mut invert_permissions: mode_t,
) {
    let mut fs: cpio_file_stat = cpio_file_stat {
        c_magic: 0,
        c_ino: 0,
        c_mode: 0,
        c_uid: 0,
        c_gid: 0,
        c_nlink: 0,
        c_mtime: 0,
        c_filesize: 0,
        c_dev_maj: 0,
        c_dev_min: 0,
        c_rdev_maj: 0,
        c_rdev_min: 0,
        c_namesize: 0,
        c_chksum: 0,
        c_name: 0 as *mut libc::c_char,
        c_name_buflen: 0,
        c_tar_linkname: 0 as *const libc::c_char,
    };
    stat_to_cpio(&mut fs, st);
    fs.c_name = file_name as *mut libc::c_char;
    delay_cpio_set_stat(&mut fs, invert_permissions);
}
pub unsafe extern "C" fn repair_inter_delayed_set_stat(
    mut dir_stat_info: *mut stat,
) -> libc::c_int {
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
        if stat((*data).stat.c_name, &mut st) != 0 as libc::c_int {
            stat_error((*data).stat.c_name);
            return -(1 as libc::c_int);
        }
        if st.st_dev == (*dir_stat_info).st_dev && st.st_ino == (*dir_stat_info).st_ino {
            stat_to_cpio(&mut (*data).stat, dir_stat_info);
            (*data)
                .invert_permissions = ((*dir_stat_info).st_mode ^ st.st_mode)
                & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                    | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                    | (0o200 as libc::c_int | 0o200 as libc::c_int >> 3 as libc::c_int
                        | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                        | (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int))) as libc::c_uint & !newdir_umask;
            return 0 as libc::c_int;
        }
        data = (*data).next;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn repair_delayed_set_stat(
    mut file_hdr: *mut cpio_file_stat,
) -> libc::c_int {
    let mut data: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    data = delayed_set_stat_head;
    while !data.is_null() {
        if strcmp((*file_hdr).c_name, (*data).stat.c_name) == 0 as libc::c_int {
            (*data).invert_permissions = 0 as libc::c_int as mode_t;
            memcpy(
                &mut (*data).stat as *mut cpio_file_stat as *mut libc::c_void,
                file_hdr as *const libc::c_void,
                88 as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
        data = (*data).next;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn apply_delayed_set_stat() {
    while !delayed_set_stat_head.is_null() {
        let mut data: *mut delayed_set_stat = delayed_set_stat_head;
        if (*data).invert_permissions != 0 {
            (*data).stat.c_mode ^= (*data).invert_permissions;
        }
        set_perms(-(1 as libc::c_int), &mut (*data).stat);
        delayed_set_stat_head = (*data).next;
        rpl_free(data as *mut libc::c_void);
    }
}
unsafe extern "C" fn cpio_mkdir(
    mut file_hdr: *mut cpio_file_stat,
    mut setstat_delayed: *mut libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut mode: mode_t = (*file_hdr).c_mode;
    if (*file_hdr).c_mode & 0o200 as libc::c_int as libc::c_uint == 0 {
        rc = mkdir((*file_hdr).c_name, mode | 0o200 as libc::c_int as libc::c_uint);
        if rc == 0 as libc::c_int {
            delay_cpio_set_stat(file_hdr, 0 as libc::c_int as mode_t);
            *setstat_delayed = 1 as libc::c_int;
        }
    } else {
        rc = mkdir((*file_hdr).c_name, mode);
        *setstat_delayed = 0 as libc::c_int;
    }
    return rc;
}
pub unsafe extern "C" fn cpio_create_dir(
    mut file_hdr: *mut cpio_file_stat,
    mut existing_dir: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut setstat_delayed: libc::c_int = 0 as libc::c_int;
    if to_stdout_option {
        return 0 as libc::c_int;
    }
    strip_trailing_slashes((*file_hdr).c_name);
    if *((*file_hdr).c_name).offset(0 as libc::c_int as isize) as libc::c_int
        == '.' as i32
        && *((*file_hdr).c_name).offset(1 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        return 0 as libc::c_int;
    }
    if existing_dir == 0 {
        res = cpio_mkdir(file_hdr, &mut setstat_delayed);
    } else {
        res = 0 as libc::c_int;
    }
    if res < 0 as libc::c_int && create_dir_flag != 0 {
        create_all_directories((*file_hdr).c_name);
        res = cpio_mkdir(file_hdr, &mut setstat_delayed);
    }
    if res < 0 as libc::c_int {
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
        if *__errno_location() != 17 as libc::c_int {
            mkdir_error((*file_hdr).c_name);
            return -(1 as libc::c_int);
        }
        if lstat((*file_hdr).c_name, &mut file_stat) != 0 {
            stat_error((*file_hdr).c_name);
            return -(1 as libc::c_int);
        }
        if !(file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint)
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is not a directory\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon((*file_hdr).c_name),
            );
            return -(1 as libc::c_int);
        }
    }
    if setstat_delayed == 0 && repair_delayed_set_stat(file_hdr) != 0 {
        set_perms(-(1 as libc::c_int), file_hdr);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn change_dir() {
    if !change_directory_option.is_null() && chdir(change_directory_option) != 0 {
        if *__errno_location() == 2 as libc::c_int && create_dir_flag != 0 {
            if make_path(
                change_directory_option,
                -(1 as libc::c_int) as uid_t,
                -(1 as libc::c_int) as gid_t,
                if warn_option & 0x2 as libc::c_int as libc::c_uint != 0 {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Creating directory `%s'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    0 as *mut libc::c_char
                },
            ) != 0
            {
                exit(2 as libc::c_int);
            }
            if chdir(change_directory_option) == 0 as libc::c_int {
                return;
            }
        }
        error(
            2 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot change to directory `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            change_directory_option,
        );
    }
}
pub unsafe extern "C" fn arf_stores_inode_p(mut arf: archive_format) -> libc::c_int {
    match arf as libc::c_uint {
        5 | 6 => return 0 as libc::c_int,
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn cpio_file_stat_init(mut file_hdr: *mut cpio_file_stat) {
    memset(
        file_hdr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cpio_file_stat>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn cpio_file_stat_free(mut file_hdr: *mut cpio_file_stat) {
    rpl_free((*file_hdr).c_name as *mut libc::c_void);
    cpio_file_stat_init(file_hdr);
}
