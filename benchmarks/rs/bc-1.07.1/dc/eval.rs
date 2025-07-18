use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bc_struct;
    pub type dc_string;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut progname: *const libc::c_char;
    fn dc_str2charp(_: dc_str) -> *const libc::c_char;
    fn dc_system(_: *const libc::c_char) -> *const libc::c_char;
    fn dc_array_set(_: libc::c_int, _: libc::c_int, _: dc_data);
    fn dc_binop(
        _: Option::<
            unsafe extern "C" fn(dc_num, dc_num, libc::c_int, *mut dc_num) -> libc::c_int,
        >,
        _: libc::c_int,
    );
    fn dc_binop2(
        _: Option::<
            unsafe extern "C" fn(
                dc_num,
                dc_num,
                libc::c_int,
                *mut dc_num,
                *mut dc_num,
            ) -> libc::c_int,
        >,
        _: libc::c_int,
    );
    fn dc_triop(
        _: Option::<
            unsafe extern "C" fn(
                dc_num,
                dc_num,
                dc_num,
                libc::c_int,
                *mut dc_num,
            ) -> libc::c_int,
        >,
        _: libc::c_int,
    );
    fn dc_clear_stack();
    fn dc_dump_num(_: dc_num, _: dc_discard);
    fn dc_free_num(_: *mut dc_num);
    fn dc_free_str(_: *mut dc_str);
    fn dc_garbage(_: *const libc::c_char, _: libc::c_int);
    fn dc_out_str(_: dc_str, _: dc_discard);
    fn dc_print(_: dc_data, _: libc::c_int, _: dc_newline, _: dc_discard);
    fn dc_printall(_: libc::c_int);
    fn dc_push(_: dc_data);
    fn dc_register_push(_: libc::c_int, _: dc_data);
    fn dc_register_set(_: libc::c_int, _: dc_data);
    fn dc_show_id(_: *mut FILE, _: libc::c_int, _: *const libc::c_char);
    fn dc_cmpop() -> libc::c_int;
    fn dc_num2int(_: dc_num, _: dc_discard) -> libc::c_int;
    fn dc_pop(_: *mut dc_data) -> libc::c_int;
    fn dc_register_get(_: libc::c_int, _: *mut dc_data) -> libc::c_int;
    fn dc_register_pop(_: libc::c_int, _: *mut dc_data) -> libc::c_int;
    fn dc_stack_rotate(_: libc::c_int);
    fn dc_tell_length(_: dc_data, _: dc_discard) -> libc::c_int;
    fn dc_tell_scale(_: dc_num, _: dc_discard) -> libc::c_int;
    fn dc_tell_stackdepth() -> libc::c_int;
    fn dc_top_of_stack(_: *mut dc_data) -> libc::c_int;
    fn dc_strlen(_: dc_str) -> size_t;
    fn dc_array_get(_: libc::c_int, _: libc::c_int) -> dc_data;
    fn dc_dup(_: dc_data) -> dc_data;
    fn dc_getnum(
        _: Option::<unsafe extern "C" fn() -> libc::c_int>,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> dc_data;
    fn dc_int2data(_: libc::c_int) -> dc_data;
    fn dc_makestring(_: *const libc::c_char, _: size_t) -> dc_data;
    fn dc_readstring(_: *mut FILE, _: libc::c_int, _: libc::c_int) -> dc_data;
    fn dc_add(_: dc_num, _: dc_num, _: libc::c_int, _: *mut dc_num) -> libc::c_int;
    fn dc_div(_: dc_num, _: dc_num, _: libc::c_int, _: *mut dc_num) -> libc::c_int;
    fn dc_divrem(
        _: dc_num,
        _: dc_num,
        _: libc::c_int,
        _: *mut dc_num,
        _: *mut dc_num,
    ) -> libc::c_int;
    fn dc_exp(_: dc_num, _: dc_num, _: libc::c_int, _: *mut dc_num) -> libc::c_int;
    fn dc_modexp(
        _: dc_num,
        _: dc_num,
        _: dc_num,
        _: libc::c_int,
        _: *mut dc_num,
    ) -> libc::c_int;
    fn dc_mul(_: dc_num, _: dc_num, _: libc::c_int, _: *mut dc_num) -> libc::c_int;
    fn dc_rem(_: dc_num, _: dc_num, _: libc::c_int, _: *mut dc_num) -> libc::c_int;
    fn dc_sub(_: dc_num, _: dc_num, _: libc::c_int, _: *mut dc_num) -> libc::c_int;
    fn dc_sqrt(_: dc_num, _: libc::c_int, _: *mut dc_num) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type dc_discard = libc::c_uint;
pub const DC_KEEP: dc_discard = 1;
pub const DC_TOSS: dc_discard = 0;
pub type dc_newline = libc::c_uint;
pub const DC_WITHNL: dc_newline = 1;
pub const DC_NONL: dc_newline = 0;
pub type dc_value_type = libc::c_uint;
pub const DC_STRING: dc_value_type = 2;
pub const DC_NUMBER: dc_value_type = 1;
pub const DC_UNINITIALIZED: dc_value_type = 0;
pub type dc_num = *mut bc_struct;
pub type dc_str = *mut dc_string;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dc_data {
    pub dc_type: dc_value_type,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub number: dc_num,
    pub string: dc_str,
}
pub type handler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub const DC_EOF_ERROR: dc_status = 10;
pub const DC_NEGCMP: dc_status = 9;
pub const DC_COMMENT: dc_status = 8;
pub const DC_SYSTEM: dc_status = 7;
pub const DC_STR: dc_status = 6;
pub const DC_INT: dc_status = 5;
pub const DC_TRUE: dc_boolean = 1;
pub type dc_boolean = libc::c_uint;
pub const DC_FALSE: dc_boolean = 0;
pub const DC_QUIT: dc_status = 4;
pub type dc_status = libc::c_uint;
pub const DC_EVALTOS: dc_status = 3;
pub const DC_EVALREG: dc_status = 2;
pub const DC_EATONE: dc_status = 1;
pub const DC_OKAY: dc_status = 0;
static mut dc_ibase: libc::c_int = 10 as libc::c_int;
static mut dc_obase: libc::c_int = 10 as libc::c_int;
static mut dc_scale: libc::c_int = 0 as libc::c_int;
static mut unwind_depth: libc::c_int = 0 as libc::c_int;
static mut interrupt_seen: sig_atomic_t = 0 as libc::c_int;
static mut unwind_noexit: dc_boolean = DC_FALSE;
static mut stdin_lookahead: libc::c_int = -(1 as libc::c_int);
static mut input_fil_fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut input_str_string: *const libc::c_char = 0 as *const libc::c_char;
static mut input_pushback: libc::c_int = 0;
unsafe extern "C" fn input_fil() -> libc::c_int {
    if input_pushback != -(1 as libc::c_int) {
        let mut c: libc::c_int = input_pushback;
        input_pushback = -(1 as libc::c_int);
        return c;
    }
    return getc(input_fil_fp);
}
unsafe extern "C" fn input_str() -> libc::c_int {
    if *input_str_string as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    let fresh0 = input_str_string;
    input_str_string = input_str_string.offset(1);
    return *(fresh0 as *const libc::c_uchar) as libc::c_int;
}
unsafe extern "C" fn dc_eval_and_free_str(mut string: *mut dc_data) -> libc::c_int {
    let mut status: dc_status = DC_OKAY;
    status = evalstr(string) as dc_status;
    if (*string).dc_type as libc::c_uint == DC_STRING as libc::c_int as libc::c_uint {
        dc_free_str(&mut (*string).v.string);
    }
    return status as libc::c_int;
}
unsafe extern "C" fn dc_trap_interrupt(mut signo: libc::c_int) {
    signal(signo, Some(dc_trap_interrupt as unsafe extern "C" fn(libc::c_int) -> ()));
    ::std::ptr::write_volatile(
        &mut interrupt_seen as *mut sig_atomic_t,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn skip_past_eol(
    mut strptr: *const libc::c_char,
    mut strend: *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = memchr(
        strptr as *const libc::c_void,
        '\n' as i32,
        strend.offset_from(strptr) as libc::c_long as size_t,
    ) as *const libc::c_char;
    if !p.is_null() {
        return p.offset(1 as libc::c_int as isize);
    }
    return strend;
}
unsafe extern "C" fn dc_func(
    mut c: libc::c_int,
    mut peekc: libc::c_int,
    mut negcmp: libc::c_int,
) -> dc_status {
    let mut datum: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    let mut tmpint: libc::c_int = 0;
    match c {
        95 | 46 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 65 | 66 | 67 | 68
        | 69 | 70 => return DC_INT,
        32 | 9 | 10 => {}
        43 => {
            dc_binop(
                Some(
                    dc_add
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        45 => {
            dc_binop(
                Some(
                    dc_sub
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        42 => {
            dc_binop(
                Some(
                    dc_mul
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        47 => {
            dc_binop(
                Some(
                    dc_div
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        37 => {
            dc_binop(
                Some(
                    dc_rem
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        126 => {
            dc_binop2(
                Some(
                    dc_divrem
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        124 => {
            dc_triop(
                Some(
                    dc_modexp
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        94 => {
            dc_binop(
                Some(
                    dc_exp
                        as unsafe extern "C" fn(
                            dc_num,
                            dc_num,
                            libc::c_int,
                            *mut dc_num,
                        ) -> libc::c_int,
                ),
                dc_scale,
            );
        }
        60 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if (dc_cmpop() < 0 as libc::c_int) as libc::c_int
                == (negcmp == 0 as libc::c_int) as libc::c_int
            {
                return DC_EVALREG;
            }
            return DC_EATONE;
        }
        61 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if (dc_cmpop() == 0 as libc::c_int) as libc::c_int
                == (negcmp == 0 as libc::c_int) as libc::c_int
            {
                return DC_EVALREG;
            }
            return DC_EATONE;
        }
        62 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if (dc_cmpop() > 0 as libc::c_int) as libc::c_int
                == (negcmp == 0 as libc::c_int) as libc::c_int
            {
                return DC_EVALREG;
            }
            return DC_EATONE;
        }
        63 => {
            if stdin_lookahead != -(1 as libc::c_int) {
                ungetc(stdin_lookahead, stdin);
                stdin_lookahead = -(1 as libc::c_int);
            }
            datum = dc_readstring(stdin, '\n' as i32, '\n' as i32);
            if ferror(stdin) != 0 {
                return DC_EOF_ERROR;
            }
            dc_push(datum);
            return DC_EVALTOS;
        }
        91 => return DC_STR,
        33 => {
            if peekc == '<' as i32 || peekc == '=' as i32 || peekc == '>' as i32 {
                return DC_NEGCMP;
            }
            return DC_SYSTEM;
        }
        35 => return DC_COMMENT,
        97 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                let mut tmps: libc::c_char = 0;
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmps = dc_num2int(datum.v.number, DC_TOSS) as libc::c_char;
                } else if datum.dc_type as libc::c_uint
                    == DC_STRING as libc::c_int as libc::c_uint
                {
                    tmps = *dc_str2charp(datum.v.string);
                    dc_free_str(&mut datum.v.string);
                } else {
                    dc_garbage(
                        b"at top of stack\0" as *const u8 as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                }
                dc_push(dc_makestring(&mut tmps, 1 as libc::c_int as size_t));
            }
        }
        99 => {
            dc_clear_stack();
        }
        100 => {
            if dc_top_of_stack(&mut datum) == 0 as libc::c_int {
                dc_push(dc_dup(datum));
            }
        }
        102 => {
            dc_printall(dc_obase);
        }
        105 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                tmpint = 0 as libc::c_int;
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmpint = dc_num2int(datum.v.number, DC_TOSS);
                }
                if 2 as libc::c_int <= tmpint && tmpint <= 16 as libc::c_int {
                    dc_ibase = tmpint;
                } else {
                    fprintf(
                        stderr,
                        b"%s: input base must be a number between 2 and %d (inclusive)\n\0"
                            as *const u8 as *const libc::c_char,
                        progname,
                        16 as libc::c_int,
                    );
                }
            }
        }
        107 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                tmpint = -(1 as libc::c_int);
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmpint = dc_num2int(datum.v.number, DC_TOSS);
                }
                if !(tmpint >= 0 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"%s: scale must be a nonnegative number\n\0" as *const u8
                            as *const libc::c_char,
                        progname,
                    );
                } else {
                    dc_scale = tmpint;
                }
            }
        }
        108 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if dc_register_get(peekc, &mut datum) == 0 as libc::c_int {
                dc_push(datum);
            }
            return DC_EATONE;
        }
        110 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                dc_print(datum, dc_obase, DC_NONL, DC_TOSS);
            }
        }
        111 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                tmpint = 0 as libc::c_int;
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmpint = dc_num2int(datum.v.number, DC_TOSS);
                }
                if !(tmpint > 1 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"%s: output base must be a number greater than 1\n\0"
                            as *const u8 as *const libc::c_char,
                        progname,
                    );
                } else {
                    dc_obase = tmpint;
                }
            }
        }
        112 => {
            if dc_top_of_stack(&mut datum) == 0 as libc::c_int {
                dc_print(datum, dc_obase, DC_WITHNL, DC_KEEP);
            }
        }
        113 => {
            unwind_depth = 1 as libc::c_int;
            unwind_noexit = DC_FALSE;
            return DC_QUIT;
        }
        114 => {
            dc_stack_rotate(2 as libc::c_int);
        }
        115 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if dc_pop(&mut datum) == 0 as libc::c_int {
                dc_register_set(peekc, datum);
            }
            return DC_EATONE;
        }
        118 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                let mut tmpnum: dc_num = 0 as *mut bc_struct;
                if datum.dc_type as libc::c_uint
                    != DC_NUMBER as libc::c_int as libc::c_uint
                {
                    fprintf(
                        stderr,
                        b"%s: square root of nonnumeric attempted\n\0" as *const u8
                            as *const libc::c_char,
                        progname,
                    );
                } else if dc_sqrt(datum.v.number, dc_scale, &mut tmpnum)
                    == 0 as libc::c_int
                {
                    dc_free_num(&mut datum.v.number);
                    datum.v.number = tmpnum;
                    dc_push(datum);
                }
            }
        }
        120 => return DC_EVALTOS,
        122 => {
            dc_push(dc_int2data(dc_tell_stackdepth()));
        }
        73 => {
            dc_push(dc_int2data(dc_ibase));
        }
        75 => {
            dc_push(dc_int2data(dc_scale));
        }
        76 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if dc_register_pop(peekc, &mut datum) == 0 as libc::c_int {
                dc_push(datum);
            }
            return DC_EATONE;
        }
        79 => {
            dc_push(dc_int2data(dc_obase));
        }
        80 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    dc_dump_num(datum.v.number, DC_TOSS);
                } else if datum.dc_type as libc::c_uint
                    == DC_STRING as libc::c_int as libc::c_uint
                {
                    dc_out_str(datum.v.string, DC_TOSS);
                } else {
                    dc_garbage(
                        b"at top of stack\0" as *const u8 as *const libc::c_char,
                        -(1 as libc::c_int),
                    );
                }
            }
            fflush(stdout);
        }
        81 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                unwind_depth = 0 as libc::c_int;
                unwind_noexit = DC_TRUE;
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    unwind_depth = dc_num2int(datum.v.number, DC_TOSS);
                }
                let fresh1 = unwind_depth;
                unwind_depth = unwind_depth - 1;
                if fresh1 > 0 as libc::c_int {
                    return DC_QUIT;
                }
                unwind_depth = 0 as libc::c_int;
                fprintf(
                    stderr,
                    b"%s: Q command requires a number >= 1\n\0" as *const u8
                        as *const libc::c_char,
                    progname,
                );
            }
        }
        82 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                tmpint = 0 as libc::c_int;
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmpint = dc_num2int(datum.v.number, DC_TOSS);
                }
                dc_stack_rotate(tmpint);
            }
        }
        83 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if dc_pop(&mut datum) == 0 as libc::c_int {
                dc_register_push(peekc, datum);
            }
            return DC_EATONE;
        }
        88 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                tmpint = 0 as libc::c_int;
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmpint = dc_tell_scale(datum.v.number, DC_TOSS);
                }
                dc_push(dc_int2data(tmpint));
            }
        }
        90 => {
            if dc_pop(&mut datum) == 0 as libc::c_int {
                dc_push(dc_int2data(dc_tell_length(datum, DC_TOSS)));
            }
        }
        58 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if dc_pop(&mut datum) == 0 as libc::c_int {
                tmpint = -(1 as libc::c_int);
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmpint = dc_num2int(datum.v.number, DC_TOSS);
                }
                if dc_pop(&mut datum) == 0 as libc::c_int {
                    if tmpint < 0 as libc::c_int {
                        fprintf(
                            stderr,
                            b"%s: array index must be a nonnegative integer\n\0"
                                as *const u8 as *const libc::c_char,
                            progname,
                        );
                    } else {
                        dc_array_set(peekc, tmpint, datum);
                    }
                }
            }
            return DC_EATONE;
        }
        59 => {
            if peekc == -(1 as libc::c_int) {
                return DC_EOF_ERROR;
            }
            if dc_pop(&mut datum) == 0 as libc::c_int {
                tmpint = -(1 as libc::c_int);
                if datum.dc_type as libc::c_uint
                    == DC_NUMBER as libc::c_int as libc::c_uint
                {
                    tmpint = dc_num2int(datum.v.number, DC_TOSS);
                }
                if tmpint < 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: array index must be a nonnegative integer\n\0" as *const u8
                            as *const libc::c_char,
                        progname,
                    );
                } else {
                    dc_push(dc_array_get(peekc, tmpint));
                }
            }
            return DC_EATONE;
        }
        _ => {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, progname);
            dc_show_id(
                stdout,
                c,
                b" unimplemented\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return DC_OKAY;
}
unsafe extern "C" fn evalstr(mut string: *mut dc_data) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut c: libc::c_int = 0;
    let mut peekc: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut negcmp: libc::c_int = 0;
    let mut next_negcmp: libc::c_int = 0 as libc::c_int;
    let mut tail_depth: libc::c_int = 1 as libc::c_int;
    let mut evalstr_0: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    if (*string).dc_type as libc::c_uint != DC_STRING as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"%s: eval called with non-string argument\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        return DC_OKAY as libc::c_int;
    }
    ::std::ptr::write_volatile(
        &mut interrupt_seen as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    s = dc_str2charp((*string).v.string);
    end = s.offset(dc_strlen((*string).v.string) as isize);
    while s < end && interrupt_seen == 0 as libc::c_int {
        let fresh2 = s;
        s = s.offset(1);
        c = *(fresh2 as *const libc::c_uchar) as libc::c_int;
        peekc = -(1 as libc::c_int);
        if s < end {
            peekc = *(s as *const libc::c_uchar) as libc::c_int;
        }
        negcmp = next_negcmp;
        next_negcmp = 0 as libc::c_int;
        let mut current_block_72: u64;
        match dc_func(c, peekc, negcmp) as libc::c_uint {
            1 => {
                if peekc != -(1 as libc::c_int) {
                    s = s.offset(1);
                    s;
                }
                current_block_72 = 15970011996474399071;
            }
            2 => {
                s = s.offset(1);
                s;
                if dc_register_get(peekc, &mut evalstr_0) != 0 as libc::c_int {
                    current_block_72 = 15970011996474399071;
                } else {
                    dc_push(evalstr_0);
                    current_block_72 = 2609838824379316571;
                }
            }
            3 => {
                current_block_72 = 2609838824379316571;
            }
            4 => {
                if unwind_depth >= tail_depth {
                    unwind_depth -= tail_depth;
                    return DC_QUIT as libc::c_int;
                }
                tail_depth -= unwind_depth;
                current_block_72 = 15970011996474399071;
            }
            5 => {
                input_str_string = s.offset(-(1 as libc::c_int as isize));
                dc_push(
                    dc_getnum(
                        Some(input_str as unsafe extern "C" fn() -> libc::c_int),
                        dc_ibase,
                        &mut peekc,
                    ),
                );
                s = input_str_string;
                if peekc != -(1 as libc::c_int) {
                    s = s.offset(-1);
                    s;
                }
                current_block_72 = 15970011996474399071;
            }
            6 => {
                count = 1 as libc::c_int;
                p = s;
                while p < end && count > 0 as libc::c_int {
                    if *p as libc::c_int == ']' as i32 {
                        count -= 1;
                        count;
                    } else if *p as libc::c_int == '[' as i32 {
                        count += 1;
                        count;
                    }
                    p = p.offset(1);
                    p;
                }
                len = p.offset_from(s) as libc::c_long as size_t;
                dc_push(
                    dc_makestring(
                        s,
                        if count == 0 as libc::c_int {
                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        } else {
                            len
                        },
                    ),
                );
                s = p;
                current_block_72 = 15970011996474399071;
            }
            7 => {
                s = dc_system(s);
                current_block_72 = 15970011996474399071;
            }
            8 => {
                s = skip_past_eol(s, end);
                current_block_72 = 15970011996474399071;
            }
            9 => {
                next_negcmp = 1 as libc::c_int;
                current_block_72 = 15970011996474399071;
            }
            10 => {
                if ferror(stdin) != 0 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        progname,
                    );
                    perror(b"error reading stdin\0" as *const u8 as *const libc::c_char);
                    return 2 as libc::c_int;
                }
                fprintf(
                    stderr,
                    b"%s: unexpected EOS\n\0" as *const u8 as *const libc::c_char,
                    progname,
                );
                return DC_OKAY as libc::c_int;
            }
            0 | _ => {
                current_block_72 = 15970011996474399071;
            }
        }
        match current_block_72 {
            2609838824379316571 => {
                while s < end
                    && (*s as libc::c_int == ' ' as i32
                        || *s as libc::c_int == '\t' as i32
                        || *s as libc::c_int == '\n' as i32
                        || *s as libc::c_int == '#' as i32)
                {
                    let fresh3 = s;
                    s = s.offset(1);
                    if *fresh3 as libc::c_int == '#' as i32 {
                        s = skip_past_eol(s, end);
                    }
                }
                if dc_pop(&mut evalstr_0) == 0 as libc::c_int {
                    if evalstr_0.dc_type as libc::c_uint
                        == DC_NUMBER as libc::c_int as libc::c_uint
                    {
                        dc_push(evalstr_0);
                    } else if evalstr_0.dc_type as libc::c_uint
                        != DC_STRING as libc::c_int as libc::c_uint
                    {
                        dc_garbage(
                            b"at top of stack\0" as *const u8 as *const libc::c_char,
                            -(1 as libc::c_int),
                        );
                    } else if s == end {
                        dc_free_str(&mut (*string).v.string);
                        *string = evalstr_0;
                        s = dc_str2charp((*string).v.string);
                        end = s.offset(dc_strlen((*string).v.string) as isize);
                        tail_depth += 1;
                        tail_depth;
                    } else if dc_eval_and_free_str(&mut evalstr_0)
                        == DC_QUIT as libc::c_int
                    {
                        if unwind_depth > 0 as libc::c_int {
                            unwind_depth -= 1;
                            unwind_depth;
                            return DC_QUIT as libc::c_int;
                        }
                        return DC_OKAY as libc::c_int;
                    }
                }
            }
            _ => {}
        }
    }
    return DC_OKAY as libc::c_int;
}
pub unsafe extern "C" fn dc_evalstr(mut string: *mut dc_data) -> libc::c_int {
    match evalstr(string) {
        0 => return 0 as libc::c_int,
        4 => {
            if unwind_noexit as libc::c_uint != DC_TRUE as libc::c_int as libc::c_uint {
                return 2 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        _ => return 2 as libc::c_int,
    };
}
pub unsafe extern "C" fn dc_evalfile(mut fp: *mut FILE) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut peekc: libc::c_int = 0;
    let mut negcmp: libc::c_int = 0;
    let mut next_negcmp: libc::c_int = 0 as libc::c_int;
    let mut sigint_handler: handler_t = Some(
        dc_trap_interrupt as unsafe extern "C" fn(libc::c_int) -> (),
    );
    let mut sigint_default: handler_t = signal(
        2 as libc::c_int,
        ::std::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    let mut datum: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed {
            number: 0 as *mut bc_struct,
        },
    };
    signal(2 as libc::c_int, sigint_default);
    if isatty(fileno(fp)) == 0 {
        sigint_handler = sigint_default;
    }
    stdin_lookahead = -(1 as libc::c_int);
    c = getc(fp);
    loop {
        if !(c != -(1 as libc::c_int)) {
            current_block = 3546145585875536353;
            break;
        }
        peekc = getc(fp);
        if fp == stdin {
            stdin_lookahead = peekc;
        }
        negcmp = next_negcmp;
        next_negcmp = 0 as libc::c_int;
        signal(2 as libc::c_int, sigint_handler);
        match dc_func(c, peekc, negcmp) as libc::c_uint {
            0 => {
                if stdin_lookahead != peekc && fp == stdin {
                    peekc = getc(fp);
                }
                current_block = 2520131295878969859;
            }
            1 => {
                peekc = getc(fp);
                current_block = 2520131295878969859;
            }
            2 => {
                c = peekc;
                peekc = getc(fp);
                stdin_lookahead = peekc;
                if dc_register_get(c, &mut datum) != 0 as libc::c_int {
                    current_block = 2520131295878969859;
                } else {
                    dc_push(datum);
                    current_block = 5023088878038355716;
                }
            }
            3 => {
                current_block = 5023088878038355716;
            }
            4 => {
                if unwind_noexit as libc::c_uint
                    != DC_TRUE as libc::c_int as libc::c_uint
                {
                    current_block = 18279488546989494124;
                    break;
                }
                fprintf(
                    stderr,
                    b"%s: Q command argument exceeded string execution depth\n\0"
                        as *const u8 as *const libc::c_char,
                    progname,
                );
                if stdin_lookahead != peekc && fp == stdin {
                    peekc = getc(fp);
                }
                current_block = 2520131295878969859;
            }
            5 => {
                input_fil_fp = fp;
                input_pushback = c;
                ungetc(peekc, fp);
                dc_push(
                    dc_getnum(
                        Some(input_fil as unsafe extern "C" fn() -> libc::c_int),
                        dc_ibase,
                        &mut peekc,
                    ),
                );
                if ferror(fp) != 0 {
                    current_block = 3607553864772779902;
                    break;
                }
                current_block = 2520131295878969859;
            }
            6 => {
                ungetc(peekc, fp);
                datum = dc_readstring(fp, '[' as i32, ']' as i32);
                if ferror(fp) != 0 {
                    current_block = 3607553864772779902;
                    break;
                }
                dc_push(datum);
                peekc = getc(fp);
                current_block = 2520131295878969859;
            }
            7 => {
                ungetc(peekc, fp);
                datum = dc_readstring(fp, '\n' as i32, '\n' as i32);
                if ferror(fp) != 0 {
                    current_block = 3607553864772779902;
                    break;
                }
                dc_system(dc_str2charp(datum.v.string));
                dc_free_str(&mut datum.v.string);
                peekc = getc(fp);
                current_block = 2520131295878969859;
            }
            8 => {
                while peekc != -(1 as libc::c_int) && peekc != '\n' as i32 {
                    peekc = getc(fp);
                }
                if peekc != -(1 as libc::c_int) {
                    peekc = getc(fp);
                }
                current_block = 2520131295878969859;
            }
            9 => {
                next_negcmp = 1 as libc::c_int;
                current_block = 2520131295878969859;
            }
            10 => {
                if ferror(fp) != 0 {
                    current_block = 3607553864772779902;
                    break;
                }
                fprintf(
                    stderr,
                    b"%s: unexpected EOF\n\0" as *const u8 as *const libc::c_char,
                    progname,
                );
                current_block = 18279488546989494124;
                break;
            }
            _ => {
                current_block = 2520131295878969859;
            }
        }
        match current_block {
            5023088878038355716 => {
                if stdin_lookahead != peekc && fp == stdin {
                    peekc = getc(fp);
                }
                if dc_pop(&mut datum) == 0 as libc::c_int {
                    if datum.dc_type as libc::c_uint
                        == DC_NUMBER as libc::c_int as libc::c_uint
                    {
                        dc_push(datum);
                    } else if datum.dc_type as libc::c_uint
                        == DC_STRING as libc::c_int as libc::c_uint
                    {
                        if dc_eval_and_free_str(&mut datum) == DC_QUIT as libc::c_int {
                            if unwind_noexit as libc::c_uint
                                != DC_TRUE as libc::c_int as libc::c_uint
                            {
                                current_block = 18279488546989494124;
                                break;
                            }
                            fprintf(
                                stderr,
                                b"%s: Q command argument exceeded string execution depth\n\0"
                                    as *const u8 as *const libc::c_char,
                                progname,
                            );
                        }
                    } else {
                        dc_garbage(
                            b"at top of stack\0" as *const u8 as *const libc::c_char,
                            -(1 as libc::c_int),
                        );
                    }
                }
            }
            _ => {}
        }
        if interrupt_seen != 0 {
            fprintf(stderr, b"\nInterrupt!\n\0" as *const u8 as *const libc::c_char);
        }
        ::std::ptr::write_volatile(
            &mut interrupt_seen as *mut sig_atomic_t,
            0 as libc::c_int,
        );
        signal(2 as libc::c_int, sigint_default);
        c = peekc;
    }
    match current_block {
        3546145585875536353 => {
            if ferror(fp) == 0 {
                signal(2 as libc::c_int, sigint_default);
                return 0 as libc::c_int;
            }
        }
        18279488546989494124 => {
            signal(2 as libc::c_int, sigint_default);
            return 2 as libc::c_int;
        }
        _ => {}
    }
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, progname);
    perror(b"error reading input\0" as *const u8 as *const libc::c_char);
    return 2 as libc::c_int;
}
