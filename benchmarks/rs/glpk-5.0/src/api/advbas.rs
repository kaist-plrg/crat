use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_set_row_stat(P: *mut glp_prob, i: libc::c_int, stat: libc::c_int);
    fn glp_set_col_stat(P: *mut glp_prob, j: libc::c_int, stat: libc::c_int);
    fn glp_std_basis(P: *mut glp_prob);
    fn _glp_triang(
        m: libc::c_int,
        n: libc::c_int,
        mat_0: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_double,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
        tol: libc::c_double,
        rn: *mut libc::c_int,
        cn: *mut libc::c_int,
    ) -> libc::c_int;
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
unsafe extern "C" fn mat(
    mut info: *mut libc::c_void,
    mut k: libc::c_int,
    mut ind: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut P: *mut glp_prob = info as *mut glp_prob;
    let mut m: libc::c_int = (*P).m;
    let mut n: libc::c_int = (*P).n;
    let mut row: *mut *mut GLPROW = (*P).row;
    let mut col: *mut *mut GLPCOL = (*P).col;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if k > 0 as libc::c_int {
        i = k;
        (1 as libc::c_int <= i && i <= m
            || {
                glp_assert_(
                    b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                    b"api/advbas.c\0" as *const u8 as *const libc::c_char,
                    59 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        len = 0 as libc::c_int;
        if (**row.offset(i as isize)).type_0 == 5 as libc::c_int {
            aij = (**row.offset(i as isize)).ptr;
            while !aij.is_null() {
                j = (*(*aij).col).j;
                if (**col.offset(j as isize)).type_0 != 5 as libc::c_int {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = j;
                    *val
                        .offset(
                            len as isize,
                        ) = (*(*aij).row).rii * (*aij).val * (*(*aij).col).sjj;
                }
                aij = (*aij).r_next;
            }
        }
    } else {
        j = -k;
        (1 as libc::c_int <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                    b"api/advbas.c\0" as *const u8 as *const libc::c_char,
                    75 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        len = 0 as libc::c_int;
        if (**col.offset(j as isize)).type_0 != 5 as libc::c_int {
            aij = (**col.offset(j as isize)).ptr;
            while !aij.is_null() {
                i = (*(*aij).row).i;
                if (**row.offset(i as isize)).type_0 == 5 as libc::c_int {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = i;
                    *val
                        .offset(
                            len as isize,
                        ) = (*(*aij).row).rii * (*aij).val * (*(*aij).col).sjj;
                }
                aij = (*aij).c_next;
            }
        }
    }
    return len;
}
pub unsafe extern "C" fn glp_adv_basis(mut P: *mut glp_prob, mut flags: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut min_mn: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut rn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cn: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut flag: *mut libc::c_char = 0 as *mut libc::c_char;
    if flags != 0 as libc::c_int {
        (glp_error_(
            b"api/advbas.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
        ))
            .unwrap()(
            b"glp_adv_basis: flags = %d; invalid flags\n\0" as *const u8
                as *const libc::c_char,
            flags,
        );
    }
    m = (*P).m;
    n = (*P).n;
    if m == 0 as libc::c_int || n == 0 as libc::c_int {
        glp_std_basis(P);
    } else {
        glp_printf(
            b"Constructing initial basis...\n\0" as *const u8 as *const libc::c_char,
        );
        min_mn = if m < n { m } else { n };
        rn = glp_alloc(
            1 as libc::c_int + min_mn,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        cn = glp_alloc(
            1 as libc::c_int + min_mn,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_int;
        flag = glp_alloc(
            1 as libc::c_int + m,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        i = 1 as libc::c_int;
        while i <= m {
            *flag.offset(i as isize) = 0 as libc::c_int as libc::c_char;
            glp_set_row_stat(P, i, 5 as libc::c_int);
            i += 1;
            i;
        }
        j = 1 as libc::c_int;
        while j <= n {
            glp_set_col_stat(P, j, 5 as libc::c_int);
            j += 1;
            j;
        }
        size = _glp_triang(
            m,
            n,
            Some(
                mat
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_double,
                    ) -> libc::c_int,
            ),
            P as *mut libc::c_void,
            0.001f64,
            rn,
            cn,
        );
        (0 as libc::c_int <= size && size <= min_mn
            || {
                glp_assert_(
                    b"0 <= size && size <= min_mn\0" as *const u8 as *const libc::c_char,
                    b"api/advbas.c\0" as *const u8 as *const libc::c_char,
                    123 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        k = 1 as libc::c_int;
        while k <= size {
            i = *rn.offset(k as isize);
            (1 as libc::c_int <= i && i <= m
                || {
                    glp_assert_(
                        b"1 <= i && i <= m\0" as *const u8 as *const libc::c_char,
                        b"api/advbas.c\0" as *const u8 as *const libc::c_char,
                        128 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            *flag.offset(i as isize) = 1 as libc::c_int as libc::c_char;
            j = *cn.offset(k as isize);
            (1 as libc::c_int <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const libc::c_char,
                        b"api/advbas.c\0" as *const u8 as *const libc::c_char,
                        131 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            glp_set_col_stat(P, j, 1 as libc::c_int);
            k += 1;
            k;
        }
        i = 1 as libc::c_int;
        while i <= m {
            if *flag.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                glp_set_row_stat(P, i, 1 as libc::c_int);
                if (**((*P).row).offset(i as isize)).type_0 != 5 as libc::c_int {
                    size += 1;
                    size;
                }
            }
            i += 1;
            i;
        }
        glp_printf(
            b"Size of triangular part is %d\n\0" as *const u8 as *const libc::c_char,
            size,
        );
        glp_free(rn as *mut libc::c_void);
        glp_free(cn as *mut libc::c_void);
        glp_free(flag as *mut libc::c_void);
    };
}
