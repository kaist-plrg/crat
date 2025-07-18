use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
pub type UUCONF_SIZE_T = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblock {
    pub qnext: *mut sblock,
    pub ifree: size_t,
    pub plast: pointer,
    pub qadded: *mut sadded,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ab: [libc::c_char; 1008],
    pub l: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sadded {
    pub qnext: *mut sadded,
    pub padded: pointer,
}
pub static mut _uuconf_alloc_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: alloc.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_malloc(
    mut pblock: pointer,
    mut c: size_t,
) -> *mut libc::c_void {
    let mut q: *mut sblock = pblock as *mut sblock;
    let mut pret: pointer = 0 as *mut libc::c_void;
    if c == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    if q.is_null() {
        return malloc(c);
    }
    c = c
        .wrapping_add(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong);
    while ((*q).ifree).wrapping_add(c) > 1008 as libc::c_int as libc::c_ulong {
        if !((*q).qnext).is_null() {
            q = (*q).qnext;
        } else {
            if c > 1008 as libc::c_int as libc::c_ulong {
                (*q)
                    .qnext = malloc(
                    (::std::mem::size_of::<sblock>() as libc::c_ulong)
                        .wrapping_add(c)
                        .wrapping_sub(1008 as libc::c_int as libc::c_ulong),
                ) as *mut sblock;
            } else {
                (*q)
                    .qnext = malloc(::std::mem::size_of::<sblock>() as libc::c_ulong)
                    as *mut sblock;
            }
            if ((*q).qnext).is_null() {
                return 0 as *mut libc::c_void;
            }
            q = (*q).qnext;
            (*q).qnext = 0 as *mut sblock;
            (*q).ifree = 0 as libc::c_int as size_t;
            (*q).qadded = 0 as *mut sadded;
            break;
        }
    }
    pret = ((*q).u.ab).as_mut_ptr().offset((*q).ifree as isize) as pointer;
    (*q).ifree = ((*q).ifree as libc::c_ulong).wrapping_add(c) as size_t as size_t;
    (*q).plast = pret;
    return pret;
}
