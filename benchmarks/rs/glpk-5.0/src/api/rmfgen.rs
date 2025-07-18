use ::libc;
extern "C" {
    fn abort() -> !;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
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
pub unsafe extern "C" fn glp_rmfgen(
    mut G_: *mut glp_graph,
    mut s_: *mut libc::c_int,
    mut t_: *mut libc::c_int,
    mut a_cap_: libc::c_int,
    mut parm: *const libc::c_int,
) -> libc::c_int {
    static mut func: [libc::c_char; 11] = unsafe {
        *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"glp_rmfgen\0")
    };
    (G_ == G_
        || {
            glp_assert_(
                b"G_ == G_\0" as *const u8 as *const libc::c_char,
                b"api/rmfgen.c\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (s_ == s_
        || {
            glp_assert_(
                b"s_ == s_\0" as *const u8 as *const libc::c_char,
                b"api/rmfgen.c\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (t_ == t_
        || {
            glp_assert_(
                b"t_ == t_\0" as *const u8 as *const libc::c_char,
                b"api/rmfgen.c\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (a_cap_ == a_cap_
        || {
            glp_assert_(
                b"a_cap_ == a_cap_\0" as *const u8 as *const libc::c_char,
                b"api/rmfgen.c\0" as *const u8 as *const libc::c_char,
                12 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const libc::c_char,
                b"api/rmfgen.c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(
        b"api/rmfgen.c\0" as *const u8 as *const libc::c_char,
        14 as libc::c_int,
    ))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
