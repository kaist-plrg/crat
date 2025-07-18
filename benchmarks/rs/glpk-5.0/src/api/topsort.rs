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
unsafe extern "C" fn top_sort(
    mut G: *mut glp_graph,
    mut num: *mut libc::c_int,
) -> libc::c_int {
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut stack: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut indeg: *mut libc::c_int = 0 as *mut libc::c_int;
    indeg = glp_alloc(
        1 as libc::c_int + (*G).nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    stack = glp_alloc(
        1 as libc::c_int + (*G).nv,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    top = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*G).nv {
        let ref mut fresh0 = *indeg.offset(i as isize);
        *fresh0 = 0 as libc::c_int;
        *num.offset(i as isize) = *fresh0;
        a = (**((*G).v).offset(i as isize)).in_0;
        while !a.is_null() {
            let ref mut fresh1 = *indeg.offset(i as isize);
            *fresh1 += 1;
            *fresh1;
            a = (*a).h_next;
        }
        if *indeg.offset(i as isize) == 0 as libc::c_int {
            top += 1;
            *stack.offset(top as isize) = i;
        }
        i += 1;
        i;
    }
    cnt = 0 as libc::c_int;
    while top > 0 as libc::c_int {
        let fresh2 = top;
        top = top - 1;
        i = *stack.offset(fresh2 as isize);
        (*indeg.offset(i as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"indeg[i] == 0\0" as *const u8 as *const libc::c_char,
                    b"api/topsort.c\0" as *const u8 as *const libc::c_char,
                    78 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*num.offset(i as isize) == 0 as libc::c_int
            || {
                glp_assert_(
                    b"num[i] == 0\0" as *const u8 as *const libc::c_char,
                    b"api/topsort.c\0" as *const u8 as *const libc::c_char,
                    80 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        cnt += 1;
        *num.offset(i as isize) = cnt;
        a = (**((*G).v).offset(i as isize)).out;
        while !a.is_null() {
            j = (*(*a).head).i;
            (*indeg.offset(j as isize) > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"indeg[j] > 0\0" as *const u8 as *const libc::c_char,
                        b"api/topsort.c\0" as *const u8 as *const libc::c_char,
                        88 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            let ref mut fresh3 = *indeg.offset(j as isize);
            *fresh3 -= 1;
            *fresh3;
            if *indeg.offset(j as isize) == 0 as libc::c_int {
                top += 1;
                *stack.offset(top as isize) = j;
            }
            a = (*a).t_next;
        }
    }
    glp_free(indeg as *mut libc::c_void);
    glp_free(stack as *mut libc::c_void);
    return (*G).nv - cnt;
}
pub unsafe extern "C" fn glp_top_sort(
    mut G: *mut glp_graph,
    mut v_num: libc::c_int,
) -> libc::c_int {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut num: *mut libc::c_int = 0 as *mut libc::c_int;
    if v_num >= 0 as libc::c_int
        && v_num
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/topsort.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
        ))
            .unwrap()(
            b"glp_top_sort: v_num = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_num,
        );
    }
    if (*G).nv == 0 as libc::c_int {
        cnt = 0 as libc::c_int;
    } else {
        num = glp_alloc(
            1 as libc::c_int + (*G).nv,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        cnt = top_sort(G, num);
        if v_num >= 0 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= (*G).nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut libc::c_char).offset(v_num as isize)
                        as *mut libc::c_void,
                    &mut *num.offset(i as isize) as *mut libc::c_int
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                );
                i += 1;
                i;
            }
        }
        glp_free(num as *mut libc::c_void);
    }
    return cnt;
}
