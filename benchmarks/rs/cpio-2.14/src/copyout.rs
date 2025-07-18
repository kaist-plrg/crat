use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type inode_val;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn abort() -> !;
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
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
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
    fn close_error(_: *const libc::c_char);
    fn open_error(_: *const libc::c_char);
    fn readlink_warn(_: *const libc::c_char);
    fn stat_error(_: *const libc::c_char);
    fn cpio_file_stat_free(file_hdr: *mut cpio_file_stat);
    fn cpio_set_c_name(file_hdr: *mut cpio_file_stat, name: *mut libc::c_char);
    fn ds_free(string: *mut dynamic_string);
    fn ds_fgetstr(
        f: *mut FILE,
        s: *mut dynamic_string,
        eos: libc::c_char,
    ) -> *mut libc::c_char;
    fn ds_append(s: *mut dynamic_string, c: libc::c_int);
    fn ds_endswith(s: *mut dynamic_string, c: libc::c_int) -> libc::c_int;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut archive_format: archive_format;
    static mut reset_time_flag: libc::c_int;
    static mut io_block_size: libc::c_int;
    static mut verbose_flag: libc::c_int;
    static mut dot_flag: libc::c_int;
    static mut append_flag: libc::c_int;
    static mut quiet_flag: libc::c_int;
    static mut no_abs_paths_flag: libc::c_int;
    static mut warn_option: libc::c_uint;
    static mut ignore_dirnlink_option: libc::c_int;
    static mut archive_des: libc::c_int;
    static mut output_size: size_t;
    fn process_copy_in();
    static mut output_bytes: off_t;
    static mut output_is_special: libc::c_char;
    static mut name_end: libc::c_char;
    static mut output_is_seekable: libc::c_char;
    static mut xstat: Option::<unsafe extern "C" fn() -> libc::c_int>;
    fn copy_files_disk_to_tape(
        in_des: libc::c_int,
        out_des: libc::c_int,
        num_bytes: off_t,
        filename: *mut libc::c_char,
    );
    fn tape_empty_output_buffer(out_des: libc::c_int);
    fn is_tar_filename_too_long(name: *mut libc::c_char) -> libc::c_int;
    fn tape_buffered_write(
        in_buf: *mut libc::c_char,
        out_des: libc::c_int,
        num_bytes: off_t,
    );
    fn write_out_tar_header(
        file_hdr: *mut cpio_file_stat,
        out_des: libc::c_int,
    ) -> libc::c_int;
    fn add_inode(
        node_num: ino_t,
        file_name: *mut libc::c_char,
        major_num: libc::c_ulong,
        minor_num: libc::c_ulong,
    ) -> *mut inode_val;
    fn prepare_append(out_file_des: libc::c_int);
    fn warn_if_file_changed(
        file_name: *mut libc::c_char,
        old_file_size: off_t,
        old_file_mtime: time_t,
    );
    fn find_inode_file(
        node_num: ino_t,
        major_num: libc::c_ulong,
        minor_num: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn stat_to_cpio(hdr: *mut cpio_file_stat, st: *mut stat);
    fn cpio_safer_name_suffix(
        name: *mut libc::c_char,
        link_target: bool,
        absolute_names: bool,
        strip_leading_dots: bool,
    );
    fn change_dir();
    fn write_nuls_to_file(
        num_bytes: off_t,
        out_des: libc::c_int,
        writer: Option::<
            unsafe extern "C" fn(*mut libc::c_char, libc::c_int, off_t) -> (),
        >,
    );
    fn set_file_times(
        fd: libc::c_int,
        name: *const libc::c_char,
        atime: libc::c_ulong,
        mtime: libc::c_ulong,
    );
    fn create_deferment(file_hdr: *mut cpio_file_stat) -> *mut deferment;
    fn free_deferment(d: *mut deferment);
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
pub type dev_t = __dev_t;
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
pub struct deferment {
    pub next: *mut deferment,
    pub header: cpio_file_stat,
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
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
unsafe extern "C" fn read_for_checksum(
    mut in_file_des: libc::c_int,
    mut file_size: off_t,
    mut file_name: *mut libc::c_char,
) -> uint32_t {
    let mut crc: uint32_t = 0;
    let mut buf: [libc::c_uchar; 8192] = [0; 8192];
    let mut bytes_read: ssize_t = 0;
    let mut i: ssize_t = 0;
    crc = 0 as libc::c_int as uint32_t;
    while file_size > 0 as libc::c_int as libc::c_long {
        bytes_read = read(
            in_file_des,
            buf.as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if bytes_read < 0 as libc::c_int as libc::c_long {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot read checksum for %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_name,
            );
        }
        if bytes_read == 0 as libc::c_int as libc::c_long {
            break;
        }
        i = 0 as libc::c_int as ssize_t;
        while i < bytes_read {
            crc = (crc as libc::c_uint)
                .wrapping_add(
                    (buf[i as usize] as libc::c_int & 0xff as libc::c_int)
                        as libc::c_uint,
                ) as uint32_t as uint32_t;
            i += 1;
            i;
        }
        file_size -= bytes_read;
    }
    if lseek(in_file_des, 0 as libc::c_long, 0 as libc::c_int) != 0 {
        error(
            2 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot read checksum for %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file_name,
        );
    }
    return crc;
}
unsafe extern "C" fn tape_clear_rest_of_block(mut out_file_des: libc::c_int) {
    write_nuls_to_file(
        (io_block_size as libc::c_ulong).wrapping_sub(output_size) as off_t,
        out_file_des,
        Some(
            tape_buffered_write
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_int, off_t) -> (),
        ),
    );
}
unsafe extern "C" fn tape_pad_output(
    mut out_file_des: libc::c_int,
    mut offset: libc::c_int,
) {
    let mut pad: size_t = 0;
    if archive_format as libc::c_uint == arf_newascii as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_crcascii as libc::c_int as libc::c_uint
    {
        pad = ((4 as libc::c_int - offset % 4 as libc::c_int) % 4 as libc::c_int)
            as size_t;
    } else if archive_format as libc::c_uint == arf_tar as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint
    {
        pad = ((512 as libc::c_int - offset % 512 as libc::c_int) % 512 as libc::c_int)
            as size_t;
    } else if archive_format as libc::c_uint
        != arf_oldascii as libc::c_int as libc::c_uint
        && archive_format as libc::c_uint
            != arf_hpoldascii as libc::c_int as libc::c_uint
    {
        pad = ((2 as libc::c_int - offset % 2 as libc::c_int) % 2 as libc::c_int)
            as size_t;
    } else {
        pad = 0 as libc::c_int as size_t;
    }
    if pad != 0 as libc::c_int as libc::c_ulong {
        write_nuls_to_file(
            pad as off_t,
            out_file_des,
            Some(
                tape_buffered_write
                    as unsafe extern "C" fn(*mut libc::c_char, libc::c_int, off_t) -> (),
            ),
        );
    }
}
pub static mut deferouts: *mut deferment = 0 as *const deferment as *mut deferment;
unsafe extern "C" fn count_defered_links_to_dev_ino(
    mut file_hdr: *mut cpio_file_stat,
) -> size_t {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut ino: ino_t = (*file_hdr).c_ino;
    let mut maj: libc::c_long = (*file_hdr).c_dev_maj as libc::c_long;
    let mut min: libc::c_long = (*file_hdr).c_dev_min as libc::c_long;
    let mut count: size_t = 0 as libc::c_int as size_t;
    d = deferouts;
    while !d.is_null() {
        if (*d).header.c_ino == ino && (*d).header.c_dev_maj as libc::c_long == maj
            && (*d).header.c_dev_min as libc::c_long == min
        {
            count = count.wrapping_add(1);
            count;
        }
        d = (*d).next;
    }
    return count;
}
unsafe extern "C" fn last_link(mut file_hdr: *mut cpio_file_stat) -> libc::c_int {
    return ((*file_hdr).c_nlink
        == (count_defered_links_to_dev_ino(file_hdr))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
}
unsafe extern "C" fn add_link_defer(mut file_hdr: *mut cpio_file_stat) {
    let mut d: *mut deferment = 0 as *mut deferment;
    d = create_deferment(file_hdr);
    (*d).next = deferouts;
    deferouts = d;
}
unsafe extern "C" fn writeout_other_defers(
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: libc::c_int,
) {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut d_prev: *mut deferment = 0 as *mut deferment;
    let mut ino: ino_t = 0;
    let mut maj: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    ino = (*file_hdr).c_ino;
    maj = (*file_hdr).c_dev_maj as libc::c_int;
    min = (*file_hdr).c_dev_min as libc::c_int;
    d_prev = 0 as *mut deferment;
    d = deferouts;
    while !d.is_null() {
        if (*d).header.c_ino == ino && (*d).header.c_dev_maj == maj as libc::c_uint
            && (*d).header.c_dev_min == min as libc::c_uint
        {
            let mut d_free: *mut deferment = 0 as *mut deferment;
            (*d).header.c_filesize = 0 as libc::c_int as off_t;
            write_out_header(&mut (*d).header, out_des);
            if !d_prev.is_null() {
                (*d_prev).next = (*d).next;
            } else {
                deferouts = (*d).next;
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
unsafe extern "C" fn writeout_defered_file(
    mut header: *mut cpio_file_stat,
    mut out_file_des: libc::c_int,
) {
    let mut in_file_des: libc::c_int = 0;
    let mut file_hdr: cpio_file_stat = cpio_file_stat {
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
    file_hdr = *header;
    in_file_des = open(
        (*header).c_name,
        0 as libc::c_int | 0 as libc::c_int,
        0 as libc::c_int,
    );
    if in_file_des < 0 as libc::c_int {
        open_error((*header).c_name);
        return;
    }
    if archive_format as libc::c_uint == arf_crcascii as libc::c_int as libc::c_uint {
        file_hdr
            .c_chksum = read_for_checksum(
            in_file_des,
            file_hdr.c_filesize,
            (*header).c_name,
        );
    }
    if write_out_header(&mut file_hdr, out_file_des) != 0 {
        return;
    }
    copy_files_disk_to_tape(
        in_file_des,
        out_file_des,
        file_hdr.c_filesize,
        (*header).c_name,
    );
    warn_if_file_changed((*header).c_name, file_hdr.c_filesize, file_hdr.c_mtime);
    if archive_format as libc::c_uint == arf_tar as libc::c_int as libc::c_uint
        || archive_format as libc::c_uint == arf_ustar as libc::c_int as libc::c_uint
    {
        add_inode(
            file_hdr.c_ino,
            file_hdr.c_name,
            file_hdr.c_dev_maj as libc::c_ulong,
            file_hdr.c_dev_min as libc::c_ulong,
        );
    }
    tape_pad_output(out_file_des, file_hdr.c_filesize as libc::c_int);
    if reset_time_flag != 0 {
        set_file_times(
            in_file_des,
            file_hdr.c_name,
            file_hdr.c_mtime as libc::c_ulong,
            file_hdr.c_mtime as libc::c_ulong,
        );
    }
    if close(in_file_des) < 0 as libc::c_int {
        close_error((*header).c_name);
    }
}
unsafe extern "C" fn writeout_final_defers(mut out_des: libc::c_int) {
    let mut d: *mut deferment = 0 as *mut deferment;
    let mut other_count: libc::c_int = 0;
    while !deferouts.is_null() {
        d = deferouts;
        other_count = count_defered_links_to_dev_ino(&mut (*d).header) as libc::c_int;
        if other_count == 1 as libc::c_int {
            writeout_defered_file(&mut (*d).header, out_des);
        } else {
            let mut file_hdr: cpio_file_stat = cpio_file_stat {
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
            file_hdr = (*d).header;
            file_hdr.c_filesize = 0 as libc::c_int as off_t;
            write_out_header(&mut file_hdr, out_des);
        }
        deferouts = (*deferouts).next;
    }
}
pub unsafe extern "C" fn to_ascii(
    mut where_0: *mut libc::c_char,
    mut v: uintmax_t,
    mut digits: size_t,
    mut logbase: libc::c_uint,
    mut nul: bool,
) -> libc::c_int {
    static mut codetab: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<
            &[u8; 17],
            &mut [libc::c_char; 17],
        >(b"0123456789ABCDEF\0")
    };
    if nul {
        digits = digits.wrapping_sub(1);
        *where_0.offset(digits as isize) = 0 as libc::c_int as libc::c_char;
    }
    while digits > 0 as libc::c_int as libc::c_ulong {
        digits = digits.wrapping_sub(1);
        *where_0
            .offset(
                digits as isize,
            ) = codetab[(v
            & (((1 as libc::c_int) << logbase) - 1 as libc::c_int) as libc::c_ulong)
            as usize];
        v >>= logbase;
    }
    return (v != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn field_width_error(
    mut filename: *const libc::c_char,
    mut fieldname: *const libc::c_char,
    mut value: uintmax_t,
    mut width: size_t,
    mut nul: bool,
) {
    let mut valbuf: [libc::c_char; 22] = [0; 22];
    let mut maxbuf: [libc::c_char; 22] = [0; 22];
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: value %s %s out of allowed range 0..%s\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        filename,
        fieldname,
        umaxtostr(value, valbuf.as_mut_ptr()),
        umaxtostr(
            if width
                .wrapping_sub(nul as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                < (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            {
                ((1 as libc::c_int as uintmax_t)
                    << width
                        .wrapping_sub(nul as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            } else {
                -(1 as libc::c_int) as uintmax_t
            },
            maxbuf.as_mut_ptr(),
        ),
    );
}
unsafe extern "C" fn field_width_warning(
    mut filename: *const libc::c_char,
    mut fieldname: *const libc::c_char,
) {
    if warn_option & 0x1 as libc::c_int as libc::c_uint != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: truncating %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            fieldname,
        );
    }
}
pub unsafe extern "C" fn to_ascii_or_warn(
    mut where_0: *mut libc::c_char,
    mut n: uintmax_t,
    mut digits: size_t,
    mut logbase: libc::c_uint,
    mut filename: *const libc::c_char,
    mut fieldname: *const libc::c_char,
) {
    if to_ascii(where_0, n, digits, logbase, 0 as libc::c_int != 0) != 0 {
        field_width_warning(filename, fieldname);
    }
}
pub unsafe extern "C" fn to_ascii_or_error(
    mut where_0: *mut libc::c_char,
    mut n: uintmax_t,
    mut digits: size_t,
    mut logbase: libc::c_uint,
    mut filename: *const libc::c_char,
    mut fieldname: *const libc::c_char,
) -> libc::c_int {
    if to_ascii(where_0, n, digits, logbase, 0 as libc::c_int != 0) != 0 {
        field_width_error(filename, fieldname, n, digits, 0 as libc::c_int != 0);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn write_out_new_ascii_header(
    mut magic_string: *const libc::c_char,
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: libc::c_int,
) -> libc::c_int {
    let mut ascii_header: [libc::c_char; 110] = [0; 110];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = stpcpy(ascii_header.as_mut_ptr(), magic_string);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_ino,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"inode number\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(8 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mode as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"file mode\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(8 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_uid as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"uid\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(8 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_gid as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"gid\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(8 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_nlink,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"number of links\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(8 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mtime as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"modification time\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(8 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_filesize as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"file size\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    p = p.offset(8 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_dev_maj as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"device major number\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    p = p.offset(8 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_dev_min as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"device minor number\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    p = p.offset(8 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_rdev_maj as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"rdev major\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    p = p.offset(8 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_rdev_min as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"rdev minor\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    p = p.offset(8 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_namesize,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"name size\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    p = p.offset(8 as libc::c_int as isize);
    to_ascii(
        p,
        ((*file_hdr).c_chksum & 0xffffffff as libc::c_uint) as uintmax_t,
        8 as libc::c_int as size_t,
        4 as libc::c_int as libc::c_uint,
        0 as libc::c_int != 0,
    );
    tape_buffered_write(
        ascii_header.as_mut_ptr(),
        out_des,
        ::std::mem::size_of::<[libc::c_char; 110]>() as libc::c_ulong as off_t,
    );
    tape_buffered_write(
        (*file_hdr).c_name,
        out_des,
        (*file_hdr).c_namesize as libc::c_long,
    );
    tape_pad_output(
        out_des,
        ((*file_hdr).c_namesize)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 110]>() as libc::c_ulong)
            as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn write_out_old_ascii_header(
    mut dev: dev_t,
    mut rdev: dev_t,
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: libc::c_int,
) -> libc::c_int {
    let mut ascii_header: [libc::c_char; 76] = [0; 76];
    let mut p: *mut libc::c_char = ascii_header.as_mut_ptr();
    to_ascii(
        p,
        (*file_hdr).c_magic as uintmax_t,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        0 as libc::c_int != 0,
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        dev,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"device number\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_ino,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"inode number\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mode as uintmax_t,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"file mode\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_uid as uintmax_t,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"uid\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_gid as uintmax_t,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"gid\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_nlink,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"number of links\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        rdev,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"rdev\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(6 as libc::c_int as isize);
    to_ascii_or_warn(
        p,
        (*file_hdr).c_mtime as uintmax_t,
        11 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"modification time\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    p = p.offset(11 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_namesize,
        6 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"name size\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    p = p.offset(6 as libc::c_int as isize);
    if to_ascii_or_error(
        p,
        (*file_hdr).c_filesize as uintmax_t,
        11 as libc::c_int as size_t,
        3 as libc::c_int as libc::c_uint,
        (*file_hdr).c_name,
        dcgettext(
            0 as *const libc::c_char,
            b"file size\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    ) != 0
    {
        return 1 as libc::c_int;
    }
    tape_buffered_write(
        ascii_header.as_mut_ptr(),
        out_des,
        ::std::mem::size_of::<[libc::c_char; 76]>() as libc::c_ulong as off_t,
    );
    tape_buffered_write((*file_hdr).c_name, out_des, (*file_hdr).c_namesize as off_t);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hp_compute_dev(
    mut file_hdr: *mut cpio_file_stat,
    mut pdev: *mut dev_t,
    mut prdev: *mut dev_t,
) {
    match (*file_hdr).c_mode & 0o170000 as libc::c_int as libc::c_uint {
        8192 | 24576 | 49152 | 4096 => {
            (*file_hdr)
                .c_filesize = gnu_dev_makedev(
                (*file_hdr).c_rdev_maj,
                (*file_hdr).c_rdev_min,
            ) as off_t;
            *prdev = gnu_dev_makedev(
                0 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            );
            *pdev = *prdev;
        }
        _ => {
            *pdev = gnu_dev_makedev((*file_hdr).c_dev_maj, (*file_hdr).c_dev_min);
            *prdev = gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min);
        }
    };
}
pub unsafe extern "C" fn write_out_binary_header(
    mut rdev: dev_t,
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: libc::c_int,
) -> libc::c_int {
    let mut short_hdr: old_cpio_header = old_cpio_header {
        c_magic: 0,
        c_dev: 0,
        c_ino: 0,
        c_mode: 0,
        c_uid: 0,
        c_gid: 0,
        c_nlink: 0,
        c_rdev: 0,
        c_mtimes: [0; 2],
        c_namesize: 0,
        c_filesizes: [0; 2],
    };
    short_hdr.c_magic = 0o70707 as libc::c_int as libc::c_ushort;
    short_hdr
        .c_dev = gnu_dev_makedev((*file_hdr).c_dev_maj, (*file_hdr).c_dev_min)
        as libc::c_ushort;
    if warn_option & 0x1 as libc::c_int as libc::c_uint != 0
        && (*file_hdr).c_ino >> 16 as libc::c_int != 0 as libc::c_int as libc::c_ulong
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: truncating inode number\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file_hdr).c_name,
        );
    }
    short_hdr
        .c_ino = ((*file_hdr).c_ino & 0xffff as libc::c_int as libc::c_ulong)
        as libc::c_ushort;
    if short_hdr.c_ino as libc::c_ulong != (*file_hdr).c_ino {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(
                0 as *const libc::c_char,
                b"inode number\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    short_hdr
        .c_mode = ((*file_hdr).c_mode & 0xffff as libc::c_int as libc::c_uint)
        as libc::c_ushort;
    if short_hdr.c_mode as libc::c_uint != (*file_hdr).c_mode {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(
                0 as *const libc::c_char,
                b"file mode\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    short_hdr
        .c_uid = ((*file_hdr).c_uid & 0xffff as libc::c_int as libc::c_uint)
        as libc::c_ushort;
    if short_hdr.c_uid as libc::c_uint != (*file_hdr).c_uid {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(
                0 as *const libc::c_char,
                b"uid\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    short_hdr
        .c_gid = ((*file_hdr).c_gid & 0xffff as libc::c_int as libc::c_uint)
        as libc::c_ushort;
    if short_hdr.c_gid as libc::c_uint != (*file_hdr).c_gid {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(
                0 as *const libc::c_char,
                b"gid\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    short_hdr
        .c_nlink = ((*file_hdr).c_nlink & 0xffff as libc::c_int as libc::c_ulong)
        as libc::c_ushort;
    if short_hdr.c_nlink as libc::c_ulong != (*file_hdr).c_nlink {
        field_width_warning(
            (*file_hdr).c_name,
            dcgettext(
                0 as *const libc::c_char,
                b"number of links\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    short_hdr.c_rdev = rdev as libc::c_ushort;
    short_hdr
        .c_mtimes[0 as libc::c_int
        as usize] = ((*file_hdr).c_mtime >> 16 as libc::c_int) as libc::c_ushort;
    short_hdr
        .c_mtimes[1 as libc::c_int
        as usize] = ((*file_hdr).c_mtime & 0xffff as libc::c_int as libc::c_long)
        as libc::c_ushort;
    short_hdr
        .c_namesize = ((*file_hdr).c_namesize & 0xffff as libc::c_int as libc::c_ulong)
        as libc::c_ushort;
    if short_hdr.c_namesize as libc::c_ulong != (*file_hdr).c_namesize {
        let mut maxbuf: [libc::c_char; 22] = [0; 22];
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: value %s %s out of allowed range 0..%u\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file_hdr).c_name,
            dcgettext(
                0 as *const libc::c_char,
                b"name size\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            umaxtostr((*file_hdr).c_namesize, maxbuf.as_mut_ptr()),
            0xffff as libc::c_uint,
        );
        return 1 as libc::c_int;
    }
    short_hdr
        .c_filesizes[0 as libc::c_int
        as usize] = ((*file_hdr).c_filesize >> 16 as libc::c_int) as libc::c_ushort;
    short_hdr
        .c_filesizes[1 as libc::c_int
        as usize] = ((*file_hdr).c_filesize & 0xffff as libc::c_int as libc::c_long)
        as libc::c_ushort;
    if ((short_hdr.c_filesizes[0 as libc::c_int as usize] as off_t) << 16 as libc::c_int)
        + short_hdr.c_filesizes[1 as libc::c_int as usize] as libc::c_long
        != (*file_hdr).c_filesize
    {
        let mut maxbuf_0: [libc::c_char; 22] = [0; 22];
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: value %s %s out of allowed range 0..%lu\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file_hdr).c_name,
            dcgettext(
                0 as *const libc::c_char,
                b"file size\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            umaxtostr((*file_hdr).c_namesize, maxbuf_0.as_mut_ptr()),
            0xffffffff as libc::c_ulong,
        );
        return 1 as libc::c_int;
    }
    tape_buffered_write(
        &mut short_hdr as *mut old_cpio_header as *mut libc::c_char,
        out_des,
        26 as libc::c_int as off_t,
    );
    tape_buffered_write((*file_hdr).c_name, out_des, (*file_hdr).c_namesize as off_t);
    tape_pad_output(
        out_des,
        ((*file_hdr).c_namesize).wrapping_add(26 as libc::c_int as libc::c_ulong)
            as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn write_out_header(
    mut file_hdr: *mut cpio_file_stat,
    mut out_des: libc::c_int,
) -> libc::c_int {
    let mut dev: dev_t = 0;
    let mut rdev: dev_t = 0;
    match archive_format as libc::c_uint {
        3 => {
            return write_out_new_ascii_header(
                b"070701\0" as *const u8 as *const libc::c_char,
                file_hdr,
                out_des,
            );
        }
        4 => {
            return write_out_new_ascii_header(
                b"070702\0" as *const u8 as *const libc::c_char,
                file_hdr,
                out_des,
            );
        }
        2 => {
            return write_out_old_ascii_header(
                gnu_dev_makedev((*file_hdr).c_dev_maj, (*file_hdr).c_dev_min),
                gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min),
                file_hdr,
                out_des,
            );
        }
        7 => {
            hp_compute_dev(file_hdr, &mut dev, &mut rdev);
            return write_out_old_ascii_header(dev, rdev, file_hdr, out_des);
        }
        5 | 6 => {
            if is_tar_filename_too_long((*file_hdr).c_name) != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: file name too long\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file_hdr).c_name,
                );
                return 1 as libc::c_int;
            }
            return write_out_tar_header(file_hdr, out_des);
        }
        1 => {
            return write_out_binary_header(
                gnu_dev_makedev((*file_hdr).c_rdev_maj, (*file_hdr).c_rdev_min),
                file_hdr,
                out_des,
            );
        }
        8 => {
            hp_compute_dev(file_hdr, &mut dev, &mut rdev);
            return write_out_binary_header(rdev, file_hdr, out_des);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn assign_string(
    mut pvar: *mut *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    let mut p: *mut libc::c_char = xrealloc(
        *pvar as *mut libc::c_void,
        (strlen(value)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(p, value);
    *pvar = p;
}
pub unsafe extern "C" fn process_copy_out() {
    let mut input_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as libc::c_int as size_t,
            ds_idx: 0 as libc::c_int as size_t,
            ds_string: 0 as *mut libc::c_char,
        };
        init
    };
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
    let mut out_file_des: libc::c_int = 0;
    let mut orig_file_name: *mut libc::c_char = 0 as *mut libc::c_char;
    file_hdr.c_magic = 0o70707 as libc::c_int as libc::c_ushort;
    out_file_des = archive_des;
    if out_file_des >= (1 as libc::c_int) << 30 as libc::c_int {
        output_is_special = 1 as libc::c_int as libc::c_char;
        output_is_seekable = 0 as libc::c_int as libc::c_char;
    } else {
        if fstat(out_file_des, &mut file_stat) != 0 {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"standard output is closed\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        output_is_special = (file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o60000 as libc::c_int as libc::c_uint
            || file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint) as libc::c_int
            as libc::c_char;
        output_is_seekable = (file_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_char;
    }
    if append_flag != 0 {
        process_copy_in();
        prepare_append(out_file_des);
    } else {
        change_dir();
    }
    let mut current_block_72: u64;
    while !(ds_fgetstr(stdin, &mut input_name, name_end)).is_null() {
        if *(input_name.ds_string).offset(0 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
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
        } else if ::std::mem::transmute::<
            _,
            fn(_, _) -> libc::c_int,
        >((Some(xstat.unwrap())).unwrap())(input_name.ds_string, &mut file_stat)
            < 0 as libc::c_int
        {
            stat_error(input_name.ds_string);
        } else {
            stat_to_cpio(&mut file_hdr, &mut file_stat);
            if archive_format as libc::c_uint == arf_tar as libc::c_int as libc::c_uint
                || archive_format as libc::c_uint
                    == arf_ustar as libc::c_int as libc::c_uint
            {
                if file_hdr.c_mode & 0o40000 as libc::c_int as libc::c_uint != 0 {
                    if ds_endswith(&mut input_name, '/' as i32) == 0 {
                        ds_append(&mut input_name, '/' as i32);
                    }
                }
            }
            assign_string(&mut orig_file_name, input_name.ds_string);
            cpio_safer_name_suffix(
                input_name.ds_string,
                0 as libc::c_int != 0,
                no_abs_paths_flag == 0,
                1 as libc::c_int != 0,
            );
            cpio_set_c_name(&mut file_hdr, input_name.ds_string);
            match file_hdr.c_mode & 0o170000 as libc::c_int as libc::c_uint {
                32768 => {
                    if archive_format as libc::c_uint
                        == arf_tar as libc::c_int as libc::c_uint
                        || archive_format as libc::c_uint
                            == arf_ustar as libc::c_int as libc::c_uint
                    {
                        let mut otherfile: *mut libc::c_char = 0 as *mut libc::c_char;
                        otherfile = find_inode_file(
                            file_hdr.c_ino,
                            file_hdr.c_dev_maj as libc::c_ulong,
                            file_hdr.c_dev_min as libc::c_ulong,
                        );
                        if !otherfile.is_null() {
                            file_hdr.c_tar_linkname = otherfile;
                            if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                continue;
                            }
                            current_block_72 = 2310077433060450808;
                        } else {
                            current_block_72 = 15897653523371991391;
                        }
                    } else {
                        current_block_72 = 15897653523371991391;
                    }
                    match current_block_72 {
                        2310077433060450808 => {}
                        _ => {
                            if (archive_format as libc::c_uint
                                == arf_newascii as libc::c_int as libc::c_uint
                                || archive_format as libc::c_uint
                                    == arf_crcascii as libc::c_int as libc::c_uint)
                                && file_hdr.c_nlink > 1 as libc::c_int as libc::c_ulong
                            {
                                if last_link(&mut file_hdr) != 0 {
                                    writeout_other_defers(&mut file_hdr, out_file_des);
                                    current_block_72 = 6450636197030046351;
                                } else {
                                    add_link_defer(&mut file_hdr);
                                    current_block_72 = 2310077433060450808;
                                }
                            } else {
                                current_block_72 = 6450636197030046351;
                            }
                            match current_block_72 {
                                2310077433060450808 => {}
                                _ => {
                                    in_file_des = open(
                                        orig_file_name,
                                        0 as libc::c_int | 0 as libc::c_int,
                                        0 as libc::c_int,
                                    );
                                    if in_file_des < 0 as libc::c_int {
                                        open_error(orig_file_name);
                                        continue;
                                    } else {
                                        if archive_format as libc::c_uint
                                            == arf_crcascii as libc::c_int as libc::c_uint
                                        {
                                            file_hdr
                                                .c_chksum = read_for_checksum(
                                                in_file_des,
                                                file_hdr.c_filesize,
                                                orig_file_name,
                                            );
                                        }
                                        if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                            continue;
                                        }
                                        copy_files_disk_to_tape(
                                            in_file_des,
                                            out_file_des,
                                            file_hdr.c_filesize,
                                            orig_file_name,
                                        );
                                        warn_if_file_changed(
                                            orig_file_name,
                                            file_hdr.c_filesize,
                                            file_hdr.c_mtime,
                                        );
                                        if archive_format as libc::c_uint
                                            == arf_tar as libc::c_int as libc::c_uint
                                            || archive_format as libc::c_uint
                                                == arf_ustar as libc::c_int as libc::c_uint
                                        {
                                            add_inode(
                                                file_hdr.c_ino,
                                                orig_file_name,
                                                file_hdr.c_dev_maj as libc::c_ulong,
                                                file_hdr.c_dev_min as libc::c_ulong,
                                            );
                                        }
                                        tape_pad_output(
                                            out_file_des,
                                            file_hdr.c_filesize as libc::c_int,
                                        );
                                        if reset_time_flag != 0 {
                                            set_file_times(
                                                in_file_des,
                                                orig_file_name,
                                                file_stat.st_atim.tv_sec as libc::c_ulong,
                                                file_stat.st_mtim.tv_sec as libc::c_ulong,
                                            );
                                        }
                                        if close(in_file_des) < 0 as libc::c_int {
                                            close_error(orig_file_name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                16384 => {
                    file_hdr.c_filesize = 0 as libc::c_int as off_t;
                    if ignore_dirnlink_option != 0 {
                        file_hdr.c_nlink = 2 as libc::c_int as size_t;
                    }
                    if write_out_header(&mut file_hdr, out_file_des) != 0 {
                        continue;
                    }
                }
                8192 | 24576 | 49152 | 4096 => {
                    if archive_format as libc::c_uint
                        == arf_tar as libc::c_int as libc::c_uint
                    {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s not dumped: not a regular file\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            orig_file_name,
                        );
                        continue;
                    } else {
                        if archive_format as libc::c_uint
                            == arf_ustar as libc::c_int as libc::c_uint
                        {
                            let mut otherfile_0: *mut libc::c_char = 0
                                as *mut libc::c_char;
                            otherfile_0 = find_inode_file(
                                file_hdr.c_ino,
                                file_hdr.c_dev_maj as libc::c_ulong,
                                file_hdr.c_dev_min as libc::c_ulong,
                            );
                            if !otherfile_0.is_null() {
                                file_hdr
                                    .c_mode = file_stat.st_mode
                                    & 0o7777 as libc::c_int as libc::c_uint;
                                file_hdr.c_mode |= 0o100000 as libc::c_int as libc::c_uint;
                                file_hdr.c_tar_linkname = otherfile_0;
                                if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                    continue;
                                }
                                current_block_72 = 2310077433060450808;
                            } else {
                                add_inode(
                                    file_hdr.c_ino,
                                    orig_file_name,
                                    file_hdr.c_dev_maj as libc::c_ulong,
                                    file_hdr.c_dev_min as libc::c_ulong,
                                );
                                current_block_72 = 5807581744382915773;
                            }
                        } else {
                            current_block_72 = 5807581744382915773;
                        }
                        match current_block_72 {
                            2310077433060450808 => {}
                            _ => {
                                file_hdr.c_filesize = 0 as libc::c_int as off_t;
                                if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                    continue;
                                }
                            }
                        }
                    }
                }
                40960 => {
                    let mut link_name: *mut libc::c_char = xmalloc(
                        (file_stat.st_size + 1 as libc::c_int as libc::c_long) as size_t,
                    ) as *mut libc::c_char;
                    let mut link_size: libc::c_int = 0;
                    link_size = readlink(
                        orig_file_name,
                        link_name,
                        file_stat.st_size as size_t,
                    ) as libc::c_int;
                    if link_size < 0 as libc::c_int {
                        readlink_warn(orig_file_name);
                        rpl_free(link_name as *mut libc::c_void);
                        continue;
                    } else {
                        *link_name
                            .offset(
                                link_size as isize,
                            ) = 0 as libc::c_int as libc::c_char;
                        cpio_safer_name_suffix(
                            link_name,
                            0 as libc::c_int != 0,
                            no_abs_paths_flag == 0,
                            1 as libc::c_int != 0,
                        );
                        link_size = strlen(link_name) as libc::c_int;
                        file_hdr.c_filesize = link_size as off_t;
                        if archive_format as libc::c_uint
                            == arf_tar as libc::c_int as libc::c_uint
                            || archive_format as libc::c_uint
                                == arf_ustar as libc::c_int as libc::c_uint
                        {
                            if link_size + 1 as libc::c_int > 100 as libc::c_int {
                                error(
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s: symbolic link too long\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    file_hdr.c_name,
                                );
                            } else {
                                *link_name
                                    .offset(link_size as isize) = '\0' as i32 as libc::c_char;
                                file_hdr.c_tar_linkname = link_name;
                                if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                    continue;
                                }
                            }
                        } else {
                            if write_out_header(&mut file_hdr, out_file_des) != 0 {
                                continue;
                            }
                            tape_buffered_write(
                                link_name,
                                out_file_des,
                                link_size as off_t,
                            );
                            tape_pad_output(out_file_des, link_size);
                        }
                        rpl_free(link_name as *mut libc::c_void);
                    }
                }
                _ => {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: unknown file type\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        orig_file_name,
                    );
                }
            }
            if verbose_flag != 0 {
                fprintf(
                    stderr,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    orig_file_name,
                );
            }
            if dot_flag != 0 {
                fputc_unlocked('.' as i32, stderr);
            }
        }
    }
    rpl_free(orig_file_name as *mut libc::c_void);
    writeout_final_defers(out_file_des);
    file_hdr.c_ino = 0 as libc::c_int as ino_t;
    file_hdr.c_mode = 0 as libc::c_int as mode_t;
    file_hdr.c_uid = 0 as libc::c_int as uid_t;
    file_hdr.c_gid = 0 as libc::c_int as gid_t;
    file_hdr.c_nlink = 1 as libc::c_int as size_t;
    file_hdr.c_dev_maj = 0 as libc::c_int as libc::c_uint;
    file_hdr.c_dev_min = 0 as libc::c_int as libc::c_uint;
    file_hdr.c_rdev_maj = 0 as libc::c_int as libc::c_uint;
    file_hdr.c_rdev_min = 0 as libc::c_int as libc::c_uint;
    file_hdr.c_mtime = 0 as libc::c_int as time_t;
    file_hdr.c_chksum = 0 as libc::c_int as uint32_t;
    file_hdr.c_filesize = 0 as libc::c_int as off_t;
    cpio_set_c_name(
        &mut file_hdr,
        b"TRAILER!!!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if archive_format as libc::c_uint != arf_tar as libc::c_int as libc::c_uint
        && archive_format as libc::c_uint != arf_ustar as libc::c_int as libc::c_uint
    {
        write_out_header(&mut file_hdr, out_file_des);
    } else {
        write_nuls_to_file(
            1024 as libc::c_int as off_t,
            out_file_des,
            Some(
                tape_buffered_write
                    as unsafe extern "C" fn(*mut libc::c_char, libc::c_int, off_t) -> (),
            ),
        );
    }
    tape_clear_rest_of_block(out_file_des);
    tape_empty_output_buffer(out_file_des);
    if dot_flag != 0 {
        fputc_unlocked('\n' as i32, stderr);
    }
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
    cpio_file_stat_free(&mut file_hdr);
    ds_free(&mut input_name);
}
