use ::libc;
use std::arch::asm;
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
pub unsafe extern "C" fn gsl_ieee_set_mode(
    mut precision: libc::c_int,
    mut rounding: libc::c_int,
    mut exception_mask: libc::c_int,
) -> libc::c_int {
    let mut mode: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    match precision {
        1 => {
            mode = (mode as libc::c_int | 0 as libc::c_int) as libc::c_ushort;
        }
        2 => {
            mode = (mode as libc::c_int | 0x200 as libc::c_int) as libc::c_ushort;
        }
        3 => {
            mode = (mode as libc::c_int | 0x300 as libc::c_int) as libc::c_ushort;
        }
        _ => {
            mode = (mode as libc::c_int | 0x300 as libc::c_int) as libc::c_ushort;
        }
    }
    match rounding {
        1 => {
            mode = (mode as libc::c_int | 0 as libc::c_int) as libc::c_ushort;
        }
        2 => {
            mode = (mode as libc::c_int | 0x400 as libc::c_int) as libc::c_ushort;
        }
        3 => {
            mode = (mode as libc::c_int | 0x800 as libc::c_int) as libc::c_ushort;
        }
        4 => {
            mode = (mode as libc::c_int | 0xc00 as libc::c_int) as libc::c_ushort;
        }
        _ => {
            mode = (mode as libc::c_int | 0 as libc::c_int) as libc::c_ushort;
        }
    }
    if exception_mask & GSL_IEEE_MASK_INVALID as libc::c_int != 0 {
        mode = (mode as libc::c_int | 0x1 as libc::c_int) as libc::c_ushort;
    }
    if exception_mask & GSL_IEEE_MASK_DENORMALIZED as libc::c_int != 0 {
        mode = (mode as libc::c_int | 0x2 as libc::c_int) as libc::c_ushort;
    }
    if exception_mask & GSL_IEEE_MASK_DIVISION_BY_ZERO as libc::c_int != 0 {
        mode = (mode as libc::c_int | 0x4 as libc::c_int) as libc::c_ushort;
    }
    if exception_mask & GSL_IEEE_MASK_OVERFLOW as libc::c_int != 0 {
        mode = (mode as libc::c_int | 0x8 as libc::c_int) as libc::c_ushort;
    }
    if exception_mask & GSL_IEEE_MASK_UNDERFLOW as libc::c_int != 0 {
        mode = (mode as libc::c_int | 0x10 as libc::c_int) as libc::c_ushort;
    }
    if exception_mask & GSL_IEEE_TRAP_INEXACT as libc::c_int != 0 {
        mode = (mode as libc::c_int & !(0x20 as libc::c_int)) as libc::c_ushort;
    } else {
        mode = (mode as libc::c_int | 0x20 as libc::c_int) as libc::c_ushort;
    }
    asm!("fldcw [{0}]", in (reg) & mode, options(preserves_flags));
    let mut mode_sse: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    mode_sse
        |= ((mode as libc::c_int & 0x3f as libc::c_int) << 7 as libc::c_int)
            as libc::c_uint;
    mode_sse
        |= ((mode as libc::c_int & 0xc00 as libc::c_int) << 3 as libc::c_int)
            as libc::c_uint;
    asm!("ldmxcsr [{0}]", in (reg) & mode_sse, options(preserves_flags));
    return GSL_SUCCESS as libc::c_int;
}
