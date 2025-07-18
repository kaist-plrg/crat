use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
pub unsafe extern "C" fn gsl_permutation_size(mut p: *const gsl_permutation) -> size_t {
    return (*p).size;
}
pub unsafe extern "C" fn gsl_permutation_data(
    mut p: *const gsl_permutation,
) -> *mut size_t {
    return (*p).data;
}
pub unsafe extern "C" fn gsl_permutation_swap(
    mut p: *mut gsl_permutation,
    i: size_t,
    j: size_t,
) -> libc::c_int {
    let size: size_t = (*p).size;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const libc::c_char,
            b"permutation.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const libc::c_char,
            b"permutation.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if i != j {
        let mut tmp: size_t = *((*p).data).offset(i as isize);
        *((*p).data).offset(i as isize) = *((*p).data).offset(j as isize);
        *((*p).data).offset(j as isize) = tmp;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_valid(
    mut p: *const gsl_permutation,
) -> libc::c_int {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size {
        if *((*p).data).offset(i as isize) >= size {
            gsl_error(
                b"permutation index outside range\0" as *const u8 as *const libc::c_char,
                b"permutation.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int,
                GSL_FAILURE as libc::c_int,
            );
            return GSL_FAILURE as libc::c_int;
        }
        j = 0 as libc::c_int as size_t;
        while j < i {
            if *((*p).data).offset(i as isize) == *((*p).data).offset(j as isize) {
                gsl_error(
                    b"duplicate permutation index\0" as *const u8 as *const libc::c_char,
                    b"permutation.c\0" as *const u8 as *const libc::c_char,
                    80 as libc::c_int,
                    GSL_FAILURE as libc::c_int,
                );
                return GSL_FAILURE as libc::c_int;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_reverse(mut p: *mut gsl_permutation) {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < size.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut j: size_t = size
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut tmp: size_t = *((*p).data).offset(i as isize);
        *((*p).data).offset(i as isize) = *((*p).data).offset(j as isize);
        *((*p).data).offset(j as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn gsl_permutation_inverse(
    mut inv: *mut gsl_permutation,
    mut p: *const gsl_permutation,
) -> libc::c_int {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    if (*inv).size != size {
        gsl_error(
            b"permutation lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"permutation.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        *((*inv).data).offset(*((*p).data).offset(i as isize) as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_next(
    mut p: *mut gsl_permutation,
) -> libc::c_int {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size < 2 as libc::c_int as libc::c_ulong {
        return GSL_FAILURE as libc::c_int;
    }
    i = size.wrapping_sub(2 as libc::c_int as libc::c_ulong);
    while *((*p).data).offset(i as isize)
        > *((*p).data).offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        && i != 0 as libc::c_int as libc::c_ulong
    {
        i = i.wrapping_sub(1);
        i;
    }
    if i == 0 as libc::c_int as libc::c_ulong
        && *((*p).data).offset(0 as libc::c_int as isize)
            > *((*p).data).offset(1 as libc::c_int as isize)
    {
        return GSL_FAILURE as libc::c_int;
    }
    k = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
    j = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
    while j < size {
        if *((*p).data).offset(j as isize) > *((*p).data).offset(i as isize)
            && *((*p).data).offset(j as isize) < *((*p).data).offset(k as isize)
        {
            k = j;
        }
        j = j.wrapping_add(1);
        j;
    }
    let mut tmp: size_t = *((*p).data).offset(i as isize);
    *((*p).data).offset(i as isize) = *((*p).data).offset(k as isize);
    *((*p).data).offset(k as isize) = tmp;
    j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while j <= size.wrapping_add(i).wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut tmp_0: size_t = *((*p).data).offset(j as isize);
        *((*p).data)
            .offset(
                j as isize,
            ) = *((*p).data).offset(size.wrapping_add(i).wrapping_sub(j) as isize);
        *((*p).data).offset(size.wrapping_add(i).wrapping_sub(j) as isize) = tmp_0;
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_prev(
    mut p: *mut gsl_permutation,
) -> libc::c_int {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size < 2 as libc::c_int as libc::c_ulong {
        return GSL_FAILURE as libc::c_int;
    }
    i = size.wrapping_sub(2 as libc::c_int as libc::c_ulong);
    while *((*p).data).offset(i as isize)
        < *((*p).data).offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        && i != 0 as libc::c_int as libc::c_ulong
    {
        i = i.wrapping_sub(1);
        i;
    }
    if i == 0 as libc::c_int as libc::c_ulong
        && *((*p).data).offset(0 as libc::c_int as isize)
            < *((*p).data).offset(1 as libc::c_int as isize)
    {
        return GSL_FAILURE as libc::c_int;
    }
    k = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
    j = i.wrapping_add(2 as libc::c_int as libc::c_ulong);
    while j < size {
        if *((*p).data).offset(j as isize) < *((*p).data).offset(i as isize)
            && *((*p).data).offset(j as isize) > *((*p).data).offset(k as isize)
        {
            k = j;
        }
        j = j.wrapping_add(1);
        j;
    }
    let mut tmp: size_t = *((*p).data).offset(i as isize);
    *((*p).data).offset(i as isize) = *((*p).data).offset(k as isize);
    *((*p).data).offset(k as isize) = tmp;
    j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while j <= size.wrapping_add(i).wrapping_div(2 as libc::c_int as libc::c_ulong) {
        let mut tmp_0: size_t = *((*p).data).offset(j as isize);
        *((*p).data)
            .offset(
                j as isize,
            ) = *((*p).data).offset(size.wrapping_add(i).wrapping_sub(j) as isize);
        *((*p).data).offset(size.wrapping_add(i).wrapping_sub(j) as isize) = tmp_0;
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_mul(
    mut p: *mut gsl_permutation,
    mut pa: *const gsl_permutation,
    mut pb: *const gsl_permutation,
) -> libc::c_int {
    let mut i: size_t = 0;
    let size: size_t = (*p).size;
    if (*pa).size != size {
        gsl_error(
            b"size of result does not match size of pa\0" as *const u8
                as *const libc::c_char,
            b"permutation.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    if (*pb).size != size {
        gsl_error(
            b"size of result does not match size of pb\0" as *const u8
                as *const libc::c_char,
            b"permutation.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        *((*p).data)
            .offset(
                i as isize,
            ) = *((*pb).data).offset(*((*pa).data).offset(i as isize) as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_permutation_memcpy(
    mut dest: *mut gsl_permutation,
    mut src: *const gsl_permutation,
) -> libc::c_int {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"permutation lengths are not equal\0" as *const u8 as *const libc::c_char,
            b"permutation.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    }
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < src_size {
        *((*dest).data).offset(j as isize) = *((*src).data).offset(j as isize);
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as libc::c_int;
}
