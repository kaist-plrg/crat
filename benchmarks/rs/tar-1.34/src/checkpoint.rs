use ::libc;
extern "C" {
    pub type tm_zone;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wordsplit_node;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __overflow(_: *mut FILE, _: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn fatal_exit() -> !;
    static mut checkpoint_option: libc::c_uint;
    static mut archive_name_cursor: *mut *const libc::c_char;
    static mut stdlis: *mut FILE;
    fn compute_duration() -> libc::c_double;
    fn print_total_stats();
    fn format_total_stats(
        fp: *mut FILE,
        formats: *const *const libc::c_char,
        eor: libc::c_int,
        eol: libc::c_int,
    ) -> libc::c_int;
    fn unquote_string(str: *mut libc::c_char) -> libc::c_int;
    fn decode_signal(_: *const libc::c_char) -> libc::c_int;
    fn sys_exec_checkpoint_script(
        script_name: *const libc::c_char,
        archive_name: *const libc::c_char,
        checkpoint_number: libc::c_int,
    );
    static mut program_name: *const libc::c_char;
    fn wordsplit(
        s: *const libc::c_char,
        ws: *mut wordsplit_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const libc::c_char;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn fprintftime(
        fp: *mut FILE,
        fmt: *const libc::c_char,
        tm: *const tm,
        zone: timezone_t,
        nanoseconds: libc::c_int,
    ) -> size_t;
}
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type timezone_t = *mut tm_zone;
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub time: time_t,
    pub command: *mut libc::c_char,
    pub signal: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct checkpoint_action {
    pub next: *mut checkpoint_action,
    pub opcode: checkpoint_opcode,
    pub v: C2RustUnnamed,
}
pub type checkpoint_opcode = libc::c_uint;
pub const cop_wait: checkpoint_opcode = 7;
pub const cop_totals: checkpoint_opcode = 6;
pub const cop_exec: checkpoint_opcode = 5;
pub const cop_sleep: checkpoint_opcode = 4;
pub const cop_ttyout: checkpoint_opcode = 3;
pub const cop_echo: checkpoint_opcode = 2;
pub const cop_bell: checkpoint_opcode = 1;
pub const cop_dot: checkpoint_opcode = 0;
pub const CHKP_COMPILE: C2RustUnnamed_0 = 1;
pub const CHKP_INIT: C2RustUnnamed_0 = 0;
pub const CHKP_RUN: C2RustUnnamed_0 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut libc::c_char,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: libc::c_uint,
    pub ws_options: libc::c_uint,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const libc::c_char,
    pub ws_comment: *const libc::c_char,
    pub ws_escape: [*const libc::c_char; 2],
    pub ws_alloc_die: Option::<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_debug: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_env: *mut *const libc::c_char,
    pub ws_envbuf: *mut *mut libc::c_char,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut *mut libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_input: *const libc::c_char,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: libc::c_int,
    pub ws_usererr: *mut libc::c_char,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: libc::c_int,
}
pub type wordsplit_t = wordsplit;
pub type C2RustUnnamed_0 = libc::c_uint;
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
static mut checkpoint: libc::c_uint = 0;
static mut checkpoint_action: *mut checkpoint_action = 0 as *const checkpoint_action
    as *mut checkpoint_action;
static mut checkpoint_action_tail: *mut checkpoint_action = 0 as *const checkpoint_action
    as *mut checkpoint_action;
static mut checkpoint_state: libc::c_int = 0;
static mut sigs: sigset_t = sigset_t { __val: [0; 16] };
unsafe extern "C" fn alloc_action(
    mut opcode: checkpoint_opcode,
) -> *mut checkpoint_action {
    let mut p: *mut checkpoint_action = xzalloc(
        ::std::mem::size_of::<checkpoint_action>() as libc::c_ulong,
    ) as *mut checkpoint_action;
    if !checkpoint_action_tail.is_null() {
        (*checkpoint_action_tail).next = p;
    } else {
        checkpoint_action = p;
    }
    checkpoint_action_tail = p;
    (*p).opcode = opcode;
    return p;
}
unsafe extern "C" fn copy_string_unquote(
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    let mut output: *mut libc::c_char = xstrdup(str);
    let mut len: size_t = strlen(output);
    if (*output as libc::c_int == '"' as i32 || *output as libc::c_int == '\'' as i32)
        && *output.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == *output as libc::c_int
    {
        memmove(
            output as *mut libc::c_void,
            output.offset(1 as libc::c_int as isize) as *const libc::c_void,
            len.wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
        *output
            .offset(
                len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    unquote_string(output);
    return output;
}
pub unsafe extern "C" fn checkpoint_compile_action(mut str: *const libc::c_char) {
    let mut act: *mut checkpoint_action = 0 as *mut checkpoint_action;
    if checkpoint_state == CHKP_INIT as libc::c_int {
        sigemptyset(&mut sigs);
        checkpoint_state = CHKP_COMPILE as libc::c_int;
    }
    if strcmp(str, b".\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(str, b"dot\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        alloc_action(cop_dot);
    } else if strcmp(str, b"bell\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        alloc_action(cop_bell);
    } else if strcmp(str, b"echo\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        alloc_action(cop_echo);
    } else if strncmp(
        str,
        b"echo=\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        act = alloc_action(cop_echo);
        (*act).v.command = copy_string_unquote(str.offset(5 as libc::c_int as isize));
    } else if strncmp(
        str,
        b"exec=\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        act = alloc_action(cop_exec);
        (*act).v.command = copy_string_unquote(str.offset(5 as libc::c_int as isize));
    } else if strncmp(
        str,
        b"ttyout=\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        act = alloc_action(cop_ttyout);
        (*act).v.command = copy_string_unquote(str.offset(7 as libc::c_int as isize));
    } else if strncmp(
        str,
        b"sleep=\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n: time_t = strtoul(
            str.offset(6 as libc::c_int as isize),
            &mut p,
            10 as libc::c_int,
        ) as time_t;
        if *p != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: not a valid timeout\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                str,
            );
            fatal_exit();
        }
        act = alloc_action(cop_sleep);
        (*act).v.time = n;
    } else if strcmp(str, b"totals\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        alloc_action(cop_totals);
    } else if strncmp(
        str,
        b"wait=\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        act = alloc_action(cop_wait);
        (*act).v.signal = decode_signal(str.offset(5 as libc::c_int as isize));
        sigaddset(&mut sigs, (*act).v.signal);
    } else {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: unknown checkpoint action\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            str,
        );
        fatal_exit();
    };
}
pub unsafe extern "C" fn checkpoint_finish_compile() {
    if checkpoint_state == CHKP_INIT as libc::c_int && checkpoint_option != 0
        && checkpoint_action.is_null()
    {
        checkpoint_compile_action(b"echo\0" as *const u8 as *const libc::c_char);
    }
    if checkpoint_state == CHKP_COMPILE as libc::c_int {
        sigprocmask(0 as libc::c_int, &mut sigs, 0 as *mut sigset_t);
        if checkpoint_option == 0 {
            checkpoint_option = 10 as libc::c_int as libc::c_uint;
        }
        checkpoint_state = CHKP_RUN as libc::c_int;
    }
}
static mut checkpoint_total_format: [*const libc::c_char; 3] = [
    b"R\0" as *const u8 as *const libc::c_char,
    b"W\0" as *const u8 as *const libc::c_char,
    b"D\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn getwidth(mut fp: *mut FILE) -> libc::c_long {
    let mut columns: *const libc::c_char = 0 as *const libc::c_char;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if ioctl(fileno(fp), 0x5413 as libc::c_int as libc::c_ulong, &mut ws as *mut winsize)
        == 0 as libc::c_int && (0 as libc::c_int) < ws.ws_col as libc::c_int
    {
        return ws.ws_col as libc::c_long;
    }
    columns = getenv(b"COLUMNS\0" as *const u8 as *const libc::c_char);
    if !columns.is_null() {
        let mut col: libc::c_long = strtol(
            columns,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        if (0 as libc::c_int as libc::c_long) < col {
            return col;
        }
    }
    return 80 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn getarg(
    mut input: *const libc::c_char,
    mut endp: *mut *const libc::c_char,
    mut argbuf: *mut *mut libc::c_char,
    mut arglen: *mut size_t,
) -> *mut libc::c_char {
    if *input.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32 {
        let mut p: *mut libc::c_char = strchr(
            input.offset(1 as libc::c_int as isize),
            '}' as i32,
        );
        if !p.is_null() {
            let mut n: size_t = p.offset_from(input) as libc::c_long as size_t;
            if n > *arglen {
                *arglen = n;
                *argbuf = xrealloc(*argbuf as *mut libc::c_void, *arglen)
                    as *mut libc::c_char;
            }
            n = n.wrapping_sub(1);
            n;
            memcpy(
                *argbuf as *mut libc::c_void,
                input.offset(1 as libc::c_int as isize) as *const libc::c_void,
                n,
            );
            *(*argbuf).offset(n as isize) = 0 as libc::c_int as libc::c_char;
            *endp = p.offset(1 as libc::c_int as isize);
            return *argbuf;
        }
    }
    *endp = input;
    return 0 as *mut libc::c_char;
}
static mut tty_cleanup: libc::c_int = 0;
static mut def_format: *const libc::c_char = b"%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn format_checkpoint_string(
    mut fp: *mut FILE,
    mut len: size_t,
    mut input: *const libc::c_char,
    mut do_write: bool,
    mut cpn: libc::c_uint,
) -> libc::c_int {
    let mut opstr: *const libc::c_char = if do_write as libc::c_int != 0 {
        dcgettext(
            0 as *const libc::c_char,
            b"write\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        )
    } else {
        dcgettext(
            0 as *const libc::c_char,
            b"read\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        )
    };
    let mut uintbuf: [libc::c_char; 21] = [0; 21];
    let mut cps: *mut libc::c_char = umaxtostr(cpn as uintmax_t, uintbuf.as_mut_ptr());
    let mut ip: *const libc::c_char = 0 as *const libc::c_char;
    static mut argbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut arglen: size_t = 0 as libc::c_int as size_t;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    if input.is_null() {
        if do_write {
            input = dcgettext(
                0 as *const libc::c_char,
                b"Write checkpoint %u\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else {
            input = dcgettext(
                0 as *const libc::c_char,
                b"Read checkpoint %u\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
    }
    let mut current_block_56: u64;
    ip = input;
    while *ip != 0 {
        if *ip as libc::c_int == '%' as i32 {
            ip = ip.offset(1);
            if *ip as libc::c_int == '{' as i32 {
                arg = getarg(ip, &mut ip, &mut argbuf, &mut arglen);
                if arg.is_null() {
                    fputc_unlocked('%' as i32, fp);
                    fputc_unlocked(*ip as libc::c_int, fp);
                    len = (len as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                    current_block_56 = 7815301370352969686;
                } else {
                    current_block_56 = 10048703153582371463;
                }
            } else {
                current_block_56 = 10048703153582371463;
            }
            match current_block_56 {
                7815301370352969686 => {}
                _ => {
                    match *ip as libc::c_int {
                        99 => {
                            len = (len as libc::c_ulong)
                                .wrapping_add(
                                    format_checkpoint_string(fp, len, def_format, do_write, cpn)
                                        as libc::c_ulong,
                                ) as size_t as size_t;
                        }
                        117 => {
                            fputs_unlocked(cps, fp);
                            len = (len as libc::c_ulong).wrapping_add(strlen(cps))
                                as size_t as size_t;
                        }
                        115 => {
                            fputs_unlocked(opstr, fp);
                            len = (len as libc::c_ulong).wrapping_add(strlen(opstr))
                                as size_t as size_t;
                        }
                        100 => {
                            len = (len as libc::c_ulong)
                                .wrapping_add(
                                    fprintf(
                                        fp,
                                        b"%.0f\0" as *const u8 as *const libc::c_char,
                                        compute_duration(),
                                    ) as libc::c_ulong,
                                ) as size_t as size_t;
                        }
                        84 => {
                            let mut fmt: *mut *const libc::c_char = checkpoint_total_format
                                .as_mut_ptr();
                            let mut fmtbuf: [*const libc::c_char; 3] = [0
                                as *const libc::c_char; 3];
                            let mut ws: wordsplit = wordsplit {
                                ws_wordc: 0,
                                ws_wordv: 0 as *mut *mut libc::c_char,
                                ws_offs: 0,
                                ws_wordn: 0,
                                ws_flags: 0,
                                ws_options: 0,
                                ws_maxwords: 0,
                                ws_wordi: 0,
                                ws_delim: 0 as *const libc::c_char,
                                ws_comment: 0 as *const libc::c_char,
                                ws_escape: [0 as *const libc::c_char; 2],
                                ws_alloc_die: None,
                                ws_error: None,
                                ws_debug: None,
                                ws_env: 0 as *mut *const libc::c_char,
                                ws_envbuf: 0 as *mut *mut libc::c_char,
                                ws_envidx: 0,
                                ws_envsiz: 0,
                                ws_getvar: None,
                                ws_closure: 0 as *mut libc::c_void,
                                ws_command: None,
                                ws_input: 0 as *const libc::c_char,
                                ws_len: 0,
                                ws_endp: 0,
                                ws_errno: 0,
                                ws_usererr: 0 as *mut libc::c_char,
                                ws_head: 0 as *mut wordsplit_node,
                                ws_tail: 0 as *mut wordsplit_node,
                                ws_lvl: 0,
                            };
                            compute_duration();
                            if !arg.is_null() {
                                ws.ws_delim = b",\0" as *const u8 as *const libc::c_char;
                                if wordsplit(
                                    arg,
                                    &mut ws,
                                    (0x40 as libc::c_int | 0x4 as libc::c_int
                                        | (0x200 as libc::c_int | 0x400 as libc::c_int)
                                        | 0x4000 as libc::c_int) as libc::c_uint,
                                ) != 0
                                {
                                    if error_hook.is_some() {
                                        error_hook.unwrap()();
                                    }
                                    error(
                                        0 as libc::c_int,
                                        0 as libc::c_int,
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"cannot split string '%s': %s\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        arg,
                                        wordsplit_strerror(&mut ws),
                                    );
                                    exit_status = 2 as libc::c_int;
                                } else {
                                    let mut i: libc::c_int = 0;
                                    i = 0 as libc::c_int;
                                    while (i as libc::c_ulong) < ws.ws_wordc {
                                        fmtbuf[i as usize] = *(ws.ws_wordv).offset(i as isize);
                                        i += 1;
                                        i;
                                    }
                                    while i < 3 as libc::c_int {
                                        fmtbuf[i as usize] = 0 as *const libc::c_char;
                                        i += 1;
                                        i;
                                    }
                                    fmt = fmtbuf.as_mut_ptr();
                                }
                            }
                            len = (len as libc::c_ulong)
                                .wrapping_add(
                                    format_total_stats(fp, fmt, ',' as i32, 0 as libc::c_int)
                                        as libc::c_ulong,
                                ) as size_t as size_t;
                            if !arg.is_null() {
                                wordsplit_free(&mut ws);
                            }
                        }
                        116 => {
                            let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
                            let mut tm: *mut tm = 0 as *mut tm;
                            let mut fmt_0: *const libc::c_char = if !arg.is_null() {
                                arg as *const libc::c_char
                            } else {
                                b"%c\0" as *const u8 as *const libc::c_char
                            };
                            gettimeofday(&mut tv, 0 as *mut libc::c_void);
                            tm = localtime(&mut tv.tv_sec);
                            len = (len as libc::c_ulong)
                                .wrapping_add(
                                    fprintftime(
                                        fp,
                                        fmt_0,
                                        tm,
                                        0 as timezone_t,
                                        (tv.tv_usec * 1000 as libc::c_int as libc::c_long)
                                            as libc::c_int,
                                    ),
                                ) as size_t as size_t;
                        }
                        42 => {
                            let mut w: libc::c_long = if !arg.is_null() {
                                strtol(arg, 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                            } else {
                                getwidth(fp)
                            };
                            while w as libc::c_ulong > len {
                                fputc_unlocked(' ' as i32, fp);
                                len = len.wrapping_add(1);
                                len;
                            }
                        }
                        _ => {
                            fputc_unlocked('%' as i32, fp);
                            fputc_unlocked(*ip as libc::c_int, fp);
                            len = (len as libc::c_ulong)
                                .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                        }
                    }
                    arg = 0 as *mut libc::c_char;
                }
            }
        } else {
            fputc_unlocked(*ip as libc::c_int, fp);
            if *ip as libc::c_int == '\r' as i32 {
                len = 0 as libc::c_int as size_t;
                tty_cleanup = 1 as libc::c_int;
            } else {
                len = len.wrapping_add(1);
                len;
            }
        }
        ip = ip.offset(1);
        ip;
    }
    fflush_unlocked(fp);
    return len as libc::c_int;
}
static mut tty: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn run_checkpoint_actions(mut do_write: bool) {
    let mut p: *mut checkpoint_action = 0 as *mut checkpoint_action;
    p = checkpoint_action;
    while !p.is_null() {
        match (*p).opcode as libc::c_uint {
            0 => {
                fputc_unlocked('.' as i32, stdlis);
                fflush_unlocked(stdlis);
            }
            1 => {
                if tty.is_null() {
                    tty = fopen(
                        b"/dev/tty\0" as *const u8 as *const libc::c_char,
                        b"w\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !tty.is_null() {
                    fputc_unlocked('\u{7}' as i32, tty);
                    fflush_unlocked(tty);
                }
            }
            2 => {
                let mut n: libc::c_int = fprintf(
                    stderr,
                    b"%s: \0" as *const u8 as *const libc::c_char,
                    program_name,
                );
                format_checkpoint_string(
                    stderr,
                    n as size_t,
                    (*p).v.command,
                    do_write,
                    checkpoint,
                );
                fputc_unlocked('\n' as i32, stderr);
            }
            3 => {
                if tty.is_null() {
                    tty = fopen(
                        b"/dev/tty\0" as *const u8 as *const libc::c_char,
                        b"w\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !tty.is_null() {
                    format_checkpoint_string(
                        tty,
                        0 as libc::c_int as size_t,
                        (*p).v.command,
                        do_write,
                        checkpoint,
                    );
                }
            }
            4 => {
                sleep((*p).v.time as libc::c_uint);
            }
            5 => {
                sys_exec_checkpoint_script(
                    (*p).v.command,
                    *archive_name_cursor.offset(0 as libc::c_int as isize),
                    checkpoint as libc::c_int,
                );
            }
            6 => {
                compute_duration();
                print_total_stats();
            }
            7 => {
                let mut n_0: libc::c_int = 0;
                sigwait(&mut sigs, &mut n_0);
            }
            _ => {}
        }
        p = (*p).next;
    }
}
pub unsafe extern "C" fn checkpoint_flush_actions() {
    let mut p: *mut checkpoint_action = 0 as *mut checkpoint_action;
    p = checkpoint_action;
    while !p.is_null() {
        match (*p).opcode as libc::c_uint {
            3 => {
                if !tty.is_null() && tty_cleanup != 0 {
                    let mut w: libc::c_long = getwidth(tty);
                    loop {
                        let fresh1 = w;
                        w = w - 1;
                        if !(fresh1 != 0) {
                            break;
                        }
                        fputc_unlocked(' ' as i32, tty);
                    }
                    fputc_unlocked('\r' as i32, tty);
                    fflush_unlocked(tty);
                }
            }
            _ => {}
        }
        p = (*p).next;
    }
}
pub unsafe extern "C" fn checkpoint_run(mut do_write: bool) {
    if checkpoint_option != 0
        && {
            checkpoint = checkpoint.wrapping_add(1);
            checkpoint.wrapping_rem(checkpoint_option) == 0
        }
    {
        run_checkpoint_actions(do_write);
    }
}
pub unsafe extern "C" fn checkpoint_finish() {
    if checkpoint_option != 0 {
        checkpoint_flush_actions();
        if !tty.is_null() {
            fclose(tty);
        }
    }
}
