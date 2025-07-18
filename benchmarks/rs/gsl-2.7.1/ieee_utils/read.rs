use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const GSL_IEEE_EXTENDED_PRECISION: C2RustUnnamed_0 = 3;
pub const GSL_IEEE_DOUBLE_PRECISION: C2RustUnnamed_0 = 2;
pub const GSL_IEEE_SINGLE_PRECISION: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const GSL_IEEE_ROUND_TO_ZERO: C2RustUnnamed_1 = 4;
pub const GSL_IEEE_ROUND_UP: C2RustUnnamed_1 = 3;
pub const GSL_IEEE_ROUND_DOWN: C2RustUnnamed_1 = 2;
pub const GSL_IEEE_ROUND_TO_NEAREST: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const GSL_IEEE_TRAP_INEXACT: C2RustUnnamed_2 = 32;
pub const GSL_IEEE_MASK_ALL: C2RustUnnamed_2 = 31;
pub const GSL_IEEE_MASK_UNDERFLOW: C2RustUnnamed_2 = 16;
pub const GSL_IEEE_MASK_OVERFLOW: C2RustUnnamed_2 = 8;
pub const GSL_IEEE_MASK_DIVISION_BY_ZERO: C2RustUnnamed_2 = 4;
pub const GSL_IEEE_MASK_DENORMALIZED: C2RustUnnamed_2 = 2;
pub const GSL_IEEE_MASK_INVALID: C2RustUnnamed_2 = 1;
pub unsafe extern "C" fn gsl_ieee_read_mode_string(
    mut description: *const libc::c_char,
    mut precision: *mut libc::c_int,
    mut rounding: *mut libc::c_int,
    mut exception_mask: *mut libc::c_int,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut precision_count: libc::c_int = 0 as libc::c_int;
    let mut rounding_count: libc::c_int = 0 as libc::c_int;
    let mut exception_count: libc::c_int = 0 as libc::c_int;
    start = malloc((strlen(description)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if start.is_null() {
        gsl_error(
            b"no memory to parse mode string\0" as *const u8 as *const libc::c_char,
            b"read.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return GSL_ENOMEM as libc::c_int;
    }
    strcpy(start, description);
    p = start;
    *precision = 0 as libc::c_int;
    *rounding = 0 as libc::c_int;
    *exception_mask = 0 as libc::c_int;
    loop {
        let mut status: libc::c_int = 0;
        let mut new_precision: libc::c_int = 0;
        let mut new_rounding: libc::c_int = 0;
        let mut new_exception: libc::c_int = 0;
        end = strchr(p, ',' as i32);
        if !end.is_null() {
            *end = '\0' as i32 as libc::c_char;
            loop {
                end = end.offset(1);
                end;
                if !(*end as libc::c_int == ' ' as i32
                    || *end as libc::c_int == ',' as i32)
                {
                    break;
                }
            }
        }
        new_precision = 0 as libc::c_int;
        new_rounding = 0 as libc::c_int;
        new_exception = 0 as libc::c_int;
        status = lookup_string(
            p,
            &mut new_precision,
            &mut new_rounding,
            &mut new_exception,
        );
        if status != 0 {
            free(start as *mut libc::c_void);
            gsl_error(
                b"unrecognized GSL_IEEE_MODE string.\nValid settings are:\n\n  single-precision double-precision extended-precision\n  round-to-nearest round-down round-up round-to-zero\n  mask-invalid mask-denormalized mask-division-by-zero\n  mask-overflow mask-underflow mask-all\n  trap-common trap-inexact\n\nseparated by commas. (e.g. GSL_IEEE_MODE=\"round-down,mask-underflow\")\0"
                    as *const u8 as *const libc::c_char,
                b"read.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        if new_precision != 0 {
            *precision = new_precision;
            precision_count += 1;
            precision_count;
            if precision_count > 1 as libc::c_int {
                free(start as *mut libc::c_void);
                gsl_error(
                    b"attempted to set IEEE precision twice\0" as *const u8
                        as *const libc::c_char,
                    b"read.c\0" as *const u8 as *const libc::c_char,
                    103 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
        }
        if new_rounding != 0 {
            *rounding = new_rounding;
            rounding_count += 1;
            rounding_count;
            if rounding_count > 1 as libc::c_int {
                free(start as *mut libc::c_void);
                gsl_error(
                    b"attempted to set IEEE rounding mode twice\0" as *const u8
                        as *const libc::c_char,
                    b"read.c\0" as *const u8 as *const libc::c_char,
                    114 as libc::c_int,
                    GSL_EINVAL as libc::c_int,
                );
                return GSL_EINVAL as libc::c_int;
            }
        }
        if new_exception != 0 {
            *exception_mask |= new_exception;
            exception_count += 1;
            exception_count;
        }
        p = end;
        if !(!end.is_null() && *p as libc::c_int != '\0' as i32) {
            break;
        }
    }
    free(start as *mut libc::c_void);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn lookup_string(
    mut p: *const libc::c_char,
    mut precision: *mut libc::c_int,
    mut rounding: *mut libc::c_int,
    mut exception_mask: *mut libc::c_int,
) -> libc::c_int {
    if strcmp(p, b"single-precision\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *precision = GSL_IEEE_SINGLE_PRECISION as libc::c_int;
    } else if strcmp(p, b"double-precision\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *precision = GSL_IEEE_DOUBLE_PRECISION as libc::c_int;
    } else if strcmp(p, b"extended-precision\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *precision = GSL_IEEE_EXTENDED_PRECISION as libc::c_int;
    } else if strcmp(p, b"round-to-nearest\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *rounding = GSL_IEEE_ROUND_TO_NEAREST as libc::c_int;
    } else if strcmp(p, b"round-down\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *rounding = GSL_IEEE_ROUND_DOWN as libc::c_int;
    } else if strcmp(p, b"round-up\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *rounding = GSL_IEEE_ROUND_UP as libc::c_int;
    } else if strcmp(p, b"round-to-zero\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *rounding = GSL_IEEE_ROUND_TO_ZERO as libc::c_int;
    } else if strcmp(p, b"mask-all\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *exception_mask = GSL_IEEE_MASK_ALL as libc::c_int;
    } else if strcmp(p, b"mask-invalid\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *exception_mask = GSL_IEEE_MASK_INVALID as libc::c_int;
    } else if strcmp(p, b"mask-denormalized\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *exception_mask = GSL_IEEE_MASK_DENORMALIZED as libc::c_int;
    } else if strcmp(p, b"mask-division-by-zero\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *exception_mask = GSL_IEEE_MASK_DIVISION_BY_ZERO as libc::c_int;
    } else if strcmp(p, b"mask-overflow\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *exception_mask = GSL_IEEE_MASK_OVERFLOW as libc::c_int;
    } else if strcmp(p, b"mask-underflow\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *exception_mask = GSL_IEEE_MASK_UNDERFLOW as libc::c_int;
    } else if strcmp(p, b"trap-inexact\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        *exception_mask = GSL_IEEE_TRAP_INEXACT as libc::c_int;
    } else if strcmp(p, b"trap-common\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
