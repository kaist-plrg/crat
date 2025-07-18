use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type strm_queue;
    pub type node_error;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strm_cfunc_value(_: strm_cfunc) -> strm_value;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_ptr_value(_: *mut libc::c_void) -> strm_value;
    fn strm_stream_new(
        mode: strm_stream_mode,
        start: strm_callback,
        close: strm_callback,
        data: *mut libc::c_void,
    ) -> *mut strm_stream;
    fn strm_stream_close(strm: *mut strm_stream);
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_parse_args(
        _: *mut strm_stream,
        _: libc::c_int,
        _: *mut strm_value,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strm_var_def(
        _: *mut strm_state,
        _: *const libc::c_char,
        _: strm_value,
    ) -> libc::c_int;
    fn strm_signal(
        sig: libc::c_int,
        func: strm_sighandler_t,
        arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn strm_unsignal(sig: libc::c_int, func: strm_sighandler_t) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type strm_value = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_stream {
    pub type_0: strm_ptr_type,
    pub flags: libc::c_uint,
    pub mode: strm_stream_mode,
    pub start_func: strm_callback,
    pub close_func: strm_callback,
    pub data: *mut libc::c_void,
    pub dst: *mut strm_stream,
    pub rest: *mut *mut strm_stream,
    pub rsize: size_t,
    pub rcapa: size_t,
    pub exc: *mut node_error,
    pub refcnt: strm_int,
    pub queue: *mut strm_queue,
    pub excl: strm_int,
}
pub type strm_int = int32_t;
pub type strm_callback = Option::<
    unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
>;
pub type strm_stream_mode = libc::c_uint;
pub const strm_killed: strm_stream_mode = 4;
pub const strm_dying: strm_stream_mode = 3;
pub const strm_consumer: strm_stream_mode = 2;
pub const strm_filter: strm_stream_mode = 1;
pub const strm_producer: strm_stream_mode = 0;
pub type strm_ptr_type = libc::c_uint;
pub const STRM_PTR_AUX: strm_ptr_type = 4;
pub const STRM_PTR_IO: strm_ptr_type = 3;
pub const STRM_PTR_GENFUNC: strm_ptr_type = 2;
pub const STRM_PTR_LAMBDA: strm_ptr_type = 1;
pub const STRM_PTR_STREAM: strm_ptr_type = 0;
pub type strm_cfunc = Option::<
    unsafe extern "C" fn(
        *mut strm_stream,
        libc::c_int,
        *mut strm_value,
        *mut strm_value,
    ) -> libc::c_int,
>;
pub type strm_sighandler_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> (),
>;
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
pub struct bar_data {
    pub title: *const libc::c_char,
    pub tlen: strm_int,
    pub col: strm_int,
    pub row: strm_int,
    pub dlen: strm_int,
    pub llen: strm_int,
    pub offset: strm_int,
    pub max: strm_int,
    pub data: *mut libc::c_double,
}
static mut refcnt: libc::c_int = 0 as libc::c_int;
static mut winch: libc::c_int = 0 as libc::c_int;
static mut interrupt: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn sigupdate(mut sig: libc::c_int, mut arg: *mut libc::c_void) {
    let mut var: *mut libc::c_int = arg as *mut libc::c_int;
    *var = 1 as libc::c_int;
}
unsafe extern "C" fn get_winsize(
    mut row: *mut libc::c_int,
    mut col: *mut libc::c_int,
) -> libc::c_int {
    let mut w: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut n: libc::c_int = 0;
    n = ioctl(
        1 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut w as *mut winsize,
    );
    if n < 0 as libc::c_int || w.ws_col as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    *row = w.ws_row as libc::c_int;
    *col = w.ws_col as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn move_cursor(mut row: libc::c_int, mut col: libc::c_int) {
    printf(b"\x1B[%d;%dH\0" as *const u8 as *const libc::c_char, row, col);
}
unsafe extern "C" fn clear() {
    printf(b"\x1B[2J\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn erase_cursor() {
    printf(b"\x1B[?25l\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn show_cursor() {
    printf(b"\x1B[?25h\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn show_title(mut d: *mut bar_data) {
    let mut start: libc::c_int = 0;
    erase_cursor();
    clear();
    if (*d).tlen == 0 as libc::c_int {
        return;
    }
    start = ((*d).col - (*d).tlen) / 2 as libc::c_int;
    move_cursor(1 as libc::c_int, start);
    fwrite(
        (*d).title as *const libc::c_void,
        (*d).tlen as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stdout,
    );
}
unsafe extern "C" fn show_yaxis(mut d: *mut bar_data) {
    move_cursor(1 as libc::c_int, 2 as libc::c_int);
    printf(b"\x1B[0m\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*d).llen {
        move_cursor(i + 2 as libc::c_int, (*d).dlen + 1 as libc::c_int);
        if i == 0 as libc::c_int {
            printf(
                b"\xE2\x94\x9C %d   \0" as *const u8 as *const libc::c_char,
                (*d).max,
            );
        } else if i == (*d).llen - 1 as libc::c_int {
            printf(b"\xE2\x94\x9C 0\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"|\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn show_bar(
    mut d: *mut bar_data,
    mut i: libc::c_int,
    mut n: libc::c_int,
) {
    let mut f: libc::c_double = *((*d).data).offset(i as isize)
        / (*d).max as libc::c_double * (*d).llen as libc::c_double;
    let mut line: libc::c_int = 0 as libc::c_int;
    while line < (*d).llen {
        move_cursor((*d).llen + 1 as libc::c_int - line, n);
        if (line as libc::c_double) < f {
            printf(b"\x1B[7m \0" as *const u8 as *const libc::c_char);
        } else if line == 0 as libc::c_int {
            printf(b"\x1B[0m_\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"\x1B[0m \0" as *const u8 as *const libc::c_char);
        }
        line += 1;
        line;
    }
}
unsafe extern "C" fn show_graph(mut d: *mut bar_data) {
    let mut n: libc::c_int = 1 as libc::c_int;
    show_yaxis(d);
    let mut i: libc::c_int = (*d).offset;
    while i < (*d).dlen {
        let fresh0 = n;
        n = n + 1;
        show_bar(d, i, fresh0);
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*d).offset {
        let fresh1 = n;
        n = n + 1;
        show_bar(d, i_0, fresh1);
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn init_bar(mut d: *mut bar_data) -> libc::c_int {
    if get_winsize(&mut (*d).row, &mut (*d).col) != 0 {
        return 1 as libc::c_int;
    }
    (*d).max = 1 as libc::c_int;
    (*d).offset = 0 as libc::c_int;
    (*d).dlen = (*d).col - 6 as libc::c_int;
    (*d).llen = (*d).row - 3 as libc::c_int;
    (*d)
        .data = malloc(
        ((*d).dlen as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*d).dlen {
        *((*d).data).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i += 1;
        i;
    }
    show_title(d);
    return 0 as libc::c_int;
}
unsafe extern "C" fn iter_bar(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut bar_data = (*strm).data as *mut bar_data;
    let mut f: libc::c_double = 0.;
    let mut max: libc::c_double = 1.0f64;
    if interrupt != 0 {
        interrupt = 0 as libc::c_int;
        strm_unsignal(
            2 as libc::c_int,
            Some(sigupdate as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()),
        );
        move_cursor((*d).row - 1 as libc::c_int, 1 as libc::c_int);
        show_cursor();
        exit(1 as libc::c_int);
    }
    if strm_number_p(data) == 0 {
        strm_raise(strm, b"invalid data\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    if winch != 0 {
        winch = 0 as libc::c_int;
        free((*d).data as *mut libc::c_void);
        if init_bar(d) == 1 as libc::c_int {
            strm_stream_close(strm);
            return 1 as libc::c_int;
        }
    }
    f = strm_value_float(data);
    if f < 0 as libc::c_int as libc::c_double {
        f = 0 as libc::c_int as libc::c_double;
    }
    let fresh2 = (*d).offset;
    (*d).offset = (*d).offset + 1;
    *((*d).data).offset(fresh2 as isize) = f;
    max = 1.0f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*d).dlen {
        f = *((*d).data).offset(i as isize);
        if f > max {
            max = f;
        }
        i += 1;
        i;
    }
    (*d).max = max as strm_int;
    if (*d).offset == (*d).dlen {
        (*d).offset = 0 as libc::c_int;
    }
    show_graph(d);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fin_bar(
    mut strm: *mut strm_stream,
    mut data: strm_value,
) -> libc::c_int {
    let mut d: *mut bar_data = (*strm).data as *mut bar_data;
    move_cursor((*d).row, 1 as libc::c_int);
    if !((*d).title).is_null() {
        free((*d).title as *mut libc::c_void);
    }
    free((*d).data as *mut libc::c_void);
    free(d as *mut libc::c_void);
    show_cursor();
    ::std::intrinsics::atomic_xadd_seqcst(&mut refcnt, 1 as libc::c_int);
    if refcnt <= 0 as libc::c_int {
        strm_unsignal(
            28 as libc::c_int,
            Some(sigupdate as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()),
        );
        strm_unsignal(
            2 as libc::c_int,
            Some(sigupdate as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> ()),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn exec_bgraph(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut args: *mut strm_value,
    mut ret: *mut strm_value,
) -> libc::c_int {
    let mut d: *mut bar_data = 0 as *mut bar_data;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tlen: strm_int = 0 as libc::c_int;
    if strm_parse_args(
        strm,
        argc,
        args,
        b"|s\0" as *const u8 as *const libc::c_char,
        &mut title as *mut *mut libc::c_char,
        &mut tlen as *mut strm_int,
    ) == 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    d = malloc(::std::mem::size_of::<bar_data>() as libc::c_ulong) as *mut bar_data;
    if d.is_null() {
        return 1 as libc::c_int;
    }
    (*d).title = malloc(tlen as libc::c_ulong) as *const libc::c_char;
    memcpy(
        (*d).title as *mut libc::c_void,
        title as *const libc::c_void,
        tlen as libc::c_ulong,
    );
    (*d).tlen = tlen;
    if refcnt == 0 as libc::c_int {
        ::std::intrinsics::atomic_xadd_seqcst(&mut refcnt, 1 as libc::c_int);
        strm_signal(
            28 as libc::c_int,
            Some(
                sigupdate as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> (),
            ),
            &mut winch as *mut libc::c_int as *mut libc::c_void,
        );
        strm_signal(
            2 as libc::c_int,
            Some(
                sigupdate as unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> (),
            ),
            &mut interrupt as *mut libc::c_int as *mut libc::c_void,
        );
    }
    if init_bar(d) == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    *ret = strm_ptr_value(
        strm_stream_new(
            strm_consumer,
            Some(
                iter_bar
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            Some(
                fin_bar
                    as unsafe extern "C" fn(*mut strm_stream, strm_value) -> libc::c_int,
            ),
            d as *mut libc::c_void,
        ) as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn strm_graph_init(mut state: *mut strm_state) {
    strm_var_def(
        state,
        b"graph_bar\0" as *const u8 as *const libc::c_char,
        strm_cfunc_value(
            Some(
                exec_bgraph
                    as unsafe extern "C" fn(
                        *mut strm_stream,
                        libc::c_int,
                        *mut strm_value,
                        *mut strm_value,
                    ) -> libc::c_int,
            ),
        ),
    );
}
