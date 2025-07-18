use ::libc;
extern "C" {
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn glp_get_row_name(P: *mut glp_prob, i: libc::c_int) -> *const libc::c_char;
    fn glp_get_col_name(P: *mut glp_prob, j: libc::c_int) -> *const libc::c_char;
    fn glp_get_row_lb(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: libc::c_int) -> libc::c_double;
    fn glp_get_col_lb(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_bf_exists(P: *mut glp_prob) -> libc::c_int;
    fn glp_analyze_bound(
        P: *mut glp_prob,
        k: libc::c_int,
        value1: *mut libc::c_double,
        var1: *mut libc::c_int,
        value2: *mut libc::c_double,
        var2: *mut libc::c_int,
    );
    fn glp_analyze_coef(
        P: *mut glp_prob,
        k: libc::c_int,
        coef1: *mut libc::c_double,
        var1: *mut libc::c_int,
        value1: *mut libc::c_double,
        coef2: *mut libc::c_double,
        var2: *mut libc::c_int,
        value2: *mut libc::c_double,
    );
    fn glp_version() -> *const libc::c_char;
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
unsafe extern "C" fn format(
    mut buf: *mut libc::c_char,
    mut x: libc::c_double,
) -> *mut libc::c_char {
    if x == -1.7976931348623157e+308f64 {
        strcpy(buf, b"         -Inf\0" as *const u8 as *const libc::c_char);
    } else if x == 1.7976931348623157e+308f64 {
        strcpy(buf, b"         +Inf\0" as *const u8 as *const libc::c_char);
    } else if fabs(x) <= 999999.99998f64 {
        sprintf(buf, b"%13.5f\0" as *const u8 as *const libc::c_char, x);
        if strcmp(
            buf as *const libc::c_char,
            b"      0.00000\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                buf as *const libc::c_char,
                b"     -0.00000\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            strcpy(buf, b"       .     \0" as *const u8 as *const libc::c_char);
        } else if memcmp(
            buf as *const libc::c_void,
            b"      0.\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            memcpy(
                buf as *mut libc::c_void,
                b"       .\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
        } else if memcmp(
            buf as *const libc::c_void,
            b"     -0.\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            memcpy(
                buf as *mut libc::c_void,
                b"      -.\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
        }
    } else {
        sprintf(buf, b"%13.6g\0" as *const u8 as *const libc::c_char, x);
    }
    return buf;
}
pub unsafe extern "C" fn glp_print_ranges(
    mut P: *mut glp_prob,
    mut len: libc::c_int,
    mut list: *const libc::c_int,
    mut flags: libc::c_int,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut numb: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut stat: libc::c_int = 0;
    let mut var1: libc::c_int = 0;
    let mut var2: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut page: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut slack: libc::c_double = 0.;
    let mut coef: libc::c_double = 0.;
    let mut prim: libc::c_double = 0.;
    let mut dual: libc::c_double = 0.;
    let mut value1: libc::c_double = 0.;
    let mut value2: libc::c_double = 0.;
    let mut coef1: libc::c_double = 0.;
    let mut coef2: libc::c_double = 0.;
    let mut obj1: libc::c_double = 0.;
    let mut obj2: libc::c_double = 0.;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut limit: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 14] = [0; 14];
    m = (*P).m;
    n = (*P).n;
    if len < 0 as libc::c_int {
        (glp_error_(
            b"api/prrngs.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
        ))
            .unwrap()(
            b"glp_print_ranges: len = %d; invalid list length\n\0" as *const u8
                as *const libc::c_char,
            len,
        );
    }
    if len > 0 as libc::c_int {
        if list.is_null() {
            (glp_error_(
                b"api/prrngs.c\0" as *const u8 as *const libc::c_char,
                74 as libc::c_int,
            ))
                .unwrap()(
                b"glp_print_ranges: list = %p: invalid parameter\n\0" as *const u8
                    as *const libc::c_char,
                list,
            );
        }
        t = 1 as libc::c_int;
        while t <= len {
            k = *list.offset(t as isize);
            if !(1 as libc::c_int <= k && k <= m + n) {
                (glp_error_(
                    b"api/prrngs.c\0" as *const u8 as *const libc::c_char,
                    79 as libc::c_int,
                ))
                    .unwrap()(
                    b"glp_print_ranges: list[%d] = %d; row/column number out of range\n\0"
                        as *const u8 as *const libc::c_char,
                    t,
                    k,
                );
            }
            t += 1;
            t;
        }
    }
    if flags != 0 as libc::c_int {
        (glp_error_(
            b"api/prrngs.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
        ))
            .unwrap()(
            b"glp_print_ranges: flags = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            flags,
        );
    }
    if fname.is_null() {
        (glp_error_(
            b"api/prrngs.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
        ))
            .unwrap()(
            b"glp_print_ranges: fname = %p; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
    }
    if glp_get_status(P) != 5 as libc::c_int {
        glp_printf(
            b"glp_print_ranges: optimal basic solution required\n\0" as *const u8
                as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
    } else if glp_bf_exists(P) == 0 {
        glp_printf(
            b"glp_print_ranges: basis factorization required\n\0" as *const u8
                as *const libc::c_char,
        );
        ret = 2 as libc::c_int;
    } else {
        glp_printf(
            b"Write sensitivity analysis report to '%s'...\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        fp = _glp_open(fname, b"w\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            glp_printf(
                b"Unable to create '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                fname,
                _glp_get_err_msg(),
            );
            ret = 3 as libc::c_int;
        } else {
            count = 0 as libc::c_int;
            page = count;
            pass = 1 as libc::c_int;
            while pass <= 2 as libc::c_int {
                t = 1 as libc::c_int;
                while t <= (if len == 0 as libc::c_int { m + n } else { len }) {
                    if t == 1 as libc::c_int {
                        count = 0 as libc::c_int;
                    }
                    k = if len == 0 as libc::c_int {
                        t
                    } else {
                        *list.offset(t as isize)
                    };
                    if !(pass == 1 as libc::c_int && k > m
                        || pass == 2 as libc::c_int && k <= m)
                    {
                        if count == 0 as libc::c_int {
                            page += 1;
                            _glp_format(
                                fp,
                                b"GLPK %-4s - SENSITIVITY ANALYSIS REPORT%73sPage%4d\n\0"
                                    as *const u8 as *const libc::c_char,
                                glp_version(),
                                b"\0" as *const u8 as *const libc::c_char,
                                page,
                            );
                            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                            _glp_format(
                                fp,
                                b"%-12s%s\n\0" as *const u8 as *const libc::c_char,
                                b"Problem:\0" as *const u8 as *const libc::c_char,
                                if ((*P).name).is_null() {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    (*P).name as *const libc::c_char
                                },
                            );
                            _glp_format(
                                fp,
                                b"%-12s%s%s%.10g (%s)\n\0" as *const u8
                                    as *const libc::c_char,
                                b"Objective:\0" as *const u8 as *const libc::c_char,
                                if ((*P).obj).is_null() {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    (*P).obj as *const libc::c_char
                                },
                                if ((*P).obj).is_null() {
                                    b"\0" as *const u8 as *const libc::c_char
                                } else {
                                    b" = \0" as *const u8 as *const libc::c_char
                                },
                                (*P).obj_val,
                                if (*P).dir == 1 as libc::c_int {
                                    b"MINimum\0" as *const u8 as *const libc::c_char
                                } else if (*P).dir == 2 as libc::c_int {
                                    b"MAXimum\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"???\0" as *const u8 as *const libc::c_char
                                },
                            );
                            _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                            _glp_format(
                                fp,
                                b"%6s %-12s %2s %13s %13s %13s  %13s %13s %13s %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                b"No.\0" as *const u8 as *const libc::c_char,
                                if pass == 1 as libc::c_int {
                                    b"Row name\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"Column name\0" as *const u8 as *const libc::c_char
                                },
                                b"St\0" as *const u8 as *const libc::c_char,
                                b"Activity\0" as *const u8 as *const libc::c_char,
                                if pass == 1 as libc::c_int {
                                    b"Slack\0" as *const u8 as *const libc::c_char
                                } else {
                                    b"Obj coef\0" as *const u8 as *const libc::c_char
                                },
                                b"Lower bound\0" as *const u8 as *const libc::c_char,
                                b"Activity\0" as *const u8 as *const libc::c_char,
                                b"Obj coef\0" as *const u8 as *const libc::c_char,
                                b"Obj value at\0" as *const u8 as *const libc::c_char,
                                b"Limiting\0" as *const u8 as *const libc::c_char,
                            );
                            _glp_format(
                                fp,
                                b"%6s %-12s %2s %13s %13s %13s  %13s %13s %13s %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                                b"Marginal\0" as *const u8 as *const libc::c_char,
                                b"Upper bound\0" as *const u8 as *const libc::c_char,
                                b"range\0" as *const u8 as *const libc::c_char,
                                b"range\0" as *const u8 as *const libc::c_char,
                                b"break point\0" as *const u8 as *const libc::c_char,
                                b"variable\0" as *const u8 as *const libc::c_char,
                            );
                            _glp_format(
                                fp,
                                b"------ ------------ -- ------------- ------------- -------------  ------------- ------------- ------------- ------------\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        if pass == 1 as libc::c_int {
                            numb = k;
                            (1 as libc::c_int <= numb && numb <= m
                                || {
                                    glp_assert_(
                                        b"1 <= numb && numb <= m\0" as *const u8
                                            as *const libc::c_char,
                                        b"api/prrngs.c\0" as *const u8 as *const libc::c_char,
                                        140 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            row = *((*P).row).offset(numb as isize);
                            name = (*row).name;
                            type_0 = (*row).type_0;
                            lb = glp_get_row_lb(P, numb);
                            ub = glp_get_row_ub(P, numb);
                            coef = 0.0f64;
                            stat = (*row).stat;
                            prim = (*row).prim;
                            if type_0 == 1 as libc::c_int {
                                slack = -prim;
                            } else if type_0 == 2 as libc::c_int {
                                slack = lb - prim;
                            } else if type_0 == 3 as libc::c_int
                                || type_0 == 4 as libc::c_int || type_0 == 5 as libc::c_int
                            {
                                slack = ub - prim;
                            }
                            dual = (*row).dual;
                        } else {
                            numb = k - m;
                            (1 as libc::c_int <= numb && numb <= n
                                || {
                                    glp_assert_(
                                        b"1 <= numb && numb <= n\0" as *const u8
                                            as *const libc::c_char,
                                        b"api/prrngs.c\0" as *const u8 as *const libc::c_char,
                                        159 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            col = *((*P).col).offset(numb as isize);
                            name = (*col).name;
                            lb = glp_get_col_lb(P, numb);
                            ub = glp_get_col_ub(P, numb);
                            coef = (*col).coef;
                            stat = (*col).stat;
                            prim = (*col).prim;
                            slack = 0.0f64;
                            dual = (*col).dual;
                        }
                        if stat != 1 as libc::c_int {
                            glp_analyze_bound(
                                P,
                                k,
                                &mut value1,
                                &mut var1,
                                &mut value2,
                                &mut var2,
                            );
                            if stat == 4 as libc::c_int {
                                coef2 = coef;
                                coef1 = coef2;
                            } else if stat == 5 as libc::c_int {
                                coef1 = -1.7976931348623157e+308f64;
                                coef2 = 1.7976931348623157e+308f64;
                            } else if stat == 2 as libc::c_int
                                && (*P).dir == 1 as libc::c_int
                                || stat == 3 as libc::c_int && (*P).dir == 2 as libc::c_int
                            {
                                coef1 = coef - dual;
                                coef2 = 1.7976931348623157e+308f64;
                            } else {
                                coef1 = -1.7976931348623157e+308f64;
                                coef2 = coef - dual;
                            }
                            if value1 == -1.7976931348623157e+308f64 {
                                if dual < -1e-9f64 {
                                    obj1 = 1.7976931348623157e+308f64;
                                } else if dual > 1e-9f64 {
                                    obj1 = -1.7976931348623157e+308f64;
                                } else {
                                    obj1 = (*P).obj_val;
                                }
                            } else {
                                obj1 = (*P).obj_val + dual * (value1 - prim);
                            }
                            if value2 == 1.7976931348623157e+308f64 {
                                if dual < -1e-9f64 {
                                    obj2 = -1.7976931348623157e+308f64;
                                } else if dual > 1e-9f64 {
                                    obj2 = 1.7976931348623157e+308f64;
                                } else {
                                    obj2 = (*P).obj_val;
                                }
                            } else {
                                obj2 = (*P).obj_val + dual * (value2 - prim);
                            }
                        } else {
                            glp_analyze_coef(
                                P,
                                k,
                                &mut coef1,
                                &mut var1,
                                &mut value1,
                                &mut coef2,
                                &mut var2,
                                &mut value2,
                            );
                            if coef1 == -1.7976931348623157e+308f64 {
                                if prim < -1e-9f64 {
                                    obj1 = 1.7976931348623157e+308f64;
                                } else if prim > 1e-9f64 {
                                    obj1 = -1.7976931348623157e+308f64;
                                } else {
                                    obj1 = (*P).obj_val;
                                }
                            } else {
                                obj1 = (*P).obj_val + (coef1 - coef) * prim;
                            }
                            if coef2 == 1.7976931348623157e+308f64 {
                                if prim < -1e-9f64 {
                                    obj2 = -1.7976931348623157e+308f64;
                                } else if prim > 1e-9f64 {
                                    obj2 = 1.7976931348623157e+308f64;
                                } else {
                                    obj2 = (*P).obj_val;
                                }
                            } else {
                                obj2 = (*P).obj_val + (coef2 - coef) * prim;
                            }
                        }
                        _glp_format(
                            fp,
                            b"%6d\0" as *const u8 as *const libc::c_char,
                            numb,
                        );
                        _glp_format(
                            fp,
                            b" %-12.12s\0" as *const u8 as *const libc::c_char,
                            if name.is_null() {
                                b"\0" as *const u8 as *const libc::c_char
                            } else {
                                name
                            },
                        );
                        if !name.is_null()
                            && strlen(name) > 12 as libc::c_int as libc::c_ulong
                        {
                            _glp_format(
                                fp,
                                b"%s\n%6s %12s\0" as *const u8 as *const libc::c_char,
                                name.offset(12 as libc::c_int as isize),
                                b"\0" as *const u8 as *const libc::c_char,
                                b"\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        _glp_format(
                            fp,
                            b" %2s\0" as *const u8 as *const libc::c_char,
                            if stat == 1 as libc::c_int {
                                b"BS\0" as *const u8 as *const libc::c_char
                            } else if stat == 2 as libc::c_int {
                                b"NL\0" as *const u8 as *const libc::c_char
                            } else if stat == 3 as libc::c_int {
                                b"NU\0" as *const u8 as *const libc::c_char
                            } else if stat == 4 as libc::c_int {
                                b"NF\0" as *const u8 as *const libc::c_char
                            } else if stat == 5 as libc::c_int {
                                b"NS\0" as *const u8 as *const libc::c_char
                            } else {
                                b"??\0" as *const u8 as *const libc::c_char
                            },
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), prim),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), if k <= m { slack } else { coef }),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), lb),
                        );
                        _glp_format(
                            fp,
                            b"  %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), value1),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), coef1),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), obj1),
                        );
                        if var1 != 0 as libc::c_int {
                            if var1 <= m {
                                limit = glp_get_row_name(P, var1);
                            } else {
                                limit = glp_get_col_name(P, var1 - m);
                            }
                            if !limit.is_null() {
                                _glp_format(
                                    fp,
                                    b" %s\0" as *const u8 as *const libc::c_char,
                                    limit,
                                );
                            }
                        }
                        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                        _glp_format(
                            fp,
                            b"%6s %-12s %2s %13s\0" as *const u8 as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), dual),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), ub),
                        );
                        _glp_format(
                            fp,
                            b"  %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), value2),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), coef2),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const libc::c_char,
                            format(buf.as_mut_ptr(), obj2),
                        );
                        if var2 != 0 as libc::c_int {
                            if var2 <= m {
                                limit = glp_get_row_name(P, var2);
                            } else {
                                limit = glp_get_col_name(P, var2 - m);
                            }
                            if !limit.is_null() {
                                _glp_format(
                                    fp,
                                    b" %s\0" as *const u8 as *const libc::c_char,
                                    limit,
                                );
                            }
                        }
                        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                        _glp_format(fp, b"\n\0" as *const u8 as *const libc::c_char);
                        count = (count + 1 as libc::c_int) % 10 as libc::c_int;
                    }
                    t += 1;
                    t;
                }
                pass += 1;
                pass;
            }
            _glp_format(fp, b"End of report\n\0" as *const u8 as *const libc::c_char);
            if _glp_ioerr(fp) != 0 {
                glp_printf(
                    b"Write error on '%s' - %s\n\0" as *const u8 as *const libc::c_char,
                    fname,
                    _glp_get_err_msg(),
                );
                ret = 4 as libc::c_int;
            } else {
                ret = 0 as libc::c_int;
            }
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}
