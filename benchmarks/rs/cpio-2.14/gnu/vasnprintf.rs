use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn abort() -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn printf_fetchargs(args: ::std::ffi::VaList, a: *mut arguments) -> libc::c_int;
    fn printf_parse(
        format: *const libc::c_char,
        d: *mut char_directives,
        a: *mut arguments,
    ) -> libc::c_int;
}
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
pub type int_fast64_t = libc::c_long;
pub type int_fast32_t = libc::c_long;
pub type int_fast16_t = libc::c_long;
pub type int_fast8_t = libc::c_schar;
pub type wint_t = libc::c_uint;
pub type uint_fast64_t = libc::c_ulong;
pub type uint_fast32_t = libc::c_ulong;
pub type uint_fast16_t = libc::c_ulong;
pub type uint_fast8_t = libc::c_uchar;
pub type uint64_t = __uint64_t;
pub type uint32_t = __uint32_t;
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
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
pub struct arguments {
    pub count: size_t,
    pub arg: *mut argument,
    pub direct_alloc_arg: [argument; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_directive {
    pub dir_start: *const libc::c_char,
    pub dir_end: *const libc::c_char,
    pub flags: libc::c_int,
    pub width_start: *const libc::c_char,
    pub width_end: *const libc::c_char,
    pub width_arg_index: size_t,
    pub precision_start: *const libc::c_char,
    pub precision_end: *const libc::c_char,
    pub precision_arg_index: size_t,
    pub conversion: libc::c_char,
    pub arg_index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_directives {
    pub count: size_t,
    pub dir: *mut char_directive,
    pub max_width_length: size_t,
    pub max_precision_length: size_t,
    pub direct_alloc_dir: [char_directive; 7],
}
#[inline]
unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as libc::c_ulong };
}
#[inline]
unsafe extern "C" fn xsum4(
    mut size1: size_t,
    mut size2: size_t,
    mut size3: size_t,
    mut size4: size_t,
) -> size_t {
    return xsum(xsum(xsum(size1, size2), size3), size4);
}
#[inline]
unsafe extern "C" fn xmax(mut size1: size_t, mut size2: size_t) -> size_t {
    return if size1 >= size2 { size1 } else { size2 };
}
pub unsafe extern "C" fn vasnprintf(
    mut resultbuf: *mut libc::c_char,
    mut lengthp: *mut size_t,
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut d: char_directives = char_directives {
        count: 0,
        dir: 0 as *mut char_directive,
        max_width_length: 0,
        max_precision_length: 0,
        direct_alloc_dir: [char_directive {
            dir_start: 0 as *const libc::c_char,
            dir_end: 0 as *const libc::c_char,
            flags: 0,
            width_start: 0 as *const libc::c_char,
            width_end: 0 as *const libc::c_char,
            width_arg_index: 0,
            precision_start: 0 as *const libc::c_char,
            precision_end: 0 as *const libc::c_char,
            precision_arg_index: 0,
            conversion: 0,
            arg_index: 0,
        }; 7],
    };
    let mut a: arguments = arguments {
        count: 0,
        arg: 0 as *mut argument,
        direct_alloc_arg: [argument {
            type_0: TYPE_NONE,
            a: C2RustUnnamed { a_schar: 0 },
        }; 7],
    };
    if printf_parse(format, &mut d, &mut a) < 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if printf_fetchargs(args.as_va_list(), &mut a) < 0 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
    } else {
        let mut buf_neededlength: size_t = 0;
        let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf_malloced: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut cp: *const libc::c_char = 0 as *const libc::c_char;
        let mut i: size_t = 0;
        let mut dp: *mut char_directive = 0 as *mut char_directive;
        let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut allocated: size_t = 0;
        let mut length: size_t = 0;
        buf_neededlength = xsum4(
            7 as libc::c_int as size_t,
            d.max_width_length,
            d.max_precision_length,
            6 as libc::c_int as size_t,
        );
        if buf_neededlength
            < (4000 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        {
            let mut fresh0 = ::std::vec::from_elem(
                0,
                buf_neededlength
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as usize,
            );
            buf = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
            buf_malloced = 0 as *mut libc::c_char;
            current_block = 10048703153582371463;
        } else {
            let mut buf_memsize: size_t = if buf_neededlength
                <= (18446744073709551615 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            {
                buf_neededlength
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            } else {
                18446744073709551615 as libc::c_ulong
            };
            if buf_memsize == 18446744073709551615 as libc::c_ulong {
                current_block = 1962964257086291845;
            } else {
                buf = malloc(buf_memsize) as *mut libc::c_char;
                if buf.is_null() {
                    current_block = 1962964257086291845;
                } else {
                    buf_malloced = buf;
                    current_block = 10048703153582371463;
                }
            }
            match current_block {
                10048703153582371463 => {}
                _ => {
                    *__errno_location() = 12 as libc::c_int;
                    current_block = 13093426784173366444;
                }
            }
        }
        match current_block {
            13093426784173366444 => {}
            _ => {
                result = resultbuf;
                allocated = if !resultbuf.is_null() {
                    *lengthp
                } else {
                    0 as libc::c_int as libc::c_ulong
                };
                length = 0 as libc::c_int as size_t;
                cp = format;
                i = 0 as libc::c_int as size_t;
                dp = &mut *(d.dir).offset(0 as libc::c_int as isize)
                    as *mut char_directive;
                's_92: loop {
                    if cp != (*dp).dir_start {
                        let mut n: size_t = ((*dp).dir_start).offset_from(cp)
                            as libc::c_long as size_t;
                        let mut augmented_length: size_t = xsum(length, n);
                        if augmented_length > allocated {
                            let mut memory_size: size_t = 0;
                            let mut memory: *mut libc::c_char = 0 as *mut libc::c_char;
                            allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                            {
                                if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                {
                                    allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                }
                            } else {
                                12 as libc::c_int as libc::c_ulong
                            };
                            if augmented_length > allocated {
                                allocated = augmented_length;
                            }
                            memory_size = if allocated
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            {
                                allocated
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            } else {
                                18446744073709551615 as libc::c_ulong
                            };
                            if memory_size == 18446744073709551615 as libc::c_ulong {
                                current_block = 14593156655374830084;
                                break;
                            }
                            if result == resultbuf {
                                memory = malloc(memory_size) as *mut libc::c_char;
                            } else {
                                memory = realloc(result as *mut libc::c_void, memory_size)
                                    as *mut libc::c_char;
                            }
                            if memory.is_null() {
                                current_block = 14593156655374830084;
                                break;
                            }
                            if result == resultbuf
                                && length > 0 as libc::c_int as libc::c_ulong
                            {
                                memcpy(
                                    memory as *mut libc::c_void,
                                    result as *const libc::c_void,
                                    length,
                                );
                            }
                            result = memory;
                        }
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        {
                            memcpy(
                                result.offset(length as isize) as *mut libc::c_void,
                                cp as *const libc::c_void,
                                n,
                            );
                            length = augmented_length;
                        } else {
                            loop {
                                let fresh1 = cp;
                                cp = cp.offset(1);
                                let fresh2 = length;
                                length = length.wrapping_add(1);
                                *result.offset(fresh2 as isize) = *fresh1;
                                n = n.wrapping_sub(1);
                                if !(n > 0 as libc::c_int as libc::c_ulong) {
                                    break;
                                }
                            }
                        }
                    }
                    if i == d.count {
                        current_block = 10060946412984486177;
                        break;
                    }
                    if (*dp).conversion as libc::c_int == '%' as i32 {
                        let mut augmented_length_0: size_t = 0;
                        if !((*dp).arg_index == !(0 as libc::c_int as size_t)) {
                            abort();
                        }
                        augmented_length_0 = xsum(length, 1 as libc::c_int as size_t);
                        if augmented_length_0 > allocated {
                            let mut memory_size_0: size_t = 0;
                            let mut memory_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                            {
                                if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                {
                                    allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                }
                            } else {
                                12 as libc::c_int as libc::c_ulong
                            };
                            if augmented_length_0 > allocated {
                                allocated = augmented_length_0;
                            }
                            memory_size_0 = if allocated
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            {
                                allocated
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            } else {
                                18446744073709551615 as libc::c_ulong
                            };
                            if memory_size_0 == 18446744073709551615 as libc::c_ulong {
                                current_block = 14593156655374830084;
                                break;
                            }
                            if result == resultbuf {
                                memory_0 = malloc(memory_size_0) as *mut libc::c_char;
                            } else {
                                memory_0 = realloc(
                                    result as *mut libc::c_void,
                                    memory_size_0,
                                ) as *mut libc::c_char;
                            }
                            if memory_0.is_null() {
                                current_block = 14593156655374830084;
                                break;
                            }
                            if result == resultbuf
                                && length > 0 as libc::c_int as libc::c_ulong
                            {
                                memcpy(
                                    memory_0 as *mut libc::c_void,
                                    result as *const libc::c_void,
                                    length,
                                );
                            }
                            result = memory_0;
                        }
                        *result.offset(length as isize) = '%' as i32 as libc::c_char;
                        length = augmented_length_0;
                    } else {
                        if !((*dp).arg_index != !(0 as libc::c_int as size_t)) {
                            abort();
                        }
                        if (*dp).conversion as libc::c_int == 'n' as i32 {
                            match (*(a.arg).offset((*dp).arg_index as isize)).type_0
                                as libc::c_uint
                            {
                                34 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_schar_pointer = length as libc::c_schar;
                                }
                                35 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_short_pointer = length as libc::c_short;
                                }
                                36 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int_pointer = length as libc::c_int;
                                }
                                37 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_longint_pointer = length as libc::c_long;
                                }
                                38 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_longlongint_pointer = length as libc::c_longlong;
                                }
                                39 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int8_t_pointer = length as int8_t;
                                }
                                40 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int16_t_pointer = length as int16_t;
                                }
                                41 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int32_t_pointer = length as int32_t;
                                }
                                42 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int64_t_pointer = length as int64_t;
                                }
                                43 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int_fast8_t_pointer = length as int_fast8_t;
                                }
                                44 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int_fast16_t_pointer = length as int_fast16_t;
                                }
                                45 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int_fast32_t_pointer = length as int_fast32_t;
                                }
                                46 => {
                                    *(*(a.arg).offset((*dp).arg_index as isize))
                                        .a
                                        .a_count_int_fast64_t_pointer = length as int_fast64_t;
                                }
                                _ => {
                                    abort();
                                }
                            }
                        } else {
                            let mut type_0: arg_type = (*(a.arg)
                                .offset((*dp).arg_index as isize))
                                .type_0;
                            let mut flags: libc::c_int = (*dp).flags;
                            let mut fbp: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut prefix_count: libc::c_uint = 0;
                            let mut prefixes: [libc::c_int; 2] = [0; 2];
                            let mut orig_errno: libc::c_int = 0;
                            fbp = buf;
                            let fresh3 = fbp;
                            fbp = fbp.offset(1);
                            *fresh3 = '%' as i32 as libc::c_char;
                            if flags & 1 as libc::c_int != 0 {
                                let fresh4 = fbp;
                                fbp = fbp.offset(1);
                                *fresh4 = '\'' as i32 as libc::c_char;
                            }
                            if flags & 2 as libc::c_int != 0 {
                                let fresh5 = fbp;
                                fbp = fbp.offset(1);
                                *fresh5 = '-' as i32 as libc::c_char;
                            }
                            if flags & 4 as libc::c_int != 0 {
                                let fresh6 = fbp;
                                fbp = fbp.offset(1);
                                *fresh6 = '+' as i32 as libc::c_char;
                            }
                            if flags & 8 as libc::c_int != 0 {
                                let fresh7 = fbp;
                                fbp = fbp.offset(1);
                                *fresh7 = ' ' as i32 as libc::c_char;
                            }
                            if flags & 16 as libc::c_int != 0 {
                                let fresh8 = fbp;
                                fbp = fbp.offset(1);
                                *fresh8 = '#' as i32 as libc::c_char;
                            }
                            if flags & 64 as libc::c_int != 0 {
                                let fresh9 = fbp;
                                fbp = fbp.offset(1);
                                *fresh9 = 'I' as i32 as libc::c_char;
                            }
                            if 0 as libc::c_int == 0 {
                                if flags & 32 as libc::c_int != 0 {
                                    let fresh10 = fbp;
                                    fbp = fbp.offset(1);
                                    *fresh10 = '0' as i32 as libc::c_char;
                                }
                                if (*dp).width_start != (*dp).width_end {
                                    let mut n_0: size_t = ((*dp).width_end)
                                        .offset_from((*dp).width_start) as libc::c_long as size_t;
                                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        == ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    {
                                        memcpy(
                                            fbp as *mut libc::c_void,
                                            (*dp).width_start as *const libc::c_void,
                                            n_0
                                                .wrapping_mul(
                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        );
                                        fbp = fbp.offset(n_0 as isize);
                                    } else {
                                        let mut mp: *const libc::c_char = (*dp).width_start;
                                        loop {
                                            let fresh11 = mp;
                                            mp = mp.offset(1);
                                            let fresh12 = fbp;
                                            fbp = fbp.offset(1);
                                            *fresh12 = *fresh11;
                                            n_0 = n_0.wrapping_sub(1);
                                            if !(n_0 > 0 as libc::c_int as libc::c_ulong) {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            if 0 as libc::c_int == 0 {
                                if (*dp).precision_start != (*dp).precision_end {
                                    let mut n_1: size_t = ((*dp).precision_end)
                                        .offset_from((*dp).precision_start) as libc::c_long
                                        as size_t;
                                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                        == ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                                    {
                                        memcpy(
                                            fbp as *mut libc::c_void,
                                            (*dp).precision_start as *const libc::c_void,
                                            n_1
                                                .wrapping_mul(
                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        );
                                        fbp = fbp.offset(n_1 as isize);
                                    } else {
                                        let mut mp_0: *const libc::c_char = (*dp).precision_start;
                                        loop {
                                            let fresh13 = mp_0;
                                            mp_0 = mp_0.offset(1);
                                            let fresh14 = fbp;
                                            fbp = fbp.offset(1);
                                            *fresh14 = *fresh13;
                                            n_1 = n_1.wrapping_sub(1);
                                            if !(n_1 > 0 as libc::c_int as libc::c_ulong) {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            let mut current_block_105: u64;
                            match type_0 as libc::c_uint {
                                9 | 10 => {
                                    let fresh15 = fbp;
                                    fbp = fbp.offset(1);
                                    *fresh15 = 'l' as i32 as libc::c_char;
                                    current_block_105 = 9718172846789742953;
                                }
                                7 | 8 | 17 | 18 | 21 | 22 | 23 | 24 | 25 | 26 | 30 | 32 => {
                                    current_block_105 = 9718172846789742953;
                                }
                                28 => {
                                    let fresh17 = fbp;
                                    fbp = fbp.offset(1);
                                    *fresh17 = 'L' as i32 as libc::c_char;
                                    current_block_105 = 4485073238441121731;
                                }
                                _ => {
                                    current_block_105 = 4485073238441121731;
                                }
                            }
                            match current_block_105 {
                                9718172846789742953 => {
                                    let fresh16 = fbp;
                                    fbp = fbp.offset(1);
                                    *fresh16 = 'l' as i32 as libc::c_char;
                                }
                                _ => {}
                            }
                            *fbp = (*dp).conversion;
                            *fbp
                                .offset(
                                    1 as libc::c_int as isize,
                                ) = '\0' as i32 as libc::c_char;
                            prefix_count = 0 as libc::c_int as libc::c_uint;
                            if 0 as libc::c_int == 0
                                && (*dp).width_arg_index != !(0 as libc::c_int as size_t)
                            {
                                if !((*(a.arg).offset((*dp).width_arg_index as isize))
                                    .type_0 as libc::c_uint
                                    == TYPE_INT as libc::c_int as libc::c_uint)
                                {
                                    abort();
                                }
                                let fresh18 = prefix_count;
                                prefix_count = prefix_count.wrapping_add(1);
                                prefixes[fresh18
                                    as usize] = (*(a.arg)
                                    .offset((*dp).width_arg_index as isize))
                                    .a
                                    .a_int;
                            }
                            if 0 as libc::c_int == 0
                                && (*dp).precision_arg_index
                                    != !(0 as libc::c_int as size_t)
                            {
                                if !((*(a.arg).offset((*dp).precision_arg_index as isize))
                                    .type_0 as libc::c_uint
                                    == TYPE_INT as libc::c_int as libc::c_uint)
                                {
                                    abort();
                                }
                                let fresh19 = prefix_count;
                                prefix_count = prefix_count.wrapping_add(1);
                                prefixes[fresh19
                                    as usize] = (*(a.arg)
                                    .offset((*dp).precision_arg_index as isize))
                                    .a
                                    .a_int;
                            }
                            if xsum(
                                length,
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    )
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_div(
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    ),
                            ) > allocated
                            {
                                let mut memory_size_1: size_t = 0;
                                let mut memory_1: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                                {
                                    if allocated
                                        <= (18446744073709551615 as libc::c_ulong)
                                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                    {
                                        allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    } else {
                                        18446744073709551615 as libc::c_ulong
                                    }
                                } else {
                                    12 as libc::c_int as libc::c_ulong
                                };
                                if xsum(
                                    length,
                                    (2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(
                                            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                .wrapping_div(
                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        )
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_div(
                                            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                .wrapping_div(
                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        ),
                                ) > allocated
                                {
                                    allocated = xsum(
                                        length,
                                        (2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(
                                                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                    .wrapping_div(
                                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                    ),
                                            )
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_div(
                                                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                    .wrapping_div(
                                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                    ),
                                            ),
                                    );
                                }
                                memory_size_1 = if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                {
                                    allocated
                                        .wrapping_mul(
                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                        )
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                };
                                if memory_size_1 == 18446744073709551615 as libc::c_ulong {
                                    current_block = 14593156655374830084;
                                    break;
                                }
                                if result == resultbuf {
                                    memory_1 = malloc(memory_size_1) as *mut libc::c_char;
                                } else {
                                    memory_1 = realloc(
                                        result as *mut libc::c_void,
                                        memory_size_1,
                                    ) as *mut libc::c_char;
                                }
                                if memory_1.is_null() {
                                    current_block = 14593156655374830084;
                                    break;
                                }
                                if result == resultbuf
                                    && length > 0 as libc::c_int as libc::c_ulong
                                {
                                    memcpy(
                                        memory_1 as *mut libc::c_void,
                                        result as *const libc::c_void,
                                        length,
                                    );
                                }
                                result = memory_1;
                            }
                            *result
                                .offset(length as isize) = '\0' as i32 as libc::c_char;
                            orig_errno = *__errno_location();
                            loop {
                                let mut count: libc::c_int = -(1 as libc::c_int);
                                let mut retcount: libc::c_int = 0 as libc::c_int;
                                let mut maxlen: size_t = allocated.wrapping_sub(length);
                                if maxlen
                                    > (2147483647 as libc::c_int as libc::c_ulong)
                                        .wrapping_div(
                                            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                .wrapping_div(
                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        )
                                {
                                    maxlen = (2147483647 as libc::c_int as libc::c_ulong)
                                        .wrapping_div(
                                            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                .wrapping_div(
                                                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                ),
                                        );
                                }
                                maxlen = maxlen
                                    .wrapping_mul(
                                        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    );
                                *__errno_location() = 0 as libc::c_int;
                                match type_0 as libc::c_uint {
                                    1 => {
                                        let mut arg: libc::c_int = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_schar as libc::c_int;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    2 => {
                                        let mut arg_0: libc::c_uint = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uchar as libc::c_uint;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_0,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_0,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_0,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    3 => {
                                        let mut arg_1: libc::c_int = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_short as libc::c_int;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_1,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_1,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_1,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    4 => {
                                        let mut arg_2: libc::c_uint = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_ushort as libc::c_uint;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_2,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_2,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_2,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    5 => {
                                        let mut arg_3: libc::c_int = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_3,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_3,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_3,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    6 => {
                                        let mut arg_4: libc::c_uint = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_4,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_4,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_4,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    7 => {
                                        let mut arg_5: libc::c_long = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_longint;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_5,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_5,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_5,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    8 => {
                                        let mut arg_6: libc::c_ulong = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_ulongint;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_6,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_6,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_6,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    9 => {
                                        let mut arg_7: libc::c_longlong = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_longlongint;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_7,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_7,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_7,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    10 => {
                                        let mut arg_8: libc::c_ulonglong = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_ulonglongint;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_8,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_8,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_8,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    11 => {
                                        let mut arg_9: int8_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int8_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_9 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_9 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_9 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    12 => {
                                        let mut arg_10: uint8_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint8_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_10 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_10 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_10 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    13 => {
                                        let mut arg_11: int16_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int16_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_11 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_11 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_11 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    14 => {
                                        let mut arg_12: uint16_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint16_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_12 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_12 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_12 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    15 => {
                                        let mut arg_13: int32_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int32_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_13,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_13,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_13,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    16 => {
                                        let mut arg_14: uint32_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint32_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_14,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_14,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_14,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    17 => {
                                        let mut arg_15: int64_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int64_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_15,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_15,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_15,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    18 => {
                                        let mut arg_16: uint64_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint64_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_16,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_16,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_16,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    19 => {
                                        let mut arg_17: int_fast8_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int_fast8_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_17 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_17 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_17 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    20 => {
                                        let mut arg_18: uint_fast8_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint_fast8_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_18 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_18 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_18 as libc::c_int,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    21 => {
                                        let mut arg_19: int_fast16_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int_fast16_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_19,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_19,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_19,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    22 => {
                                        let mut arg_20: uint_fast16_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint_fast16_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_20,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_20,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_20,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    23 => {
                                        let mut arg_21: int_fast32_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int_fast32_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_21,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_21,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_21,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    24 => {
                                        let mut arg_22: uint_fast32_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint_fast32_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_22,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_22,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_22,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    25 => {
                                        let mut arg_23: int_fast64_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_int_fast64_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_23,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_23,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_23,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    26 => {
                                        let mut arg_24: uint_fast64_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_uint_fast64_t;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_24,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_24,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_24,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    27 => {
                                        let mut arg_25: libc::c_double = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_double;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_25,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_25,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_25,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    28 => {
                                        let mut arg_26: f128::f128 = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_longdouble;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_26,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_26,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_26,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    29 => {
                                        let mut arg_27: libc::c_int = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_char;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_27,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_27,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_27,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    30 => {
                                        let mut arg_28: wint_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_wide_char;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_28,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_28,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_28,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    31 => {
                                        let mut arg_29: *const libc::c_char = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_string;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_29,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_29,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_29,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    32 => {
                                        let mut arg_30: *const wchar_t = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_wide_string;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_30,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_30,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_30,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    33 => {
                                        let mut arg_31: *mut libc::c_void = (*(a.arg)
                                            .offset((*dp).arg_index as isize))
                                            .a
                                            .a_pointer;
                                        match prefix_count {
                                            0 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    arg_31,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            1 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    arg_31,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            2 => {
                                                retcount = snprintf(
                                                    result.offset(length as isize),
                                                    maxlen,
                                                    buf,
                                                    prefixes[0 as libc::c_int as usize],
                                                    prefixes[1 as libc::c_int as usize],
                                                    arg_31,
                                                    &mut count as *mut libc::c_int,
                                                );
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    }
                                    _ => {
                                        abort();
                                    }
                                }
                                if count >= 0 as libc::c_int {
                                    if (count as libc::c_uint as libc::c_ulong) < maxlen
                                        && *result.offset(length as isize).offset(count as isize)
                                            as libc::c_int != '\0' as i32
                                    {
                                        abort();
                                    }
                                    if retcount > count {
                                        count = retcount;
                                    }
                                } else if *fbp.offset(1 as libc::c_int as isize)
                                    as libc::c_int != '\0' as i32
                                {
                                    *fbp
                                        .offset(
                                            1 as libc::c_int as isize,
                                        ) = '\0' as i32 as libc::c_char;
                                    continue;
                                } else if !(retcount < 0 as libc::c_int) {
                                    count = retcount;
                                }
                                if count < 0 as libc::c_int {
                                    if *__errno_location() == 0 as libc::c_int {
                                        if (*dp).conversion as libc::c_int == 'c' as i32
                                            || (*dp).conversion as libc::c_int == 's' as i32
                                        {
                                            *__errno_location() = 84 as libc::c_int;
                                        } else {
                                            *__errno_location() = 22 as libc::c_int;
                                        }
                                    }
                                    current_block = 15807818182995622322;
                                    break 's_92;
                                } else if (count as libc::c_uint)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong >= maxlen
                                {
                                    if maxlen
                                        == (2147483647 as libc::c_int as libc::c_ulong)
                                            .wrapping_div(
                                                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                    .wrapping_div(
                                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                    ),
                                            )
                                    {
                                        current_block = 17240948456513576502;
                                        break 's_92;
                                    }
                                    let mut n_2: size_t = xmax(
                                        xsum(
                                            length,
                                            ((count as libc::c_uint)
                                                .wrapping_add(2 as libc::c_int as libc::c_uint)
                                                as libc::c_ulong)
                                                .wrapping_add(
                                                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                        .wrapping_div(
                                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                        ),
                                                )
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_div(
                                                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                                        .wrapping_div(
                                                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                        ),
                                                ),
                                        ),
                                        if allocated
                                            <= (18446744073709551615 as libc::c_ulong)
                                                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                        {
                                            allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        } else {
                                            18446744073709551615 as libc::c_ulong
                                        },
                                    );
                                    if !(n_2 > allocated) {
                                        continue;
                                    }
                                    let mut memory_size_2: size_t = 0;
                                    let mut memory_2: *mut libc::c_char = 0
                                        as *mut libc::c_char;
                                    allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                                    {
                                        if allocated
                                            <= (18446744073709551615 as libc::c_ulong)
                                                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                        {
                                            allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        } else {
                                            18446744073709551615 as libc::c_ulong
                                        }
                                    } else {
                                        12 as libc::c_int as libc::c_ulong
                                    };
                                    if n_2 > allocated {
                                        allocated = n_2;
                                    }
                                    memory_size_2 = if allocated
                                        <= (18446744073709551615 as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    {
                                        allocated
                                            .wrapping_mul(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            )
                                    } else {
                                        18446744073709551615 as libc::c_ulong
                                    };
                                    if memory_size_2 == 18446744073709551615 as libc::c_ulong {
                                        current_block = 14593156655374830084;
                                        break 's_92;
                                    }
                                    if result == resultbuf {
                                        memory_2 = malloc(memory_size_2) as *mut libc::c_char;
                                    } else {
                                        memory_2 = realloc(
                                            result as *mut libc::c_void,
                                            memory_size_2,
                                        ) as *mut libc::c_char;
                                    }
                                    if memory_2.is_null() {
                                        current_block = 14593156655374830084;
                                        break 's_92;
                                    }
                                    if result == resultbuf
                                        && length > 0 as libc::c_int as libc::c_ulong
                                    {
                                        memcpy(
                                            memory_2 as *mut libc::c_void,
                                            result as *const libc::c_void,
                                            length,
                                        );
                                    }
                                    result = memory_2;
                                } else {
                                    length = (length as libc::c_ulong)
                                        .wrapping_add(count as libc::c_ulong) as size_t as size_t;
                                    break;
                                }
                            }
                            *__errno_location() = orig_errno;
                        }
                    }
                    cp = (*dp).dir_end;
                    i = i.wrapping_add(1);
                    i;
                    dp = dp.offset(1);
                    dp;
                }
                match current_block {
                    10060946412984486177 => {
                        if xsum(length, 1 as libc::c_int as size_t) > allocated {
                            let mut memory_size_3: size_t = 0;
                            let mut memory_3: *mut libc::c_char = 0 as *mut libc::c_char;
                            allocated = if allocated > 0 as libc::c_int as libc::c_ulong
                            {
                                if allocated
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                {
                                    allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                }
                            } else {
                                12 as libc::c_int as libc::c_ulong
                            };
                            if xsum(length, 1 as libc::c_int as size_t) > allocated {
                                allocated = xsum(length, 1 as libc::c_int as size_t);
                            }
                            memory_size_3 = if allocated
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            {
                                allocated
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                    )
                            } else {
                                18446744073709551615 as libc::c_ulong
                            };
                            if memory_size_3 == 18446744073709551615 as libc::c_ulong {
                                current_block = 14593156655374830084;
                            } else {
                                if result == resultbuf {
                                    memory_3 = malloc(memory_size_3) as *mut libc::c_char;
                                } else {
                                    memory_3 = realloc(
                                        result as *mut libc::c_void,
                                        memory_size_3,
                                    ) as *mut libc::c_char;
                                }
                                if memory_3.is_null() {
                                    current_block = 14593156655374830084;
                                } else {
                                    if result == resultbuf
                                        && length > 0 as libc::c_int as libc::c_ulong
                                    {
                                        memcpy(
                                            memory_3 as *mut libc::c_void,
                                            result as *const libc::c_void,
                                            length,
                                        );
                                    }
                                    result = memory_3;
                                    current_block = 7558485100648599841;
                                }
                            }
                        } else {
                            current_block = 7558485100648599841;
                        }
                        match current_block {
                            14593156655374830084 => {}
                            _ => {
                                *result
                                    .offset(length as isize) = '\0' as i32 as libc::c_char;
                                if result != resultbuf
                                    && length.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        < allocated
                                {
                                    let mut memory_4: *mut libc::c_char = 0
                                        as *mut libc::c_char;
                                    memory_4 = realloc(
                                        result as *mut libc::c_void,
                                        length
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                            ),
                                    ) as *mut libc::c_char;
                                    if !memory_4.is_null() {
                                        result = memory_4;
                                    }
                                }
                                if !buf_malloced.is_null() {
                                    rpl_free(buf_malloced as *mut libc::c_void);
                                }
                                if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                                    rpl_free(d.dir as *mut libc::c_void);
                                }
                                if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                                    rpl_free(a.arg as *mut libc::c_void);
                                }
                                *lengthp = length;
                                return result;
                            }
                        }
                    }
                    17240948456513576502 => {
                        *__errno_location() = 75 as libc::c_int;
                        current_block = 15807818182995622322;
                    }
                    _ => {}
                }
                match current_block {
                    14593156655374830084 => {
                        *__errno_location() = 12 as libc::c_int;
                    }
                    _ => {}
                }
                if result != resultbuf {
                    rpl_free(result as *mut libc::c_void);
                }
                if !buf_malloced.is_null() {
                    rpl_free(buf_malloced as *mut libc::c_void);
                }
                if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
                    rpl_free(d.dir as *mut libc::c_void);
                }
                if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
                    rpl_free(a.arg as *mut libc::c_void);
                }
                return 0 as *mut libc::c_char;
            }
        }
    }
    if d.dir != (d.direct_alloc_dir).as_mut_ptr() {
        rpl_free(d.dir as *mut libc::c_void);
    }
    if a.arg != (a.direct_alloc_arg).as_mut_ptr() {
        rpl_free(a.arg as *mut libc::c_void);
    }
    return 0 as *mut libc::c_char;
}
