use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
pub type arg_type = libc::c_uint;
pub const TYPE_COUNT_LONGLONGINT_POINTER: arg_type = 22;
pub const TYPE_COUNT_LONGINT_POINTER: arg_type = 21;
pub const TYPE_COUNT_INT_POINTER: arg_type = 20;
pub const TYPE_COUNT_SHORT_POINTER: arg_type = 19;
pub const TYPE_COUNT_SCHAR_POINTER: arg_type = 18;
pub const TYPE_POINTER: arg_type = 17;
pub const TYPE_WIDE_STRING: arg_type = 16;
pub const TYPE_STRING: arg_type = 15;
pub const TYPE_WIDE_CHAR: arg_type = 14;
pub const TYPE_CHAR: arg_type = 13;
pub const TYPE_LONGDOUBLE: arg_type = 12;
pub const TYPE_DOUBLE: arg_type = 11;
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
}
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
pub type intmax_t = __intmax_t;
pub type __intmax_t = libc::c_long;
#[inline]
unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as libc::c_ulong };
}
pub unsafe extern "C" fn printf_parse(
    mut format: *const libc::c_char,
    mut d: *mut char_directives,
    mut a: *mut arguments,
) -> libc::c_int {
    let mut current_block: u64;
    let mut cp: *const libc::c_char = format;
    let mut arg_posn: size_t = 0 as libc::c_int as size_t;
    let mut d_allocated: size_t = 0;
    let mut a_allocated: size_t = 0;
    let mut max_width_length: size_t = 0 as libc::c_int as size_t;
    let mut max_precision_length: size_t = 0 as libc::c_int as size_t;
    (*d).count = 0 as libc::c_int as size_t;
    d_allocated = 7 as libc::c_int as size_t;
    (*d).dir = ((*d).direct_alloc_dir).as_mut_ptr();
    (*a).count = 0 as libc::c_int as size_t;
    a_allocated = 7 as libc::c_int as size_t;
    (*a).arg = ((*a).direct_alloc_arg).as_mut_ptr();
    loop {
        if !(*cp as libc::c_int != '\0' as i32) {
            current_block = 7942882540430375978;
            break;
        }
        let fresh0 = cp;
        cp = cp.offset(1);
        let mut c: libc::c_char = *fresh0;
        if !(c as libc::c_int == '%' as i32) {
            continue;
        }
        let mut arg_index: size_t = !(0 as libc::c_int as size_t);
        let mut dp: *mut char_directive = &mut *((*d).dir).offset((*d).count as isize)
            as *mut char_directive;
        (*dp).dir_start = cp.offset(-(1 as libc::c_int as isize));
        (*dp).flags = 0 as libc::c_int;
        (*dp).width_start = 0 as *const libc::c_char;
        (*dp).width_end = 0 as *const libc::c_char;
        (*dp).width_arg_index = !(0 as libc::c_int as size_t);
        (*dp).precision_start = 0 as *const libc::c_char;
        (*dp).precision_end = 0 as *const libc::c_char;
        (*dp).precision_arg_index = !(0 as libc::c_int as size_t);
        (*dp).arg_index = !(0 as libc::c_int as size_t);
        if *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
            let mut np: *const libc::c_char = 0 as *const libc::c_char;
            np = cp;
            while *np as libc::c_int >= '0' as i32 && *np as libc::c_int <= '9' as i32 {
                np = np.offset(1);
                np;
            }
            if *np as libc::c_int == '$' as i32 {
                let mut n: size_t = 0 as libc::c_int as size_t;
                np = cp;
                while *np as libc::c_int >= '0' as i32
                    && *np as libc::c_int <= '9' as i32
                {
                    n = xsum(
                        if n
                            <= (18446744073709551615 as libc::c_ulong)
                                .wrapping_div(10 as libc::c_int as libc::c_ulong)
                        {
                            n.wrapping_mul(10 as libc::c_int as libc::c_ulong)
                        } else {
                            18446744073709551615 as libc::c_ulong
                        },
                        (*np as libc::c_int - '0' as i32) as size_t,
                    );
                    np = np.offset(1);
                    np;
                }
                if n == 0 as libc::c_int as libc::c_ulong {
                    current_block = 5476587429079166228;
                    break;
                } else if n == 18446744073709551615 as libc::c_ulong {
                    current_block = 5476587429079166228;
                    break;
                } else {
                    arg_index = n.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    cp = np.offset(1 as libc::c_int as isize);
                }
            }
        }
        loop {
            if *cp as libc::c_int == '\'' as i32 {
                (*dp).flags |= 1 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == '-' as i32 {
                (*dp).flags |= 2 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == '+' as i32 {
                (*dp).flags |= 4 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == ' ' as i32 {
                (*dp).flags |= 8 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == '#' as i32 {
                (*dp).flags |= 16 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == '0' as i32 {
                (*dp).flags |= 32 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else {
                if !(*cp as libc::c_int == 'I' as i32) {
                    break;
                }
                (*dp).flags |= 64 as libc::c_int;
                cp = cp.offset(1);
                cp;
            }
        }
        if *cp as libc::c_int == '*' as i32 {
            (*dp).width_start = cp;
            cp = cp.offset(1);
            cp;
            (*dp).width_end = cp;
            if max_width_length < 1 as libc::c_int as libc::c_ulong {
                max_width_length = 1 as libc::c_int as size_t;
            }
            if *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
                let mut np_0: *const libc::c_char = 0 as *const libc::c_char;
                np_0 = cp;
                while *np_0 as libc::c_int >= '0' as i32
                    && *np_0 as libc::c_int <= '9' as i32
                {
                    np_0 = np_0.offset(1);
                    np_0;
                }
                if *np_0 as libc::c_int == '$' as i32 {
                    let mut n_0: size_t = 0 as libc::c_int as size_t;
                    np_0 = cp;
                    while *np_0 as libc::c_int >= '0' as i32
                        && *np_0 as libc::c_int <= '9' as i32
                    {
                        n_0 = xsum(
                            if n_0
                                <= (18446744073709551615 as libc::c_ulong)
                                    .wrapping_div(10 as libc::c_int as libc::c_ulong)
                            {
                                n_0.wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            } else {
                                18446744073709551615 as libc::c_ulong
                            },
                            (*np_0 as libc::c_int - '0' as i32) as size_t,
                        );
                        np_0 = np_0.offset(1);
                        np_0;
                    }
                    if n_0 == 0 as libc::c_int as libc::c_ulong {
                        current_block = 5476587429079166228;
                        break;
                    } else if n_0 == 18446744073709551615 as libc::c_ulong {
                        current_block = 5476587429079166228;
                        break;
                    } else {
                        (*dp)
                            .width_arg_index = n_0
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        cp = np_0.offset(1 as libc::c_int as isize);
                    }
                }
            }
            if (*dp).width_arg_index == !(0 as libc::c_int as size_t) {
                let fresh1 = arg_posn;
                arg_posn = arg_posn.wrapping_add(1);
                (*dp).width_arg_index = fresh1;
                if (*dp).width_arg_index == !(0 as libc::c_int as size_t) {
                    current_block = 5476587429079166228;
                    break;
                }
            }
            let mut n_1: size_t = (*dp).width_arg_index;
            if n_1 >= a_allocated {
                let mut memory_size: size_t = 0;
                let mut memory: *mut argument = 0 as *mut argument;
                a_allocated = if a_allocated
                    <= (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                {
                    a_allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                } else {
                    18446744073709551615 as libc::c_ulong
                };
                if a_allocated <= n_1 {
                    a_allocated = xsum(n_1, 1 as libc::c_int as size_t);
                }
                memory_size = if a_allocated
                    <= (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<argument>() as libc::c_ulong)
                {
                    a_allocated
                        .wrapping_mul(::std::mem::size_of::<argument>() as libc::c_ulong)
                } else {
                    18446744073709551615 as libc::c_ulong
                };
                if memory_size == 18446744073709551615 as libc::c_ulong {
                    current_block = 12866011334537285818;
                    break;
                }
                memory = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                    realloc((*a).arg as *mut libc::c_void, memory_size)
                } else {
                    malloc(memory_size)
                }) as *mut argument;
                if memory.is_null() {
                    current_block = 12866011334537285818;
                    break;
                }
                if (*a).arg == ((*a).direct_alloc_arg).as_mut_ptr() {
                    memcpy(
                        memory as *mut libc::c_void,
                        (*a).arg as *const libc::c_void,
                        ((*a).count)
                            .wrapping_mul(
                                ::std::mem::size_of::<argument>() as libc::c_ulong,
                            ),
                    );
                }
                (*a).arg = memory;
            }
            while (*a).count <= n_1 {
                let fresh2 = (*a).count;
                (*a).count = ((*a).count).wrapping_add(1);
                (*((*a).arg).offset(fresh2 as isize)).type_0 = TYPE_NONE;
            }
            if (*((*a).arg).offset(n_1 as isize)).type_0 as libc::c_uint
                == TYPE_NONE as libc::c_int as libc::c_uint
            {
                (*((*a).arg).offset(n_1 as isize)).type_0 = TYPE_INT;
            } else if (*((*a).arg).offset(n_1 as isize)).type_0 as libc::c_uint
                != TYPE_INT as libc::c_int as libc::c_uint
            {
                current_block = 5476587429079166228;
                break;
            }
        } else if *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
            let mut width_length: size_t = 0;
            (*dp).width_start = cp;
            while *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
                cp = cp.offset(1);
                cp;
            }
            (*dp).width_end = cp;
            width_length = ((*dp).width_end).offset_from((*dp).width_start)
                as libc::c_long as size_t;
            if max_width_length < width_length {
                max_width_length = width_length;
            }
        }
        if *cp as libc::c_int == '.' as i32 {
            cp = cp.offset(1);
            cp;
            if *cp as libc::c_int == '*' as i32 {
                (*dp).precision_start = cp.offset(-(1 as libc::c_int as isize));
                cp = cp.offset(1);
                cp;
                (*dp).precision_end = cp;
                if max_precision_length < 2 as libc::c_int as libc::c_ulong {
                    max_precision_length = 2 as libc::c_int as size_t;
                }
                if *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
                    let mut np_1: *const libc::c_char = 0 as *const libc::c_char;
                    np_1 = cp;
                    while *np_1 as libc::c_int >= '0' as i32
                        && *np_1 as libc::c_int <= '9' as i32
                    {
                        np_1 = np_1.offset(1);
                        np_1;
                    }
                    if *np_1 as libc::c_int == '$' as i32 {
                        let mut n_2: size_t = 0 as libc::c_int as size_t;
                        np_1 = cp;
                        while *np_1 as libc::c_int >= '0' as i32
                            && *np_1 as libc::c_int <= '9' as i32
                        {
                            n_2 = xsum(
                                if n_2
                                    <= (18446744073709551615 as libc::c_ulong)
                                        .wrapping_div(10 as libc::c_int as libc::c_ulong)
                                {
                                    n_2.wrapping_mul(10 as libc::c_int as libc::c_ulong)
                                } else {
                                    18446744073709551615 as libc::c_ulong
                                },
                                (*np_1 as libc::c_int - '0' as i32) as size_t,
                            );
                            np_1 = np_1.offset(1);
                            np_1;
                        }
                        if n_2 == 0 as libc::c_int as libc::c_ulong {
                            current_block = 5476587429079166228;
                            break;
                        } else if n_2 == 18446744073709551615 as libc::c_ulong {
                            current_block = 5476587429079166228;
                            break;
                        } else {
                            (*dp)
                                .precision_arg_index = n_2
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                            cp = np_1.offset(1 as libc::c_int as isize);
                        }
                    }
                }
                if (*dp).precision_arg_index == !(0 as libc::c_int as size_t) {
                    let fresh3 = arg_posn;
                    arg_posn = arg_posn.wrapping_add(1);
                    (*dp).precision_arg_index = fresh3;
                    if (*dp).precision_arg_index == !(0 as libc::c_int as size_t) {
                        current_block = 5476587429079166228;
                        break;
                    }
                }
                let mut n_3: size_t = (*dp).precision_arg_index;
                if n_3 >= a_allocated {
                    let mut memory_size_0: size_t = 0;
                    let mut memory_0: *mut argument = 0 as *mut argument;
                    a_allocated = if a_allocated
                        <= (18446744073709551615 as libc::c_ulong)
                            .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    {
                        a_allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    } else {
                        18446744073709551615 as libc::c_ulong
                    };
                    if a_allocated <= n_3 {
                        a_allocated = xsum(n_3, 1 as libc::c_int as size_t);
                    }
                    memory_size_0 = if a_allocated
                        <= (18446744073709551615 as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<argument>() as libc::c_ulong,
                            )
                    {
                        a_allocated
                            .wrapping_mul(
                                ::std::mem::size_of::<argument>() as libc::c_ulong,
                            )
                    } else {
                        18446744073709551615 as libc::c_ulong
                    };
                    if memory_size_0 == 18446744073709551615 as libc::c_ulong {
                        current_block = 12866011334537285818;
                        break;
                    }
                    memory_0 = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                        realloc((*a).arg as *mut libc::c_void, memory_size_0)
                    } else {
                        malloc(memory_size_0)
                    }) as *mut argument;
                    if memory_0.is_null() {
                        current_block = 12866011334537285818;
                        break;
                    }
                    if (*a).arg == ((*a).direct_alloc_arg).as_mut_ptr() {
                        memcpy(
                            memory_0 as *mut libc::c_void,
                            (*a).arg as *const libc::c_void,
                            ((*a).count)
                                .wrapping_mul(
                                    ::std::mem::size_of::<argument>() as libc::c_ulong,
                                ),
                        );
                    }
                    (*a).arg = memory_0;
                }
                while (*a).count <= n_3 {
                    let fresh4 = (*a).count;
                    (*a).count = ((*a).count).wrapping_add(1);
                    (*((*a).arg).offset(fresh4 as isize)).type_0 = TYPE_NONE;
                }
                if (*((*a).arg).offset(n_3 as isize)).type_0 as libc::c_uint
                    == TYPE_NONE as libc::c_int as libc::c_uint
                {
                    (*((*a).arg).offset(n_3 as isize)).type_0 = TYPE_INT;
                } else if (*((*a).arg).offset(n_3 as isize)).type_0 as libc::c_uint
                    != TYPE_INT as libc::c_int as libc::c_uint
                {
                    current_block = 5476587429079166228;
                    break;
                }
            } else {
                let mut precision_length: size_t = 0;
                (*dp).precision_start = cp.offset(-(1 as libc::c_int as isize));
                while *cp as libc::c_int >= '0' as i32
                    && *cp as libc::c_int <= '9' as i32
                {
                    cp = cp.offset(1);
                    cp;
                }
                (*dp).precision_end = cp;
                precision_length = ((*dp).precision_end)
                    .offset_from((*dp).precision_start) as libc::c_long as size_t;
                if max_precision_length < precision_length {
                    max_precision_length = precision_length;
                }
            }
        }
        let mut type_0: arg_type = TYPE_NONE;
        let mut flags: libc::c_int = 0 as libc::c_int;
        loop {
            if *cp as libc::c_int == 'h' as i32 {
                flags |= (1 as libc::c_int) << (flags & 1 as libc::c_int);
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == 'L' as i32 {
                flags |= 4 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == 'l' as i32 {
                flags += 8 as libc::c_int;
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == 'j' as i32 {
                if ::std::mem::size_of::<intmax_t>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    flags += 16 as libc::c_int;
                } else if ::std::mem::size_of::<intmax_t>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    flags += 8 as libc::c_int;
                }
                cp = cp.offset(1);
                cp;
            } else if *cp as libc::c_int == 'z' as i32
                || *cp as libc::c_int == 'Z' as i32
            {
                if ::std::mem::size_of::<size_t>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    flags += 16 as libc::c_int;
                } else if ::std::mem::size_of::<size_t>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    flags += 8 as libc::c_int;
                }
                cp = cp.offset(1);
                cp;
            } else {
                if !(*cp as libc::c_int == 't' as i32) {
                    break;
                }
                if ::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                {
                    flags += 16 as libc::c_int;
                } else if ::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong
                    > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                {
                    flags += 8 as libc::c_int;
                }
                cp = cp.offset(1);
                cp;
            }
        }
        let fresh5 = cp;
        cp = cp.offset(1);
        c = *fresh5;
        match c as libc::c_int {
            100 | 105 => {
                if flags >= 16 as libc::c_int || flags & 4 as libc::c_int != 0 {
                    type_0 = TYPE_LONGLONGINT;
                } else if flags >= 8 as libc::c_int {
                    type_0 = TYPE_LONGINT;
                } else if flags & 2 as libc::c_int != 0 {
                    type_0 = TYPE_SCHAR;
                } else if flags & 1 as libc::c_int != 0 {
                    type_0 = TYPE_SHORT;
                } else {
                    type_0 = TYPE_INT;
                }
            }
            111 | 117 | 120 | 88 => {
                if flags >= 16 as libc::c_int || flags & 4 as libc::c_int != 0 {
                    type_0 = TYPE_ULONGLONGINT;
                } else if flags >= 8 as libc::c_int {
                    type_0 = TYPE_ULONGINT;
                } else if flags & 2 as libc::c_int != 0 {
                    type_0 = TYPE_UCHAR;
                } else if flags & 1 as libc::c_int != 0 {
                    type_0 = TYPE_USHORT;
                } else {
                    type_0 = TYPE_UINT;
                }
            }
            102 | 70 | 101 | 69 | 103 | 71 | 97 | 65 => {
                if flags >= 16 as libc::c_int || flags & 4 as libc::c_int != 0 {
                    type_0 = TYPE_LONGDOUBLE;
                } else {
                    type_0 = TYPE_DOUBLE;
                }
            }
            99 => {
                if flags >= 8 as libc::c_int {
                    type_0 = TYPE_WIDE_CHAR;
                } else {
                    type_0 = TYPE_CHAR;
                }
            }
            67 => {
                type_0 = TYPE_WIDE_CHAR;
                c = 'c' as i32 as libc::c_char;
            }
            115 => {
                if flags >= 8 as libc::c_int {
                    type_0 = TYPE_WIDE_STRING;
                } else {
                    type_0 = TYPE_STRING;
                }
            }
            83 => {
                type_0 = TYPE_WIDE_STRING;
                c = 's' as i32 as libc::c_char;
            }
            112 => {
                type_0 = TYPE_POINTER;
            }
            110 => {
                if flags >= 16 as libc::c_int || flags & 4 as libc::c_int != 0 {
                    type_0 = TYPE_COUNT_LONGLONGINT_POINTER;
                } else if flags >= 8 as libc::c_int {
                    type_0 = TYPE_COUNT_LONGINT_POINTER;
                } else if flags & 2 as libc::c_int != 0 {
                    type_0 = TYPE_COUNT_SCHAR_POINTER;
                } else if flags & 1 as libc::c_int != 0 {
                    type_0 = TYPE_COUNT_SHORT_POINTER;
                } else {
                    type_0 = TYPE_COUNT_INT_POINTER;
                }
            }
            37 => {
                type_0 = TYPE_NONE;
            }
            _ => {
                current_block = 5476587429079166228;
                break;
            }
        }
        if type_0 as libc::c_uint != TYPE_NONE as libc::c_int as libc::c_uint {
            (*dp).arg_index = arg_index;
            if (*dp).arg_index == !(0 as libc::c_int as size_t) {
                let fresh6 = arg_posn;
                arg_posn = arg_posn.wrapping_add(1);
                (*dp).arg_index = fresh6;
                if (*dp).arg_index == !(0 as libc::c_int as size_t) {
                    current_block = 5476587429079166228;
                    break;
                }
            }
            let mut n_4: size_t = (*dp).arg_index;
            if n_4 >= a_allocated {
                let mut memory_size_1: size_t = 0;
                let mut memory_1: *mut argument = 0 as *mut argument;
                a_allocated = if a_allocated
                    <= (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                {
                    a_allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                } else {
                    18446744073709551615 as libc::c_ulong
                };
                if a_allocated <= n_4 {
                    a_allocated = xsum(n_4, 1 as libc::c_int as size_t);
                }
                memory_size_1 = if a_allocated
                    <= (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<argument>() as libc::c_ulong)
                {
                    a_allocated
                        .wrapping_mul(::std::mem::size_of::<argument>() as libc::c_ulong)
                } else {
                    18446744073709551615 as libc::c_ulong
                };
                if memory_size_1 == 18446744073709551615 as libc::c_ulong {
                    current_block = 12866011334537285818;
                    break;
                }
                memory_1 = (if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                    realloc((*a).arg as *mut libc::c_void, memory_size_1)
                } else {
                    malloc(memory_size_1)
                }) as *mut argument;
                if memory_1.is_null() {
                    current_block = 12866011334537285818;
                    break;
                }
                if (*a).arg == ((*a).direct_alloc_arg).as_mut_ptr() {
                    memcpy(
                        memory_1 as *mut libc::c_void,
                        (*a).arg as *const libc::c_void,
                        ((*a).count)
                            .wrapping_mul(
                                ::std::mem::size_of::<argument>() as libc::c_ulong,
                            ),
                    );
                }
                (*a).arg = memory_1;
            }
            while (*a).count <= n_4 {
                let fresh7 = (*a).count;
                (*a).count = ((*a).count).wrapping_add(1);
                (*((*a).arg).offset(fresh7 as isize)).type_0 = TYPE_NONE;
            }
            if (*((*a).arg).offset(n_4 as isize)).type_0 as libc::c_uint
                == TYPE_NONE as libc::c_int as libc::c_uint
            {
                (*((*a).arg).offset(n_4 as isize)).type_0 = type_0;
            } else if (*((*a).arg).offset(n_4 as isize)).type_0 as libc::c_uint
                != type_0 as libc::c_uint
            {
                current_block = 5476587429079166228;
                break;
            }
        }
        (*dp).conversion = c;
        (*dp).dir_end = cp;
        (*d).count = ((*d).count).wrapping_add(1);
        (*d).count;
        if !((*d).count >= d_allocated) {
            continue;
        }
        let mut memory_size_2: size_t = 0;
        let mut memory_2: *mut char_directive = 0 as *mut char_directive;
        d_allocated = if d_allocated
            <= (18446744073709551615 as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong)
        {
            d_allocated.wrapping_mul(2 as libc::c_int as libc::c_ulong)
        } else {
            18446744073709551615 as libc::c_ulong
        };
        memory_size_2 = if d_allocated
            <= (18446744073709551615 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<char_directive>() as libc::c_ulong)
        {
            d_allocated
                .wrapping_mul(::std::mem::size_of::<char_directive>() as libc::c_ulong)
        } else {
            18446744073709551615 as libc::c_ulong
        };
        if memory_size_2 == 18446744073709551615 as libc::c_ulong {
            current_block = 12866011334537285818;
            break;
        } else {
            memory_2 = (if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                realloc((*d).dir as *mut libc::c_void, memory_size_2)
            } else {
                malloc(memory_size_2)
            }) as *mut char_directive;
            if memory_2.is_null() {
                current_block = 12866011334537285818;
                break;
            }
            if (*d).dir == ((*d).direct_alloc_dir).as_mut_ptr() {
                memcpy(
                    memory_2 as *mut libc::c_void,
                    (*d).dir as *const libc::c_void,
                    ((*d).count)
                        .wrapping_mul(
                            ::std::mem::size_of::<char_directive>() as libc::c_ulong,
                        ),
                );
            }
            (*d).dir = memory_2;
        }
    }
    match current_block {
        12866011334537285818 => {
            if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                free((*a).arg as *mut libc::c_void);
            }
            if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                free((*d).dir as *mut libc::c_void);
            }
            *__errno_location() = 12 as libc::c_int;
            return -(1 as libc::c_int);
        }
        5476587429079166228 => {
            if (*a).arg != ((*a).direct_alloc_arg).as_mut_ptr() {
                free((*a).arg as *mut libc::c_void);
            }
            if (*d).dir != ((*d).direct_alloc_dir).as_mut_ptr() {
                free((*d).dir as *mut libc::c_void);
            }
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        _ => {
            let ref mut fresh8 = (*((*d).dir).offset((*d).count as isize)).dir_start;
            *fresh8 = cp;
            (*d).max_width_length = max_width_length;
            (*d).max_precision_length = max_precision_length;
            return 0 as libc::c_int;
        }
    };
}
