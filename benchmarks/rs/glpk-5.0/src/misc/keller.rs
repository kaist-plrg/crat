use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_add_vertices(G: *mut glp_graph, nadd: libc::c_int) -> libc::c_int;
    fn glp_add_arc(G: *mut glp_graph, i: libc::c_int, j: libc::c_int) -> *mut glp_arc;
    fn glp_erase_graph(G: *mut glp_graph, v_size: libc::c_int, a_size: libc::c_int);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct set {
    pub size: libc::c_int,
    pub list: *mut libc::c_int,
    pub pos: *mut libc::c_int,
}
pub unsafe extern "C" fn _glp_kellerman(
    mut n: libc::c_int,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
    mut H_: *mut libc::c_void,
) -> libc::c_int {
    let mut H: *mut glp_graph = H_ as *mut glp_graph;
    let mut W_: set = set {
        size: 0,
        list: 0 as *mut libc::c_int,
        pos: 0 as *mut libc::c_int,
    };
    let mut W: *mut set = &mut W_;
    let mut V_: set = set {
        size: 0,
        list: 0 as *mut libc::c_int,
        pos: 0 as *mut libc::c_int,
    };
    let mut V: *mut set = &mut V_;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut card: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    (n >= 0 as libc::c_int
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const libc::c_char,
                b"misc/keller.c\0" as *const u8 as *const libc::c_char,
                99 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_erase_graph(H, (*H).v_size, (*H).a_size);
    glp_add_vertices(H, n);
    (*W).size = 0 as libc::c_int;
    (*W)
        .list = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*W)
        .pos = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        &mut *((*W).pos).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    (*V).size = 0 as libc::c_int;
    (*V)
        .list = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*V)
        .pos = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        &mut *((*V).pos).offset(1 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    i = 1 as libc::c_int;
    while i <= n {
        ((*W).size == 0 as libc::c_int
            || {
                glp_assert_(
                    b"W->size == 0\0" as *const u8 as *const libc::c_char,
                    b"misc/keller.c\0" as *const u8 as *const libc::c_char,
                    116 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        len = func.unwrap()(info, i, (*W).list);
        (0 as libc::c_int <= len && len <= n
            || {
                glp_assert_(
                    b"0 <= len && len <= n\0" as *const u8 as *const libc::c_char,
                    b"misc/keller.c\0" as *const u8 as *const libc::c_char,
                    119 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        t = 1 as libc::c_int;
        while t <= len {
            j = *((*W).list).offset(t as isize);
            (1 as libc::c_int <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"misc/keller.c\0" as *const u8 as *const libc::c_char,
                        122 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if !(j >= i) {
                (*((*W).pos).offset(j as isize) == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"W->pos[j] == 0\0" as *const u8 as *const libc::c_char,
                            b"misc/keller.c\0" as *const u8 as *const libc::c_char,
                            124 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*W).size += 1;
                *((*W).list).offset((*W).size as isize) = j;
                *((*W).pos).offset(j as isize) = (*W).size;
            }
            t += 1;
            t;
        }
        if (*W).size == 0 as libc::c_int {
            k = glp_add_vertices(H, 1 as libc::c_int) - n;
            glp_add_arc(H, i, n + k);
        } else {
            ((*V).size == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"V->size == 0\0" as *const u8 as *const libc::c_char,
                        b"misc/keller.c\0" as *const u8 as *const libc::c_char,
                        138 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            k = (*H).nv - n;
            m = 1 as libc::c_int;
            while m <= k {
                if (*V).size == (*W).size {
                    break;
                }
                a = (**((*H).v).offset((n + m) as isize)).in_0;
                while !a.is_null() {
                    j = (*(*a).tail).i;
                    if *((*W).pos).offset(j as isize) == 0 as libc::c_int {
                        break;
                    }
                    a = (*a).h_next;
                }
                if a.is_null() {
                    glp_add_arc(H, i, n + m);
                    a = (**((*H).v).offset((n + m) as isize)).in_0;
                    while !a.is_null() {
                        j = (*(*a).tail).i;
                        if *((*V).pos).offset(j as isize) == 0 as libc::c_int {
                            (*V).size += 1;
                            *((*V).list).offset((*V).size as isize) = j;
                            *((*V).pos).offset(j as isize) = (*V).size;
                        }
                        a = (*a).h_next;
                    }
                }
                m += 1;
                m;
            }
            t = 1 as libc::c_int;
            while t <= (*V).size {
                j = *((*V).list).offset(t as isize);
                *((*V).pos).offset(j as isize) = 0 as libc::c_int;
                if *((*W).pos).offset(j as isize) != 0 as libc::c_int {
                    if *((*W).pos).offset(j as isize) != (*W).size {
                        let mut jj: libc::c_int = *((*W).list)
                            .offset((*W).size as isize);
                        *((*W).list)
                            .offset(*((*W).pos).offset(j as isize) as isize) = jj;
                        *((*W).pos).offset(jj as isize) = *((*W).pos).offset(j as isize);
                    }
                    (*W).size -= 1;
                    (*W).size;
                    *((*W).pos).offset(j as isize) = 0 as libc::c_int;
                }
                t += 1;
                t;
            }
            (*V).size = 0 as libc::c_int;
            while (*W).size > 0 as libc::c_int {
                m = 0 as libc::c_int;
                best = -(1 as libc::c_int);
                k = (*H).nv - n;
                t = 1 as libc::c_int;
                while t <= k {
                    card = 0 as libc::c_int;
                    a = (**((*H).v).offset((n + t) as isize)).in_0;
                    while !a.is_null() {
                        j = (*(*a).tail).i;
                        if *((*W).pos).offset(j as isize) != 0 as libc::c_int {
                            card += 1;
                            card;
                        }
                        a = (*a).h_next;
                    }
                    if best < card {
                        m = t;
                        best = card;
                    }
                    t += 1;
                    t;
                }
                (m > 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"m > 0\0" as *const u8 as *const libc::c_char,
                            b"misc/keller.c\0" as *const u8 as *const libc::c_char,
                            198 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                k = glp_add_vertices(H, 1 as libc::c_int) - n;
                a = (**((*H).v).offset((n + m) as isize)).in_0;
                while !a.is_null() {
                    j = (*(*a).tail).i;
                    if *((*W).pos).offset(j as isize) != 0 as libc::c_int {
                        glp_add_arc(H, j, n + k);
                        if *((*W).pos).offset(j as isize) != (*W).size {
                            let mut jj_0: libc::c_int = *((*W).list)
                                .offset((*W).size as isize);
                            *((*W).list)
                                .offset(*((*W).pos).offset(j as isize) as isize) = jj_0;
                            *((*W).pos)
                                .offset(jj_0 as isize) = *((*W).pos).offset(j as isize);
                        }
                        (*W).size -= 1;
                        (*W).size;
                        *((*W).pos).offset(j as isize) = 0 as libc::c_int;
                    }
                    a = (*a).h_next;
                }
                glp_add_arc(H, i, n + k);
            }
        }
        i += 1;
        i;
    }
    glp_free((*W).list as *mut libc::c_void);
    glp_free((*W).pos as *mut libc::c_void);
    glp_free((*V).list as *mut libc::c_void);
    glp_free((*V).pos as *mut libc::c_void);
    return (*H).nv - n;
}
