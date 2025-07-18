use ::libc;
pub type cpFloat = libc::c_double;
pub type cpBool = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
pub unsafe extern "C" fn cpCheckPointGreater(a: cpVect, b: cpVect, c: cpVect) -> cpBool {
    return ((b.y - a.y) * (a.x + b.x - 2 as libc::c_int as libc::c_double * c.x)
        > (b.x - a.x) * (a.y + b.y - 2 as libc::c_int as libc::c_double * c.y))
        as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpCheckAxis(
    mut v0: cpVect,
    mut v1: cpVect,
    mut p: cpVect,
    mut n: cpVect,
) -> cpBool {
    return (cpvdot(p, n) <= cpfmax(cpvdot(v0, n), cpvdot(v1, n))) as libc::c_int
        as cpBool;
}
