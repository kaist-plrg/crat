use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type small_t = libc::c_uchar;
pub const sa_alignment_max: C2RustUnnamed = 16;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed = libc::c_uint;
pub const sa_alignment_longdouble: C2RustUnnamed = 16;
pub const sa_alignment_longlong: C2RustUnnamed = 8;
pub const sa_alignment_double: C2RustUnnamed = 8;
pub const sa_alignment_long: C2RustUnnamed = 8;
pub unsafe extern "C" fn mmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut alignment2_mask: uintptr_t = (2 as libc::c_int
        * sa_alignment_max as libc::c_int - 1 as libc::c_int) as uintptr_t;
    let mut plus: libc::c_int = (::std::mem::size_of::<small_t>() as libc::c_ulong)
        .wrapping_add(alignment2_mask) as libc::c_int;
    let mut nplus: idx_t = 0;
    let (fresh0, fresh1) = n.overflowing_add(plus as _);
    *(&mut nplus as *mut idx_t) = fresh0 as _;
    if !fresh1
        && !(1 as libc::c_int != 0 as libc::c_int
            && (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
                .wrapping_div(1 as libc::c_int as libc::c_ulong)
                < nplus as libc::c_ulong)
    {
        let mut mem: *mut libc::c_char = malloc(nplus as libc::c_ulong)
            as *mut libc::c_char;
        if !mem.is_null() {
            let mut umem: uintptr_t = mem as uintptr_t;
            let mut umemplus: uintptr_t = 0;
            let (fresh2, fresh3) = umem
                .overflowing_add(
                    (::std::mem::size_of::<small_t>() as libc::c_ulong)
                        .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            *&mut umemplus = fresh2 as _;
            let mut offset: idx_t = (umemplus & !alignment2_mask)
                .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                .wrapping_sub(umem) as idx_t;
            let mut vp: *mut libc::c_void = mem.offset(offset as isize)
                as *mut libc::c_void;
            let mut p: *mut small_t = vp as *mut small_t;
            *p.offset(-(1 as libc::c_int) as isize) = offset as small_t;
            return p as *mut libc::c_void;
        }
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn freea(mut p: *mut libc::c_void) {
    if p as uintptr_t
        & (sa_alignment_max as libc::c_int - 1 as libc::c_int) as libc::c_ulong != 0
    {
        abort();
    }
    if p as uintptr_t & sa_alignment_max as libc::c_int as libc::c_ulong != 0 {
        let mut mem: *mut libc::c_void = (p as *mut libc::c_char)
            .offset(
                -(*(p as *mut small_t).offset(-(1 as libc::c_int) as isize)
                    as libc::c_int as isize),
            ) as *mut libc::c_void;
        rpl_free(mem);
    }
}
