use ::libc;
extern "C" {
    pub type DMP;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type glp_tree;
    fn _glp_cfg_delete_graph(G: *mut CFG);
    fn _glp_cfg_build_graph(P: *mut libc::c_void) -> *mut CFG;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_cfg {
    pub n: libc::c_int,
    pub pos: *mut libc::c_int,
    pub neg: *mut libc::c_int,
    pub pool: *mut DMP,
    pub nv_max: libc::c_int,
    pub nv: libc::c_int,
    pub ref_0: *mut libc::c_int,
    pub vptr: *mut *mut CFGVLE,
    pub cptr: *mut *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGCLE {
    pub vptr: *mut CFGVLE,
    pub next: *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGVLE {
    pub v: libc::c_int,
    pub next: *mut CFGVLE,
}
pub type CFG = glp_cfg;
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
pub unsafe extern "C" fn glp_cfg_init(mut P: *mut glp_prob) -> *mut glp_cfg {
    let mut G: *mut glp_cfg = 0 as *mut glp_cfg;
    let mut j: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    glp_printf(
        b"Constructing conflict graph...\n\0" as *const u8 as *const libc::c_char,
    );
    G = _glp_cfg_build_graph(P as *mut libc::c_void);
    n2 = 0 as libc::c_int;
    n1 = n2;
    j = 1 as libc::c_int;
    while j <= (*P).n {
        if *((*G).pos).offset(j as isize) != 0 {
            n1 += 1;
            n1;
        }
        if *((*G).neg).offset(j as isize) != 0 {
            n2 += 1;
            n2;
        }
        j += 1;
        j;
    }
    if n1 == 0 as libc::c_int && n2 == 0 as libc::c_int {
        glp_printf(b"No conflicts found\n\0" as *const u8 as *const libc::c_char);
        _glp_cfg_delete_graph(G);
        G = 0 as *mut glp_cfg;
    } else {
        glp_printf(
            b"Conflict graph has %d + %d = %d vertices\n\0" as *const u8
                as *const libc::c_char,
            n1,
            n2,
            (*G).nv,
        );
    }
    return G;
}
pub unsafe extern "C" fn glp_cfg_free(mut G: *mut glp_cfg) {
    (!G.is_null()
        || {
            glp_assert_(
                b"G != NULL\0" as *const u8 as *const libc::c_char,
                b"intopt/cfg2.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_cfg_delete_graph(G);
}
