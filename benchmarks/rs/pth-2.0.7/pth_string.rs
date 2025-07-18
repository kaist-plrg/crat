use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
unsafe extern "C" fn dopr(
    mut buffer: *mut libc::c_char,
    mut maxlen: size_t,
    mut retlen: *mut size_t,
    mut format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut ch: libc::c_char = 0;
    let mut value: libc::c_longlong = 0;
    let mut fvalue: f128::f128 = f128::f128::ZERO;
    let mut strvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut state: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut cflags: libc::c_int = 0;
    let mut currlen: size_t = 0;
    state = 0 as libc::c_int;
    min = 0 as libc::c_int;
    cflags = min;
    currlen = cflags as size_t;
    flags = currlen as libc::c_int;
    max = -(1 as libc::c_int);
    let fresh0 = format;
    format = format.offset(1);
    ch = *fresh0;
    if maxlen == -(1 as libc::c_int) as libc::c_ulong {
        maxlen = !((1 as libc::c_int as size_t)
            << (::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong));
    }
    while state != 7 as libc::c_int {
        if ch as libc::c_int == '\0' as i32 || currlen >= maxlen {
            state = 7 as libc::c_int;
        }
        match state {
            0 => {
                if ch as libc::c_int == '%' as i32 {
                    state = 1 as libc::c_int;
                } else {
                    dopr_outch(buffer, &mut currlen, maxlen, ch as libc::c_int);
                }
                let fresh1 = format;
                format = format.offset(1);
                ch = *fresh1;
            }
            1 => {
                match ch as libc::c_int {
                    45 => {
                        flags |= (1 as libc::c_int) << 0 as libc::c_int;
                        let fresh2 = format;
                        format = format.offset(1);
                        ch = *fresh2;
                    }
                    43 => {
                        flags |= (1 as libc::c_int) << 1 as libc::c_int;
                        let fresh3 = format;
                        format = format.offset(1);
                        ch = *fresh3;
                    }
                    32 => {
                        flags |= (1 as libc::c_int) << 2 as libc::c_int;
                        let fresh4 = format;
                        format = format.offset(1);
                        ch = *fresh4;
                    }
                    35 => {
                        flags |= (1 as libc::c_int) << 3 as libc::c_int;
                        let fresh5 = format;
                        format = format.offset(1);
                        ch = *fresh5;
                    }
                    48 => {
                        flags |= (1 as libc::c_int) << 4 as libc::c_int;
                        let fresh6 = format;
                        format = format.offset(1);
                        ch = *fresh6;
                    }
                    _ => {
                        state = 2 as libc::c_int;
                    }
                }
            }
            2 => {
                if *(*__ctype_b_loc())
                    .offset(ch as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    min = 10 as libc::c_int * min + (ch as libc::c_int - '0' as i32);
                    let fresh7 = format;
                    format = format.offset(1);
                    ch = *fresh7;
                } else if ch as libc::c_int == '*' as i32 {
                    min = args.arg::<libc::c_int>();
                    let fresh8 = format;
                    format = format.offset(1);
                    ch = *fresh8;
                    state = 3 as libc::c_int;
                } else {
                    state = 3 as libc::c_int;
                }
            }
            3 => {
                if ch as libc::c_int == '.' as i32 {
                    state = 4 as libc::c_int;
                    let fresh9 = format;
                    format = format.offset(1);
                    ch = *fresh9;
                } else {
                    state = 5 as libc::c_int;
                }
            }
            4 => {
                if *(*__ctype_b_loc())
                    .offset(ch as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    if max < 0 as libc::c_int {
                        max = 0 as libc::c_int;
                    }
                    max = 10 as libc::c_int * max + (ch as libc::c_int - '0' as i32);
                    let fresh10 = format;
                    format = format.offset(1);
                    ch = *fresh10;
                } else if ch as libc::c_int == '*' as i32 {
                    max = args.arg::<libc::c_int>();
                    let fresh11 = format;
                    format = format.offset(1);
                    ch = *fresh11;
                    state = 5 as libc::c_int;
                } else {
                    state = 5 as libc::c_int;
                }
            }
            5 => {
                match ch as libc::c_int {
                    104 => {
                        cflags = 1 as libc::c_int;
                        let fresh12 = format;
                        format = format.offset(1);
                        ch = *fresh12;
                    }
                    108 => {
                        if *format as libc::c_int == 'l' as i32 {
                            cflags = 4 as libc::c_int;
                            format = format.offset(1);
                            format;
                        } else {
                            cflags = 2 as libc::c_int;
                        }
                        let fresh13 = format;
                        format = format.offset(1);
                        ch = *fresh13;
                    }
                    113 => {
                        cflags = 4 as libc::c_int;
                        let fresh14 = format;
                        format = format.offset(1);
                        ch = *fresh14;
                    }
                    76 => {
                        cflags = 3 as libc::c_int;
                        let fresh15 = format;
                        format = format.offset(1);
                        ch = *fresh15;
                    }
                    _ => {}
                }
                state = 6 as libc::c_int;
            }
            6 => {
                let mut current_block_114: u64;
                match ch as libc::c_int {
                    100 | 105 => {
                        match cflags {
                            1 => {
                                value = args.arg::<libc::c_int>() as libc::c_short
                                    as libc::c_longlong;
                            }
                            2 => {
                                value = args.arg::<libc::c_long>() as libc::c_longlong;
                            }
                            4 => {
                                value = args.arg::<libc::c_longlong>();
                            }
                            _ => {
                                value = args.arg::<libc::c_int>() as libc::c_longlong;
                            }
                        }
                        fmtint(
                            buffer,
                            &mut currlen,
                            maxlen,
                            value,
                            10 as libc::c_int,
                            min,
                            max,
                            flags,
                        );
                        current_block_114 = 16314074004867283505;
                    }
                    88 => {
                        flags |= (1 as libc::c_int) << 5 as libc::c_int;
                        current_block_114 = 15952463821955552698;
                    }
                    120 | 111 | 117 => {
                        current_block_114 = 15952463821955552698;
                    }
                    102 => {
                        if cflags == 3 as libc::c_int {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        } else {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        }
                        fmtfp(buffer, &mut currlen, maxlen, fvalue, min, max, flags);
                        current_block_114 = 16314074004867283505;
                    }
                    69 => {
                        flags |= (1 as libc::c_int) << 5 as libc::c_int;
                        current_block_114 = 5419139134696370412;
                    }
                    101 => {
                        current_block_114 = 5419139134696370412;
                    }
                    71 => {
                        flags |= (1 as libc::c_int) << 5 as libc::c_int;
                        current_block_114 = 9823790708098527527;
                    }
                    103 => {
                        current_block_114 = 9823790708098527527;
                    }
                    99 => {
                        dopr_outch(
                            buffer,
                            &mut currlen,
                            maxlen,
                            args.arg::<libc::c_int>(),
                        );
                        current_block_114 = 16314074004867283505;
                    }
                    115 => {
                        strvalue = args.arg::<*mut libc::c_char>();
                        if max < 0 as libc::c_int {
                            max = maxlen as libc::c_int;
                        }
                        fmtstr(buffer, &mut currlen, maxlen, strvalue, flags, min, max);
                        current_block_114 = 16314074004867283505;
                    }
                    112 => {
                        value = args.arg::<*mut libc::c_void>() as libc::c_long
                            as libc::c_longlong;
                        fmtint(
                            buffer,
                            &mut currlen,
                            maxlen,
                            value,
                            16 as libc::c_int,
                            min,
                            max,
                            flags,
                        );
                        current_block_114 = 16314074004867283505;
                    }
                    110 => {
                        if cflags == 1 as libc::c_int {
                            let mut num: *mut libc::c_short = 0 as *mut libc::c_short;
                            num = args.arg::<*mut libc::c_short>();
                            *num = currlen as libc::c_short;
                        } else if cflags == 2 as libc::c_int {
                            let mut num_0: *mut libc::c_long = 0 as *mut libc::c_long;
                            num_0 = args.arg::<*mut libc::c_long>();
                            *num_0 = currlen as libc::c_long;
                        } else if cflags == 4 as libc::c_int {
                            let mut num_1: *mut libc::c_longlong = 0
                                as *mut libc::c_longlong;
                            num_1 = args.arg::<*mut libc::c_longlong>();
                            *num_1 = currlen as libc::c_longlong;
                        } else {
                            let mut num_2: *mut libc::c_int = 0 as *mut libc::c_int;
                            num_2 = args.arg::<*mut libc::c_int>();
                            *num_2 = currlen as libc::c_int;
                        }
                        current_block_114 = 16314074004867283505;
                    }
                    37 => {
                        dopr_outch(buffer, &mut currlen, maxlen, ch as libc::c_int);
                        current_block_114 = 16314074004867283505;
                    }
                    119 => {
                        let fresh16 = format;
                        format = format.offset(1);
                        ch = *fresh16;
                        current_block_114 = 16314074004867283505;
                    }
                    _ => {
                        current_block_114 = 16314074004867283505;
                    }
                }
                match current_block_114 {
                    15952463821955552698 => {
                        flags |= (1 as libc::c_int) << 6 as libc::c_int;
                        match cflags {
                            1 => {
                                value = args.arg::<libc::c_uint>() as libc::c_ushort
                                    as libc::c_longlong;
                            }
                            2 => {
                                value = args.arg::<libc::c_ulong>() as libc::c_longlong;
                            }
                            4 => {
                                value = args.arg::<libc::c_ulonglong>() as libc::c_longlong;
                            }
                            _ => {
                                value = args.arg::<libc::c_uint>() as libc::c_longlong;
                            }
                        }
                        fmtint(
                            buffer,
                            &mut currlen,
                            maxlen,
                            value,
                            if ch as libc::c_int == 'o' as i32 {
                                8 as libc::c_int
                            } else if ch as libc::c_int == 'u' as i32 {
                                10 as libc::c_int
                            } else {
                                16 as libc::c_int
                            },
                            min,
                            max,
                            flags,
                        );
                    }
                    5419139134696370412 => {
                        if cflags == 3 as libc::c_int {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        } else {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        }
                    }
                    9823790708098527527 => {
                        if cflags == 3 as libc::c_int {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        } else {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        }
                    }
                    _ => {}
                }
                let fresh17 = format;
                format = format.offset(1);
                ch = *fresh17;
                state = 0 as libc::c_int;
                min = 0 as libc::c_int;
                cflags = min;
                flags = cflags;
                max = -(1 as libc::c_int);
            }
            7 | _ => {}
        }
    }
    if currlen >= maxlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        currlen = maxlen.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    if !buffer.is_null() {
        *buffer.offset(currlen as isize) = '\0' as i32 as libc::c_char;
    }
    *retlen = currlen;
}
unsafe extern "C" fn fmtstr(
    mut buffer: *mut libc::c_char,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
) {
    let mut padlen: libc::c_int = 0;
    let mut strln: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    if value.is_null() {
        value = b"<NULL>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    strln = 0 as libc::c_int;
    while *value.offset(strln as isize) as libc::c_int != '\0' as i32 {
        strln += 1;
        strln;
    }
    padlen = min - strln;
    if padlen < 0 as libc::c_int {
        padlen = 0 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        padlen = -padlen;
    }
    while padlen > 0 as libc::c_int && cnt < max {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen -= 1;
        padlen;
        cnt += 1;
        cnt;
    }
    while *value as libc::c_int != 0 && cnt < max {
        let fresh18 = value;
        value = value.offset(1);
        dopr_outch(buffer, currlen, maxlen, *fresh18 as libc::c_int);
        cnt += 1;
        cnt;
    }
    while padlen < 0 as libc::c_int && cnt < max {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen += 1;
        padlen;
        cnt += 1;
        cnt;
    }
}
unsafe extern "C" fn fmtint(
    mut buffer: *mut libc::c_char,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut value: libc::c_longlong,
    mut base: libc::c_int,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut signvalue: libc::c_int = 0 as libc::c_int;
    let mut uvalue: libc::c_ulonglong = 0;
    let mut convert: [libc::c_char; 20] = [0; 20];
    let mut place: libc::c_int = 0 as libc::c_int;
    let mut spadlen: libc::c_int = 0 as libc::c_int;
    let mut zpadlen: libc::c_int = 0 as libc::c_int;
    let mut caps: libc::c_int = 0 as libc::c_int;
    if max < 0 as libc::c_int {
        max = 0 as libc::c_int;
    }
    uvalue = value as libc::c_ulonglong;
    if flags & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        if value < 0 as libc::c_int as libc::c_longlong {
            signvalue = '-' as i32;
            uvalue = -value as libc::c_ulonglong;
        } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            signvalue = '+' as i32;
        } else if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            signvalue = ' ' as i32;
        }
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        caps = 1 as libc::c_int;
    }
    loop {
        let fresh19 = place;
        place = place + 1;
        convert[fresh19
            as usize] = *if caps != 0 {
            b"0123456789ABCDEF\0" as *const u8 as *const libc::c_char
        } else {
            b"0123456789abcdef\0" as *const u8 as *const libc::c_char
        }
            .offset(
                uvalue.wrapping_rem(base as libc::c_uint as libc::c_ulonglong) as isize,
            );
        uvalue = uvalue.wrapping_div(base as libc::c_uint as libc::c_ulonglong);
        if !(uvalue != 0 && place < 20 as libc::c_int) {
            break;
        }
    }
    if place == 20 as libc::c_int {
        place -= 1;
        place;
    }
    convert[place as usize] = 0 as libc::c_int as libc::c_char;
    zpadlen = max - place;
    spadlen = min - (if max >= place { max } else { place })
        - (if signvalue != 0 { 1 as libc::c_int } else { 0 as libc::c_int });
    if zpadlen < 0 as libc::c_int {
        zpadlen = 0 as libc::c_int;
    }
    if spadlen < 0 as libc::c_int {
        spadlen = 0 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        zpadlen = if zpadlen >= spadlen { zpadlen } else { spadlen };
        spadlen = 0 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        spadlen = -spadlen;
    }
    while spadlen > 0 as libc::c_int {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        spadlen -= 1;
        spadlen;
    }
    if signvalue != 0 {
        dopr_outch(buffer, currlen, maxlen, signvalue);
    }
    if zpadlen > 0 as libc::c_int {
        while zpadlen > 0 as libc::c_int {
            dopr_outch(buffer, currlen, maxlen, '0' as i32);
            zpadlen -= 1;
            zpadlen;
        }
    }
    while place > 0 as libc::c_int {
        place -= 1;
        dopr_outch(buffer, currlen, maxlen, convert[place as usize] as libc::c_int);
    }
    while spadlen < 0 as libc::c_int {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        spadlen += 1;
        spadlen;
    }
}
unsafe extern "C" fn math_abs(mut value: f128::f128) -> f128::f128 {
    let mut result: f128::f128 = value;
    if value < f128::f128::new(0 as libc::c_int) {
        result = -value;
    }
    return result;
}
unsafe extern "C" fn math_pow10(mut exponent: libc::c_int) -> f128::f128 {
    let mut result: f128::f128 = f128::f128::new(1 as libc::c_int);
    while exponent > 0 as libc::c_int {
        result *= f128::f128::new(10 as libc::c_int);
        exponent -= 1;
        exponent;
    }
    return result;
}
unsafe extern "C" fn math_round(mut value: f128::f128) -> libc::c_long {
    let mut intpart: libc::c_long = 0;
    intpart = value.to_i64().unwrap();
    value = value - f128::f128::new(intpart);
    if value >= f128::f128::new(0.5f64) {
        intpart += 1;
        intpart;
    }
    return intpart;
}
unsafe extern "C" fn fmtfp(
    mut buffer: *mut libc::c_char,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut fvalue: f128::f128,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut signvalue: libc::c_int = 0 as libc::c_int;
    let mut ufvalue: f128::f128 = f128::f128::ZERO;
    let mut iconvert: [libc::c_char; 20] = [0; 20];
    let mut fconvert: [libc::c_char; 20] = [0; 20];
    let mut iplace: libc::c_int = 0 as libc::c_int;
    let mut fplace: libc::c_int = 0 as libc::c_int;
    let mut padlen: libc::c_int = 0 as libc::c_int;
    let mut zpadlen: libc::c_int = 0 as libc::c_int;
    let mut caps: libc::c_int = 0 as libc::c_int;
    let mut intpart: libc::c_long = 0;
    let mut fracpart: libc::c_long = 0;
    if max < 0 as libc::c_int {
        max = 6 as libc::c_int;
    }
    ufvalue = math_abs(fvalue);
    if fvalue < f128::f128::new(0 as libc::c_int) {
        signvalue = '-' as i32;
    } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        signvalue = '+' as i32;
    } else if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        signvalue = ' ' as i32;
    }
    intpart = ufvalue.to_i64().unwrap();
    if max > 9 as libc::c_int {
        max = 9 as libc::c_int;
    }
    fracpart = math_round(math_pow10(max) * (ufvalue - f128::f128::new(intpart)));
    if f128::f128::new(fracpart) >= math_pow10(max) {
        intpart += 1;
        intpart;
        fracpart = (f128::f128::from(fracpart) - math_pow10(max)).to_i64().unwrap();
    }
    loop {
        let fresh20 = iplace;
        iplace = iplace + 1;
        iconvert[fresh20
            as usize] = *if caps != 0 {
            b"0123456789ABCDEF\0" as *const u8 as *const libc::c_char
        } else {
            b"0123456789abcdef\0" as *const u8 as *const libc::c_char
        }
            .offset((intpart % 10 as libc::c_int as libc::c_long) as isize);
        intpart = intpart / 10 as libc::c_int as libc::c_long;
        if !(intpart != 0 && iplace < 20 as libc::c_int) {
            break;
        }
    }
    if iplace == 20 as libc::c_int {
        iplace -= 1;
        iplace;
    }
    iconvert[iplace as usize] = 0 as libc::c_int as libc::c_char;
    loop {
        let fresh21 = fplace;
        fplace = fplace + 1;
        fconvert[fresh21
            as usize] = *if caps != 0 {
            b"0123456789ABCDEF\0" as *const u8 as *const libc::c_char
        } else {
            b"0123456789abcdef\0" as *const u8 as *const libc::c_char
        }
            .offset((fracpart % 10 as libc::c_int as libc::c_long) as isize);
        fracpart = fracpart / 10 as libc::c_int as libc::c_long;
        if !(fracpart != 0 && fplace < 20 as libc::c_int) {
            break;
        }
    }
    if fplace == 20 as libc::c_int {
        fplace -= 1;
        fplace;
    }
    fconvert[fplace as usize] = 0 as libc::c_int as libc::c_char;
    padlen = min - iplace - max - 1 as libc::c_int
        - (if signvalue != 0 { 1 as libc::c_int } else { 0 as libc::c_int });
    zpadlen = max - fplace;
    if zpadlen < 0 as libc::c_int {
        zpadlen = 0 as libc::c_int;
    }
    if padlen < 0 as libc::c_int {
        padlen = 0 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        padlen = -padlen;
    }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 && padlen > 0 as libc::c_int {
        if signvalue != 0 {
            dopr_outch(buffer, currlen, maxlen, signvalue);
            padlen -= 1;
            padlen;
            signvalue = 0 as libc::c_int;
        }
        while padlen > 0 as libc::c_int {
            dopr_outch(buffer, currlen, maxlen, '0' as i32);
            padlen -= 1;
            padlen;
        }
    }
    while padlen > 0 as libc::c_int {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen -= 1;
        padlen;
    }
    if signvalue != 0 {
        dopr_outch(buffer, currlen, maxlen, signvalue);
    }
    while iplace > 0 as libc::c_int {
        iplace -= 1;
        dopr_outch(buffer, currlen, maxlen, iconvert[iplace as usize] as libc::c_int);
    }
    if max > 0 as libc::c_int {
        dopr_outch(buffer, currlen, maxlen, '.' as i32);
        while fplace > 0 as libc::c_int {
            fplace -= 1;
            dopr_outch(
                buffer,
                currlen,
                maxlen,
                fconvert[fplace as usize] as libc::c_int,
            );
        }
    }
    while zpadlen > 0 as libc::c_int {
        dopr_outch(buffer, currlen, maxlen, '0' as i32);
        zpadlen -= 1;
        zpadlen;
    }
    while padlen < 0 as libc::c_int {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen += 1;
        padlen;
    }
}
unsafe extern "C" fn dopr_outch(
    mut buffer: *mut libc::c_char,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut c: libc::c_int,
) {
    if *currlen < maxlen {
        if !buffer.is_null() {
            *buffer.offset(*currlen as isize) = c as libc::c_char;
        }
        *currlen = (*currlen).wrapping_add(1);
        *currlen;
    }
}
pub unsafe extern "C" fn __pth_vsnprintf(
    mut str: *mut libc::c_char,
    mut count: size_t,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) -> libc::c_int {
    let mut retlen: size_t = 0;
    if !str.is_null() {
        *str.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    dopr(str, count, &mut retlen, fmt, args.as_va_list());
    return retlen as libc::c_int;
}
pub unsafe extern "C" fn __pth_snprintf(
    mut str: *mut libc::c_char,
    mut count: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut rv: libc::c_int = 0;
    ap = args.clone();
    rv = __pth_vsnprintf(str, count, fmt, ap.as_va_list());
    return rv;
}
pub unsafe extern "C" fn __pth_vasprintf(
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> *mut libc::c_char {
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    n = __pth_vsnprintf(
        0 as *mut libc::c_char,
        -(1 as libc::c_int) as size_t,
        fmt,
        ap.as_va_list(),
    );
    rv = malloc((n + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    if rv.is_null() {
        return 0 as *mut libc::c_char;
    }
    __pth_vsnprintf(rv, (n + 1 as libc::c_int) as size_t, fmt, ap.as_va_list());
    return rv;
}
pub unsafe extern "C" fn __pth_asprintf(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut ap: ::std::ffi::VaListImpl;
    let mut rv: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    rv = __pth_vasprintf(fmt, ap.as_va_list());
    return rv;
}
