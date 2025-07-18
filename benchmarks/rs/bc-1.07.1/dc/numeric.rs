use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dc_string;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut _zero_: bc_num;
    static mut _one_: bc_num;
    fn bc_init_numbers();
    fn bc_free_num(num: *mut bc_num);
    fn bc_copy_num(num: bc_num) -> bc_num;
    fn bc_init_num(num: *mut bc_num);
    fn bc_int2num(num: *mut bc_num, val: libc::c_int);
    fn bc_num2long(num: bc_num) -> libc::c_long;
    fn bc_compare(n1: bc_num, n2: bc_num) -> libc::c_int;
    fn bc_is_zero(num: bc_num) -> libc::c_char;
    fn bc_add(n1: bc_num, n2: bc_num, result: *mut bc_num, scale_min: libc::c_int);
    fn bc_sub(n1: bc_num, n2: bc_num, result: *mut bc_num, scale_min: libc::c_int);
    fn bc_multiply(n1: bc_num, n2: bc_num, prod: *mut bc_num, scale: libc::c_int);
    fn bc_divide(
        n1: bc_num,
        n2: bc_num,
        quot: *mut bc_num,
        scale: libc::c_int,
    ) -> libc::c_int;
    fn bc_modulo(
        num1: bc_num,
        num2: bc_num,
        result: *mut bc_num,
        scale: libc::c_int,
    ) -> libc::c_int;
    fn bc_divmod(
        num1: bc_num,
        num2: bc_num,
        quot: *mut bc_num,
        rem: *mut bc_num,
        scale: libc::c_int,
    ) -> libc::c_int;
    fn bc_raisemod(
        base: bc_num,
        expo: bc_num,
        mod_0: bc_num,
        result: *mut bc_num,
        scale: libc::c_int,
    ) -> libc::c_int;
    fn bc_raise(num1: bc_num, num2: bc_num, result: *mut bc_num, scale: libc::c_int);
    fn bc_sqrt(num: *mut bc_num, scale: libc::c_int) -> libc::c_int;
    fn bc_out_num(
        num: bc_num,
        o_base: libc::c_int,
        out_char_0: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        leading_zero: libc::c_int,
    );
    static mut progname: *const libc::c_char;
    fn dc_malloc(_: size_t) -> *mut libc::c_void;
    fn dc_memfail();
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type sign = libc::c_uint;
pub const MINUS: sign = 1;
pub const PLUS: sign = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_struct {
    pub n_sign: sign,
    pub n_len: libc::c_int,
    pub n_scale: libc::c_int,
    pub n_refs: libc::c_int,
    pub n_next: bc_num,
    pub n_ptr: *mut libc::c_char,
    pub n_value: *mut libc::c_char,
}
pub type bc_num = *mut bc_struct;
pub type dc_discard = libc::c_uint;
pub const DC_KEEP: dc_discard = 1;
pub const DC_TOSS: dc_discard = 0;
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
    pub v: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub number: dc_num,
    pub string: dc_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct digit_stack {
    pub digit: libc::c_int,
    pub link: *mut digit_stack,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub static mut std_only: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn dc_add(
    mut a: dc_num,
    mut b: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    bc_init_num(result);
    bc_add(a, b, result, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_sub(
    mut a: dc_num,
    mut b: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    bc_init_num(result);
    bc_sub(a, b, result, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_mul(
    mut a: dc_num,
    mut b: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    bc_init_num(result);
    bc_multiply(a, b, result, kscale);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_div(
    mut a: dc_num,
    mut b: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    bc_init_num(result);
    if bc_divide(a, b, result, kscale) != 0 {
        fprintf(
            stderr,
            b"%s: divide by zero\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_divrem(
    mut a: dc_num,
    mut b: dc_num,
    mut kscale: libc::c_int,
    mut quotient: *mut dc_num,
    mut remainder: *mut dc_num,
) -> libc::c_int {
    bc_init_num(quotient);
    bc_init_num(remainder);
    if bc_divmod(a, b, quotient, remainder, kscale) != 0 {
        fprintf(
            stderr,
            b"%s: divide by zero\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_rem(
    mut a: dc_num,
    mut b: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    bc_init_num(result);
    if bc_modulo(a, b, result, kscale) != 0 {
        fprintf(
            stderr,
            b"%s: remainder by zero\n\0" as *const u8 as *const libc::c_char,
            progname,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_modexp(
    mut base: dc_num,
    mut expo: dc_num,
    mut mod_0: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    bc_init_num(result);
    if bc_raisemod(base, expo, mod_0, result, kscale) != 0 {
        if bc_is_zero(mod_0) != 0 {
            fprintf(
                stderr,
                b"%s: remainder by zero\n\0" as *const u8 as *const libc::c_char,
                progname,
            );
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_exp(
    mut a: dc_num,
    mut b: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    bc_init_num(result);
    bc_raise(a, b, result, kscale);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_sqrt(
    mut value: dc_num,
    mut kscale: libc::c_int,
    mut result: *mut dc_num,
) -> libc::c_int {
    let mut tmp: bc_num = 0 as *mut bc_struct;
    tmp = bc_copy_num(value);
    if bc_sqrt(&mut tmp, kscale) == 0 {
        fprintf(
            stderr,
            b"%s: square root of negative number\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        bc_free_num(&mut tmp);
        return 1 as libc::c_int;
    }
    *result = tmp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn dc_compare(mut a: dc_num, mut b: dc_num) -> libc::c_int {
    return bc_compare(a, b);
}
pub unsafe extern "C" fn dc_num2int(
    mut value: dc_num,
    mut discard_p: dc_discard,
) -> libc::c_int {
    let mut result: libc::c_long = 0;
    result = bc_num2long(value);
    if result == 0 as libc::c_int as libc::c_long && bc_is_zero(value) == 0 {
        fprintf(
            stderr,
            b"%s: value overflows simple integer; punting...\n\0" as *const u8
                as *const libc::c_char,
            progname,
        );
        result = -(1 as libc::c_int) as libc::c_long;
    }
    if discard_p as libc::c_uint == DC_TOSS as libc::c_int as libc::c_uint {
        dc_free_num(&mut value);
    }
    return result as libc::c_int;
}
pub unsafe extern "C" fn dc_int2data(mut value: libc::c_int) -> dc_data {
    let mut result: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed_0 {
            number: 0 as *mut bc_struct,
        },
    };
    bc_init_num(&mut result.v.number);
    bc_int2num(&mut result.v.number, value);
    result.dc_type = DC_NUMBER;
    return result;
}
pub unsafe extern "C" fn dc_getnum(
    mut input: Option::<unsafe extern "C" fn() -> libc::c_int>,
    mut ibase: libc::c_int,
    mut readahead: *mut libc::c_int,
) -> dc_data {
    let mut base: bc_num = 0 as *mut bc_struct;
    let mut result: bc_num = 0 as *mut bc_struct;
    let mut build: bc_num = 0 as *mut bc_struct;
    let mut tmp: bc_num = 0 as *mut bc_struct;
    let mut divisor: bc_num = 0 as *mut bc_struct;
    let mut full_result: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed_0 {
            number: 0 as *mut bc_struct,
        },
    };
    let mut negative: libc::c_int = 0 as libc::c_int;
    let mut digit: libc::c_int = 0;
    let mut decimal: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    bc_init_num(&mut tmp);
    bc_init_num(&mut build);
    bc_init_num(&mut base);
    result = bc_copy_num(_zero_);
    bc_int2num(&mut base, ibase);
    c = (Some(input.unwrap())).unwrap()();
    while *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        c = (Some(input.unwrap())).unwrap()();
    }
    if c == '_' as i32 || c == '-' as i32 {
        negative = c;
        c = (Some(input.unwrap())).unwrap()();
    } else if c == '+' as i32 {
        c = (Some(input.unwrap())).unwrap()();
    }
    while *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        c = (Some(input.unwrap())).unwrap()();
    }
    loop {
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            digit = c - '0' as i32;
        } else {
            if !('A' as i32 <= c && c <= 'F' as i32) {
                break;
            }
            digit = 10 as libc::c_int + c - 'A' as i32;
        }
        c = (Some(input.unwrap())).unwrap()();
        bc_int2num(&mut tmp, digit);
        bc_multiply(result, base, &mut result, 0 as libc::c_int);
        bc_add(result, tmp, &mut result, 0 as libc::c_int);
    }
    if c == '.' as i32 {
        bc_free_num(&mut build);
        bc_free_num(&mut tmp);
        divisor = bc_copy_num(_one_);
        build = bc_copy_num(_zero_);
        decimal = 0 as libc::c_int;
        loop {
            c = (Some(input.unwrap())).unwrap()();
            if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                digit = c - '0' as i32;
            } else {
                if !('A' as i32 <= c && c <= 'F' as i32) {
                    break;
                }
                digit = 10 as libc::c_int + c - 'A' as i32;
            }
            bc_int2num(&mut tmp, digit);
            bc_multiply(build, base, &mut build, 0 as libc::c_int);
            bc_add(build, tmp, &mut build, 0 as libc::c_int);
            bc_multiply(divisor, base, &mut divisor, 0 as libc::c_int);
            decimal += 1;
            decimal;
        }
        bc_divide(build, divisor, &mut build, decimal);
        bc_add(result, build, &mut result, 0 as libc::c_int);
    }
    if negative != 0 {
        bc_sub(_zero_, result, &mut result, 0 as libc::c_int);
    }
    bc_free_num(&mut tmp);
    bc_free_num(&mut build);
    bc_free_num(&mut base);
    if !readahead.is_null() {
        *readahead = c;
    }
    full_result.v.number = result;
    full_result.dc_type = DC_NUMBER;
    return full_result;
}
pub unsafe extern "C" fn dc_numlen(mut value: dc_num) -> libc::c_int {
    let mut num: bc_num = value;
    let mut p: *mut libc::c_char = (*num).n_value;
    let mut i: libc::c_int = (*num).n_len + (*num).n_scale;
    while (1 as libc::c_int) < i && *p as libc::c_int == '\0' as i32 {
        i -= 1;
        i;
        p = p.offset(1);
        p;
    }
    return i;
}
pub unsafe extern "C" fn dc_tell_scale(
    mut value: dc_num,
    mut discard_p: dc_discard,
) -> libc::c_int {
    let mut kscale: libc::c_int = 0;
    kscale = (*value).n_scale;
    if discard_p as libc::c_uint == DC_TOSS as libc::c_int as libc::c_uint {
        dc_free_num(&mut value);
    }
    return kscale;
}
pub unsafe extern "C" fn dc_math_init() {
    bc_init_numbers();
}
pub unsafe extern "C" fn dc_out_num(
    mut value: dc_num,
    mut obase: libc::c_int,
    mut discard_p: dc_discard,
) {
    out_char('\0' as i32);
    bc_out_num(
        value,
        obase,
        Some(out_char as unsafe extern "C" fn(libc::c_int) -> ()),
        0 as libc::c_int,
    );
    if discard_p as libc::c_uint == DC_TOSS as libc::c_int as libc::c_uint {
        dc_free_num(&mut value);
    }
}
pub unsafe extern "C" fn dc_dump_num(mut dcvalue: dc_num, mut discard_p: dc_discard) {
    let mut top_of_stack: *mut digit_stack = 0 as *mut digit_stack;
    let mut cur: *mut digit_stack = 0 as *mut digit_stack;
    let mut next: *mut digit_stack = 0 as *mut digit_stack;
    let mut value: bc_num = 0 as *mut bc_struct;
    let mut obase: bc_num = 0 as *mut bc_struct;
    let mut digit: bc_num = 0 as *mut bc_struct;
    bc_init_num(&mut value);
    bc_init_num(&mut obase);
    bc_init_num(&mut digit);
    bc_divide(dcvalue, _one_, &mut value, 0 as libc::c_int);
    (*value).n_sign = PLUS;
    if discard_p as libc::c_uint == DC_TOSS as libc::c_int as libc::c_uint {
        dc_free_num(&mut dcvalue);
    }
    bc_int2num(
        &mut obase,
        1 as libc::c_int + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int),
    );
    loop {
        bc_divmod(value, obase, &mut value, &mut digit, 0 as libc::c_int);
        cur = dc_malloc(::std::mem::size_of::<digit_stack>() as libc::c_ulong)
            as *mut digit_stack;
        (*cur).digit = bc_num2long(digit) as libc::c_int;
        (*cur).link = top_of_stack;
        top_of_stack = cur;
        if !(bc_is_zero(value) == 0) {
            break;
        }
    }
    cur = top_of_stack;
    while !cur.is_null() {
        putchar((*cur).digit);
        next = (*cur).link;
        free(cur as *mut libc::c_void);
        cur = next;
    }
    bc_free_num(&mut digit);
    bc_free_num(&mut obase);
    bc_free_num(&mut value);
}
pub unsafe extern "C" fn dc_free_num(mut value: *mut dc_num) {
    bc_free_num(value);
}
pub unsafe extern "C" fn dc_dup_num(mut value: dc_num) -> dc_data {
    let mut result: dc_data = dc_data {
        dc_type: DC_UNINITIALIZED,
        v: C2RustUnnamed_0 {
            number: 0 as *mut bc_struct,
        },
    };
    (*value).n_refs += 1;
    (*value).n_refs;
    result.v.number = value;
    result.dc_type = DC_NUMBER;
    return result;
}
static mut out_col: libc::c_int = 0 as libc::c_int;
static mut line_max: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn set_line_max_from_environment() {
    let mut env_line_len: *const libc::c_char = getenv(
        b"DC_LINE_LENGTH\0" as *const u8 as *const libc::c_char,
    );
    line_max = 70 as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    if !env_line_len.is_null() {
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut proposed_line_len: libc::c_long = strtol(
            env_line_len,
            &mut endptr,
            0 as libc::c_int,
        );
        line_max = proposed_line_len as libc::c_int;
        while *(*__ctype_b_loc())
            .offset(*(endptr as *const libc::c_uchar) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            endptr = endptr.offset(1);
            endptr;
        }
        if *endptr as libc::c_int != 0 || *__errno_location() != 0
            || line_max as libc::c_long != proposed_line_len
            || line_max < 0 as libc::c_int || line_max == 1 as libc::c_int
        {
            line_max = 70 as libc::c_int;
        }
    }
}
unsafe extern "C" fn out_char(mut ch: libc::c_int) {
    if ch == '\0' as i32 {
        out_col = 0 as libc::c_int;
    } else {
        if line_max < 0 as libc::c_int {
            set_line_max_from_environment();
        }
        out_col += 1;
        if out_col >= line_max && line_max != 0 as libc::c_int {
            putchar('\\' as i32);
            putchar('\n' as i32);
            out_col = 1 as libc::c_int;
        }
        putchar(ch);
    };
}
pub unsafe extern "C" fn out_of_memory() {
    dc_memfail();
}
pub unsafe extern "C" fn rt_error(mut mesg: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(stderr, b"Runtime error: \0" as *const u8 as *const libc::c_char);
    args_0 = args.clone();
    vfprintf(stderr, mesg, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn rt_warn(mut mesg: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(stderr, b"Runtime warning: \0" as *const u8 as *const libc::c_char);
    args_0 = args.clone();
    vfprintf(stderr, mesg, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
