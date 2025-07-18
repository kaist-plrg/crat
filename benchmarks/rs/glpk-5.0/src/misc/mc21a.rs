use ::libc;
pub unsafe extern "C" fn _glp_mc21a(
    mut n: libc::c_int,
    mut icn: *const libc::c_int,
    mut ip: *const libc::c_int,
    mut lenr: *const libc::c_int,
    mut iperm: *mut libc::c_int,
    mut pr: *mut libc::c_int,
    mut arp: *mut libc::c_int,
    mut cv: *mut libc::c_int,
    mut out: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut in1: libc::c_int = 0;
    let mut in2: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut j1: libc::c_int = 0;
    let mut jord: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut numnz: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i <= n {
        *arp.offset(i as isize) = *lenr.offset(i as isize) - 1 as libc::c_int;
        let ref mut fresh0 = *iperm.offset(i as isize);
        *fresh0 = 0 as libc::c_int;
        *cv.offset(i as isize) = *fresh0;
        i += 1;
        i;
    }
    numnz = 0 as libc::c_int;
    jord = 1 as libc::c_int;
    while jord <= n {
        let mut current_block_39: u64;
        j = jord;
        *pr.offset(j as isize) = -(1 as libc::c_int);
        k = 1 as libc::c_int;
        's_43: loop {
            if !(k <= jord) {
                current_block_39 = 7356893052265527703;
                break;
            }
            in1 = *arp.offset(j as isize);
            if in1 >= 0 as libc::c_int {
                in2 = *ip.offset(j as isize) + *lenr.offset(j as isize)
                    - 1 as libc::c_int;
                in1 = in2 - in1;
                ii = in1;
                while ii <= in2 {
                    i = *icn.offset(ii as isize);
                    if *iperm.offset(i as isize) == 0 as libc::c_int {
                        current_block_39 = 7356893052265527703;
                        break 's_43;
                    }
                    ii += 1;
                    ii;
                }
                *arp.offset(j as isize) = -(1 as libc::c_int);
            }
            *out.offset(j as isize) = *lenr.offset(j as isize) - 1 as libc::c_int;
            kk = 1 as libc::c_int;
            's_95: while kk <= jord {
                in1 = *out.offset(j as isize);
                if in1 >= 0 as libc::c_int {
                    in2 = *ip.offset(j as isize) + *lenr.offset(j as isize)
                        - 1 as libc::c_int;
                    in1 = in2 - in1;
                    ii = in1;
                    while ii <= in2 {
                        i = *icn.offset(ii as isize);
                        if *cv.offset(i as isize) != jord {
                            j1 = j;
                            j = *iperm.offset(i as isize);
                            *cv.offset(i as isize) = jord;
                            *pr.offset(j as isize) = j1;
                            *out.offset(j1 as isize) = in2 - ii - 1 as libc::c_int;
                            break 's_95;
                        } else {
                            ii += 1;
                            ii;
                        }
                    }
                }
                j = *pr.offset(j as isize);
                if j == -(1 as libc::c_int) {
                    current_block_39 = 15004371738079956865;
                    break 's_43;
                }
                kk += 1;
                kk;
            }
            k += 1;
            k;
        }
        match current_block_39 {
            7356893052265527703 => {
                *iperm.offset(i as isize) = j;
                *arp.offset(j as isize) = in2 - ii - 1 as libc::c_int;
                numnz += 1;
                numnz;
                k = 1 as libc::c_int;
                while k <= jord {
                    j = *pr.offset(j as isize);
                    if j == -(1 as libc::c_int) {
                        break;
                    }
                    ii = *ip.offset(j as isize) + *lenr.offset(j as isize)
                        - *out.offset(j as isize) - 2 as libc::c_int;
                    i = *icn.offset(ii as isize);
                    *iperm.offset(i as isize) = j;
                    k += 1;
                    k;
                }
            }
            _ => {}
        }
        jord += 1;
        jord;
    }
    if numnz < n {
        i = 1 as libc::c_int;
        while i <= n {
            *arp.offset(i as isize) = 0 as libc::c_int;
            i += 1;
            i;
        }
        k = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= n {
            if *iperm.offset(i as isize) == 0 as libc::c_int {
                k += 1;
                *out.offset(k as isize) = i;
            } else {
                *arp.offset(*iperm.offset(i as isize) as isize) = i;
            }
            i += 1;
            i;
        }
        k = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= n {
            if *arp.offset(i as isize) == 0 as libc::c_int {
                k += 1;
                *iperm.offset(*out.offset(k as isize) as isize) = i;
            }
            i += 1;
            i;
        }
    }
    return numnz;
}
