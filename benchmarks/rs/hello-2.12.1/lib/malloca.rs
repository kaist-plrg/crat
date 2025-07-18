use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub const sa_alignment_max: C2RustUnnamed = 16;
pub type small_t = libc::c_uchar;
pub type C2RustUnnamed = libc::c_uint;
pub const sa_alignment_longdouble: C2RustUnnamed = 16;
pub const sa_alignment_longlong: C2RustUnnamed = 8;
pub const sa_alignment_double: C2RustUnnamed = 8;
pub const sa_alignment_long: C2RustUnnamed = 8;
pub unsafe extern "C" fn mmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut nplus: size_t = n
        .wrapping_add(::std::mem::size_of::<small_t>() as libc::c_ulong)
        .wrapping_add(
            (2 as libc::c_int * sa_alignment_max as libc::c_int) as libc::c_ulong,
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if nplus >= n {
        let mut mem: *mut libc::c_char = malloc(nplus) as *mut libc::c_char;
        if !mem.is_null() {
            let mut p: *mut libc::c_char = ((mem as uintptr_t)
                .wrapping_add(::std::mem::size_of::<small_t>() as libc::c_ulong)
                .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !((2 as libc::c_int * sa_alignment_max as libc::c_int
                    - 1 as libc::c_int) as uintptr_t))
                .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                as *mut libc::c_char;
            *(p as *mut small_t)
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = p.offset_from(mem) as libc::c_long as small_t;
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
        free(mem);
    }
}
