use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn abort() -> !;
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
pub unsafe extern "C" fn _glp_genqmd(
    mut neqns: *mut libc::c_int,
    mut xadj: *mut libc::c_int,
    mut adjncy: *mut libc::c_int,
    mut perm: *mut libc::c_int,
    mut invp: *mut libc::c_int,
    mut deg: *mut libc::c_int,
    mut marker: *mut libc::c_int,
    mut rchset: *mut libc::c_int,
    mut nbrhd: *mut libc::c_int,
    mut qsize: *mut libc::c_int,
    mut qlink: *mut libc::c_int,
    mut nofsub: *mut libc::c_int,
) {
    static mut func: [libc::c_char; 7] = unsafe {
        *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"genqmd\0")
    };
    (neqns == neqns
        || {
            glp_assert_(
                b"neqns == neqns\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                12 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (perm == perm
        || {
            glp_assert_(
                b"perm == perm\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (invp == invp
        || {
            glp_assert_(
                b"invp == invp\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                14 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                16 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                17 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                18 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (qsize == qsize
        || {
            glp_assert_(
                b"qsize == qsize\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                19 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (qlink == qlink
        || {
            glp_assert_(
                b"qlink == qlink\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                20 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nofsub == nofsub
        || {
            glp_assert_(
                b"nofsub == nofsub\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                21 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const libc::c_char, 22 as libc::c_int))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
pub unsafe extern "C" fn _glp_qmdrch(
    mut root: *mut libc::c_int,
    mut xadj: *mut libc::c_int,
    mut adjncy: *mut libc::c_int,
    mut deg: *mut libc::c_int,
    mut marker: *mut libc::c_int,
    mut rchsze: *mut libc::c_int,
    mut rchset: *mut libc::c_int,
    mut nhdsze: *mut libc::c_int,
    mut nbrhd: *mut libc::c_int,
) {
    static mut func: [libc::c_char; 7] = unsafe {
        *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"qmdrch\0")
    };
    (root == root
        || {
            glp_assert_(
                b"root == root\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                31 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                33 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                34 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                35 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (rchsze == rchsze
        || {
            glp_assert_(
                b"rchsze == rchsze\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                36 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                37 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nhdsze == nhdsze
        || {
            glp_assert_(
                b"nhdsze == nhdsze\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                39 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const libc::c_char, 40 as libc::c_int))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
pub unsafe extern "C" fn _glp_qmdqt(
    mut root: *mut libc::c_int,
    mut xadj: *mut libc::c_int,
    mut adjncy: *mut libc::c_int,
    mut marker: *mut libc::c_int,
    mut rchsze: *mut libc::c_int,
    mut rchset: *mut libc::c_int,
    mut nbrhd: *mut libc::c_int,
) {
    static mut func: [libc::c_char; 6] = unsafe {
        *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"qmdqt\0")
    };
    (root == root
        || {
            glp_assert_(
                b"root == root\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                49 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                50 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (rchsze == rchsze
        || {
            glp_assert_(
                b"rchsze == rchsze\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                52 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const libc::c_char, 55 as libc::c_int))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
pub unsafe extern "C" fn _glp_qmdupd(
    mut xadj: *mut libc::c_int,
    mut adjncy: *mut libc::c_int,
    mut nlist: *mut libc::c_int,
    mut list: *mut libc::c_int,
    mut deg: *mut libc::c_int,
    mut qsize: *mut libc::c_int,
    mut qlink: *mut libc::c_int,
    mut marker: *mut libc::c_int,
    mut rchset: *mut libc::c_int,
    mut nbrhd: *mut libc::c_int,
) {
    static mut func: [libc::c_char; 7] = unsafe {
        *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"qmdupd\0")
    };
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                65 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nlist == nlist
        || {
            glp_assert_(
                b"nlist == nlist\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (list == list
        || {
            glp_assert_(
                b"list == list\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                67 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (qsize == qsize
        || {
            glp_assert_(
                b"qsize == qsize\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                69 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (qlink == qlink
        || {
            glp_assert_(
                b"qlink == qlink\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const libc::c_char, 74 as libc::c_int))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
pub unsafe extern "C" fn _glp_qmdmrg(
    mut xadj: *mut libc::c_int,
    mut adjncy: *mut libc::c_int,
    mut deg: *mut libc::c_int,
    mut qsize: *mut libc::c_int,
    mut qlink: *mut libc::c_int,
    mut marker: *mut libc::c_int,
    mut deg0: *mut libc::c_int,
    mut nhdsze: *mut libc::c_int,
    mut nbrhd: *mut libc::c_int,
    mut rchset: *mut libc::c_int,
    mut ovrlp: *mut libc::c_int,
) {
    static mut func: [libc::c_char; 7] = unsafe {
        *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"qmdmrg\0")
    };
    (xadj == xadj
        || {
            glp_assert_(
                b"xadj == xadj\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (adjncy == adjncy
        || {
            glp_assert_(
                b"adjncy == adjncy\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (deg == deg
        || {
            glp_assert_(
                b"deg == deg\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (qsize == qsize
        || {
            glp_assert_(
                b"qsize == qsize\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (qlink == qlink
        || {
            glp_assert_(
                b"qlink == qlink\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (marker == marker
        || {
            glp_assert_(
                b"marker == marker\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                88 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (deg0 == deg0
        || {
            glp_assert_(
                b"deg0 == deg0\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nhdsze == nhdsze
        || {
            glp_assert_(
                b"nhdsze == nhdsze\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                90 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (nbrhd == nbrhd
        || {
            glp_assert_(
                b"nbrhd == nbrhd\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (rchset == rchset
        || {
            glp_assert_(
                b"rchset == rchset\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (ovrlp == ovrlp
        || {
            glp_assert_(
                b"ovrlp == ovrlp\0" as *const u8 as *const libc::c_char,
                b"misc/qmd.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (glp_error_(b"misc/qmd.c\0" as *const u8 as *const libc::c_char, 94 as libc::c_int))
        .unwrap()(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const libc::c_char,
        func.as_ptr(),
    );
    abort();
}
