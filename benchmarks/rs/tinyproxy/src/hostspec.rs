use ::libc;
extern "C" {
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn full_inet_pton(ip: *const libc::c_char, dst: *mut libc::c_void) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type hostspec_type = libc::c_uint;
pub const HST_NUMERIC: hostspec_type = 2;
pub const HST_STRING: hostspec_type = 1;
pub const HST_NONE: hostspec_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostspec {
    pub type_0: hostspec_type,
    pub address: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub string: *mut libc::c_char,
    pub ip: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub network: [libc::c_uchar; 16],
    pub mask: [libc::c_uchar; 16],
}
unsafe extern "C" fn dotted_mask(
    mut bitmask_string: *mut libc::c_char,
    mut array: *mut libc::c_uchar,
) -> libc::c_int {
    let mut v4bits: [libc::c_uchar; 4] = [0; 4];
    if 1 as libc::c_int
        != inet_pton(
            2 as libc::c_int,
            bitmask_string,
            v4bits.as_mut_ptr() as *mut libc::c_void,
        )
    {
        return -(1 as libc::c_int);
    }
    memset(
        array as *mut libc::c_void,
        0xff as libc::c_int,
        (16 as libc::c_int - 4 as libc::c_int) as libc::c_ulong,
    );
    memcpy(
        array.offset(16 as libc::c_int as isize).offset(-(4 as libc::c_int as isize))
            as *mut libc::c_void,
        v4bits.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn fill_netmask_array(
    mut bitmask_string: *mut libc::c_char,
    mut v6: libc::c_int,
    mut array: *mut libc::c_uchar,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut mask: libc::c_ulong = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    if !(strchr(bitmask_string, '.' as i32)).is_null() {
        if v6 != 0 {
            return -(1 as libc::c_int);
        }
        return dotted_mask(bitmask_string, array);
    }
    mask = strtoul(bitmask_string, &mut endptr, 10 as libc::c_int);
    if *__errno_location() == 34 as libc::c_int
        && mask
            == (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
        || *__errno_location() != 0 as libc::c_int
            && mask == 0 as libc::c_int as libc::c_ulong || endptr == bitmask_string
    {
        return -(1 as libc::c_int);
    }
    if v6 == 0 as libc::c_int {
        mask = mask
            .wrapping_add((12 as libc::c_int * 8 as libc::c_int) as libc::c_ulong);
    }
    if mask > (8 as libc::c_int * 16 as libc::c_int) as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i != 16 as libc::c_int as libc::c_uint {
        if mask >= 8 as libc::c_int as libc::c_ulong {
            *array.offset(i as isize) = 0xff as libc::c_int as libc::c_uchar;
            mask = mask.wrapping_sub(8 as libc::c_int as libc::c_ulong);
        } else if mask > 0 as libc::c_int as libc::c_ulong {
            *array
                .offset(
                    i as isize,
                ) = ((0xff as libc::c_int)
                << (8 as libc::c_int as libc::c_ulong).wrapping_sub(mask))
                as libc::c_uchar;
            mask = 0 as libc::c_int as libc::c_ulong;
        } else {
            *array.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hostspec_parse(
    mut location: *mut libc::c_char,
    mut h: *mut hostspec,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mask: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip_dst: [libc::c_char; 16] = [0; 16];
    (*h).type_0 = HST_NONE;
    if location.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        h as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<hostspec>() as libc::c_ulong,
    );
    mask = strrchr(location, '/' as i32);
    if !mask.is_null() {
        let fresh0 = mask;
        mask = mask.offset(1);
        *fresh0 = 0 as libc::c_int as libc::c_char;
    }
    if full_inet_pton(location, ip_dst.as_mut_ptr() as *mut libc::c_void)
        > 0 as libc::c_int
    {
        (*h).type_0 = HST_NUMERIC;
        memcpy(
            ((*h).address.ip.network).as_mut_ptr() as *mut libc::c_void,
            ip_dst.as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        if mask.is_null() {
            memset(
                ((*h).address.ip.mask).as_mut_ptr() as *mut libc::c_void,
                0xff as libc::c_int,
                16 as libc::c_int as libc::c_ulong,
            );
            current_block = 4808432441040389987;
        } else {
            let mut dst: [libc::c_char; 16] = [0; 16];
            let mut v6: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            if inet_pton(
                10 as libc::c_int,
                location,
                dst.as_mut_ptr() as *mut libc::c_void,
            ) > 0 as libc::c_int
            {
                v6 = 1 as libc::c_int;
            } else {
                v6 = 0 as libc::c_int;
            }
            if fill_netmask_array(
                mask,
                v6,
                &mut *((*h).address.ip.mask)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
            ) < 0 as libc::c_int
            {
                current_block = 11307063007268554308;
            } else {
                i = 0 as libc::c_int;
                while i < 16 as libc::c_int {
                    (*h)
                        .address
                        .ip
                        .network[i
                        as usize] = (ip_dst[i as usize] as libc::c_int
                        & (*h).address.ip.mask[i as usize] as libc::c_int)
                        as libc::c_uchar;
                    i += 1;
                    i;
                }
                current_block = 4808432441040389987;
            }
        }
    } else if !mask.is_null() || !(strchr(location, ':' as i32)).is_null() {
        current_block = 11307063007268554308;
    } else {
        (*h).type_0 = HST_STRING;
        (*h).address.string = strdup(location);
        if ((*h).address.string).is_null() {
            current_block = 11307063007268554308;
        } else {
            current_block = 4808432441040389987;
        }
    }
    match current_block {
        11307063007268554308 => {
            if !mask.is_null() {
                mask = mask.offset(-1);
                *mask = '/' as i32 as libc::c_char;
            }
            return -(1 as libc::c_int);
        }
        _ => {
            if !mask.is_null() {
                mask = mask.offset(-1);
                *mask = '/' as i32 as libc::c_char;
            }
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn string_match(
    mut ip: *const libc::c_char,
    mut addrspec: *const libc::c_char,
) -> libc::c_int {
    let mut test_length: size_t = 0;
    let mut match_length: size_t = 0;
    if strcasecmp(ip, addrspec) == 0 {
        return 1 as libc::c_int;
    }
    if *addrspec.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 {
        return 0 as libc::c_int;
    }
    test_length = strlen(ip);
    match_length = strlen(addrspec);
    if test_length < match_length {
        return 0 as libc::c_int;
    }
    return (strcasecmp(
        ip.offset(test_length.wrapping_sub(match_length) as isize),
        addrspec,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn numeric_match(
    mut addr: *const uint8_t,
    mut h: *const hostspec,
) -> libc::c_int {
    let mut x: uint8_t = 0;
    let mut y: uint8_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i != 16 as libc::c_int {
        x = (*addr.offset(i as isize) as libc::c_int
            & (*h).address.ip.mask[i as usize] as libc::c_int) as uint8_t;
        y = (*h).address.ip.network[i as usize];
        if x as libc::c_int != y as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn hostspec_match(
    mut ip: *const libc::c_char,
    mut h: *const hostspec,
) -> libc::c_int {
    let mut is_numeric_addr: libc::c_int = 0;
    let mut numeric_addr: [uint8_t; 16] = [0; 16];
    if *ip.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    is_numeric_addr = (full_inet_pton(
        ip,
        &mut numeric_addr as *mut [uint8_t; 16] as *mut libc::c_void,
    ) > 0 as libc::c_int) as libc::c_int;
    match (*h).type_0 as libc::c_uint {
        1 => {
            if is_numeric_addr != 0 {
                return 0 as libc::c_int;
            }
            return string_match(ip, (*h).address.string);
        }
        2 => return numeric_match(numeric_addr.as_mut_ptr() as *const uint8_t, h),
        0 => return 0 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
