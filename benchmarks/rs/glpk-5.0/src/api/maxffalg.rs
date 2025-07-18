use ::libc;
extern "C" {
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn _glp_ffalg(
        nv: libc::c_int,
        na: libc::c_int,
        tail: *const libc::c_int,
        head: *const libc::c_int,
        s: libc::c_int,
        t: libc::c_int,
        cap: *const libc::c_int,
        x: *mut libc::c_int,
        cut: *mut libc::c_char,
    );
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
pub unsafe extern "C" fn glp_maxflow_ffalg(
    mut G: *mut glp_graph,
    mut s: libc::c_int,
    mut t: libc::c_int,
    mut a_cap: libc::c_int,
    mut sol: *mut libc::c_double,
    mut a_x: libc::c_int,
    mut v_cut: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut nv: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut tail: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut head: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cap: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut cut: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: libc::c_double = 0.;
    if !(1 as libc::c_int <= s && s <= (*G).nv) {
        (glp_error_(
            b"api/maxffalg.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
        ))
            .unwrap()(
            b"glp_maxflow_ffalg: s = %d; source node number out of range\n\0"
                as *const u8 as *const libc::c_char,
            s,
        );
    }
    if !(1 as libc::c_int <= t && t <= (*G).nv) {
        (glp_error_(
            b"api/maxffalg.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int,
        ))
            .unwrap()(
            b"glp_maxflow_ffalg: t = %d: sink node number out of range\n\0" as *const u8
                as *const libc::c_char,
            t,
        );
    }
    if s == t {
        (glp_error_(
            b"api/maxffalg.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
        ))
            .unwrap()(
            b"glp_maxflow_ffalg: s = t = %d; source and sink nodes must be distinct\n\0"
                as *const u8 as *const libc::c_char,
            s,
        );
    }
    if a_cap >= 0 as libc::c_int
        && a_cap
            > (*G).a_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/maxffalg.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        ))
            .unwrap()(
            b"glp_maxflow_ffalg: a_cap = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cap,
        );
    }
    if v_cut >= 0 as libc::c_int
        && v_cut
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/maxffalg.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
        ))
            .unwrap()(
            b"glp_maxflow_ffalg: v_cut = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_cut,
        );
    }
    nv = (*G).nv;
    na = (*G).na;
    tail = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    head = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    cap = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    x = glp_alloc(
        1 as libc::c_int + na,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    if v_cut < 0 as libc::c_int {
        cut = 0 as *mut libc::c_char;
    } else {
        cut = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
    }
    k = 0 as libc::c_int;
    i = 1 as libc::c_int;
    's_85: loop {
        if !(i <= (*G).nv) {
            current_block = 11636175345244025579;
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
                current_block = 5264500853760215793;
                break 's_85;
            } else {
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
                if !(0.0f64 <= temp
                    && temp <= 2147483647 as libc::c_int as libc::c_double
                    && temp == floor(temp))
                {
                    ret = 0x12 as libc::c_int;
                    current_block = 5264500853760215793;
                    break 's_85;
                } else {
                    *cap.offset(k as isize) = temp as libc::c_int;
                    a = (*a).t_next;
                }
            }
        }
        i += 1;
        i;
    }
    match current_block {
        11636175345244025579 => {
            (k == na
                || {
                    glp_assert_(
                        b"k == na\0" as *const u8 as *const libc::c_char,
                        b"api/maxffalg.c\0" as *const u8 as *const libc::c_char,
                        84 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_ffalg(
                nv,
                na,
                tail as *const libc::c_int,
                head as *const libc::c_int,
                s,
                t,
                cap as *const libc::c_int,
                x,
                cut,
            );
            ret = 0 as libc::c_int;
            if !sol.is_null() {
                temp = 0.0f64;
                k = 1 as libc::c_int;
                while k <= na {
                    if *tail.offset(k as isize) == s {
                        temp += *x.offset(k as isize) as libc::c_double;
                    } else if *head.offset(k as isize) == s {
                        temp -= *x.offset(k as isize) as libc::c_double;
                    }
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
            if v_cut >= 0 as libc::c_int {
                i = 1 as libc::c_int;
                while i <= (*G).nv {
                    v = *((*G).v).offset(i as isize);
                    flag = *cut.offset(i as isize) as libc::c_int;
                    memcpy(
                        ((*v).data as *mut libc::c_char).offset(v_cut as isize)
                            as *mut libc::c_void,
                        &mut flag as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    );
                    i += 1;
                    i;
                }
            }
        }
        _ => {}
    }
    glp_free(tail as *mut libc::c_void);
    glp_free(head as *mut libc::c_void);
    glp_free(cap as *mut libc::c_void);
    glp_free(x as *mut libc::c_void);
    if !cut.is_null() {
        glp_free(cut as *mut libc::c_void);
    }
    return ret;
}
