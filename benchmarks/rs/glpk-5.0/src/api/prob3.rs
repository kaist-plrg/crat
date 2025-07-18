use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _glp_avl_create_tree(
        fcmp: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
    ) -> *mut AVL;
    fn _glp_avl_strcmp(
        info: *mut libc::c_void,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut libc::c_void);
    fn _glp_avl_find_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_delete_tree(tree: *mut AVL);
    fn _glp_avl_get_node_link(node: *mut AVLNODE) -> *mut libc::c_void;
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
pub unsafe extern "C" fn glp_create_index(mut lp: *mut glp_prob) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if ((*lp).r_tree).is_null() {
        (*lp)
            .r_tree = _glp_avl_create_tree(
            Some(
                _glp_avl_strcmp
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        i = 1 as libc::c_int;
        while i <= (*lp).m {
            row = *((*lp).row).offset(i as isize);
            (((*row).node).is_null()
                || {
                    glp_assert_(
                        b"row->node == NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob3.c\0" as *const u8 as *const libc::c_char,
                        53 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if !((*row).name).is_null() {
                (*row)
                    .node = _glp_avl_insert_node(
                    (*lp).r_tree,
                    (*row).name as *const libc::c_void,
                );
                _glp_avl_set_node_link((*row).node, row as *mut libc::c_void);
            }
            i += 1;
            i;
        }
    }
    if ((*lp).c_tree).is_null() {
        (*lp)
            .c_tree = _glp_avl_create_tree(
            Some(
                _glp_avl_strcmp
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        j = 1 as libc::c_int;
        while j <= (*lp).n {
            col = *((*lp).col).offset(j as isize);
            (((*col).node).is_null()
                || {
                    glp_assert_(
                        b"col->node == NULL\0" as *const u8 as *const libc::c_char,
                        b"api/prob3.c\0" as *const u8 as *const libc::c_char,
                        65 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if !((*col).name).is_null() {
                (*col)
                    .node = _glp_avl_insert_node(
                    (*lp).c_tree,
                    (*col).name as *const libc::c_void,
                );
                _glp_avl_set_node_link((*col).node, col as *mut libc::c_void);
            }
            j += 1;
            j;
        }
    }
}
pub unsafe extern "C" fn glp_find_row(
    mut lp: *mut glp_prob,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut i: libc::c_int = 0 as libc::c_int;
    if ((*lp).r_tree).is_null() {
        (glp_error_(
            b"api/prob3.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
        ))
            .unwrap()(
            b"glp_find_row: row name index does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || strlen(name) > 255 as libc::c_int as libc::c_ulong)
    {
        node = _glp_avl_find_node((*lp).r_tree, name as *const libc::c_void);
        if !node.is_null() {
            i = (*(_glp_avl_get_node_link(node) as *mut GLPROW)).i;
        }
    }
    return i;
}
pub unsafe extern "C" fn glp_find_col(
    mut lp: *mut glp_prob,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut j: libc::c_int = 0 as libc::c_int;
    if ((*lp).c_tree).is_null() {
        (glp_error_(
            b"api/prob3.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
        ))
            .unwrap()(
            b"glp_find_col: column name index does not exist\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || strlen(name) > 255 as libc::c_int as libc::c_ulong)
    {
        node = _glp_avl_find_node((*lp).c_tree, name as *const libc::c_void);
        if !node.is_null() {
            j = (*(_glp_avl_get_node_link(node) as *mut GLPCOL)).j;
        }
    }
    return j;
}
pub unsafe extern "C" fn glp_delete_index(mut lp: *mut glp_prob) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !((*lp).r_tree).is_null() {
        i = 1 as libc::c_int;
        while i <= (*lp).m {
            let ref mut fresh0 = (**((*lp).row).offset(i as isize)).node;
            *fresh0 = 0 as *mut AVLNODE;
            i += 1;
            i;
        }
        _glp_avl_delete_tree((*lp).r_tree);
        (*lp).r_tree = 0 as *mut AVL;
    }
    if !((*lp).c_tree).is_null() {
        j = 1 as libc::c_int;
        while j <= (*lp).n {
            let ref mut fresh1 = (**((*lp).col).offset(j as isize)).node;
            *fresh1 = 0 as *mut AVLNODE;
            j += 1;
            j;
        }
        _glp_avl_delete_tree((*lp).c_tree);
        (*lp).c_tree = 0 as *mut AVL;
    }
}
