use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn log_error(fmt: *const libc::c_char, _: ...);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type va_list = __builtin_va_list;
pub type uint64_t = __uint64_t;
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
pub struct addr_memo_s {
    pub php_bin_path: [libc::c_char; 256],
    pub php_bin_path_root: [libc::c_char; 256],
    pub php_base_addr: uint64_t,
}
pub type addr_memo_t = addr_memo_s;
unsafe extern "C" fn shell_escape(
    mut arg: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut buf_size: size_t,
) -> libc::c_int {
    let buf_end: *mut libc::c_char = buf.offset(buf_size as isize);
    if buf_size >= 1 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"buf_size >= 1\0" as *const u8 as *const libc::c_char,
            b"addr_objdump.c\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int shell_escape(const char *, char *, size_t)\0"))
                .as_ptr(),
        );
    };
    let fresh0 = buf;
    buf = buf.offset(1);
    *fresh0 = '\'' as i32 as libc::c_char;
    while *arg != 0 {
        if *arg as libc::c_int == '\'' as i32 {
            if (buf_end.offset_from(buf) as libc::c_long)
                < 4 as libc::c_int as libc::c_long
            {
                return 1 as libc::c_int;
            }
            let fresh1 = buf;
            buf = buf.offset(1);
            *fresh1 = '\'' as i32 as libc::c_char;
            let fresh2 = buf;
            buf = buf.offset(1);
            *fresh2 = '\\' as i32 as libc::c_char;
            let fresh3 = buf;
            buf = buf.offset(1);
            *fresh3 = '\'' as i32 as libc::c_char;
            let fresh4 = buf;
            buf = buf.offset(1);
            *fresh4 = '\'' as i32 as libc::c_char;
            arg = arg.offset(1);
            arg;
        } else {
            if (buf_end.offset_from(buf) as libc::c_long)
                < 1 as libc::c_int as libc::c_long
            {
                return 1 as libc::c_int;
            }
            let fresh5 = arg;
            arg = arg.offset(1);
            let fresh6 = buf;
            buf = buf.offset(1);
            *fresh6 = *fresh5;
        }
    }
    if (buf_end.offset_from(buf) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        return 1 as libc::c_int;
    }
    let fresh7 = buf;
    buf = buf.offset(1);
    *fresh7 = '\'' as i32 as libc::c_char;
    *buf = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_symbol_addr(
    mut memo: *mut addr_memo_t,
    mut pid: pid_t,
    mut symbol: *const libc::c_char,
    mut raddr: *mut uint64_t,
) -> libc::c_int {
    let mut php_bin_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut php_bin_path_root: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut php_base_addr: *mut uint64_t = 0 as *mut uint64_t;
    let mut addr_offset: uint64_t = 0;
    php_bin_path = ((*memo).php_bin_path).as_mut_ptr();
    php_bin_path_root = ((*memo).php_bin_path_root).as_mut_ptr();
    php_base_addr = &mut (*memo).php_base_addr;
    if *php_bin_path as libc::c_int == '\0' as i32
        && get_php_bin_path(pid, php_bin_path_root, php_bin_path) != 0 as libc::c_int
    {
        return 1 as libc::c_int
    } else if *php_base_addr == 0 as libc::c_int as libc::c_ulong
        && get_php_base_addr(pid, php_bin_path_root, php_bin_path, php_base_addr)
            != 0 as libc::c_int
    {
        return 1 as libc::c_int
    } else if get_symbol_offset(php_bin_path_root, symbol, &mut addr_offset)
        != 0 as libc::c_int
    {
        return 1 as libc::c_int
    }
    *raddr = (*php_base_addr).wrapping_add(addr_offset);
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_php_bin_path(
    mut pid: pid_t,
    mut path_root: *mut libc::c_char,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut cmd_fmt: *mut libc::c_char = b"awk -ve=1 '/libphp[78]?/{print $NF; e=0; exit} END{exit e}' /proc/%d/maps || readlink /proc/%d/exe\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    if popen_read_line(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        cmd_fmt,
        pid,
        pid,
    ) != 0 as libc::c_int
    {
        log_error(b"get_php_bin_path: Failed\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if snprintf(
        path_root,
        256 as libc::c_int as libc::c_ulong,
        b"/proc/%d/root/%s\0" as *const u8 as *const libc::c_char,
        pid,
        buf.as_mut_ptr(),
    ) > 256 as libc::c_int - 1 as libc::c_int
    {
        log_error(
            b"get_php_bin_path: snprintf overflow\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if access(path_root, 0 as libc::c_int) != 0 as libc::c_int {
        snprintf(
            path_root,
            256 as libc::c_int as libc::c_ulong,
            b"/proc/%d/exe\0" as *const u8 as *const libc::c_char,
            pid,
        );
    }
    strcpy(path, buf.as_mut_ptr());
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_php_base_addr(
    mut pid: pid_t,
    mut path_root: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut raddr: *mut uint64_t,
) -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut arg_buf: [libc::c_char; 256] = [0; 256];
    let mut start_addr: uint64_t = 0;
    let mut virt_addr: uint64_t = 0;
    let mut cmd_fmt: *mut libc::c_char = b"grep -m1 ' '%s\\$ /proc/%d/maps\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    if shell_escape(
        path,
        arg_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    ) != 0
    {
        log_error(
            b"shell_escape: Buffer too small to escape path: %s\n\0" as *const u8
                as *const libc::c_char,
            path,
        );
        return 1 as libc::c_int;
    }
    if popen_read_line(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        cmd_fmt,
        arg_buf.as_mut_ptr(),
        pid,
    ) != 0 as libc::c_int
    {
        log_error(
            b"get_php_base_addr: Failed to get start_addr\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    start_addr = strtoull(
        buf.as_mut_ptr(),
        0 as *mut *mut libc::c_char,
        16 as libc::c_int,
    ) as uint64_t;
    if shell_escape(
        path_root,
        arg_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    ) != 0
    {
        log_error(
            b"shell_escape: Buffer too small to escape path_root: %s\n\0" as *const u8
                as *const libc::c_char,
            path_root,
        );
        return 1 as libc::c_int;
    }
    cmd_fmt = b"objdump -p %s | awk '/LOAD/{print $5; exit}'\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    if popen_read_line(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        cmd_fmt,
        arg_buf.as_mut_ptr(),
    ) != 0 as libc::c_int
    {
        log_error(
            b"get_php_base_addr: Failed to get virt_addr\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    virt_addr = strtoull(
        buf.as_mut_ptr(),
        0 as *mut *mut libc::c_char,
        16 as libc::c_int,
    ) as uint64_t;
    *raddr = start_addr.wrapping_sub(virt_addr);
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_symbol_offset(
    mut path_root: *mut libc::c_char,
    mut symbol: *const libc::c_char,
    mut raddr: *mut uint64_t,
) -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut arg_buf: [libc::c_char; 256] = [0; 256];
    let mut cmd_fmt: *mut libc::c_char = b"objdump -Tt %s | awk '/ %s$/{print $1; exit}'\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    if shell_escape(
        path_root,
        arg_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    ) != 0
    {
        log_error(
            b"shell_escape: Buffer too smal to escape path_root: %s\n\0" as *const u8
                as *const libc::c_char,
            path_root,
        );
        return 1 as libc::c_int;
    }
    if popen_read_line(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        cmd_fmt,
        arg_buf.as_mut_ptr(),
        symbol,
    ) != 0 as libc::c_int
    {
        log_error(b"get_symbol_offset: Failed\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    *raddr = strtoull(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char, 16 as libc::c_int)
        as uint64_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn popen_read_line(
    mut buf: *mut libc::c_char,
    mut buf_size: size_t,
    mut cmd_fmt: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut cmd: [libc::c_char; 256] = [0; 256];
    let mut buf_len: libc::c_int = 0;
    let mut cmd_args: ::std::ffi::VaListImpl;
    cmd_args = args.clone();
    if vsnprintf(
        cmd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        cmd_fmt,
        cmd_args.as_va_list(),
    )
        >= (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
    {
        log_error(b"vsnprintf overflow\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    fp = popen(cmd.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        perror(b"popen\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if (fgets(
        buf,
        buf_size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        fp,
    ))
        .is_null()
    {
        log_error(
            b"popen_read_line: No stdout; cmd=%s\n\0" as *const u8
                as *const libc::c_char,
            cmd.as_mut_ptr(),
        );
        pclose(fp);
        return 1 as libc::c_int;
    }
    pclose(fp);
    buf_len = strlen(buf) as libc::c_int;
    while buf_len > 0 as libc::c_int
        && *buf.offset((buf_len - 1 as libc::c_int) as isize) as libc::c_int
            == '\n' as i32
    {
        buf_len -= 1;
        buf_len;
    }
    if buf_len < 1 as libc::c_int {
        log_error(
            b"popen_read_line: Expected strlen(buf)>0; cmd=%s\n\0" as *const u8
                as *const libc::c_char,
            cmd.as_mut_ptr(),
        );
        return 1 as libc::c_int;
    }
    *buf.offset(buf_len as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
