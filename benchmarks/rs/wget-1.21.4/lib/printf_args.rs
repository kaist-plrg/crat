use ::libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = libc::c_schar;
pub type int_fast16_t = libc::c_long;
pub type int_fast32_t = libc::c_long;
pub type int_fast64_t = libc::c_long;
pub type uint_fast8_t = libc::c_uchar;
pub type uint_fast16_t = libc::c_ulong;
pub type uint_fast32_t = libc::c_ulong;
pub type uint_fast64_t = libc::c_ulong;
pub type arg_type = libc::c_uint;
pub const TYPE_COUNT_INT_FAST64_T_POINTER: arg_type = 46;
pub const TYPE_COUNT_INT_FAST32_T_POINTER: arg_type = 45;
pub const TYPE_COUNT_INT_FAST16_T_POINTER: arg_type = 44;
pub const TYPE_COUNT_INT_FAST8_T_POINTER: arg_type = 43;
pub const TYPE_COUNT_INT64_T_POINTER: arg_type = 42;
pub const TYPE_COUNT_INT32_T_POINTER: arg_type = 41;
pub const TYPE_COUNT_INT16_T_POINTER: arg_type = 40;
pub const TYPE_COUNT_INT8_T_POINTER: arg_type = 39;
pub const TYPE_COUNT_LONGLONGINT_POINTER: arg_type = 38;
pub const TYPE_COUNT_LONGINT_POINTER: arg_type = 37;
pub const TYPE_COUNT_INT_POINTER: arg_type = 36;
pub const TYPE_COUNT_SHORT_POINTER: arg_type = 35;
pub const TYPE_COUNT_SCHAR_POINTER: arg_type = 34;
pub const TYPE_POINTER: arg_type = 33;
pub const TYPE_WIDE_STRING: arg_type = 32;
pub const TYPE_STRING: arg_type = 31;
pub const TYPE_WIDE_CHAR: arg_type = 30;
pub const TYPE_CHAR: arg_type = 29;
pub const TYPE_LONGDOUBLE: arg_type = 28;
pub const TYPE_DOUBLE: arg_type = 27;
pub const TYPE_UINT_FAST64_T: arg_type = 26;
pub const TYPE_INT_FAST64_T: arg_type = 25;
pub const TYPE_UINT_FAST32_T: arg_type = 24;
pub const TYPE_INT_FAST32_T: arg_type = 23;
pub const TYPE_UINT_FAST16_T: arg_type = 22;
pub const TYPE_INT_FAST16_T: arg_type = 21;
pub const TYPE_UINT_FAST8_T: arg_type = 20;
pub const TYPE_INT_FAST8_T: arg_type = 19;
pub const TYPE_UINT64_T: arg_type = 18;
pub const TYPE_INT64_T: arg_type = 17;
pub const TYPE_UINT32_T: arg_type = 16;
pub const TYPE_INT32_T: arg_type = 15;
pub const TYPE_UINT16_T: arg_type = 14;
pub const TYPE_INT16_T: arg_type = 13;
pub const TYPE_UINT8_T: arg_type = 12;
pub const TYPE_INT8_T: arg_type = 11;
pub const TYPE_ULONGLONGINT: arg_type = 10;
pub const TYPE_LONGLONGINT: arg_type = 9;
pub const TYPE_ULONGINT: arg_type = 8;
pub const TYPE_LONGINT: arg_type = 7;
pub const TYPE_UINT: arg_type = 6;
pub const TYPE_INT: arg_type = 5;
pub const TYPE_USHORT: arg_type = 4;
pub const TYPE_SHORT: arg_type = 3;
pub const TYPE_UCHAR: arg_type = 2;
pub const TYPE_SCHAR: arg_type = 1;
pub const TYPE_NONE: arg_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argument {
    pub type_0: arg_type,
    pub a: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub a_schar: libc::c_schar,
    pub a_uchar: libc::c_uchar,
    pub a_short: libc::c_short,
    pub a_ushort: libc::c_ushort,
    pub a_int: libc::c_int,
    pub a_uint: libc::c_uint,
    pub a_longint: libc::c_long,
    pub a_ulongint: libc::c_ulong,
    pub a_longlongint: libc::c_longlong,
    pub a_ulonglongint: libc::c_ulonglong,
    pub a_int8_t: int8_t,
    pub a_uint8_t: uint8_t,
    pub a_int16_t: int16_t,
    pub a_uint16_t: uint16_t,
    pub a_int32_t: int32_t,
    pub a_uint32_t: uint32_t,
    pub a_int64_t: int64_t,
    pub a_uint64_t: uint64_t,
    pub a_int_fast8_t: int_fast8_t,
    pub a_uint_fast8_t: uint_fast8_t,
    pub a_int_fast16_t: int_fast16_t,
    pub a_uint_fast16_t: uint_fast16_t,
    pub a_int_fast32_t: int_fast32_t,
    pub a_uint_fast32_t: uint_fast32_t,
    pub a_int_fast64_t: int_fast64_t,
    pub a_uint_fast64_t: uint_fast64_t,
    pub a_float: libc::c_float,
    pub a_double: libc::c_double,
    pub a_longdouble: f128::f128,
    pub a_char: libc::c_int,
    pub a_wide_char: wint_t,
    pub a_string: *const libc::c_char,
    pub a_wide_string: *const wchar_t,
    pub a_pointer: *mut libc::c_void,
    pub a_count_schar_pointer: *mut libc::c_schar,
    pub a_count_short_pointer: *mut libc::c_short,
    pub a_count_int_pointer: *mut libc::c_int,
    pub a_count_longint_pointer: *mut libc::c_long,
    pub a_count_longlongint_pointer: *mut libc::c_longlong,
    pub a_count_int8_t_pointer: *mut int8_t,
    pub a_count_int16_t_pointer: *mut int16_t,
    pub a_count_int32_t_pointer: *mut int32_t,
    pub a_count_int64_t_pointer: *mut int64_t,
    pub a_count_int_fast8_t_pointer: *mut int_fast8_t,
    pub a_count_int_fast16_t_pointer: *mut int_fast16_t,
    pub a_count_int_fast32_t_pointer: *mut int_fast32_t,
    pub a_count_int_fast64_t_pointer: *mut int_fast64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arguments {
    pub count: size_t,
    pub arg: *mut argument,
    pub direct_alloc_arg: [argument; 7],
}
pub unsafe extern "C" fn printf_fetchargs(
    mut args: ::std::ffi::VaList,
    mut a: *mut arguments,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut ap: *mut argument = 0 as *mut argument;
    i = 0 as libc::c_int as size_t;
    ap = &mut *((*a).arg).offset(0 as libc::c_int as isize) as *mut argument;
    while i < (*a).count {
        match (*ap).type_0 as libc::c_uint {
            1 => {
                (*ap).a.a_schar = args.arg::<libc::c_int>() as libc::c_schar;
            }
            2 => {
                (*ap).a.a_uchar = args.arg::<libc::c_int>() as libc::c_uchar;
            }
            3 => {
                (*ap).a.a_short = args.arg::<libc::c_int>() as libc::c_short;
            }
            4 => {
                (*ap).a.a_ushort = args.arg::<libc::c_int>() as libc::c_ushort;
            }
            5 => {
                (*ap).a.a_int = args.arg::<libc::c_int>();
            }
            6 => {
                (*ap).a.a_uint = args.arg::<libc::c_uint>();
            }
            7 => {
                (*ap).a.a_longint = args.arg::<libc::c_long>();
            }
            8 => {
                (*ap).a.a_ulongint = args.arg::<libc::c_ulong>();
            }
            9 => {
                (*ap).a.a_longlongint = args.arg::<libc::c_longlong>();
            }
            10 => {
                (*ap).a.a_ulonglongint = args.arg::<libc::c_ulonglong>();
            }
            11 => {
                (*ap).a.a_int8_t = args.arg::<libc::c_int>() as int8_t;
            }
            12 => {
                (*ap).a.a_uint8_t = args.arg::<libc::c_int>() as uint8_t;
            }
            13 => {
                (*ap).a.a_int16_t = args.arg::<libc::c_int>() as int16_t;
            }
            14 => {
                (*ap).a.a_uint16_t = args.arg::<libc::c_int>() as uint16_t;
            }
            15 => {
                (*ap).a.a_int32_t = args.arg::<int32_t>();
            }
            16 => {
                (*ap).a.a_uint32_t = args.arg::<uint32_t>();
            }
            17 => {
                (*ap).a.a_int64_t = args.arg::<int64_t>();
            }
            18 => {
                (*ap).a.a_uint64_t = args.arg::<uint64_t>();
            }
            19 => {
                (*ap).a.a_int_fast8_t = args.arg::<libc::c_int>() as int_fast8_t;
            }
            20 => {
                (*ap).a.a_uint_fast8_t = args.arg::<libc::c_int>() as uint_fast8_t;
            }
            21 => {
                (*ap).a.a_int_fast16_t = args.arg::<int_fast16_t>();
            }
            22 => {
                (*ap).a.a_uint_fast16_t = args.arg::<uint_fast16_t>();
            }
            23 => {
                (*ap).a.a_int_fast32_t = args.arg::<int_fast32_t>();
            }
            24 => {
                (*ap).a.a_uint_fast32_t = args.arg::<uint_fast32_t>();
            }
            25 => {
                (*ap).a.a_int_fast64_t = args.arg::<int_fast64_t>();
            }
            26 => {
                (*ap).a.a_uint_fast64_t = args.arg::<uint_fast64_t>();
            }
            27 => {
                (*ap).a.a_double = args.arg::<libc::c_double>();
            }
            28 => {
                (*ap).a.a_longdouble = f128::f128::new(args.arg::<libc::c_double>());
            }
            29 => {
                (*ap).a.a_char = args.arg::<libc::c_int>();
            }
            30 => {
                (*ap)
                    .a
                    .a_wide_char = if (::std::mem::size_of::<wint_t>() as libc::c_ulong)
                    < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    args.arg::<libc::c_int>() as wint_t
                } else {
                    args.arg::<wint_t>()
                };
            }
            31 => {
                (*ap).a.a_string = args.arg::<*const libc::c_char>();
                if ((*ap).a.a_string).is_null() {
                    (*ap).a.a_string = b"(NULL)\0" as *const u8 as *const libc::c_char;
                }
            }
            32 => {
                (*ap).a.a_wide_string = args.arg::<*const wchar_t>();
                if ((*ap).a.a_wide_string).is_null() {
                    static mut wide_null_string: [wchar_t; 7] = [
                        '(' as i32,
                        'N' as i32,
                        'U' as i32,
                        'L' as i32,
                        'L' as i32,
                        ')' as i32,
                        0 as libc::c_int,
                    ];
                    (*ap).a.a_wide_string = wide_null_string.as_ptr();
                }
            }
            33 => {
                (*ap).a.a_pointer = args.arg::<*mut libc::c_void>();
            }
            34 => {
                (*ap).a.a_count_schar_pointer = args.arg::<*mut libc::c_schar>();
            }
            35 => {
                (*ap).a.a_count_short_pointer = args.arg::<*mut libc::c_short>();
            }
            36 => {
                (*ap).a.a_count_int_pointer = args.arg::<*mut libc::c_int>();
            }
            37 => {
                (*ap).a.a_count_longint_pointer = args.arg::<*mut libc::c_long>();
            }
            38 => {
                (*ap)
                    .a
                    .a_count_longlongint_pointer = args.arg::<*mut libc::c_longlong>();
            }
            39 => {
                (*ap).a.a_count_int8_t_pointer = args.arg::<*mut int8_t>();
            }
            40 => {
                (*ap).a.a_count_int16_t_pointer = args.arg::<*mut int16_t>();
            }
            41 => {
                (*ap).a.a_count_int32_t_pointer = args.arg::<*mut int32_t>();
            }
            42 => {
                (*ap).a.a_count_int64_t_pointer = args.arg::<*mut int64_t>();
            }
            43 => {
                (*ap).a.a_count_int_fast8_t_pointer = args.arg::<*mut int_fast8_t>();
            }
            44 => {
                (*ap).a.a_count_int_fast16_t_pointer = args.arg::<*mut int_fast16_t>();
            }
            45 => {
                (*ap).a.a_count_int_fast32_t_pointer = args.arg::<*mut int_fast32_t>();
            }
            46 => {
                (*ap).a.a_count_int_fast64_t_pointer = args.arg::<*mut int_fast64_t>();
            }
            _ => return -(1 as libc::c_int),
        }
        i = i.wrapping_add(1);
        i;
        ap = ap.offset(1);
        ap;
    }
    return 0 as libc::c_int;
}
