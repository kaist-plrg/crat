use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_check_asnprob(G: *mut glp_graph, v_set: libc::c_int) -> libc::c_int;
    fn _glp_okalg(
        nv: libc::c_int,
        na: libc::c_int,
        tail: *const libc::c_int,
        head: *const libc::c_int,
        low: *const libc::c_int,
        cap: *const libc::c_int,
        cost: *const libc::c_int,
        x: *mut libc::c_int,
        pi: *mut libc::c_int,
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
pub unsafe extern "C" fn glp_asnprob_okalg(
    mut form: libc::c_int,
    mut G: *mut glp_graph,
    mut v_set: libc::c_int,
    mut a_cost: libc::c_int,
    mut sol: *mut libc::c_double,
    mut a_x: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut nv: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tail: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut head: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cost: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pi: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut temp: libc::c_double = 0.;
    if !(form == 1 as libc::c_int || form == 2 as libc::c_int
        || form == 3 as libc::c_int)
    {
        (glp_error_(
            b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
        ))
            .unwrap()(
            b"glp_asnprob_okalg: form = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            form,
        );
    }
    if v_set >= 0 as libc::c_int
        && v_set
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
        ))
            .unwrap()(
            b"glp_asnprob_okalg: v_set = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_set,
        );
    }
    if a_cost >= 0 as libc::c_int
        && a_cost
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
        ))
            .unwrap()(
            b"glp_asnprob_okalg: a_cost = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cost,
        );
    }
    if a_x >= 0 as libc::c_int
        && a_x
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        ))
            .unwrap()(
            b"glp_asnprob_okalg: a_x = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_x,
        );
    }
    if glp_check_asnprob(G, v_set) != 0 {
        return 0x12 as libc::c_int;
    }
    nv = (*G).nv + 1 as libc::c_int;
    na = (*G).na + (*G).nv;
    tail = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    head = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    low = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    cap = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    cost = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    x = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    pi = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    k = 0 as libc::c_int;
    i = 1 as libc::c_int;
    's_79: loop {
        if !(i <= (*G).nv) {
            current_block = 8693738493027456495;
            break;
        }
        v = *((*G).v).offset(i as isize);
        a = (*v).out;
        while !a.is_null() {
            k += 1;
            k;
            *tail.offset(k as isize) = (*(*a).tail).i;
            *head.offset(k as isize) = (*(*a).head).i;
            *low.offset(k as isize) = 0 as libc::c_int;
            *cap.offset(k as isize) = 1 as libc::c_int;
            if a_cost >= 0 as libc::c_int {
                memcpy(
                    &mut temp as *mut libc::c_double as *mut libc::c_void,
                    ((*a).data as *mut libc::c_char).offset(a_cost as isize)
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            } else {
                temp = 1.0f64;
            }
            if !(fabs(temp) <= 2147483647 as libc::c_int as libc::c_double
                && temp == floor(temp))
            {
                ret = 0x12 as libc::c_int;
                current_block = 13702299577388315891;
                break 's_79;
            } else {
                *cost.offset(k as isize) = temp as libc::c_int;
                if form != 1 as libc::c_int {
                    *cost.offset(k as isize) = -*cost.offset(k as isize);
                }
                a = (*a).t_next;
            }
        }
        i += 1;
        i;
    }
    match current_block {
        8693738493027456495 => {
            i = 1 as libc::c_int;
            while i <= (*G).nv {
                v = *((*G).v).offset(i as isize);
                k += 1;
                k;
                if ((*v).out).is_null() {
                    *tail.offset(k as isize) = i;
                    *head.offset(k as isize) = nv;
                } else if ((*v).in_0).is_null() {
                    *tail.offset(k as isize) = nv;
                    *head.offset(k as isize) = i;
                } else {
                    (v != v
                        || {
                            glp_assert_(
                                b"v != v\0" as *const u8 as *const libc::c_char,
                                b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
                                91 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                *low
                    .offset(
                        k as isize,
                    ) = if form == 3 as libc::c_int {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                *cap.offset(k as isize) = 1 as libc::c_int;
                *cost.offset(k as isize) = 0 as libc::c_int;
                i += 1;
                i;
            }
            (k == na
                || {
                    glp_assert_(
                        b"k == na\0" as *const u8 as *const libc::c_char,
                        b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
                        96 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ret = _glp_okalg(
                nv,
                na,
                tail as *const libc::c_int,
                head as *const libc::c_int,
                low as *const libc::c_int,
                cap as *const libc::c_int,
                cost as *const libc::c_int,
                x,
                pi,
            );
            match ret {
                0 => {
                    ret = 0 as libc::c_int;
                    current_block = 13678349939556791712;
                }
                1 => {
                    ret = 0xa as libc::c_int;
                    current_block = 13678349939556791712;
                }
                2 => {
                    ret = 0x13 as libc::c_int;
                    current_block = 13702299577388315891;
                }
                3 => {
                    ret = 0x5 as libc::c_int;
                    current_block = 13702299577388315891;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const libc::c_char,
                                b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
                                117 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    current_block = 13678349939556791712;
                }
            }
            match current_block {
                13702299577388315891 => {}
                _ => {
                    if !sol.is_null() {
                        temp = 0.0f64;
                        k = 1 as libc::c_int;
                        while k <= na {
                            temp
                                += *cost.offset(k as isize) as libc::c_double
                                    * *x.offset(k as isize) as libc::c_double;
                            k += 1;
                            k;
                        }
                        if form != 1 as libc::c_int {
                            temp = -temp;
                        }
                        *sol = temp;
                    }
                    if a_x >= 0 as libc::c_int {
                        k = 0 as libc::c_int;
                        i = 1 as libc::c_int;
                        while i <= (*G).nv {
                            v = *((*G).v).offset(i as isize);
                            a = (*v).out;
                            while !a.is_null() {
                                k += 1;
                                k;
                                if ret == 0 as libc::c_int {
                                    (*x.offset(k as isize) == 0 as libc::c_int
                                        || *x.offset(k as isize) == 1 as libc::c_int
                                        || {
                                            glp_assert_(
                                                b"x[k] == 0 || x[k] == 1\0" as *const u8
                                                    as *const libc::c_char,
                                                b"api/asnokalg.c\0" as *const u8 as *const libc::c_char,
                                                136 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                }
                                memcpy(
                                    ((*a).data as *mut libc::c_char).offset(a_x as isize)
                                        as *mut libc::c_void,
                                    &mut *x.offset(k as isize) as *mut libc::c_int
                                        as *const libc::c_void,
                                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                );
                                a = (*a).t_next;
                            }
                            i += 1;
                            i;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(tail as *mut libc::c_void);
    glp_free(head as *mut libc::c_void);
    glp_free(low as *mut libc::c_void);
    glp_free(cap as *mut libc::c_void);
    glp_free(cost as *mut libc::c_void);
    glp_free(x as *mut libc::c_void);
    glp_free(pi as *mut libc::c_void);
    return ret;
}
