use ::libc;
extern "C" {
    pub type glp_prob;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_add_cols(P: *mut glp_prob, ncs: libc::c_int) -> libc::c_int;
    fn glp_set_row_name(P: *mut glp_prob, i: libc::c_int, name: *const libc::c_char);
    fn glp_set_col_name(P: *mut glp_prob, j: libc::c_int, name: *const libc::c_char);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_obj_coef(P: *mut glp_prob, j: libc::c_int, coef: libc::c_double);
    fn glp_set_mat_col(
        P: *mut glp_prob,
        j: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_erase_prob(P: *mut glp_prob);
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
pub unsafe extern "C" fn glp_mincost_lp(
    mut lp: *mut glp_prob,
    mut G: *mut glp_graph,
    mut names: libc::c_int,
    mut v_rhs: libc::c_int,
    mut a_low: libc::c_int,
    mut a_cap: libc::c_int,
    mut a_cost: libc::c_int,
) {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut ind: [libc::c_int; 3] = [0; 3];
    let mut rhs: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut cap: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut val: [libc::c_double; 3] = [0.; 3];
    if !(names == 1 as libc::c_int || names == 0 as libc::c_int) {
        (glp_error_(
            b"api/mcflp.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_lp: names = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            names,
        );
    }
    if v_rhs >= 0 as libc::c_int
        && v_rhs
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/mcflp.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_lp: v_rhs = %d; invalid offset\n\0" as *const u8
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
            b"api/mcflp.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_lp: a_low = %d; invalid offset\n\0" as *const u8
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
            b"api/mcflp.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_lp: a_cap = %d; invalid offset\n\0" as *const u8
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
            b"api/mcflp.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mincost_lp: a_cost = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            a_cost,
        );
    }
    glp_erase_prob(lp);
    if names != 0 {
        glp_set_prob_name(lp, (*G).name);
    }
    if (*G).nv > 0 as libc::c_int {
        glp_add_rows(lp, (*G).nv);
    }
    i = 1 as libc::c_int;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if names != 0 {
            glp_set_row_name(lp, i, (*v).name);
        }
        if v_rhs >= 0 as libc::c_int {
            memcpy(
                &mut rhs as *mut libc::c_double as *mut libc::c_void,
                ((*v).data as *mut libc::c_char).offset(v_rhs as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            );
        } else {
            rhs = 0.0f64;
        }
        glp_set_row_bnds(lp, i, 5 as libc::c_int, rhs, rhs);
        i += 1;
        i;
    }
    if (*G).na > 0 as libc::c_int {
        glp_add_cols(lp, (*G).na);
    }
    i = 1 as libc::c_int;
    j = 0 as libc::c_int;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        a = (*v).out;
        while !a.is_null() {
            j += 1;
            j;
            if names != 0 {
                let mut name: [libc::c_char; 51] = [0; 51];
                sprintf(
                    name.as_mut_ptr(),
                    b"x[%d,%d]\0" as *const u8 as *const libc::c_char,
                    (*(*a).tail).i,
                    (*(*a).head).i,
                );
                (strlen(name.as_mut_ptr())
                    < ::std::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                    || {
                        glp_assert_(
                            b"strlen(name) < sizeof(name)\0" as *const u8
                                as *const libc::c_char,
                            b"api/mcflp.c\0" as *const u8 as *const libc::c_char,
                            78 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                glp_set_col_name(lp, j, name.as_mut_ptr());
            }
            if (*(*a).tail).i != (*(*a).head).i {
                ind[1 as libc::c_int as usize] = (*(*a).tail).i;
                val[1 as libc::c_int as usize] = 1.0f64;
                ind[2 as libc::c_int as usize] = (*(*a).head).i;
                val[2 as libc::c_int as usize] = -1.0f64;
                glp_set_mat_col(
                    lp,
                    j,
                    2 as libc::c_int,
                    ind.as_mut_ptr() as *const libc::c_int,
                    val.as_mut_ptr() as *const libc::c_double,
                );
            }
            if a_low >= 0 as libc::c_int {
                memcpy(
                    &mut low as *mut libc::c_double as *mut libc::c_void,
                    ((*a).data as *mut libc::c_char).offset(a_low as isize)
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            } else {
                low = 0.0f64;
            }
            if a_cap >= 0 as libc::c_int {
                memcpy(
                    &mut cap as *mut libc::c_double as *mut libc::c_void,
                    ((*a).data as *mut libc::c_char).offset(a_cap as isize)
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            } else {
                cap = 1.0f64;
            }
            if cap == 1.7976931348623157e+308f64 {
                type_0 = 2 as libc::c_int;
            } else if low != cap {
                type_0 = 4 as libc::c_int;
            } else {
                type_0 = 5 as libc::c_int;
            }
            glp_set_col_bnds(lp, j, type_0, low, cap);
            if a_cost >= 0 as libc::c_int {
                memcpy(
                    &mut cost as *mut libc::c_double as *mut libc::c_void,
                    ((*a).data as *mut libc::c_char).offset(a_cost as isize)
                        as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
            } else {
                cost = 0.0f64;
            }
            glp_set_obj_coef(lp, j, cost);
            a = (*a).t_next;
        }
        i += 1;
        i;
    }
    (j == (*G).na
        || {
            glp_assert_(
                b"j == G->na\0" as *const u8 as *const libc::c_char,
                b"api/mcflp.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
