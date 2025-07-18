use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_check_asnprob(G: *mut glp_graph, v_set: libc::c_int) -> libc::c_int;
    fn _glp_mc21a(
        n: libc::c_int,
        icn: *const libc::c_int,
        ip: *const libc::c_int,
        lenr: *const libc::c_int,
        iperm: *mut libc::c_int,
        pr: *mut libc::c_int,
        arp: *mut libc::c_int,
        cv: *mut libc::c_int,
        out: *mut libc::c_int,
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
pub unsafe extern "C" fn glp_asnprob_hall(
    mut G: *mut glp_graph,
    mut v_set: libc::c_int,
    mut a_x: libc::c_int,
) -> libc::c_int {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut card: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut loc: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut xij: libc::c_int = 0;
    let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut icn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ip: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lenr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut iperm: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut arp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    if v_set >= 0 as libc::c_int
        && v_set
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
        ))
            .unwrap()(
            b"glp_asnprob_hall: v_set = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_set,
        );
    }
    if a_x >= 0 as libc::c_int
        && a_x
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
        ))
            .unwrap()(
            b"glp_asnprob_hall: a_x = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_x,
        );
    }
    if glp_check_asnprob(G, v_set) != 0 {
        return -(1 as libc::c_int);
    }
    num = glp_alloc(
        1 as libc::c_int + (*G).nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    n2 = 0 as libc::c_int;
    n1 = n2;
    i = 1 as libc::c_int;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if ((*v).in_0).is_null() && !((*v).out).is_null() {
            n1 += 1;
            n1;
            *num.offset(i as isize) = 0 as libc::c_int;
        } else if !((*v).in_0).is_null() && ((*v).out).is_null() {
            n2 += 1;
            n2;
            *num.offset(i as isize) = n2;
        } else {
            (((*v).in_0).is_null() && ((*v).out).is_null()
                || {
                    glp_assert_(
                        b"v->in == NULL && v->out == NULL\0" as *const u8
                            as *const libc::c_char,
                        b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
                        80 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *num.offset(i as isize) = -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    n = if n1 >= n2 { n1 } else { n2 };
    icn = glp_alloc(
        1 as libc::c_int + (*G).na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    ip = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    lenr = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    iperm = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    pr = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    arp = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    cv = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    out = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    k = 0 as libc::c_int;
    loc = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*G).nv {
        if !(*num.offset(i as isize) != 0 as libc::c_int) {
            k += 1;
            *ip.offset(k as isize) = loc;
            v = *((*G).v).offset(i as isize);
            a = (*v).out;
            while !a.is_null() {
                (*num.offset((*(*a).head).i as isize) != 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"num[a->head->i] != 0\0" as *const u8
                                as *const libc::c_char,
                            b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
                            105 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                let fresh0 = loc;
                loc = loc + 1;
                *icn.offset(fresh0 as isize) = *num.offset((*(*a).head).i as isize);
                a = (*a).t_next;
            }
            *lenr.offset(k as isize) = loc - *ip.offset(k as isize);
        }
        i += 1;
        i;
    }
    (loc - 1 as libc::c_int == (*G).na
        || {
            glp_assert_(
                b"loc-1 == G->na\0" as *const u8 as *const libc::c_char,
                b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    k += 1;
    k;
    while k <= n {
        *ip.offset(k as isize) = loc;
        *lenr.offset(k as isize) = 0 as libc::c_int;
        k += 1;
        k;
    }
    card = _glp_mc21a(
        n,
        icn as *const libc::c_int,
        ip as *const libc::c_int,
        lenr as *const libc::c_int,
        iperm,
        pr,
        arp,
        cv,
        out,
    );
    i = 1 as libc::c_int;
    while i <= n {
        *arp.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= card {
        k = *iperm.offset(i as isize);
        (1 as libc::c_int <= k && k <= n
            || {
                glp_assert_(
                    b"1 <= k && k <= n\0" as *const u8 as *const libc::c_char,
                    b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
                    124 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*arp.offset(k as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"arp[k] == 0\0" as *const u8 as *const libc::c_char,
                    b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
                    125 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *arp.offset(k as isize) = i;
        i += 1;
        i;
    }
    if !(a_x < 0 as libc::c_int) {
        k = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= (*G).nv {
            if !(*num.offset(i as isize) != 0 as libc::c_int) {
                k += 1;
                k;
                v = *((*G).v).offset(i as isize);
                a = (*v).out;
                while !a.is_null() {
                    if *arp.offset(k as isize) == *num.offset((*(*a).head).i as isize) {
                        (*arp.offset(k as isize) != 0 as libc::c_int
                            || {
                                glp_assert_(
                                    b"arp[k] != 0\0" as *const u8 as *const libc::c_char,
                                    b"api/asnhall.c\0" as *const u8 as *const libc::c_char,
                                    140 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        xij = 1 as libc::c_int;
                    } else {
                        xij = 0 as libc::c_int;
                    }
                    memcpy(
                        ((*a).data as *mut libc::c_char).offset(a_x as isize)
                            as *mut libc::c_void,
                        &mut xij as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    );
                    a = (*a).t_next;
                }
            }
            i += 1;
            i;
        }
    }
    glp_free(num as *mut libc::c_void);
    glp_free(icn as *mut libc::c_void);
    glp_free(ip as *mut libc::c_void);
    glp_free(lenr as *mut libc::c_void);
    glp_free(iperm as *mut libc::c_void);
    glp_free(pr as *mut libc::c_void);
    glp_free(arp as *mut libc::c_void);
    glp_free(cv as *mut libc::c_void);
    glp_free(out as *mut libc::c_void);
    return card;
}
