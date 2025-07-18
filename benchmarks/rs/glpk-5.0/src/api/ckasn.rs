use ::libc;
extern "C" {
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
pub unsafe extern "C" fn glp_check_asnprob(
    mut G: *mut glp_graph,
    mut v_set: libc::c_int,
) -> libc::c_int {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if v_set >= 0 as libc::c_int
        && v_set
            > (*G).v_size
                - ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int
    {
        (glp_error_(
            b"api/ckasn.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_asnprob: v_set = %d; invalid offset\n\0" as *const u8
                as *const libc::c_char,
            v_set,
        );
    }
    i = 1 as libc::c_int;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if v_set >= 0 as libc::c_int {
            memcpy(
                &mut k as *mut libc::c_int as *mut libc::c_void,
                ((*v).data as *mut libc::c_char).offset(v_set as isize)
                    as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            if k == 0 as libc::c_int {
                if !((*v).in_0).is_null() {
                    ret = 1 as libc::c_int;
                    break;
                }
            } else if k == 1 as libc::c_int {
                if !((*v).out).is_null() {
                    ret = 2 as libc::c_int;
                    break;
                }
            } else {
                ret = 3 as libc::c_int;
                break;
            }
        } else if !((*v).in_0).is_null() && !((*v).out).is_null() {
            ret = 4 as libc::c_int;
            break;
        }
        i += 1;
        i;
    }
    return ret;
}
