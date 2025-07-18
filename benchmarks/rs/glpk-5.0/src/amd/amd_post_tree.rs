use ::libc;
pub unsafe extern "C" fn _glp_amd_post_tree(
    mut root: libc::c_int,
    mut k: libc::c_int,
    mut Child: *mut libc::c_int,
    mut Sibling: *const libc::c_int,
    mut Order: *mut libc::c_int,
    mut Stack: *mut libc::c_int,
) -> libc::c_int {
    let mut f: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    head = 0 as libc::c_int;
    *Stack.offset(0 as libc::c_int as isize) = root;
    while head >= 0 as libc::c_int {
        i = *Stack.offset(head as isize);
        if *Child.offset(i as isize) != -(1 as libc::c_int) {
            f = *Child.offset(i as isize);
            while f != -(1 as libc::c_int) {
                head += 1;
                head;
                f = *Sibling.offset(f as isize);
            }
            h = head;
            f = *Child.offset(i as isize);
            while f != -(1 as libc::c_int) {
                let fresh0 = h;
                h = h - 1;
                *Stack.offset(fresh0 as isize) = f;
                f = *Sibling.offset(f as isize);
            }
            *Child.offset(i as isize) = -(1 as libc::c_int);
        } else {
            head -= 1;
            head;
            let fresh1 = k;
            k = k + 1;
            *Order.offset(i as isize) = fresh1;
        }
    }
    return k;
}
