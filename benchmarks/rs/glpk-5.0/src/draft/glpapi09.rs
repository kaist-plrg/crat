use ::libc;
extern "C" {
    pub type BFD;
    pub type AVL;
    pub type AVLNODE;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    pub type DMP;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: libc::c_int,
        type_0: libc::c_int,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_scale_prob(P: *mut glp_prob, flags: libc::c_int);
    fn glp_adv_basis(P: *mut glp_prob, flags: libc::c_int);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn glp_version() -> *const libc::c_char;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn floor(_: libc::c_double) -> libc::c_double;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn _glp_ios_delete_tree(tree: *mut glp_tree);
    fn _glp_ios_driver(tree: *mut glp_tree) -> libc::c_int;
    fn _glp_ios_create_tree(mip: *mut glp_prob, parm: *const glp_iocp) -> *mut glp_tree;
    fn _glp_npp_create_wksp() -> *mut NPP;
    fn _glp_npp_load_prob(
        npp: *mut NPP,
        orig: *mut glp_prob,
        names: libc::c_int,
        sol: libc::c_int,
        scaling: libc::c_int,
    );
    fn _glp_npp_build_prob(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
    fn _glp_npp_delete_wksp(npp: *mut NPP);
    fn _glp_npp_integer(npp: *mut NPP, parm: *const glp_iocp) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_smcp {
    pub msg_lev: libc::c_int,
    pub meth: libc::c_int,
    pub pricing: libc::c_int,
    pub r_test: libc::c_int,
    pub tol_bnd: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_ll: libc::c_double,
    pub obj_ul: libc::c_double,
    pub it_lim: libc::c_int,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub presolve: libc::c_int,
    pub excl: libc::c_int,
    pub shift: libc::c_int,
    pub aorn: libc::c_int,
    pub foo_bar: [libc::c_double; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prep {
    pub orig_dir: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_n: libc::c_int,
    pub orig_nnz: libc::c_int,
    pub pool: *mut DMP,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub c0: libc::c_double,
    pub nrows: libc::c_int,
    pub ncols: libc::c_int,
    pub r_head: *mut NPPROW,
    pub r_tail: *mut NPPROW,
    pub c_head: *mut NPPCOL,
    pub c_tail: *mut NPPCOL,
    pub stack: *mut DMP,
    pub top: *mut NPPTSE,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row_ref: *mut libc::c_int,
    pub col_ref: *mut libc::c_int,
    pub sol: libc::c_int,
    pub scaling: libc::c_int,
    pub p_stat: libc::c_int,
    pub d_stat: libc::c_int,
    pub t_stat: libc::c_int,
    pub i_stat: libc::c_int,
    pub r_stat: *mut libc::c_char,
    pub c_stat: *mut libc::c_char,
    pub r_pi: *mut libc::c_double,
    pub c_value: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPTSE {
    pub func: Option::<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> libc::c_int>,
    pub info: *mut libc::c_void,
    pub link: *mut NPPTSE,
}
pub type NPP = glp_prep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub is_int: libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub ll: C2RustUnnamed_0,
    pub uu: C2RustUnnamed,
    pub prev: *mut NPPCOL,
    pub next: *mut NPPCOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uu: libc::c_double,
    pub neg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ll: libc::c_double,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPAIJ {
    pub row: *mut NPPROW,
    pub col: *mut NPPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut NPPAIJ,
    pub r_next: *mut NPPAIJ,
    pub c_prev: *mut NPPAIJ,
    pub c_next: *mut NPPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPROW {
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: libc::c_int,
    pub prev: *mut NPPROW,
    pub next: *mut NPPROW,
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut libc::c_char,
    pub term_out: libc::c_int,
    pub term_hook: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: libc::c_int,
    pub err_file: *const libc::c_char,
    pub err_line: libc::c_int,
    pub err_hook: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut libc::c_char,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: libc::c_int,
    pub mem_cpeak: libc::c_int,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: libc::c_int,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub unsafe extern "C" fn glp_set_col_kind(
    mut mip: *mut glp_prob,
    mut j: libc::c_int,
    mut kind: libc::c_int,
) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !(1 as libc::c_int <= j && j <= (*mip).n) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
        ))
            .unwrap()(
            b"glp_set_col_kind: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    col = *((*mip).col).offset(j as isize);
    match kind {
        1 => {
            (*col).kind = 1 as libc::c_int;
        }
        2 => {
            (*col).kind = 2 as libc::c_int;
        }
        3 => {
            (*col).kind = 2 as libc::c_int;
            if !((*col).type_0 == 4 as libc::c_int && (*col).lb == 0.0f64
                && (*col).ub == 1.0f64)
            {
                glp_set_col_bnds(mip, j, 4 as libc::c_int, 0.0f64, 1.0f64);
            }
        }
        _ => {
            (glp_error_(
                b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
            ))
                .unwrap()(
                b"glp_set_col_kind: j = %d; kind = %d; invalid column kind\n\0"
                    as *const u8 as *const libc::c_char,
                j,
                kind,
            );
        }
    };
}
pub unsafe extern "C" fn glp_get_col_kind(
    mut mip: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_int {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut kind: libc::c_int = 0;
    if !(1 as libc::c_int <= j && j <= (*mip).n) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_kind: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    col = *((*mip).col).offset(j as isize);
    kind = (*col).kind;
    match kind {
        1 => {}
        2 => {
            if (*col).type_0 == 4 as libc::c_int && (*col).lb == 0.0f64
                && (*col).ub == 1.0f64
            {
                kind = 3 as libc::c_int;
            }
        }
        _ => {
            (kind != kind
                || {
                    glp_assert_(
                        b"kind != kind\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
                        104 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return kind;
}
pub unsafe extern "C" fn glp_get_num_int(mut mip: *mut glp_prob) -> libc::c_int {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*mip).n {
        col = *((*mip).col).offset(j as isize);
        if (*col).kind == 2 as libc::c_int {
            count += 1;
            count;
        }
        j += 1;
        j;
    }
    return count;
}
pub unsafe extern "C" fn glp_get_num_bin(mut mip: *mut glp_prob) -> libc::c_int {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= (*mip).n {
        col = *((*mip).col).offset(j as isize);
        if (*col).kind == 2 as libc::c_int && (*col).type_0 == 4 as libc::c_int
            && (*col).lb == 0.0f64 && (*col).ub == 1.0f64
        {
            count += 1;
            count;
        }
        j += 1;
        j;
    }
    return count;
}
unsafe extern "C" fn solve_mip(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
    mut P0: *mut glp_prob,
    mut npp: *mut NPP,
) -> libc::c_int {
    let mut T: *mut glp_tree = 0 as *mut glp_tree;
    let mut ret: libc::c_int = 0;
    if glp_get_status(P) != 5 as libc::c_int {
        if (*parm).msg_lev >= 1 as libc::c_int {
            glp_printf(
                b"glp_intopt: optimal basis to initial LP relaxation not provided\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        ret = 0xc as libc::c_int;
    } else {
        if (*parm).msg_lev >= 3 as libc::c_int {
            glp_printf(
                b"Integer optimization begins...\n\0" as *const u8 as *const libc::c_char,
            );
        }
        T = _glp_ios_create_tree(P, parm);
        (*T).P = P0 as *mut libc::c_void;
        (*T).npp = npp as *mut libc::c_void;
        ret = _glp_ios_driver(T);
        _glp_ios_delete_tree(T);
        if ret == 0 as libc::c_int {
            if (*P).mip_stat == 2 as libc::c_int {
                if (*parm).msg_lev >= 3 as libc::c_int {
                    glp_printf(
                        b"INTEGER OPTIMAL SOLUTION FOUND\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*P).mip_stat = 5 as libc::c_int;
            } else {
                if (*parm).msg_lev >= 3 as libc::c_int {
                    glp_printf(
                        b"PROBLEM HAS NO INTEGER FEASIBLE SOLUTION\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*P).mip_stat = 4 as libc::c_int;
            }
        } else if ret == 0xe as libc::c_int {
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"RELATIVE MIP GAP TOLERANCE REACHED; SEARCH TERMINATED\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else if ret == 0x9 as libc::c_int {
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"TIME LIMIT EXCEEDED; SEARCH TERMINATED\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if ret == 0x5 as libc::c_int {
            if (*parm).msg_lev >= 1 as libc::c_int {
                glp_printf(
                    b"glp_intopt: cannot solve current LP relaxation\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if ret == 0xd as libc::c_int {
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"SEARCH TERMINATED BY APPLICATION\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
                        281 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return ret;
}
unsafe extern "C" fn preprocess_and_solve_mip(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
) -> libc::c_int {
    let mut current_block: u64;
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut term_out: libc::c_int = (*env).term_out;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut mip: *mut glp_prob = 0 as *mut glp_prob;
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
    let mut smcp: glp_smcp = glp_smcp {
        msg_lev: 0,
        meth: 0,
        pricing: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_dj: 0.,
        tol_piv: 0.,
        obj_ll: 0.,
        obj_ul: 0.,
        it_lim: 0,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        presolve: 0,
        excl: 0,
        shift: 0,
        aorn: 0,
        foo_bar: [0.; 33],
    };
    let mut ret: libc::c_int = 0;
    if (*parm).msg_lev >= 3 as libc::c_int {
        glp_printf(b"Preprocessing...\n\0" as *const u8 as *const libc::c_char);
    }
    npp = _glp_npp_create_wksp();
    _glp_npp_load_prob(npp, P, 0 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int);
    if term_out == 0 || (*parm).msg_lev < 3 as libc::c_int {
        (*env).term_out = 0 as libc::c_int;
    } else {
        (*env).term_out = 1 as libc::c_int;
    }
    ret = _glp_npp_integer(npp, parm);
    (*env).term_out = term_out;
    if !(ret == 0 as libc::c_int) {
        if ret == 0xa as libc::c_int {
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"PROBLEM HAS NO PRIMAL FEASIBLE SOLUTION\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if ret == 0xb as libc::c_int {
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"LP RELAXATION HAS NO DUAL FEASIBLE SOLUTION\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
                        318 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if !(ret != 0 as libc::c_int) {
        mip = glp_create_prob();
        _glp_npp_build_prob(npp, mip);
        if (*mip).m == 0 as libc::c_int && (*mip).n == 0 as libc::c_int {
            (*mip).mip_stat = 5 as libc::c_int;
            (*mip).mip_obj = (*mip).c0;
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"Objective value = %17.9e\n\0" as *const u8 as *const libc::c_char,
                    (*mip).mip_obj,
                );
                glp_printf(
                    b"INTEGER OPTIMAL SOLUTION FOUND BY MIP PREPROCESSOR\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            current_block = 15602298108735265016;
        } else {
            if (*parm).msg_lev >= 3 as libc::c_int {
                let mut ni: libc::c_int = glp_get_num_int(mip);
                let mut nb: libc::c_int = glp_get_num_bin(mip);
                let mut s: [libc::c_char; 50] = [0; 50];
                glp_printf(
                    b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*mip).m,
                    if (*mip).m == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    (*mip).n,
                    if (*mip).n == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    (*mip).nnz,
                    if (*mip).nnz == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                );
                if nb == 0 as libc::c_int {
                    strcpy(
                        s.as_mut_ptr(),
                        b"none of\0" as *const u8 as *const libc::c_char,
                    );
                } else if ni == 1 as libc::c_int && nb == 1 as libc::c_int {
                    strcpy(s.as_mut_ptr(), b"\0" as *const u8 as *const libc::c_char);
                } else if nb == 1 as libc::c_int {
                    strcpy(
                        s.as_mut_ptr(),
                        b"one of\0" as *const u8 as *const libc::c_char,
                    );
                } else if nb == ni {
                    strcpy(
                        s.as_mut_ptr(),
                        b"all of\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    sprintf(
                        s.as_mut_ptr(),
                        b"%d of\0" as *const u8 as *const libc::c_char,
                        nb,
                    );
                }
                glp_printf(
                    b"%d integer variable%s, %s which %s binary\n\0" as *const u8
                        as *const libc::c_char,
                    ni,
                    if ni == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    s.as_mut_ptr(),
                    if nb == 1 as libc::c_int {
                        b"is\0" as *const u8 as *const libc::c_char
                    } else {
                        b"are\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            glp_get_bfcp(P, &mut bfcp);
            glp_set_bfcp(mip, &mut bfcp);
            if term_out == 0 || (*parm).msg_lev < 3 as libc::c_int {
                (*env).term_out = 0 as libc::c_int;
            } else {
                (*env).term_out = 1 as libc::c_int;
            }
            glp_scale_prob(
                mip,
                0x1 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                    | 0x40 as libc::c_int,
            );
            (*env).term_out = term_out;
            if term_out == 0 || (*parm).msg_lev < 3 as libc::c_int {
                (*env).term_out = 0 as libc::c_int;
            } else {
                (*env).term_out = 1 as libc::c_int;
            }
            glp_adv_basis(mip, 0 as libc::c_int);
            (*env).term_out = term_out;
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"Solving LP relaxation...\n\0" as *const u8 as *const libc::c_char,
                );
            }
            glp_init_smcp(&mut smcp);
            smcp.msg_lev = (*parm).msg_lev;
            smcp.tm_lim = (*parm).tm_lim;
            (*mip).it_cnt = (*P).it_cnt;
            ret = glp_simplex(mip, &mut smcp);
            (*P).it_cnt = (*mip).it_cnt;
            if ret == 0x9 as libc::c_int {
                current_block = 11429376971389363909;
            } else if ret != 0 as libc::c_int {
                if (*parm).msg_lev >= 1 as libc::c_int {
                    glp_printf(
                        b"glp_intopt: cannot solve LP relaxation\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                ret = 0x5 as libc::c_int;
                current_block = 11429376971389363909;
            } else {
                ret = glp_get_status(mip);
                if ret == 5 as libc::c_int {
                    ret = 0 as libc::c_int;
                } else if ret == 4 as libc::c_int {
                    ret = 0xa as libc::c_int;
                } else if ret == 6 as libc::c_int {
                    ret = 0xb as libc::c_int;
                } else {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const libc::c_char,
                                b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
                                401 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
                if ret != 0 as libc::c_int {
                    current_block = 11429376971389363909;
                } else {
                    (*mip).it_cnt = (*P).it_cnt;
                    if (*parm).use_sol != 0 {
                        (*mip).mip_stat = (*P).mip_stat;
                        (*mip).mip_obj = (*P).mip_obj;
                    }
                    ret = solve_mip(mip, parm, P, npp);
                    (*P).it_cnt = (*mip).it_cnt;
                    if !((*mip).mip_stat == 5 as libc::c_int
                        || (*mip).mip_stat == 2 as libc::c_int)
                    {
                        (*P).mip_stat = (*mip).mip_stat;
                        current_block = 11429376971389363909;
                    } else {
                        current_block = 15602298108735265016;
                    }
                }
            }
        }
        match current_block {
            11429376971389363909 => {}
            _ => {
                _glp_npp_postprocess(npp, mip);
                glp_delete_prob(mip);
                mip = 0 as *mut glp_prob;
                _glp_npp_unload_sol(npp, P);
            }
        }
    }
    if !mip.is_null() {
        glp_delete_prob(mip);
    }
    _glp_npp_delete_wksp(npp);
    return ret;
}
pub unsafe extern "C" fn _glp_intopt1(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
) -> libc::c_int {
    (P == P
        || {
            glp_assert_(
                b"P == P\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
                435 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
                436 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_printf(
        b"glp_intopt: no alien solver is available\n\0" as *const u8
            as *const libc::c_char,
    );
    return 0x5 as libc::c_int;
}
pub unsafe extern "C" fn glp_intopt(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
) -> libc::c_int {
    let mut current_block: u64;
    let mut _parm: glp_iocp = glp_iocp {
        msg_lev: 0,
        br_tech: 0,
        bt_tech: 0,
        tol_int: 0.,
        tol_obj: 0.,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        cb_func: None,
        cb_info: 0 as *mut libc::c_void,
        cb_size: 0,
        pp_tech: 0,
        mip_gap: 0.,
        mir_cuts: 0,
        gmi_cuts: 0,
        cov_cuts: 0,
        clq_cuts: 0,
        presolve: 0,
        binarize: 0,
        fp_heur: 0,
        ps_heur: 0,
        ps_tm_lim: 0,
        sr_heur: 0,
        use_sol: 0,
        save_sol: 0 as *const libc::c_char,
        alien: 0,
        flip: 0,
        foo_bar: [0.; 23],
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if !((*P).tree).is_null() {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            452 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: operation not allowed\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if parm.is_null() {
        parm = &mut _parm;
        glp_init_iocp(parm as *mut glp_iocp);
    }
    if !((*parm).msg_lev == 0 as libc::c_int || (*parm).msg_lev == 1 as libc::c_int
        || (*parm).msg_lev == 2 as libc::c_int || (*parm).msg_lev == 3 as libc::c_int
        || (*parm).msg_lev == 4 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: msg_lev = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).msg_lev,
        );
    }
    if !((*parm).br_tech == 1 as libc::c_int || (*parm).br_tech == 2 as libc::c_int
        || (*parm).br_tech == 3 as libc::c_int || (*parm).br_tech == 4 as libc::c_int
        || (*parm).br_tech == 5 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            468 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: br_tech = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).br_tech,
        );
    }
    if !((*parm).bt_tech == 1 as libc::c_int || (*parm).bt_tech == 2 as libc::c_int
        || (*parm).bt_tech == 3 as libc::c_int || (*parm).bt_tech == 4 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            474 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: bt_tech = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).bt_tech,
        );
    }
    if !(0.0f64 < (*parm).tol_int && (*parm).tol_int < 1.0f64) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            477 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: tol_int = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tol_int,
        );
    }
    if !(0.0f64 < (*parm).tol_obj && (*parm).tol_obj < 1.0f64) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            480 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: tol_obj = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tol_obj,
        );
    }
    if (*parm).tm_lim < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: tm_lim = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tm_lim,
        );
    }
    if (*parm).out_frq < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: out_frq = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).out_frq,
        );
    }
    if (*parm).out_dly < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: out_dly = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).out_dly,
        );
    }
    if !(0 as libc::c_int <= (*parm).cb_size && (*parm).cb_size <= 256 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            492 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: cb_size = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).cb_size,
        );
    }
    if !((*parm).pp_tech == 0 as libc::c_int || (*parm).pp_tech == 1 as libc::c_int
        || (*parm).pp_tech == 2 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: pp_tech = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).pp_tech,
        );
    }
    if (*parm).mip_gap < 0.0f64 {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            500 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: mip_gap = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).mip_gap,
        );
    }
    if !((*parm).mir_cuts == 1 as libc::c_int || (*parm).mir_cuts == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: mir_cuts = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).mir_cuts,
        );
    }
    if !((*parm).gmi_cuts == 1 as libc::c_int || (*parm).gmi_cuts == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            506 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: gmi_cuts = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).gmi_cuts,
        );
    }
    if !((*parm).cov_cuts == 1 as libc::c_int || (*parm).cov_cuts == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: cov_cuts = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).cov_cuts,
        );
    }
    if !((*parm).clq_cuts == 1 as libc::c_int || (*parm).clq_cuts == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            512 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: clq_cuts = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).clq_cuts,
        );
    }
    if !((*parm).presolve == 1 as libc::c_int || (*parm).presolve == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            515 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: presolve = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).presolve,
        );
    }
    if !((*parm).binarize == 1 as libc::c_int || (*parm).binarize == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            518 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: binarize = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).binarize,
        );
    }
    if !((*parm).fp_heur == 1 as libc::c_int || (*parm).fp_heur == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            521 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: fp_heur = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).fp_heur,
        );
    }
    if !((*parm).alien == 1 as libc::c_int || (*parm).alien == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            525 as libc::c_int,
        ))
            .unwrap()(
            b"glp_intopt: alien = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).alien,
        );
    }
    if (*parm).use_sol == 0 {
        (*P).mip_stat = 1 as libc::c_int;
    }
    if (*P).mip_stat == 4 as libc::c_int {
        (*P).mip_stat = 1 as libc::c_int;
    }
    if (*P).mip_stat == 1 as libc::c_int {
        (*P).mip_obj = 0.0f64;
    } else if (*P).mip_stat == 5 as libc::c_int {
        (*P).mip_stat = 2 as libc::c_int;
    }
    i = 1 as libc::c_int;
    loop {
        if !(i <= (*P).m) {
            current_block = 10150597327160359210;
            break;
        }
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if (*row).type_0 == 4 as libc::c_int && (*row).lb >= (*row).ub {
            if (*parm).msg_lev >= 1 as libc::c_int {
                glp_printf(
                    b"glp_intopt: row %d: lb = %g, ub = %g; incorrect bounds\n\0"
                        as *const u8 as *const libc::c_char,
                    i,
                    (*row).lb,
                    (*row).ub,
                );
            }
            ret = 0x4 as libc::c_int;
            current_block = 5310105525648602799;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        10150597327160359210 => {
            j = 1 as libc::c_int;
            loop {
                if !(j <= (*P).n) {
                    current_block = 7018308795614528254;
                    break;
                }
                let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
                if (*col).type_0 == 4 as libc::c_int && (*col).lb >= (*col).ub {
                    if (*parm).msg_lev >= 1 as libc::c_int {
                        glp_printf(
                            b"glp_intopt: column %d: lb = %g, ub = %g; incorrect bounds\n\0"
                                as *const u8 as *const libc::c_char,
                            j,
                            (*col).lb,
                            (*col).ub,
                        );
                    }
                    ret = 0x4 as libc::c_int;
                    current_block = 5310105525648602799;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            match current_block {
                5310105525648602799 => {}
                _ => {
                    j = 1 as libc::c_int;
                    loop {
                        if !(j <= (*P).n) {
                            current_block = 7158658067966855297;
                            break;
                        }
                        let mut col_0: *mut GLPCOL = *((*P).col).offset(j as isize);
                        if !((*col_0).kind != 2 as libc::c_int) {
                            if (*col_0).type_0 == 2 as libc::c_int
                                || (*col_0).type_0 == 4 as libc::c_int
                            {
                                if (*col_0).lb != floor((*col_0).lb) {
                                    if (*parm).msg_lev >= 1 as libc::c_int {
                                        glp_printf(
                                            b"glp_intopt: integer column %d has non-integer lower bound %g\n\0"
                                                as *const u8 as *const libc::c_char,
                                            j,
                                            (*col_0).lb,
                                        );
                                    }
                                    ret = 0x4 as libc::c_int;
                                    current_block = 5310105525648602799;
                                    break;
                                }
                            }
                            if (*col_0).type_0 == 3 as libc::c_int
                                || (*col_0).type_0 == 4 as libc::c_int
                            {
                                if (*col_0).ub != floor((*col_0).ub) {
                                    if (*parm).msg_lev >= 1 as libc::c_int {
                                        glp_printf(
                                            b"glp_intopt: integer column %d has non-integer upper bound %g\n\0"
                                                as *const u8 as *const libc::c_char,
                                            j,
                                            (*col_0).ub,
                                        );
                                    }
                                    ret = 0x4 as libc::c_int;
                                    current_block = 5310105525648602799;
                                    break;
                                }
                            }
                            if (*col_0).type_0 == 5 as libc::c_int {
                                if (*col_0).lb != floor((*col_0).lb) {
                                    if (*parm).msg_lev >= 1 as libc::c_int {
                                        glp_printf(
                                            b"glp_intopt: integer column %d has non-integer fixed value %g\n\0"
                                                as *const u8 as *const libc::c_char,
                                            j,
                                            (*col_0).lb,
                                        );
                                    }
                                    ret = 0x4 as libc::c_int;
                                    current_block = 5310105525648602799;
                                    break;
                                }
                            }
                        }
                        j += 1;
                        j;
                    }
                    match current_block {
                        5310105525648602799 => {}
                        _ => {
                            if (*parm).msg_lev >= 3 as libc::c_int {
                                let mut ni: libc::c_int = glp_get_num_int(P);
                                let mut nb: libc::c_int = glp_get_num_bin(P);
                                let mut s: [libc::c_char; 50] = [0; 50];
                                glp_printf(
                                    b"GLPK Integer Optimizer %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    glp_version(),
                                );
                                glp_printf(
                                    b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*P).m,
                                    if (*P).m == 1 as libc::c_int {
                                        b"\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"s\0" as *const u8 as *const libc::c_char
                                    },
                                    (*P).n,
                                    if (*P).n == 1 as libc::c_int {
                                        b"\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"s\0" as *const u8 as *const libc::c_char
                                    },
                                    (*P).nnz,
                                    if (*P).nnz == 1 as libc::c_int {
                                        b"\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"s\0" as *const u8 as *const libc::c_char
                                    },
                                );
                                if nb == 0 as libc::c_int {
                                    strcpy(
                                        s.as_mut_ptr(),
                                        b"none of\0" as *const u8 as *const libc::c_char,
                                    );
                                } else if ni == 1 as libc::c_int && nb == 1 as libc::c_int {
                                    strcpy(
                                        s.as_mut_ptr(),
                                        b"\0" as *const u8 as *const libc::c_char,
                                    );
                                } else if nb == 1 as libc::c_int {
                                    strcpy(
                                        s.as_mut_ptr(),
                                        b"one of\0" as *const u8 as *const libc::c_char,
                                    );
                                } else if nb == ni {
                                    strcpy(
                                        s.as_mut_ptr(),
                                        b"all of\0" as *const u8 as *const libc::c_char,
                                    );
                                } else {
                                    sprintf(
                                        s.as_mut_ptr(),
                                        b"%d of\0" as *const u8 as *const libc::c_char,
                                        nb,
                                    );
                                }
                                glp_printf(
                                    b"%d integer variable%s, %s which %s binary\n\0"
                                        as *const u8 as *const libc::c_char,
                                    ni,
                                    if ni == 1 as libc::c_int {
                                        b"\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"s\0" as *const u8 as *const libc::c_char
                                    },
                                    s.as_mut_ptr(),
                                    if nb == 1 as libc::c_int {
                                        b"is\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"are\0" as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            if (*parm).alien != 0 {
                                ret = _glp_intopt1(P, parm);
                            } else {
                                if (*parm).presolve == 0 {
                                    ret = solve_mip(P, parm, P, 0 as *mut NPP);
                                } else {
                                    ret = preprocess_and_solve_mip(P, parm);
                                }
                                if ret == 0xa as libc::c_int {
                                    (*P).mip_stat = 4 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
pub unsafe extern "C" fn glp_init_iocp(mut parm: *mut glp_iocp) {
    (*parm).msg_lev = 3 as libc::c_int;
    (*parm).br_tech = 4 as libc::c_int;
    (*parm).bt_tech = 3 as libc::c_int;
    (*parm).tol_int = 1e-5f64;
    (*parm).tol_obj = 1e-7f64;
    (*parm).tm_lim = 2147483647 as libc::c_int;
    (*parm).out_frq = 5000 as libc::c_int;
    (*parm).out_dly = 10000 as libc::c_int;
    (*parm).cb_func = None;
    (*parm).cb_info = 0 as *mut libc::c_void;
    (*parm).cb_size = 0 as libc::c_int;
    (*parm).pp_tech = 2 as libc::c_int;
    (*parm).mip_gap = 0.0f64;
    (*parm).mir_cuts = 0 as libc::c_int;
    (*parm).gmi_cuts = 0 as libc::c_int;
    (*parm).cov_cuts = 0 as libc::c_int;
    (*parm).clq_cuts = 0 as libc::c_int;
    (*parm).presolve = 0 as libc::c_int;
    (*parm).binarize = 0 as libc::c_int;
    (*parm).fp_heur = 0 as libc::c_int;
    (*parm).ps_heur = 0 as libc::c_int;
    (*parm).ps_tm_lim = 60000 as libc::c_int;
    (*parm).sr_heur = 1 as libc::c_int;
    (*parm).use_sol = 0 as libc::c_int;
    (*parm).save_sol = 0 as *const libc::c_char;
    (*parm).alien = 0 as libc::c_int;
    (*parm).flip = 1 as libc::c_int;
}
pub unsafe extern "C" fn glp_mip_status(mut mip: *mut glp_prob) -> libc::c_int {
    let mut mip_stat: libc::c_int = (*mip).mip_stat;
    return mip_stat;
}
pub unsafe extern "C" fn glp_mip_obj_val(mut mip: *mut glp_prob) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    z = (*mip).mip_obj;
    return z;
}
pub unsafe extern "C" fn glp_mip_row_val(
    mut mip: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut mipx: libc::c_double = 0.;
    if !(1 as libc::c_int <= i && i <= (*mip).m) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            763 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mip_row_val: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    mipx = (**((*mip).row).offset(i as isize)).mipx;
    return mipx;
}
pub unsafe extern "C" fn glp_mip_col_val(
    mut mip: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut mipx: libc::c_double = 0.;
    if !(1 as libc::c_int <= j && j <= (*mip).n) {
        (glp_error_(
            b"draft/glpapi09.c\0" as *const u8 as *const libc::c_char,
            788 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mip_col_val: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    mipx = (**((*mip).col).offset(j as isize)).mipx;
    return mipx;
}
