use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type DMP;
    pub type glp_file;
    pub type BFD;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_rng_init_rand(rand: *mut RNG, seed: libc::c_int);
    fn _glp_mpl_get_row_name(mpl: *mut MPL, i: libc::c_int) -> *mut libc::c_char;
    fn _glp_mpl_initialize() -> *mut MPL;
    fn _glp_mpl_read_model(
        mpl: *mut MPL,
        file: *mut libc::c_char,
        skip_data: libc::c_int,
    ) -> libc::c_int;
    fn _glp_mpl_read_data(mpl: *mut MPL, file: *mut libc::c_char) -> libc::c_int;
    fn _glp_mpl_generate(mpl: *mut MPL, file: *mut libc::c_char) -> libc::c_int;
    fn _glp_mpl_get_prob_name(mpl: *mut MPL) -> *mut libc::c_char;
    fn _glp_mpl_get_num_rows(mpl: *mut MPL) -> libc::c_int;
    fn _glp_mpl_get_num_cols(mpl: *mut MPL) -> libc::c_int;
    fn _glp_mpl_get_row_kind(mpl: *mut MPL, i: libc::c_int) -> libc::c_int;
    fn _glp_mpl_postsolve(mpl: *mut MPL) -> libc::c_int;
    fn _glp_mpl_put_col_soln(
        mpl: *mut MPL,
        j: libc::c_int,
        stat: libc::c_int,
        prim: libc::c_double,
        dual: libc::c_double,
    );
    fn _glp_mpl_put_row_soln(
        mpl: *mut MPL,
        i: libc::c_int,
        stat: libc::c_int,
        prim: libc::c_double,
        dual: libc::c_double,
    );
    fn _glp_mpl_has_solve_stmt(mpl: *mut MPL) -> libc::c_int;
    fn _glp_mpl_get_col_bnds(
        mpl: *mut MPL,
        j: libc::c_int,
        lb: *mut libc::c_double,
        ub: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_mpl_get_col_kind(mpl: *mut MPL, j: libc::c_int) -> libc::c_int;
    fn _glp_mpl_get_col_name(mpl: *mut MPL, j: libc::c_int) -> *mut libc::c_char;
    fn _glp_mpl_get_row_c0(mpl: *mut MPL, i: libc::c_int) -> libc::c_double;
    fn _glp_mpl_get_row_bnds(
        mpl: *mut MPL,
        i: libc::c_int,
        lb: *mut libc::c_double,
        ub: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_mpl_get_mat_row(
        mpl: *mut MPL,
        i: libc::c_int,
        ndx: *mut libc::c_int,
        val: *mut libc::c_double,
    ) -> libc::c_int;
    fn _glp_mpl_terminate(mpl: *mut MPL);
    fn glp_set_prob_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_name(P: *mut glp_prob, name: *const libc::c_char);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: libc::c_int);
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
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: libc::c_int,
        len: libc::c_int,
        ind: *const libc::c_int,
        val: *const libc::c_double,
    );
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_get_num_rows(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_row_stat(P: *mut glp_prob, i: libc::c_int) -> libc::c_int;
    fn glp_get_row_prim(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_row_dual(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_col_stat(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
    fn glp_get_col_prim(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_dual(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_ipt_row_prim(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_ipt_row_dual(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_ipt_col_prim(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_ipt_col_dual(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_set_col_kind(P: *mut glp_prob, j: libc::c_int, kind: libc::c_int);
    fn glp_mip_row_val(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_mip_col_val(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [libc::c_int; 56],
    pub fptr: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tran {
    pub line: libc::c_int,
    pub c: libc::c_int,
    pub token: libc::c_int,
    pub imlen: libc::c_int,
    pub image: *mut libc::c_char,
    pub value: libc::c_double,
    pub b_token: libc::c_int,
    pub b_imlen: libc::c_int,
    pub b_image: *mut libc::c_char,
    pub b_value: libc::c_double,
    pub f_dots: libc::c_int,
    pub f_scan: libc::c_int,
    pub f_token: libc::c_int,
    pub f_imlen: libc::c_int,
    pub f_image: *mut libc::c_char,
    pub f_value: libc::c_double,
    pub context: *mut libc::c_char,
    pub c_ptr: libc::c_int,
    pub flag_d: libc::c_int,
    pub pool: *mut DMP,
    pub tree: *mut AVL,
    pub model: *mut STATEMENT,
    pub flag_x: libc::c_int,
    pub as_within: libc::c_int,
    pub as_in: libc::c_int,
    pub as_binary: libc::c_int,
    pub flag_s: libc::c_int,
    pub strings: *mut DMP,
    pub symbols: *mut DMP,
    pub tuples: *mut DMP,
    pub arrays: *mut DMP,
    pub members: *mut DMP,
    pub elemvars: *mut DMP,
    pub formulae: *mut DMP,
    pub elemcons: *mut DMP,
    pub a_list: *mut ARRAY,
    pub sym_buf: *mut libc::c_char,
    pub tup_buf: *mut libc::c_char,
    pub rand: *mut RNG,
    pub flag_p: libc::c_int,
    pub stmt: *mut STATEMENT,
    pub dca: *mut TABDCA,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub row: *mut *mut ELEMCON,
    pub col: *mut *mut ELEMVAR,
    pub in_fp: *mut glp_file,
    pub in_file: *mut libc::c_char,
    pub out_fp: *mut glp_file,
    pub out_file: *mut libc::c_char,
    pub prt_fp: *mut glp_file,
    pub prt_file: *mut libc::c_char,
    pub jump: jmp_buf,
    pub phase: libc::c_int,
    pub mod_file: *mut libc::c_char,
    pub mpl_buf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMVAR {
    pub j: libc::c_int,
    pub var: *mut VARIABLE,
    pub memb: *mut MEMBER,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub temp: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEMBER {
    pub tuple: *mut TUPLE,
    pub next: *mut MEMBER,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VALUE {
    pub none: *mut libc::c_void,
    pub num: libc::c_double,
    pub sym: *mut SYMBOL,
    pub bit: libc::c_int,
    pub tuple: *mut TUPLE,
    pub set: *mut ELEMSET,
    pub var: *mut ELEMVAR,
    pub form: *mut FORMULA,
    pub con: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMCON {
    pub i: libc::c_int,
    pub con: *mut CONSTRAINT,
    pub memb: *mut MEMBER,
    pub form: *mut FORMULA,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMULA {
    pub coef: libc::c_double,
    pub var: *mut ELEMVAR,
    pub next: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONSTRAINT {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub code: *mut CODE,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY {
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub size: libc::c_int,
    pub head: *mut MEMBER,
    pub tail: *mut MEMBER,
    pub tree: *mut AVL,
    pub prev: *mut ARRAY,
    pub next: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODE {
    pub op: libc::c_int,
    pub arg: OPERANDS,
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub up: *mut CODE,
    pub vflag: libc::c_int,
    pub valid: libc::c_int,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OPERANDS {
    pub num: libc::c_double,
    pub str_0: *mut libc::c_char,
    pub index: C2RustUnnamed_5,
    pub par: C2RustUnnamed_4,
    pub set: C2RustUnnamed_3,
    pub var: C2RustUnnamed_2,
    pub con: C2RustUnnamed_1,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_0,
    pub loop_0: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub domain: *mut DOMAIN1,
    pub x: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN1 {
    pub list: *mut DOMAIN_BLOCK,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_BLOCK {
    pub list: *mut DOMAIN_SLOT,
    pub code: *mut CODE,
    pub backup: *mut TUPLE,
    pub next: *mut DOMAIN_BLOCK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUPLE {
    pub sym: *mut SYMBOL,
    pub next: *mut TUPLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub num: libc::c_double,
    pub str_0: *mut STRING,
}
pub type STRING = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_SLOT {
    pub name: *mut libc::c_char,
    pub code: *mut CODE,
    pub value: *mut SYMBOL,
    pub list: *mut CODE,
    pub next: *mut DOMAIN_SLOT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub x: *mut CODE,
    pub y: *mut CODE,
    pub z: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARG_LIST {
    pub x: *mut CODE,
    pub next: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub var: *mut VARIABLE,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VARIABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub set: *mut SET,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SET {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub dimen: libc::c_int,
    pub within: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub gadget: *mut GADGET,
    pub data: libc::c_int,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GADGET {
    pub set: *mut SET,
    pub ind: [libc::c_int; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WITHIN {
    pub code: *mut CODE,
    pub next: *mut WITHIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub par: *mut PARAMETER,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARAMETER {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub cond: *mut CONDITION,
    pub in_0: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub data: libc::c_int,
    pub defval: *mut SYMBOL,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONDITION {
    pub rho: libc::c_int,
    pub code: *mut CODE,
    pub next: *mut CONDITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub slot: *mut DOMAIN_SLOT,
    pub next: *mut CODE,
}
pub type ELEMSET = ARRAY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: libc::c_int,
    pub link: *mut libc::c_void,
    pub na: libc::c_int,
    pub arg: *mut *mut libc::c_char,
    pub nf: libc::c_int,
    pub name: *mut *mut libc::c_char,
    pub type_0: *mut libc::c_int,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATEMENT {
    pub line: libc::c_int,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_6,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub tab: *mut TABLE,
    pub slv: *mut libc::c_void,
    pub chk: *mut CHECK,
    pub dpy: *mut DISPLAY,
    pub prt: *mut PRINTF,
    pub fur: *mut FOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FOR {
    pub domain: *mut DOMAIN1,
    pub list: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF {
    pub domain: *mut DOMAIN1,
    pub fmt: *mut CODE,
    pub list: *mut PRINTF1,
    pub fname: *mut CODE,
    pub app: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF1 {
    pub code: *mut CODE,
    pub next: *mut PRINTF1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY {
    pub domain: *mut DOMAIN1,
    pub list: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY1 {
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_7,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub slot: *mut DOMAIN_SLOT,
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHECK {
    pub domain: *mut DOMAIN1,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub arg: *mut TABARG,
    pub u: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: C2RustUnnamed_10,
    pub out: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub domain: *mut DOMAIN1,
    pub list: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABOUT {
    pub code: *mut CODE,
    pub name: *mut libc::c_char,
    pub next: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub set: *mut SET,
    pub fld: *mut TABFLD,
    pub list: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABIN {
    pub par: *mut PARAMETER,
    pub name: *mut libc::c_char,
    pub next: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABFLD {
    pub name: *mut libc::c_char,
    pub next: *mut TABFLD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABARG {
    pub code: *mut CODE,
    pub next: *mut TABARG,
}
pub type MPL = glp_tran;
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
pub unsafe extern "C" fn glp_mpl_alloc_wksp() -> *mut glp_tran {
    let mut tran: *mut glp_tran = 0 as *mut glp_tran;
    tran = _glp_mpl_initialize();
    return tran;
}
pub unsafe extern "C" fn glp_mpl_init_rand(
    mut tran: *mut glp_tran,
    mut seed: libc::c_int,
) {
    if (*tran).phase != 0 as libc::c_int {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_init_rand: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    _glp_rng_init_rand((*tran).rand, seed);
}
pub unsafe extern "C" fn glp_mpl_read_model(
    mut tran: *mut glp_tran,
    mut fname: *const libc::c_char,
    mut skip: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*tran).phase != 0 as libc::c_int {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_read_model: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    ret = _glp_mpl_read_model(tran, fname as *mut libc::c_char, skip);
    if ret == 1 as libc::c_int || ret == 2 as libc::c_int {
        ret = 0 as libc::c_int;
    } else if ret == 4 as libc::c_int {
        ret = 1 as libc::c_int;
    } else {
        (ret != ret
            || {
                glp_assert_(
                    b"ret != ret\0" as *const u8 as *const libc::c_char,
                    b"api/mpl.c\0" as *const u8 as *const libc::c_char,
                    51 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn glp_mpl_read_data(
    mut tran: *mut glp_tran,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if !((*tran).phase == 1 as libc::c_int || (*tran).phase == 2 as libc::c_int) {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_read_data: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    ret = _glp_mpl_read_data(tran, fname as *mut libc::c_char);
    if ret == 2 as libc::c_int {
        ret = 0 as libc::c_int;
    } else if ret == 4 as libc::c_int {
        ret = 1 as libc::c_int;
    } else {
        (ret != ret
            || {
                glp_assert_(
                    b"ret != ret\0" as *const u8 as *const libc::c_char,
                    b"api/mpl.c\0" as *const u8 as *const libc::c_char,
                    66 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn glp_mpl_generate(
    mut tran: *mut glp_tran,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if !((*tran).phase == 1 as libc::c_int || (*tran).phase == 2 as libc::c_int) {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_generate: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    ret = _glp_mpl_generate(tran, fname as *mut libc::c_char);
    if ret == 3 as libc::c_int {
        ret = 0 as libc::c_int;
    } else if ret == 4 as libc::c_int {
        ret = 1 as libc::c_int;
    }
    return ret;
}
pub unsafe extern "C" fn glp_mpl_build_prob(
    mut tran: *mut glp_tran,
    mut prob: *mut glp_prob,
) {
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut kind: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ind: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    if (*tran).phase != 3 as libc::c_int {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_build_prob: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    glp_erase_prob(prob);
    glp_set_prob_name(prob, _glp_mpl_get_prob_name(tran));
    m = _glp_mpl_get_num_rows(tran);
    if m > 0 as libc::c_int {
        glp_add_rows(prob, m);
    }
    i = 1 as libc::c_int;
    while i <= m {
        glp_set_row_name(prob, i, _glp_mpl_get_row_name(tran, i));
        type_0 = _glp_mpl_get_row_bnds(tran, i, &mut lb, &mut ub);
        match type_0 {
            401 => {
                type_0 = 1 as libc::c_int;
            }
            402 => {
                type_0 = 2 as libc::c_int;
            }
            403 => {
                type_0 = 3 as libc::c_int;
            }
            404 => {
                type_0 = 4 as libc::c_int;
            }
            405 => {
                type_0 = 5 as libc::c_int;
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const libc::c_char,
                            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
                            108 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        if type_0 == 4 as libc::c_int && fabs(lb - ub) < 1e-9f64 * (1.0f64 + fabs(lb)) {
            type_0 = 5 as libc::c_int;
            if fabs(lb) <= fabs(ub) {
                ub = lb;
            } else {
                lb = ub;
            }
        }
        glp_set_row_bnds(prob, i, type_0, lb, ub);
        if _glp_mpl_get_row_c0(tran, i) != 0.0f64 {
            glp_printf(
                b"glp_mpl_build_prob: row %s; constant term %.12g ignored\n\0"
                    as *const u8 as *const libc::c_char,
                _glp_mpl_get_row_name(tran, i),
                _glp_mpl_get_row_c0(tran, i),
            );
        }
        i += 1;
        i;
    }
    n = _glp_mpl_get_num_cols(tran);
    if n > 0 as libc::c_int {
        glp_add_cols(prob, n);
    }
    j = 1 as libc::c_int;
    while j <= n {
        glp_set_col_name(prob, j, _glp_mpl_get_col_name(tran, j));
        kind = _glp_mpl_get_col_kind(tran, j);
        match kind {
            421 => {}
            422 | 423 => {
                glp_set_col_kind(prob, j, 2 as libc::c_int);
            }
            _ => {
                (kind != kind
                    || {
                        glp_assert_(
                            b"kind != kind\0" as *const u8 as *const libc::c_char,
                            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
                            138 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        type_0 = _glp_mpl_get_col_bnds(tran, j, &mut lb, &mut ub);
        match type_0 {
            401 => {
                type_0 = 1 as libc::c_int;
            }
            402 => {
                type_0 = 2 as libc::c_int;
            }
            403 => {
                type_0 = 3 as libc::c_int;
            }
            404 => {
                type_0 = 4 as libc::c_int;
            }
            405 => {
                type_0 = 5 as libc::c_int;
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const libc::c_char,
                            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
                            148 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        if kind == 423 as libc::c_int {
            if type_0 == 1 as libc::c_int || type_0 == 3 as libc::c_int || lb < 0.0f64 {
                lb = 0.0f64;
            }
            if type_0 == 1 as libc::c_int || type_0 == 2 as libc::c_int || ub > 1.0f64 {
                ub = 1.0f64;
            }
            type_0 = 4 as libc::c_int;
        }
        if type_0 == 4 as libc::c_int && fabs(lb - ub) < 1e-9f64 * (1.0f64 + fabs(lb)) {
            type_0 = 5 as libc::c_int;
            if fabs(lb) <= fabs(ub) {
                ub = lb;
            } else {
                lb = ub;
            }
        }
        glp_set_col_bnds(prob, j, type_0, lb, ub);
        j += 1;
        j;
    }
    ind = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    val = glp_alloc(
        1 as libc::c_int + n,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i <= m {
        len = _glp_mpl_get_mat_row(tran, i, ind, val);
        glp_set_mat_row(
            prob,
            i,
            len,
            ind as *const libc::c_int,
            val as *const libc::c_double,
        );
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i <= m {
        kind = _glp_mpl_get_row_kind(tran, i);
        if kind == 412 as libc::c_int || kind == 413 as libc::c_int {
            glp_set_obj_name(prob, _glp_mpl_get_row_name(tran, i));
            glp_set_obj_dir(
                prob,
                if kind == 412 as libc::c_int {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                },
            );
            glp_set_obj_coef(prob, 0 as libc::c_int, _glp_mpl_get_row_c0(tran, i));
            len = _glp_mpl_get_mat_row(tran, i, ind, val);
            t = 1 as libc::c_int;
            while t <= len {
                glp_set_obj_coef(prob, *ind.offset(t as isize), *val.offset(t as isize));
                t += 1;
                t;
            }
            break;
        } else {
            i += 1;
            i;
        }
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
pub unsafe extern "C" fn glp_mpl_postsolve(
    mut tran: *mut glp_tran,
    mut prob: *mut glp_prob,
    mut sol: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut prim: libc::c_double = 0.;
    let mut dual: libc::c_double = 0.;
    if !((*tran).phase == 3 as libc::c_int && (*tran).flag_p == 0) {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_postsolve: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(sol == 1 as libc::c_int || sol == 2 as libc::c_int || sol == 3 as libc::c_int) {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_postsolve: sol = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            sol,
        );
    }
    m = _glp_mpl_get_num_rows(tran);
    n = _glp_mpl_get_num_cols(tran);
    if !(m == glp_get_num_rows(prob) && n == glp_get_num_cols(prob)) {
        (glp_error_(
            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mpl_postsolve: wrong problem object\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _glp_mpl_has_solve_stmt(tran) == 0 {
        ret = 0 as libc::c_int;
    } else {
        i = 1 as libc::c_int;
        while i <= m {
            if sol == 1 as libc::c_int {
                stat = glp_get_row_stat(prob, i);
                prim = glp_get_row_prim(prob, i);
                dual = glp_get_row_dual(prob, i);
            } else if sol == 2 as libc::c_int {
                stat = 0 as libc::c_int;
                prim = glp_ipt_row_prim(prob, i);
                dual = glp_ipt_row_dual(prob, i);
            } else if sol == 3 as libc::c_int {
                stat = 0 as libc::c_int;
                prim = glp_mip_row_val(prob, i);
                dual = 0.0f64;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
                            226 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if fabs(prim) < 1e-9f64 {
                prim = 0.0f64;
            }
            if fabs(dual) < 1e-9f64 {
                dual = 0.0f64;
            }
            _glp_mpl_put_row_soln(tran, i, stat, prim, dual);
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            if sol == 1 as libc::c_int {
                stat = glp_get_col_stat(prob, j);
                prim = glp_get_col_prim(prob, j);
                dual = glp_get_col_dual(prob, j);
            } else if sol == 2 as libc::c_int {
                stat = 0 as libc::c_int;
                prim = glp_ipt_col_prim(prob, j);
                dual = glp_ipt_col_dual(prob, j);
            } else if sol == 3 as libc::c_int {
                stat = 0 as libc::c_int;
                prim = glp_mip_col_val(prob, j);
                dual = 0.0f64;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const libc::c_char,
                            b"api/mpl.c\0" as *const u8 as *const libc::c_char,
                            248 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            if fabs(prim) < 1e-9f64 {
                prim = 0.0f64;
            }
            if fabs(dual) < 1e-9f64 {
                dual = 0.0f64;
            }
            _glp_mpl_put_col_soln(tran, j, stat, prim, dual);
            j += 1;
            j;
        }
        ret = _glp_mpl_postsolve(tran);
        if ret == 3 as libc::c_int {
            ret = 0 as libc::c_int;
        } else if ret == 4 as libc::c_int {
            ret = 1 as libc::c_int;
        }
    }
    return ret;
}
pub unsafe extern "C" fn glp_mpl_free_wksp(mut tran: *mut glp_tran) {
    _glp_mpl_terminate(tran);
}
