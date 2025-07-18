use ::libc;
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub unsafe extern "C" fn glp_weak_comp(
    mut G: *mut glp_graph,
    mut v_num: libc::c_int,
) -> libc::c_int {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut f: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut pos1: libc::c_int = 0;
    let mut pos2: libc::c_int = 0;
    let mut prev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut next: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    if v_num >= 0 as libc::c_int
        && v_num
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/weak.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
        ))
            .unwrap()(
            b"glp_weak_comp: v_num = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_num,
        );
    }
    nv = (*G).nv;
    if nv == 0 as libc::c_int {
        nc = 0 as libc::c_int;
    } else {
        prev = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        next = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        list = glp_alloc(
            1 as libc::c_int + nv,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        f = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= nv {
            *prev.offset(i as isize) = i - 1 as libc::c_int;
            *next.offset(i as isize) = i + 1 as libc::c_int;
            i += 1;
            i;
        }
        *next.offset(nv as isize) = 0 as libc::c_int;
        nc = 0 as libc::c_int;
        while f != 0 as libc::c_int {
            i = f;
            f = *next.offset(i as isize);
            if f != 0 as libc::c_int {
                *prev.offset(f as isize) = 0 as libc::c_int;
            }
            *prev.offset(i as isize) = -(1 as libc::c_int);
            nc += 1;
            *next.offset(i as isize) = nc;
            *list.offset(1 as libc::c_int as isize) = i;
            pos2 = 1 as libc::c_int;
            pos1 = pos2;
            while pos1 <= pos2 {
                let fresh0 = pos1;
                pos1 = pos1 + 1;
                i = *list.offset(fresh0 as isize);
                a = (**((*G).v).offset(i as isize)).in_0;
                while !a.is_null() {
                    j = (*(*a).tail).i;
                    if *prev.offset(j as isize) >= 0 as libc::c_int {
                        if *prev.offset(j as isize) == 0 as libc::c_int {
                            f = *next.offset(j as isize);
                        } else {
                            *next
                                .offset(
                                    *prev.offset(j as isize) as isize,
                                ) = *next.offset(j as isize);
                        }
                        if !(*next.offset(j as isize) == 0 as libc::c_int) {
                            *prev
                                .offset(
                                    *next.offset(j as isize) as isize,
                                ) = *prev.offset(j as isize);
                        }
                        *prev.offset(j as isize) = -(1 as libc::c_int);
                        *next.offset(j as isize) = nc;
                        pos2 += 1;
                        *list.offset(pos2 as isize) = j;
                    }
                    a = (*a).h_next;
                }
                a = (**((*G).v).offset(i as isize)).out;
                while !a.is_null() {
                    j = (*(*a).head).i;
                    if *prev.offset(j as isize) >= 0 as libc::c_int {
                        if *prev.offset(j as isize) == 0 as libc::c_int {
                            f = *next.offset(j as isize);
                        } else {
                            *next
                                .offset(
                                    *prev.offset(j as isize) as isize,
                                ) = *next.offset(j as isize);
                        }
                        if !(*next.offset(j as isize) == 0 as libc::c_int) {
                            *prev
                                .offset(
                                    *next.offset(j as isize) as isize,
                                ) = *prev.offset(j as isize);
                        }
                        *prev.offset(j as isize) = -(1 as libc::c_int);
                        *next.offset(j as isize) = nc;
                        pos2 += 1;
                        *list.offset(pos2 as isize) = j;
                    }
                    a = (*a).t_next;
                }
            }
        }
        if v_num >= 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut libc::c_char).offset(v_num as isize)
                        as *mut libc::c_void,
                    &mut *next.offset(i as isize) as *mut libc::c_int
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                );
                i += 1;
                i;
            }
        }
        glp_free(prev as *mut libc::c_void);
        glp_free(next as *mut libc::c_void);
        glp_free(list as *mut libc::c_void);
    }
    return nc;
}
