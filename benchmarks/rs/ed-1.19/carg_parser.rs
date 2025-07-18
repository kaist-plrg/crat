use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type ap_Has_arg = libc::c_uint;
pub const ap_maybe: ap_Has_arg = 2;
pub const ap_yes: ap_Has_arg = 1;
pub const ap_no: ap_Has_arg = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ap_Option {
    pub code: libc::c_int,
    pub long_name: *const libc::c_char,
    pub has_arg: ap_Has_arg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ap_Record {
    pub code: libc::c_int,
    pub parsed_name: *mut libc::c_char,
    pub argument: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_parser {
    pub data: *mut ap_Record,
    pub error: *mut libc::c_char,
    pub data_size: libc::c_int,
    pub error_size: libc::c_int,
}
unsafe extern "C" fn ap_resize_buffer(
    mut buf: *mut libc::c_void,
    min_size: libc::c_int,
) -> *mut libc::c_void {
    if !buf.is_null() {
        buf = realloc(buf, min_size as libc::c_ulong);
    } else {
        buf = malloc(min_size as libc::c_ulong);
    }
    return buf;
}
unsafe extern "C" fn push_back_record(
    ap: *mut Arg_parser,
    code: libc::c_int,
    long_name: *const libc::c_char,
    argument: *const libc::c_char,
) -> libc::c_char {
    let mut p: *mut ap_Record = 0 as *mut ap_Record;
    let mut tmp: *mut libc::c_void = ap_resize_buffer(
        (*ap).data as *mut libc::c_void,
        (((*ap).data_size + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<ap_Record>() as libc::c_ulong)
            as libc::c_int,
    );
    if tmp.is_null() {
        return 0 as libc::c_int as libc::c_char;
    }
    (*ap).data = tmp as *mut ap_Record;
    p = &mut *((*ap).data).offset((*ap).data_size as isize) as *mut ap_Record;
    (*p).code = code;
    if !long_name.is_null() {
        let len: libc::c_int = strlen(long_name) as libc::c_int;
        (*p)
            .parsed_name = malloc(
            (len + 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        if ((*p).parsed_name).is_null() {
            return 0 as libc::c_int as libc::c_char;
        }
        let ref mut fresh0 = *((*p).parsed_name).offset(1 as libc::c_int as isize);
        *fresh0 = '-' as i32 as libc::c_char;
        *((*p).parsed_name).offset(0 as libc::c_int as isize) = *fresh0;
        strncpy(
            ((*p).parsed_name).offset(2 as libc::c_int as isize),
            long_name,
            (len + 1 as libc::c_int) as libc::c_ulong,
        );
    } else if code > 0 as libc::c_int && code < 256 as libc::c_int {
        (*p)
            .parsed_name = malloc((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        if ((*p).parsed_name).is_null() {
            return 0 as libc::c_int as libc::c_char;
        }
        *((*p).parsed_name)
            .offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
        *((*p).parsed_name).offset(1 as libc::c_int as isize) = code as libc::c_char;
        *((*p).parsed_name)
            .offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        (*p).parsed_name = 0 as *mut libc::c_char;
    }
    if !argument.is_null() {
        let len_0: libc::c_int = strlen(argument) as libc::c_int;
        (*p)
            .argument = malloc((len_0 + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        if ((*p).argument).is_null() {
            free((*p).parsed_name as *mut libc::c_void);
            return 0 as libc::c_int as libc::c_char;
        }
        strncpy((*p).argument, argument, (len_0 + 1 as libc::c_int) as libc::c_ulong);
    } else {
        (*p).argument = 0 as *mut libc::c_char;
    }
    (*ap).data_size += 1;
    (*ap).data_size;
    return 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn add_error(
    ap: *mut Arg_parser,
    msg: *const libc::c_char,
) -> libc::c_char {
    let len: libc::c_int = strlen(msg) as libc::c_int;
    let mut tmp: *mut libc::c_void = ap_resize_buffer(
        (*ap).error as *mut libc::c_void,
        (*ap).error_size + len + 1 as libc::c_int,
    );
    if tmp.is_null() {
        return 0 as libc::c_int as libc::c_char;
    }
    (*ap).error = tmp as *mut libc::c_char;
    strncpy(
        ((*ap).error).offset((*ap).error_size as isize),
        msg,
        (len + 1 as libc::c_int) as libc::c_ulong,
    );
    (*ap).error_size += len;
    return 1 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn free_data(ap: *mut Arg_parser) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ap).data_size {
        free((*((*ap).data).offset(i as isize)).argument as *mut libc::c_void);
        free((*((*ap).data).offset(i as isize)).parsed_name as *mut libc::c_void);
        i += 1;
        i;
    }
    if !((*ap).data).is_null() {
        free((*ap).data as *mut libc::c_void);
        (*ap).data = 0 as *mut ap_Record;
    }
    (*ap).data_size = 0 as libc::c_int;
}
unsafe extern "C" fn parse_long_option(
    ap: *mut Arg_parser,
    opt: *const libc::c_char,
    arg: *const libc::c_char,
    mut options: *const ap_Option,
    argindp: *mut libc::c_int,
) -> libc::c_char {
    let mut len: libc::c_uint = 0;
    let mut index: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut exact: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut ambig: libc::c_char = 0 as libc::c_int as libc::c_char;
    len = 0 as libc::c_int as libc::c_uint;
    while *opt.offset(len.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
        as libc::c_int != 0
        && *opt.offset(len.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int != '=' as i32
    {
        len = len.wrapping_add(1);
        len;
    }
    i = 0 as libc::c_int;
    while (*options.offset(i as isize)).code != 0 as libc::c_int {
        if !((*options.offset(i as isize)).long_name).is_null()
            && strncmp(
                (*options.offset(i as isize)).long_name,
                &*opt.offset(2 as libc::c_int as isize),
                len as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            if strlen((*options.offset(i as isize)).long_name) == len as libc::c_ulong {
                index = i;
                exact = 1 as libc::c_int as libc::c_char;
                break;
            } else if index < 0 as libc::c_int {
                index = i;
            } else if (*options.offset(index as isize)).code
                != (*options.offset(i as isize)).code
                || (*options.offset(index as isize)).has_arg as libc::c_uint
                    != (*options.offset(i as isize)).has_arg as libc::c_uint
            {
                ambig = 1 as libc::c_int as libc::c_char;
            }
        }
        i += 1;
        i;
    }
    if ambig as libc::c_int != 0 && exact == 0 {
        add_error(ap, b"option '\0" as *const u8 as *const libc::c_char);
        add_error(ap, opt);
        add_error(ap, b"' is ambiguous\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int as libc::c_char;
    }
    if index < 0 as libc::c_int {
        add_error(ap, b"unrecognized option '\0" as *const u8 as *const libc::c_char);
        add_error(ap, opt);
        add_error(ap, b"'\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int as libc::c_char;
    }
    *argindp += 1;
    *argindp;
    if *opt.offset(len.wrapping_add(2 as libc::c_int as libc::c_uint) as isize) != 0 {
        if (*options.offset(index as isize)).has_arg as libc::c_uint
            == ap_no as libc::c_int as libc::c_uint
        {
            add_error(ap, b"option '--\0" as *const u8 as *const libc::c_char);
            add_error(ap, (*options.offset(index as isize)).long_name);
            add_error(
                ap,
                b"' doesn't allow an argument\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int as libc::c_char;
        }
        if (*options.offset(index as isize)).has_arg as libc::c_uint
            == ap_yes as libc::c_int as libc::c_uint
            && *opt.offset(len.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
                == 0
        {
            add_error(ap, b"option '--\0" as *const u8 as *const libc::c_char);
            add_error(ap, (*options.offset(index as isize)).long_name);
            add_error(
                ap,
                b"' requires an argument\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int as libc::c_char;
        }
        return push_back_record(
            ap,
            (*options.offset(index as isize)).code,
            (*options.offset(index as isize)).long_name,
            &*opt.offset(len.wrapping_add(3 as libc::c_int as libc::c_uint) as isize),
        );
    }
    if (*options.offset(index as isize)).has_arg as libc::c_uint
        == ap_yes as libc::c_int as libc::c_uint
    {
        if arg.is_null() || *arg.offset(0 as libc::c_int as isize) == 0 {
            add_error(ap, b"option '--\0" as *const u8 as *const libc::c_char);
            add_error(ap, (*options.offset(index as isize)).long_name);
            add_error(
                ap,
                b"' requires an argument\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int as libc::c_char;
        }
        *argindp += 1;
        *argindp;
        return push_back_record(
            ap,
            (*options.offset(index as isize)).code,
            (*options.offset(index as isize)).long_name,
            arg,
        );
    }
    return push_back_record(
        ap,
        (*options.offset(index as isize)).code,
        (*options.offset(index as isize)).long_name,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn parse_short_option(
    ap: *mut Arg_parser,
    opt: *const libc::c_char,
    arg: *const libc::c_char,
    mut options: *const ap_Option,
    argindp: *mut libc::c_int,
) -> libc::c_char {
    let mut cind: libc::c_int = 1 as libc::c_int;
    while cind > 0 as libc::c_int {
        let mut index: libc::c_int = -(1 as libc::c_int);
        let mut i: libc::c_int = 0;
        let c: libc::c_uchar = *opt.offset(cind as isize) as libc::c_uchar;
        let mut code_str: [libc::c_char; 2] = [0; 2];
        code_str[0 as libc::c_int as usize] = c as libc::c_char;
        code_str[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        if c as libc::c_int != 0 as libc::c_int {
            i = 0 as libc::c_int;
            while (*options.offset(i as isize)).code != 0 {
                if c as libc::c_int == (*options.offset(i as isize)).code {
                    index = i;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        if index < 0 as libc::c_int {
            add_error(ap, b"invalid option -- '\0" as *const u8 as *const libc::c_char);
            add_error(ap, code_str.as_mut_ptr());
            add_error(ap, b"'\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int as libc::c_char;
        }
        cind += 1;
        if *opt.offset(cind as isize) as libc::c_int == 0 as libc::c_int {
            *argindp += 1;
            *argindp;
            cind = 0 as libc::c_int;
        }
        if (*options.offset(index as isize)).has_arg as libc::c_uint
            != ap_no as libc::c_int as libc::c_uint && cind > 0 as libc::c_int
            && *opt.offset(cind as isize) as libc::c_int != 0
        {
            if push_back_record(
                ap,
                c as libc::c_int,
                0 as *const libc::c_char,
                &*opt.offset(cind as isize),
            ) == 0
            {
                return 0 as libc::c_int as libc::c_char;
            }
            *argindp += 1;
            *argindp;
            cind = 0 as libc::c_int;
        } else if (*options.offset(index as isize)).has_arg as libc::c_uint
            == ap_yes as libc::c_int as libc::c_uint
        {
            if arg.is_null() || *arg.offset(0 as libc::c_int as isize) == 0 {
                add_error(
                    ap,
                    b"option requires an argument -- '\0" as *const u8
                        as *const libc::c_char,
                );
                add_error(ap, code_str.as_mut_ptr());
                add_error(ap, b"'\0" as *const u8 as *const libc::c_char);
                return 1 as libc::c_int as libc::c_char;
            }
            *argindp += 1;
            *argindp;
            cind = 0 as libc::c_int;
            if push_back_record(ap, c as libc::c_int, 0 as *const libc::c_char, arg) == 0
            {
                return 0 as libc::c_int as libc::c_char;
            }
        } else if push_back_record(
            ap,
            c as libc::c_int,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
        ) == 0
        {
            return 0 as libc::c_int as libc::c_char
        }
    }
    return 1 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn ap_init(
    ap: *mut Arg_parser,
    argc: libc::c_int,
    mut argv: *const *const libc::c_char,
    mut options: *const ap_Option,
    in_order: libc::c_char,
) -> libc::c_char {
    let mut current_block: u64;
    let mut non_options: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut non_options_size: libc::c_int = 0 as libc::c_int;
    let mut argind: libc::c_int = 1 as libc::c_int;
    let mut done: libc::c_char = 0 as libc::c_int as libc::c_char;
    (*ap).data = 0 as *mut ap_Record;
    (*ap).error = 0 as *mut libc::c_char;
    (*ap).data_size = 0 as libc::c_int;
    (*ap).error_size = 0 as libc::c_int;
    if argc < 2 as libc::c_int || argv.is_null() || options.is_null() {
        return 1 as libc::c_int as libc::c_char;
    }
    loop {
        if !(argind < argc) {
            current_block = 5634871135123216486;
            break;
        }
        let ch1: libc::c_uchar = *(*argv.offset(argind as isize))
            .offset(0 as libc::c_int as isize) as libc::c_uchar;
        let ch2: libc::c_uchar = (if ch1 as libc::c_int != 0 {
            *(*argv.offset(argind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uchar;
        if ch1 as libc::c_int == '-' as i32 && ch2 as libc::c_int != 0 {
            let opt: *const libc::c_char = *argv.offset(argind as isize);
            let arg: *const libc::c_char = if (argind + 1 as libc::c_int) < argc {
                *argv.offset((argind + 1 as libc::c_int) as isize)
            } else {
                0 as *const libc::c_char
            };
            if ch2 as libc::c_int == '-' as i32 {
                if *(*argv.offset(argind as isize)).offset(2 as libc::c_int as isize)
                    == 0
                {
                    argind += 1;
                    argind;
                    current_block = 5634871135123216486;
                    break;
                } else if parse_long_option(ap, opt, arg, options, &mut argind) == 0 {
                    current_block = 3799277752955337734;
                    break;
                }
            } else if parse_short_option(ap, opt, arg, options, &mut argind) == 0 {
                current_block = 3799277752955337734;
                break;
            }
            if !((*ap).error).is_null() {
                current_block = 5634871135123216486;
                break;
            }
        } else if in_order != 0 {
            let fresh1 = argind;
            argind = argind + 1;
            if push_back_record(
                ap,
                0 as libc::c_int,
                0 as *const libc::c_char,
                *argv.offset(fresh1 as isize),
            ) == 0
            {
                current_block = 3799277752955337734;
                break;
            }
        } else {
            let mut tmp: *mut libc::c_void = ap_resize_buffer(
                non_options as *mut libc::c_void,
                ((non_options_size + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ) as libc::c_int,
            );
            if tmp.is_null() {
                current_block = 3799277752955337734;
                break;
            }
            non_options = tmp as *mut *const libc::c_char;
            let fresh2 = argind;
            argind = argind + 1;
            let fresh3 = non_options_size;
            non_options_size = non_options_size + 1;
            let ref mut fresh4 = *non_options.offset(fresh3 as isize);
            *fresh4 = *argv.offset(fresh2 as isize);
        }
    }
    match current_block {
        5634871135123216486 => {
            if !((*ap).error).is_null() {
                free_data(ap);
                current_block = 11459959175219260272;
            } else {
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                loop {
                    if !(i < non_options_size) {
                        current_block = 14648156034262866959;
                        break;
                    }
                    if push_back_record(
                        ap,
                        0 as libc::c_int,
                        0 as *const libc::c_char,
                        *non_options.offset(i as isize),
                    ) == 0
                    {
                        current_block = 3799277752955337734;
                        break;
                    }
                    i += 1;
                    i;
                }
                match current_block {
                    3799277752955337734 => {}
                    _ => {
                        loop {
                            if !(argind < argc) {
                                current_block = 11459959175219260272;
                                break;
                            }
                            let fresh5 = argind;
                            argind = argind + 1;
                            if push_back_record(
                                ap,
                                0 as libc::c_int,
                                0 as *const libc::c_char,
                                *argv.offset(fresh5 as isize),
                            ) == 0
                            {
                                current_block = 3799277752955337734;
                                break;
                            }
                        }
                    }
                }
            }
            match current_block {
                3799277752955337734 => {}
                _ => {
                    done = 1 as libc::c_int as libc::c_char;
                }
            }
        }
        _ => {}
    }
    if !non_options.is_null() {
        free(non_options as *mut libc::c_void);
    }
    return done;
}
pub unsafe extern "C" fn ap_free(ap: *mut Arg_parser) {
    free_data(ap);
    if !((*ap).error).is_null() {
        free((*ap).error as *mut libc::c_void);
        (*ap).error = 0 as *mut libc::c_char;
    }
    (*ap).error_size = 0 as libc::c_int;
}
pub unsafe extern "C" fn ap_error(ap: *const Arg_parser) -> *const libc::c_char {
    return (*ap).error;
}
pub unsafe extern "C" fn ap_arguments(ap: *const Arg_parser) -> libc::c_int {
    return (*ap).data_size;
}
pub unsafe extern "C" fn ap_code(ap: *const Arg_parser, i: libc::c_int) -> libc::c_int {
    if i < 0 as libc::c_int || i >= ap_arguments(ap) {
        return 0 as libc::c_int;
    }
    return (*((*ap).data).offset(i as isize)).code;
}
pub unsafe extern "C" fn ap_parsed_name(
    ap: *const Arg_parser,
    i: libc::c_int,
) -> *const libc::c_char {
    if i < 0 as libc::c_int || i >= ap_arguments(ap)
        || ((*((*ap).data).offset(i as isize)).parsed_name).is_null()
    {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    return (*((*ap).data).offset(i as isize)).parsed_name;
}
pub unsafe extern "C" fn ap_argument(
    ap: *const Arg_parser,
    i: libc::c_int,
) -> *const libc::c_char {
    if i < 0 as libc::c_int || i >= ap_arguments(ap)
        || ((*((*ap).data).offset(i as isize)).argument).is_null()
    {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    return (*((*ap).data).offset(i as isize)).argument;
}
