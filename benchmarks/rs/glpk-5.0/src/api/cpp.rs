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
    fn glp_top_sort(G: *mut glp_graph, v_num: libc::c_int) -> libc::c_int;
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
pub unsafe extern "C" fn glp_cpp(
    mut G: *mut glp_graph,
    mut v_t: libc::c_int,
    mut v_es: libc::c_int,
    mut v_ls: libc::c_int,
) -> libc::c_double {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut temp: libc::c_double = 0.;
    let mut total: libc::c_double = 0.;
    let mut t: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut es: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ls: *mut libc::c_double = 0 as *mut libc::c_double;
    if v_t >= 0 as libc::c_int
        && v_t
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/cpp.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
        ))
            .unwrap()(
            b"glp_cpp: v_t = %d; invalid offset\n\0" as *const u8 as *const libc::c_char,
            v_t,
        );
    }
    if v_es >= 0 as libc::c_int
        && v_es
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/cpp.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
        ))
            .unwrap()(
            b"glp_cpp: v_es = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_es,
        );
    }
    if v_ls >= 0 as libc::c_int
        && v_ls
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/cpp.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
        ))
            .unwrap()(
            b"glp_cpp: v_ls = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_ls,
        );
    }
    nv = (*G).nv;
    if nv == 0 as libc::c_int {
        total = 0.0f64;
    } else {
        t = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        es = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        ls = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        list = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        i = 1 as libc::c_int;
        while i <= nv {
            v = *((*G).v).offset(i as isize);
            if v_t >= 0 as libc::c_int {
                memcpy(
                    &mut *t.offset(i as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                    ((*v).data as *mut libc::c_char).offset(v_t as isize)
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                if *t.offset(i as isize) < 0.0f64 {
                    (glp_error_(
                        b"api/cpp.c\0" as *const u8 as *const libc::c_char,
                        90 as libc::c_int,
                    ))
                        .unwrap()(
                        b"glp_cpp: t[%d] = %g; invalid time\n\0" as *const u8
                            as *const libc::c_char,
                        i,
                        *t.offset(i as isize),
                    );
                }
            } else {
                *t.offset(i as isize) = 1.0f64;
            }
            i += 1;
            i;
        }
        sorting(G, list);
        k = 1 as libc::c_int;
        while k <= nv {
            j = *list.offset(k as isize);
            *es.offset(j as isize) = 0.0f64;
            a = (**((*G).v).offset(j as isize)).in_0;
            while !a.is_null() {
                i = (*(*a).tail).i;
                temp = *es.offset(i as isize) + *t.offset(i as isize);
                if *es.offset(j as isize) < temp {
                    *es.offset(j as isize) = temp;
                }
                a = (*a).h_next;
            }
            k += 1;
            k;
        }
        total = 0.0f64;
        i = 1 as libc::c_int;
        while i <= nv {
            temp = *es.offset(i as isize) + *t.offset(i as isize);
            if total < temp {
                total = temp;
            }
            i += 1;
            i;
        }
        k = nv;
        while k >= 1 as libc::c_int {
            i = *list.offset(k as isize);
            *ls.offset(i as isize) = total - *t.offset(i as isize);
            a = (**((*G).v).offset(i as isize)).out;
            while !a.is_null() {
                j = (*(*a).head).i;
                temp = *ls.offset(j as isize) - *t.offset(i as isize);
                if *ls.offset(i as isize) > temp {
                    *ls.offset(i as isize) = temp;
                }
                a = (*a).t_next;
            }
            if *ls.offset(i as isize) < *es.offset(i as isize) {
                *ls.offset(i as isize) = *es.offset(i as isize);
            }
            k -= 1;
            k;
        }
        if v_es >= 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut libc::c_char).offset(v_es as isize)
                        as *mut libc::c_void,
                    &mut *es.offset(i as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                i += 1;
                i;
            }
        }
        if v_ls >= 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut libc::c_char).offset(v_ls as isize)
                        as *mut libc::c_void,
                    &mut *ls.offset(i as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                i += 1;
                i;
            }
        }
        glp_free(t as *mut libc::c_void);
        glp_free(es as *mut libc::c_void);
        glp_free(ls as *mut libc::c_void);
        glp_free(list as *mut libc::c_void);
    }
    return total;
}
unsafe extern "C" fn sorting(mut G: *mut glp_graph, mut list: *mut libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut v_size: libc::c_int = 0;
    let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut save: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    nv = (*G).nv;
    v_size = (*G).v_size;
    save = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut libc::c_void;
    num = glp_alloc(
        1 as libc::c_int + nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*G).v_size = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    i = 1 as libc::c_int;
    while i <= nv {
        let ref mut fresh0 = *save.offset(i as isize);
        *fresh0 = (**((*G).v).offset(i as isize)).data;
        let ref mut fresh1 = (**((*G).v).offset(i as isize)).data;
        *fresh1 = &mut *num.offset(i as isize) as *mut libc::c_int as *mut libc::c_void;
        *list.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    if glp_top_sort(G, 0 as libc::c_int) != 0 as libc::c_int {
        (glp_error_(
            b"api/cpp.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
        ))
            .unwrap()(
            b"glp_cpp: project network is not acyclic\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*G).v_size = v_size;
    i = 1 as libc::c_int;
    while i <= nv {
        let ref mut fresh2 = (**((*G).v).offset(i as isize)).data;
        *fresh2 = *save.offset(i as isize);
        k = *num.offset(i as isize);
        (1 as libc::c_int <= k && k <= nv
            || {
                glp_assert_(
                    b"1 <= k && k <= nv\0" as *const u8 as *const libc::c_char,
                    b"api/cpp.c\0" as *const u8 as *const libc::c_char,
                    174 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*list.offset(k as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"list[k] == 0\0" as *const u8 as *const libc::c_char,
                    b"api/cpp.c\0" as *const u8 as *const libc::c_char,
                    175 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *list.offset(k as isize) = i;
        i += 1;
        i;
    }
    glp_free(save as *mut libc::c_void);
    glp_free(num as *mut libc::c_void);
}
