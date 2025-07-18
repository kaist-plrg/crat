use ::libc;
extern "C" {
    pub type glp_prob;
    pub type glp_tree;
    pub type glp_tran;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: libc::c_int);
    fn glp_set_row_name(P: *mut glp_prob, i: libc::c_int, name: *const libc::c_char);
    fn glp_set_col_name(P: *mut glp_prob, j: libc::c_int, name: *const libc::c_char);
    fn glp_sort_matrix(P: *mut glp_prob);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_num_rows(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_obj_coef(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_num_nz(P: *mut glp_prob) -> libc::c_int;
    fn glp_delete_index(P: *mut glp_prob);
    fn glp_scale_prob(P: *mut glp_prob, flags: libc::c_int);
    fn glp_std_basis(P: *mut glp_prob);
    fn glp_adv_basis(P: *mut glp_prob, flags: libc::c_int);
    fn glp_cpx_basis(P: *mut glp_prob);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn glp_exact(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_interior(P: *mut glp_prob, parm: *const glp_iptcp) -> libc::c_int;
    fn glp_init_iptcp(parm: *mut glp_iptcp);
    fn glp_ipt_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_int(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_bin(P: *mut glp_prob) -> libc::c_int;
    fn glp_intopt(P: *mut glp_prob, parm: *const glp_iocp) -> libc::c_int;
    fn glp_init_iocp(parm: *mut glp_iocp);
    fn glp_mip_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_print_sol(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_read_sol(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_write_sol(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_print_ranges(
        P: *mut glp_prob,
        len: libc::c_int,
        list: *const libc::c_int,
        flags: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_print_ipt(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_read_ipt(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_write_ipt(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_print_mip(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_read_mip(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_write_mip(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_bf_exists(P: *mut glp_prob) -> libc::c_int;
    fn glp_factorize(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn glp_read_mps(
        P: *mut glp_prob,
        fmt: libc::c_int,
        parm: *const glp_mpscp,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_write_mps(
        P: *mut glp_prob,
        fmt: libc::c_int,
        parm: *const glp_mpscp,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_read_lp(
        P: *mut glp_prob,
        parm: *const glp_cpxcp,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_write_lp(
        P: *mut glp_prob,
        parm: *const glp_cpxcp,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_read_prob(
        P: *mut glp_prob,
        flags: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_write_prob(
        P: *mut glp_prob,
        flags: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_mpl_alloc_wksp() -> *mut glp_tran;
    fn glp_mpl_init_rand(tran: *mut glp_tran, seed: libc::c_int);
    fn glp_mpl_read_model(
        tran: *mut glp_tran,
        fname: *const libc::c_char,
        skip: libc::c_int,
    ) -> libc::c_int;
    fn glp_mpl_read_data(tran: *mut glp_tran, fname: *const libc::c_char) -> libc::c_int;
    fn glp_mpl_generate(tran: *mut glp_tran, fname: *const libc::c_char) -> libc::c_int;
    fn glp_mpl_build_prob(tran: *mut glp_tran, prob: *mut glp_prob);
    fn glp_mpl_postsolve(
        tran: *mut glp_tran,
        prob: *mut glp_prob,
        sol: libc::c_int,
    ) -> libc::c_int;
    fn glp_mpl_free_wksp(tran: *mut glp_tran);
    fn glp_read_cnfsat(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_check_cnfsat(P: *mut glp_prob) -> libc::c_int;
    fn glp_write_cnfsat(P: *mut glp_prob, fname: *const libc::c_char) -> libc::c_int;
    fn glp_minisat1(P: *mut glp_prob) -> libc::c_int;
    fn glp_intfeas1(
        P: *mut glp_prob,
        use_bound: libc::c_int,
        obj_bound: libc::c_int,
    ) -> libc::c_int;
    fn glp_version() -> *const libc::c_char;
    fn glp_free_env() -> libc::c_int;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_open_tee(name: *const libc::c_char) -> libc::c_int;
    fn glp_close_tee() -> libc::c_int;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_mem_limit(limit: libc::c_int);
    fn glp_mem_usage(
        count: *mut libc::c_int,
        cpeak: *mut libc::c_int,
        total: *mut size_t,
        tpeak: *mut size_t,
    );
    fn glp_time() -> libc::c_double;
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_create_graph(v_size: libc::c_int, a_size: libc::c_int) -> *mut glp_graph;
    fn glp_delete_graph(G: *mut glp_graph);
    fn glp_mincost_lp(
        P: *mut glp_prob,
        G: *mut glp_graph,
        names: libc::c_int,
        v_rhs: libc::c_int,
        a_low: libc::c_int,
        a_cap: libc::c_int,
        a_cost: libc::c_int,
    );
    fn glp_maxflow_lp(
        P: *mut glp_prob,
        G: *mut glp_graph,
        names: libc::c_int,
        s: libc::c_int,
        t: libc::c_int,
        a_cap: libc::c_int,
    );
    fn glp_read_mincost(
        G: *mut glp_graph,
        v_rhs: libc::c_int,
        a_low: libc::c_int,
        a_cap: libc::c_int,
        a_cost: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn glp_read_maxflow(
        G: *mut glp_graph,
        s: *mut libc::c_int,
        t: *mut libc::c_int,
        a_cap: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
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
pub struct glp_iptcp {
    pub msg_lev: libc::c_int,
    pub ord_alg: libc::c_int,
    pub foo_bar: [libc::c_double; 48],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_mpscp {
    pub blank: libc::c_int,
    pub obj_name: *mut libc::c_char,
    pub tol_mps: libc::c_double,
    pub foo_bar: [libc::c_double; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_cpxcp {
    pub foo_bar: [libc::c_double; 20],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub prob: *mut glp_prob,
    pub bfcp: glp_bfcp,
    pub smcp: glp_smcp,
    pub iptcp: glp_iptcp,
    pub iocp: glp_iocp,
    pub tran: *mut glp_tran,
    pub graph: *mut glp_graph,
    pub format: libc::c_int,
    pub in_file: *const libc::c_char,
    pub ndf: libc::c_int,
    pub in_data: [*const libc::c_char; 11],
    pub out_dpy: *const libc::c_char,
    pub seed: libc::c_int,
    pub solution: libc::c_int,
    pub in_res: *const libc::c_char,
    pub dir: libc::c_int,
    pub scale: libc::c_int,
    pub out_sol: *const libc::c_char,
    pub out_res: *const libc::c_char,
    pub out_ranges: *const libc::c_char,
    pub check: libc::c_int,
    pub new_name: *const libc::c_char,
    pub hide: libc::c_int,
    pub out_mps: *const libc::c_char,
    pub out_freemps: *const libc::c_char,
    pub out_cpxlp: *const libc::c_char,
    pub out_glp: *const libc::c_char,
    pub out_cnf: *const libc::c_char,
    pub log_file: *const libc::c_char,
    pub crash: libc::c_int,
    pub ini_file: *const libc::c_char,
    pub exact: libc::c_int,
    pub xcheck: libc::c_int,
    pub nomip: libc::c_int,
    pub minisat: libc::c_int,
    pub use_bnd: libc::c_int,
    pub obj_bnd: libc::c_int,
    pub use_sol: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v_data {
    pub rhs: libc::c_double,
    pub pi: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct a_data {
    pub low: libc::c_double,
    pub cap: libc::c_double,
    pub cost: libc::c_double,
    pub x: libc::c_double,
}
unsafe extern "C" fn str2int(
    mut s: *const libc::c_char,
    mut x: *mut libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_long = 0;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    t = strtol(s, &mut endptr, 10 as libc::c_int);
    if *endptr as libc::c_int != '\0' as i32 {
        return 2 as libc::c_int;
    }
    if !((-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long <= t
        && t <= 2147483647 as libc::c_int as libc::c_long)
    {
        return 1 as libc::c_int;
    }
    *x = t as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn str2num(
    mut s: *const libc::c_char,
    mut x: *mut libc::c_double,
) -> libc::c_int {
    let mut t: libc::c_double = 0.;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    t = strtod(s, &mut endptr);
    if *endptr as libc::c_int != '\0' as i32 {
        return 2 as libc::c_int;
    }
    if !(-1.7976931348623157e+308f64 <= t && t <= 1.7976931348623157e+308f64) {
        return 1 as libc::c_int;
    }
    *x = t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_help(mut my_name: *const libc::c_char) {
    glp_printf(
        b"Usage: %s [options...] filename\n\0" as *const u8 as *const libc::c_char,
        my_name,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(b"General options:\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"   --mps             read LP/MIP problem in fixed MPS format\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --freemps         read LP/MIP problem in free MPS format (default)\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --lp              read LP/MIP problem in CPLEX LP format\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --glp             read LP/MIP problem in GLPK format \n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --math            read LP/MIP model written in GNU MathProg modeling\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(b"                     language\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"   -m filename, --model filename\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     read model section and optional data section from\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     filename (same as --math)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   -d filename, --data filename\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     read data section from filename (for --math only);\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     if model file also has data section, it is ignored\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   -y filename, --display filename\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     send display output to filename (for --math only);\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     by default the output is sent to terminal\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --seed value      initialize pseudo-random number generator used in\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     MathProg model with specified seed (any integer);\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     if seed value is ?, some random seed will be used\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --mincost         read min-cost flow problem in DIMACS format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --maxflow         read maximum flow problem in DIMACS format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --cnf             read CNF-SAT problem in DIMACS format\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --simplex         use simplex method (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --interior        use interior point method (LP only)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   -r filename, --read filename\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     read solution from filename rather to find it with\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     the solver\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --min             minimization\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --max             maximization\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --scale           scale problem (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --noscale         do not scale problem\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   -o filename, --output filename\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     write solution to filename in printable format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   -w filename, --write filename\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     write solution to filename in plain text format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(b"   --ranges filename\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"                     write sensitivity analysis report to filename in\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     printable format (simplex only)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --tmlim nnn       limit solution time to nnn seconds \n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --memlim nnn      limit available memory to nnn megabytes\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --check           do not solve problem, check input data only\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --name probname   change problem name to probname\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --hide            remove all symbolic names from problem object\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --wmps filename   write problem to filename in fixed MPS format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(b"   --wfreemps filename\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"                     write problem to filename in free MPS format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --wlp filename    write problem to filename in CPLEX LP format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --wglp filename   write problem to filename in GLPK format\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --wcnf filename   write problem to filename in DIMACS CNF-SAT format\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --log filename    write copy of terminal output to filename\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   -h, --help        display this help information and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   -v, --version     display program version and exit\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"LP basis factorization options:\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --luf             plain LU-factorization (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --btf             block triangular LU-factorization\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --ft              Forrest-Tomlin update (requires --luf; default)\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --cbg             Schur complement + Bartels-Golub update\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --cgr             Schur complement + Givens rotation update\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"Options specific to simplex solver:\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --primal          use primal simplex (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --dual            use dual simplex\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --std             use standard initial basis of all slacks\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --adv             use advanced initial basis (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --bib             use Bixby's initial basis\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --ini filename    use as initial basis previously saved with -w\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     (disables LP presolver)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --steep           use steepest edge technique (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --nosteep         use standard \"textbook\" pricing\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --relax           use Harris' two-pass ratio test (default)\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --norelax         use standard \"textbook\" ratio test\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --flip            use long-step ratio test\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --presol          use presolver (default; assumes --scale and --adv)\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --nopresol        do not use presolver\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --exact           use simplex method based on exact arithmetic\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --xcheck          check final basis using exact arithmetic\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"Options specific to interior-point solver:\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --nord            use natural (original) ordering\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --qmd             use quotient minimum degree ordering\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --amd             use approximate minimum degree ordering (default)\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --symamd          use approximate minimum degree ordering\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"Options specific to MIP solver:\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --nomip           consider all integer variables as continuous\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     (allows solving MIP as pure LP)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --first           branch on first integer variable\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --last            branch on last integer variable\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --mostf           branch on most fractional variable \n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --drtom           branch using heuristic by Driebeck and Tomlin\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     (default)\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --pcost           branch using hybrid pseudocost heuristic (may be\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     useful for hard instances)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --dfs             backtrack using depth first search \n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --bfs             backtrack using breadth first search\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --bestp           backtrack using the best projection heuristic\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --bestb           backtrack using node with best local bound\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     (default)\n\0" as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --intopt          use MIP presolver (default)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --nointopt        do not use MIP presolver\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --binarize        replace general integer variables by binary ones\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     (assumes --intopt)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --fpump           apply feasibility pump heuristic\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --proxy [nnn]     apply proximity search heuristic (nnn is time limit\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     in seconds; default is 60)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --gomory          generate Gomory's mixed integer cuts\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --mir             generate MIR (mixed integer rounding) cuts\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"   --cover           generate mixed cover cuts\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --clique          generate clique cuts\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --cuts            generate all cuts above\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --mipgap tol      set relative mip gap tolerance to tol\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --minisat         translate integer feasibility problem to CNF-SAT\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     and solve it with MiniSat solver\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(
        b"   --objbnd bound    add inequality obj <= bound (minimization) or\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     obj >= bound (maximization) to integer feasibility\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"                     problem (assumes --minisat)\n\0" as *const u8
            as *const libc::c_char,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"For description of the MPS and CPLEX LP formats see Reference Manual.\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"For description of the modeling language see \"GLPK: Modeling Language\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(
        b"GNU MathProg\". Both documents are included in the GLPK distribution.\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"See GLPK web page at <http://www.gnu.org/software/glpk/glpk.html>.\n\0"
            as *const u8 as *const libc::c_char,
    );
    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
    glp_printf(
        b"Please report bugs to <bug-glpk@gnu.org>.\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn print_version(mut briefly: libc::c_int) {
    glp_printf(
        b"GLPSOL--GLPK LP/MIP Solver %s\n\0" as *const u8 as *const libc::c_char,
        glp_version(),
    );
    if !(briefly != 0) {
        glp_printf(
            b"Copyright (C) 2000-2020 Free Software Foundation, Inc.\n\0" as *const u8
                as *const libc::c_char,
        );
        glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
        glp_printf(
            b"This program has ABSOLUTELY NO WARRANTY.\n\0" as *const u8
                as *const libc::c_char,
        );
        glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
        glp_printf(
            b"This program is free software; you may re-distribute it under the terms\n\0"
                as *const u8 as *const libc::c_char,
        );
        glp_printf(
            b"of the GNU General Public License version 3 or later.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn parse_cmdline(
    mut csa: *mut csa,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    k = 1 as libc::c_int;
    while k < argc {
        if strcmp(
            *argv.offset(k as isize),
            b"--mps\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).format = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--freemps\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).format = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--lp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--cpxlp\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            (*csa).format = 3 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--glp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).format = 4 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--math\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"-m\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--model\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            (*csa).format = 5 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--data\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No input data file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if (*csa).ndf == 10 as libc::c_int {
                glp_printf(
                    b"Too many input data files\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).ndf += 1;
            (*csa).in_data[(*csa).ndf as usize] = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"-y\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--display\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No display output file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_dpy).is_null() {
                glp_printf(
                    b"Only one display output file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_dpy = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--seed\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                    && *(*__ctype_b_loc())
                        .offset(
                            *(*argv.offset(k as isize)).offset(1 as libc::c_int as isize)
                                as libc::c_uchar as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                glp_printf(
                    b"No seed value specified\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if strcmp(
                *argv.offset(k as isize),
                b"?\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*csa).seed = 0x80000000 as libc::c_uint as libc::c_int;
            } else if str2int(*argv.offset(k as isize), &mut (*csa).seed) != 0 {
                glp_printf(
                    b"Invalid seed value '%s'\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(k as isize),
                );
                return 1 as libc::c_int;
            }
        } else if strcmp(
            *argv.offset(k as isize),
            b"--mincost\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).format = 6 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--maxflow\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).format = 7 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--cnf\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).format = 8 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--simplex\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).solution = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--interior\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).solution = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--alien\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.alien = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"-r\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--read\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No input solution file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).in_res).is_null() {
                glp_printf(
                    b"Only one input solution file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).in_res = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--min\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).dir = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--max\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).dir = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--scale\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).scale = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--noscale\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).scale = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"-o\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--output\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No output solution file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_sol).is_null() {
                glp_printf(
                    b"Only one output solution file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_sol = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"-w\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--write\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No output solution file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_res).is_null() {
                glp_printf(
                    b"Only one output solution file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_res = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--ranges\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--bounds\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No output file specified to write sensitivity analysis report\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_ranges).is_null() {
                glp_printf(
                    b"Only one output file allowed to write sensitivity analysis report\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_ranges = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--tmlim\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let mut tm_lim: libc::c_int = 0;
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No time limit specified\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if str2int(*argv.offset(k as isize), &mut tm_lim) != 0
                || tm_lim < 0 as libc::c_int
            {
                glp_printf(
                    b"Invalid time limit '%s'\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(k as isize),
                );
                return 1 as libc::c_int;
            }
            if tm_lim <= 2147483647 as libc::c_int / 1000 as libc::c_int {
                (*csa).iocp.tm_lim = 1000 as libc::c_int * tm_lim;
                (*csa).smcp.tm_lim = (*csa).iocp.tm_lim;
            } else {
                (*csa).iocp.tm_lim = 2147483647 as libc::c_int;
                (*csa).smcp.tm_lim = (*csa).iocp.tm_lim;
            }
        } else if strcmp(
            *argv.offset(k as isize),
            b"--memlim\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let mut mem_lim: libc::c_int = 0;
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No memory limit specified\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if str2int(*argv.offset(k as isize), &mut mem_lim) != 0
                || mem_lim < 1 as libc::c_int
            {
                glp_printf(
                    b"Invalid memory limit '%s'\n\0" as *const u8 as *const libc::c_char,
                    *argv.offset(k as isize),
                );
                return 1 as libc::c_int;
            }
            glp_mem_limit(mem_lim);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--check\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).check = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--name\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No problem name specified\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).new_name).is_null() {
                glp_printf(
                    b"Only one problem name allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).new_name = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--hide\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).hide = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--wmps\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No fixed MPS output file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_mps).is_null() {
                glp_printf(
                    b"Only one fixed MPS output file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_mps = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--wfreemps\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No free MPS output file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_freemps).is_null() {
                glp_printf(
                    b"Only one free MPS output file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_freemps = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--wlp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--wcpxlp\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--wlpt\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No CPLEX LP output file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_cpxlp).is_null() {
                glp_printf(
                    b"Only one CPLEX LP output file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_cpxlp = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--wglp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No GLPK LP/MIP output file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_glp).is_null() {
                glp_printf(
                    b"Only one GLPK LP/MIP output file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_glp = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--wcnf\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No problem output file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).out_cnf).is_null() {
                glp_printf(
                    b"Only one output DIMACS CNF-SAT file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).out_cnf = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--log\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No log file specified\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).log_file).is_null() {
                glp_printf(
                    b"Only one log file allowed\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).log_file = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--help\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            print_help(*argv.offset(0 as libc::c_int as isize));
            return -(1 as libc::c_int);
        } else if strcmp(
            *argv.offset(k as isize),
            b"-v\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(k as isize),
                b"--version\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            print_version(0 as libc::c_int);
            return -(1 as libc::c_int);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--luf\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).bfcp.type_0 &= 0xf as libc::c_int;
            (*csa).bfcp.type_0 |= 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--btf\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).bfcp.type_0 &= 0xf as libc::c_int;
            (*csa).bfcp.type_0 |= 0x10 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--ft\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).bfcp.type_0 &= 0xf0 as libc::c_int;
            (*csa).bfcp.type_0 |= 0x1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--cbg\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).bfcp.type_0 &= 0xf0 as libc::c_int;
            (*csa).bfcp.type_0 |= 0x2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--cgr\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).bfcp.type_0 &= 0xf0 as libc::c_int;
            (*csa).bfcp.type_0 |= 0x3 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--primal\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.meth = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--dual\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.meth = 3 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--std\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).crash = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--adv\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).crash = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--bib\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).crash = 3 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--ini\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).crash = 4 as libc::c_int;
            (*csa).smcp.presolve = 0 as libc::c_int;
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No initial basis file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).ini_file).is_null() {
                glp_printf(
                    b"Only one initial basis file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).ini_file = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--steep\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.pricing = 0x22 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--nosteep\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.pricing = 0x11 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--relax\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.r_test = 0x22 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--norelax\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.r_test = 0x11 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--flip\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.r_test = 0x33 as libc::c_int;
            (*csa).iocp.flip = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--presol\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.presolve = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--nopresol\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).smcp.presolve = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--exact\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).exact = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--xcheck\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).xcheck = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--nord\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iptcp.ord_alg = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--qmd\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iptcp.ord_alg = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--amd\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iptcp.ord_alg = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--symamd\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iptcp.ord_alg = 3 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--nomip\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).nomip = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--first\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.br_tech = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--last\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.br_tech = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--drtom\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.br_tech = 4 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--mostf\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.br_tech = 3 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--pcost\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.br_tech = 5 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--dfs\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.bt_tech = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--bfs\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.bt_tech = 2 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--bestp\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.bt_tech = 4 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--bestb\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.bt_tech = 3 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--intopt\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.presolve = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--nointopt\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.presolve = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--binarize\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.binarize = 1 as libc::c_int;
            (*csa).iocp.presolve = (*csa).iocp.binarize;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--fpump\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.fp_heur = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--proxy\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.ps_heur = 1 as libc::c_int;
            if !(*argv.offset((k + 1 as libc::c_int) as isize)).is_null()
                && *(*__ctype_b_loc())
                    .offset(
                        *(*argv.offset((k + 1 as libc::c_int) as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                let mut nnn: libc::c_int = 0;
                k += 1;
                k;
                if str2int(*argv.offset(k as isize), &mut nnn) != 0
                    || nnn < 1 as libc::c_int
                {
                    glp_printf(
                        b"Invalid proxy time limit '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(k as isize),
                    );
                    return 1 as libc::c_int;
                }
                (*csa).iocp.ps_tm_lim = 1000 as libc::c_int * nnn;
            }
        } else if strcmp(
            *argv.offset(k as isize),
            b"--gomory\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.gmi_cuts = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--mir\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.mir_cuts = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--cover\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.cov_cuts = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--clique\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.clq_cuts = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--cuts\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).iocp.clq_cuts = 1 as libc::c_int;
            (*csa).iocp.cov_cuts = (*csa).iocp.clq_cuts;
            (*csa).iocp.mir_cuts = (*csa).iocp.cov_cuts;
            (*csa).iocp.gmi_cuts = (*csa).iocp.mir_cuts;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--mipgap\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let mut mip_gap: libc::c_double = 0.;
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No relative gap tolerance specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if str2num(*argv.offset(k as isize), &mut mip_gap) != 0 || mip_gap < 0.0f64 {
                glp_printf(
                    b"Invalid relative mip gap tolerance '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(k as isize),
                );
                return 1 as libc::c_int;
            }
            (*csa).iocp.mip_gap = mip_gap;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--minisat\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*csa).minisat = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(k as isize),
            b"--objbnd\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                    && *(*__ctype_b_loc())
                        .offset(
                            *(*argv.offset(k as isize)).offset(1 as libc::c_int as isize)
                                as libc::c_uchar as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                glp_printf(
                    b"No objective bound specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).minisat = 1 as libc::c_int;
            (*csa).use_bnd = 1 as libc::c_int;
            if str2int(*argv.offset(k as isize), &mut (*csa).obj_bnd) != 0 {
                glp_printf(
                    b"Invalid objective bound '%s' (should be integer value)\n\0"
                        as *const u8 as *const libc::c_char,
                    *argv.offset(k as isize),
                );
                return 1 as libc::c_int;
            }
        } else if strcmp(
            *argv.offset(k as isize),
            b"--use\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No input MIP solution file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).use_sol).is_null() {
                glp_printf(
                    b"Only one input MIP solution file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).use_sol = *argv.offset(k as isize);
        } else if strcmp(
            *argv.offset(k as isize),
            b"--save\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            k += 1;
            k;
            if k == argc
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                glp_printf(
                    b"No output MIP solution file specified\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            if !((*csa).iocp.save_sol).is_null() {
                glp_printf(
                    b"Only one output MIP solution file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).iocp.save_sol = *argv.offset(k as isize);
        } else if *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '-' as i32
            || *(*argv.offset(k as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32
                && *(*argv.offset(k as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
        {
            glp_printf(
                b"Invalid option '%s'; try %s --help\n\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(k as isize),
                *argv.offset(0 as libc::c_int as isize),
            );
            return 1 as libc::c_int;
        } else {
            if !((*csa).in_file).is_null() {
                glp_printf(
                    b"Only one input problem file allowed\n\0" as *const u8
                        as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            (*csa).in_file = *argv.offset(k as isize);
        }
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut _csa: csa = csa {
        prob: 0 as *mut glp_prob,
        bfcp: glp_bfcp {
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
        },
        smcp: glp_smcp {
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
        },
        iptcp: glp_iptcp {
            msg_lev: 0,
            ord_alg: 0,
            foo_bar: [0.; 48],
        },
        iocp: glp_iocp {
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
        },
        tran: 0 as *mut glp_tran,
        graph: 0 as *mut glp_graph,
        format: 0,
        in_file: 0 as *const libc::c_char,
        ndf: 0,
        in_data: [0 as *const libc::c_char; 11],
        out_dpy: 0 as *const libc::c_char,
        seed: 0,
        solution: 0,
        in_res: 0 as *const libc::c_char,
        dir: 0,
        scale: 0,
        out_sol: 0 as *const libc::c_char,
        out_res: 0 as *const libc::c_char,
        out_ranges: 0 as *const libc::c_char,
        check: 0,
        new_name: 0 as *const libc::c_char,
        hide: 0,
        out_mps: 0 as *const libc::c_char,
        out_freemps: 0 as *const libc::c_char,
        out_cpxlp: 0 as *const libc::c_char,
        out_glp: 0 as *const libc::c_char,
        out_cnf: 0 as *const libc::c_char,
        log_file: 0 as *const libc::c_char,
        crash: 0,
        ini_file: 0 as *const libc::c_char,
        exact: 0,
        xcheck: 0,
        nomip: 0,
        minisat: 0,
        use_bnd: 0,
        obj_bnd: 0,
        use_sol: 0 as *const libc::c_char,
    };
    let mut csa: *mut csa = &mut _csa;
    let mut ret: libc::c_int = 0;
    let mut start: libc::c_double = 0.;
    (*csa).prob = glp_create_prob();
    glp_get_bfcp((*csa).prob, &mut (*csa).bfcp);
    glp_init_smcp(&mut (*csa).smcp);
    (*csa).smcp.presolve = 1 as libc::c_int;
    glp_init_iptcp(&mut (*csa).iptcp);
    glp_init_iocp(&mut (*csa).iocp);
    (*csa).iocp.presolve = 1 as libc::c_int;
    (*csa).tran = 0 as *mut glp_tran;
    (*csa).graph = 0 as *mut glp_graph;
    (*csa).format = 2 as libc::c_int;
    (*csa).in_file = 0 as *const libc::c_char;
    (*csa).ndf = 0 as libc::c_int;
    (*csa).out_dpy = 0 as *const libc::c_char;
    (*csa).seed = 1 as libc::c_int;
    (*csa).solution = 1 as libc::c_int;
    (*csa).in_res = 0 as *const libc::c_char;
    (*csa).dir = 0 as libc::c_int;
    (*csa).scale = 1 as libc::c_int;
    (*csa).out_sol = 0 as *const libc::c_char;
    (*csa).out_res = 0 as *const libc::c_char;
    (*csa).out_ranges = 0 as *const libc::c_char;
    (*csa).check = 0 as libc::c_int;
    (*csa).new_name = 0 as *const libc::c_char;
    (*csa).hide = 0 as libc::c_int;
    (*csa).out_mps = 0 as *const libc::c_char;
    (*csa).out_freemps = 0 as *const libc::c_char;
    (*csa).out_cpxlp = 0 as *const libc::c_char;
    (*csa).out_glp = 0 as *const libc::c_char;
    (*csa).out_cnf = 0 as *const libc::c_char;
    (*csa).log_file = 0 as *const libc::c_char;
    (*csa).crash = 2 as libc::c_int;
    (*csa).ini_file = 0 as *const libc::c_char;
    (*csa).exact = 0 as libc::c_int;
    (*csa).xcheck = 0 as libc::c_int;
    (*csa).nomip = 0 as libc::c_int;
    (*csa).minisat = 0 as libc::c_int;
    (*csa).use_bnd = 0 as libc::c_int;
    (*csa).obj_bnd = 0 as libc::c_int;
    (*csa).use_sol = 0 as *const libc::c_char;
    ret = parse_cmdline(csa, argc, argv);
    if ret < 0 as libc::c_int {
        ret = 0 as libc::c_int;
    } else if ret > 0 as libc::c_int {
        ret = 1 as libc::c_int;
    } else {
        if !((*csa).out_dpy).is_null() {
            remove((*csa).out_dpy);
        }
        if !((*csa).out_sol).is_null() {
            remove((*csa).out_sol);
        }
        if !((*csa).out_res).is_null() {
            remove((*csa).out_res);
        }
        if !((*csa).out_ranges).is_null() {
            remove((*csa).out_ranges);
        }
        if !((*csa).out_mps).is_null() {
            remove((*csa).out_mps);
        }
        if !((*csa).out_freemps).is_null() {
            remove((*csa).out_freemps);
        }
        if !((*csa).out_cpxlp).is_null() {
            remove((*csa).out_cpxlp);
        }
        if !((*csa).out_glp).is_null() {
            remove((*csa).out_glp);
        }
        if !((*csa).out_cnf).is_null() {
            remove((*csa).out_cnf);
        }
        if !((*csa).log_file).is_null() {
            remove((*csa).log_file);
        }
        if !((*csa).log_file).is_null() {
            if glp_open_tee((*csa).log_file) != 0 {
                glp_printf(
                    b"Unable to create log file\n\0" as *const u8 as *const libc::c_char,
                );
                ret = 1 as libc::c_int;
                current_block = 16495531352233468986;
            } else {
                current_block = 16738040538446813684;
            }
        } else {
            current_block = 16738040538446813684;
        }
        match current_block {
            16495531352233468986 => {}
            _ => {
                print_version(1 as libc::c_int);
                if argc > 1 as libc::c_int {
                    let mut k: libc::c_int = 0;
                    let mut len: libc::c_int = 2147483647 as libc::c_int;
                    glp_printf(
                        b"Parameter(s) specified in the command line:\0" as *const u8
                            as *const libc::c_char,
                    );
                    k = 1 as libc::c_int;
                    while k < argc {
                        if len > 72 as libc::c_int {
                            glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
                            len = 0 as libc::c_int;
                        }
                        glp_printf(
                            b" %s\0" as *const u8 as *const libc::c_char,
                            *argv.offset(k as isize),
                        );
                        len = (len as libc::c_ulong)
                            .wrapping_add(
                                (1 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(strlen(*argv.offset(k as isize))),
                            ) as libc::c_int as libc::c_int;
                        k += 1;
                        k;
                    }
                    glp_printf(b"\n\0" as *const u8 as *const libc::c_char);
                }
                if ((*csa).in_file).is_null() {
                    glp_printf(
                        b"No input problem file specified; try %s --help\n\0"
                            as *const u8 as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                    );
                    ret = 1 as libc::c_int;
                } else {
                    if (*csa).format == 1 as libc::c_int {
                        ret = glp_read_mps(
                            (*csa).prob,
                            1 as libc::c_int,
                            0 as *const glp_mpscp,
                            (*csa).in_file,
                        );
                        if ret != 0 as libc::c_int {
                            current_block = 14001958660280927786;
                        } else {
                            current_block = 5195798230510548452;
                        }
                    } else if (*csa).format == 2 as libc::c_int {
                        ret = glp_read_mps(
                            (*csa).prob,
                            2 as libc::c_int,
                            0 as *const glp_mpscp,
                            (*csa).in_file,
                        );
                        if ret != 0 as libc::c_int {
                            current_block = 14001958660280927786;
                        } else {
                            current_block = 5195798230510548452;
                        }
                    } else if (*csa).format == 3 as libc::c_int {
                        ret = glp_read_lp(
                            (*csa).prob,
                            0 as *const glp_cpxcp,
                            (*csa).in_file,
                        );
                        if ret != 0 as libc::c_int {
                            glp_printf(
                                b"CPLEX LP file processing error\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ret = 1 as libc::c_int;
                            current_block = 16495531352233468986;
                        } else {
                            current_block = 5195798230510548452;
                        }
                    } else if (*csa).format == 4 as libc::c_int {
                        ret = glp_read_prob(
                            (*csa).prob,
                            0 as libc::c_int,
                            (*csa).in_file,
                        );
                        if ret != 0 as libc::c_int {
                            glp_printf(
                                b"GLPK LP/MIP file processing error\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ret = 1 as libc::c_int;
                            current_block = 16495531352233468986;
                        } else {
                            current_block = 5195798230510548452;
                        }
                    } else if (*csa).format == 5 as libc::c_int {
                        let mut k_0: libc::c_int = 0;
                        (*csa).tran = glp_mpl_alloc_wksp();
                        if (*csa).seed as libc::c_uint == 0x80000000 as libc::c_uint {
                            (*csa)
                                .seed = fmod(glp_time(), 1000000000.0f64) as libc::c_int;
                            glp_printf(
                                b"Seed value %d will be used\n\0" as *const u8
                                    as *const libc::c_char,
                                (*csa).seed,
                            );
                        }
                        glp_mpl_init_rand((*csa).tran, (*csa).seed);
                        if glp_mpl_read_model(
                            (*csa).tran,
                            (*csa).in_file,
                            ((*csa).ndf > 0 as libc::c_int) as libc::c_int,
                        ) != 0
                        {
                            current_block = 18325745679564279244;
                        } else {
                            k_0 = 1 as libc::c_int;
                            loop {
                                if !(k_0 <= (*csa).ndf) {
                                    current_block = 10261677128829721533;
                                    break;
                                }
                                if glp_mpl_read_data(
                                    (*csa).tran,
                                    (*csa).in_data[k_0 as usize],
                                ) != 0
                                {
                                    current_block = 18325745679564279244;
                                    break;
                                }
                                k_0 += 1;
                                k_0;
                            }
                            match current_block {
                                18325745679564279244 => {}
                                _ => {
                                    if glp_mpl_generate((*csa).tran, (*csa).out_dpy) != 0 {
                                        current_block = 18325745679564279244;
                                    } else {
                                        glp_mpl_build_prob((*csa).tran, (*csa).prob);
                                        current_block = 5195798230510548452;
                                    }
                                }
                            }
                        }
                        match current_block {
                            5195798230510548452 => {}
                            _ => {
                                glp_printf(
                                    b"MathProg model processing error\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                ret = 1 as libc::c_int;
                                current_block = 16495531352233468986;
                            }
                        }
                    } else if (*csa).format == 6 as libc::c_int {
                        (*csa)
                            .graph = glp_create_graph(
                            ::std::mem::size_of::<v_data>() as libc::c_ulong
                                as libc::c_int,
                            ::std::mem::size_of::<a_data>() as libc::c_ulong
                                as libc::c_int,
                        );
                        ret = glp_read_mincost(
                            (*csa).graph,
                            0 as libc::c_ulong as libc::c_int,
                            0 as libc::c_ulong as libc::c_int,
                            8 as libc::c_ulong as libc::c_int,
                            16 as libc::c_ulong as libc::c_int,
                            (*csa).in_file,
                        );
                        if ret != 0 as libc::c_int {
                            glp_printf(
                                b"DIMACS file processing error\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ret = 1 as libc::c_int;
                            current_block = 16495531352233468986;
                        } else {
                            glp_mincost_lp(
                                (*csa).prob,
                                (*csa).graph,
                                1 as libc::c_int,
                                0 as libc::c_ulong as libc::c_int,
                                0 as libc::c_ulong as libc::c_int,
                                8 as libc::c_ulong as libc::c_int,
                                16 as libc::c_ulong as libc::c_int,
                            );
                            glp_set_prob_name((*csa).prob, (*csa).in_file);
                            current_block = 5195798230510548452;
                        }
                    } else if (*csa).format == 7 as libc::c_int {
                        let mut s: libc::c_int = 0;
                        let mut t: libc::c_int = 0;
                        (*csa)
                            .graph = glp_create_graph(
                            ::std::mem::size_of::<v_data>() as libc::c_ulong
                                as libc::c_int,
                            ::std::mem::size_of::<a_data>() as libc::c_ulong
                                as libc::c_int,
                        );
                        ret = glp_read_maxflow(
                            (*csa).graph,
                            &mut s,
                            &mut t,
                            8 as libc::c_ulong as libc::c_int,
                            (*csa).in_file,
                        );
                        if ret != 0 as libc::c_int {
                            glp_printf(
                                b"DIMACS file processing error\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ret = 1 as libc::c_int;
                            current_block = 16495531352233468986;
                        } else {
                            glp_maxflow_lp(
                                (*csa).prob,
                                (*csa).graph,
                                1 as libc::c_int,
                                s,
                                t,
                                8 as libc::c_ulong as libc::c_int,
                            );
                            glp_set_prob_name((*csa).prob, (*csa).in_file);
                            current_block = 5195798230510548452;
                        }
                    } else if (*csa).format == 8 as libc::c_int {
                        ret = glp_read_cnfsat((*csa).prob, (*csa).in_file);
                        if ret != 0 as libc::c_int {
                            glp_printf(
                                b"DIMACS file processing error\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ret = 1 as libc::c_int;
                            current_block = 16495531352233468986;
                        } else {
                            glp_set_prob_name((*csa).prob, (*csa).in_file);
                            current_block = 5195798230510548452;
                        }
                    } else {
                        (csa != csa
                            || {
                                glp_assert_(
                                    b"csa != csa\0" as *const u8 as *const libc::c_char,
                                    b"glpsol.c\0" as *const u8 as *const libc::c_char,
                                    1178 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        current_block = 5195798230510548452;
                    }
                    match current_block {
                        16495531352233468986 => {}
                        _ => {
                            match current_block {
                                14001958660280927786 => {
                                    glp_printf(
                                        b"MPS file processing error\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    ret = 1 as libc::c_int;
                                }
                                _ => {
                                    if !((*csa).new_name).is_null() {
                                        glp_set_prob_name((*csa).prob, (*csa).new_name);
                                    }
                                    if (*csa).dir != 0 as libc::c_int {
                                        glp_set_obj_dir((*csa).prob, (*csa).dir);
                                    }
                                    glp_sort_matrix((*csa).prob);
                                    if (*csa).hide != 0 {
                                        let mut i: libc::c_int = 0;
                                        let mut j: libc::c_int = 0;
                                        glp_set_obj_name((*csa).prob, 0 as *const libc::c_char);
                                        glp_delete_index((*csa).prob);
                                        i = glp_get_num_rows((*csa).prob);
                                        while i >= 1 as libc::c_int {
                                            glp_set_row_name((*csa).prob, i, 0 as *const libc::c_char);
                                            i -= 1;
                                            i;
                                        }
                                        j = glp_get_num_cols((*csa).prob);
                                        while j >= 1 as libc::c_int {
                                            glp_set_col_name((*csa).prob, j, 0 as *const libc::c_char);
                                            j -= 1;
                                            j;
                                        }
                                    }
                                    if !((*csa).out_mps).is_null() {
                                        ret = glp_write_mps(
                                            (*csa).prob,
                                            1 as libc::c_int,
                                            0 as *const glp_mpscp,
                                            (*csa).out_mps,
                                        );
                                        if ret != 0 as libc::c_int {
                                            glp_printf(
                                                b"Unable to write problem in fixed MPS format\n\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                            ret = 1 as libc::c_int;
                                            current_block = 16495531352233468986;
                                        } else {
                                            current_block = 4736343053266048700;
                                        }
                                    } else {
                                        current_block = 4736343053266048700;
                                    }
                                    match current_block {
                                        16495531352233468986 => {}
                                        _ => {
                                            if !((*csa).out_freemps).is_null() {
                                                ret = glp_write_mps(
                                                    (*csa).prob,
                                                    2 as libc::c_int,
                                                    0 as *const glp_mpscp,
                                                    (*csa).out_freemps,
                                                );
                                                if ret != 0 as libc::c_int {
                                                    glp_printf(
                                                        b"Unable to write problem in free MPS format\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                    );
                                                    ret = 1 as libc::c_int;
                                                    current_block = 16495531352233468986;
                                                } else {
                                                    current_block = 10938659635288570931;
                                                }
                                            } else {
                                                current_block = 10938659635288570931;
                                            }
                                            match current_block {
                                                16495531352233468986 => {}
                                                _ => {
                                                    if !((*csa).out_cpxlp).is_null() {
                                                        ret = glp_write_lp(
                                                            (*csa).prob,
                                                            0 as *const glp_cpxcp,
                                                            (*csa).out_cpxlp,
                                                        );
                                                        if ret != 0 as libc::c_int {
                                                            glp_printf(
                                                                b"Unable to write problem in CPLEX LP format\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                            );
                                                            ret = 1 as libc::c_int;
                                                            current_block = 16495531352233468986;
                                                        } else {
                                                            current_block = 15321816652064063775;
                                                        }
                                                    } else {
                                                        current_block = 15321816652064063775;
                                                    }
                                                    match current_block {
                                                        16495531352233468986 => {}
                                                        _ => {
                                                            if !((*csa).out_glp).is_null() {
                                                                ret = glp_write_prob(
                                                                    (*csa).prob,
                                                                    0 as libc::c_int,
                                                                    (*csa).out_glp,
                                                                );
                                                                if ret != 0 as libc::c_int {
                                                                    glp_printf(
                                                                        b"Unable to write problem in GLPK format\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                    ret = 1 as libc::c_int;
                                                                    current_block = 16495531352233468986;
                                                                } else {
                                                                    current_block = 5151888778912688305;
                                                                }
                                                            } else {
                                                                current_block = 5151888778912688305;
                                                            }
                                                            match current_block {
                                                                16495531352233468986 => {}
                                                                _ => {
                                                                    if !((*csa).out_cnf).is_null() {
                                                                        ret = glp_write_cnfsat((*csa).prob, (*csa).out_cnf);
                                                                        if ret != 0 as libc::c_int {
                                                                            glp_printf(
                                                                                b"Unable to write problem in DIMACS CNF-SAT format\n\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                            );
                                                                            ret = 1 as libc::c_int;
                                                                            current_block = 16495531352233468986;
                                                                        } else {
                                                                            current_block = 12223373342341601825;
                                                                        }
                                                                    } else {
                                                                        current_block = 12223373342341601825;
                                                                    }
                                                                    match current_block {
                                                                        16495531352233468986 => {}
                                                                        _ => {
                                                                            if (*csa).check != 0 {
                                                                                let mut j_0: libc::c_int = 0;
                                                                                let mut cnt: libc::c_int = 0 as libc::c_int;
                                                                                glp_printf(
                                                                                    b"--- Problem Characteristics ---\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                );
                                                                                glp_printf(
                                                                                    b"Number of rows               = %8d\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    glp_get_num_rows((*csa).prob),
                                                                                );
                                                                                glp_printf(
                                                                                    b"Number of columns            = %8d\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    glp_get_num_cols((*csa).prob),
                                                                                );
                                                                                glp_printf(
                                                                                    b"Number of non-zeros (matrix) = %8d\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    glp_get_num_nz((*csa).prob),
                                                                                );
                                                                                j_0 = glp_get_num_cols((*csa).prob);
                                                                                while j_0 >= 1 as libc::c_int {
                                                                                    if glp_get_obj_coef((*csa).prob, j_0) != 0.0f64 {
                                                                                        cnt += 1;
                                                                                        cnt;
                                                                                    }
                                                                                    j_0 -= 1;
                                                                                    j_0;
                                                                                }
                                                                                glp_printf(
                                                                                    b"Number of non-zeros (objrow) = %8d\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    cnt,
                                                                                );
                                                                                ret = 0 as libc::c_int;
                                                                            } else {
                                                                                if (*csa).nomip == 0
                                                                                    && glp_get_num_int((*csa).prob)
                                                                                        + glp_get_num_bin((*csa).prob) > 0 as libc::c_int
                                                                                {
                                                                                    if (*csa).solution == 2 as libc::c_int {
                                                                                        glp_printf(
                                                                                            b"Interior-point method is not able to solve MIP problem; use --simplex\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        ret = 1 as libc::c_int;
                                                                                        current_block = 16495531352233468986;
                                                                                    } else {
                                                                                        (*csa).solution = 3 as libc::c_int;
                                                                                        current_block = 12969817083969514432;
                                                                                    }
                                                                                } else {
                                                                                    current_block = 12969817083969514432;
                                                                                }
                                                                                match current_block {
                                                                                    16495531352233468986 => {}
                                                                                    _ => {
                                                                                        if !((*csa).in_res).is_null() {
                                                                                            if (*csa).solution == 1 as libc::c_int {
                                                                                                ret = glp_read_sol((*csa).prob, (*csa).in_res);
                                                                                            } else if (*csa).solution == 2 as libc::c_int {
                                                                                                ret = glp_read_ipt((*csa).prob, (*csa).in_res);
                                                                                            } else if (*csa).solution == 3 as libc::c_int {
                                                                                                ret = glp_read_mip((*csa).prob, (*csa).in_res);
                                                                                            } else {
                                                                                                (csa != csa
                                                                                                    || {
                                                                                                        glp_assert_(
                                                                                                            b"csa != csa\0" as *const u8 as *const libc::c_char,
                                                                                                            b"glpsol.c\0" as *const u8 as *const libc::c_char,
                                                                                                            1319 as libc::c_int,
                                                                                                        );
                                                                                                        1 as libc::c_int != 0
                                                                                                    }) as libc::c_int;
                                                                                            }
                                                                                            if ret != 0 as libc::c_int {
                                                                                                glp_printf(
                                                                                                    b"Unable to read problem solution\n\0" as *const u8
                                                                                                        as *const libc::c_char,
                                                                                                );
                                                                                                ret = 1 as libc::c_int;
                                                                                                current_block = 16495531352233468986;
                                                                                            } else {
                                                                                                current_block = 9653143987502044805;
                                                                                            }
                                                                                        } else {
                                                                                            if (*csa).solution == 3 as libc::c_int
                                                                                                && !((*csa).use_sol).is_null()
                                                                                            {
                                                                                                ret = glp_read_mip((*csa).prob, (*csa).use_sol);
                                                                                                if ret != 0 as libc::c_int {
                                                                                                    glp_printf(
                                                                                                        b"Unable to read initial MIP solution\n\0" as *const u8
                                                                                                            as *const libc::c_char,
                                                                                                    );
                                                                                                    ret = 1 as libc::c_int;
                                                                                                    current_block = 16495531352233468986;
                                                                                                } else {
                                                                                                    (*csa).iocp.use_sol = 1 as libc::c_int;
                                                                                                    current_block = 2925215368761540503;
                                                                                                }
                                                                                            } else {
                                                                                                current_block = 2925215368761540503;
                                                                                            }
                                                                                            match current_block {
                                                                                                16495531352233468986 => {}
                                                                                                _ => {
                                                                                                    if (*csa).scale != 0 {
                                                                                                        if (*csa).solution == 1 as libc::c_int
                                                                                                            && (*csa).smcp.presolve == 0
                                                                                                            || (*csa).solution == 2 as libc::c_int
                                                                                                            || (*csa).solution == 3 as libc::c_int
                                                                                                                && (*csa).iocp.presolve == 0
                                                                                                        {
                                                                                                            glp_scale_prob((*csa).prob, 0x80 as libc::c_int);
                                                                                                        }
                                                                                                    }
                                                                                                    if (*csa).solution == 1 as libc::c_int
                                                                                                        && (*csa).smcp.presolve == 0
                                                                                                        || (*csa).solution == 3 as libc::c_int
                                                                                                            && (*csa).iocp.presolve == 0
                                                                                                    {
                                                                                                        if (*csa).crash == 1 as libc::c_int {
                                                                                                            glp_std_basis((*csa).prob);
                                                                                                            current_block = 4877859826192283278;
                                                                                                        } else if (*csa).crash == 2 as libc::c_int {
                                                                                                            glp_adv_basis((*csa).prob, 0 as libc::c_int);
                                                                                                            current_block = 4877859826192283278;
                                                                                                        } else if (*csa).crash == 3 as libc::c_int {
                                                                                                            glp_cpx_basis((*csa).prob);
                                                                                                            current_block = 4877859826192283278;
                                                                                                        } else if (*csa).crash == 4 as libc::c_int {
                                                                                                            ret = glp_read_sol((*csa).prob, (*csa).ini_file);
                                                                                                            if ret != 0 as libc::c_int {
                                                                                                                glp_printf(
                                                                                                                    b"Unable to read initial basis\n\0" as *const u8
                                                                                                                        as *const libc::c_char,
                                                                                                                );
                                                                                                                ret = 1 as libc::c_int;
                                                                                                                current_block = 16495531352233468986;
                                                                                                            } else {
                                                                                                                current_block = 4877859826192283278;
                                                                                                            }
                                                                                                        } else {
                                                                                                            (csa != csa
                                                                                                                || {
                                                                                                                    glp_assert_(
                                                                                                                        b"csa != csa\0" as *const u8 as *const libc::c_char,
                                                                                                                        b"glpsol.c\0" as *const u8 as *const libc::c_char,
                                                                                                                        1367 as libc::c_int,
                                                                                                                    );
                                                                                                                    1 as libc::c_int != 0
                                                                                                                }) as libc::c_int;
                                                                                                            current_block = 4877859826192283278;
                                                                                                        }
                                                                                                    } else {
                                                                                                        current_block = 4877859826192283278;
                                                                                                    }
                                                                                                    match current_block {
                                                                                                        16495531352233468986 => {}
                                                                                                        _ => {
                                                                                                            start = glp_time();
                                                                                                            if (*csa).solution == 1 as libc::c_int {
                                                                                                                if (*csa).exact == 0 {
                                                                                                                    glp_set_bfcp((*csa).prob, &mut (*csa).bfcp);
                                                                                                                    glp_simplex((*csa).prob, &mut (*csa).smcp);
                                                                                                                    if (*csa).xcheck != 0 {
                                                                                                                        if (*csa).smcp.presolve != 0
                                                                                                                            && glp_get_status((*csa).prob) != 5 as libc::c_int
                                                                                                                        {
                                                                                                                            glp_printf(
                                                                                                                                b"If you need to check final basis for non-optimal solution, use --nopresol\n\0"
                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                            );
                                                                                                                        } else {
                                                                                                                            glp_exact((*csa).prob, &mut (*csa).smcp);
                                                                                                                        }
                                                                                                                    }
                                                                                                                    if !((*csa).out_sol).is_null()
                                                                                                                        || !((*csa).out_res).is_null()
                                                                                                                    {
                                                                                                                        if (*csa).smcp.presolve != 0
                                                                                                                            && glp_get_status((*csa).prob) != 5 as libc::c_int
                                                                                                                        {
                                                                                                                            glp_printf(
                                                                                                                                b"If you need actual output for non-optimal solution, use --nopresol\n\0"
                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                            );
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    glp_exact((*csa).prob, &mut (*csa).smcp);
                                                                                                                }
                                                                                                            } else if (*csa).solution == 2 as libc::c_int {
                                                                                                                glp_interior((*csa).prob, &mut (*csa).iptcp);
                                                                                                            } else if (*csa).solution == 3 as libc::c_int
                                                                                                                && (*csa).minisat != 0
                                                                                                            {
                                                                                                                if glp_check_cnfsat((*csa).prob) == 0 as libc::c_int {
                                                                                                                    glp_minisat1((*csa).prob);
                                                                                                                } else {
                                                                                                                    glp_intfeas1((*csa).prob, (*csa).use_bnd, (*csa).obj_bnd);
                                                                                                                }
                                                                                                            } else if (*csa).solution == 3 as libc::c_int {
                                                                                                                glp_set_bfcp((*csa).prob, &mut (*csa).bfcp);
                                                                                                                if (*csa).iocp.presolve == 0 {
                                                                                                                    glp_simplex((*csa).prob, &mut (*csa).smcp);
                                                                                                                }
                                                                                                                glp_intopt((*csa).prob, &mut (*csa).iocp);
                                                                                                            } else {
                                                                                                                (csa != csa
                                                                                                                    || {
                                                                                                                        glp_assert_(
                                                                                                                            b"csa != csa\0" as *const u8 as *const libc::c_char,
                                                                                                                            b"glpsol.c\0" as *const u8 as *const libc::c_char,
                                                                                                                            1421 as libc::c_int,
                                                                                                                        );
                                                                                                                        1 as libc::c_int != 0
                                                                                                                    }) as libc::c_int;
                                                                                                            }
                                                                                                            glp_printf(
                                                                                                                b"Time used:   %.1f secs\n\0" as *const u8
                                                                                                                    as *const libc::c_char,
                                                                                                                glp_difftime(glp_time(), start),
                                                                                                            );
                                                                                                            let mut tpeak: size_t = 0;
                                                                                                            glp_mem_usage(
                                                                                                                0 as *mut libc::c_int,
                                                                                                                0 as *mut libc::c_int,
                                                                                                                0 as *mut size_t,
                                                                                                                &mut tpeak,
                                                                                                            );
                                                                                                            glp_printf(
                                                                                                                b"Memory used: %.1f Mb (%.0f bytes)\n\0" as *const u8
                                                                                                                    as *const libc::c_char,
                                                                                                                tpeak as libc::c_double / 1048576.0f64,
                                                                                                                tpeak as libc::c_double,
                                                                                                            );
                                                                                                            current_block = 9653143987502044805;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        match current_block {
                                                                                            16495531352233468986 => {}
                                                                                            _ => {
                                                                                                if !((*csa).tran).is_null() {
                                                                                                    if (*csa).solution == 1 as libc::c_int {
                                                                                                        if !(glp_get_status((*csa).prob) == 5 as libc::c_int
                                                                                                            || glp_get_status((*csa).prob) == 2 as libc::c_int)
                                                                                                        {
                                                                                                            ret = -(1 as libc::c_int);
                                                                                                        } else {
                                                                                                            ret = glp_mpl_postsolve(
                                                                                                                (*csa).tran,
                                                                                                                (*csa).prob,
                                                                                                                1 as libc::c_int,
                                                                                                            );
                                                                                                        }
                                                                                                    } else if (*csa).solution == 2 as libc::c_int {
                                                                                                        if !(glp_ipt_status((*csa).prob) == 5 as libc::c_int
                                                                                                            || glp_ipt_status((*csa).prob) == 2 as libc::c_int)
                                                                                                        {
                                                                                                            ret = -(1 as libc::c_int);
                                                                                                        } else {
                                                                                                            ret = glp_mpl_postsolve(
                                                                                                                (*csa).tran,
                                                                                                                (*csa).prob,
                                                                                                                2 as libc::c_int,
                                                                                                            );
                                                                                                        }
                                                                                                    } else if (*csa).solution == 3 as libc::c_int {
                                                                                                        if !(glp_mip_status((*csa).prob) == 5 as libc::c_int
                                                                                                            || glp_mip_status((*csa).prob) == 2 as libc::c_int)
                                                                                                        {
                                                                                                            ret = -(1 as libc::c_int);
                                                                                                        } else {
                                                                                                            ret = glp_mpl_postsolve(
                                                                                                                (*csa).tran,
                                                                                                                (*csa).prob,
                                                                                                                3 as libc::c_int,
                                                                                                            );
                                                                                                        }
                                                                                                    } else {
                                                                                                        (csa != csa
                                                                                                            || {
                                                                                                                glp_assert_(
                                                                                                                    b"csa != csa\0" as *const u8 as *const libc::c_char,
                                                                                                                    b"glpsol.c\0" as *const u8 as *const libc::c_char,
                                                                                                                    1465 as libc::c_int,
                                                                                                                );
                                                                                                                1 as libc::c_int != 0
                                                                                                            }) as libc::c_int;
                                                                                                    }
                                                                                                    if ret > 0 as libc::c_int {
                                                                                                        glp_printf(
                                                                                                            b"Model postsolving error\n\0" as *const u8
                                                                                                                as *const libc::c_char,
                                                                                                        );
                                                                                                        ret = 1 as libc::c_int;
                                                                                                        current_block = 16495531352233468986;
                                                                                                    } else {
                                                                                                        current_block = 16225421537987648902;
                                                                                                    }
                                                                                                } else {
                                                                                                    current_block = 16225421537987648902;
                                                                                                }
                                                                                                match current_block {
                                                                                                    16495531352233468986 => {}
                                                                                                    _ => {
                                                                                                        if !((*csa).out_sol).is_null() {
                                                                                                            if (*csa).solution == 1 as libc::c_int {
                                                                                                                ret = glp_print_sol((*csa).prob, (*csa).out_sol);
                                                                                                            } else if (*csa).solution == 2 as libc::c_int {
                                                                                                                ret = glp_print_ipt((*csa).prob, (*csa).out_sol);
                                                                                                            } else if (*csa).solution == 3 as libc::c_int {
                                                                                                                ret = glp_print_mip((*csa).prob, (*csa).out_sol);
                                                                                                            } else {
                                                                                                                (csa != csa
                                                                                                                    || {
                                                                                                                        glp_assert_(
                                                                                                                            b"csa != csa\0" as *const u8 as *const libc::c_char,
                                                                                                                            b"glpsol.c\0" as *const u8 as *const libc::c_char,
                                                                                                                            1482 as libc::c_int,
                                                                                                                        );
                                                                                                                        1 as libc::c_int != 0
                                                                                                                    }) as libc::c_int;
                                                                                                            }
                                                                                                            if ret != 0 as libc::c_int {
                                                                                                                glp_printf(
                                                                                                                    b"Unable to write problem solution\n\0" as *const u8
                                                                                                                        as *const libc::c_char,
                                                                                                                );
                                                                                                                ret = 1 as libc::c_int;
                                                                                                                current_block = 16495531352233468986;
                                                                                                            } else {
                                                                                                                current_block = 2606663910910355487;
                                                                                                            }
                                                                                                        } else {
                                                                                                            current_block = 2606663910910355487;
                                                                                                        }
                                                                                                        match current_block {
                                                                                                            16495531352233468986 => {}
                                                                                                            _ => {
                                                                                                                if !((*csa).out_res).is_null() {
                                                                                                                    if (*csa).solution == 1 as libc::c_int {
                                                                                                                        ret = glp_write_sol((*csa).prob, (*csa).out_res);
                                                                                                                    } else if (*csa).solution == 2 as libc::c_int {
                                                                                                                        ret = glp_write_ipt((*csa).prob, (*csa).out_res);
                                                                                                                    } else if (*csa).solution == 3 as libc::c_int {
                                                                                                                        ret = glp_write_mip((*csa).prob, (*csa).out_res);
                                                                                                                    } else {
                                                                                                                        (csa != csa
                                                                                                                            || {
                                                                                                                                glp_assert_(
                                                                                                                                    b"csa != csa\0" as *const u8 as *const libc::c_char,
                                                                                                                                    b"glpsol.c\0" as *const u8 as *const libc::c_char,
                                                                                                                                    1498 as libc::c_int,
                                                                                                                                );
                                                                                                                                1 as libc::c_int != 0
                                                                                                                            }) as libc::c_int;
                                                                                                                    }
                                                                                                                    if ret != 0 as libc::c_int {
                                                                                                                        glp_printf(
                                                                                                                            b"Unable to write problem solution\n\0" as *const u8
                                                                                                                                as *const libc::c_char,
                                                                                                                        );
                                                                                                                        ret = 1 as libc::c_int;
                                                                                                                        current_block = 16495531352233468986;
                                                                                                                    } else {
                                                                                                                        current_block = 964580764365397959;
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    current_block = 964580764365397959;
                                                                                                                }
                                                                                                                match current_block {
                                                                                                                    16495531352233468986 => {}
                                                                                                                    _ => {
                                                                                                                        if !((*csa).out_ranges).is_null() {
                                                                                                                            if (*csa).solution == 1 as libc::c_int {
                                                                                                                                if glp_get_status((*csa).prob) == 5 as libc::c_int {
                                                                                                                                    if glp_bf_exists((*csa).prob) != 0 {
                                                                                                                                        current_block = 5564518856185825108;
                                                                                                                                    } else {
                                                                                                                                        ret = glp_factorize((*csa).prob);
                                                                                                                                        if ret == 0 as libc::c_int {
                                                                                                                                            current_block = 5564518856185825108;
                                                                                                                                        } else {
                                                                                                                                            glp_printf(
                                                                                                                                                b"Cannot produce sensitivity analysis report due to error in basis factorization (glp_factorize returned %d); try --nopresol\n\0"
                                                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                                                ret,
                                                                                                                                            );
                                                                                                                                            current_block = 9602112577253180622;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    match current_block {
                                                                                                                                        9602112577253180622 => {}
                                                                                                                                        _ => {
                                                                                                                                            ret = glp_print_ranges(
                                                                                                                                                (*csa).prob,
                                                                                                                                                0 as libc::c_int,
                                                                                                                                                0 as *const libc::c_int,
                                                                                                                                                0 as libc::c_int,
                                                                                                                                                (*csa).out_ranges,
                                                                                                                                            );
                                                                                                                                            if ret != 0 as libc::c_int {
                                                                                                                                                glp_printf(
                                                                                                                                                    b"Unable to write sensitivity analysis report\n\0"
                                                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                                                );
                                                                                                                                                ret = 1 as libc::c_int;
                                                                                                                                                current_block = 16495531352233468986;
                                                                                                                                            } else {
                                                                                                                                                current_block = 9602112577253180622;
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    glp_printf(
                                                                                                                                        b"Cannot produce sensitivity analysis report for non-optimal basic solution\n\0"
                                                                                                                                            as *const u8 as *const libc::c_char,
                                                                                                                                    );
                                                                                                                                    current_block = 9602112577253180622;
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                glp_printf(
                                                                                                                                    b"Cannot produce sensitivity analysis report for interior-point or MIP solution\n\0"
                                                                                                                                        as *const u8 as *const libc::c_char,
                                                                                                                                );
                                                                                                                                current_block = 9602112577253180622;
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            current_block = 9602112577253180622;
                                                                                                                        }
                                                                                                                        match current_block {
                                                                                                                            16495531352233468986 => {}
                                                                                                                            _ => {
                                                                                                                                ret = 0 as libc::c_int;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !((*csa).prob).is_null() {
        glp_delete_prob((*csa).prob);
    }
    if !((*csa).tran).is_null() {
        glp_mpl_free_wksp((*csa).tran);
    }
    if !((*csa).graph).is_null() {
        glp_delete_graph((*csa).graph);
    }
    if !((*csa).log_file).is_null() {
        glp_close_tee();
    }
    let mut count: libc::c_int = 0;
    let mut total: size_t = 0;
    glp_mem_usage(&mut count, 0 as *mut libc::c_int, &mut total, 0 as *mut size_t);
    if count != 0 as libc::c_int {
        (glp_error_(
            b"glpsol.c\0" as *const u8 as *const libc::c_char,
            1569 as libc::c_int,
        ))
            .unwrap()(
            b"Error: %d memory block(s) were lost\n\0" as *const u8
                as *const libc::c_char,
            count,
        );
    }
    (total == 0 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"total == 0\0" as *const u8 as *const libc::c_char,
                b"glpsol.c\0" as *const u8 as *const libc::c_char,
                1570 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    glp_free_env();
    return ret;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
