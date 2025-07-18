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
    fn _glp_mc13d(
        n: libc::c_int,
        icn: *const libc::c_int,
        ip: *const libc::c_int,
        lenr: *const libc::c_int,
        ior: *mut libc::c_int,
        ib: *mut libc::c_int,
        lowl: *mut libc::c_int,
        numb: *mut libc::c_int,
        prev: *mut libc::c_int,
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
pub unsafe extern "C" fn glp_strong_comp(
    mut G: *mut glp_graph,
    mut v_num: libc::c_int,
) -> libc::c_int {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut icn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ip: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lenr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ior: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ib: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lowl: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut numb: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prev: *mut libc::c_int = 0 as *mut libc::c_int;
    if v_num >= 0 as libc::c_int
        && v_num
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/strong.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
        ))
            .unwrap()(
            b"glp_strong_comp: v_num = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_num,
        );
    }
    n = (*G).nv;
    if n == 0 as libc::c_int {
        nc = 0 as libc::c_int;
    } else {
        na = (*G).na;
        icn = glp_alloc(
            1 as libc::c_int + na,
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
        ior = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        ib = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        lowl = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        numb = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        prev = glp_alloc(
            1 as libc::c_int + n,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        k = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= n {
            v = *((*G).v).offset(i as isize);
            *ip.offset(i as isize) = k;
            a = (*v).out;
            while !a.is_null() {
                let fresh0 = k;
                k = k + 1;
                *icn.offset(fresh0 as isize) = (*(*a).head).i;
                a = (*a).t_next;
            }
            *lenr.offset(i as isize) = k - *ip.offset(i as isize);
            i += 1;
            i;
        }
        (na == k - 1 as libc::c_int
            || {
                glp_assert_(
                    b"na == k-1\0" as *const u8 as *const libc::c_char,
                    b"api/strong.c\0" as *const u8 as *const libc::c_char,
                    84 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        nc = _glp_mc13d(
            n,
            icn as *const libc::c_int,
            ip as *const libc::c_int,
            lenr as *const libc::c_int,
            ior,
            ib,
            lowl,
            numb,
            prev,
        );
        if v_num >= 0 as libc::c_int {
            (*ib.offset(1 as libc::c_int as isize) == 1 as libc::c_int
                || {
                    glp_assert_(
                        b"ib[1] == 1\0" as *const u8 as *const libc::c_char,
                        b"api/strong.c\0" as *const u8 as *const libc::c_char,
                        87 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            k = 1 as libc::c_int;
            while k <= nc {
                last = if k < nc {
                    *ib.offset((k + 1 as libc::c_int) as isize)
                } else {
                    n + 1 as libc::c_int
                };
                (*ib.offset(k as isize) < last
                    || {
                        glp_assert_(
                            b"ib[k] < last\0" as *const u8 as *const libc::c_char,
                            b"api/strong.c\0" as *const u8 as *const libc::c_char,
                            90 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                i = *ib.offset(k as isize);
                while i < last {
                    v = *((*G).v).offset(*ior.offset(i as isize) as isize);
                    memcpy(
                        ((*v).data as *mut libc::c_char).offset(v_num as isize)
                            as *mut libc::c_void,
                        &mut k as *mut libc::c_int as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    );
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
        }
        glp_free(icn as *mut libc::c_void);
        glp_free(ip as *mut libc::c_void);
        glp_free(lenr as *mut libc::c_void);
        glp_free(ior as *mut libc::c_void);
        glp_free(ib as *mut libc::c_void);
        glp_free(lowl as *mut libc::c_void);
        glp_free(numb as *mut libc::c_void);
        glp_free(prev as *mut libc::c_void);
    }
    return nc;
}
