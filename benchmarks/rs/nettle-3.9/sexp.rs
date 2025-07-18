use ::libc;
extern "C" {
    fn abort() -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type sexp_type = libc::c_uint;
pub const SEXP_END: sexp_type = 2;
pub const SEXP_LIST: sexp_type = 1;
pub const SEXP_ATOM: sexp_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_iterator {
    pub length: size_t,
    pub buffer: *const uint8_t,
    pub start: size_t,
    pub pos: size_t,
    pub level: libc::c_uint,
    pub type_0: sexp_type,
    pub display_length: size_t,
    pub display: *const uint8_t,
    pub atom_length: size_t,
    pub atom: *const uint8_t,
}
unsafe extern "C" fn sexp_iterator_init(
    mut iterator: *mut sexp_iterator,
    mut length: libc::c_uint,
    mut input: *const uint8_t,
) {
    (*iterator).length = length as size_t;
    (*iterator).buffer = input;
    (*iterator).pos = 0 as libc::c_int as size_t;
    (*iterator).level = 0 as libc::c_int as libc::c_uint;
    (*iterator).type_0 = SEXP_END;
    (*iterator).display_length = 0 as libc::c_int as size_t;
    (*iterator).display = 0 as *const uint8_t;
    (*iterator).atom_length = 0 as libc::c_int as size_t;
    (*iterator).atom = 0 as *const uint8_t;
}
unsafe extern "C" fn sexp_iterator_simple(
    mut iterator: *mut sexp_iterator,
    mut size: *mut size_t,
    mut string: *mut *const uint8_t,
) -> libc::c_int {
    let mut length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut c: uint8_t = 0;
    if (*iterator).pos == (*iterator).length {
        return 0 as libc::c_int;
    }
    let fresh0 = (*iterator).pos;
    (*iterator).pos = ((*iterator).pos).wrapping_add(1);
    c = *((*iterator).buffer).offset(fresh0 as isize);
    if (*iterator).pos == (*iterator).length {
        return 0 as libc::c_int;
    }
    if c as libc::c_int >= '1' as i32 && c as libc::c_int <= '9' as i32 {
        loop {
            length = length
                .wrapping_mul(10 as libc::c_int as libc::c_uint)
                .wrapping_add((c as libc::c_int - '0' as i32) as libc::c_uint);
            if length as libc::c_ulong
                > ((*iterator).length).wrapping_sub((*iterator).pos)
            {
                return 0 as libc::c_int;
            }
            if (*iterator).pos == (*iterator).length {
                return 0 as libc::c_int;
            }
            let fresh1 = (*iterator).pos;
            (*iterator).pos = ((*iterator).pos).wrapping_add(1);
            c = *((*iterator).buffer).offset(fresh1 as isize);
            if !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32) {
                break;
            }
        }
    } else if c as libc::c_int == '0' as i32 {
        let fresh2 = (*iterator).pos;
        (*iterator).pos = ((*iterator).pos).wrapping_add(1);
        c = *((*iterator).buffer).offset(fresh2 as isize);
    } else {
        return 0 as libc::c_int
    }
    if c as libc::c_int != ':' as i32 {
        return 0 as libc::c_int;
    }
    *size = length as size_t;
    *string = ((*iterator).buffer).offset((*iterator).pos as isize);
    (*iterator)
        .pos = ((*iterator).pos as libc::c_ulong).wrapping_add(length as libc::c_ulong)
        as size_t as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn sexp_iterator_parse(
    mut iterator: *mut sexp_iterator,
) -> libc::c_int {
    (*iterator).start = (*iterator).pos;
    if (*iterator).pos == (*iterator).length {
        if (*iterator).level != 0 {
            return 0 as libc::c_int;
        }
        (*iterator).type_0 = SEXP_END;
        return 1 as libc::c_int;
    }
    match *((*iterator).buffer).offset((*iterator).pos as isize) as libc::c_int {
        40 => {
            (*iterator).type_0 = SEXP_LIST;
            return 1 as libc::c_int;
        }
        41 => {
            if (*iterator).level == 0 {
                return 0 as libc::c_int;
            }
            (*iterator).pos = ((*iterator).pos).wrapping_add(1);
            (*iterator).pos;
            (*iterator).type_0 = SEXP_END;
            return 1 as libc::c_int;
        }
        91 => {
            (*iterator).pos = ((*iterator).pos).wrapping_add(1);
            (*iterator).pos;
            if sexp_iterator_simple(
                iterator,
                &mut (*iterator).display_length,
                &mut (*iterator).display,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            if (*iterator).pos == (*iterator).length
                || {
                    let fresh3 = (*iterator).pos;
                    (*iterator).pos = ((*iterator).pos).wrapping_add(1);
                    *((*iterator).buffer).offset(fresh3 as isize) as libc::c_int
                        != ']' as i32
                }
            {
                return 0 as libc::c_int;
            }
        }
        _ => {
            (*iterator).display_length = 0 as libc::c_int as size_t;
            (*iterator).display = 0 as *const uint8_t;
        }
    }
    (*iterator).type_0 = SEXP_ATOM;
    return sexp_iterator_simple(
        iterator,
        &mut (*iterator).atom_length,
        &mut (*iterator).atom,
    );
}
pub unsafe extern "C" fn nettle_sexp_iterator_first(
    mut iterator: *mut sexp_iterator,
    mut length: size_t,
    mut input: *const uint8_t,
) -> libc::c_int {
    sexp_iterator_init(iterator, length as libc::c_uint, input);
    return sexp_iterator_parse(iterator);
}
pub unsafe extern "C" fn nettle_sexp_iterator_next(
    mut iterator: *mut sexp_iterator,
) -> libc::c_int {
    match (*iterator).type_0 as libc::c_uint {
        2 => return 1 as libc::c_int,
        1 => {
            return (nettle_sexp_iterator_enter_list(iterator) != 0
                && nettle_sexp_iterator_exit_list(iterator) != 0) as libc::c_int;
        }
        0 => return sexp_iterator_parse(iterator),
        _ => {}
    }
    abort();
}
pub unsafe extern "C" fn nettle_sexp_iterator_enter_list(
    mut iterator: *mut sexp_iterator,
) -> libc::c_int {
    if (*iterator).type_0 as libc::c_uint != SEXP_LIST as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*iterator).pos == (*iterator).length
        || {
            let fresh4 = (*iterator).pos;
            (*iterator).pos = ((*iterator).pos).wrapping_add(1);
            *((*iterator).buffer).offset(fresh4 as isize) as libc::c_int != '(' as i32
        }
    {
        abort();
    }
    (*iterator).level = ((*iterator).level).wrapping_add(1);
    (*iterator).level;
    return sexp_iterator_parse(iterator);
}
pub unsafe extern "C" fn nettle_sexp_iterator_exit_list(
    mut iterator: *mut sexp_iterator,
) -> libc::c_int {
    if (*iterator).level == 0 {
        return 0 as libc::c_int;
    }
    while (*iterator).type_0 as libc::c_uint != SEXP_END as libc::c_int as libc::c_uint {
        if nettle_sexp_iterator_next(iterator) == 0 {
            return 0 as libc::c_int;
        }
    }
    (*iterator).level = ((*iterator).level).wrapping_sub(1);
    (*iterator).level;
    return sexp_iterator_parse(iterator);
}
pub unsafe extern "C" fn nettle_sexp_iterator_subexpr(
    mut iterator: *mut sexp_iterator,
    mut length: *mut size_t,
) -> *const uint8_t {
    let mut start: size_t = (*iterator).start;
    if nettle_sexp_iterator_next(iterator) == 0 {
        return 0 as *const uint8_t;
    }
    *length = ((*iterator).start).wrapping_sub(start);
    return ((*iterator).buffer).offset(start as isize);
}
pub unsafe extern "C" fn nettle_sexp_iterator_get_uint32(
    mut iterator: *mut sexp_iterator,
    mut x: *mut uint32_t,
) -> libc::c_int {
    if (*iterator).type_0 as libc::c_uint == SEXP_ATOM as libc::c_int as libc::c_uint
        && ((*iterator).display).is_null() && (*iterator).atom_length != 0
        && (*((*iterator).atom).offset(0 as libc::c_int as isize) as libc::c_int)
            < 0x80 as libc::c_int
    {
        let mut length: size_t = (*iterator).atom_length;
        let mut p: *const uint8_t = (*iterator).atom;
        while length != 0 && *p == 0 {
            length = length.wrapping_sub(1);
            length;
            p = p.offset(1);
            p;
        }
        match length {
            0 => {
                *x = 0 as libc::c_int as uint32_t;
            }
            1 => {
                *x = *p.offset(0 as libc::c_int as isize) as uint32_t;
            }
            2 => {
                *x = (*p.offset(0 as libc::c_int as isize) as uint32_t)
                    << 8 as libc::c_int
                    | *p.offset(1 as libc::c_int as isize) as uint32_t;
            }
            3 => {
                *x = (*p.offset(0 as libc::c_int as isize) as uint32_t)
                    << 16 as libc::c_int
                    | (*p.offset(1 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int
                    | *p.offset(2 as libc::c_int as isize) as uint32_t;
            }
            4 => {
                *x = (*p.offset(0 as libc::c_int as isize) as uint32_t)
                    << 24 as libc::c_int
                    | (*p.offset(1 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int
                    | (*p.offset(2 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int
                    | *p.offset(3 as libc::c_int as isize) as uint32_t;
            }
            _ => return 0 as libc::c_int,
        }
        return nettle_sexp_iterator_next(iterator);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn nettle_sexp_iterator_check_type(
    mut iterator: *mut sexp_iterator,
    mut type_0: *const libc::c_char,
) -> libc::c_int {
    return (nettle_sexp_iterator_enter_list(iterator) != 0
        && (*iterator).type_0 as libc::c_uint == SEXP_ATOM as libc::c_int as libc::c_uint
        && ((*iterator).display).is_null() && strlen(type_0) == (*iterator).atom_length
        && memcmp(
            type_0 as *const libc::c_void,
            (*iterator).atom as *const libc::c_void,
            (*iterator).atom_length,
        ) == 0 && nettle_sexp_iterator_next(iterator) != 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_sexp_iterator_check_types(
    mut iterator: *mut sexp_iterator,
    mut ntypes: libc::c_uint,
    mut types: *const *const libc::c_char,
) -> *const libc::c_char {
    if nettle_sexp_iterator_enter_list(iterator) != 0
        && (*iterator).type_0 as libc::c_uint == SEXP_ATOM as libc::c_int as libc::c_uint
        && ((*iterator).display).is_null()
    {
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < ntypes {
            if strlen(*types.offset(i as isize)) == (*iterator).atom_length
                && memcmp(
                    *types.offset(i as isize) as *const libc::c_void,
                    (*iterator).atom as *const libc::c_void,
                    (*iterator).atom_length,
                ) == 0
            {
                return if nettle_sexp_iterator_next(iterator) != 0 {
                    *types.offset(i as isize)
                } else {
                    0 as *const libc::c_char
                };
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn nettle_sexp_iterator_assoc(
    mut iterator: *mut sexp_iterator,
    mut nkeys: libc::c_uint,
    mut keys: *const *const libc::c_char,
    mut values: *mut sexp_iterator,
) -> libc::c_int {
    let mut found: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nfound: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nkeys as libc::c_ulong) as usize,
    );
    found = fresh5.leak().as_mut_ptr() as *mut libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < nkeys {
        *found.offset(i as isize) = 0 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    nfound = 0 as libc::c_int as libc::c_uint;
    loop {
        match (*iterator).type_0 as libc::c_uint {
            1 => {
                if nettle_sexp_iterator_enter_list(iterator) == 0 {
                    return 0 as libc::c_int;
                }
                if (*iterator).type_0 as libc::c_uint
                    == SEXP_ATOM as libc::c_int as libc::c_uint
                    && ((*iterator).display).is_null()
                {
                    i = 0 as libc::c_int as libc::c_uint;
                    while i < nkeys {
                        if strlen(*keys.offset(i as isize)) == (*iterator).atom_length
                            && memcmp(
                                *keys.offset(i as isize) as *const libc::c_void,
                                (*iterator).atom as *const libc::c_void,
                                (*iterator).atom_length,
                            ) == 0
                        {
                            if *found.offset(i as isize) != 0 {
                                return 0 as libc::c_int;
                            }
                            if nettle_sexp_iterator_next(iterator) == 0 {
                                return 0 as libc::c_int;
                            }
                            *found.offset(i as isize) = 1 as libc::c_int;
                            nfound = nfound.wrapping_add(1);
                            nfound;
                            *values.offset(i as isize) = *iterator;
                            break;
                        } else {
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                }
                if nettle_sexp_iterator_exit_list(iterator) == 0 {
                    return 0 as libc::c_int;
                }
            }
            0 => {
                if nettle_sexp_iterator_next(iterator) == 0 {
                    return 0 as libc::c_int;
                }
            }
            2 => {
                return (nettle_sexp_iterator_exit_list(iterator) != 0 && nfound == nkeys)
                    as libc::c_int;
            }
            _ => {
                abort();
            }
        }
    };
}
