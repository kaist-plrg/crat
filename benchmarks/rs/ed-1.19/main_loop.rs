use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn append_lines(
        ibufpp: *mut *const libc::c_char,
        addr: libc::c_int,
        insert: bool_0,
        isglobal: bool_0,
    ) -> bool_0;
    fn close_sbuf() -> bool_0;
    fn copy_lines(
        first_addr_0: libc::c_int,
        second_addr_0: libc::c_int,
        addr: libc::c_int,
    ) -> bool_0;
    fn current_addr() -> libc::c_int;
    fn delete_lines(from: libc::c_int, to: libc::c_int, isglobal: bool_0) -> bool_0;
    fn get_line_node_addr(lp: *const line_t) -> libc::c_int;
    fn join_lines(from: libc::c_int, to: libc::c_int, isglobal: bool_0) -> bool_0;
    fn last_addr() -> libc::c_int;
    fn modified() -> bool_0;
    fn move_lines(
        first_addr_0: libc::c_int,
        second_addr_0: libc::c_int,
        addr: libc::c_int,
        isglobal: bool_0,
    ) -> bool_0;
    fn open_sbuf() -> bool_0;
    fn path_max(filename: *const libc::c_char) -> libc::c_int;
    fn put_lines(addr: libc::c_int) -> bool_0;
    fn search_line_node(addr: libc::c_int) -> *mut line_t;
    fn set_current_addr(addr: libc::c_int);
    fn set_modified(m: bool_0);
    fn yank_lines(from: libc::c_int, to: libc::c_int) -> bool_0;
    fn clear_undo_stack();
    fn reset_undo_state();
    fn undo(isglobal: bool_0) -> bool_0;
    fn next_active_node() -> *const line_t;
    fn get_extended_line(
        ibufpp: *mut *const libc::c_char,
        lenp: *mut libc::c_int,
        strip_escaped_newlines: bool_0,
    ) -> bool_0;
    fn get_stdin_line(sizep: *mut libc::c_int) -> *const libc::c_char;
    fn linenum() -> libc::c_int;
    fn print_lines(from: libc::c_int, to: libc::c_int, pflags: libc::c_int) -> bool_0;
    fn read_file(filename: *const libc::c_char, addr: libc::c_int) -> libc::c_int;
    fn write_file(
        filename: *const libc::c_char,
        mode: *const libc::c_char,
        from: libc::c_int,
        to: libc::c_int,
    ) -> libc::c_int;
    fn interactive() -> bool_0;
    fn may_access_filename(name: *const libc::c_char) -> bool_0;
    fn restricted() -> bool_0;
    fn scripted() -> bool_0;
    fn traditional() -> bool_0;
    fn build_active_list(
        ibufpp: *mut *const libc::c_char,
        first_addr_0: libc::c_int,
        second_addr_0: libc::c_int,
        match_0: bool_0,
    ) -> bool_0;
    fn get_pattern_for_s(ibufpp: *mut *const libc::c_char) -> *const libc::c_char;
    fn extract_replacement(ibufpp: *mut *const libc::c_char, isglobal: bool_0) -> bool_0;
    fn next_matching_node_addr(ibufpp: *mut *const libc::c_char) -> libc::c_int;
    fn search_and_replace(
        first_addr_0: libc::c_int,
        second_addr_0: libc::c_int,
        snum: libc::c_int,
        isglobal: bool_0,
    ) -> bool_0;
    fn set_subst_regex(pat: *const libc::c_char, ignore_case: bool_0) -> bool_0;
    fn replace_subst_re_by_search_re() -> bool_0;
    fn subst_regex() -> bool_0;
    fn disable_interrupts();
    fn enable_interrupts();
    fn resize_buffer(
        buf: *mut *mut libc::c_char,
        size: *mut libc::c_int,
        min_size: libc::c_uint,
    ) -> bool_0;
    fn set_signals();
    fn set_window_lines(lines: libc::c_int);
    fn window_lines() -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
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
pub type Bool = libc::c_uint;
pub const true_0: Bool = 1;
pub const false_0: Bool = 0;
pub type bool_0 = Bool;
pub type Pflags = libc::c_uint;
pub const pf_p: Pflags = 4;
pub const pf_n: Pflags = 2;
pub const pf_l: Pflags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub q_forw: *mut line,
    pub q_back: *mut line,
    pub pos: libc::c_long,
    pub len: libc::c_int,
}
pub type line_t = line;
pub const FATAL: Status = -4;
pub const EMOD: Status = -3;
pub const QUIT: Status = -1;
pub const ERR: Status = -2;
pub const sf_p: Sflags = 2;
pub type Sflags = libc::c_uint;
pub const sf_none: Sflags = 8;
pub const sf_r: Sflags = 4;
pub const sf_g: Sflags = 1;
pub type Status = libc::c_int;
static mut no_prev_subst: *const libc::c_char = b"No previous substitution\0"
    as *const u8 as *const libc::c_char;
static mut inv_com_suf: *const libc::c_char = b"Invalid command suffix\0" as *const u8
    as *const libc::c_char;
static mut inv_mark_ch: *const libc::c_char = b"Invalid mark character\0" as *const u8
    as *const libc::c_char;
static mut no_cur_fn: *const libc::c_char = b"No current filename\0" as *const u8
    as *const libc::c_char;
static mut no_prev_com: *const libc::c_char = b"No previous command\0" as *const u8
    as *const libc::c_char;
static mut def_filename: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
static mut errmsg: [libc::c_char; 80] = unsafe {
    *::std::mem::transmute::<
        &[u8; 80],
        &mut [libc::c_char; 80],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut prompt_str: *const libc::c_char = b"*\0" as *const u8 as *const libc::c_char;
static mut first_addr: libc::c_int = 0 as libc::c_int;
static mut second_addr: libc::c_int = 0 as libc::c_int;
static mut prompt_on: bool_0 = false_0;
static mut verbose: bool_0 = false_0;
pub unsafe extern "C" fn invalid_address() {
    set_error_msg(b"Invalid address\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn set_def_filename(s: *const libc::c_char) -> bool_0 {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let len: libc::c_int = strlen(s) as libc::c_int;
    if resize_buffer(&mut buf, &mut bufsz, (len + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return false_0;
    }
    memcpy(
        buf as *mut libc::c_void,
        s as *const libc::c_void,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    def_filename = buf;
    return true_0;
}
pub unsafe extern "C" fn set_error_msg(msg: *const libc::c_char) {
    strncpy(
        errmsg.as_mut_ptr(),
        msg,
        ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
    );
    errmsg[(::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn set_prompt(s: *const libc::c_char) -> bool_0 {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let len: libc::c_int = strlen(s) as libc::c_int;
    if resize_buffer(&mut buf, &mut bufsz, (len + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return false_0;
    }
    memcpy(
        buf as *mut libc::c_void,
        s as *const libc::c_void,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    prompt_str = buf;
    prompt_on = true_0;
    return true_0;
}
pub unsafe extern "C" fn set_verbose() {
    verbose = true_0;
}
static mut mark: [*const line_t; 26] = [0 as *const line_t; 26];
static mut markno: libc::c_int = 0;
unsafe extern "C" fn mark_line_node(lp: *const line_t, mut c: libc::c_int) -> bool_0 {
    c -= 'a' as i32;
    if c < 0 as libc::c_int || c >= 26 as libc::c_int {
        set_error_msg(inv_mark_ch);
        return false_0;
    }
    if (mark[c as usize]).is_null() {
        markno += 1;
        markno;
    }
    mark[c as usize] = lp;
    return true_0;
}
pub unsafe extern "C" fn unmark_line_node(lp: *const line_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while markno != 0 && i < 26 as libc::c_int {
        if mark[i as usize] == lp {
            mark[i as usize] = 0 as *const line_t;
            markno -= 1;
            markno;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn get_marked_node_addr(mut c: libc::c_int) -> libc::c_int {
    c -= 'a' as i32;
    if c < 0 as libc::c_int || c >= 26 as libc::c_int {
        set_error_msg(inv_mark_ch);
        return -(1 as libc::c_int);
    }
    return get_line_node_addr(mark[c as usize]);
}
unsafe extern "C" fn get_shell_command(
    ibufpp: *mut *const libc::c_char,
) -> *const libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    static mut shcmd: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut shcmdsz: libc::c_int = 0 as libc::c_int;
    static mut shcmdlen: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut replacement: bool_0 = false_0;
    if restricted() as u64 != 0 {
        set_error_msg(b"Shell access restricted\0" as *const u8 as *const libc::c_char);
        return 0 as *const libc::c_char;
    }
    if get_extended_line(ibufpp, &mut len, true_0) as u64 == 0 {
        return 0 as *const libc::c_char;
    }
    if resize_buffer(&mut buf, &mut bufsz, (len + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return 0 as *const libc::c_char;
    }
    if **ibufpp as libc::c_int != '!' as i32 {
        let fresh0 = i;
        i = i + 1;
        *buf.offset(fresh0 as isize) = '!' as i32 as libc::c_char;
    } else {
        if shcmdlen <= 0 as libc::c_int
            || traditional() as libc::c_uint != 0
                && *shcmd.offset(1 as libc::c_int as isize) == 0
        {
            set_error_msg(no_prev_com);
            return 0 as *const libc::c_char;
        }
        memcpy(
            buf as *mut libc::c_void,
            shcmd as *const libc::c_void,
            shcmdlen as libc::c_ulong,
        );
        i += shcmdlen;
        *ibufpp = (*ibufpp).offset(1);
        *ibufpp;
        replacement = true_0;
    }
    while **ibufpp as libc::c_int != '\n' as i32 {
        if **ibufpp as libc::c_int == '%' as i32 {
            if *def_filename.offset(0 as libc::c_int as isize) == 0 {
                set_error_msg(no_cur_fn);
                return 0 as *const libc::c_char;
            }
            len = strlen(def_filename) as libc::c_int;
            if resize_buffer(&mut buf, &mut bufsz, (i + len) as libc::c_uint) as u64 == 0
            {
                return 0 as *const libc::c_char;
            }
            memcpy(
                buf.offset(i as isize) as *mut libc::c_void,
                def_filename as *const libc::c_void,
                len as libc::c_ulong,
            );
            i += len;
            *ibufpp = (*ibufpp).offset(1);
            *ibufpp;
            replacement = true_0;
        } else {
            let fresh1 = *ibufpp;
            *ibufpp = (*ibufpp).offset(1);
            let mut ch: libc::c_char = *fresh1;
            if resize_buffer(
                &mut buf,
                &mut bufsz,
                (i + 2 as libc::c_int) as libc::c_uint,
            ) as u64 == 0
            {
                return 0 as *const libc::c_char;
            }
            if ch as libc::c_int != '\\' as i32 {
                let fresh2 = i;
                i = i + 1;
                *buf.offset(fresh2 as isize) = ch;
            } else {
                let fresh3 = *ibufpp;
                *ibufpp = (*ibufpp).offset(1);
                ch = *fresh3;
                if ch as libc::c_int != '%' as i32 {
                    let fresh4 = i;
                    i = i + 1;
                    *buf.offset(fresh4 as isize) = '\\' as i32 as libc::c_char;
                }
                let fresh5 = i;
                i = i + 1;
                *buf.offset(fresh5 as isize) = ch;
            }
        }
    }
    while **ibufpp as libc::c_int == '\n' as i32 {
        *ibufpp = (*ibufpp).offset(1);
        *ibufpp;
    }
    if resize_buffer(&mut shcmd, &mut shcmdsz, (i + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return 0 as *const libc::c_char;
    }
    memcpy(shcmd as *mut libc::c_void, buf as *const libc::c_void, i as libc::c_ulong);
    *shcmd.offset(i as isize) = 0 as libc::c_int as libc::c_char;
    shcmdlen = i;
    if replacement as u64 != 0 {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            shcmd.offset(1 as libc::c_int as isize),
        );
        fflush(stdout);
    }
    return shcmd;
}
unsafe extern "C" fn skip_blanks(ibufpp: *mut *const libc::c_char) {
    while *(*__ctype_b_loc()).offset(**ibufpp as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        && **ibufpp as libc::c_int != '\n' as i32
    {
        *ibufpp = (*ibufpp).offset(1);
        *ibufpp;
    }
}
unsafe extern "C" fn get_filename(
    ibufpp: *mut *const libc::c_char,
    traditional_f_command: bool_0,
) -> *const libc::c_char {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let pmax: libc::c_int = path_max(0 as *const libc::c_char);
    let mut n: libc::c_int = 0;
    skip_blanks(ibufpp);
    if **ibufpp as libc::c_int != '\n' as i32 {
        let mut size: libc::c_int = 0 as libc::c_int;
        if get_extended_line(ibufpp, &mut size, true_0) as u64 == 0 {
            return 0 as *const libc::c_char;
        }
        if **ibufpp as libc::c_int == '!' as i32 {
            *ibufpp = (*ibufpp).offset(1);
            *ibufpp;
            return get_shell_command(ibufpp);
        } else if size > pmax {
            set_error_msg(b"Filename too long\0" as *const u8 as *const libc::c_char);
            return 0 as *const libc::c_char;
        }
    } else if traditional_f_command as u64 == 0
        && *def_filename.offset(0 as libc::c_int as isize) == 0
    {
        set_error_msg(no_cur_fn);
        return 0 as *const libc::c_char;
    }
    if resize_buffer(&mut buf, &mut bufsz, (pmax + 1 as libc::c_int) as libc::c_uint)
        as u64 == 0
    {
        return 0 as *const libc::c_char;
    }
    n = 0 as libc::c_int;
    while **ibufpp as libc::c_int != '\n' as i32 {
        *buf.offset(n as isize) = **ibufpp;
        n += 1;
        n;
        *ibufpp = (*ibufpp).offset(1);
        *ibufpp;
    }
    *buf.offset(n as isize) = 0 as libc::c_int as libc::c_char;
    while **ibufpp as libc::c_int == '\n' as i32 {
        *ibufpp = (*ibufpp).offset(1);
        *ibufpp;
    }
    return if may_access_filename(buf) as libc::c_uint != 0 {
        buf
    } else {
        0 as *mut libc::c_char
    };
}
unsafe extern "C" fn parse_int(
    i: *mut libc::c_int,
    ibufpp: *mut *const libc::c_char,
) -> bool_0 {
    let mut tail: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    let li: libc::c_long = strtol(*ibufpp, &mut tail, 10 as libc::c_int);
    if tail == *ibufpp as *mut libc::c_char {
        set_error_msg(b"Invalid number\0" as *const u8 as *const libc::c_char);
        return false_0;
    }
    if *__errno_location() == 34 as libc::c_int
        || li > 2147483647 as libc::c_int as libc::c_long
        || li < -(2147483647 as libc::c_int) as libc::c_long
    {
        set_error_msg(b"Number out of range\0" as *const u8 as *const libc::c_char);
        return false_0;
    }
    *ibufpp = tail;
    *i = li as libc::c_int;
    return true_0;
}
unsafe extern "C" fn extract_addresses(ibufpp: *mut *const libc::c_char) -> libc::c_int {
    let mut first: bool_0 = true_0;
    second_addr = -(1 as libc::c_int);
    first_addr = second_addr;
    skip_blanks(ibufpp);
    while true_0 as libc::c_int != 0 {
        let mut n: libc::c_int = 0;
        let ch: libc::c_uchar = **ibufpp as libc::c_uchar;
        if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            if parse_int(&mut n, ibufpp) as u64 == 0 {
                return -(1 as libc::c_int);
            }
            if first as u64 != 0 {
                first = false_0;
                second_addr = n;
            } else {
                second_addr += n;
            }
        } else {
            match ch as libc::c_int {
                9 | 32 => {
                    *ibufpp = (*ibufpp).offset(1);
                    *ibufpp;
                    skip_blanks(ibufpp);
                }
                43 | 45 => {
                    if first as u64 != 0 {
                        first = false_0;
                        second_addr = current_addr();
                    }
                    if *(*__ctype_b_loc())
                        .offset(
                            *(*ibufpp).offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        if parse_int(&mut n, ibufpp) as u64 == 0 {
                            return -(1 as libc::c_int);
                        }
                        second_addr += n;
                    } else {
                        *ibufpp = (*ibufpp).offset(1);
                        *ibufpp;
                        if ch as libc::c_int == '+' as i32 {
                            second_addr += 1;
                            second_addr;
                        } else {
                            second_addr -= 1;
                            second_addr;
                        }
                    }
                }
                46 | 36 => {
                    if first as u64 == 0 {
                        invalid_address();
                        return -(1 as libc::c_int);
                    }
                    first = false_0;
                    *ibufpp = (*ibufpp).offset(1);
                    *ibufpp;
                    second_addr = if ch as libc::c_int == '.' as i32 {
                        current_addr()
                    } else {
                        last_addr()
                    };
                }
                47 | 63 => {
                    if first as u64 == 0 {
                        invalid_address();
                        return -(1 as libc::c_int);
                    }
                    second_addr = next_matching_node_addr(ibufpp);
                    if second_addr < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    first = false_0;
                }
                39 => {
                    if first as u64 == 0 {
                        invalid_address();
                        return -(1 as libc::c_int);
                    }
                    first = false_0;
                    *ibufpp = (*ibufpp).offset(1);
                    *ibufpp;
                    let fresh6 = *ibufpp;
                    *ibufpp = (*ibufpp).offset(1);
                    second_addr = get_marked_node_addr(*fresh6 as libc::c_int);
                    if second_addr < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                }
                37 | 44 | 59 => {
                    if first as u64 != 0 {
                        if first_addr < 0 as libc::c_int {
                            first_addr = if ch as libc::c_int == ';' as i32 {
                                current_addr()
                            } else {
                                1 as libc::c_int
                            };
                            second_addr = last_addr();
                        } else {
                            first_addr = second_addr;
                        }
                    } else {
                        if second_addr < 0 as libc::c_int || second_addr > last_addr() {
                            invalid_address();
                            return -(1 as libc::c_int);
                        }
                        if ch as libc::c_int == ';' as i32 {
                            set_current_addr(second_addr);
                        }
                        first_addr = second_addr;
                        first = true_0;
                    }
                    *ibufpp = (*ibufpp).offset(1);
                    *ibufpp;
                }
                _ => {
                    if first as u64 == 0
                        && (second_addr < 0 as libc::c_int || second_addr > last_addr())
                    {
                        invalid_address();
                        return -(1 as libc::c_int);
                    }
                    let mut addr_cnt: libc::c_int = 0 as libc::c_int;
                    if second_addr >= 0 as libc::c_int {
                        addr_cnt = if first_addr >= 0 as libc::c_int {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        };
                    }
                    if addr_cnt <= 0 as libc::c_int {
                        second_addr = current_addr();
                    }
                    if addr_cnt <= 1 as libc::c_int {
                        first_addr = second_addr;
                    }
                    return addr_cnt;
                }
            }
        }
    }
    panic!();
}
unsafe extern "C" fn get_third_addr(
    ibufpp: *mut *const libc::c_char,
    addr: *mut libc::c_int,
) -> bool_0 {
    let old1: libc::c_int = first_addr;
    let old2: libc::c_int = second_addr;
    let mut addr_cnt: libc::c_int = extract_addresses(ibufpp);
    if addr_cnt < 0 as libc::c_int {
        return false_0;
    }
    if traditional() as libc::c_uint != 0 && addr_cnt == 0 as libc::c_int {
        set_error_msg(b"Destination expected\0" as *const u8 as *const libc::c_char);
        return false_0;
    }
    if second_addr < 0 as libc::c_int || second_addr > last_addr() {
        invalid_address();
        return false_0;
    }
    *addr = second_addr;
    first_addr = old1;
    second_addr = old2;
    return true_0;
}
unsafe extern "C" fn check_addr_range(
    n: libc::c_int,
    m: libc::c_int,
    addr_cnt: libc::c_int,
) -> bool_0 {
    if addr_cnt == 0 as libc::c_int {
        first_addr = n;
        second_addr = m;
    }
    if first_addr < 1 as libc::c_int || first_addr > second_addr
        || second_addr > last_addr()
    {
        invalid_address();
        return false_0;
    }
    return true_0;
}
unsafe extern "C" fn check_addr_range2(addr_cnt: libc::c_int) -> bool_0 {
    return check_addr_range(current_addr(), current_addr(), addr_cnt);
}
unsafe extern "C" fn check_second_addr(
    addr: libc::c_int,
    addr_cnt: libc::c_int,
) -> bool_0 {
    if addr_cnt == 0 as libc::c_int {
        second_addr = addr;
    }
    if second_addr < 1 as libc::c_int || second_addr > last_addr() {
        invalid_address();
        return false_0;
    }
    return true_0;
}
unsafe extern "C" fn get_command_suffix(
    ibufpp: *mut *const libc::c_char,
    pflagsp: *mut libc::c_int,
) -> bool_0 {
    while true_0 as libc::c_int != 0 {
        let ch: libc::c_uchar = **ibufpp as libc::c_uchar;
        if ch as libc::c_int == 'l' as i32 {
            if *pflagsp & pf_l as libc::c_int != 0 {
                break;
            }
            *pflagsp |= pf_l as libc::c_int;
        } else if ch as libc::c_int == 'n' as i32 {
            if *pflagsp & pf_n as libc::c_int != 0 {
                break;
            }
            *pflagsp |= pf_n as libc::c_int;
        } else {
            if !(ch as libc::c_int == 'p' as i32) {
                break;
            }
            if *pflagsp & pf_p as libc::c_int != 0 {
                break;
            }
            *pflagsp |= pf_p as libc::c_int;
        }
        *ibufpp = (*ibufpp).offset(1);
        *ibufpp;
    }
    let fresh7 = *ibufpp;
    *ibufpp = (*ibufpp).offset(1);
    if *fresh7 as libc::c_int != '\n' as i32 {
        set_error_msg(inv_com_suf);
        return false_0;
    }
    return true_0;
}
unsafe extern "C" fn get_command_s_suffix(
    ibufpp: *mut *const libc::c_char,
    pflagsp: *mut libc::c_int,
    snump: *mut libc::c_int,
    ignore_casep: *mut bool_0,
) -> bool_0 {
    let mut rep: bool_0 = false_0;
    let mut error: bool_0 = false_0;
    while true_0 as libc::c_int != 0 {
        let ch: libc::c_uchar = **ibufpp as libc::c_uchar;
        if ch as libc::c_int >= '1' as i32 && ch as libc::c_int <= '9' as i32 {
            let mut n: libc::c_int = 0 as libc::c_int;
            if rep as libc::c_uint != 0 || parse_int(&mut n, ibufpp) as u64 == 0
                || n <= 0 as libc::c_int
            {
                error = true_0;
                break;
            } else {
                rep = true_0;
                *snump = n;
            }
        } else {
            if ch as libc::c_int == 'g' as i32 {
                if rep as u64 != 0 {
                    break;
                }
                rep = true_0;
                *snump = 0 as libc::c_int;
            } else if ch as libc::c_int == 'i' as i32 || ch as libc::c_int == 'I' as i32
            {
                if *ignore_casep as u64 != 0 {
                    break;
                }
                *ignore_casep = true_0;
            } else if ch as libc::c_int == 'l' as i32 {
                if *pflagsp & pf_l as libc::c_int != 0 {
                    break;
                }
                *pflagsp |= pf_l as libc::c_int;
            } else if ch as libc::c_int == 'n' as i32 {
                if *pflagsp & pf_n as libc::c_int != 0 {
                    break;
                }
                *pflagsp |= pf_n as libc::c_int;
            } else {
                if !(ch as libc::c_int == 'p' as i32) {
                    break;
                }
                if *pflagsp & pf_p as libc::c_int != 0 {
                    break;
                }
                *pflagsp |= pf_p as libc::c_int;
            }
            *ibufpp = (*ibufpp).offset(1);
            *ibufpp;
        }
    }
    if error as libc::c_uint != 0
        || {
            let fresh8 = *ibufpp;
            *ibufpp = (*ibufpp).offset(1);
            *fresh8 as libc::c_int != '\n' as i32
        }
    {
        set_error_msg(inv_com_suf);
        return false_0;
    }
    return true_0;
}
unsafe extern "C" fn unexpected_address(addr_cnt: libc::c_int) -> bool_0 {
    if addr_cnt > 0 as libc::c_int {
        set_error_msg(b"Unexpected address\0" as *const u8 as *const libc::c_char);
        return true_0;
    }
    return false_0;
}
unsafe extern "C" fn unexpected_command_suffix(ch: libc::c_uchar) -> bool_0 {
    if *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        set_error_msg(
            b"Unexpected command suffix\0" as *const u8 as *const libc::c_char,
        );
        return true_0;
    }
    return false_0;
}
unsafe extern "C" fn command_s(
    ibufpp: *mut *const libc::c_char,
    pflagsp: *mut libc::c_int,
    addr_cnt: libc::c_int,
    isglobal: bool_0,
) -> bool_0 {
    static mut pflags: libc::c_int = 0 as libc::c_int;
    static mut pmask: libc::c_int = pf_p as libc::c_int;
    static mut snum: libc::c_int = 1 as libc::c_int;
    let mut sflags: Sflags = 0 as Sflags;
    if check_addr_range2(addr_cnt) as u64 == 0 {
        return false_0;
    }
    loop {
        let mut error: bool_0 = false_0;
        if **ibufpp as libc::c_int >= '1' as i32 && **ibufpp as libc::c_int <= '9' as i32
        {
            let mut n: libc::c_int = 0 as libc::c_int;
            if sflags as libc::c_uint & sf_g as libc::c_int as libc::c_uint != 0
                || parse_int(&mut n, ibufpp) as u64 == 0 || n <= 0 as libc::c_int
            {
                error = true_0;
            } else {
                sflags = ::std::mem::transmute::<
                    libc::c_uint,
                    Sflags,
                >(sflags as libc::c_uint | sf_g as libc::c_int as libc::c_uint);
                snum = n;
            }
        } else {
            match **ibufpp as libc::c_int {
                10 => {
                    sflags = ::std::mem::transmute::<
                        libc::c_uint,
                        Sflags,
                    >(sflags as libc::c_uint | sf_none as libc::c_int as libc::c_uint);
                }
                103 => {
                    if sflags as libc::c_uint & sf_g as libc::c_int as libc::c_uint != 0
                    {
                        error = true_0;
                    } else {
                        sflags = ::std::mem::transmute::<
                            libc::c_uint,
                            Sflags,
                        >(sflags as libc::c_uint | sf_g as libc::c_int as libc::c_uint);
                        snum = (snum == 0) as libc::c_int;
                        *ibufpp = (*ibufpp).offset(1);
                        *ibufpp;
                    }
                }
                112 => {
                    if sflags as libc::c_uint & sf_p as libc::c_int as libc::c_uint != 0
                    {
                        error = true_0;
                    } else {
                        sflags = ::std::mem::transmute::<
                            libc::c_uint,
                            Sflags,
                        >(sflags as libc::c_uint | sf_p as libc::c_int as libc::c_uint);
                        *ibufpp = (*ibufpp).offset(1);
                        *ibufpp;
                    }
                }
                114 => {
                    if sflags as libc::c_uint & sf_r as libc::c_int as libc::c_uint != 0
                    {
                        error = true_0;
                    } else {
                        sflags = ::std::mem::transmute::<
                            libc::c_uint,
                            Sflags,
                        >(sflags as libc::c_uint | sf_r as libc::c_int as libc::c_uint);
                        *ibufpp = (*ibufpp).offset(1);
                        *ibufpp;
                    }
                }
                _ => {
                    if sflags as u64 != 0 {
                        error = true_0;
                    }
                }
            }
        }
        if error as u64 != 0 {
            set_error_msg(inv_com_suf);
            return false_0;
        }
        if !(sflags as libc::c_uint != 0 && **ibufpp as libc::c_int != '\n' as i32) {
            break;
        }
    }
    if sflags as u64 != 0 {
        if subst_regex() as u64 == 0 {
            set_error_msg(no_prev_subst);
            return false_0;
        }
        if sflags as libc::c_uint & sf_r as libc::c_int as libc::c_uint != 0
            && replace_subst_re_by_search_re() as u64 == 0
        {
            return false_0;
        }
        if sflags as libc::c_uint & sf_p as libc::c_int as libc::c_uint != 0 {
            pflags ^= pmask;
        }
    } else {
        let mut pat: *const libc::c_char = get_pattern_for_s(ibufpp);
        if pat.is_null() {
            return false_0;
        }
        let delimiter: libc::c_char = **ibufpp;
        if extract_replacement(ibufpp, isglobal) as u64 == 0 {
            return false_0;
        }
        pflags = 0 as libc::c_int;
        snum = 1 as libc::c_int;
        let mut ignore_case: bool_0 = false_0;
        if **ibufpp as libc::c_int == '\n' as i32 {
            pflags = pf_p as libc::c_int;
        } else {
            if **ibufpp as libc::c_int == delimiter as libc::c_int {
                *ibufpp = (*ibufpp).offset(1);
                *ibufpp;
            }
            if get_command_s_suffix(ibufpp, &mut pflags, &mut snum, &mut ignore_case)
                as u64 == 0
            {
                return false_0;
            }
        }
        pmask = pflags
            & (pf_l as libc::c_int | pf_n as libc::c_int | pf_p as libc::c_int);
        if pmask == 0 as libc::c_int {
            pmask = pf_p as libc::c_int;
        }
        if set_subst_regex(pat, ignore_case) as u64 == 0 {
            return false_0;
        }
    }
    *pflagsp = pflags;
    if isglobal as u64 == 0 {
        clear_undo_stack();
    }
    if search_and_replace(first_addr, second_addr, snum, isglobal) as u64 == 0 {
        return false_0;
    }
    return true_0;
}
unsafe extern "C" fn exec_command(
    ibufpp: *mut *const libc::c_char,
    prev_status: libc::c_int,
    isglobal: bool_0,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fnp: *const libc::c_char = 0 as *const libc::c_char;
    let mut pflags: libc::c_int = 0 as libc::c_int;
    let mut addr: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let addr_cnt: libc::c_int = extract_addresses(ibufpp);
    if addr_cnt < 0 as libc::c_int {
        return ERR as libc::c_int;
    }
    skip_blanks(ibufpp);
    let fresh9 = *ibufpp;
    *ibufpp = (*ibufpp).offset(1);
    c = *fresh9 as libc::c_int;
    match c {
        97 => {
            if get_command_suffix(ibufpp, &mut pflags) as u64 == 0 {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if append_lines(ibufpp, second_addr, false_0, isglobal) as u64 == 0 {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        99 => {
            if check_addr_range2(addr_cnt) as u64 == 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if delete_lines(first_addr, second_addr, isglobal) as u64 == 0
                || append_lines(
                    ibufpp,
                    current_addr(),
                    (current_addr() >= first_addr) as libc::c_int as bool_0,
                    isglobal,
                ) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        100 => {
            if check_addr_range2(addr_cnt) as u64 == 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if delete_lines(first_addr, second_addr, isglobal) as u64 == 0 {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        101 => {
            if modified() as libc::c_uint != 0 && prev_status != EMOD as libc::c_int {
                return EMOD as libc::c_int;
            }
            current_block = 7179728134253626143;
        }
        69 => {
            current_block = 7179728134253626143;
        }
        102 => {
            if unexpected_address(addr_cnt) as libc::c_uint != 0
                || unexpected_command_suffix(**ibufpp as libc::c_uchar) as libc::c_uint
                    != 0
            {
                return ERR as libc::c_int;
            }
            fnp = get_filename(ibufpp, traditional());
            if fnp.is_null() {
                return ERR as libc::c_int;
            }
            if *fnp.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32 {
                set_error_msg(
                    b"Invalid redirection\0" as *const u8 as *const libc::c_char,
                );
                return ERR as libc::c_int;
            }
            if *fnp.offset(0 as libc::c_int as isize) as libc::c_int != 0
                && set_def_filename(fnp) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, def_filename);
            current_block = 5482373152242628851;
        }
        103 | 118 | 71 | 86 => {
            if isglobal as u64 != 0 {
                set_error_msg(
                    b"Cannot nest global commands\0" as *const u8 as *const libc::c_char,
                );
                return ERR as libc::c_int;
            }
            n = (c == 'g' as i32 || c == 'G' as i32) as libc::c_int;
            if check_addr_range(1 as libc::c_int, last_addr(), addr_cnt) as u64 == 0
                || build_active_list(ibufpp, first_addr, second_addr, n as bool_0) as u64
                    == 0
            {
                return ERR as libc::c_int;
            }
            n = (c == 'G' as i32 || c == 'V' as i32) as libc::c_int;
            if n != 0 && get_command_suffix(ibufpp, &mut pflags) as u64 == 0 {
                return ERR as libc::c_int;
            }
            n = exec_global(ibufpp, pflags, n as bool_0);
            if n != 0 as libc::c_int {
                return n;
            }
            current_block = 5482373152242628851;
        }
        104 | 72 => {
            if unexpected_address(addr_cnt) as libc::c_uint != 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if c == 'H' as i32 {
                verbose = (verbose as u64 == 0) as libc::c_int as bool_0;
            }
            if (c == 'h' as i32 || verbose as libc::c_uint != 0)
                && errmsg[0 as libc::c_int as usize] as libc::c_int != 0
            {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    errmsg.as_mut_ptr(),
                );
            }
            current_block = 5482373152242628851;
        }
        105 => {
            if get_command_suffix(ibufpp, &mut pflags) as u64 == 0 {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if append_lines(ibufpp, second_addr, true_0, isglobal) as u64 == 0 {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        106 => {
            if check_addr_range(
                current_addr(),
                current_addr() + 1 as libc::c_int,
                addr_cnt,
            ) as u64 == 0 || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if first_addr < second_addr
                && join_lines(first_addr, second_addr, isglobal) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        107 => {
            let fresh10 = *ibufpp;
            *ibufpp = (*ibufpp).offset(1);
            n = *fresh10 as libc::c_int;
            if second_addr == 0 as libc::c_int {
                invalid_address();
                return ERR as libc::c_int;
            }
            if get_command_suffix(ibufpp, &mut pflags) as u64 == 0
                || mark_line_node(search_line_node(second_addr), n) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        108 => {
            n = pf_l as libc::c_int;
            current_block = 4542414805295425712;
        }
        110 => {
            n = pf_n as libc::c_int;
            current_block = 4542414805295425712;
        }
        112 => {
            n = pf_p as libc::c_int;
            current_block = 4542414805295425712;
        }
        109 => {
            if check_addr_range2(addr_cnt) as u64 == 0
                || get_third_addr(ibufpp, &mut addr) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if addr >= first_addr && addr < second_addr {
                set_error_msg(
                    b"Invalid destination\0" as *const u8 as *const libc::c_char,
                );
                return ERR as libc::c_int;
            }
            if get_command_suffix(ibufpp, &mut pflags) as u64 == 0 {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if move_lines(first_addr, second_addr, addr, isglobal) as u64 == 0 {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        80 | 113 | 81 => {
            if unexpected_address(addr_cnt) as libc::c_uint != 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if c == 'P' as i32 {
                prompt_on = (prompt_on as u64 == 0) as libc::c_int as bool_0;
            } else if c == 'q' as i32 && modified() as libc::c_uint != 0
                && prev_status != EMOD as libc::c_int
            {
                return EMOD as libc::c_int
            } else {
                return QUIT as libc::c_int
            }
            current_block = 5482373152242628851;
        }
        114 => {
            if unexpected_command_suffix(**ibufpp as libc::c_uchar) as u64 != 0 {
                return ERR as libc::c_int;
            }
            if addr_cnt == 0 as libc::c_int {
                second_addr = last_addr();
            }
            fnp = get_filename(ibufpp, false_0);
            if fnp.is_null() {
                return ERR as libc::c_int;
            }
            if *def_filename.offset(0 as libc::c_int as isize) == 0
                && *fnp.offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32
                && set_def_filename(fnp) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            addr = read_file(
                if *fnp.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    fnp
                } else {
                    def_filename
                },
                second_addr,
            );
            if addr < 0 as libc::c_int {
                return ERR as libc::c_int;
            }
            if addr != 0 {
                set_modified(true_0);
            }
            current_block = 5482373152242628851;
        }
        115 => {
            if command_s(ibufpp, &mut pflags, addr_cnt, isglobal) as u64 == 0 {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        116 => {
            if check_addr_range2(addr_cnt) as u64 == 0
                || get_third_addr(ibufpp, &mut addr) as u64 == 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if copy_lines(first_addr, second_addr, addr) as u64 == 0 {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        117 => {
            if unexpected_address(addr_cnt) as libc::c_uint != 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
                || undo(isglobal) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        119 | 87 => {
            n = **ibufpp as libc::c_int;
            if n == 'q' as i32 || n == 'Q' as i32 {
                *ibufpp = (*ibufpp).offset(1);
                *ibufpp;
            }
            if unexpected_command_suffix(**ibufpp as libc::c_uchar) as u64 != 0 {
                return ERR as libc::c_int;
            }
            fnp = get_filename(ibufpp, false_0);
            if fnp.is_null() {
                return ERR as libc::c_int;
            }
            if addr_cnt == 0 as libc::c_int && last_addr() == 0 as libc::c_int {
                second_addr = 0 as libc::c_int;
                first_addr = second_addr;
            } else if check_addr_range(1 as libc::c_int, last_addr(), addr_cnt) as u64
                == 0
            {
                return ERR as libc::c_int
            }
            if *def_filename.offset(0 as libc::c_int as isize) == 0
                && *fnp.offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32
                && set_def_filename(fnp) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            addr = write_file(
                if *fnp.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    fnp
                } else {
                    def_filename
                },
                if c == 'W' as i32 {
                    b"a\0" as *const u8 as *const libc::c_char
                } else {
                    b"w\0" as *const u8 as *const libc::c_char
                },
                first_addr,
                second_addr,
            );
            if addr < 0 as libc::c_int {
                return ERR as libc::c_int;
            }
            if addr == last_addr()
                && *fnp.offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32
            {
                set_modified(false_0);
            } else if n == 'q' as i32 && modified() as libc::c_uint != 0
                && prev_status != EMOD as libc::c_int
            {
                return EMOD as libc::c_int
            }
            if n == 'q' as i32 || n == 'Q' as i32 {
                return QUIT as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        120 => {
            if second_addr < 0 as libc::c_int || second_addr > last_addr() {
                invalid_address();
                return ERR as libc::c_int;
            }
            if get_command_suffix(ibufpp, &mut pflags) as u64 == 0 {
                return ERR as libc::c_int;
            }
            if isglobal as u64 == 0 {
                clear_undo_stack();
            }
            if put_lines(second_addr) as u64 == 0 {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        121 => {
            if check_addr_range2(addr_cnt) as u64 == 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
                || yank_lines(first_addr, second_addr) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        122 => {
            if check_second_addr(
                current_addr() + (isglobal as u64 == 0) as libc::c_int,
                addr_cnt,
            ) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if **ibufpp as libc::c_int > '0' as i32
                && **ibufpp as libc::c_int <= '9' as i32
            {
                if parse_int(&mut n, ibufpp) as u64 != 0 {
                    set_window_lines(n);
                } else {
                    return ERR as libc::c_int
                }
            }
            if get_command_suffix(ibufpp, &mut pflags) as u64 == 0
                || print_lines(
                    second_addr,
                    (if last_addr() < second_addr + window_lines() - 1 as libc::c_int {
                        last_addr()
                    } else {
                        second_addr + window_lines() - 1 as libc::c_int
                    }),
                    pflags,
                ) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            pflags = 0 as libc::c_int;
            current_block = 5482373152242628851;
        }
        61 => {
            if get_command_suffix(ibufpp, &mut pflags) as u64 == 0 {
                return ERR as libc::c_int;
            }
            printf(
                b"%d\n\0" as *const u8 as *const libc::c_char,
                if addr_cnt != 0 { second_addr } else { last_addr() },
            );
            current_block = 5482373152242628851;
        }
        33 => {
            if unexpected_address(addr_cnt) as u64 != 0 {
                return ERR as libc::c_int;
            }
            fnp = get_shell_command(ibufpp);
            if fnp.is_null() {
                return ERR as libc::c_int;
            }
            if system(fnp.offset(1 as libc::c_int as isize)) < 0 as libc::c_int {
                set_error_msg(
                    b"Can't create shell process\0" as *const u8 as *const libc::c_char,
                );
                return ERR as libc::c_int;
            }
            if scripted() as u64 == 0 {
                fputs(b"!\n\0" as *const u8 as *const libc::c_char, stdout);
            }
            current_block = 5482373152242628851;
        }
        10 => {
            if check_second_addr(
                current_addr()
                    + (traditional() as libc::c_uint != 0 || isglobal as u64 == 0)
                        as libc::c_int,
                addr_cnt,
            ) as u64 == 0
                || print_lines(second_addr, second_addr, 0 as libc::c_int) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            current_block = 5482373152242628851;
        }
        35 => {
            loop {
                let fresh11 = *ibufpp;
                *ibufpp = (*ibufpp).offset(1);
                if !(*fresh11 as libc::c_int != '\n' as i32) {
                    break;
                }
            }
            current_block = 5482373152242628851;
        }
        _ => {
            set_error_msg(b"Unknown command\0" as *const u8 as *const libc::c_char);
            return ERR as libc::c_int;
        }
    }
    match current_block {
        4542414805295425712 => {
            if check_addr_range2(addr_cnt) as u64 == 0
                || get_command_suffix(ibufpp, &mut pflags) as u64 == 0
                || print_lines(first_addr, second_addr, pflags | n) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            pflags = 0 as libc::c_int;
        }
        7179728134253626143 => {
            if unexpected_address(addr_cnt) as libc::c_uint != 0
                || unexpected_command_suffix(**ibufpp as libc::c_uchar) as libc::c_uint
                    != 0
            {
                return ERR as libc::c_int;
            }
            fnp = get_filename(ibufpp, false_0);
            if fnp.is_null()
                || delete_lines(1 as libc::c_int, last_addr(), isglobal) as u64 == 0
                || close_sbuf() as u64 == 0
            {
                return ERR as libc::c_int;
            }
            set_modified(false_0);
            if open_sbuf() as u64 == 0 {
                return FATAL as libc::c_int;
            }
            if *fnp.offset(0 as libc::c_int as isize) as libc::c_int != 0
                && *fnp.offset(0 as libc::c_int as isize) as libc::c_int != '!' as i32
                && set_def_filename(fnp) as u64 == 0
            {
                return ERR as libc::c_int;
            }
            if read_file(
                (if *fnp.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                    fnp
                } else {
                    def_filename
                }),
                0 as libc::c_int,
            ) < 0 as libc::c_int
            {
                return ERR as libc::c_int;
            }
            reset_undo_state();
        }
        _ => {}
    }
    if pflags != 0 && print_lines(current_addr(), current_addr(), pflags) as u64 == 0 {
        return ERR as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_global(
    ibufpp: *mut *const libc::c_char,
    pflags: libc::c_int,
    interactive_0: bool_0,
) -> libc::c_int {
    static mut buf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut bufsz: libc::c_int = 0 as libc::c_int;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    if interactive_0 as u64 == 0 {
        if traditional() as libc::c_uint != 0
            && strcmp(*ibufpp, b"\n\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            cmd = b"p\n\0" as *const u8 as *const libc::c_char;
        } else {
            if get_extended_line(ibufpp, 0 as *mut libc::c_int, false_0) as u64 == 0 {
                return ERR as libc::c_int;
            }
            cmd = *ibufpp;
        }
    }
    clear_undo_stack();
    while true_0 as libc::c_int != 0 {
        let lp: *const line_t = next_active_node();
        if lp.is_null() {
            break;
        }
        set_current_addr(get_line_node_addr(lp));
        if current_addr() < 0 as libc::c_int {
            return ERR as libc::c_int;
        }
        if interactive_0 as u64 != 0 {
            let mut len: libc::c_int = 0 as libc::c_int;
            if print_lines(current_addr(), current_addr(), pflags) as u64 == 0 {
                return ERR as libc::c_int;
            }
            *ibufpp = get_stdin_line(&mut len);
            if (*ibufpp).is_null() {
                return ERR as libc::c_int;
            }
            if len <= 0 as libc::c_int {
                return ERR as libc::c_int;
            }
            if len == 1 as libc::c_int
                && strcmp(*ibufpp, b"\n\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                continue;
            }
            if len == 2 as libc::c_int
                && strcmp(*ibufpp, b"&\n\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                if cmd.is_null() {
                    set_error_msg(no_prev_com);
                    return ERR as libc::c_int;
                }
            } else {
                if get_extended_line(ibufpp, &mut len, false_0) as u64 == 0
                    || resize_buffer(
                        &mut buf,
                        &mut bufsz,
                        (len + 1 as libc::c_int) as libc::c_uint,
                    ) as u64 == 0
                {
                    return ERR as libc::c_int;
                }
                memcpy(
                    buf as *mut libc::c_void,
                    *ibufpp as *const libc::c_void,
                    (len + 1 as libc::c_int) as libc::c_ulong,
                );
                cmd = buf;
            }
        }
        *ibufpp = cmd;
        while **ibufpp != 0 {
            let status: libc::c_int = exec_command(ibufpp, 0 as libc::c_int, true_0);
            if status != 0 as libc::c_int {
                return status;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn main_loop(initial_error: bool_0, loose: bool_0) -> libc::c_int {
    extern "C" {
        static mut jmp_state: jmp_buf;
    }
    let mut ibufp: *const libc::c_char = 0 as *const libc::c_char;
    let mut err_status: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut status: libc::c_int = 0;
    disable_interrupts();
    set_signals();
    status = _setjmp(jmp_state.as_mut_ptr());
    if status == 0 as libc::c_int {
        enable_interrupts();
        if initial_error as u64 != 0 {
            status = -(1 as libc::c_int);
            ::std::ptr::write_volatile(
                &mut err_status as *mut libc::c_int,
                1 as libc::c_int,
            );
        }
    } else {
        status = -(1 as libc::c_int);
        fputs(b"\n?\n\0" as *const u8 as *const libc::c_char, stdout);
        set_error_msg(b"Interrupt\0" as *const u8 as *const libc::c_char);
    }
    while true_0 as libc::c_int != 0 {
        fflush(stdout);
        fflush(stderr);
        if status < 0 as libc::c_int && verbose as libc::c_uint != 0 {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, errmsg.as_mut_ptr());
            fflush(stdout);
        }
        if prompt_on as u64 != 0 {
            fputs(prompt_str, stdout);
            fflush(stdout);
        }
        ibufp = get_stdin_line(&mut len);
        if ibufp.is_null() {
            return 2 as libc::c_int;
        }
        if len <= 0 as libc::c_int {
            if modified() as u64 == 0 || status == EMOD as libc::c_int {
                status = QUIT as libc::c_int;
            } else {
                status = EMOD as libc::c_int;
                if loose as u64 == 0 {
                    ::std::ptr::write_volatile(
                        &mut err_status as *mut libc::c_int,
                        2 as libc::c_int,
                    );
                }
            }
        } else {
            status = exec_command(&mut ibufp, status, false_0);
        }
        if status == 0 as libc::c_int {
            continue;
        }
        if status == QUIT as libc::c_int {
            return err_status;
        }
        fputs(b"?\n\0" as *const u8 as *const libc::c_char, stdout);
        if loose as u64 == 0 && err_status == 0 as libc::c_int {
            ::std::ptr::write_volatile(
                &mut err_status as *mut libc::c_int,
                1 as libc::c_int,
            );
        }
        if status == EMOD as libc::c_int {
            set_error_msg(
                b"Warning: buffer modified\0" as *const u8 as *const libc::c_char,
            );
        }
        if interactive() as u64 == 0 {
            if verbose as u64 != 0 {
                printf(
                    b"script, line %d: %s\n\0" as *const u8 as *const libc::c_char,
                    linenum(),
                    errmsg.as_mut_ptr(),
                );
            }
            return if status == FATAL as libc::c_int {
                1 as libc::c_int
            } else {
                err_status
            };
        }
        if status == FATAL as libc::c_int {
            if verbose as u64 != 0 {
                printf(
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    errmsg.as_mut_ptr(),
                );
            }
            return 1 as libc::c_int;
        }
    }
    panic!();
}
