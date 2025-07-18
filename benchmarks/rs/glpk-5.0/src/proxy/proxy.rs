use ::libc;
extern "C" {
    pub type glp_prob;
    pub type glp_tree;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_set_obj_dir(P: *mut glp_prob, dir: libc::c_int);
    fn glp_add_rows(P: *mut glp_prob, nrs: libc::c_int) -> libc::c_int;
    fn glp_set_row_name(P: *mut glp_prob, i: libc::c_int, name: *const libc::c_char);
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
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: libc::c_int);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_obj_dir(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_col_lb(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_obj_coef(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> libc::c_int;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_get_col_prim(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_get_col_kind(P: *mut glp_prob, j: libc::c_int) -> libc::c_int;
    fn glp_intopt(P: *mut glp_prob, parm: *const glp_iocp) -> libc::c_int;
    fn glp_init_iocp(parm: *mut glp_iocp);
    fn glp_mip_status(P: *mut glp_prob) -> libc::c_int;
    fn glp_mip_obj_val(P: *mut glp_prob) -> libc::c_double;
    fn glp_mip_col_val(P: *mut glp_prob, j: libc::c_int) -> libc::c_double;
    fn glp_ios_reason(T: *mut glp_tree) -> libc::c_int;
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> libc::c_int;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_term_out(flag: libc::c_int) -> libc::c_int;
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_mem_usage(
        count: *mut libc::c_int,
        cpeak: *mut libc::c_int,
        total: *mut size_t,
        tpeak: *mut size_t,
    );
    fn glp_time() -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub integer_obj: libc::c_int,
    pub b_vars_exist: libc::c_int,
    pub i_vars_exist: libc::c_int,
    pub startsol: *const libc::c_double,
    pub ckind: *mut libc::c_int,
    pub clb: *mut libc::c_double,
    pub cub: *mut libc::c_double,
    pub true_obj: *mut libc::c_double,
    pub dir: libc::c_int,
    pub ncols: libc::c_int,
    pub GLOtstart: time_t,
    pub lp_ref: *mut glp_prob,
}
pub unsafe extern "C" fn _glp_proxy(
    mut lp: *mut glp_prob,
    mut zfinal: *mut libc::c_double,
    mut xfinal: *mut libc::c_double,
    mut initsol: *const libc::c_double,
    mut rel_impr: libc::c_double,
    mut tlim: libc::c_int,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut csa_: csa = csa {
        integer_obj: 0,
        b_vars_exist: 0,
        i_vars_exist: 0,
        startsol: 0 as *const libc::c_double,
        ckind: 0 as *mut libc::c_int,
        clb: 0 as *mut libc::c_double,
        cub: 0 as *mut libc::c_double,
        true_obj: 0 as *mut libc::c_double,
        dir: 0,
        ncols: 0,
        GLOtstart: 0,
        lp_ref: 0 as *mut glp_prob,
    };
    let mut csa: *mut csa = &mut csa_;
    let mut parm: glp_iocp = glp_iocp {
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
    let mut parm_lp: glp_smcp = glp_smcp {
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
    let mut tpeak: size_t = 0;
    let mut refine: libc::c_int = 0;
    let mut tref_lim: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut cutoff_row: libc::c_int = 0;
    let mut niter: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tout: libc::c_int = 0;
    let mut xref: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xstar: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut zstar: libc::c_double = 0.;
    let mut tela: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut zz: libc::c_double = 0.;
    memset(
        csa as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<csa>() as libc::c_ulong,
    );
    (*csa).dir = glp_get_obj_dir(lp);
    (*csa).ncols = glp_get_num_cols(lp);
    get_info(csa, lp);
    check_integrality(csa);
    if (*csa).b_vars_exist == 0 as libc::c_int {
        if verbose != 0 {
            glp_printf(
                b"The problem has not binary variables. Proximity search cannot be used.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        glp_free((*csa).ckind as *mut libc::c_void);
        glp_free((*csa).clb as *mut libc::c_void);
        glp_free((*csa).cub as *mut libc::c_void);
        glp_free((*csa).true_obj as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    xref = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    refine = check_ref(csa, lp, xref);
    xstar = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    if verbose != 0 {
        glp_printf(
            b"Applying PROXY heuristic...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    (*csa).GLOtstart = second() as time_t;
    glp_init_iocp(&mut parm);
    glp_init_smcp(&mut parm_lp);
    parm.bt_tech = 4 as libc::c_int;
    if rel_impr <= 0.0f64 {
        rel_impr = 0.01f64;
    }
    if tlim <= 0 as libc::c_int {
        tlim = 2147483647 as libc::c_int;
    }
    if verbose != 0 {
        glp_printf(
            b"Proxy's time limit set to %d seconds.\n\0" as *const u8
                as *const libc::c_char,
            tlim / 1000 as libc::c_int,
        );
        glp_printf(
            b"Proxy's relative improvement set to %2.2lf %c.\n\0" as *const u8
                as *const libc::c_char,
            rel_impr * 100 as libc::c_int as libc::c_double,
            37 as libc::c_int,
        );
    }
    parm_lp.tm_lim = tlim;
    parm.mip_gap = 9999999.9f64;
    if verbose != 0 {
        glp_printf(
            b"Searching for a feasible solution...\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !initsol.is_null() {
        (*csa).startsol = initsol;
        parm
            .cb_func = Some(
            callback as unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> (),
        );
        parm.cb_info = csa as *mut libc::c_void;
        if verbose != 0 {
            glp_printf(b"Input solution found.\n\0" as *const u8 as *const libc::c_char);
        }
    }
    tout = glp_term_out(0 as libc::c_int);
    err = glp_simplex(lp, &mut parm_lp);
    glp_term_out(tout);
    status = glp_get_status(lp);
    if status != 5 as libc::c_int {
        if verbose != 0 {
            glp_printf(
                b"Proxy heuristic terminated.\n\0" as *const u8 as *const libc::c_char,
            );
        }
        glp_free(xref as *mut libc::c_void);
        glp_free(xstar as *mut libc::c_void);
        deallocate(csa, refine);
        return -(1 as libc::c_int);
    }
    tela = elapsed_time(csa);
    if tlim as libc::c_double - tela * 1000 as libc::c_int as libc::c_double
        <= 0 as libc::c_int as libc::c_double
    {
        if verbose != 0 {
            glp_printf(
                b"Time limit exceeded. Proxy could not find optimal solution to LP relaxation.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            glp_printf(
                b"Proxy heuristic aborted.\n\0" as *const u8 as *const libc::c_char,
            );
        }
        glp_free(xref as *mut libc::c_void);
        glp_free(xstar as *mut libc::c_void);
        deallocate(csa, refine);
        return -(1 as libc::c_int);
    }
    parm
        .tm_lim = (tlim as libc::c_double - tela * 1000 as libc::c_int as libc::c_double)
        as libc::c_int;
    tref_lim = ((tlim as libc::c_double - tela * 1000 as libc::c_int as libc::c_double)
        / 20 as libc::c_int as libc::c_double) as libc::c_int;
    tout = glp_term_out(0 as libc::c_int);
    err = glp_intopt(lp, &mut parm);
    glp_term_out(tout);
    status = glp_mip_status(lp);
    if status == 4 as libc::c_int || status == 1 as libc::c_int {
        if err == 0x9 as libc::c_int {
            if verbose != 0 {
                glp_printf(
                    b"Time limit exceeded. Proxy could not find an initial integer feasible solution.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                glp_printf(
                    b"Proxy heuristic aborted.\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else if verbose != 0 {
            glp_printf(
                b"Proxy could not find an initial integer feasible solution.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            glp_printf(
                b"Proxy heuristic aborted.\n\0" as *const u8 as *const libc::c_char,
            );
        }
        glp_free(xref as *mut libc::c_void);
        glp_free(xstar as *mut libc::c_void);
        deallocate(csa, refine);
        return -(1 as libc::c_int);
    }
    get_sol(csa, lp, xstar);
    zstar = glp_mip_obj_val(lp);
    if verbose != 0 {
        glp_printf(
            b">>>>> first solution = %e;\n\0" as *const u8 as *const libc::c_char,
            zstar,
        );
    }
    if err == 0x9 as libc::c_int {
        if verbose != 0 {
            glp_printf(
                b"Time limit exceeded. Proxy heuristic terminated.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        tela = elapsed_time(csa);
        tpeak = 0 as libc::c_int as size_t;
        glp_mem_usage(
            0 as *mut libc::c_int,
            0 as *mut libc::c_int,
            0 as *mut size_t,
            &mut tpeak,
        );
        if verbose != 0 {
            glp_printf(
                b"Time used: %3.1lf secs.  Memory used: %2.1lf Mb\n\0" as *const u8
                    as *const libc::c_char,
                tela,
                tpeak as libc::c_double / 1048576 as libc::c_int as libc::c_double,
            );
            glp_printf(
                b"Starting proximity search...\n\0" as *const u8 as *const libc::c_char,
            );
        }
        cutoff_row = add_cutoff(csa, lp);
        if (*csa).dir == 2 as libc::c_int {
            glp_set_obj_dir(lp, 1 as libc::c_int);
        }
        niter = 0 as libc::c_int;
        loop {
            niter += 1;
            niter;
            redefine_obj(lp, xstar, (*csa).ncols, (*csa).ckind, (*csa).clb, (*csa).cub);
            cutoff = update_cutoff(csa, lp, zstar, cutoff_row, rel_impr);
            tela = elapsed_time(csa);
            if tlim as libc::c_double - tela * 1000 as libc::c_int as libc::c_double
                <= 0 as libc::c_int as libc::c_double
            {
                if verbose != 0 {
                    glp_printf(
                        b"Time limit exceeded. Proxy heuristic terminated.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                break;
            } else {
                parm_lp
                    .tm_lim = (tlim as libc::c_double
                    - tela * 1000 as libc::c_int as libc::c_double) as libc::c_int;
                tout = glp_term_out(0 as libc::c_int);
                err = glp_simplex(lp, &mut parm_lp);
                glp_term_out(tout);
                status = glp_get_status(lp);
                if status != 5 as libc::c_int {
                    if status == 4 as libc::c_int {
                        if verbose != 0 {
                            glp_printf(
                                b"Bound exceeded = %f. \0" as *const u8
                                    as *const libc::c_char,
                                cutoff,
                            );
                        }
                    }
                    if verbose != 0 {
                        glp_printf(
                            b"Proxy heuristic terminated.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    break;
                } else {
                    tela = elapsed_time(csa);
                    if tlim as libc::c_double
                        - tela * 1000 as libc::c_int as libc::c_double
                        <= 0 as libc::c_int as libc::c_double
                    {
                        if verbose != 0 {
                            glp_printf(
                                b"Time limit exceeded. Proxy heuristic terminated.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        break;
                    } else {
                        parm
                            .tm_lim = (tlim as libc::c_double
                            - tela * 1000 as libc::c_int as libc::c_double)
                            as libc::c_int;
                        parm.cb_func = None;
                        tout = glp_term_out(0 as libc::c_int);
                        err = glp_intopt(lp, &mut parm);
                        glp_term_out(tout);
                        status = glp_mip_status(lp);
                        if status == 4 as libc::c_int {
                            if verbose != 0 {
                                glp_printf(
                                    b"Bound exceeded = %f. Proxy heuristic terminated.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    cutoff,
                                );
                            }
                            break;
                        } else if status == 1 as libc::c_int {
                            if err == 0x9 as libc::c_int {
                                if verbose != 0 {
                                    glp_printf(
                                        b"Time limit exceeded. Proxy heuristic terminated.\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                            } else if verbose != 0 {
                                glp_printf(
                                    b"Proxy terminated unexpectedly.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            break;
                        } else if status == 2 as libc::c_int
                            || status == 5 as libc::c_int
                        {
                            get_sol(csa, lp, xstar);
                            zz = objval((*csa).ncols, xstar, (*csa).true_obj);
                            if zz < zstar && (*csa).dir == 1 as libc::c_int
                                || zz > zstar && (*csa).dir == 2 as libc::c_int
                            {
                                if refine != 0 {
                                    array_copy(
                                        1 as libc::c_int,
                                        (*csa).ncols + 1 as libc::c_int,
                                        xstar,
                                        xref,
                                    );
                                    err = do_refine(
                                        csa,
                                        (*csa).lp_ref,
                                        (*csa).ncols,
                                        (*csa).ckind,
                                        xref,
                                        &mut tlim,
                                        tref_lim,
                                        verbose,
                                    );
                                    if err == 0 {
                                        let mut zref: libc::c_double = objval(
                                            (*csa).ncols,
                                            xref,
                                            (*csa).true_obj,
                                        );
                                        if zref < zz && (*csa).dir == 1 as libc::c_int
                                            || zref > zz && (*csa).dir == 2 as libc::c_int
                                        {
                                            zz = zref;
                                            array_copy(
                                                1 as libc::c_int,
                                                (*csa).ncols + 1 as libc::c_int,
                                                xref,
                                                xstar,
                                            );
                                        }
                                    }
                                }
                                zstar = zz;
                                tela = elapsed_time(csa);
                                if verbose != 0 {
                                    glp_printf(
                                        b">>>>> it: %3d:   mip = %e;   elapsed time %3.1lf sec.s\n\0"
                                            as *const u8 as *const libc::c_char,
                                        niter,
                                        zstar,
                                        tela,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tela = elapsed_time(csa);
    glp_mem_usage(
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut size_t,
        &mut tpeak,
    );
    if verbose != 0 {
        glp_printf(
            b"Time used: %3.1lf.  Memory used: %2.1lf Mb\n\0" as *const u8
                as *const libc::c_char,
            tela,
            tpeak as libc::c_double / 1048576 as libc::c_int as libc::c_double,
        );
    }
    *zfinal = zstar;
    i = 1 as libc::c_int;
    while i < (*csa).ncols + 1 as libc::c_int {
        *xfinal.offset(i as isize) = *xstar.offset(i as isize);
        i += 1;
        i;
    }
    glp_free(xref as *mut libc::c_void);
    glp_free(xstar as *mut libc::c_void);
    deallocate(csa, refine);
    return 0 as libc::c_int;
}
unsafe extern "C" fn callback(mut tree: *mut glp_tree, mut info: *mut libc::c_void) {
    let mut csa: *mut csa = info as *mut csa;
    match glp_ios_reason(tree) {
        3 => {
            glp_ios_heur_sol(tree, (*csa).startsol);
        }
        _ => {}
    };
}
unsafe extern "C" fn get_info(mut csa: *mut csa, mut lp: *mut glp_prob) {
    let mut i: libc::c_int = 0;
    (*csa)
        .ckind = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    (*csa)
        .clb = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .cub = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    (*csa)
        .true_obj = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    i = 1 as libc::c_int;
    while i < (*csa).ncols + 1 as libc::c_int {
        *((*csa).ckind).offset(i as isize) = glp_get_col_kind(lp, i);
        *((*csa).clb).offset(i as isize) = glp_get_col_lb(lp, i);
        *((*csa).cub).offset(i as isize) = glp_get_col_ub(lp, i);
        *((*csa).true_obj).offset(i as isize) = glp_get_obj_coef(lp, i);
        i += 1;
        i;
    }
    *((*csa).true_obj)
        .offset(0 as libc::c_int as isize) = glp_get_obj_coef(lp, 0 as libc::c_int);
}
unsafe extern "C" fn is_integer(mut csa: *mut csa) -> libc::c_int {
    let mut i: libc::c_int = 0;
    (*csa).integer_obj = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*csa).ncols + 1 as libc::c_int {
        if fabs(*((*csa).true_obj).offset(i as isize))
            > 2147483647 as libc::c_int as libc::c_double
        {
            (*csa).integer_obj = 0 as libc::c_int;
        }
        if fabs(*((*csa).true_obj).offset(i as isize))
            <= 2147483647 as libc::c_int as libc::c_double
        {
            let mut tmp: libc::c_double = 0.;
            let mut rem: libc::c_double = 0.;
            if fabs(*((*csa).true_obj).offset(i as isize))
                - floor(fabs(*((*csa).true_obj).offset(i as isize))) < 0.5f64
            {
                tmp = floor(fabs(*((*csa).true_obj).offset(i as isize)));
            } else {
                tmp = ceil(fabs(*((*csa).true_obj).offset(i as isize)));
            }
            rem = fabs(*((*csa).true_obj).offset(i as isize)) - tmp;
            rem = fabs(rem);
            if rem > 1e-6f64 {
                (*csa).integer_obj = 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    return (*csa).integer_obj;
}
unsafe extern "C" fn check_integrality(mut csa: *mut csa) {
    let mut i: libc::c_int = 0;
    (*csa).integer_obj = is_integer(csa);
    (*csa).b_vars_exist = 0 as libc::c_int;
    (*csa).i_vars_exist = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < (*csa).ncols + 1 as libc::c_int {
        if *((*csa).ckind).offset(i as isize) == 2 as libc::c_int {
            (*csa).i_vars_exist = 1 as libc::c_int;
        } else if *((*csa).ckind).offset(i as isize) == 3 as libc::c_int {
            (*csa).b_vars_exist = 1 as libc::c_int;
        } else {
            (*csa).integer_obj = 0 as libc::c_int;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn check_ref(
    mut csa: *mut csa,
    mut lp: *mut glp_prob,
    mut xref: *mut libc::c_double,
) -> libc::c_int {
    let mut refine: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*csa).ncols + 1 as libc::c_int {
        if *((*csa).ckind).offset(i as isize) != 3 as libc::c_int {
            refine = 1 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if refine != 0 {
        (*csa).lp_ref = glp_create_prob();
        glp_copy_prob((*csa).lp_ref, lp, 1 as libc::c_int);
    }
    return refine;
}
unsafe extern "C" fn second() -> libc::c_double {
    return glp_time() / 1000.0f64;
}
unsafe extern "C" fn add_cutoff(
    mut csa: *mut csa,
    mut lp: *mut glp_prob,
) -> libc::c_int {
    let mut obj_index: *mut libc::c_int = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_int;
    let mut obj_value: *mut libc::c_double = glp_alloc(
        (*csa).ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    let mut obj_nzcnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut irow: libc::c_int = 0;
    let mut rowname: *const libc::c_char = 0 as *const libc::c_char;
    i = 1 as libc::c_int;
    while i < (*csa).ncols + 1 as libc::c_int {
        if fabs(*((*csa).true_obj).offset(i as isize)) > 1e-6f64 {
            obj_nzcnt += 1;
            obj_nzcnt;
            *obj_index.offset(obj_nzcnt as isize) = i;
            *obj_value
                .offset(obj_nzcnt as isize) = *((*csa).true_obj).offset(i as isize);
        }
        i += 1;
        i;
    }
    irow = glp_add_rows(lp, 1 as libc::c_int);
    rowname = b"Cutoff\0" as *const u8 as *const libc::c_char;
    glp_set_row_name(lp, irow, rowname);
    if (*csa).dir == 1 as libc::c_int {
        glp_set_row_bnds(lp, irow, 3 as libc::c_int, 1e20f64, 1e20f64);
    } else {
        glp_set_row_bnds(lp, irow, 2 as libc::c_int, -1e20f64, -1e20f64);
    }
    glp_set_mat_row(
        lp,
        irow,
        obj_nzcnt,
        obj_index as *const libc::c_int,
        obj_value as *const libc::c_double,
    );
    glp_free(obj_index as *mut libc::c_void);
    glp_free(obj_value as *mut libc::c_void);
    return irow;
}
unsafe extern "C" fn get_sol(
    mut csa: *mut csa,
    mut lp: *mut glp_prob,
    mut xstar: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*csa).ncols + 1 as libc::c_int {
        *xstar.offset(i as isize) = glp_mip_col_val(lp, i);
        i += 1;
        i;
    }
}
unsafe extern "C" fn elapsed_time(mut csa: *mut csa) -> libc::c_double {
    let mut tela: libc::c_double = second() - (*csa).GLOtstart as libc::c_double;
    if tela < 0 as libc::c_int as libc::c_double {
        tela += 86400.0f64;
    }
    return tela;
}
unsafe extern "C" fn redefine_obj(
    mut lp: *mut glp_prob,
    mut xtilde: *mut libc::c_double,
    mut ncols: libc::c_int,
    mut ckind: *mut libc::c_int,
    mut clb: *mut libc::c_double,
    mut cub: *mut libc::c_double,
) {
    let mut j: libc::c_int = 0;
    let mut delta: *mut libc::c_double = glp_alloc(
        ncols + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_double;
    j = 1 as libc::c_int;
    while j < ncols + 1 as libc::c_int {
        *delta.offset(j as isize) = 0.0f64;
        if !(*ckind.offset(j as isize) == 1 as libc::c_int) {
            if !(*cub.offset(j as isize) - *clb.offset(j as isize) < 0.5f64) {
                if *ckind.offset(j as isize) == 3 as libc::c_int {
                    if *xtilde.offset(j as isize) > 0.5f64 {
                        *delta.offset(j as isize) = -1.0f64;
                    } else {
                        *delta.offset(j as isize) = 1.0f64;
                    }
                }
            }
        }
        j += 1;
        j;
    }
    j = 1 as libc::c_int;
    while j < ncols + 1 as libc::c_int {
        glp_set_obj_coef(lp, j, *delta.offset(j as isize));
        j += 1;
        j;
    }
    glp_set_obj_coef(lp, 0 as libc::c_int, 0.0f64);
    glp_free(delta as *mut libc::c_void);
}
unsafe extern "C" fn update_cutoff(
    mut csa: *mut csa,
    mut lp: *mut glp_prob,
    mut zstar: libc::c_double,
    mut cutoff_row: libc::c_int,
    mut rel_impr: libc::c_double,
) -> libc::c_double {
    let mut cutoff: libc::c_double = 0.;
    zstar -= *((*csa).true_obj).offset(0 as libc::c_int as isize);
    if (*csa).dir == 1 as libc::c_int {
        cutoff = zstar - compute_delta(csa, zstar, rel_impr);
        glp_set_row_bnds(lp, cutoff_row, 3 as libc::c_int, cutoff, cutoff);
    } else {
        cutoff = zstar + compute_delta(csa, zstar, rel_impr);
        glp_set_row_bnds(lp, cutoff_row, 2 as libc::c_int, cutoff, cutoff);
    }
    return cutoff;
}
unsafe extern "C" fn compute_delta(
    mut csa: *mut csa,
    mut z: libc::c_double,
    mut rel_impr: libc::c_double,
) -> libc::c_double {
    let mut delta: libc::c_double = rel_impr * fabs(z);
    if (*csa).integer_obj != 0 {
        delta = ceil(delta);
    }
    return delta;
}
unsafe extern "C" fn objval(
    mut ncols: libc::c_int,
    mut x: *mut libc::c_double,
    mut true_obj: *mut libc::c_double,
) -> libc::c_double {
    let mut j: libc::c_int = 0;
    let mut z: libc::c_double = 0.0f64;
    j = 1 as libc::c_int;
    while j < ncols + 1 as libc::c_int {
        z += *x.offset(j as isize) * *true_obj.offset(j as isize);
        j += 1;
        j;
    }
    return z + *true_obj.offset(0 as libc::c_int as isize);
}
unsafe extern "C" fn array_copy(
    mut begin: libc::c_int,
    mut end: libc::c_int,
    mut source: *mut libc::c_double,
    mut destination: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = begin;
    while i < end {
        *destination.offset(i as isize) = *source.offset(i as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn do_refine(
    mut csa: *mut csa,
    mut lp_ref: *mut glp_prob,
    mut ncols: libc::c_int,
    mut ckind: *mut libc::c_int,
    mut xref: *mut libc::c_double,
    mut tlim: *mut libc::c_int,
    mut tref_lim: libc::c_int,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut tout: libc::c_int = 0;
    let mut refineStart: libc::c_double = second();
    let mut val: libc::c_double = 0.;
    let mut tela: libc::c_double = 0.;
    let mut tlimit: libc::c_double = 0.;
    if glp_get_num_cols(lp_ref) != ncols {
        if verbose != 0 {
            glp_printf(
                b"Error in Proxy refinement: \0" as *const u8 as *const libc::c_char,
            );
            glp_printf(
                b"wrong number of columns (%d vs %d).\n\0" as *const u8
                    as *const libc::c_char,
                ncols,
                glp_get_num_cols(lp_ref),
            );
        }
        return 1 as libc::c_int;
    }
    val = -1.0f64;
    j = 1 as libc::c_int;
    while j < ncols + 1 as libc::c_int {
        if *ckind.offset(j as isize) == 3 as libc::c_int {
            val = 0.0f64;
            if *xref.offset(j as isize) > 0.5f64 {
                val = 1.0f64;
            }
            glp_set_col_bnds(lp_ref, j, 5 as libc::c_int, val, val);
        }
        j += 1;
        j;
    }
    if val > -1.0f64 {
        let mut parm_ref: glp_iocp = glp_iocp {
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
        let mut parm_ref_lp: glp_smcp = glp_smcp {
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
        let mut err: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        glp_init_iocp(&mut parm_ref);
        parm_ref.presolve = 1 as libc::c_int;
        glp_init_smcp(&mut parm_ref_lp);
        parm_ref.tm_lim = tref_lim;
        if parm_ref.tm_lim > *tlim {
            parm_ref.tm_lim = *tlim;
        }
        parm_ref_lp.tm_lim = parm_ref.tm_lim;
        tout = glp_term_out(0 as libc::c_int);
        if (*csa).i_vars_exist == 1 as libc::c_int {
            err = glp_intopt(lp_ref, &mut parm_ref);
        } else {
            err = glp_simplex(lp_ref, &mut parm_ref_lp);
        }
        glp_term_out(tout);
        if (*csa).i_vars_exist == 1 as libc::c_int {
            status = glp_mip_status(lp_ref);
        } else {
            status = glp_get_status(lp_ref);
        }
        match status {
            5 | 2 => {}
            _ => {
                status = 1 as libc::c_int;
            }
        }
        if status == 1 as libc::c_int {
            if err == 0x9 as libc::c_int {
                return 1 as libc::c_int;
            }
        }
        j = 1 as libc::c_int;
        while j < ncols + 1 as libc::c_int {
            if *ckind.offset(j as isize) != 3 as libc::c_int {
                if (*csa).i_vars_exist == 1 as libc::c_int {
                    *xref.offset(j as isize) = glp_mip_col_val(lp_ref, j);
                } else {
                    *xref.offset(j as isize) = glp_get_col_prim(lp_ref, j);
                }
            }
            j += 1;
            j;
        }
    }
    tela = second() - refineStart;
    return 0 as libc::c_int;
}
unsafe extern "C" fn deallocate(mut csa: *mut csa, mut refine: libc::c_int) {
    if refine != 0 {
        glp_delete_prob((*csa).lp_ref);
    }
    glp_free((*csa).ckind as *mut libc::c_void);
    glp_free((*csa).clb as *mut libc::c_void);
    glp_free((*csa).cub as *mut libc::c_void);
    glp_free((*csa).true_obj as *mut libc::c_void);
}
