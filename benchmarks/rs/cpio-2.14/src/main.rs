use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn geteuid() -> __uid_t;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
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
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    fn pax_exit();
    fn argp_parse(
        __argp: *const argp,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        __flags: libc::c_uint,
        __arg_index: *mut libc::c_int,
        __input: *mut libc::c_void,
    ) -> error_t;
    fn argp_help(
        __argp: *const argp,
        __stream: *mut FILE,
        __flags: libc::c_uint,
        __name: *mut libc::c_char,
    );
    fn argp_version_setup(
        name: *const libc::c_char,
        authors: *const *const libc::c_char,
    );
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn close_stdout();
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    static mut unconditional_flag: libc::c_int;
    static mut verbose_flag: libc::c_int;
    static mut dot_flag: libc::c_int;
    static mut link_flag: libc::c_int;
    static mut retain_time_flag: libc::c_int;
    static mut crc_i_flag: libc::c_int;
    static mut append_flag: libc::c_int;
    static mut swap_bytes_flag: libc::c_int;
    static mut swap_halfwords_flag: libc::c_int;
    static mut set_owner_flag: libc::c_int;
    static mut set_owner: uid_t;
    static mut set_group_flag: libc::c_int;
    static mut set_group: gid_t;
    static mut no_chown_flag: libc::c_int;
    static mut sparse_flag: libc::c_int;
    static mut quiet_flag: libc::c_int;
    static mut only_verify_crc_flag: libc::c_int;
    static mut no_abs_paths_flag: libc::c_int;
    static mut warn_option: libc::c_uint;
    static mut renumber_inodes_option: libc::c_int;
    static mut ignore_devno_option: libc::c_int;
    static mut ignore_dirnlink_option: libc::c_int;
    static mut to_stdout_option: bool;
    static mut copy_matching_files: libc::c_int;
    static mut numeric_uid: libc::c_int;
    static mut pattern_file_name: *mut libc::c_char;
    static mut archive_des: libc::c_int;
    static mut archive_name: *mut libc::c_char;
    static mut rsh_command_option: *mut libc::c_char;
    static mut input_buffer: *mut libc::c_char;
    static mut output_buffer: *mut libc::c_char;
    static mut in_buff: *mut libc::c_char;
    static mut out_buff: *mut libc::c_char;
    static mut input_bytes: off_t;
    static mut output_bytes: off_t;
    static mut directory_name: *mut libc::c_char;
    static mut save_patterns: *mut *mut libc::c_char;
    static mut num_patterns: libc::c_int;
    static mut name_end: libc::c_char;
    static mut xstat: Option::<unsafe extern "C" fn() -> libc::c_int>;
    static mut copy_function: Option::<unsafe extern "C" fn() -> ()>;
    static mut change_directory_option: *mut libc::c_char;
    fn process_copy_in();
    fn process_copy_out();
    fn process_copy_pass();
    static mut input_size: size_t;
    static mut input_buffer_size: size_t;
    static mut archive_format: archive_format;
    static mut output_size: size_t;
    static mut reset_time_flag: libc::c_int;
    static mut io_block_size: libc::c_int;
    static mut create_dir_flag: libc::c_int;
    static mut table_flag: libc::c_int;
    static mut rename_batch_file: *mut libc::c_char;
    static mut rename_flag: libc::c_int;
    fn parse_user_spec(
        spec_arg: *const libc::c_char,
        uid: *mut uid_t,
        gid: *mut gid_t,
        username_arg: *mut *mut libc::c_char,
        groupname_arg: *mut *mut libc::c_char,
    ) -> *const libc::c_char;
    fn open_archive(file: *mut libc::c_char) -> libc::c_int;
    fn set_new_media_message(message: *mut libc::c_char);
    fn arf_stores_inode_p(arf: archive_format) -> libc::c_int;
    fn rmt_close__(_: libc::c_int) -> libc::c_int;
    static mut force_local_option: bool;
}
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type error_t = libc::c_int;
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
pub struct argp_option {
    pub name: *const libc::c_char,
    pub key: libc::c_int,
    pub arg: *const libc::c_char,
    pub flags: libc::c_int,
    pub doc: *const libc::c_char,
    pub group: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const libc::c_char,
    pub doc: *const libc::c_char,
    pub children: *const argp_child,
    pub help_filter: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_void,
        ) -> *mut libc::c_char,
    >,
    pub argp_domain: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: libc::c_int,
    pub header: *const libc::c_char,
    pub group: libc::c_int,
}
pub type argp_parser_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub next: libc::c_int,
    pub flags: libc::c_uint,
    pub arg_num: libc::c_uint,
    pub quoted: libc::c_int,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
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
pub const TO_STDOUT_OPTION: cpio_options = 267;
pub const FORCE_LOCAL_OPTION: cpio_options = 264;
pub const SPARSE_OPTION: cpio_options = 263;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warn_tab {
    pub name: *mut libc::c_char,
    pub flag: libc::c_int,
}
pub const QUIET_OPTION: cpio_options = 262;
pub const RENAME_BATCH_FILE_OPTION: cpio_options = 260;
pub const RSH_COMMAND_OPTION: cpio_options = 261;
pub const DEVICE_INDEPENDENT_OPTION: cpio_options = 271;
pub const IGNORE_DIRNLINK_OPTION: cpio_options = 270;
pub const RENUMBER_INODES_OPTION: cpio_options = 268;
pub const IGNORE_DEVNO_OPTION: cpio_options = 269;
pub const ONLY_VERIFY_CRC_OPTION: cpio_options = 259;
pub const NO_PRESERVE_OWNER_OPTION: cpio_options = 258;
pub const ABSOLUTE_FILENAMES_OPTION: cpio_options = 257;
pub const NO_ABSOLUTE_FILENAMES_OPTION: cpio_options = 256;
pub const BLOCK_SIZE_OPTION: cpio_options = 266;
pub type cpio_options = libc::c_uint;
pub const DEBUG_OPTION: cpio_options = 265;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
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
pub static mut program_authors: [*const libc::c_char; 5] = [
    b"Phil Nelson\0" as *const u8 as *const libc::c_char,
    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
    b"John Oleynick\0" as *const u8 as *const libc::c_char,
    b"Sergey Poznyakoff\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub static mut argp_program_bug_address: *const libc::c_char = b"<bug-cpio@gnu.org>\0"
    as *const u8 as *const libc::c_char;
static mut doc: [libc::c_char; 300] = unsafe {
    *::std::mem::transmute::<
        &[u8; 300],
        &mut [libc::c_char; 300],
    >(
        b"GNU `cpio' copies files to and from archives\n\nExamples:\n  # Copy files named in name-list to the archive\n  cpio -o < name-list [> archive]\n  # Extract files from the archive\n  cpio -i [< archive]\n  # Copy files named in name-list to destination-directory\n  cpio -p destination-directory < name-list\n\0",
    )
};
static mut options: [argp_option; 58] = [
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Main operation mode:\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"create\0" as *const u8 as *const libc::c_char,
            key: 'o' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Create the archive (run in copy-out mode)\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"extract\0" as *const u8 as *const libc::c_char,
            key: 'i' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Extract files from an archive (run in copy-in mode)\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"pass-through\0" as *const u8 as *const libc::c_char,
            key: 'p' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Run in copy-pass mode\0" as *const u8 as *const libc::c_char,
            group: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"list\0" as *const u8 as *const libc::c_char,
            key: 't' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Print a table of contents of the input\0" as *const u8
                as *const libc::c_char,
            group: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid in any mode:\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"directory\0" as *const u8 as *const libc::c_char,
            key: 'D' as i32,
            arg: b"DIR\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Change to directory DIR\0" as *const u8 as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"force-local\0" as *const u8 as *const libc::c_char,
            key: FORCE_LOCAL_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Archive file is local, even if its name contains colons\0"
                as *const u8 as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"format\0" as *const u8 as *const libc::c_char,
            key: 'H' as i32,
            arg: b"FORMAT\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Use given archive FORMAT\0" as *const u8 as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 'B' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Set the I/O block size to 5120 bytes\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"block-size\0" as *const u8 as *const libc::c_char,
            key: BLOCK_SIZE_OPTION as libc::c_int,
            arg: b"BLOCK-SIZE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Set the I/O block size to BLOCK-SIZE * 512 bytes\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 'c' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Use the old portable (ASCII) archive format\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"dot\0" as *const u8 as *const libc::c_char,
            key: 'V' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Print a \".\" for each file processed\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"io-size\0" as *const u8 as *const libc::c_char,
            key: 'C' as i32,
            arg: b"NUMBER\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Set the I/O block size to the given NUMBER of bytes\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            key: QUIET_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Do not print the number of blocks copied\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            key: 'v' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Verbosely list the files processed\0" as *const u8
                as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"warning\0" as *const u8 as *const libc::c_char,
            key: 'W' as i32,
            arg: b"FLAG\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Control warning display. Currently FLAG is one of 'none', 'truncate', 'all'. Multiple options accumulate.\0"
                as *const u8 as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"owner\0" as *const u8 as *const libc::c_char,
            key: 'R' as i32,
            arg: b"[USER][:.][GROUP]\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Set the ownership of all files created to the specified USER and/or GROUP\0"
                as *const u8 as *const libc::c_char,
            group: 100 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid in copy-in and copy-out modes\0"
                as *const u8 as *const libc::c_char,
            group: 110 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"file\0" as *const u8 as *const libc::c_char,
            key: 'F' as i32,
            arg: b"[[USER@]HOST:]FILE-NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Use this FILE-NAME instead of standard input or output. Optional USER and HOST specify the user and host names in case of a remote archive\0"
                as *const u8 as *const libc::c_char,
            group: 110 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"message\0" as *const u8 as *const libc::c_char,
            key: 'M' as i32,
            arg: b"STRING\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Print STRING when the end of a volume of the backup media is reached\0"
                as *const u8 as *const libc::c_char,
            group: 110 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rsh-command\0" as *const u8 as *const libc::c_char,
            key: RSH_COMMAND_OPTION as libc::c_int,
            arg: b"COMMAND\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Use COMMAND instead of rsh\0" as *const u8 as *const libc::c_char,
            group: 110 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid only in copy-in mode:\0" as *const u8
                as *const libc::c_char,
            group: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"nonmatching\0" as *const u8 as *const libc::c_char,
            key: 'f' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Only copy files that do not match any of the given patterns\0"
                as *const u8 as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"numeric-uid-gid\0" as *const u8 as *const libc::c_char,
            key: 'n' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"In the verbose table of contents listing, show numeric UID and GID\0"
                as *const u8 as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"pattern-file\0" as *const u8 as *const libc::c_char,
            key: 'E' as i32,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Read additional patterns specifying filenames to extract or list from FILE\0"
                as *const u8 as *const libc::c_char,
            group: 210 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"only-verify-crc\0" as *const u8 as *const libc::c_char,
            key: ONLY_VERIFY_CRC_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"When reading a CRC format archive, only verify the CRC's of each file in the archive, don't actually extract the files\0"
                as *const u8 as *const libc::c_char,
            group: 210 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rename\0" as *const u8 as *const libc::c_char,
            key: 'r' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Interactively rename files\0" as *const u8 as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"rename-batch-file\0" as *const u8 as *const libc::c_char,
            key: RENAME_BATCH_FILE_OPTION as libc::c_int,
            arg: b"FILE\0" as *const u8 as *const libc::c_char,
            flags: 0x2 as libc::c_int,
            doc: b"\0" as *const u8 as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"swap\0" as *const u8 as *const libc::c_char,
            key: 'b' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Swap both halfwords of words and bytes of halfwords in the data. Equivalent to -sS\0"
                as *const u8 as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"swap-bytes\0" as *const u8 as *const libc::c_char,
            key: 's' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Swap the bytes of each halfword in the files\0" as *const u8
                as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"swap-halfwords\0" as *const u8 as *const libc::c_char,
            key: 'S' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Swap the halfwords of each word (4 bytes) in the files\0" as *const u8
                as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"to-stdout\0" as *const u8 as *const libc::c_char,
            key: TO_STDOUT_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Extract files to standard output\0" as *const u8
                as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 'I' as i32,
            arg: b"[[USER@]HOST:]FILE-NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Archive filename to use instead of standard input. Optional USER and HOST specify the user and host names in case of a remote archive\0"
                as *const u8 as *const libc::c_char,
            group: 200 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid only in copy-out mode:\0" as *const u8
                as *const libc::c_char,
            group: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"append\0" as *const u8 as *const libc::c_char,
            key: 'A' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Append to an existing archive.\0" as *const u8 as *const libc::c_char,
            group: 300 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 'O' as i32,
            arg: b"[[USER@]HOST:]FILE-NAME\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Archive filename to use instead of standard output. Optional USER and HOST specify the user and host names in case of a remote archive\0"
                as *const u8 as *const libc::c_char,
            group: 300 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"renumber-inodes\0" as *const u8 as *const libc::c_char,
            key: RENUMBER_INODES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Renumber inodes\0" as *const u8 as *const libc::c_char,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-devno\0" as *const u8 as *const libc::c_char,
            key: IGNORE_DEVNO_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Don't store device numbers\0" as *const u8 as *const libc::c_char,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-dirnlink\0" as *const u8 as *const libc::c_char,
            key: IGNORE_DIRNLINK_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"ignore number of links of a directory; always assume 2\0" as *const u8
                as *const libc::c_char,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"device-independent\0" as *const u8 as *const libc::c_char,
            key: DEVICE_INDEPENDENT_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Create device-independent (reproducible) archives\0" as *const u8
                as *const libc::c_char,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"reproducible\0" as *const u8 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0x4 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: 0,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid only in copy-pass mode:\0" as *const u8
                as *const libc::c_char,
            group: 400 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"link\0" as *const u8 as *const libc::c_char,
            key: 'l' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Link files instead of copying them, when  possible\0" as *const u8
                as *const libc::c_char,
            group: 400 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid in copy-in and copy-out modes:\0"
                as *const u8 as *const libc::c_char,
            group: 500 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"absolute-filenames\0" as *const u8 as *const libc::c_char,
            key: ABSOLUTE_FILENAMES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Do not strip file system prefix components from the file names\0"
                as *const u8 as *const libc::c_char,
            group: 500 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-absolute-filenames\0" as *const u8 as *const libc::c_char,
            key: NO_ABSOLUTE_FILENAMES_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Create all files relative to the current directory\0" as *const u8
                as *const libc::c_char,
            group: 500 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid in copy-out and copy-pass modes:\0"
                as *const u8 as *const libc::c_char,
            group: 600 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"null\0" as *const u8 as *const libc::c_char,
            key: '0' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Filenames in the list are delimited by null characters instead of newlines\0"
                as *const u8 as *const libc::c_char,
            group: 600 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"dereference\0" as *const u8 as *const libc::c_char,
            key: 'L' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Dereference  symbolic  links  (copy  the files that they point to instead of copying the links).\0"
                as *const u8 as *const libc::c_char,
            group: 600 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"reset-access-time\0" as *const u8 as *const libc::c_char,
            key: 'a' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Reset the access times of files after reading them\0" as *const u8
                as *const libc::c_char,
            group: 600 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Operation modifiers valid in copy-in and copy-pass modes:\0"
                as *const u8 as *const libc::c_char,
            group: 700 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"preserve-modification-time\0" as *const u8 as *const libc::c_char,
            key: 'm' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Retain previous file modification times when creating files\0"
                as *const u8 as *const libc::c_char,
            group: 700 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"make-directories\0" as *const u8 as *const libc::c_char,
            key: 'd' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Create leading directories where needed\0" as *const u8
                as *const libc::c_char,
            group: 700 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-preserve-owner\0" as *const u8 as *const libc::c_char,
            key: NO_PRESERVE_OWNER_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Do not change the ownership of the files\0" as *const u8
                as *const libc::c_char,
            group: 700 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"unconditional\0" as *const u8 as *const libc::c_char,
            key: 'u' as i32,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Replace all files unconditionally\0" as *const u8
                as *const libc::c_char,
            group: 700 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"sparse\0" as *const u8 as *const libc::c_char,
            key: SPARSE_OPTION as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: b"Write files with large blocks of zeros as sparse files\0" as *const u8
                as *const libc::c_char,
            group: 700 as libc::c_int + 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const libc::c_char,
            key: 0 as libc::c_int,
            arg: 0 as *const libc::c_char,
            flags: 0 as libc::c_int,
            doc: 0 as *const libc::c_char,
            group: 0,
        };
        init
    },
];
static mut input_archive_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut output_archive_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe extern "C" fn warn_control(mut arg: *mut libc::c_char) -> libc::c_int {
    static mut warn_tab_0: [warn_tab; 5] = [
        {
            let mut init = warn_tab {
                name: b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                flag: -(1 as libc::c_int) as libc::c_uint as libc::c_int,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: b"truncate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flag: 0x1 as libc::c_int,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: b"all\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                flag: -(1 as libc::c_int) as libc::c_uint as libc::c_int,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: b"interdir\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flag: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = warn_tab {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                flag: 0,
            };
            init
        },
    ];
    let mut wt: *mut warn_tab = 0 as *mut warn_tab;
    let mut offset: libc::c_int = 0 as libc::c_int;
    if strcmp(arg, b"none\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        warn_option = 0 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int;
    }
    if strlen(arg) > 2 as libc::c_int as libc::c_ulong
        && memcmp(
            arg as *const libc::c_void,
            b"no-\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        offset = 3 as libc::c_int;
    }
    wt = warn_tab_0.as_mut_ptr();
    while !((*wt).name).is_null() {
        if strcmp(arg.offset(offset as isize), (*wt).name) == 0 as libc::c_int {
            if offset != 0 {
                warn_option &= !(*wt).flag as libc::c_uint;
            } else {
                warn_option |= (*wt).flag as libc::c_uint;
            }
            return 0 as libc::c_int;
        }
        wt = wt.offset(1);
        wt;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse_opt(
    mut key: libc::c_int,
    mut arg: *mut libc::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        48 => {
            name_end = '\0' as i32 as libc::c_char;
        }
        97 => {
            reset_time_flag = 1 as libc::c_int;
        }
        65 => {
            append_flag = 1 as libc::c_int;
        }
        98 => {
            swap_bytes_flag = 1 as libc::c_int;
            swap_halfwords_flag = 1 as libc::c_int;
        }
        66 => {
            io_block_size = 5120 as libc::c_int;
        }
        266 => {
            io_block_size = atoi(arg);
            if io_block_size < 1 as libc::c_int
                || io_block_size > 2147483647 as libc::c_int / 512 as libc::c_int
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid block size\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            io_block_size *= 512 as libc::c_int;
        }
        99 => {
            if archive_format as libc::c_uint
                != arf_unknown as libc::c_int as libc::c_uint
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Archive format multiply defined\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            archive_format = arf_oldascii;
        }
        67 => {
            io_block_size = atoi(arg);
            if io_block_size < 1 as libc::c_int {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid block size\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
        }
        100 => {
            create_dir_flag = 1 as libc::c_int;
        }
        68 => {
            change_directory_option = arg;
        }
        102 => {
            copy_matching_files = 0 as libc::c_int;
        }
        69 => {
            pattern_file_name = arg;
        }
        70 => {
            archive_name = arg;
        }
        72 => {
            if archive_format as libc::c_uint
                != arf_unknown as libc::c_int as libc::c_uint
            {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Archive format multiply defined\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            if strcasecmp(arg, b"crc\0" as *const u8 as *const libc::c_char) == 0 {
                archive_format = arf_crcascii;
            } else if strcasecmp(arg, b"newc\0" as *const u8 as *const libc::c_char) == 0
            {
                archive_format = arf_newascii;
            } else if strcasecmp(arg, b"odc\0" as *const u8 as *const libc::c_char) == 0
            {
                archive_format = arf_oldascii;
            } else if strcasecmp(arg, b"bin\0" as *const u8 as *const libc::c_char) == 0
            {
                archive_format = arf_binary;
            } else if strcasecmp(arg, b"ustar\0" as *const u8 as *const libc::c_char)
                == 0
            {
                archive_format = arf_ustar;
            } else if strcasecmp(arg, b"tar\0" as *const u8 as *const libc::c_char) == 0
            {
                archive_format = arf_tar;
            } else if strcasecmp(arg, b"hpodc\0" as *const u8 as *const libc::c_char)
                == 0
            {
                archive_format = arf_hpoldascii;
            } else if strcasecmp(arg, b"hpbin\0" as *const u8 as *const libc::c_char)
                == 0
            {
                archive_format = arf_hpbinary;
            } else {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"invalid archive format `%s'; valid formats are:\ncrc newc odc bin ustar tar (all-caps also recognized)\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    arg,
                );
                usage(2 as libc::c_int);
            }
        }
        105 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Mode already defined\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            copy_function = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_in as unsafe extern "C" fn() -> ()));
        }
        73 => {
            input_archive_name = arg;
        }
        108 => {
            link_flag = 1 as libc::c_int;
        }
        76 => {
            xstat = ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
                >,
                Option::<unsafe extern "C" fn() -> libc::c_int>,
            >(
                Some(
                    stat
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *mut stat,
                        ) -> libc::c_int,
                ),
            );
        }
        109 => {
            retain_time_flag = 1 as libc::c_int;
        }
        77 => {
            set_new_media_message(arg);
        }
        110 => {
            numeric_uid = 1 as libc::c_int;
        }
        256 => {
            no_abs_paths_flag = 1 as libc::c_int;
        }
        257 => {
            no_abs_paths_flag = 0 as libc::c_int;
        }
        258 => {
            if set_owner_flag != 0 || set_group_flag != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--no-preserve-owner cannot be used with --owner\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            no_chown_flag = 1 as libc::c_int;
        }
        111 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Mode already defined\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            copy_function = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_out as unsafe extern "C" fn() -> ()));
        }
        79 => {
            output_archive_name = arg;
        }
        259 => {
            only_verify_crc_flag = 1 as libc::c_int;
        }
        112 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Mode already defined\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            }
            copy_function = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_pass as unsafe extern "C" fn() -> ()));
        }
        269 => {
            ignore_devno_option = 1 as libc::c_int;
        }
        268 => {
            renumber_inodes_option = 1 as libc::c_int;
        }
        270 => {
            ignore_dirnlink_option = 1 as libc::c_int;
        }
        271 => {
            ignore_dirnlink_option = 1 as libc::c_int;
            renumber_inodes_option = ignore_dirnlink_option;
            ignore_devno_option = renumber_inodes_option;
        }
        261 => {
            rsh_command_option = arg;
        }
        114 => {
            rename_flag = 1 as libc::c_int;
        }
        260 => {
            rename_batch_file = arg;
        }
        262 => {
            quiet_flag = 1 as libc::c_int;
        }
        82 => {
            if no_chown_flag != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--owner cannot be used with --no-preserve-owner\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                usage(2 as libc::c_int);
            } else {
                let mut e: *const libc::c_char = 0 as *const libc::c_char;
                let mut u: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut g: *mut libc::c_char = 0 as *mut libc::c_char;
                e = parse_user_spec(arg, &mut set_owner, &mut set_group, &mut u, &mut g);
                if !e.is_null() {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        b"%s: %s\0" as *const u8 as *const libc::c_char,
                        arg,
                        e,
                    );
                    usage(2 as libc::c_int);
                }
                if !u.is_null() {
                    rpl_free(u as *mut libc::c_void);
                    set_owner_flag = 1 as libc::c_int;
                }
                if !g.is_null() {
                    rpl_free(g as *mut libc::c_void);
                    set_group_flag = 1 as libc::c_int;
                }
            }
        }
        115 => {
            swap_bytes_flag = 1 as libc::c_int;
        }
        83 => {
            swap_halfwords_flag = 1 as libc::c_int;
        }
        116 => {
            table_flag = 1 as libc::c_int;
        }
        117 => {
            unconditional_flag = 1 as libc::c_int;
        }
        118 => {
            verbose_flag = 1 as libc::c_int;
        }
        86 => {
            dot_flag = 1 as libc::c_int;
        }
        87 => {
            if warn_control(arg) != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Invalid value for --warning option: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    arg,
                );
                usage(2 as libc::c_int);
            }
        }
        263 => {
            sparse_flag = 1 as libc::c_int;
        }
        264 => {
            force_local_option = 1 as libc::c_int != 0;
        }
        267 => {
            to_stdout_option = 1 as libc::c_int != 0;
        }
        _ => return 7 as libc::c_int,
    }
    return 0 as libc::c_int;
}
static mut argp: argp = unsafe {
    {
        let mut init = argp {
            options: options.as_ptr() as *mut _,
            parser: Some(
                parse_opt
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut argp_state,
                    ) -> error_t,
            ),
            args_doc: b"[destination-directory]\0" as *const u8 as *const libc::c_char,
            doc: doc.as_ptr() as *mut _,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const libc::c_char,
        };
        init
    }
};
unsafe extern "C" fn usage(mut status: libc::c_int) {
    argp_help(
        &mut argp,
        stderr,
        0x4 as libc::c_int as libc::c_uint,
        program_name as *mut libc::c_char,
    );
    close_stdout();
    exit(status);
}
pub unsafe extern "C" fn process_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut index: libc::c_int = 0;
    xstat = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int>,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            lstat as unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
        ),
    );
    if argp_parse(
        &mut argp,
        argc,
        argv,
        0x8 as libc::c_int as libc::c_uint,
        &mut index,
        0 as *mut libc::c_void,
    ) != 0
    {
        exit(2 as libc::c_int);
    }
    if copy_function.is_none() {
        if table_flag != 0 {
            copy_function = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_in as unsafe extern "C" fn() -> ()));
        } else {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"You must specify one of -oipt options.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
    }
    if copy_function
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_in as unsafe extern "C" fn() -> ()))
    {
        archive_des = 0 as libc::c_int;
        if link_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--link\0" as *const u8 as *const libc::c_char,
                b"--extract\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if reset_time_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--reset\0" as *const u8 as *const libc::c_char,
                b"--extract\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if xstat
            != ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
                >,
                Option::<unsafe extern "C" fn() -> libc::c_int>,
            >(
                Some(
                    lstat
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            *mut stat,
                        ) -> libc::c_int,
                ),
            )
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--dereference\0" as *const u8 as *const libc::c_char,
                b"--extract\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if append_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--append\0" as *const u8 as *const libc::c_char,
                b"--extract\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if !output_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"-O\0" as *const u8 as *const libc::c_char,
                b"--extract\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if renumber_inodes_option != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--renumber-inodes\0" as *const u8 as *const libc::c_char,
                b"--extract\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if ignore_devno_option != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--ignore-devno\0" as *const u8 as *const libc::c_char,
                b"--extract\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if to_stdout_option {
            if create_dir_flag != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s is meaningless with %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"--make-directories\0" as *const u8 as *const libc::c_char,
                    b"--to-stdout\0" as *const u8 as *const libc::c_char,
                );
                usage(2 as libc::c_int);
            }
            if rename_flag != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s is meaningless with %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"--rename\0" as *const u8 as *const libc::c_char,
                    b"--to-stdout\0" as *const u8 as *const libc::c_char,
                );
                usage(2 as libc::c_int);
            }
            if no_chown_flag != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s is meaningless with %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"--no-preserve-owner\0" as *const u8 as *const libc::c_char,
                    b"--to-stdout\0" as *const u8 as *const libc::c_char,
                );
                usage(2 as libc::c_int);
            }
            if set_owner_flag != 0 || set_group_flag != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s is meaningless with %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"--owner\0" as *const u8 as *const libc::c_char,
                    b"--to-stdout\0" as *const u8 as *const libc::c_char,
                );
                usage(2 as libc::c_int);
            }
            if retain_time_flag != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s is meaningless with %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    b"--preserve-modification-time\0" as *const u8
                        as *const libc::c_char,
                    b"--to-stdout\0" as *const u8 as *const libc::c_char,
                );
                usage(2 as libc::c_int);
            }
        }
        if !archive_name.is_null() && !input_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Both -I and -F are used in copy-in mode\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if archive_format as libc::c_uint == arf_crcascii as libc::c_int as libc::c_uint
        {
            crc_i_flag = 1 as libc::c_int;
        }
        num_patterns = argc - index;
        save_patterns = &mut *argv.offset(index as isize) as *mut *mut libc::c_char;
        if !input_archive_name.is_null() {
            archive_name = input_archive_name;
        }
    } else if copy_function
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_out as unsafe extern "C" fn() -> ()))
    {
        if index != argc {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Too many arguments\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        archive_des = 1 as libc::c_int;
        if create_dir_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--make-directories\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if rename_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--rename\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if table_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--list\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if unconditional_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--unconditional\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if link_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--link\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if sparse_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--sparse\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if retain_time_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--preserve-modification-time\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if no_chown_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--no-preserve-owner\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if swap_bytes_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--swap-bytes (--swap)\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if swap_halfwords_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--swap-halfwords (--swap)\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if to_stdout_option {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--to-stdout\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if append_flag != 0
            && !(!archive_name.is_null() || !output_archive_name.is_null())
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"--append is used but no archive file name is given (use -F or -O options)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if !rename_batch_file.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--rename-batch-file\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if !input_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"-I\0" as *const u8 as *const libc::c_char,
                b"--create\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if !archive_name.is_null() && !output_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Both -O and -F are used in copy-out mode\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if archive_format as libc::c_uint == arf_unknown as libc::c_int as libc::c_uint {
            archive_format = arf_binary;
        }
        if !output_archive_name.is_null() {
            archive_name = output_archive_name;
        }
        if arf_stores_inode_p(archive_format) == 0 {
            ignore_devno_option = 0 as libc::c_int;
            renumber_inodes_option = ignore_devno_option;
        }
    } else {
        if index < argc - 1 as libc::c_int {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Too many arguments\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        } else if index > argc - 1 as libc::c_int {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Not enough arguments\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if archive_format as libc::c_uint != arf_unknown as libc::c_int as libc::c_uint {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Archive format is not specified in copy-pass mode (use --format option)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            usage(2 as libc::c_int);
        }
        if swap_bytes_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--swap-bytes (--swap)\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if swap_halfwords_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--swap-halfwords (--swap)\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if table_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--list\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if rename_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--rename\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if append_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--append\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if !rename_batch_file.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--rename-batch-file\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if no_abs_paths_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--no-absolute-pathnames\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if no_abs_paths_flag != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--absolute-pathnames\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if to_stdout_option {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--to-stdout\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if renumber_inodes_option != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--renumber-inodes\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        if ignore_devno_option != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s is meaningless with %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"--ignore-devno\0" as *const u8 as *const libc::c_char,
                b"--pass-through\0" as *const u8 as *const libc::c_char,
            );
            usage(2 as libc::c_int);
        }
        directory_name = *argv.offset(index as isize);
    }
    if !archive_name.is_null() {
        if copy_function
            != ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(process_copy_in as unsafe extern "C" fn() -> ()))
            && copy_function
                != ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(process_copy_out as unsafe extern "C" fn() -> ()))
        {
            error(
                2 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"-F can be used only with --create or --extract\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        archive_des = open_archive(archive_name);
        if archive_des < 0 as libc::c_int {
            error(
                2 as libc::c_int,
                *__errno_location(),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Cannot open %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_colon(archive_name),
            );
        }
    }
    if set_owner_flag == 0 as libc::c_int && set_group_flag == 0 as libc::c_int
        && geteuid() != 0
    {
        no_chown_flag = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn initialize_buffers() {
    let mut in_buf_size: libc::c_int = 0;
    let mut out_buf_size: libc::c_int = 0;
    if copy_function
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_in as unsafe extern "C" fn() -> ()))
    {
        if io_block_size >= 512 as libc::c_int {
            in_buf_size = 2 as libc::c_int * io_block_size;
        } else {
            in_buf_size = 1024 as libc::c_int;
        }
        out_buf_size = 512 as libc::c_int;
    } else if copy_function
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(process_copy_out as unsafe extern "C" fn() -> ()))
    {
        in_buf_size = 512 as libc::c_int;
        out_buf_size = io_block_size;
    } else {
        in_buf_size = 512 as libc::c_int;
        out_buf_size = 512 as libc::c_int;
    }
    input_buffer = xmalloc(in_buf_size as size_t) as *mut libc::c_char;
    in_buff = input_buffer;
    input_buffer_size = in_buf_size as size_t;
    input_size = 0 as libc::c_int as size_t;
    input_bytes = 0 as libc::c_int as off_t;
    output_buffer = xmalloc(out_buf_size as size_t) as *mut libc::c_char;
    out_buff = output_buffer;
    output_size = 0 as libc::c_int as size_t;
    output_bytes = 0 as libc::c_int as off_t;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"cpio\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"cpio\0" as *const u8 as *const libc::c_char);
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    argp_version_setup(
        b"cpio\0" as *const u8 as *const libc::c_char,
        program_authors.as_mut_ptr(),
    );
    process_args(argc, argv);
    initialize_buffers();
    ::std::mem::transmute::<_, fn()>((Some(copy_function.unwrap())).unwrap())();
    if archive_des >= 0 as libc::c_int
        && (if archive_des >= (1 as libc::c_int) << 30 as libc::c_int {
            rmt_close__(archive_des - ((1 as libc::c_int) << 30 as libc::c_int))
        } else {
            close(archive_des)
        }) == -(1 as libc::c_int)
    {
        error(
            2 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"error closing archive\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    pax_exit();
    return 0;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
