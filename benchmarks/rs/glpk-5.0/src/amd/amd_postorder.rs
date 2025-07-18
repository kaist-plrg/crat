use ::libc;
extern "C" {
    fn _glp_amd_post_tree(
        root: libc::c_int,
        k: libc::c_int,
        Child: *mut libc::c_int,
        Sibling: *const libc::c_int,
        Order: *mut libc::c_int,
        Stack: *mut libc::c_int,
    ) -> libc::c_int;
}
pub unsafe extern "C" fn _glp_amd_postorder(
    mut nn: libc::c_int,
    mut Parent: *mut libc::c_int,
    mut Nv: *mut libc::c_int,
    mut Fsize: *mut libc::c_int,
    mut Order: *mut libc::c_int,
    mut Child: *mut libc::c_int,
    mut Sibling: *mut libc::c_int,
    mut Stack: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    let mut frsize: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut fprev: libc::c_int = 0;
    let mut maxfrsize: libc::c_int = 0;
    let mut bigfprev: libc::c_int = 0;
    let mut bigf: libc::c_int = 0;
    let mut fnext: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < nn {
        *Child.offset(j as isize) = -(1 as libc::c_int);
        *Sibling.offset(j as isize) = -(1 as libc::c_int);
        j += 1;
        j;
    }
    j = nn - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        if *Nv.offset(j as isize) > 0 as libc::c_int {
            parent = *Parent.offset(j as isize);
            if parent != -(1 as libc::c_int) {
                *Sibling.offset(j as isize) = *Child.offset(parent as isize);
                *Child.offset(parent as isize) = j;
            }
        }
        j -= 1;
        j;
    }
    i = 0 as libc::c_int;
    while i < nn {
        if *Nv.offset(i as isize) > 0 as libc::c_int
            && *Child.offset(i as isize) != -(1 as libc::c_int)
        {
            fprev = -(1 as libc::c_int);
            maxfrsize = -(1 as libc::c_int);
            bigfprev = -(1 as libc::c_int);
            bigf = -(1 as libc::c_int);
            f = *Child.offset(i as isize);
            while f != -(1 as libc::c_int) {
                frsize = *Fsize.offset(f as isize);
                if frsize >= maxfrsize {
                    maxfrsize = frsize;
                    bigfprev = fprev;
                    bigf = f;
                }
                fprev = f;
                f = *Sibling.offset(f as isize);
            }
            fnext = *Sibling.offset(bigf as isize);
            if fnext != -(1 as libc::c_int) {
                if bigfprev == -(1 as libc::c_int) {
                    *Child.offset(i as isize) = fnext;
                } else {
                    *Sibling.offset(bigfprev as isize) = fnext;
                }
                *Sibling.offset(bigf as isize) = -(1 as libc::c_int);
                *Sibling.offset(fprev as isize) = bigf;
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nn {
        *Order.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nn {
        if *Parent.offset(i as isize) == -(1 as libc::c_int)
            && *Nv.offset(i as isize) > 0 as libc::c_int
        {
            k = _glp_amd_post_tree(
                i,
                k,
                Child,
                Sibling as *const libc::c_int,
                Order,
                Stack,
            );
        }
        i += 1;
        i;
    }
}
