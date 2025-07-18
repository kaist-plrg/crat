use ::libc;
extern "C" {
    pub type strm_queue;
    pub type node_error;
    fn strm_raise(_: *mut strm_stream, _: *const libc::c_char);
    fn strm_ary_struct(_: strm_array) -> *mut strm_array_0;
    fn strm_str_len(_: strm_string) -> strm_int;
    fn strm_strp_ptr(_: *mut strm_string) -> *const libc::c_char;
    fn strm_string_p(_: strm_value) -> libc::c_int;
    fn strm_array_p(_: strm_value) -> libc::c_int;
    fn strm_number_p(_: strm_value) -> libc::c_int;
    fn strm_bool_p(_: strm_value) -> libc::c_int;
    fn strm_nil_p(_: strm_value) -> libc::c_int;
    fn strm_value_float(_: strm_value) -> libc::c_double;
    fn strm_value_bool(_: strm_value) -> libc::c_int;
    fn strm_value_int(_: strm_value) -> strm_int;
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type strm_string = uint64_t;
pub type strm_array = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_array_0 {
    pub len: strm_int,
    pub ptr: *mut strm_value,
    pub headers: strm_array,
    pub ns: *mut strm_state,
}
pub unsafe extern "C" fn strm_parse_args(
    mut strm: *mut strm_stream,
    mut argc: libc::c_int,
    mut argv: *mut strm_value,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ap: ::std::ffi::VaListImpl;
    let mut arg_i: libc::c_int = 0 as libc::c_int;
    let mut opt: strm_int = 0 as libc::c_int;
    let mut given: strm_int = 1 as libc::c_int;
    ap = args.clone();
    loop {
        let fresh0 = format;
        format = format.offset(1);
        c = *fresh0;
        if !(c != 0) {
            break;
        }
        match c as libc::c_int {
            124 | 42 | 38 | 63 => {}
            _ => {
                if argc <= i {
                    if opt != 0 {
                        given = 0 as libc::c_int;
                    } else {
                        strm_raise(
                            strm,
                            b"wrong number of arguments\0" as *const u8
                                as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                }
            }
        }
        match c as libc::c_int {
            118 => {
                let mut p: *mut strm_value = 0 as *mut strm_value;
                p = ap.arg::<*mut strm_value>();
                if i < argc {
                    let fresh1 = arg_i;
                    arg_i = arg_i + 1;
                    *p = *argv.offset(fresh1 as isize);
                    i += 1;
                    i;
                }
            }
            78 => {
                let mut p_0: *mut strm_value = 0 as *mut strm_value;
                p_0 = ap.arg::<*mut strm_value>();
                if i < argc {
                    let fresh2 = arg_i;
                    arg_i = arg_i + 1;
                    let mut nn: strm_value = *argv.offset(fresh2 as isize);
                    if strm_number_p(nn) == 0 {
                        strm_raise(
                            strm,
                            b"number required\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    *p_0 = nn;
                    i += 1;
                    i;
                }
            }
            83 => {
                let mut ss: strm_value = 0;
                let mut p_1: *mut strm_string = 0 as *mut strm_string;
                p_1 = ap.arg::<*mut strm_string>();
                if i < argc {
                    let fresh3 = arg_i;
                    arg_i = arg_i + 1;
                    ss = *argv.offset(fresh3 as isize);
                    i += 1;
                    i;
                    if strm_string_p(ss) == 0 {
                        strm_raise(
                            strm,
                            b"string required\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    *p_1 = ss;
                }
            }
            115 => {
                let mut ss_0: strm_value = 0;
                let mut ps: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
                let mut pl: *mut strm_int = 0 as *mut strm_int;
                ps = ap.arg::<*mut *const libc::c_char>();
                pl = ap.arg::<*mut strm_int>();
                if i < argc {
                    ss_0 = *argv.offset(arg_i as isize);
                    if strm_string_p(ss_0) == 0 {
                        strm_raise(
                            strm,
                            b"string required\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    *ps = strm_strp_ptr(&mut *argv.offset(arg_i as isize));
                    *pl = strm_str_len(ss_0);
                    i += 1;
                    i;
                    arg_i += 1;
                    arg_i;
                }
            }
            65 => {
                let mut p_2: *mut strm_array = 0 as *mut strm_array;
                let mut v: strm_value = 0;
                p_2 = ap.arg::<*mut strm_array>();
                if i < argc {
                    let fresh4 = arg_i;
                    arg_i = arg_i + 1;
                    v = *argv.offset(fresh4 as isize);
                    i += 1;
                    i;
                    if strm_array_p(v) == 0 {
                        strm_raise(
                            strm,
                            b"array required\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    *p_2 = v;
                }
            }
            97 => {
                let mut aa: strm_array = 0;
                let mut pb: *mut *mut strm_value = 0 as *mut *mut strm_value;
                let mut pl_0: *mut strm_int = 0 as *mut strm_int;
                pb = ap.arg::<*mut *mut strm_value>();
                pl_0 = ap.arg::<*mut strm_int>();
                if i < argc {
                    let fresh5 = arg_i;
                    arg_i = arg_i + 1;
                    aa = *argv.offset(fresh5 as isize);
                    i += 1;
                    i;
                    if strm_nil_p(aa) != 0 {
                        *pb = 0 as *mut strm_value;
                        *pl_0 = 0 as libc::c_int;
                    } else {
                        if strm_array_p(aa) == 0 {
                            strm_raise(
                                strm,
                                b"array required\0" as *const u8 as *const libc::c_char,
                            );
                            return 1 as libc::c_int;
                        }
                        *pb = (*strm_ary_struct(aa)).ptr;
                        *pl_0 = (*strm_ary_struct(aa)).len;
                    }
                }
            }
            102 => {
                let mut p_3: *mut libc::c_double = 0 as *mut libc::c_double;
                let mut ff: strm_value = 0;
                p_3 = ap.arg::<*mut libc::c_double>();
                if i < argc {
                    let fresh6 = arg_i;
                    arg_i = arg_i + 1;
                    ff = *argv.offset(fresh6 as isize);
                    i += 1;
                    i;
                    if strm_number_p(ff) == 0 {
                        strm_raise(
                            strm,
                            b"number required\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    *p_3 = strm_value_float(ff);
                }
            }
            105 => {
                let mut p_4: *mut strm_int = 0 as *mut strm_int;
                let mut ff_0: strm_value = 0;
                p_4 = ap.arg::<*mut strm_int>();
                if i < argc {
                    let fresh7 = arg_i;
                    arg_i = arg_i + 1;
                    ff_0 = *argv.offset(fresh7 as isize);
                    i += 1;
                    i;
                    if strm_number_p(ff_0) == 0 {
                        strm_raise(
                            strm,
                            b"number required\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    *p_4 = strm_value_int(ff_0);
                }
            }
            98 => {
                let mut p_5: *mut strm_int = 0 as *mut strm_int;
                let mut bb: strm_int = 0;
                p_5 = ap.arg::<*mut strm_int>();
                if i < argc {
                    let fresh8 = arg_i;
                    arg_i = arg_i + 1;
                    bb = *argv.offset(fresh8 as isize) as strm_int;
                    i += 1;
                    i;
                    if strm_bool_p(bb as strm_value) == 0 {
                        strm_raise(
                            strm,
                            b"boolean required\0" as *const u8 as *const libc::c_char,
                        );
                        return 1 as libc::c_int;
                    }
                    *p_5 = strm_value_bool(bb as strm_value);
                }
            }
            124 => {
                opt = 1 as libc::c_int;
            }
            63 => {
                let mut p_6: *mut strm_int = 0 as *mut strm_int;
                p_6 = ap.arg::<*mut strm_int>();
                *p_6 = given;
            }
            _ => {
                strm_raise(
                    strm,
                    b"invalid argument specifier\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if c == 0 && argc > i {
        strm_raise(
            strm,
            b"wrong number of arguments\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
