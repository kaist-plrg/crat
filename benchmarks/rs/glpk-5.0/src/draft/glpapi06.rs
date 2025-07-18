use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_scale_prob(P: *mut glp_prob, flags: libc::c_int);
    fn glp_adv_basis(P: *mut glp_prob, flags: libc::c_int);
    fn glp_bf_exists(P: *mut glp_prob) -> libc::c_int;
    fn glp_factorize(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn glp_version() -> *const libc::c_char;
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
    fn _glp_npp_simplex(npp: *mut NPP, parm: *const glp_smcp) -> libc::c_int;
    fn _glp_spx_primal(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn _glp_spy_dual(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
unsafe extern "C" fn trivial_lp(mut P: *mut glp_prob, mut parm: *const glp_smcp) {
    let mut current_block: u64;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p_infeas: libc::c_double = 0.;
    let mut d_infeas: libc::c_double = 0.;
    let mut zeta: libc::c_double = 0.;
    (*P).valid = 0 as libc::c_int;
    (*P).dbs_stat = 2 as libc::c_int;
    (*P).pbs_stat = (*P).dbs_stat;
    (*P).obj_val = (*P).c0;
    (*P).some = 0 as libc::c_int;
    d_infeas = 0.0f64;
    p_infeas = d_infeas;
    i = 1 as libc::c_int;
    while i <= (*P).m {
        row = *((*P).row).offset(i as isize);
        (*row).stat = 1 as libc::c_int;
        (*row).dual = 0.0f64;
        (*row).prim = (*row).dual;
        if (*row).type_0 == 2 as libc::c_int || (*row).type_0 == 4 as libc::c_int
            || (*row).type_0 == 5 as libc::c_int
        {
            if (*row).lb > (*parm).tol_bnd {
                (*P).pbs_stat = 4 as libc::c_int;
                if (*P).some == 0 as libc::c_int && (*parm).meth != 1 as libc::c_int {
                    (*P).some = i;
                }
            }
            if p_infeas < (*row).lb {
                p_infeas = (*row).lb;
            }
        }
        if (*row).type_0 == 3 as libc::c_int || (*row).type_0 == 4 as libc::c_int
            || (*row).type_0 == 5 as libc::c_int
        {
            if (*row).ub < -(*parm).tol_bnd {
                (*P).pbs_stat = 4 as libc::c_int;
                if (*P).some == 0 as libc::c_int && (*parm).meth != 1 as libc::c_int {
                    (*P).some = i;
                }
            }
            if p_infeas < -(*row).ub {
                p_infeas = -(*row).ub;
            }
        }
        i += 1;
        i;
    }
    zeta = 1.0f64;
    j = 1 as libc::c_int;
    while j <= (*P).n {
        col = *((*P).col).offset(j as isize);
        if zeta < fabs((*col).coef) {
            zeta = fabs((*col).coef);
        }
        j += 1;
        j;
    }
    zeta = (if (*P).dir == 1 as libc::c_int { 1.0f64 } else { -1.0f64 }) / zeta;
    j = 1 as libc::c_int;
    while j <= (*P).n {
        col = *((*P).col).offset(j as isize);
        if (*col).type_0 == 1 as libc::c_int {
            (*col).stat = 4 as libc::c_int;
            (*col).prim = 0.0f64;
        } else {
            if (*col).type_0 == 2 as libc::c_int {
                current_block = 11667247526335102347;
            } else {
                if (*col).type_0 == 3 as libc::c_int {
                    current_block = 6348067032293078883;
                } else if (*col).type_0 == 4 as libc::c_int {
                    if zeta * (*col).coef > 0.0f64 {
                        current_block = 11667247526335102347;
                    } else if zeta * (*col).coef < 0.0f64 {
                        current_block = 6348067032293078883;
                    } else if fabs((*col).lb) <= fabs((*col).ub) {
                        current_block = 11667247526335102347;
                    } else {
                        current_block = 6348067032293078883;
                    }
                } else {
                    if (*col).type_0 == 5 as libc::c_int {
                        (*col).stat = 5 as libc::c_int;
                        (*col).prim = (*col).lb;
                    }
                    current_block = 16799951812150840583;
                }
                match current_block {
                    16799951812150840583 => {}
                    11667247526335102347 => {}
                    _ => {
                        (*col).stat = 3 as libc::c_int;
                        (*col).prim = (*col).ub;
                        current_block = 16799951812150840583;
                    }
                }
            }
            match current_block {
                16799951812150840583 => {}
                _ => {
                    (*col).stat = 2 as libc::c_int;
                    (*col).prim = (*col).lb;
                }
            }
        }
        (*col).dual = (*col).coef;
        (*P).obj_val += (*col).coef * (*col).prim;
        if (*col).type_0 == 1 as libc::c_int || (*col).type_0 == 2 as libc::c_int {
            if zeta * (*col).dual < -(*parm).tol_dj {
                (*P).dbs_stat = 4 as libc::c_int;
                if (*P).some == 0 as libc::c_int && (*parm).meth == 1 as libc::c_int {
                    (*P).some = (*P).m + j;
                }
            }
            if d_infeas < -zeta * (*col).dual {
                d_infeas = -zeta * (*col).dual;
            }
        }
        if (*col).type_0 == 1 as libc::c_int || (*col).type_0 == 3 as libc::c_int {
            if zeta * (*col).dual > (*parm).tol_dj {
                (*P).dbs_stat = 4 as libc::c_int;
                if (*P).some == 0 as libc::c_int && (*parm).meth == 1 as libc::c_int {
                    (*P).some = (*P).m + j;
                }
            }
            if d_infeas < zeta * (*col).dual {
                d_infeas = zeta * (*col).dual;
            }
        }
        j += 1;
        j;
    }
    if (*parm).msg_lev >= 2 as libc::c_int && (*parm).out_dly == 0 as libc::c_int {
        glp_printf(
            b"~%6d: obj = %17.9e  infeas = %10.3e\n\0" as *const u8
                as *const libc::c_char,
            (*P).it_cnt,
            (*P).obj_val,
            if (*parm).meth == 1 as libc::c_int { p_infeas } else { d_infeas },
        );
    }
    if (*parm).msg_lev >= 3 as libc::c_int && (*parm).out_dly == 0 as libc::c_int {
        if (*P).pbs_stat == 2 as libc::c_int && (*P).dbs_stat == 2 as libc::c_int {
            glp_printf(
                b"OPTIMAL SOLUTION FOUND\n\0" as *const u8 as *const libc::c_char,
            );
        } else if (*P).pbs_stat == 4 as libc::c_int {
            glp_printf(
                b"PROBLEM HAS NO FEASIBLE SOLUTION\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if (*parm).meth == 1 as libc::c_int {
            glp_printf(
                b"PROBLEM HAS UNBOUNDED SOLUTION\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            glp_printf(
                b"PROBLEM HAS NO DUAL FEASIBLE SOLUTION\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
}
unsafe extern "C" fn solve_lp(
    mut P: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    if glp_bf_exists(P) == 0 {
        ret = glp_factorize(P);
        if !(ret == 0 as libc::c_int) {
            if ret == 0x1 as libc::c_int {
                if (*parm).msg_lev >= 1 as libc::c_int {
                    glp_printf(
                        b"glp_simplex: initial basis is invalid\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if ret == 0x2 as libc::c_int {
                if (*parm).msg_lev >= 1 as libc::c_int {
                    glp_printf(
                        b"glp_simplex: initial basis is singular\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if ret == 0x3 as libc::c_int {
                if (*parm).msg_lev >= 1 as libc::c_int {
                    glp_printf(
                        b"glp_simplex: initial basis is ill-conditioned\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
                            241 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        if ret != 0 as libc::c_int {
            current_block = 9329740645624525771;
        } else {
            current_block = 12039483399334584727;
        }
    } else {
        current_block = 12039483399334584727;
    }
    match current_block {
        12039483399334584727 => {
            if (*parm).meth == 1 as libc::c_int {
                ret = _glp_spx_primal(P, parm);
            } else if (*parm).meth == 2 as libc::c_int {
                ret = _glp_spy_dual(P, parm);
                if ret == 0x5 as libc::c_int && (*P).valid != 0 {
                    ret = _glp_spx_primal(P, parm);
                }
            } else if (*parm).meth == 3 as libc::c_int {
                ret = _glp_spy_dual(P, parm);
            } else {
                (parm != parm
                    || {
                        glp_assert_(
                            b"parm != parm\0" as *const u8 as *const libc::c_char,
                            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
                            254 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn preprocess_and_solve_lp(
    mut P: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> libc::c_int {
    let mut current_block: u64;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
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
    let mut ret: libc::c_int = 0;
    if (*parm).msg_lev >= 3 as libc::c_int {
        glp_printf(b"Preprocessing...\n\0" as *const u8 as *const libc::c_char);
    }
    npp = _glp_npp_create_wksp();
    _glp_npp_load_prob(npp, P, 0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
    ret = _glp_npp_simplex(npp, parm);
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
                    b"PROBLEM HAS NO DUAL FEASIBLE SOLUTION\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
                        283 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if !(ret != 0 as libc::c_int) {
        lp = glp_create_prob();
        _glp_npp_build_prob(npp, lp);
        if (*lp).m == 0 as libc::c_int && (*lp).n == 0 as libc::c_int {
            (*lp).dbs_stat = 2 as libc::c_int;
            (*lp).pbs_stat = (*lp).dbs_stat;
            (*lp).obj_val = (*lp).c0;
            if (*parm).msg_lev >= 2 as libc::c_int && (*parm).out_dly == 0 as libc::c_int
            {
                glp_printf(
                    b"~%6d: obj = %17.9e  infeas = %10.3e\n\0" as *const u8
                        as *const libc::c_char,
                    (*P).it_cnt,
                    (*lp).obj_val,
                    0.0f64,
                );
            }
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"OPTIMAL SOLUTION FOUND BY LP PREPROCESSOR\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            current_block = 5965284011230510978;
        } else {
            if (*parm).msg_lev >= 3 as libc::c_int {
                glp_printf(
                    b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                        as *const libc::c_char,
                    (*lp).m,
                    if (*lp).m == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    (*lp).n,
                    if (*lp).n == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    (*lp).nnz,
                    if (*lp).nnz == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            glp_get_bfcp(P, &mut bfcp);
            glp_set_bfcp(lp, &mut bfcp);
            let mut env: *mut ENV = _glp_get_env_ptr();
            let mut term_out: libc::c_int = (*env).term_out;
            if term_out == 0 || (*parm).msg_lev < 3 as libc::c_int {
                (*env).term_out = 0 as libc::c_int;
            } else {
                (*env).term_out = 1 as libc::c_int;
            }
            glp_scale_prob(lp, 0x80 as libc::c_int);
            (*env).term_out = term_out;
            let mut env_0: *mut ENV = _glp_get_env_ptr();
            let mut term_out_0: libc::c_int = (*env_0).term_out;
            if term_out_0 == 0 || (*parm).msg_lev < 3 as libc::c_int {
                (*env_0).term_out = 0 as libc::c_int;
            } else {
                (*env_0).term_out = 1 as libc::c_int;
            }
            glp_adv_basis(lp, 0 as libc::c_int);
            (*env_0).term_out = term_out_0;
            (*lp).it_cnt = (*P).it_cnt;
            ret = solve_lp(lp, parm);
            (*P).it_cnt = (*lp).it_cnt;
            if !(ret == 0 as libc::c_int && (*lp).pbs_stat == 2 as libc::c_int
                && (*lp).dbs_stat == 2 as libc::c_int)
            {
                if (*parm).msg_lev >= 1 as libc::c_int {
                    glp_printf(
                        b"glp_simplex: unable to recover undefined or non-optimal solution\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                if ret == 0 as libc::c_int {
                    if (*lp).pbs_stat == 4 as libc::c_int {
                        ret = 0xa as libc::c_int;
                    } else if (*lp).dbs_stat == 4 as libc::c_int {
                        ret = 0xb as libc::c_int;
                    } else {
                        (lp != lp
                            || {
                                glp_assert_(
                                    b"lp != lp\0" as *const u8 as *const libc::c_char,
                                    b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
                                    345 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                current_block = 5304746711381260782;
            } else {
                current_block = 5965284011230510978;
            }
        }
        match current_block {
            5304746711381260782 => {}
            _ => {
                _glp_npp_postprocess(npp, lp);
                glp_delete_prob(lp);
                lp = 0 as *mut glp_prob;
                _glp_npp_unload_sol(npp, P);
                ret = 0 as libc::c_int;
            }
        }
    }
    if !lp.is_null() {
        glp_delete_prob(lp);
    }
    _glp_npp_delete_wksp(npp);
    return ret;
}
pub unsafe extern "C" fn glp_simplex(
    mut P: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> libc::c_int {
    let mut current_block: u64;
    let mut _parm: glp_smcp = glp_smcp {
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
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if !((*P).tree).is_null() && (*(*P).tree).reason != 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            374 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: operation not allowed\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if parm.is_null() {
        parm = &mut _parm;
        glp_init_smcp(parm as *mut glp_smcp);
    }
    if !((*parm).msg_lev == 0 as libc::c_int || (*parm).msg_lev == 1 as libc::c_int
        || (*parm).msg_lev == 2 as libc::c_int || (*parm).msg_lev == 3 as libc::c_int
        || (*parm).msg_lev == 4 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            383 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: msg_lev = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).msg_lev,
        );
    }
    if !((*parm).meth == 1 as libc::c_int || (*parm).meth == 2 as libc::c_int
        || (*parm).meth == 3 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: meth = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).meth,
        );
    }
    if !((*parm).pricing == 0x11 as libc::c_int
        || (*parm).pricing == 0x22 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            392 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: pricing = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).pricing,
        );
    }
    if !((*parm).r_test == 0x11 as libc::c_int || (*parm).r_test == 0x33 as libc::c_int
        || (*parm).r_test == 0x22 as libc::c_int)
    {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: r_test = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).r_test,
        );
    }
    if !(0.0f64 < (*parm).tol_bnd && (*parm).tol_bnd < 1.0f64) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            402 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: tol_bnd = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tol_bnd,
        );
    }
    if !(0.0f64 < (*parm).tol_dj && (*parm).tol_dj < 1.0f64) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: tol_dj = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tol_dj,
        );
    }
    if !(0.0f64 < (*parm).tol_piv && (*parm).tol_piv < 1.0f64) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: tol_piv = %g; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tol_piv,
        );
    }
    if (*parm).it_lim < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            411 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: it_lim = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).it_lim,
        );
    }
    if (*parm).tm_lim < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: tm_lim = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).tm_lim,
        );
    }
    if (*parm).out_frq < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: out_frq = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).out_frq,
        );
    }
    if (*parm).out_dly < 0 as libc::c_int {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: out_dly = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).out_dly,
        );
    }
    if !((*parm).presolve == 1 as libc::c_int || (*parm).presolve == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            427 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: presolve = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).presolve,
        );
    }
    if !((*parm).excl == 1 as libc::c_int || (*parm).excl == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            431 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: excl = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).excl,
        );
    }
    if !((*parm).shift == 1 as libc::c_int || (*parm).shift == 0 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: shift = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).shift,
        );
    }
    if !((*parm).aorn == 1 as libc::c_int || (*parm).aorn == 2 as libc::c_int) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            437 as libc::c_int,
        ))
            .unwrap()(
            b"glp_simplex: aorn = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            (*parm).aorn,
        );
    }
    (*P).dbs_stat = 1 as libc::c_int;
    (*P).pbs_stat = (*P).dbs_stat;
    (*P).obj_val = 0.0f64;
    (*P).some = 0 as libc::c_int;
    i = 1 as libc::c_int;
    loop {
        if !(i <= (*P).m) {
            current_block = 1836292691772056875;
            break;
        }
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if (*row).type_0 == 4 as libc::c_int && (*row).lb >= (*row).ub {
            if (*parm).msg_lev >= 1 as libc::c_int {
                glp_printf(
                    b"glp_simplex: row %d: lb = %g, ub = %g; incorrect bounds\n\0"
                        as *const u8 as *const libc::c_char,
                    i,
                    (*row).lb,
                    (*row).ub,
                );
            }
            ret = 0x4 as libc::c_int;
            current_block = 2604221289000121028;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        1836292691772056875 => {
            j = 1 as libc::c_int;
            loop {
                if !(j <= (*P).n) {
                    current_block = 10758786907990354186;
                    break;
                }
                let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
                if (*col).type_0 == 4 as libc::c_int && (*col).lb >= (*col).ub {
                    if (*parm).msg_lev >= 1 as libc::c_int {
                        glp_printf(
                            b"glp_simplex: column %d: lb = %g, ub = %g; incorrect bounds\n\0"
                                as *const u8 as *const libc::c_char,
                            j,
                            (*col).lb,
                            (*col).ub,
                        );
                    }
                    ret = 0x4 as libc::c_int;
                    current_block = 2604221289000121028;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            match current_block {
                2604221289000121028 => {}
                _ => {
                    if (*parm).msg_lev >= 3 as libc::c_int {
                        glp_printf(
                            b"GLPK Simplex Optimizer %s\n\0" as *const u8
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
                    }
                    if (*P).nnz == 0 as libc::c_int {
                        trivial_lp(P, parm);
                        ret = 0 as libc::c_int;
                    } else if (*parm).presolve == 0 {
                        ret = solve_lp(P, parm);
                    } else {
                        ret = preprocess_and_solve_lp(P, parm);
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
pub unsafe extern "C" fn glp_init_smcp(mut parm: *mut glp_smcp) {
    (*parm).msg_lev = 3 as libc::c_int;
    (*parm).meth = 1 as libc::c_int;
    (*parm).pricing = 0x22 as libc::c_int;
    (*parm).r_test = 0x22 as libc::c_int;
    (*parm).tol_bnd = 1e-7f64;
    (*parm).tol_dj = 1e-7f64;
    (*parm).tol_piv = 1e-9f64;
    (*parm).obj_ll = -1.7976931348623157e+308f64;
    (*parm).obj_ul = 1.7976931348623157e+308f64;
    (*parm).it_lim = 2147483647 as libc::c_int;
    (*parm).tm_lim = 2147483647 as libc::c_int;
    (*parm).out_frq = 5000 as libc::c_int;
    (*parm).out_dly = 0 as libc::c_int;
    (*parm).presolve = 0 as libc::c_int;
    (*parm).excl = 1 as libc::c_int;
    (*parm).shift = 1 as libc::c_int;
    (*parm).aorn = 2 as libc::c_int;
}
pub unsafe extern "C" fn glp_get_status(mut lp: *mut glp_prob) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = glp_get_prim_stat(lp);
    match status {
        2 => {
            match glp_get_dual_stat(lp) {
                2 => {
                    status = 5 as libc::c_int;
                }
                4 => {
                    status = 6 as libc::c_int;
                }
                1 | 3 => {
                    status = status;
                }
                _ => {
                    (lp != lp
                        || {
                            glp_assert_(
                                b"lp != lp\0" as *const u8 as *const libc::c_char,
                                b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
                                568 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
        }
        1 | 3 | 4 => {
            status = status;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const libc::c_char,
                        b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
                        577 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return status;
}
pub unsafe extern "C" fn glp_get_prim_stat(mut lp: *mut glp_prob) -> libc::c_int {
    let mut pbs_stat: libc::c_int = (*lp).pbs_stat;
    return pbs_stat;
}
pub unsafe extern "C" fn glp_get_dual_stat(mut lp: *mut glp_prob) -> libc::c_int {
    let mut dbs_stat: libc::c_int = (*lp).dbs_stat;
    return dbs_stat;
}
pub unsafe extern "C" fn glp_get_obj_val(mut lp: *mut glp_prob) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    z = (*lp).obj_val;
    return z;
}
pub unsafe extern "C" fn glp_get_row_stat(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_int {
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_stat: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).stat;
}
pub unsafe extern "C" fn glp_get_row_prim(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut prim: libc::c_double = 0.;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            697 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_prim: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    prim = (**((*lp).row).offset(i as isize)).prim;
    return prim;
}
pub unsafe extern "C" fn glp_get_row_dual(
    mut lp: *mut glp_prob,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut dual: libc::c_double = 0.;
    if !(1 as libc::c_int <= i && i <= (*lp).m) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            722 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_row_dual: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    dual = (**((*lp).row).offset(i as isize)).dual;
    return dual;
}
pub unsafe extern "C" fn glp_get_col_stat(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_int {
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            751 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_stat: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).stat;
}
pub unsafe extern "C" fn glp_get_col_prim(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut prim: libc::c_double = 0.;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            774 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_prim: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    prim = (**((*lp).col).offset(j as isize)).prim;
    return prim;
}
pub unsafe extern "C" fn glp_get_col_dual(
    mut lp: *mut glp_prob,
    mut j: libc::c_int,
) -> libc::c_double {
    let mut dual: libc::c_double = 0.;
    if !(1 as libc::c_int <= j && j <= (*lp).n) {
        (glp_error_(
            b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
            799 as libc::c_int,
        ))
            .unwrap()(
            b"glp_get_col_dual: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    dual = (**((*lp).col).offset(j as isize)).dual;
    return dual;
}
pub unsafe extern "C" fn glp_get_unbnd_ray(mut lp: *mut glp_prob) -> libc::c_int {
    let mut k: libc::c_int = 0;
    k = (*lp).some;
    (k >= 0 as libc::c_int
        || {
            glp_assert_(
                b"k >= 0\0" as *const u8 as *const libc::c_char,
                b"draft/glpapi06.c\0" as *const u8 as *const libc::c_char,
                837 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if k > (*lp).m + (*lp).n {
        k = 0 as libc::c_int;
    }
    return k;
}
pub unsafe extern "C" fn glp_get_it_cnt(mut P: *mut glp_prob) -> libc::c_int {
    return (*P).it_cnt;
}
pub unsafe extern "C" fn glp_set_it_cnt(mut P: *mut glp_prob, mut it_cnt: libc::c_int) {
    (*P).it_cnt = it_cnt;
}
