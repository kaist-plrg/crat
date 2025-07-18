use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn _glp_wclique(
        n: libc::c_int,
        w: *const libc::c_int,
        a: *const libc::c_uchar,
        ind: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_graph {
    pub pool: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub nv_max: libc::c_int,
    pub nv: libc::c_int,
    pub na: libc::c_int,
    pub v: *mut *mut glp_vertex,
    pub index: *mut libc::c_void,
    pub v_size: libc::c_int,
    pub a_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_vertex {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub entry: *mut libc::c_void,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub in_0: *mut glp_arc,
    pub out: *mut glp_arc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_arc {
    pub tail: *mut glp_vertex,
    pub head: *mut glp_vertex,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub t_prev: *mut glp_arc,
    pub t_next: *mut glp_arc,
    pub h_prev: *mut glp_arc,
    pub h_next: *mut glp_arc,
}
unsafe extern "C" fn set_edge(
    mut nv: libc::c_int,
    mut a: *mut libc::c_uchar,
    mut i: libc::c_int,
    mut j: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    (1 as libc::c_int <= j && j < i && i <= nv
        || {
            glp_assert_(
                b"1 <= j && j < i && i <= nv\0" as *const u8 as *const libc::c_char,
                b"api/wcliqex.c\0" as *const u8 as *const libc::c_char,
                28 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k = (i - 1 as libc::c_int) * (i - 2 as libc::c_int) / 2 as libc::c_int
        + (j - 1 as libc::c_int);
    let ref mut fresh0 = *a.offset((k / 8 as libc::c_int) as isize);
    *fresh0 = (*fresh0 as libc::c_int
        | ((1 as libc::c_int)
            << 8 as libc::c_int - 1 as libc::c_int - k % 8 as libc::c_int)
            as libc::c_uchar as libc::c_int) as libc::c_uchar;
}
pub unsafe extern "C" fn glp_wclique_exact(
    mut G: *mut glp_graph,
    mut v_wgt: libc::c_int,
    mut sol: *mut libc::c_double,
    mut v_set: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut e: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut a: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    if v_wgt >= 0 as libc::c_int
        && v_wgt
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/wcliqex.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
        ))
            .unwrap()(
            b"glp_wclique_exact: v_wgt = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            v_wgt,
        );
    }
    if v_set >= 0 as libc::c_int
        && v_set
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/wcliqex.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
        ))
            .unwrap()(
            b"glp_wclique_exact: v_set = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            v_set,
        );
    }
    if (*G).nv == 0 as libc::c_int {
        if !sol.is_null() {
            *sol = 0.0f64;
        }
        return 0 as libc::c_int;
    }
    w = glp_alloc(
        1 as libc::c_int + (*G).nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    ind = glp_alloc(
        1 as libc::c_int + (*G).nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    len = (*G).nv;
    len = len * (len - 1 as libc::c_int) / 2 as libc::c_int;
    len = (len + (8 as libc::c_int - 1 as libc::c_int)) / 8 as libc::c_int;
    a = glp_alloc(
        len,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_uchar;
    memset(
        a as *mut libc::c_void,
        0 as libc::c_int,
        (len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    s = 0.0f64;
    i = 1 as libc::c_int;
    loop {
        if !(i <= (*G).nv) {
            current_block = 15089075282327824602;
            break;
        }
        if v_wgt >= 0 as libc::c_int {
            memcpy(
                &mut t as *mut libc::c_double as *mut libc::c_void,
                ((**((*G).v).offset(i as isize)).data as *mut libc::c_char)
                    .offset(v_wgt as isize) as *const libc::c_void,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            );
            if !(0.0f64 <= t && t <= 2147483647 as libc::c_int as libc::c_double
                && t == floor(t))
            {
                ret = 0x12 as libc::c_int;
                current_block = 6155649320532290099;
                break;
            } else {
                *w.offset(i as isize) = t as libc::c_int;
            }
        } else {
            *w.offset(i as isize) = 1 as libc::c_int;
        }
        s += *w.offset(i as isize) as libc::c_double;
        i += 1;
        i;
    }
    match current_block {
        15089075282327824602 => {
            if s > 2147483647 as libc::c_int as libc::c_double {
                ret = 0x12 as libc::c_int;
            } else {
                i = 1 as libc::c_int;
                while i <= (*G).nv {
                    e = (**((*G).v).offset(i as isize)).in_0;
                    while !e.is_null() {
                        j = (*(*e).tail).i;
                        if i > j {
                            set_edge((*G).nv, a, i, j);
                        }
                        e = (*e).h_next;
                    }
                    e = (**((*G).v).offset(i as isize)).out;
                    while !e.is_null() {
                        j = (*(*e).head).i;
                        if i > j {
                            set_edge((*G).nv, a, i, j);
                        }
                        e = (*e).t_next;
                    }
                    i += 1;
                    i;
                }
                len = _glp_wclique(
                    (*G).nv,
                    w as *const libc::c_int,
                    a as *const libc::c_uchar,
                    ind,
                );
                s = 0.0f64;
                k = 1 as libc::c_int;
                while k <= len {
                    i = *ind.offset(k as isize);
                    (1 as libc::c_int <= i && i <= (*G).nv
                        || {
                            glp_assert_(
                                b"1 <= i && i <= G->nv\0" as *const u8
                                    as *const libc::c_char,
                                b"api/wcliqex.c\0" as *const u8 as *const libc::c_char,
                                98 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    s += *w.offset(i as isize) as libc::c_double;
                    k += 1;
                    k;
                }
                if !sol.is_null() {
                    *sol = s;
                }
                if v_set >= 0 as libc::c_int {
                    x = 0 as libc::c_int;
                    i = 1 as libc::c_int;
                    while i <= (*G).nv {
                        memcpy(
                            ((**((*G).v).offset(i as isize)).data as *mut libc::c_char)
                                .offset(v_set as isize) as *mut libc::c_void,
                            &mut x as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        i += 1;
                        i;
                    }
                    x = 1 as libc::c_int;
                    k = 1 as libc::c_int;
                    while k <= len {
                        i = *ind.offset(k as isize);
                        memcpy(
                            ((**((*G).v).offset(i as isize)).data as *mut libc::c_char)
                                .offset(v_set as isize) as *mut libc::c_void,
                            &mut x as *mut libc::c_int as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        );
                        k += 1;
                        k;
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(w as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(a as *mut libc::c_void);
    return ret;
}
