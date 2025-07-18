use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn abort() -> !;
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax4_csa {
    pub n: libc::c_int,
    pub na: libc::c_int,
    pub large: libc::c_int,
    pub repeat: libc::c_int,
    pub crash: libc::c_int,
    pub startn: *mut libc::c_int,
    pub endn: *mut libc::c_int,
    pub fou: *mut libc::c_int,
    pub nxtou: *mut libc::c_int,
    pub fin: *mut libc::c_int,
    pub nxtin: *mut libc::c_int,
    pub rc: *mut libc::c_int,
    pub u: *mut libc::c_int,
    pub dfct: *mut libc::c_int,
    pub x: *mut libc::c_int,
    pub nmultinode: libc::c_int,
    pub iter: libc::c_int,
    pub num_augm: libc::c_int,
    pub num_ascnt: libc::c_int,
    pub nsp: libc::c_int,
    pub label: *mut libc::c_int,
    pub prdcsr: *mut libc::c_int,
    pub save: *mut libc::c_int,
    pub tfstou: *mut libc::c_int,
    pub tnxtou: *mut libc::c_int,
    pub tfstin: *mut libc::c_int,
    pub tnxtin: *mut libc::c_int,
    pub nxtqueue: *mut libc::c_int,
    pub scan: *mut libc::c_char,
    pub mark: *mut libc::c_char,
    pub extend_arc: *mut libc::c_int,
    pub sb_level: *mut libc::c_int,
    pub sb_arc: *mut libc::c_int,
}
pub unsafe extern "C" fn _glp_relax4(mut csa: *mut relax4_csa) -> libc::c_int {
    static mut func: [libc::c_char; 7] = unsafe {
        *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"relax4\0")
    };
    (csa == csa
        || {
            glp_assert_(
                b"csa == csa\0" as *const u8 as *const libc::c_char,
                b"misc/relax4.c\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(
        b"misc/relax4.c\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int,
    ))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
pub unsafe extern "C" fn _glp_relax4_inidat(mut csa: *mut relax4_csa) {
    static mut func: [libc::c_char; 14] = unsafe {
        *::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"relax4_inidat\0")
    };
    (csa == csa
        || {
            glp_assert_(
                b"csa == csa\0" as *const u8 as *const libc::c_char,
                b"misc/relax4.c\0" as *const u8 as *const libc::c_char,
                17 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(
        b"misc/relax4.c\0" as *const u8 as *const libc::c_char,
        18 as libc::c_int,
    ))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
