use ::libc;
pub unsafe extern "C" fn _glp_mc13d(
    mut n: libc::c_int,
    mut icn: *const libc::c_int,
    mut ip: *const libc::c_int,
    mut lenr: *const libc::c_int,
    mut ior: *mut libc::c_int,
    mut ib: *mut libc::c_int,
    mut lowl: *mut libc::c_int,
    mut numb: *mut libc::c_int,
    mut prev: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut arp: *mut libc::c_int = ior;
    let mut dummy: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut icnt: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut isn: libc::c_int = 0;
    let mut ist: libc::c_int = 0;
    let mut ist1: libc::c_int = 0;
    let mut iv: libc::c_int = 0;
    let mut iw: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lcnt: libc::c_int = 0;
    let mut nnm1: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut stp: libc::c_int = 0;
    icnt = 0 as libc::c_int;
    num = 0 as libc::c_int;
    nnm1 = n + n - 1 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        *numb.offset(j as isize) = 0 as libc::c_int;
        *arp.offset(j as isize) = *lenr.offset(j as isize) - 1 as libc::c_int;
        j += 1;
        j;
    }
    isn = 1 as libc::c_int;
    's_36: while isn <= n {
        if !(*numb.offset(isn as isize) != 0 as libc::c_int) {
            iv = isn;
            ist = 1 as libc::c_int;
            let ref mut fresh0 = *numb.offset(iv as isize);
            *fresh0 = 1 as libc::c_int;
            *lowl.offset(iv as isize) = *fresh0;
            *ib.offset(n as isize) = iv;
            dummy = 1 as libc::c_int;
            while dummy <= nnm1 {
                i1 = *arp.offset(iv as isize);
                if i1 >= 0 as libc::c_int {
                    i2 = *ip.offset(iv as isize) + *lenr.offset(iv as isize)
                        - 1 as libc::c_int;
                    i1 = i2 - i1;
                    ii = i1;
                    loop {
                        if !(ii <= i2) {
                            current_block = 5783071609795492627;
                            break;
                        }
                        iw = *icn.offset(ii as isize);
                        if *numb.offset(iw as isize) == 0 as libc::c_int {
                            current_block = 17973567135127694340;
                            break;
                        }
                        if *lowl.offset(iw as isize) < *lowl.offset(iv as isize) {
                            *lowl.offset(iv as isize) = *lowl.offset(iw as isize);
                        }
                        ii += 1;
                        ii;
                    }
                    match current_block {
                        17973567135127694340 => {
                            *arp.offset(iv as isize) = i2 - ii - 1 as libc::c_int;
                            *prev.offset(iw as isize) = iv;
                            iv = iw;
                            ist += 1;
                            let ref mut fresh1 = *numb.offset(iv as isize);
                            *fresh1 = ist;
                            *lowl.offset(iv as isize) = *fresh1;
                            *ib.offset((n + 1 as libc::c_int - ist) as isize) = iv;
                            current_block = 1856101646708284338;
                        }
                        _ => {
                            *arp.offset(iv as isize) = -(1 as libc::c_int);
                            current_block = 5634871135123216486;
                        }
                    }
                } else {
                    current_block = 5634871135123216486;
                }
                match current_block {
                    5634871135123216486 => {
                        if !(*lowl.offset(iv as isize) < *numb.offset(iv as isize)) {
                            num += 1;
                            num;
                            ist1 = n + 1 as libc::c_int - ist;
                            lcnt = icnt + 1 as libc::c_int;
                            stp = ist1;
                            while stp <= n {
                                iw = *ib.offset(stp as isize);
                                *lowl.offset(iw as isize) = n + 1 as libc::c_int;
                                icnt += 1;
                                *numb.offset(iw as isize) = icnt;
                                if iw == iv {
                                    break;
                                }
                                stp += 1;
                                stp;
                            }
                            ist = n - stp;
                            *ib.offset(num as isize) = lcnt;
                            if !(ist != 0 as libc::c_int) {
                                if icnt < n {
                                    break;
                                } else {
                                    break 's_36;
                                }
                            }
                        }
                        iw = iv;
                        iv = *prev.offset(iv as isize);
                        if *lowl.offset(iw as isize) < *lowl.offset(iv as isize) {
                            *lowl.offset(iv as isize) = *lowl.offset(iw as isize);
                        }
                    }
                    _ => {}
                }
                dummy += 1;
                dummy;
            }
        }
        isn += 1;
        isn;
    }
    i = 1 as libc::c_int;
    while i <= n {
        *arp.offset(*numb.offset(i as isize) as isize) = i;
        i += 1;
        i;
    }
    return num;
}
