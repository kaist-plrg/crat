use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type inode_val;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn __xmknod(
        __ver: libc::c_int,
        __path: *const libc::c_char,
        __mode: __mode_t,
        __dev: *mut __dev_t,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn lchown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn chown_error_details(name: *const libc::c_char, uid: uid_t, gid: gid_t);
    fn close_error(_: *const libc::c_char);
    fn mknod_error(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn readlink_error(_: *const libc::c_char);
    fn stat_error(_: *const libc::c_char);
    fn symlink_error(_: *const libc::c_char, _: *const libc::c_char);
    fn ds_free(string: *mut dynamic_string);
    fn ds_reset(s: *mut dynamic_string, len: size_t);
    fn ds_fgetstr(
        f: *mut FILE,
        s: *mut dynamic_string,
        eos: libc::c_char,
    ) -> *mut libc::c_char;
    fn ds_append(s: *mut dynamic_string, c: libc::c_int);
    fn ds_concat(s: *mut dynamic_string, str: *const libc::c_char);
    static mut reset_time_flag: libc::c_int;
    static mut io_block_size: libc::c_int;
    static mut create_dir_flag: libc::c_int;
    static mut unconditional_flag: libc::c_int;
    static mut verbose_flag: libc::c_int;
    static mut dot_flag: libc::c_int;
    static mut link_flag: libc::c_int;
    static mut set_owner_flag: libc::c_int;
    static mut set_owner: uid_t;
    static mut set_group_flag: libc::c_int;
    static mut set_group: gid_t;
    static mut no_chown_flag: libc::c_int;
    static mut quiet_flag: libc::c_int;
    static mut newdir_umask: mode_t;
    static mut output_bytes: off_t;
    static mut directory_name: *mut libc::c_char;
    static mut name_end: libc::c_char;
    static mut output_is_seekable: libc::c_char;
    static mut xstat: Option::<unsafe extern "C" fn() -> libc::c_int>;
    static mut change_directory_option: *mut libc::c_char;
    fn apply_delayed_set_stat();
    fn create_all_directories(name: *const libc::c_char);
    fn set_perms(fd: libc::c_int, header: *mut cpio_file_stat);
    fn stat_to_cpio(hdr: *mut cpio_file_stat, st: *mut stat);
    fn add_inode(
        node_num: ino_t,
        file_name: *mut libc::c_char,
        major_num: libc::c_ulong,
        minor_num: libc::c_ulong,
    ) -> *mut inode_val;
    fn find_inode_file(
        node_num: ino_t,
        major_num: libc::c_ulong,
        minor_num: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn cpio_create_dir(
        file_hdr: *mut cpio_file_stat,
        existing_dir: libc::c_int,
    ) -> libc::c_int;
    fn warn_if_file_changed(
        file_name: *mut libc::c_char,
        old_file_size: off_t,
        old_file_mtime: time_t,
    );
    fn set_file_times(
        fd: libc::c_int,
        name: *const libc::c_char,
        atime: libc::c_ulong,
        mtime: libc::c_ulong,
    );
    fn disk_empty_output_buffer(out_des: libc::c_int, flush: bool);
    fn copy_files_disk_to_disk(
        in_des: libc::c_int,
        out_des: libc::c_int,
        num_bytes: off_t,
        filename: *mut libc::c_char,
    );
    fn change_dir();
    fn xgetcwd() -> *mut libc::c_char;
}
pub type __uint32_t = libc::c_uint;
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
unsafe extern "C" fn set_copypass_perms(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut st: *mut stat,
) {
    let mut header: cpio_file_stat = cpio_file_stat {
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
    header.c_name = name as *mut libc::c_char;
    stat_to_cpio(&mut header, st);
    set_perms(fd, &mut header);
}
pub unsafe extern "C" fn process_copy_pass() {
    let mut input_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as libc::c_int as size_t,
            ds_idx: 0 as libc::c_int as size_t,
            ds_string: 0 as *mut libc::c_char,
        };
        init
    };
    let mut output_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as libc::c_int as size_t,
            ds_idx: 0 as libc::c_int as size_t,
            ds_string: 0 as *mut libc::c_char,
        };
        init
    };
    let mut dirname_len: size_t = 0;
    let mut res: libc::c_int = 0;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_file_stat: stat = stat {
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
    let mut out_file_stat: stat = stat {
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
    let mut in_file_des: libc::c_int = 0;
    let mut out_file_des: libc::c_int = 0;
    let mut existing_dir: libc::c_int = 0;
    newdir_umask = umask(0 as libc::c_int as __mode_t);
    dirname_len = strlen(directory_name);
    if !change_directory_option.is_null()
        && !(*directory_name.offset(0 as libc::c_int as isize) as libc::c_int
            == '/' as i32)
    {
        let mut pwd: *mut libc::c_char = xgetcwd();
        ds_concat(&mut output_name, pwd);
        ds_append(&mut output_name, '/' as i32);
    }
    ds_concat(&mut output_name, directory_name);
    ds_append(&mut output_name, '/' as i32);
    dirname_len = output_name.ds_idx;
    output_is_seekable = 1 as libc::c_int as libc::c_char;
    change_dir();
    while !(ds_fgetstr(stdin, &mut input_name, name_end)).is_null() {
        let mut link_res: libc::c_int = -(1 as libc::c_int);
        if *(input_name.ds_string).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
        {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"blank line ignored\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            if *(input_name.ds_string).offset(0 as libc::c_int as isize) as libc::c_int
                == '.' as i32
                && (*(input_name.ds_string).offset(1 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                    || *(input_name.ds_string).offset(1 as libc::c_int as isize)
                        as libc::c_int == '/' as i32
                        && *(input_name.ds_string).offset(2 as libc::c_int as isize)
                            as libc::c_int == '\0' as i32)
            {
                continue;
            }
            if ::std::mem::transmute::<
                _,
                fn(_, _) -> libc::c_int,
            >((Some(xstat.unwrap())).unwrap())(input_name.ds_string, &mut in_file_stat)
                < 0 as libc::c_int
            {
                stat_error(input_name.ds_string);
            } else {
                slash = input_name.ds_string;
                while *slash as libc::c_int == '/' as i32 {
                    slash = slash.offset(1);
                    slash;
                }
                ds_reset(&mut output_name, dirname_len);
                ds_concat(&mut output_name, slash);
                existing_dir = 0 as libc::c_int;
                if lstat(output_name.ds_string, &mut out_file_stat) == 0 as libc::c_int {
                    if out_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                        && in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                            == 0o40000 as libc::c_int as libc::c_uint
                    {
                        existing_dir = 1 as libc::c_int;
                    } else if unconditional_flag == 0
                        && in_file_stat.st_mtim.tv_sec <= out_file_stat.st_mtim.tv_sec
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s not created: newer or same age version exists\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            output_name.ds_string,
                        );
                        continue;
                    } else if if out_file_stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    {
                        rmdir(output_name.ds_string)
                    } else {
                        unlink(output_name.ds_string)
                    } != 0
                    {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"cannot remove current %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            output_name.ds_string,
                        );
                        continue;
                    }
                }
                if in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                {
                    if link_flag != 0 {
                        link_res = link_to_name(
                            output_name.ds_string,
                            input_name.ds_string,
                        );
                    }
                    if link_res < 0 as libc::c_int
                        && in_file_stat.st_nlink > 1 as libc::c_int as libc::c_ulong
                    {
                        link_res = link_to_maj_min_ino(
                            output_name.ds_string,
                            gnu_dev_major(in_file_stat.st_dev) as libc::c_int,
                            gnu_dev_minor(in_file_stat.st_dev) as libc::c_int,
                            in_file_stat.st_ino,
                        );
                    }
                    if link_res < 0 as libc::c_int {
                        in_file_des = open(
                            input_name.ds_string,
                            0 as libc::c_int | 0 as libc::c_int,
                            0 as libc::c_int,
                        );
                        if in_file_des < 0 as libc::c_int {
                            open_error(input_name.ds_string);
                            continue;
                        } else {
                            out_file_des = open(
                                output_name.ds_string,
                                0o100 as libc::c_int | 0o1 as libc::c_int
                                    | 0 as libc::c_int,
                                0o600 as libc::c_int,
                            );
                            if out_file_des < 0 as libc::c_int && create_dir_flag != 0 {
                                create_all_directories(output_name.ds_string);
                                out_file_des = open(
                                    output_name.ds_string,
                                    0o100 as libc::c_int | 0o1 as libc::c_int
                                        | 0 as libc::c_int,
                                    0o600 as libc::c_int,
                                );
                            }
                            if out_file_des < 0 as libc::c_int {
                                open_error(output_name.ds_string);
                                close(in_file_des);
                                continue;
                            } else {
                                copy_files_disk_to_disk(
                                    in_file_des,
                                    out_file_des,
                                    in_file_stat.st_size,
                                    input_name.ds_string,
                                );
                                disk_empty_output_buffer(
                                    out_file_des,
                                    1 as libc::c_int != 0,
                                );
                                set_copypass_perms(
                                    out_file_des,
                                    output_name.ds_string,
                                    &mut in_file_stat,
                                );
                                if reset_time_flag != 0 {
                                    set_file_times(
                                        in_file_des,
                                        input_name.ds_string,
                                        in_file_stat.st_atim.tv_sec as libc::c_ulong,
                                        in_file_stat.st_mtim.tv_sec as libc::c_ulong,
                                    );
                                    set_file_times(
                                        out_file_des,
                                        output_name.ds_string,
                                        in_file_stat.st_atim.tv_sec as libc::c_ulong,
                                        in_file_stat.st_mtim.tv_sec as libc::c_ulong,
                                    );
                                }
                                if close(in_file_des) < 0 as libc::c_int {
                                    close_error(input_name.ds_string);
                                }
                                if close(out_file_des) < 0 as libc::c_int {
                                    close_error(output_name.ds_string);
                                }
                                warn_if_file_changed(
                                    input_name.ds_string,
                                    in_file_stat.st_size,
                                    in_file_stat.st_mtim.tv_sec,
                                );
                            }
                        }
                    }
                } else if in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint
                {
                    let mut file_stat: cpio_file_stat = cpio_file_stat {
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
                    stat_to_cpio(&mut file_stat, &mut in_file_stat);
                    file_stat.c_name = output_name.ds_string;
                    cpio_create_dir(&mut file_stat, existing_dir);
                } else if in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o20000 as libc::c_int as libc::c_uint
                    || in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o60000 as libc::c_int as libc::c_uint
                    || in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o10000 as libc::c_int as libc::c_uint
                    || in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o140000 as libc::c_int as libc::c_uint
                    || 0 as libc::c_int != 0
                {
                    if link_flag != 0 {
                        link_res = link_to_name(
                            output_name.ds_string,
                            input_name.ds_string,
                        );
                    }
                    if link_res < 0 as libc::c_int
                        && in_file_stat.st_nlink > 1 as libc::c_int as libc::c_ulong
                    {
                        link_res = link_to_maj_min_ino(
                            output_name.ds_string,
                            gnu_dev_major(in_file_stat.st_dev) as libc::c_int,
                            gnu_dev_minor(in_file_stat.st_dev) as libc::c_int,
                            in_file_stat.st_ino,
                        );
                    }
                    if link_res < 0 as libc::c_int {
                        res = mknod(
                            output_name.ds_string,
                            in_file_stat.st_mode,
                            in_file_stat.st_rdev,
                        );
                        if res < 0 as libc::c_int && create_dir_flag != 0 {
                            create_all_directories(output_name.ds_string);
                            res = mknod(
                                output_name.ds_string,
                                in_file_stat.st_mode,
                                in_file_stat.st_rdev,
                            );
                        }
                        if res < 0 as libc::c_int {
                            mknod_error(output_name.ds_string);
                            continue;
                        } else {
                            set_copypass_perms(
                                -(1 as libc::c_int),
                                output_name.ds_string,
                                &mut in_file_stat,
                            );
                        }
                    }
                } else if in_file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint
                {
                    let mut link_name: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut link_size: libc::c_int = 0;
                    link_name = xmalloc(
                        (in_file_stat.st_size as libc::c_uint)
                            .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                    ) as *mut libc::c_char;
                    link_size = readlink(
                        input_name.ds_string,
                        link_name,
                        in_file_stat.st_size as size_t,
                    ) as libc::c_int;
                    if link_size < 0 as libc::c_int {
                        readlink_error(input_name.ds_string);
                        rpl_free(link_name as *mut libc::c_void);
                        continue;
                    } else {
                        *link_name
                            .offset(link_size as isize) = '\0' as i32 as libc::c_char;
                        res = symlink(link_name, output_name.ds_string);
                        if res < 0 as libc::c_int && create_dir_flag != 0 {
                            create_all_directories(output_name.ds_string);
                            res = symlink(link_name, output_name.ds_string);
                        }
                        if res < 0 as libc::c_int {
                            symlink_error(output_name.ds_string, link_name);
                            rpl_free(link_name as *mut libc::c_void);
                            continue;
                        } else {
                            if no_chown_flag == 0 {
                                let mut uid: uid_t = if set_owner_flag != 0 {
                                    set_owner
                                } else {
                                    in_file_stat.st_uid
                                };
                                let mut gid: gid_t = if set_group_flag != 0 {
                                    set_group
                                } else {
                                    in_file_stat.st_gid
                                };
                                if lchown(output_name.ds_string, uid, gid)
                                    < 0 as libc::c_int
                                    && *__errno_location() != 1 as libc::c_int
                                {
                                    chown_error_details(output_name.ds_string, uid, gid);
                                }
                            }
                            rpl_free(link_name as *mut libc::c_void);
                        }
                    }
                } else {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: unknown file type\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        input_name.ds_string,
                    );
                }
                if verbose_flag != 0 {
                    fprintf(
                        stderr,
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        output_name.ds_string,
                    );
                }
                if dot_flag != 0 {
                    fputc_unlocked('.' as i32, stderr);
                }
            }
        }
    }
    if dot_flag != 0 {
        fputc_unlocked('\n' as i32, stderr);
    }
    apply_delayed_set_stat();
    if quiet_flag == 0 {
        let mut blocks: size_t = ((output_bytes + io_block_size as libc::c_long
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
    ds_free(&mut input_name);
    ds_free(&mut output_name);
}
pub unsafe extern "C" fn link_to_maj_min_ino(
    mut file_name: *mut libc::c_char,
    mut st_dev_maj: libc::c_int,
    mut st_dev_min: libc::c_int,
    mut st_ino: ino_t,
) -> libc::c_int {
    let mut link_res: libc::c_int = 0;
    let mut link_name: *mut libc::c_char = 0 as *mut libc::c_char;
    link_res = -(1 as libc::c_int);
    link_name = find_inode_file(
        st_ino,
        st_dev_maj as libc::c_ulong,
        st_dev_min as libc::c_ulong,
    );
    if link_name.is_null() {
        add_inode(
            st_ino,
            file_name,
            st_dev_maj as libc::c_ulong,
            st_dev_min as libc::c_ulong,
        );
    } else {
        link_res = link_to_name(file_name, link_name);
    }
    return link_res;
}
pub unsafe extern "C" fn link_to_name(
    mut link_name: *const libc::c_char,
    mut link_target: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = link(link_target, link_name);
    if res < 0 as libc::c_int && create_dir_flag != 0 {
        create_all_directories(link_name);
        res = link(link_target, link_name);
    }
    if res == 0 as libc::c_int {
        if verbose_flag != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s linked to %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                link_target,
                link_name,
            );
        }
    } else if link_flag != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot link %s to %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            link_target,
            link_name,
        );
    }
    return res;
}
