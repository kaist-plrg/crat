use ::libc;
extern "C" {
    pub type DMP;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_lux_solve(lux: *mut LUX, tr: libc::c_int, x: *mut mpq_t);
    fn _glp_lux_delete(lux: *mut LUX);
    fn _glp_lux_decomp(
        lux: *mut LUX,
        col: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
                *mut mpq_t,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
    ) -> libc::c_int;
    fn _glp_lux_create(n: libc::c_int) -> *mut LUX;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz {
    pub val: libc::c_int,
    pub ptr: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz_seg {
    pub d: [libc::c_ushort; 6],
    pub next: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}
pub type mpq_t = *mut mpq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BFX {
    pub valid: libc::c_int,
    pub lux: *mut LUX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUX {
    pub n: libc::c_int,
    pub pool: *mut DMP,
    pub F_row: *mut *mut LUXELM,
    pub F_col: *mut *mut LUXELM,
    pub V_piv: *mut mpq_t,
    pub V_row: *mut *mut LUXELM,
    pub V_col: *mut *mut LUXELM,
    pub P_row: *mut libc::c_int,
    pub P_col: *mut libc::c_int,
    pub Q_row: *mut libc::c_int,
    pub Q_col: *mut libc::c_int,
    pub rank: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LUXELM {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub val: mpq_t,
    pub r_prev: *mut LUXELM,
    pub r_next: *mut LUXELM,
    pub c_prev: *mut LUXELM,
    pub c_next: *mut LUXELM,
}
pub unsafe extern "C" fn _glp_bfx_create_binv() -> *mut BFX {
    let mut bfx: *mut BFX = 0 as *mut BFX;
    bfx = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<BFX>() as libc::c_ulong as libc::c_int,
    ) as *mut BFX;
    (*bfx).valid = 0 as libc::c_int;
    (*bfx).lux = 0 as *mut LUX;
    return bfx;
}
pub unsafe extern "C" fn _glp_bfx_factorize(
    mut binv: *mut BFX,
    mut m: libc::c_int,
    mut col: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_int,
            *mut mpq_t,
        ) -> libc::c_int,
    >,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    (m > 0 as libc::c_int
        || {
            glp_assert_(
                b"m > 0\0" as *const u8 as *const libc::c_char,
                b"draft/bfx.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*binv).lux).is_null() && (*(*binv).lux).n != m {
        _glp_lux_delete((*binv).lux);
        (*binv).lux = 0 as *mut LUX;
    }
    if ((*binv).lux).is_null() {
        (*binv).lux = _glp_lux_create(m);
    }
    ret = _glp_lux_decomp((*binv).lux, col, info);
    (*binv).valid = (ret == 0 as libc::c_int) as libc::c_int;
    return ret;
}
pub unsafe extern "C" fn _glp_bfx_ftran(
    mut binv: *mut BFX,
    mut x: *mut mpq_t,
    mut save: libc::c_int,
) {
    ((*binv).valid != 0
        || {
            glp_assert_(
                b"binv->valid\0" as *const u8 as *const libc::c_char,
                b"draft/bfx.c\0" as *const u8 as *const libc::c_char,
                58 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_lux_solve((*binv).lux, 0 as libc::c_int, x);
    (save == save
        || {
            glp_assert_(
                b"save == save\0" as *const u8 as *const libc::c_char,
                b"draft/bfx.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_bfx_btran(mut binv: *mut BFX, mut x: *mut mpq_t) {
    ((*binv).valid != 0
        || {
            glp_assert_(
                b"binv->valid\0" as *const u8 as *const libc::c_char,
                b"draft/bfx.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_lux_solve((*binv).lux, 1 as libc::c_int, x);
}
pub unsafe extern "C" fn _glp_bfx_update(
    mut binv: *mut BFX,
    mut j: libc::c_int,
) -> libc::c_int {
    ((*binv).valid != 0
        || {
            glp_assert_(
                b"binv->valid\0" as *const u8 as *const libc::c_char,
                b"draft/bfx.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= j && j <= (*(*binv).lux).n
        || {
            glp_assert_(
                b"1 <= j && j <= binv->lux->n\0" as *const u8 as *const libc::c_char,
                b"draft/bfx.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_bfx_delete_binv(mut binv: *mut BFX) {
    if !((*binv).lux).is_null() {
        _glp_lux_delete((*binv).lux);
    }
    glp_free(binv as *mut libc::c_void);
}
