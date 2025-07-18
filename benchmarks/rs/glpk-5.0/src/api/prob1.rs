use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: libc::c_int,
        ind: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut libc::c_void);
    fn _glp_avl_delete_node(tree: *mut AVL, node: *mut AVLNODE);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_bfd_delete_it(bfd: *mut BFD);
    fn _glp_avl_delete_tree(tree: *mut AVL);
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub dir: libc::c_int,
    pub c0: libc::c_double,
    pub m_max: libc::c_int,
    pub n_max: libc::c_int,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: libc::c_int,
    pub head: *mut libc::c_int,
    pub bfd: *mut BFD,
    pub pbs_stat: libc::c_int,
    pub dbs_stat: libc::c_int,
    pub obj_val: libc::c_double,
    pub it_cnt: libc::c_int,
    pub some: libc::c_int,
    pub ipt_stat: libc::c_int,
    pub ipt_obj: libc::c_double,
    pub mip_stat: libc::c_int,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub kind: libc::c_int,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub level: libc::c_int,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tree {
    pub magic: libc::c_int,
    pub pool: *mut DMP,
    pub n: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_type: *mut libc::c_uchar,
    pub orig_lb: *mut libc::c_double,
    pub orig_ub: *mut libc::c_double,
    pub orig_stat: *mut libc::c_uchar,
    pub orig_prim: *mut libc::c_double,
    pub orig_dual: *mut libc::c_double,
    pub orig_obj: libc::c_double,
    pub nslots: libc::c_int,
    pub avail: libc::c_int,
    pub slot: *mut IOSLOT,
    pub head: *mut IOSNPD,
    pub tail: *mut IOSNPD,
    pub a_cnt: libc::c_int,
    pub n_cnt: libc::c_int,
    pub t_cnt: libc::c_int,
    pub root_m: libc::c_int,
    pub root_type: *mut libc::c_uchar,
    pub root_lb: *mut libc::c_double,
    pub root_ub: *mut libc::c_double,
    pub root_stat: *mut libc::c_uchar,
    pub curr: *mut IOSNPD,
    pub mip: *mut glp_prob,
    pub non_int: *mut libc::c_uchar,
    pub pred_m: libc::c_int,
    pub pred_max: libc::c_int,
    pub pred_type: *mut libc::c_uchar,
    pub pred_lb: *mut libc::c_double,
    pub pred_ub: *mut libc::c_double,
    pub pred_stat: *mut libc::c_uchar,
    pub local: *mut IOSPOOL,
    pub cov_gen: *mut glp_cov,
    pub mir_gen: *mut glp_mir,
    pub clq_gen: *mut glp_cfg,
    pub pcost: *mut libc::c_void,
    pub iwrk: *mut libc::c_int,
    pub dwrk: *mut libc::c_double,
    pub parm: *const glp_iocp,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
    pub sol_cnt: libc::c_int,
    pub P: *mut libc::c_void,
    pub npp: *mut libc::c_void,
    pub save_sol: *const libc::c_char,
    pub save_cnt: libc::c_int,
    pub reason: libc::c_int,
    pub stop: libc::c_int,
    pub next_p: libc::c_int,
    pub reopt: libc::c_int,
    pub reinv: libc::c_int,
    pub br_var: libc::c_int,
    pub br_sel: libc::c_int,
    pub child: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: libc::c_int,
    pub br_tech: libc::c_int,
    pub bt_tech: libc::c_int,
    pub tol_int: libc::c_double,
    pub tol_obj: libc::c_double,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub cb_func: Option::<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> ()>,
    pub cb_info: *mut libc::c_void,
    pub cb_size: libc::c_int,
    pub pp_tech: libc::c_int,
    pub mip_gap: libc::c_double,
    pub mir_cuts: libc::c_int,
    pub gmi_cuts: libc::c_int,
    pub cov_cuts: libc::c_int,
    pub clq_cuts: libc::c_int,
    pub presolve: libc::c_int,
    pub binarize: libc::c_int,
    pub fp_heur: libc::c_int,
    pub ps_heur: libc::c_int,
    pub ps_tm_lim: libc::c_int,
    pub sr_heur: libc::c_int,
    pub use_sol: libc::c_int,
    pub save_sol: *const libc::c_char,
    pub alien: libc::c_int,
    pub flip: libc::c_int,
    pub foo_bar: [libc::c_double; 23],
}
pub type IOSPOOL = glp_prob;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSNPD {
    pub p: libc::c_int,
    pub up: *mut IOSNPD,
    pub level: libc::c_int,
    pub count: libc::c_int,
    pub b_ptr: *mut IOSBND,
    pub s_ptr: *mut IOSTAT,
    pub r_ptr: *mut IOSROW,
    pub solved: libc::c_int,
    pub lp_obj: libc::c_double,
    pub bound: libc::c_double,
    pub ii_cnt: libc::c_int,
    pub ii_sum: libc::c_double,
    pub changed: libc::c_int,
    pub br_var: libc::c_int,
    pub br_val: libc::c_double,
    pub data: *mut libc::c_void,
    pub temp: *mut IOSNPD,
    pub prev: *mut IOSNPD,
    pub next: *mut IOSNPD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSROW {
    pub name: *mut libc::c_char,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut IOSAIJ,
    pub rii: libc::c_double,
    pub stat: libc::c_uchar,
    pub next: *mut IOSROW,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSAIJ {
    pub j: libc::c_int,
    pub val: libc::c_double,
    pub next: *mut IOSAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSTAT {
    pub k: libc::c_int,
    pub stat: libc::c_uchar,
    pub next: *mut IOSTAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSBND {
    pub k: libc::c_int,
    pub type_0: libc::c_uchar,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub next: *mut IOSBND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSLOT {
    pub node: *mut IOSNPD,
    pub next: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_bfcp {
    pub msg_lev: libc::c_int,
    pub type_0: libc::c_int,
    pub lu_size: libc::c_int,
    pub piv_tol: libc::c_double,
    pub piv_lim: libc::c_int,
    pub suhl: libc::c_int,
    pub eps_tol: libc::c_double,
    pub max_gro: libc::c_double,
    pub nfs_max: libc::c_int,
    pub upd_tol: libc::c_double,
    pub nrs_max: libc::c_int,
    pub rs_size: libc::c_int,
    pub foo_bar: [libc::c_double; 38],
}
unsafe extern "C" fn create_prob(mut lp: *mut glp_prob) {
    (*lp).pool = _glp_dmp_create_pool();
    (*lp).tree = 0 as *mut glp_tree;
    (*lp).name = 0 as *mut libc::c_char;
    (*lp).obj = 0 as *mut libc::c_char;
    (*lp).dir = 1 as libc::c_int;
    (*lp).c0 = 0.0f64;
    (*lp).m_max = 100 as libc::c_int;
    (*lp).n_max = 200 as libc::c_int;
    (*lp).n = 0 as libc::c_int;
    (*lp).m = (*lp).n;
    (*lp).nnz = 0 as libc::c_int;
    (*lp)
        .row = glp_alloc(
        1 as libc::c_int + (*lp).m_max,
        ::std::mem::size_of::<*mut GLPROW>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut GLPROW;
    (*lp)
        .col = glp_alloc(
        1 as libc::c_int + (*lp).n_max,
        ::std::mem::size_of::<*mut GLPCOL>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut GLPCOL;
    (*lp).c_tree = 0 as *mut AVL;
    (*lp).r_tree = (*lp).c_tree;
    (*lp).valid = 0 as libc::c_int;
    (*lp)
        .head = glp_alloc(
        1 as libc::c_int + (*lp).m_max,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*lp).bfd = 0 as *mut BFD;
    (*lp).dbs_stat = 1 as libc::c_int;
    (*lp).pbs_stat = (*lp).dbs_stat;
    (*lp).obj_val = 0.0f64;
    (*lp).it_cnt = 0 as libc::c_int;
    (*lp).some = 0 as libc::c_int;
    (*lp).ipt_stat = 1 as libc::c_int;
    (*lp).ipt_obj = 0.0f64;
    (*lp).mip_stat = 1 as libc::c_int;
    (*lp).mip_obj = 0.0f64;
}
pub unsafe extern "C" fn glp_create_prob() -> *mut glp_prob {
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
    lp = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<glp_prob>() as libc::c_ulong as libc::c_int,
    ) as *mut glp_prob;
    create_prob(lp);
    return lp;
}
pub unsafe extern "C" fn glp_set_prob_name(
    mut lp: *mut glp_prob,
    mut name: *const libc::c_char,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_prob_name: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((*lp).name).is_null() {
        _glp_dmp_free_atom(
            (*lp).pool,
            (*lp).name as *mut libc::c_void,
            (strlen((*lp).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        );
        (*lp).name = 0 as *mut libc::c_char;
    }
    if !(name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32)
    {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while *name.offset(k as isize) as libc::c_int != '\0' as i32 {
            if k == 256 as libc::c_int {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    144 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_prob_name: problem name too long\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if *(*__ctype_b_loc())
                .offset(
                    *name.offset(k as isize) as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    146 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_prob_name: problem name contains invalid character(s)\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            k += 1;
            k;
        }
        (*lp)
            .name = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*lp).name, name);
    }
}
pub unsafe extern "C" fn glp_set_obj_name(
    mut lp: *mut glp_prob,
    mut name: *const libc::c_char,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_obj_name: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((*lp).obj).is_null() {
        _glp_dmp_free_atom(
            (*lp).pool,
            (*lp).obj as *mut libc::c_void,
            (strlen((*lp).obj)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        );
        (*lp).obj = 0 as *mut libc::c_char;
    }
    if !(name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32)
    {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while *name.offset(k as isize) as libc::c_int != '\0' as i32 {
            if k == 256 as libc::c_int {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    185 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_obj_name: objective name too long\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if *(*__ctype_b_loc())
                .offset(
                    *name.offset(k as isize) as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    187 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_obj_name: objective name contains invalid character(s)\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            k += 1;
            k;
        }
        (*lp)
            .obj = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*lp).obj, name);
    }
}
pub unsafe extern "C" fn glp_set_obj_dir(mut lp: *mut glp_prob, mut dir: libc::c_int) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_obj_dir: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(dir == 1 as libc::c_int || dir == 2 as libc::c_int) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_obj_dir: dir = %d; invalid direction flag\n\0" as *const u8
                as *const libc::c_char,
            dir,
        );
    }
    (*lp).dir = dir;
}
pub unsafe extern "C" fn glp_add_rows(
    mut lp: *mut glp_prob,
    mut nrs: libc::c_int,
) -> libc::c_int {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut m_new: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if nrs < 1 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int,
        ))
            .unwrap()(
            b"glp_add_rows: nrs = %d; invalid number of rows\n\0" as *const u8
                as *const libc::c_char,
            nrs,
        );
    }
    if nrs > 100000000 as libc::c_int - (*lp).m {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
        ))
            .unwrap()(
            b"glp_add_rows: nrs = %d; too many rows\n\0" as *const u8
                as *const libc::c_char,
            nrs,
        );
    }
    m_new = (*lp).m + nrs;
    if (*lp).m_max < m_new {
        let mut save: *mut *mut GLPROW = (*lp).row;
        while (*lp).m_max < m_new {
            (*lp).m_max += (*lp).m_max;
            ((*lp).m_max > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"lp->m_max > 0\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        264 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        (*lp)
            .row = glp_alloc(
            1 as libc::c_int + (*lp).m_max,
            ::std::mem::size_of::<*mut GLPROW>() as libc::c_ulong as libc::c_int,
        ) as *mut *mut GLPROW;
        memcpy(
            &mut *((*lp).row).offset(1 as libc::c_int as isize) as *mut *mut GLPROW
                as *mut libc::c_void,
            &mut *save.offset(1 as libc::c_int as isize) as *mut *mut GLPROW
                as *const libc::c_void,
            ((*lp).m as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut GLPROW>() as libc::c_ulong),
        );
        glp_free(save as *mut libc::c_void);
        glp_free((*lp).head as *mut libc::c_void);
        (*lp)
            .head = glp_alloc(
            1 as libc::c_int + (*lp).m_max,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
    }
    i = (*lp).m + 1 as libc::c_int;
    while i <= m_new {
        row = _glp_dmp_get_atom(
            (*lp).pool,
            ::std::mem::size_of::<GLPROW>() as libc::c_ulong as libc::c_int,
        ) as *mut GLPROW;
        let ref mut fresh0 = *((*lp).row).offset(i as isize);
        *fresh0 = row;
        (*row).i = i;
        (*row).name = 0 as *mut libc::c_char;
        (*row).node = 0 as *mut AVLNODE;
        (*row).level = 0 as libc::c_int;
        (*row).origin = 0 as libc::c_int as libc::c_uchar;
        (*row).klass = 0 as libc::c_int as libc::c_uchar;
        if !tree.is_null() {
            match (*tree).reason {
                0 => {}
                1 => {
                    (!((*tree).curr).is_null()
                        || {
                            glp_assert_(
                                b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                                289 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*row).level = (*(*tree).curr).level;
                    (*row).origin = 1 as libc::c_int as libc::c_uchar;
                }
                4 => {
                    (!((*tree).curr).is_null()
                        || {
                            glp_assert_(
                                b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                                294 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*row).level = (*(*tree).curr).level;
                    (*row).origin = 2 as libc::c_int as libc::c_uchar;
                }
                _ => {
                    (tree != tree
                        || {
                            glp_assert_(
                                b"tree != tree\0" as *const u8 as *const libc::c_char,
                                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                                299 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        (*row).type_0 = 1 as libc::c_int;
        (*row).ub = 0.0f64;
        (*row).lb = (*row).ub;
        (*row).ptr = 0 as *mut GLPAIJ;
        (*row).rii = 1.0f64;
        (*row).stat = 1 as libc::c_int;
        (*row).bind = 0 as libc::c_int;
        (*row).dual = 0.0f64;
        (*row).prim = (*row).dual;
        (*row).dval = 0.0f64;
        (*row).pval = (*row).dval;
        (*row).mipx = 0.0f64;
        i += 1;
        i;
    }
    (*lp).m = m_new;
    (*lp).valid = 0 as libc::c_int;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (*tree).reopt = 1 as libc::c_int;
    }
    return m_new - nrs + 1 as libc::c_int;
}
pub unsafe extern "C" fn glp_add_cols(
    mut lp: *mut glp_prob,
    mut ncs: libc::c_int,
) -> libc::c_int {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut n_new: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            357 as libc::c_int,
        ))
            .unwrap()(
            b"glp_add_cols: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ncs < 1 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
        ))
            .unwrap()(
            b"glp_add_cols: ncs = %d; invalid number of columns\n\0" as *const u8
                as *const libc::c_char,
            ncs,
        );
    }
    if ncs > 100000000 as libc::c_int - (*lp).n {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int,
        ))
            .unwrap()(
            b"glp_add_cols: ncs = %d; too many columns\n\0" as *const u8
                as *const libc::c_char,
            ncs,
        );
    }
    n_new = (*lp).n + ncs;
    if (*lp).n_max < n_new {
        let mut save: *mut *mut GLPCOL = (*lp).col;
        while (*lp).n_max < n_new {
            (*lp).n_max += (*lp).n_max;
            ((*lp).n_max > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"lp->n_max > 0\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        370 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        (*lp)
            .col = glp_alloc(
            1 as libc::c_int + (*lp).n_max,
            ::std::mem::size_of::<*mut GLPCOL>() as libc::c_ulong as libc::c_int,
        ) as *mut *mut GLPCOL;
        memcpy(
            &mut *((*lp).col).offset(1 as libc::c_int as isize) as *mut *mut GLPCOL
                as *mut libc::c_void,
            &mut *save.offset(1 as libc::c_int as isize) as *mut *mut GLPCOL
                as *const libc::c_void,
            ((*lp).n as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut GLPCOL>() as libc::c_ulong),
        );
        glp_free(save as *mut libc::c_void);
    }
    j = (*lp).n + 1 as libc::c_int;
    while j <= n_new {
        col = _glp_dmp_get_atom(
            (*lp).pool,
            ::std::mem::size_of::<GLPCOL>() as libc::c_ulong as libc::c_int,
        ) as *mut GLPCOL;
        let ref mut fresh1 = *((*lp).col).offset(j as isize);
        *fresh1 = col;
        (*col).j = j;
        (*col).name = 0 as *mut libc::c_char;
        (*col).node = 0 as *mut AVLNODE;
        (*col).kind = 1 as libc::c_int;
        (*col).type_0 = 5 as libc::c_int;
        (*col).ub = 0.0f64;
        (*col).lb = (*col).ub;
        (*col).coef = 0.0f64;
        (*col).ptr = 0 as *mut GLPAIJ;
        (*col).sjj = 1.0f64;
        (*col).stat = 5 as libc::c_int;
        (*col).bind = 0 as libc::c_int;
        (*col).dual = 0.0f64;
        (*col).prim = (*col).dual;
        (*col).dval = 0.0f64;
        (*col).pval = (*col).dval;
        (*col).mipx = 0.0f64;
        j += 1;
        j;
    }
    (*lp).n = n_new;
    return n_new - ncs + 1 as libc::c_int;
}
pub unsafe extern "C" fn glp_set_row_name(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
    mut name: *const libc::c_char,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            427 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_row_name: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    row = *((*lp).row).offset(i as isize);
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (!((*tree).curr).is_null()
            || {
                glp_assert_(
                    b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    431 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*row).level == (*(*tree).curr).level
            || {
                glp_assert_(
                    b"row->level == tree->curr->level\0" as *const u8
                        as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    432 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    if !((*row).name).is_null() {
        if !((*row).node).is_null() {
            (!((*lp).r_tree).is_null()
                || {
                    glp_assert_(
                        b"lp->r_tree != NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        436 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_avl_delete_node((*lp).r_tree, (*row).node);
            (*row).node = 0 as *mut AVLNODE;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            (*row).name as *mut libc::c_void,
            (strlen((*row).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        );
        (*row).name = 0 as *mut libc::c_char;
    }
    if !(name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32)
    {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while *name.offset(k as isize) as libc::c_int != '\0' as i32 {
            if k == 256 as libc::c_int {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    447 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_row_name: i = %d; row name too long\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
            }
            if *(*__ctype_b_loc())
                .offset(
                    *name.offset(k as isize) as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    450 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_row_name: i = %d: row name contains invalid character(s)\n\0"
                        as *const u8 as *const libc::c_char,
                    i,
                );
            }
            k += 1;
            k;
        }
        (*row)
            .name = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*row).name, name);
        if !((*lp).r_tree).is_null() {
            (((*row).node).is_null()
                || {
                    glp_assert_(
                        b"row->node == NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        456 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*row)
                .node = _glp_avl_insert_node(
                (*lp).r_tree,
                (*row).name as *const libc::c_void,
            );
            _glp_avl_set_node_link((*row).node, row as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn glp_set_col_name(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
    mut name: *const libc::c_char,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_col_name: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_col_name: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    col = *((*lp).col).offset(j as isize);
    if !((*col).name).is_null() {
        if !((*col).node).is_null() {
            (!((*lp).c_tree).is_null()
                || {
                    glp_assert_(
                        b"lp->c_tree != NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        493 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_avl_delete_node((*lp).c_tree, (*col).node);
            (*col).node = 0 as *mut AVLNODE;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            (*col).name as *mut libc::c_void,
            (strlen((*col).name)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        );
        (*col).name = 0 as *mut libc::c_char;
    }
    if !(name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32)
    {
        let mut k: libc::c_int = 0;
        k = 0 as libc::c_int;
        while *name.offset(k as isize) as libc::c_int != '\0' as i32 {
            if k == 256 as libc::c_int {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    504 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_col_name: j = %d; column name too long\n\0" as *const u8
                        as *const libc::c_char,
                    j,
                );
            }
            if *(*__ctype_b_loc())
                .offset(
                    *name.offset(k as isize) as libc::c_uchar as libc::c_int as isize,
                ) as libc::c_int
                & _IScntrl as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    507 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_set_col_name: j = %d: column name contains invalid character(s)\n\0"
                        as *const u8 as *const libc::c_char,
                    j,
                );
            }
            k += 1;
            k;
        }
        (*col)
            .name = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*col).name, name);
        if !((*lp).c_tree).is_null() && !((*col).name).is_null() {
            (((*col).node).is_null()
                || {
                    glp_assert_(
                        b"col->node == NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        513 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*col)
                .node = _glp_avl_insert_node(
                (*lp).c_tree,
                (*col).name as *const libc::c_void,
            );
            _glp_avl_set_node_link((*col).node, col as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn glp_set_row_bnds(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
    mut type_0: libc::c_int,
    mut lb: libc::c_double,
    mut ub: libc::c_double,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            559 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_row_bnds: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    row = *((*lp).row).offset(i as isize);
    (*row).type_0 = type_0;
    match type_0 {
        1 => {
            (*row).ub = 0.0f64;
            (*row).lb = (*row).ub;
            if (*row).stat != 1 as libc::c_int {
                (*row).stat = 4 as libc::c_int;
            }
        }
        2 => {
            (*row).lb = lb;
            (*row).ub = 0.0f64;
            if (*row).stat != 1 as libc::c_int {
                (*row).stat = 2 as libc::c_int;
            }
        }
        3 => {
            (*row).lb = 0.0f64;
            (*row).ub = ub;
            if (*row).stat != 1 as libc::c_int {
                (*row).stat = 3 as libc::c_int;
            }
        }
        4 => {
            (*row).lb = lb;
            (*row).ub = ub;
            if !((*row).stat == 1 as libc::c_int || (*row).stat == 2 as libc::c_int
                || (*row).stat == 3 as libc::c_int)
            {
                (*row)
                    .stat = if fabs(lb) <= fabs(ub) {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                };
            }
        }
        5 => {
            (*row).ub = lb;
            (*row).lb = (*row).ub;
            if (*row).stat != 1 as libc::c_int {
                (*row).stat = 5 as libc::c_int;
            }
        }
        _ => {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                587 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_row_bnds: i = %d; type = %d; invalid row type\n\0" as *const u8
                    as *const libc::c_char,
                i,
                type_0,
            );
        }
    };
}
pub unsafe extern "C" fn glp_set_col_bnds(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
    mut type_0: libc::c_int,
    mut lb: libc::c_double,
    mut ub: libc::c_double,
) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            630 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_col_bnds: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    col = *((*lp).col).offset(j as isize);
    (*col).type_0 = type_0;
    match type_0 {
        1 => {
            (*col).ub = 0.0f64;
            (*col).lb = (*col).ub;
            if (*col).stat != 1 as libc::c_int {
                (*col).stat = 4 as libc::c_int;
            }
        }
        2 => {
            (*col).lb = lb;
            (*col).ub = 0.0f64;
            if (*col).stat != 1 as libc::c_int {
                (*col).stat = 2 as libc::c_int;
            }
        }
        3 => {
            (*col).lb = 0.0f64;
            (*col).ub = ub;
            if (*col).stat != 1 as libc::c_int {
                (*col).stat = 3 as libc::c_int;
            }
        }
        4 => {
            (*col).lb = lb;
            (*col).ub = ub;
            if !((*col).stat == 1 as libc::c_int || (*col).stat == 2 as libc::c_int
                || (*col).stat == 3 as libc::c_int)
            {
                (*col)
                    .stat = if fabs(lb) <= fabs(ub) {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                };
            }
        }
        5 => {
            (*col).ub = lb;
            (*col).lb = (*col).ub;
            if (*col).stat != 1 as libc::c_int {
                (*col).stat = 5 as libc::c_int;
            }
        }
        _ => {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                658 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_col_bnds: j = %d; type = %d; invalid column type\n\0"
                    as *const u8 as *const libc::c_char,
                j,
                type_0,
            );
        }
    };
}
pub unsafe extern "C" fn glp_set_obj_coef(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
    mut coef: libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            684 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_obj_coef: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(0 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            686 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_obj_coef: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    if j == 0 as libc::c_int {
        (*lp).c0 = coef;
    } else {
        (**((*lp).col).offset(j as isize)).coef = coef;
    };
}
pub unsafe extern "C" fn glp_set_mat_row(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut next: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            729 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_mat_row: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    row = *((*lp).row).offset(i as isize);
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (!((*tree).curr).is_null()
            || {
                glp_assert_(
                    b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    733 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*row).level == (*(*tree).curr).level
            || {
                glp_assert_(
                    b"row->level == tree->curr->level\0" as *const u8
                        as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    734 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    while !((*row).ptr).is_null() {
        aij = (*row).ptr;
        (*row).ptr = (*aij).r_next;
        col = (*aij).col;
        if ((*aij).c_prev).is_null() {
            (*col).ptr = (*aij).c_next;
        } else {
            (*(*aij).c_prev).c_next = (*aij).c_next;
        }
        if !((*aij).c_next).is_null() {
            (*(*aij).c_next).c_prev = (*aij).c_prev;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            aij as *mut libc::c_void,
            ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
        );
        (*lp).nnz -= 1;
        (*lp).nnz;
        if (*col).stat == 1 as libc::c_int {
            (*lp).valid = 0 as libc::c_int;
        }
    }
    if !(0 as libc::c_int <= len && len <= (*lp).n) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            761 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_mat_row: i = %d; len = %d; invalid row length \n\0" as *const u8
                as *const libc::c_char,
            i,
            len,
        );
    }
    if len > 500000000 as libc::c_int - (*lp).nnz {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            764 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_mat_row: i = %d; len = %d; too many constraint coefficients\n\0"
                as *const u8 as *const libc::c_char,
            i,
            len,
        );
    }
    k = 1 as libc::c_int;
    while k <= len {
        j = *ind.offset(k as isize);
        if !(1 as libc::c_int <= j && j <= (*lp).n) {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                771 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_mat_row: i = %d; ind[%d] = %d; column index out of range\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                k,
                j,
            );
        }
        col = *((*lp).col).offset(j as isize);
        if !((*col).ptr).is_null() && (*(*(*col).ptr).row).i == i {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                777 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_mat_row: i = %d; ind[%d] = %d; duplicate column indices not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                i,
                k,
                j,
            );
        }
        aij = _glp_dmp_get_atom(
            (*lp).pool,
            ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
        ) as *mut GLPAIJ;
        (*lp).nnz += 1;
        (*lp).nnz;
        (*aij).row = row;
        (*aij).col = col;
        (*aij).val = *val.offset(k as isize);
        (*aij).r_prev = 0 as *mut GLPAIJ;
        (*aij).r_next = (*row).ptr;
        (*aij).c_prev = 0 as *mut GLPAIJ;
        (*aij).c_next = (*col).ptr;
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = aij;
        }
        if !((*aij).c_next).is_null() {
            (*(*aij).c_next).c_prev = aij;
        }
        (*col).ptr = aij;
        (*row).ptr = (*col).ptr;
        if (*col).stat == 1 as libc::c_int && (*aij).val != 0.0f64 {
            (*lp).valid = 0 as libc::c_int;
        }
        k += 1;
        k;
    }
    aij = (*row).ptr;
    while !aij.is_null() {
        next = (*aij).r_next;
        if (*aij).val == 0.0f64 {
            if ((*aij).r_prev).is_null() {
                (*row).ptr = next;
            } else {
                (*(*aij).r_prev).r_next = next;
            }
            if !next.is_null() {
                (*next).r_prev = (*aij).r_prev;
            }
            (((*aij).c_prev).is_null()
                || {
                    glp_assert_(
                        b"aij->c_prev == NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        811 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*(*aij).col).ptr = (*aij).c_next;
            if !((*aij).c_next).is_null() {
                (*(*aij).c_next).c_prev = 0 as *mut GLPAIJ;
            }
            _glp_dmp_free_atom(
                (*lp).pool,
                aij as *mut libc::c_void,
                ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
            );
            (*lp).nnz -= 1;
            (*lp).nnz;
        }
        aij = next;
    }
}
pub unsafe extern "C" fn glp_set_mat_col(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
    mut len: libc::c_int,
    mut ind: *const libc::c_int,
    mut val: *const libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut next: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            854 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_mat_col: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            857 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_mat_col: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    col = *((*lp).col).offset(j as isize);
    while !((*col).ptr).is_null() {
        aij = (*col).ptr;
        (*col).ptr = (*aij).c_next;
        row = (*aij).row;
        if ((*aij).r_prev).is_null() {
            (*row).ptr = (*aij).r_next;
        } else {
            (*(*aij).r_prev).r_next = (*aij).r_next;
        }
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = (*aij).r_prev;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            aij as *mut libc::c_void,
            ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
        );
        (*lp).nnz -= 1;
        (*lp).nnz;
    }
    if !(0 as libc::c_int <= len && len <= (*lp).m) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            882 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_mat_col: j = %d; len = %d; invalid column length\n\0" as *const u8
                as *const libc::c_char,
            j,
            len,
        );
    }
    if len > 500000000 as libc::c_int - (*lp).nnz {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            885 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_mat_col: j = %d; len = %d; too many constraint coefficients\n\0"
                as *const u8 as *const libc::c_char,
            j,
            len,
        );
    }
    k = 1 as libc::c_int;
    while k <= len {
        i = *ind.offset(k as isize);
        if !(1 as libc::c_int <= i && i <= (*lp).m) {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                892 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_mat_col: j = %d; ind[%d] = %d; row index out of range\n\0"
                    as *const u8 as *const libc::c_char,
                j,
                k,
                i,
            );
        }
        row = *((*lp).row).offset(i as isize);
        if !((*row).ptr).is_null() && (*(*(*row).ptr).col).j == j {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                898 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_mat_col: j = %d; ind[%d] = %d; duplicate row indices not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                j,
                k,
                i,
            );
        }
        aij = _glp_dmp_get_atom(
            (*lp).pool,
            ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
        ) as *mut GLPAIJ;
        (*lp).nnz += 1;
        (*lp).nnz;
        (*aij).row = row;
        (*aij).col = col;
        (*aij).val = *val.offset(k as isize);
        (*aij).r_prev = 0 as *mut GLPAIJ;
        (*aij).r_next = (*row).ptr;
        (*aij).c_prev = 0 as *mut GLPAIJ;
        (*aij).c_next = (*col).ptr;
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = aij;
        }
        if !((*aij).c_next).is_null() {
            (*(*aij).c_next).c_prev = aij;
        }
        (*col).ptr = aij;
        (*row).ptr = (*col).ptr;
        k += 1;
        k;
    }
    aij = (*col).ptr;
    while !aij.is_null() {
        next = (*aij).c_next;
        if (*aij).val == 0.0f64 {
            (((*aij).r_prev).is_null()
                || {
                    glp_assert_(
                        b"aij->r_prev == NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        920 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*(*aij).row).ptr = (*aij).r_next;
            if !((*aij).r_next).is_null() {
                (*(*aij).r_next).r_prev = 0 as *mut GLPAIJ;
            }
            if ((*aij).c_prev).is_null() {
                (*col).ptr = next;
            } else {
                (*(*aij).c_prev).c_next = next;
            }
            if !next.is_null() {
                (*next).c_prev = (*aij).c_prev;
            }
            _glp_dmp_free_atom(
                (*lp).pool,
                aij as *mut libc::c_void,
                ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
            );
            (*lp).nnz -= 1;
            (*lp).nnz;
        }
        aij = next;
    }
    if (*col).stat == 1 as libc::c_int {
        (*lp).valid = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn glp_load_matrix(
    mut lp: *mut glp_prob,
    mut ne: libc::c_int,
    mut ia: *const libc::c_int,
    mut ja: *const libc::c_int,
    mut ar: *const libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut next: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            977 as libc::c_int,
        ))
            .unwrap()(
            b"glp_load_matrix: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 1 as libc::c_int;
    while i <= (*lp).m {
        row = *((*lp).row).offset(i as isize);
        while !((*row).ptr).is_null() {
            aij = (*row).ptr;
            (*row).ptr = (*aij).r_next;
            _glp_dmp_free_atom(
                (*lp).pool,
                aij as *mut libc::c_void,
                ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
            );
            (*lp).nnz -= 1;
            (*lp).nnz;
        }
        i += 1;
        i;
    }
    ((*lp).nnz == 0 as libc::c_int
        || {
            glp_assert_(
                b"lp->nnz == 0\0" as *const u8 as *const libc::c_char,
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                987 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*lp).n {
        let ref mut fresh2 = (**((*lp).col).offset(j as isize)).ptr;
        *fresh2 = 0 as *mut GLPAIJ;
        j += 1;
        j;
    }
    if ne < 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            992 as libc::c_int,
        ))
            .unwrap()(
            b"glp_load_matrix: ne = %d; invalid number of constraint coefficients\n\0"
                as *const u8 as *const libc::c_char,
            ne,
        );
    }
    if ne > 500000000 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            995 as libc::c_int,
        ))
            .unwrap()(
            b"glp_load_matrix: ne = %d; too many constraint coefficients\n\0"
                as *const u8 as *const libc::c_char,
            ne,
        );
    }
    k = 1 as libc::c_int;
    while k <= ne {
        i = *ia.offset(k as isize);
        j = *ja.offset(k as isize);
        if !(1 as libc::c_int <= i && i <= (*lp).m) {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1002 as libc::c_int,
            ))
                .unwrap()(
                b"glp_load_matrix: ia[%d] = %d; row index out of range\n\0" as *const u8
                    as *const libc::c_char,
                k,
                i,
            );
        }
        row = *((*lp).row).offset(i as isize);
        if !(1 as libc::c_int <= j && j <= (*lp).n) {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1007 as libc::c_int,
            ))
                .unwrap()(
                b"glp_load_matrix: ja[%d] = %d; column index out of range\n\0"
                    as *const u8 as *const libc::c_char,
                k,
                j,
            );
        }
        col = *((*lp).col).offset(j as isize);
        aij = _glp_dmp_get_atom(
            (*lp).pool,
            ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
        ) as *mut GLPAIJ;
        (*lp).nnz += 1;
        (*lp).nnz;
        (*aij).row = row;
        (*aij).col = col;
        (*aij).val = *ar.offset(k as isize);
        (*aij).r_prev = 0 as *mut GLPAIJ;
        (*aij).r_next = (*row).ptr;
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = aij;
        }
        (*row).ptr = aij;
        k += 1;
        k;
    }
    ((*lp).nnz == ne
        || {
            glp_assert_(
                b"lp->nnz == ne\0" as *const u8 as *const libc::c_char,
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1021 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*lp).m {
        aij = (**((*lp).row).offset(i as isize)).ptr;
        while !aij.is_null() {
            col = (*aij).col;
            if !((*col).ptr).is_null() && (*(*(*col).ptr).row).i == i {
                k = 1 as libc::c_int;
                while k <= ne {
                    if *ia.offset(k as isize) == i && *ja.offset(k as isize) == (*col).j
                    {
                        break;
                    }
                    k += 1;
                    k;
                }
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1033 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_load_mat: ia[%d] = %d; ja[%d] = %d; duplicate indices not allowed\n\0"
                        as *const u8 as *const libc::c_char,
                    k,
                    i,
                    k,
                    (*col).j,
                );
            }
            (*aij).c_prev = 0 as *mut GLPAIJ;
            (*aij).c_next = (*col).ptr;
            if !((*aij).c_next).is_null() {
                (*(*aij).c_next).c_prev = aij;
            }
            (*col).ptr = aij;
            aij = (*aij).r_next;
        }
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= (*lp).m {
        row = *((*lp).row).offset(i as isize);
        aij = (*row).ptr;
        while !aij.is_null() {
            next = (*aij).r_next;
            if (*aij).val == 0.0f64 {
                if ((*aij).r_prev).is_null() {
                    (*row).ptr = next;
                } else {
                    (*(*aij).r_prev).r_next = next;
                }
                if !next.is_null() {
                    (*next).r_prev = (*aij).r_prev;
                }
                if ((*aij).c_prev).is_null() {
                    (*(*aij).col).ptr = (*aij).c_next;
                } else {
                    (*(*aij).c_prev).c_next = (*aij).c_next;
                }
                if !((*aij).c_next).is_null() {
                    (*(*aij).c_next).c_prev = (*aij).c_prev;
                }
                _glp_dmp_free_atom(
                    (*lp).pool,
                    aij as *mut libc::c_void,
                    ::std::mem::size_of::<GLPAIJ>() as libc::c_ulong as libc::c_int,
                );
                (*lp).nnz -= 1;
                (*lp).nnz;
            }
            aij = next;
        }
        i += 1;
        i;
    }
    (*lp).valid = 0 as libc::c_int;
}
pub unsafe extern "C" fn glp_check_dup(
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut ne: libc::c_int,
    mut ia: *const libc::c_int,
    mut ja: *const libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut next: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    if m < 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1120 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_dup: m = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if n < 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1122 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_dup: n = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ne < 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1124 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_dup: ne = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if ne > 0 as libc::c_int && ia.is_null() {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1126 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_dup: ia = %p; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            ia,
        );
    }
    if ne > 0 as libc::c_int && ja.is_null() {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1128 as libc::c_int,
        ))
            .unwrap()(
            b"glp_check_dup: ja = %p; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            ja,
        );
    }
    k = 1 as libc::c_int;
    loop {
        if !(k <= ne) {
            current_block = 4166486009154926805;
            break;
        }
        i = *ia.offset(k as isize);
        j = *ja.offset(k as isize);
        if !(1 as libc::c_int <= i && i <= m && 1 as libc::c_int <= j && j <= n) {
            ret = -k;
            current_block = 8894124077600403767;
            break;
        } else {
            k += 1;
            k;
        }
    }
    match current_block {
        4166486009154926805 => {
            if m == 0 as libc::c_int || n == 0 as libc::c_int {
                ret = 0 as libc::c_int;
            } else {
                ptr = glp_alloc(
                    1 as libc::c_int + m,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                ) as *mut libc::c_int;
                next = glp_alloc(
                    1 as libc::c_int + ne,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                ) as *mut libc::c_int;
                flag = glp_alloc(
                    1 as libc::c_int + n,
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
                ) as *mut libc::c_char;
                i = 1 as libc::c_int;
                while i <= m {
                    *ptr.offset(i as isize) = 0 as libc::c_int;
                    i += 1;
                    i;
                }
                k = 1 as libc::c_int;
                while k <= ne {
                    i = *ia.offset(k as isize);
                    *next.offset(k as isize) = *ptr.offset(i as isize);
                    *ptr.offset(i as isize) = k;
                    k += 1;
                    k;
                }
                j = 1 as libc::c_int;
                while j <= n {
                    *flag.offset(j as isize) = 0 as libc::c_int as libc::c_char;
                    j += 1;
                    j;
                }
                i = 1 as libc::c_int;
                's_143: loop {
                    if !(i <= m) {
                        current_block = 3160140712158701372;
                        break;
                    }
                    k = *ptr.offset(i as isize);
                    while k != 0 as libc::c_int {
                        j = *ja.offset(k as isize);
                        if *flag.offset(j as isize) != 0 {
                            k = 1 as libc::c_int;
                            while k <= ne {
                                if *ia.offset(k as isize) == i
                                    && *ja.offset(k as isize) == j
                                {
                                    break;
                                }
                                k += 1;
                                k;
                            }
                            (k <= ne
                                || {
                                    glp_assert_(
                                        b"k <= ne\0" as *const u8 as *const libc::c_char,
                                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                                        1163 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            k += 1;
                            k;
                            while k <= ne {
                                if *ia.offset(k as isize) == i
                                    && *ja.offset(k as isize) == j
                                {
                                    break;
                                }
                                k += 1;
                                k;
                            }
                            (k <= ne
                                || {
                                    glp_assert_(
                                        b"k <= ne\0" as *const u8 as *const libc::c_char,
                                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                                        1167 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            ret = k;
                            current_block = 9549138949565282146;
                            break 's_143;
                        } else {
                            *flag.offset(j as isize) = 1 as libc::c_int as libc::c_char;
                            k = *next.offset(k as isize);
                        }
                    }
                    k = *ptr.offset(i as isize);
                    while k != 0 as libc::c_int {
                        *flag
                            .offset(
                                *ja.offset(k as isize) as isize,
                            ) = 0 as libc::c_int as libc::c_char;
                        k = *next.offset(k as isize);
                    }
                    i += 1;
                    i;
                }
                match current_block {
                    3160140712158701372 => {
                        ret = 0 as libc::c_int;
                    }
                    _ => {}
                }
                glp_free(ptr as *mut libc::c_void);
                glp_free(next as *mut libc::c_void);
                glp_free(flag as *mut libc::c_void);
            }
        }
        _ => {}
    }
    return ret;
}
pub unsafe extern "C" fn glp_sort_matrix(mut P: *mut glp_prob) {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = (*P).m;
    while i >= 1 as libc::c_int {
        let ref mut fresh3 = (**((*P).row).offset(i as isize)).ptr;
        *fresh3 = 0 as *mut GLPAIJ;
        i -= 1;
        i;
    }
    j = (*P).n;
    while j >= 1 as libc::c_int {
        aij = (**((*P).col).offset(j as isize)).ptr;
        while !aij.is_null() {
            i = (*(*aij).row).i;
            (*aij).r_prev = 0 as *mut GLPAIJ;
            (*aij).r_next = (**((*P).row).offset(i as isize)).ptr;
            if !((*aij).r_next).is_null() {
                (*(*aij).r_next).r_prev = aij;
            }
            let ref mut fresh4 = (**((*P).row).offset(i as isize)).ptr;
            *fresh4 = aij;
            aij = (*aij).c_next;
        }
        j -= 1;
        j;
    }
    j = (*P).n;
    while j >= 1 as libc::c_int {
        let ref mut fresh5 = (**((*P).col).offset(j as isize)).ptr;
        *fresh5 = 0 as *mut GLPAIJ;
        j -= 1;
        j;
    }
    i = (*P).m;
    while i >= 1 as libc::c_int {
        aij = (**((*P).row).offset(i as isize)).ptr;
        while !aij.is_null() {
            j = (*(*aij).col).j;
            (*aij).c_prev = 0 as *mut GLPAIJ;
            (*aij).c_next = (**((*P).col).offset(j as isize)).ptr;
            if !((*aij).c_next).is_null() {
                (*(*aij).c_next).c_prev = aij;
            }
            let ref mut fresh6 = (**((*P).col).offset(j as isize)).ptr;
            *fresh6 = aij;
            aij = (*aij).r_next;
        }
        i -= 1;
        i;
    }
}
pub unsafe extern "C" fn glp_del_rows(
    mut lp: *mut glp_prob,
    mut nrs: libc::c_int,
    mut num: *const libc::c_int,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m_new: libc::c_int = 0;
    if !(1 as libc::c_int <= nrs && nrs <= (*lp).m) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1265 as libc::c_int,
        ))
            .unwrap()(
            b"glp_del_rows: nrs = %d; invalid number of rows\n\0" as *const u8
                as *const libc::c_char,
            nrs,
        );
    }
    k = 1 as libc::c_int;
    while k <= nrs {
        i = *num.offset(k as isize);
        if !(1 as libc::c_int <= i && i <= (*lp).m) {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1272 as libc::c_int,
            ))
                .unwrap()(
                b"glp_del_rows: num[%d] = %d; row number out of range\n\0" as *const u8
                    as *const libc::c_char,
                k,
                i,
            );
        }
        row = *((*lp).row).offset(i as isize);
        if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
            if !((*tree).reason == 0x1 as libc::c_int
                || (*tree).reason == 0x4 as libc::c_int)
            {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1278 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_del_rows: operation not allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (!((*tree).curr).is_null()
                || {
                    glp_assert_(
                        b"tree->curr != NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                        1279 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*row).level != (*(*tree).curr).level {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1281 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_del_rows: num[%d] = %d; invalid attempt to delete row created not in current subproblem\n\0"
                        as *const u8 as *const libc::c_char,
                    k,
                    i,
                );
            }
            if (*row).stat != 1 as libc::c_int {
                (glp_error_(
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1284 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_del_rows: num[%d] = %d; invalid attempt to delete active row (constraint)\n\0"
                        as *const u8 as *const libc::c_char,
                    k,
                    i,
                );
            }
            (*tree).reinv = 1 as libc::c_int;
        }
        if (*row).i == 0 as libc::c_int {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1290 as libc::c_int,
            ))
                .unwrap()(
                b"glp_del_rows: num[%d] = %d; duplicate row numbers not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                k,
                i,
            );
        }
        glp_set_row_name(lp, i, 0 as *const libc::c_char);
        (((*row).node).is_null()
            || {
                glp_assert_(
                    b"row->node == NULL\0" as *const u8 as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1294 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        glp_set_mat_row(
            lp,
            i,
            0 as libc::c_int,
            0 as *const libc::c_int,
            0 as *const libc::c_double,
        );
        (((*row).ptr).is_null()
            || {
                glp_assert_(
                    b"row->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1297 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*row).i = 0 as libc::c_int;
        k += 1;
        k;
    }
    m_new = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= (*lp).m {
        row = *((*lp).row).offset(i as isize);
        if (*row).i == 0 as libc::c_int {
            _glp_dmp_free_atom(
                (*lp).pool,
                row as *mut libc::c_void,
                ::std::mem::size_of::<GLPROW>() as libc::c_ulong as libc::c_int,
            );
        } else {
            m_new += 1;
            (*row).i = m_new;
            let ref mut fresh7 = *((*lp).row).offset((*row).i as isize);
            *fresh7 = row;
        }
        i += 1;
        i;
    }
    (*lp).m = m_new;
    (*lp).valid = 0 as libc::c_int;
}
pub unsafe extern "C" fn glp_del_cols(
    mut lp: *mut glp_prob,
    mut ncs: libc::c_int,
    mut num: *const libc::c_int,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n_new: libc::c_int = 0;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1349 as libc::c_int,
        ))
            .unwrap()(
            b"glp_del_cols: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= ncs && ncs <= (*lp).n) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1352 as libc::c_int,
        ))
            .unwrap()(
            b"glp_del_cols: ncs = %d; invalid number of columns\n\0" as *const u8
                as *const libc::c_char,
            ncs,
        );
    }
    k = 1 as libc::c_int;
    while k <= ncs {
        j = *num.offset(k as isize);
        if !(1 as libc::c_int <= j && j <= (*lp).n) {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1359 as libc::c_int,
            ))
                .unwrap()(
                b"glp_del_cols: num[%d] = %d; column number out of range\0" as *const u8
                    as *const libc::c_char,
                k,
                j,
            );
        }
        col = *((*lp).col).offset(j as isize);
        if (*col).j == 0 as libc::c_int {
            (glp_error_(
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1364 as libc::c_int,
            ))
                .unwrap()(
                b"glp_del_cols: num[%d] = %d; duplicate column numbers not allowed\n\0"
                    as *const u8 as *const libc::c_char,
                k,
                j,
            );
        }
        glp_set_col_name(lp, j, 0 as *const libc::c_char);
        (((*col).node).is_null()
            || {
                glp_assert_(
                    b"col->node == NULL\0" as *const u8 as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1368 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        glp_set_mat_col(
            lp,
            j,
            0 as libc::c_int,
            0 as *const libc::c_int,
            0 as *const libc::c_double,
        );
        (((*col).ptr).is_null()
            || {
                glp_assert_(
                    b"col->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                    1371 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*col).j = 0 as libc::c_int;
        if (*col).stat == 1 as libc::c_int {
            (*lp).valid = 0 as libc::c_int;
        }
        k += 1;
        k;
    }
    n_new = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*lp).n {
        col = *((*lp).col).offset(j as isize);
        if (*col).j == 0 as libc::c_int {
            _glp_dmp_free_atom(
                (*lp).pool,
                col as *mut libc::c_void,
                ::std::mem::size_of::<GLPCOL>() as libc::c_ulong as libc::c_int,
            );
        } else {
            n_new += 1;
            (*col).j = n_new;
            let ref mut fresh8 = *((*lp).col).offset((*col).j as isize);
            *fresh8 = col;
        }
        j += 1;
        j;
    }
    (*lp).n = n_new;
    if (*lp).valid != 0 {
        let mut m: libc::c_int = (*lp).m;
        let mut head: *mut libc::c_int = (*lp).head;
        j = 1 as libc::c_int;
        while j <= n_new {
            k = (**((*lp).col).offset(j as isize)).bind;
            if k != 0 as libc::c_int {
                (1 as libc::c_int <= k && k <= m
                    || {
                        glp_assert_(
                            b"1 <= k && k <= m\0" as *const u8 as *const libc::c_char,
                            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                            1402 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                *head.offset(k as isize) = m + j;
            }
            j += 1;
            j;
        }
    }
}
pub unsafe extern "C" fn glp_copy_prob(
    mut dest: *mut glp_prob,
    mut prob: *mut glp_prob,
    mut names: libc::c_int,
) {
    let mut tree: *mut glp_tree = (*dest).tree;
    let mut bfcp: glp_bfcp = glp_bfcp {
        msg_lev: 0,
        type_0: 0,
        lu_size: 0,
        piv_tol: 0.,
        piv_lim: 0,
        suhl: 0,
        eps_tol: 0.,
        max_gro: 0.,
        nfs_max: 0,
        upd_tol: 0.,
        nrs_max: 0,
        rs_size: 0,
        foo_bar: [0.; 38],
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1434 as libc::c_int,
        ))
            .unwrap()(
            b"glp_copy_prob: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if dest == prob {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1436 as libc::c_int,
        ))
            .unwrap()(
            b"glp_copy_prob: copying problem object to itself not allowed\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if !(names == 1 as libc::c_int || names == 0 as libc::c_int) {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1439 as libc::c_int,
        ))
            .unwrap()(
            b"glp_copy_prob: names = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            names,
        );
    }
    glp_erase_prob(dest);
    if names != 0 && !((*prob).name).is_null() {
        glp_set_prob_name(dest, (*prob).name);
    }
    if names != 0 && !((*prob).obj).is_null() {
        glp_set_obj_name(dest, (*prob).obj);
    }
    (*dest).dir = (*prob).dir;
    (*dest).c0 = (*prob).c0;
    if (*prob).m > 0 as libc::c_int {
        glp_add_rows(dest, (*prob).m);
    }
    if (*prob).n > 0 as libc::c_int {
        glp_add_cols(dest, (*prob).n);
    }
    glp_get_bfcp(prob, &mut bfcp);
    glp_set_bfcp(dest, &mut bfcp);
    (*dest).pbs_stat = (*prob).pbs_stat;
    (*dest).dbs_stat = (*prob).dbs_stat;
    (*dest).obj_val = (*prob).obj_val;
    (*dest).some = (*prob).some;
    (*dest).ipt_stat = (*prob).ipt_stat;
    (*dest).ipt_obj = (*prob).ipt_obj;
    (*dest).mip_stat = (*prob).mip_stat;
    (*dest).mip_obj = (*prob).mip_obj;
    i = 1 as libc::c_int;
    while i <= (*prob).m {
        let mut to: *mut GLPROW = *((*dest).row).offset(i as isize);
        let mut from: *mut GLPROW = *((*prob).row).offset(i as isize);
        if names != 0 && !((*from).name).is_null() {
            glp_set_row_name(dest, i, (*from).name);
        }
        (*to).type_0 = (*from).type_0;
        (*to).lb = (*from).lb;
        (*to).ub = (*from).ub;
        (*to).rii = (*from).rii;
        (*to).stat = (*from).stat;
        (*to).prim = (*from).prim;
        (*to).dual = (*from).dual;
        (*to).pval = (*from).pval;
        (*to).dval = (*from).dval;
        (*to).mipx = (*from).mipx;
        i += 1;
        i;
    }
    ind = glp_alloc(
        1 as libc::c_int + (*prob).m,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + (*prob).m,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j <= (*prob).n {
        let mut to_0: *mut GLPCOL = *((*dest).col).offset(j as isize);
        let mut from_0: *mut GLPCOL = *((*prob).col).offset(j as isize);
        if names != 0 && !((*from_0).name).is_null() {
            glp_set_col_name(dest, j, (*from_0).name);
        }
        (*to_0).kind = (*from_0).kind;
        (*to_0).type_0 = (*from_0).type_0;
        (*to_0).lb = (*from_0).lb;
        (*to_0).ub = (*from_0).ub;
        (*to_0).coef = (*from_0).coef;
        len = glp_get_mat_col(prob, j, ind, val);
        glp_set_mat_col(
            dest,
            j,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
        );
        (*to_0).sjj = (*from_0).sjj;
        (*to_0).stat = (*from_0).stat;
        (*to_0).prim = (*from_0).prim;
        (*to_0).dual = (*from_0).dual;
        (*to_0).pval = (*from_0).pval;
        (*to_0).dval = (*from_0).dval;
        (*to_0).mipx = (*from_0).mipx;
        j += 1;
        j;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
pub unsafe extern "C" fn glp_erase_prob(mut lp: *mut glp_prob) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1527 as libc::c_int,
        ))
            .unwrap()(
            b"glp_erase_prob: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    delete_prob(lp);
    create_prob(lp);
}
unsafe extern "C" fn delete_prob(mut lp: *mut glp_prob) {
    _glp_dmp_delete_pool((*lp).pool);
    (((*lp).tree).is_null()
        || {
            glp_assert_(
                b"lp->tree == NULL\0" as *const u8 as *const libc::c_char,
                b"api/prob1.c\0" as *const u8 as *const libc::c_char,
                1561 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free((*lp).row as *mut libc::c_void);
    glp_free((*lp).col as *mut libc::c_void);
    if !((*lp).r_tree).is_null() {
        _glp_avl_delete_tree((*lp).r_tree);
    }
    if !((*lp).c_tree).is_null() {
        _glp_avl_delete_tree((*lp).c_tree);
    }
    glp_free((*lp).head as *mut libc::c_void);
    if !((*lp).bfd).is_null() {
        _glp_bfd_delete_it((*lp).bfd);
    }
}
pub unsafe extern "C" fn glp_delete_prob(mut lp: *mut glp_prob) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"api/prob1.c\0" as *const u8 as *const libc::c_char,
            1580 as libc::c_int,
        ))
            .unwrap()(
            b"glp_delete_prob: operation not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    delete_prob(lp);
    glp_free(lp as *mut libc::c_void);
}
