use ::libc;
use std::arch::asm;
extern "C" {
    fn frexpl(_: f128::f128, _: *mut libc::c_int) -> f128::f128;
    fn ldexpl(_: f128::f128, _: libc::c_int) -> f128::f128;
}
pub type fpucw_t = libc::c_ushort;
pub unsafe extern "C" fn printf_frexpl(
    mut x: f128::f128,
    mut expptr: *mut libc::c_int,
) -> f128::f128 {
    let mut exponent: libc::c_int = 0;
    let mut oldcw: fpucw_t = 0;
    oldcw = ({
        let mut _cw: fpucw_t = 0;
        asm!("fnstcw [{0}]", in (reg) & mut _cw, options(preserves_flags));
        _cw
    });
    let mut _ncw: fpucw_t = (oldcw as libc::c_int & !(0x300 as libc::c_int)
        | 0x300 as libc::c_int) as fpucw_t;
    asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    'c_149: {
        let mut _ncw: fpucw_t = (oldcw as libc::c_int & !(0x300 as libc::c_int)
            | 0x300 as libc::c_int) as fpucw_t;
        asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    };
    x = frexpl(x, &mut exponent);
    x = x + x;
    exponent -= 1 as libc::c_int;
    if exponent < -(16381 as libc::c_int) - 1 as libc::c_int {
        x = ldexpl(x, exponent - (-(16381 as libc::c_int) - 1 as libc::c_int));
        exponent = -(16381 as libc::c_int) - 1 as libc::c_int;
    }
    let mut _ncw: fpucw_t = oldcw;
    asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    'c_59: {
        let mut _ncw: fpucw_t = oldcw;
        asm!("fldcw [{0}]", in (reg) & _ncw, options(preserves_flags));
    };
    *expptr = exponent;
    return x;
}
