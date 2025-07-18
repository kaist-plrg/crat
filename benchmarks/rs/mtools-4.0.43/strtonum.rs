use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn strtol_with_range(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
    mut min: libc::c_long,
    mut max: libc::c_long,
) -> libc::c_long {
    let mut l: libc::c_long = strtol(nptr, endptr, base);
    if l > max {
        *__errno_location() = 34 as libc::c_int;
        return max;
    }
    if l < min {
        *__errno_location() = 34 as libc::c_int;
        return min;
    }
    return l;
}
unsafe extern "C" fn strtoul_with_range(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
    mut max: libc::c_ulong,
) -> libc::c_ulong {
    let mut l: libc::c_ulong = strtoul(nptr, endptr, base);
    if l > max {
        *__errno_location() = 34 as libc::c_int;
        return max;
    }
    return l;
}
pub unsafe extern "C" fn strtoui(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> libc::c_uint {
    return strtoul_with_range(
        nptr,
        endptr,
        base,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong,
    ) as libc::c_uint;
}
pub unsafe extern "C" fn atoui(mut str: *const libc::c_char) -> libc::c_uint {
    return strtoui(str, 0 as *mut *mut libc::c_char, 0 as libc::c_int);
}
pub unsafe extern "C" fn strtosi(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> libc::c_int {
    return strtol_with_range(
        nptr,
        endptr,
        base,
        (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long,
        2147483647 as libc::c_int as libc::c_long,
    ) as libc::c_int;
}
pub unsafe extern "C" fn atoul(mut str: *const libc::c_char) -> libc::c_ulong {
    return strtoul(str, 0 as *mut *mut libc::c_char, 0 as libc::c_int);
}
pub unsafe extern "C" fn strtou8(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uint8_t {
    return strtoul_with_range(nptr, endptr, base, 255 as libc::c_int as libc::c_ulong)
        as uint8_t;
}
pub unsafe extern "C" fn atou8(mut str: *const libc::c_char) -> uint8_t {
    return strtou8(str, 0 as *mut *mut libc::c_char, 0 as libc::c_int);
}
pub unsafe extern "C" fn strtou16(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uint16_t {
    return strtoul_with_range(nptr, endptr, base, 65535 as libc::c_int as libc::c_ulong)
        as uint16_t;
}
pub unsafe extern "C" fn atou16(mut str: *const libc::c_char) -> uint16_t {
    return strtou16(str, 0 as *mut *mut libc::c_char, 0 as libc::c_int);
}
pub unsafe extern "C" fn strtou32(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uint32_t {
    return strtoul_with_range(
        nptr,
        endptr,
        base,
        4294967295 as libc::c_uint as libc::c_ulong,
    ) as uint32_t;
}
pub unsafe extern "C" fn atou32(mut str: *const libc::c_char) -> uint32_t {
    return strtou32(str, 0 as *mut *mut libc::c_char, 0 as libc::c_int);
}
unsafe extern "C" fn checkOverflow(mut tot_sectors: uint32_t, mut bits: libc::c_int) {
    if tot_sectors > 4294967295 as libc::c_uint >> bits {
        fprintf(stderr, b"Too many sectors\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn parseSize(mut sizeStr: *mut libc::c_char) -> uint32_t {
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tot_sectors: uint32_t = strtou32(sizeStr, &mut eptr, 10 as libc::c_int);
    if eptr == sizeStr {
        fprintf(stderr, b"Bad size %s\n\0" as *const u8 as *const libc::c_char, sizeStr);
        exit(1 as libc::c_int);
    }
    let mut current_block_12: u64;
    match ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = *eptr as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(*eptr as libc::c_int);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(*eptr as libc::c_int as isize);
        }
        __res
    }) {
        84 => {
            checkOverflow(tot_sectors, 10 as libc::c_int);
            tot_sectors = (tot_sectors as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint) as uint32_t
                as uint32_t;
            current_block_12 = 17595787843618381070;
        }
        71 => {
            current_block_12 = 17595787843618381070;
        }
        77 => {
            current_block_12 = 14692438510323324791;
        }
        75 => {
            current_block_12 = 14728642583546188501;
        }
        0 | _ => {
            current_block_12 = 7651349459974463963;
        }
    }
    match current_block_12 {
        17595787843618381070 => {
            checkOverflow(tot_sectors, 10 as libc::c_int);
            tot_sectors = (tot_sectors as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint) as uint32_t
                as uint32_t;
            current_block_12 = 14692438510323324791;
        }
        _ => {}
    }
    match current_block_12 {
        14692438510323324791 => {
            checkOverflow(tot_sectors, 10 as libc::c_int);
            tot_sectors = (tot_sectors as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint) as uint32_t
                as uint32_t;
            current_block_12 = 14728642583546188501;
        }
        _ => {}
    }
    match current_block_12 {
        14728642583546188501 => {
            checkOverflow(tot_sectors, 1 as libc::c_int);
            tot_sectors = (tot_sectors as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
            eptr = eptr.offset(1);
            eptr;
        }
        _ => {}
    }
    if *eptr != 0 {
        fprintf(stderr, b"Bad suffix %s\n\0" as *const u8 as *const libc::c_char, eptr);
        exit(1 as libc::c_int);
    }
    return tot_sectors;
}
