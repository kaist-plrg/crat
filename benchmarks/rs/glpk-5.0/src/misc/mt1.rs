use ::libc;
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub type integer = libc::c_int;
pub type real = libc::c_float;
unsafe extern "C" fn mt1_(
    mut n: *mut integer,
    mut p: *mut integer,
    mut w: *mut integer,
    mut c__: *mut integer,
    mut z__: *mut integer,
    mut x: *mut integer,
    mut jdim: *mut integer,
    mut jck: *mut integer,
    mut xx: *mut integer,
    mut min__: *mut integer,
    mut psign: *mut integer,
    mut wsign: *mut integer,
    mut zsign: *mut integer,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: integer = 0;
    static mut a: real = 0.;
    static mut b: real = 0.;
    static mut j: integer = 0;
    static mut r__: integer = 0;
    static mut t: integer = 0;
    static mut j1: integer = 0;
    static mut n1: integer = 0;
    static mut ch: integer = 0;
    static mut ii: integer = 0;
    static mut jj: integer = 0;
    static mut kk: integer = 0;
    static mut in_0: integer = 0;
    static mut ll: integer = 0;
    static mut ip: integer = 0;
    static mut nn: integer = 0;
    static mut iu: integer = 0;
    static mut ii1: integer = 0;
    static mut chs: integer = 0;
    static mut lim: integer = 0;
    static mut lim1: integer = 0;
    static mut diff: integer = 0;
    static mut lold: integer = 0;
    static mut mink: integer = 0;
    static mut profit: integer = 0;
    zsign = zsign.offset(-1);
    zsign;
    wsign = wsign.offset(-1);
    wsign;
    psign = psign.offset(-1);
    psign;
    min__ = min__.offset(-1);
    min__;
    xx = xx.offset(-1);
    xx;
    x = x.offset(-1);
    x;
    w = w.offset(-1);
    w;
    p = p.offset(-1);
    p;
    *z__ = 0 as libc::c_int;
    if *jck == 1 as libc::c_int {
        chmt1_(
            n,
            &mut *p.offset(1 as libc::c_int as isize),
            &mut *w.offset(1 as libc::c_int as isize),
            c__,
            z__,
            jdim,
        );
    }
    if *z__ < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ch = *c__;
    ip = 0 as libc::c_int;
    chs = ch;
    i__1 = *n;
    ll = 1 as libc::c_int;
    while ll <= i__1 {
        if *w.offset(ll as isize) > chs {
            break;
        }
        ip += *p.offset(ll as isize);
        chs -= *w.offset(ll as isize);
        ll += 1;
        ll;
    }
    ll -= 1;
    ll;
    if chs == 0 as libc::c_int {
        *z__ = ip;
        i__1 = ll;
        j = 1 as libc::c_int;
        while j <= i__1 {
            *x.offset(j as isize) = 1 as libc::c_int;
            j += 1;
            j;
        }
        nn = ll + 1 as libc::c_int;
        i__1 = *n;
        j = nn;
        while j <= i__1 {
            *x.offset(j as isize) = 0 as libc::c_int;
            j += 1;
            j;
        }
        return 0 as libc::c_int;
    } else {
        *p.offset((*n + 1 as libc::c_int) as isize) = 0 as libc::c_int;
        *w.offset((*n + 1 as libc::c_int) as isize) = ch + 1 as libc::c_int;
        lim = ip
            + chs * *p.offset((ll + 2 as libc::c_int) as isize)
                / *w.offset((ll + 2 as libc::c_int) as isize);
        a = (*w.offset((ll + 1 as libc::c_int) as isize) - chs) as real;
        b = (ip + *p.offset((ll + 1 as libc::c_int) as isize)) as real;
        lim1 = (b - a * *p.offset(ll as isize) as real / *w.offset(ll as isize) as real)
            as integer;
        if lim1 > lim {
            lim = lim1;
        }
        mink = ch + 1 as libc::c_int;
        *min__.offset(*n as isize) = mink;
        i__1 = *n;
        j = 2 as libc::c_int;
        while j <= i__1 {
            kk = *n + 2 as libc::c_int - j;
            if *w.offset(kk as isize) < mink {
                mink = *w.offset(kk as isize);
            }
            *min__.offset((kk - 1 as libc::c_int) as isize) = mink;
            j += 1;
            j;
        }
        i__1 = *n;
        j = 1 as libc::c_int;
        while j <= i__1 {
            *xx.offset(j as isize) = 0 as libc::c_int;
            j += 1;
            j;
        }
        *z__ = 0 as libc::c_int;
        profit = 0 as libc::c_int;
        lold = *n;
        ii = 1 as libc::c_int;
        loop {
            *wsign.offset(ii as isize) = ch - chs;
            *psign.offset(ii as isize) = ip;
            *zsign.offset(ii as isize) = ll + 1 as libc::c_int;
            *xx.offset(ii as isize) = 1 as libc::c_int;
            nn = ll - 1 as libc::c_int;
            if !(nn < ii) {
                i__1 = nn;
                j = ii;
                while j <= i__1 {
                    *wsign
                        .offset(
                            (j + 1 as libc::c_int) as isize,
                        ) = *wsign.offset(j as isize) - *w.offset(j as isize);
                    *psign
                        .offset(
                            (j + 1 as libc::c_int) as isize,
                        ) = *psign.offset(j as isize) - *p.offset(j as isize);
                    *zsign
                        .offset((j + 1 as libc::c_int) as isize) = ll + 1 as libc::c_int;
                    *xx.offset((j + 1 as libc::c_int) as isize) = 1 as libc::c_int;
                    j += 1;
                    j;
                }
            }
            j1 = ll + 1 as libc::c_int;
            i__1 = lold;
            j = j1;
            while j <= i__1 {
                *wsign.offset(j as isize) = 0 as libc::c_int;
                *psign.offset(j as isize) = 0 as libc::c_int;
                *zsign.offset(j as isize) = j;
                j += 1;
                j;
            }
            lold = ll;
            ch = chs;
            profit += ip;
            i__1 = ll - (*n - 2 as libc::c_int);
            if i__1 < 0 as libc::c_int {
                ii = ll + 2 as libc::c_int;
                if ch >= *min__.offset((ii - 1 as libc::c_int) as isize) {
                    current_block = 13272237652882763404;
                } else {
                    current_block = 3898703736766263824;
                }
            } else {
                if i__1 == 0 as libc::c_int {
                    if !(ch < *w.offset(*n as isize)) {
                        ch -= *w.offset(*n as isize);
                        profit += *p.offset(*n as isize);
                        *xx.offset(*n as isize) = 1 as libc::c_int;
                    }
                    ii = *n - 1 as libc::c_int;
                } else {
                    ii = *n;
                }
                current_block = 3898703736766263824;
            }
            match current_block {
                3898703736766263824 => {
                    if !(*z__ >= profit) {
                        *z__ = profit;
                        i__1 = *n;
                        j = 1 as libc::c_int;
                        while j <= i__1 {
                            *x.offset(j as isize) = *xx.offset(j as isize);
                            j += 1;
                            j;
                        }
                        if *z__ == lim {
                            return 0 as libc::c_int;
                        }
                    }
                    if *xx.offset(*n as isize) == 0 as libc::c_int {
                        current_block = 11317925467833373251;
                    } else {
                        *xx.offset(*n as isize) = 0 as libc::c_int;
                        ch += *w.offset(*n as isize);
                        profit -= *p.offset(*n as isize);
                        current_block = 11317925467833373251;
                    }
                }
                _ => {}
            }
            '_L80: loop {
                match current_block {
                    13272237652882763404 => {
                        if *w.offset(ii as isize) <= ch {
                            ip = *psign.offset(ii as isize);
                            chs = ch - *wsign.offset(ii as isize);
                            in_0 = *zsign.offset(ii as isize);
                            i__1 = *n;
                            ll = in_0;
                            loop {
                                if !(ll <= i__1) {
                                    current_block = 1874315696050160458;
                                    break;
                                }
                                if *w.offset(ll as isize) > chs {
                                    current_block = 5320431163020812371;
                                    break;
                                }
                                ip += *p.offset(ll as isize);
                                chs -= *w.offset(ll as isize);
                                ll += 1;
                                ll;
                            }
                            match current_block {
                                1874315696050160458 => {
                                    ll = *n;
                                }
                                _ => {
                                    iu = chs * *p.offset(ll as isize) / *w.offset(ll as isize);
                                    ll -= 1;
                                    ll;
                                    if !(iu == 0 as libc::c_int) {
                                        if *z__ >= profit + ip + iu {
                                            current_block = 11317925467833373251;
                                            continue;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                            if *z__ >= ip + profit {
                                current_block = 11317925467833373251;
                                continue;
                            }
                            *z__ = ip + profit;
                            nn = ii - 1 as libc::c_int;
                            i__1 = nn;
                            j = 1 as libc::c_int;
                            while j <= i__1 {
                                *x.offset(j as isize) = *xx.offset(j as isize);
                                j += 1;
                                j;
                            }
                            i__1 = ll;
                            j = ii;
                            while j <= i__1 {
                                *x.offset(j as isize) = 1 as libc::c_int;
                                j += 1;
                                j;
                            }
                            if !(ll == *n) {
                                nn = ll + 1 as libc::c_int;
                                i__1 = *n;
                                j = nn;
                                while j <= i__1 {
                                    *x.offset(j as isize) = 0 as libc::c_int;
                                    j += 1;
                                    j;
                                }
                            }
                            if *z__ != lim {
                                current_block = 11317925467833373251;
                                continue;
                            }
                            return 0 as libc::c_int;
                        } else {
                            ii1 = ii + 1 as libc::c_int;
                            if *z__
                                >= ch * *p.offset(ii1 as isize) / *w.offset(ii1 as isize)
                                    + profit
                            {
                                current_block = 11317925467833373251;
                                continue;
                            }
                            ii = ii1;
                            current_block = 13272237652882763404;
                        }
                    }
                    _ => {
                        nn = ii - 1 as libc::c_int;
                        if nn == 0 as libc::c_int {
                            return 0 as libc::c_int;
                        }
                        i__1 = nn;
                        j = 1 as libc::c_int;
                        loop {
                            if !(j <= i__1) {
                                current_block = 11957990509374275265;
                                break;
                            }
                            kk = ii - j;
                            if *xx.offset(kk as isize) == 1 as libc::c_int {
                                current_block = 10418260546950894927;
                                break;
                            }
                            j += 1;
                            j;
                        }
                        match current_block {
                            11957990509374275265 => return 0 as libc::c_int,
                            _ => {
                                r__ = ch;
                                ch += *w.offset(kk as isize);
                                profit -= *p.offset(kk as isize);
                                *xx.offset(kk as isize) = 0 as libc::c_int;
                                if r__ < *min__.offset(kk as isize) {
                                    nn = kk + 1 as libc::c_int;
                                    ii = kk;
                                    loop {
                                        if *z__
                                            >= profit
                                                + ch * *p.offset(nn as isize) / *w.offset(nn as isize)
                                        {
                                            current_block = 11317925467833373251;
                                            continue '_L80;
                                        }
                                        diff = *w.offset(nn as isize) - *w.offset(kk as isize);
                                        if diff < 0 as libc::c_int {
                                            t = r__ - diff;
                                            if !(t < *min__.offset(nn as isize)) {
                                                break;
                                            }
                                        } else if !(diff == 0 as libc::c_int) {
                                            if !(diff > r__) {
                                                if !(*z__ >= profit + *p.offset(nn as isize)) {
                                                    *z__ = profit + *p.offset(nn as isize);
                                                    i__1 = kk;
                                                    j = 1 as libc::c_int;
                                                    while j <= i__1 {
                                                        *x.offset(j as isize) = *xx.offset(j as isize);
                                                        j += 1;
                                                        j;
                                                    }
                                                    jj = kk + 1 as libc::c_int;
                                                    i__1 = *n;
                                                    j = jj;
                                                    while j <= i__1 {
                                                        *x.offset(j as isize) = 0 as libc::c_int;
                                                        j += 1;
                                                        j;
                                                    }
                                                    *x.offset(nn as isize) = 1 as libc::c_int;
                                                    if *z__ == lim {
                                                        return 0 as libc::c_int;
                                                    }
                                                    r__ -= diff;
                                                    kk = nn;
                                                    nn += 1;
                                                    nn;
                                                    continue;
                                                }
                                            }
                                        }
                                        nn += 1;
                                        nn;
                                    }
                                    if *z__
                                        >= profit + *p.offset(nn as isize)
                                            + t * *p.offset((nn + 1 as libc::c_int) as isize)
                                                / *w.offset((nn + 1 as libc::c_int) as isize)
                                    {
                                        current_block = 11317925467833373251;
                                        continue;
                                    }
                                    ch -= *w.offset(nn as isize);
                                    profit += *p.offset(nn as isize);
                                    *xx.offset(nn as isize) = 1 as libc::c_int;
                                    ii = nn + 1 as libc::c_int;
                                    *wsign.offset(nn as isize) = *w.offset(nn as isize);
                                    *psign.offset(nn as isize) = *p.offset(nn as isize);
                                    *zsign.offset(nn as isize) = ii;
                                    n1 = nn + 1 as libc::c_int;
                                    i__1 = lold;
                                    j = n1;
                                    while j <= i__1 {
                                        *wsign.offset(j as isize) = 0 as libc::c_int;
                                        *psign.offset(j as isize) = 0 as libc::c_int;
                                        *zsign.offset(j as isize) = j;
                                        j += 1;
                                        j;
                                    }
                                    lold = nn;
                                    current_block = 13272237652882763404;
                                } else {
                                    ii = kk + 1 as libc::c_int;
                                    current_block = 13272237652882763404;
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn chmt1_(
    mut n: *mut integer,
    mut p: *mut integer,
    mut w: *mut integer,
    mut c__: *mut integer,
    mut z__: *mut integer,
    mut jdim: *mut integer,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: integer = 0;
    static mut j: integer = 0;
    static mut r__: real = 0.;
    static mut rr: real = 0.;
    static mut jsw: integer = 0;
    w = w.offset(-1);
    w;
    p = p.offset(-1);
    p;
    if *n >= 2 as libc::c_int && *n <= *jdim - 1 as libc::c_int {
        if *c__ > 0 as libc::c_int {
            jsw = 0 as libc::c_int;
            rr = *p.offset(1 as libc::c_int as isize) as real
                / *w.offset(1 as libc::c_int as isize) as real;
            i__1 = *n;
            j = 1 as libc::c_int;
            loop {
                if !(j <= i__1) {
                    current_block = 17478428563724192186;
                    break;
                }
                r__ = rr;
                if *p.offset(j as isize) <= 0 as libc::c_int {
                    current_block = 74235426558393264;
                    break;
                }
                if *w.offset(j as isize) <= 0 as libc::c_int {
                    current_block = 74235426558393264;
                    break;
                }
                jsw += *w.offset(j as isize);
                if *w.offset(j as isize) <= *c__ {
                    rr = *p.offset(j as isize) as real / *w.offset(j as isize) as real;
                    if rr <= r__ {
                        j += 1;
                        j;
                    } else {
                        *z__ = -(5 as libc::c_int);
                        return 0 as libc::c_int;
                    }
                } else {
                    *z__ = -(3 as libc::c_int);
                    return 0 as libc::c_int;
                }
            }
            match current_block {
                74235426558393264 => {}
                _ => {
                    if jsw > *c__ {
                        return 0 as libc::c_int;
                    }
                    *z__ = -(4 as libc::c_int);
                    return 0 as libc::c_int;
                }
            }
        }
        *z__ = -(2 as libc::c_int);
        return 0 as libc::c_int;
    } else {
        *z__ = -(1 as libc::c_int);
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn _glp_mt1(
    mut n: libc::c_int,
    mut p: *mut libc::c_int,
    mut w: *mut libc::c_int,
    mut c: libc::c_int,
    mut x: *mut libc::c_int,
    mut jck: libc::c_int,
    mut xx: *mut libc::c_int,
    mut min: *mut libc::c_int,
    mut psign: *mut libc::c_int,
    mut wsign: *mut libc::c_int,
    mut zsign: *mut libc::c_int,
) -> libc::c_int {
    let mut z: libc::c_int = 0;
    let mut jdim: libc::c_int = n + 1 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    mt1_(
        &mut n,
        &mut *p.offset(1 as libc::c_int as isize),
        &mut *w.offset(1 as libc::c_int as isize),
        &mut c,
        &mut z,
        &mut *x.offset(1 as libc::c_int as isize),
        &mut jdim,
        &mut jck,
        &mut *xx.offset(1 as libc::c_int as isize),
        &mut *min.offset(1 as libc::c_int as isize),
        &mut *psign.offset(1 as libc::c_int as isize),
        &mut *wsign.offset(1 as libc::c_int as isize),
        &mut *zsign.offset(1 as libc::c_int as isize),
    );
    s2 = 0 as libc::c_int;
    s1 = s2;
    j = 1 as libc::c_int;
    while j <= n {
        (*x.offset(j as isize) == 0 as libc::c_int
            || *x.offset(j as isize) == 1 as libc::c_int
            || {
                glp_assert_(
                    b"x[j] == 0 || x[j] == 1\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                    290 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if *x.offset(j as isize) != 0 {
            s1 += *p.offset(j as isize);
            s2 += *w.offset(j as isize);
        }
        j += 1;
        j;
    }
    (s1 == z
        || {
            glp_assert_(
                b"s1 == z\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                294 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (s2 <= c
        || {
            glp_assert_(
                b"s2 <= c\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
                295 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return z;
}
