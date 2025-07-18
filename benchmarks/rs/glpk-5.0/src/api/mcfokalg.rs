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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub unsafe extern "C" fn glp_mincost_okalg(
    mut G: *mut glp_graph,
    mut v_rhs: libc::c_int,
    mut a_low: libc::c_int,
    mut a_cap: libc::c_int,
    mut a_cost: libc::c_int,
    mut sol: *mut libc::c_double,
    mut a_x: libc::c_int,
    mut v_pi: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut nv: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut tail: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut head: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut low: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cost: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pi: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    if v_rhs >= 0 as libc::c_int
        && v_rhs
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_okalg: v_rhs = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_rhs,
        );
    }
    if a_low >= 0 as libc::c_int
        && a_low
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_okalg: a_low = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_low,
        );
    }
    if a_cap >= 0 as libc::c_int
        && a_cap
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_okalg: a_cap = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cap,
        );
    }
    if a_cost >= 0 as libc::c_int
        && a_cost
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_okalg: a_cost = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cost,
        );
    }
    if a_x >= 0 as libc::c_int
        && a_x
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_okalg: a_x = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_x,
        );
    }
    if v_pi >= 0 as libc::c_int
        && v_pi
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_okalg: v_pi = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_pi,
        );
    }
    s = (*G).nv + 1 as libc::c_int;
    t = s + 1 as libc::c_int;
    nv = t;
    na = (*G).na + 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if v_rhs >= 0 as libc::c_int {
            memcpy(
                &mut temp as *mut libc::c_double as *mut libc::c_void,
                ((*v).data as *mut libc::c_char).offset(v_rhs as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            );
        } else {
            temp = 0.0f64;
        }
        if temp != 0.0f64 {
            na += 1;
            na;
        }
        i += 1;
        i;
    }
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
    's_132: loop {
        if !(i <= (*G).nv) {
            current_block = 14945149239039849694;
            break;
        }
        v = *((*G).v).offset(i as isize);
        a = (*v).out;
        while !a.is_null() {
            k += 1;
            k;
            *tail.offset(k as isize) = (*(*a).tail).i;
            *head.offset(k as isize) = (*(*a).head).i;
            if *tail.offset(k as isize) == *head.offset(k as isize) {
                ret = 0x12 as libc::c_int;
                current_block = 11560912589962352640;
                break 's_132;
            } else {
                if a_low >= 0 as libc::c_int {
                    memcpy(
                        &mut temp as *mut libc::c_double as *mut libc::c_void,
                        ((*a).data as *mut libc::c_char).offset(a_low as isize)
                            as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    );
                } else {
                    temp = 0.0f64;
                }
                if !(0.0f64 <= temp
                    && temp <= 2147483647 as libc::c_int as libc::c_double
                    && temp == floor(temp))
                {
                    ret = 0x12 as libc::c_int;
                    current_block = 11560912589962352640;
                    break 's_132;
                } else {
                    *low.offset(k as isize) = temp as libc::c_int;
                    if a_cap >= 0 as libc::c_int {
                        memcpy(
                            &mut temp as *mut libc::c_double as *mut libc::c_void,
                            ((*a).data as *mut libc::c_char).offset(a_cap as isize)
                                as *const libc::c_void,
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        );
                    } else {
                        temp = 1.0f64;
                    }
                    if !(*low.offset(k as isize) as libc::c_double <= temp
                        && temp <= 2147483647 as libc::c_int as libc::c_double
                        && temp == floor(temp))
                    {
                        ret = 0x12 as libc::c_int;
                        current_block = 11560912589962352640;
                        break 's_132;
                    } else {
                        *cap.offset(k as isize) = temp as libc::c_int;
                        if a_cost >= 0 as libc::c_int {
                            memcpy(
                                &mut temp as *mut libc::c_double as *mut libc::c_void,
                                ((*a).data as *mut libc::c_char).offset(a_cost as isize)
                                    as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                            );
                        } else {
                            temp = 0.0f64;
                        }
                        if !(fabs(temp) <= 2147483647 as libc::c_int as libc::c_double
                            && temp == floor(temp))
                        {
                            ret = 0x12 as libc::c_int;
                            current_block = 11560912589962352640;
                            break 's_132;
                        } else {
                            *cost.offset(k as isize) = temp as libc::c_int;
                            a = (*a).t_next;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
    match current_block {
        14945149239039849694 => {
            sum = 0.0f64;
            i = 1 as libc::c_int;
            loop {
                if !(i <= (*G).nv) {
                    current_block = 10853015579903106591;
                    break;
                }
                v = *((*G).v).offset(i as isize);
                if v_rhs >= 0 as libc::c_int {
                    memcpy(
                        &mut temp as *mut libc::c_double as *mut libc::c_void,
                        ((*v).data as *mut libc::c_char).offset(v_rhs as isize)
                            as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    );
                } else {
                    temp = 0.0f64;
                }
                if !(fabs(temp) <= 2147483647 as libc::c_int as libc::c_double
                    && temp == floor(temp))
                {
                    ret = 0x12 as libc::c_int;
                    current_block = 11560912589962352640;
                    break;
                } else {
                    if temp > 0.0f64 {
                        k += 1;
                        k;
                        *tail.offset(k as isize) = s;
                        *head.offset(k as isize) = i;
                        let ref mut fresh0 = *cap.offset(k as isize);
                        *fresh0 = temp as libc::c_int;
                        *low.offset(k as isize) = *fresh0;
                        *cost.offset(k as isize) = 0 as libc::c_int;
                        sum += temp;
                    } else if temp < 0.0f64 {
                        k += 1;
                        k;
                        *tail.offset(k as isize) = i;
                        *head.offset(k as isize) = t;
                        let ref mut fresh1 = *cap.offset(k as isize);
                        *fresh1 = -temp as libc::c_int;
                        *low.offset(k as isize) = *fresh1;
                        *cost.offset(k as isize) = 0 as libc::c_int;
                    }
                    i += 1;
                    i;
                }
            }
            match current_block {
                11560912589962352640 => {}
                _ => {
                    k += 1;
                    k;
                    (k == na
                        || {
                            glp_assert_(
                                b"k == na\0" as *const u8 as *const libc::c_char,
                                b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
                                150 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    *tail.offset(k as isize) = t;
                    *head.offset(k as isize) = s;
                    if sum > 2147483647 as libc::c_int as libc::c_double {
                        ret = 0x12 as libc::c_int;
                    } else {
                        let ref mut fresh2 = *cap.offset(k as isize);
                        *fresh2 = sum as libc::c_int;
                        *low.offset(k as isize) = *fresh2;
                        *cost.offset(k as isize) = 0 as libc::c_int;
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
                                current_block = 14648606000749551097;
                            }
                            1 => {
                                ret = 0xa as libc::c_int;
                                current_block = 14648606000749551097;
                            }
                            2 => {
                                ret = 0x13 as libc::c_int;
                                current_block = 11560912589962352640;
                            }
                            3 => {
                                ret = 0x5 as libc::c_int;
                                current_block = 11560912589962352640;
                            }
                            _ => {
                                (ret != ret
                                    || {
                                        glp_assert_(
                                            b"ret != ret\0" as *const u8 as *const libc::c_char,
                                            b"api/mcfokalg.c\0" as *const u8 as *const libc::c_char,
                                            179 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                                current_block = 14648606000749551097;
                            }
                        }
                        match current_block {
                            11560912589962352640 => {}
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
                                            temp = *x.offset(k as isize) as libc::c_double;
                                            memcpy(
                                                ((*a).data as *mut libc::c_char).offset(a_x as isize)
                                                    as *mut libc::c_void,
                                                &mut temp as *mut libc::c_double as *const libc::c_void,
                                                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                            );
                                            a = (*a).t_next;
                                        }
                                        i += 1;
                                        i;
                                    }
                                }
                                if v_pi >= 0 as libc::c_int {
                                    i = 1 as libc::c_int;
                                    while i <= (*G).nv {
                                        v = *((*G).v).offset(i as isize);
                                        temp = -(*pi.offset(i as isize) as libc::c_double);
                                        memcpy(
                                            ((*v).data as *mut libc::c_char).offset(v_pi as isize)
                                                as *mut libc::c_void,
                                            &mut temp as *mut libc::c_double as *const libc::c_void,
                                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                        );
                                        i += 1;
                                        i;
                                    }
                                }
                            }
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
