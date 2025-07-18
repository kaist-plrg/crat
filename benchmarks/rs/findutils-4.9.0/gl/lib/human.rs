use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn localeconv() -> *mut lconv;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn argmatch(
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
    ) -> ptrdiff_t;
}
pub type __uintmax_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
pub type size_t = libc::c_ulong;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const human_B: C2RustUnnamed = 256;
pub const human_SI: C2RustUnnamed = 128;
pub const human_space_before_unit: C2RustUnnamed = 64;
pub const human_base_1024: C2RustUnnamed = 32;
pub const human_autoscale: C2RustUnnamed = 16;
pub const human_suppress_point_zero: C2RustUnnamed = 8;
pub const human_group_digits: C2RustUnnamed = 4;
pub const human_floor: C2RustUnnamed = 2;
pub const human_round_to_nearest: C2RustUnnamed = 1;
pub const human_ceiling: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub type ptrdiff_t = libc::c_long;
static mut power_letter: [libc::c_char; 9] = [
    0 as libc::c_int as libc::c_char,
    'K' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
];
unsafe extern "C" fn adjust_value(
    mut inexact_style: libc::c_int,
    mut value: f128::f128,
) -> f128::f128 {
    if inexact_style != human_round_to_nearest as libc::c_int
        && value < f128::f128::new(18446744073709551615 as libc::c_ulong)
    {
        let mut u: uintmax_t = value.to_u64().unwrap();
        value = f128::f128::new(
            u
                .wrapping_add(
                    (inexact_style == human_ceiling as libc::c_int
                        && f128::f128::new(u) != value) as libc::c_int as libc::c_ulong,
                ),
        );
    }
    return value;
}
unsafe extern "C" fn group_number(
    mut number: *mut libc::c_char,
    mut numberlen: size_t,
    mut grouping: *const libc::c_char,
    mut thousands_sep: *const libc::c_char,
) -> *mut libc::c_char {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut grouplen: size_t = 18446744073709551615 as libc::c_ulong;
    let mut thousands_seplen: size_t = strlen(thousands_sep);
    let mut i: size_t = numberlen;
    let mut buf: [libc::c_char; 41] = [0; 41];
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        number as *const libc::c_void,
        numberlen,
    );
    d = number.offset(numberlen as isize);
    loop {
        let mut g: libc::c_uchar = *grouping as libc::c_uchar;
        if g != 0 {
            grouplen = if (g as libc::c_int) < 127 as libc::c_int {
                g as libc::c_ulong
            } else {
                i
            };
            grouping = grouping.offset(1);
            grouping;
        }
        if i < grouplen {
            grouplen = i;
        }
        d = d.offset(-(grouplen as isize));
        i = (i as libc::c_ulong).wrapping_sub(grouplen) as size_t as size_t;
        memcpy(
            d as *mut libc::c_void,
            buf.as_mut_ptr().offset(i as isize) as *const libc::c_void,
            grouplen,
        );
        if i == 0 as libc::c_int as libc::c_ulong {
            return d;
        }
        d = d.offset(-(thousands_seplen as isize));
        memcpy(
            d as *mut libc::c_void,
            thousands_sep as *const libc::c_void,
            thousands_seplen,
        );
    };
}
pub unsafe extern "C" fn human_readable(
    mut n: uintmax_t,
    mut buf: *mut libc::c_char,
    mut opts: libc::c_int,
    mut from_block_size: uintmax_t,
    mut to_block_size: uintmax_t,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut inexact_style: libc::c_int = opts
        & (human_round_to_nearest as libc::c_int | human_floor as libc::c_int
            | human_ceiling as libc::c_int);
    let mut base: libc::c_uint = (if opts & human_base_1024 as libc::c_int != 0 {
        1024 as libc::c_int
    } else {
        1000 as libc::c_int
    }) as libc::c_uint;
    let mut amt: uintmax_t = 0;
    let mut tenths: libc::c_int = 0;
    let mut exponent: libc::c_int = -(1 as libc::c_int);
    let mut exponent_max: libc::c_int = (::std::mem::size_of::<[libc::c_char; 9]>()
        as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psuffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut integerlim: *const libc::c_char = 0 as *const libc::c_char;
    let mut rounding: libc::c_int = 0;
    let mut decimal_point: *const libc::c_char = b".\0" as *const u8
        as *const libc::c_char;
    let mut decimal_pointlen: size_t = 1 as libc::c_int as size_t;
    let mut grouping: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut thousands_sep: *const libc::c_char = b"\0" as *const u8
        as *const libc::c_char;
    let mut l: *const lconv = localeconv();
    let mut pointlen: size_t = strlen((*l).decimal_point);
    if (0 as libc::c_int as libc::c_ulong) < pointlen
        && pointlen <= 16 as libc::c_int as libc::c_ulong
    {
        decimal_point = (*l).decimal_point;
        decimal_pointlen = pointlen;
    }
    grouping = (*l).grouping;
    if strlen((*l).thousands_sep) <= 16 as libc::c_int as libc::c_ulong {
        thousands_sep = (*l).thousands_sep;
    }
    psuffix = buf
        .offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                .wrapping_div(485 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul((16 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_sub(16 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
        )
        .offset(-(1 as libc::c_int as isize))
        .offset(-(3 as libc::c_int as isize));
    p = psuffix;
    if to_block_size <= from_block_size {
        if from_block_size.wrapping_rem(to_block_size)
            == 0 as libc::c_int as libc::c_ulong
        {
            let mut multiplier: uintmax_t = from_block_size.wrapping_div(to_block_size);
            amt = n.wrapping_mul(multiplier);
            if amt.wrapping_div(multiplier) == n {
                tenths = 0 as libc::c_int;
                rounding = 0 as libc::c_int;
                current_block = 14798692766849634534;
            } else {
                current_block = 11298138898191919651;
            }
        } else {
            current_block = 11298138898191919651;
        }
    } else if from_block_size != 0 as libc::c_int as libc::c_ulong
        && to_block_size.wrapping_rem(from_block_size)
            == 0 as libc::c_int as libc::c_ulong
    {
        let mut divisor: uintmax_t = to_block_size.wrapping_div(from_block_size);
        let mut r10: uintmax_t = n
            .wrapping_rem(divisor)
            .wrapping_mul(10 as libc::c_int as libc::c_ulong);
        let mut r2: uintmax_t = r10
            .wrapping_rem(divisor)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        amt = n.wrapping_div(divisor);
        tenths = r10.wrapping_div(divisor) as libc::c_int;
        rounding = if r2 < divisor {
            ((0 as libc::c_int as libc::c_ulong) < r2) as libc::c_int
        } else {
            2 as libc::c_int + (divisor < r2) as libc::c_int
        };
        current_block = 14798692766849634534;
    } else {
        current_block = 11298138898191919651;
    }
    match current_block {
        11298138898191919651 => {
            let mut dto_block_size: f128::f128 = f128::f128::new(to_block_size);
            let mut damt: f128::f128 = f128::f128::new(n)
                * (f128::f128::new(from_block_size) / dto_block_size);
            let mut buflen: size_t = 0;
            let mut nonintegerlen: size_t = 0;
            if opts & human_autoscale as libc::c_int == 0 {
                sprintf(
                    buf,
                    b"%.0Lf\0" as *const u8 as *const libc::c_char,
                    adjust_value(inexact_style, damt),
                );
                buflen = strlen(buf);
                nonintegerlen = 0 as libc::c_int as size_t;
            } else {
                let mut e: f128::f128 = f128::f128::new(1 as libc::c_int);
                exponent = 0 as libc::c_int;
                loop {
                    e *= f128::f128::new(base);
                    exponent += 1;
                    exponent;
                    if !(e * f128::f128::new(base) <= damt && exponent < exponent_max) {
                        break;
                    }
                }
                damt /= e;
                sprintf(
                    buf,
                    b"%.1Lf\0" as *const u8 as *const libc::c_char,
                    adjust_value(inexact_style, damt),
                );
                buflen = strlen(buf);
                nonintegerlen = decimal_pointlen
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                if (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(nonintegerlen)
                    .wrapping_add(
                        (opts & human_base_1024 as libc::c_int == 0) as libc::c_int
                            as libc::c_ulong,
                    ) < buflen
                    || opts & human_suppress_point_zero as libc::c_int != 0
                        && *buf
                            .offset(
                                buflen.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int == '0' as i32
                {
                    sprintf(
                        buf,
                        b"%.0Lf\0" as *const u8 as *const libc::c_char,
                        adjust_value(
                            inexact_style,
                            damt * f128::f128::new(10 as libc::c_int),
                        ) / f128::f128::new(10 as libc::c_int),
                    );
                    buflen = strlen(buf);
                    nonintegerlen = 0 as libc::c_int as size_t;
                }
            }
            p = psuffix.offset(-(buflen as isize));
            memmove(p as *mut libc::c_void, buf as *const libc::c_void, buflen);
            integerlim = p.offset(buflen as isize).offset(-(nonintegerlen as isize));
        }
        _ => {
            if opts & human_autoscale as libc::c_int != 0 {
                exponent = 0 as libc::c_int;
                if base as libc::c_ulong <= amt {
                    loop {
                        let mut r10_0: libc::c_uint = amt
                            .wrapping_rem(base as libc::c_ulong)
                            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                            .wrapping_add(tenths as libc::c_ulong) as libc::c_uint;
                        let mut r2_0: libc::c_uint = r10_0
                            .wrapping_rem(base)
                            .wrapping_mul(2 as libc::c_int as libc::c_uint)
                            .wrapping_add(
                                (rounding >> 1 as libc::c_int) as libc::c_uint,
                            );
                        amt = (amt as libc::c_ulong).wrapping_div(base as libc::c_ulong)
                            as uintmax_t as uintmax_t;
                        tenths = r10_0.wrapping_div(base) as libc::c_int;
                        rounding = if r2_0 < base {
                            (r2_0.wrapping_add(rounding as libc::c_uint)
                                != 0 as libc::c_int as libc::c_uint) as libc::c_int
                        } else {
                            2 as libc::c_int
                                + (base < r2_0.wrapping_add(rounding as libc::c_uint))
                                    as libc::c_int
                        };
                        exponent += 1;
                        exponent;
                        if !(base as libc::c_ulong <= amt && exponent < exponent_max) {
                            break;
                        }
                    }
                    if amt < 10 as libc::c_int as libc::c_ulong {
                        if if inexact_style == human_round_to_nearest as libc::c_int {
                            ((2 as libc::c_int) < rounding + (tenths & 1 as libc::c_int))
                                as libc::c_int
                        } else {
                            (inexact_style == human_ceiling as libc::c_int
                                && (0 as libc::c_int) < rounding) as libc::c_int
                        } != 0
                        {
                            tenths += 1;
                            tenths;
                            rounding = 0 as libc::c_int;
                            if tenths == 10 as libc::c_int {
                                amt = amt.wrapping_add(1);
                                amt;
                                tenths = 0 as libc::c_int;
                            }
                        }
                        if amt < 10 as libc::c_int as libc::c_ulong
                            && (tenths != 0
                                || opts & human_suppress_point_zero as libc::c_int == 0)
                        {
                            p = p.offset(-1);
                            *p = ('0' as i32 + tenths) as libc::c_char;
                            p = p.offset(-(decimal_pointlen as isize));
                            memcpy(
                                p as *mut libc::c_void,
                                decimal_point as *const libc::c_void,
                                decimal_pointlen,
                            );
                            rounding = 0 as libc::c_int;
                            tenths = rounding;
                        }
                    }
                }
            }
            if if inexact_style == human_round_to_nearest as libc::c_int {
                ((5 as libc::c_int)
                    < tenths
                        + ((0 as libc::c_int as libc::c_ulong)
                            < (rounding as libc::c_ulong)
                                .wrapping_add(amt & 1 as libc::c_int as libc::c_ulong))
                            as libc::c_int) as libc::c_int
            } else {
                (inexact_style == human_ceiling as libc::c_int
                    && (0 as libc::c_int) < tenths + rounding) as libc::c_int
            } != 0
            {
                amt = amt.wrapping_add(1);
                amt;
                if opts & human_autoscale as libc::c_int != 0
                    && amt == base as libc::c_ulong && exponent < exponent_max
                {
                    exponent += 1;
                    exponent;
                    if opts & human_suppress_point_zero as libc::c_int == 0 {
                        p = p.offset(-1);
                        *p = '0' as i32 as libc::c_char;
                        p = p.offset(-(decimal_pointlen as isize));
                        memcpy(
                            p as *mut libc::c_void,
                            decimal_point as *const libc::c_void,
                            decimal_pointlen,
                        );
                    }
                    amt = 1 as libc::c_int as uintmax_t;
                }
            }
            integerlim = p;
            loop {
                let mut digit: libc::c_int = amt
                    .wrapping_rem(10 as libc::c_int as libc::c_ulong) as libc::c_int;
                p = p.offset(-1);
                *p = (digit + '0' as i32) as libc::c_char;
                amt = (amt as libc::c_ulong)
                    .wrapping_div(10 as libc::c_int as libc::c_ulong) as uintmax_t
                    as uintmax_t;
                if !(amt != 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
            }
        }
    }
    if opts & human_group_digits as libc::c_int != 0 {
        p = group_number(
            p,
            integerlim.offset_from(p) as libc::c_long as size_t,
            grouping,
            thousands_sep,
        );
    }
    if opts & human_SI as libc::c_int != 0 {
        if exponent < 0 as libc::c_int {
            let mut power: uintmax_t = 0;
            exponent = 0 as libc::c_int;
            power = 1 as libc::c_int as uintmax_t;
            while power < to_block_size {
                exponent += 1;
                if exponent == exponent_max {
                    break;
                }
                power = (power as libc::c_ulong).wrapping_mul(base as libc::c_ulong)
                    as uintmax_t as uintmax_t;
            }
        }
        if exponent | opts & human_B as libc::c_int != 0
            && opts & human_space_before_unit as libc::c_int != 0
        {
            let fresh0 = psuffix;
            psuffix = psuffix.offset(1);
            *fresh0 = ' ' as i32 as libc::c_char;
        }
        if exponent != 0 {
            let fresh1 = psuffix;
            psuffix = psuffix.offset(1);
            *fresh1 = (if opts & human_base_1024 as libc::c_int == 0
                && exponent == 1 as libc::c_int
            {
                'k' as i32
            } else {
                power_letter[exponent as usize] as libc::c_int
            }) as libc::c_char;
        }
        if opts & human_B as libc::c_int != 0 {
            if opts & human_base_1024 as libc::c_int != 0 && exponent != 0 {
                let fresh2 = psuffix;
                psuffix = psuffix.offset(1);
                *fresh2 = 'i' as i32 as libc::c_char;
            }
            let fresh3 = psuffix;
            psuffix = psuffix.offset(1);
            *fresh3 = 'B' as i32 as libc::c_char;
        }
    }
    *psuffix = '\0' as i32 as libc::c_char;
    return p;
}
static mut block_size_args: [*const libc::c_char; 3] = [
    b"human-readable\0" as *const u8 as *const libc::c_char,
    b"si\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut block_size_opts: [libc::c_int; 2] = [
    human_autoscale as libc::c_int + human_SI as libc::c_int
        + human_base_1024 as libc::c_int,
    human_autoscale as libc::c_int + human_SI as libc::c_int,
];
unsafe extern "C" fn default_block_size() -> uintmax_t {
    return (if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        512 as libc::c_int
    } else {
        1024 as libc::c_int
    }) as uintmax_t;
}
unsafe extern "C" fn humblock(
    mut spec: *const libc::c_char,
    mut block_size: *mut uintmax_t,
    mut options: *mut libc::c_int,
) -> strtol_error {
    let mut i: libc::c_int = 0;
    let mut opts: libc::c_int = 0 as libc::c_int;
    if spec.is_null()
        && {
            spec = getenv(b"BLOCK_SIZE\0" as *const u8 as *const libc::c_char);
            spec.is_null()
        }
        && {
            spec = getenv(b"BLOCKSIZE\0" as *const u8 as *const libc::c_char);
            spec.is_null()
        }
    {
        *block_size = default_block_size();
    } else {
        if *spec as libc::c_int == '\'' as i32 {
            opts |= human_group_digits as libc::c_int;
            spec = spec.offset(1);
            spec;
        }
        i = argmatch(
            spec,
            block_size_args.as_ptr(),
            block_size_opts.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as libc::c_int;
        if 0 as libc::c_int <= i {
            opts |= block_size_opts[i as usize];
            *block_size = 1 as libc::c_int as uintmax_t;
        } else {
            let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut e: strtol_error = xstrtoumax(
                spec,
                &mut ptr,
                0 as libc::c_int,
                block_size,
                b"eEgGkKmMpPtTyYzZ0\0" as *const u8 as *const libc::c_char,
            );
            if e as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
                *options = 0 as libc::c_int;
                return e;
            }
            while !('0' as i32 <= *spec as libc::c_int
                && *spec as libc::c_int <= '9' as i32)
            {
                if spec == ptr as *const libc::c_char {
                    opts |= human_SI as libc::c_int;
                    if *ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == 'B' as i32
                    {
                        opts |= human_B as libc::c_int;
                    }
                    if *ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        != 'B' as i32
                        || *ptr.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == 'i' as i32
                    {
                        opts |= human_base_1024 as libc::c_int;
                    }
                    break;
                } else {
                    spec = spec.offset(1);
                    spec;
                }
            }
        }
    }
    *options = opts;
    return LONGINT_OK;
}
pub unsafe extern "C" fn human_options(
    mut spec: *const libc::c_char,
    mut opts: *mut libc::c_int,
    mut block_size: *mut uintmax_t,
) -> strtol_error {
    let mut e: strtol_error = humblock(spec, block_size, opts);
    if *block_size == 0 as libc::c_int as libc::c_ulong {
        *block_size = default_block_size();
        e = LONGINT_INVALID;
    }
    return e;
}
