use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub unsafe extern "C" fn nsfc_(
    mut n: *mut libc::c_long,
    mut r: *mut libc::c_long,
    mut ic: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut jlmax: *mut libc::c_long,
    mut il: *mut libc::c_long,
    mut jl: *mut libc::c_long,
    mut ijl: *mut libc::c_long,
    mut jumax: *mut libc::c_long,
    mut iu: *mut libc::c_long,
    mut ju: *mut libc::c_long,
    mut iju: *mut libc::c_long,
    mut q: *mut libc::c_long,
    mut ira: *mut libc::c_long,
    mut jra: *mut libc::c_long,
    mut irac: *mut libc::c_long,
    mut irl: *mut libc::c_long,
    mut jrl: *mut libc::c_long,
    mut iru: *mut libc::c_long,
    mut jru: *mut libc::c_long,
    mut flag_: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__2: libc::c_long = 0;
    let mut i__3: libc::c_long = 0;
    let mut cend: libc::c_long = 0;
    let mut irai: libc::c_long = 0;
    let mut rend: libc::c_long = 0;
    let mut jmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut long_: libc::c_long = 0;
    let mut irll: libc::c_long = 0;
    let mut jtmp: libc::c_long = 0;
    let mut irul: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut jaiak: libc::c_long = 0;
    let mut jlmin: libc::c_long = 0;
    let mut lasti: libc::c_long = 0;
    let mut jumin: libc::c_long = 0;
    let mut i1: libc::c_long = 0;
    let mut jlptr: libc::c_long = 0;
    let mut juptr: libc::c_long = 0;
    let mut qm: libc::c_long = 0;
    let mut vj: libc::c_long = 0;
    let mut jairai: libc::c_long = 0;
    let mut lastid: libc::c_long = 0;
    let mut np1: libc::c_long = 0;
    let mut iak: libc::c_long = 0;
    let mut luk: libc::c_long = 0;
    let mut rk: libc::c_long = 0 as libc::c_int as libc::c_long;
    jru = jru.offset(-1);
    jru;
    iru = iru.offset(-1);
    iru;
    jrl = jrl.offset(-1);
    jrl;
    irl = irl.offset(-1);
    irl;
    irac = irac.offset(-1);
    irac;
    jra = jra.offset(-1);
    jra;
    ira = ira.offset(-1);
    ira;
    q = q.offset(-1);
    q;
    iju = iju.offset(-1);
    iju;
    ju = ju.offset(-1);
    ju;
    iu = iu.offset(-1);
    iu;
    ijl = ijl.offset(-1);
    ijl;
    jl = jl.offset(-1);
    jl;
    il = il.offset(-1);
    il;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    ic = ic.offset(-1);
    ic;
    r = r.offset(-1);
    r;
    np1 = *n + 1 as libc::c_int as libc::c_long;
    jlmin = 1 as libc::c_int as libc::c_long;
    jlptr = 0 as libc::c_int as libc::c_long;
    *il.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_long;
    jumin = 1 as libc::c_int as libc::c_long;
    juptr = 0 as libc::c_int as libc::c_long;
    *iu.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_long;
    k = 1 as libc::c_int as libc::c_long;
    while k <= *n {
        *irac.offset(k as isize) = 0 as libc::c_int as libc::c_long;
        *jra.offset(k as isize) = 0 as libc::c_int as libc::c_long;
        *jrl.offset(k as isize) = 0 as libc::c_int as libc::c_long;
        *jru.offset(k as isize) = 0 as libc::c_int as libc::c_long;
        k += 1;
        k;
    }
    k = 1 as libc::c_int as libc::c_long;
    loop {
        if !(k <= *n) {
            current_block = 5689316957504528238;
            break;
        }
        rk = *r.offset(k as isize);
        iak = *ia.offset(rk as isize);
        if iak >= *ia.offset((rk + 1 as libc::c_int as libc::c_long) as isize) {
            current_block = 9913419768449505477;
            break;
        }
        jaiak = *ic.offset(*ja.offset(iak as isize) as isize);
        if jaiak > k {
            current_block = 16222062389406139910;
            break;
        }
        *jra.offset(k as isize) = *irac.offset(jaiak as isize);
        *irac.offset(jaiak as isize) = k;
        *ira.offset(k as isize) = iak;
        k += 1;
        k;
    }
    match current_block {
        5689316957504528238 => {
            k = 1 as libc::c_int as libc::c_long;
            's_151: loop {
                if !(k <= *n) {
                    current_block = 9728093949049737828;
                    break;
                }
                *q.offset(np1 as isize) = np1;
                luk = -(1 as libc::c_int) as libc::c_long;
                vj = *irac.offset(k as isize);
                if vj == 0 as libc::c_int as libc::c_long {
                    current_block = 12546631616664358788;
                } else {
                    current_block = 18263648427008741478;
                }
                loop {
                    match current_block {
                        12546631616664358788 => {
                            lastid = 0 as libc::c_int as libc::c_long;
                            break;
                        }
                        _ => {
                            qm = np1;
                            loop {
                                m = qm;
                                qm = *q.offset(m as isize);
                                if !(qm < vj) {
                                    break;
                                }
                            }
                            if qm == vj {
                                current_block = 8532763467788927469;
                                break 's_151;
                            }
                            luk += 1;
                            luk;
                            *q.offset(m as isize) = vj;
                            *q.offset(vj as isize) = qm;
                            vj = *jra.offset(vj as isize);
                            if vj != 0 as libc::c_int as libc::c_long {
                                current_block = 18263648427008741478;
                            } else {
                                current_block = 12546631616664358788;
                            }
                        }
                    }
                }
                lasti = 0 as libc::c_int as libc::c_long;
                *ijl.offset(k as isize) = jlptr;
                i = k;
                loop {
                    i = *jru.offset(i as isize);
                    if i == 0 as libc::c_int as libc::c_long {
                        break;
                    }
                    qm = np1;
                    jmin = *irl.offset(i as isize);
                    jmax = *ijl.offset(i as isize)
                        + *il.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                        - *il.offset(i as isize) - 1 as libc::c_int as libc::c_long;
                    long_ = jmax - jmin;
                    if long_ < 0 as libc::c_int as libc::c_long {
                        continue;
                    }
                    jtmp = *jl.offset(jmin as isize);
                    if jtmp != k {
                        long_ += 1;
                        long_;
                    }
                    if jtmp == k {
                        *r.offset(i as isize) = -*r.offset(i as isize);
                    }
                    if lastid < long_ {
                        lasti = i;
                        lastid = long_;
                    }
                    j = jmin;
                    while j <= jmax {
                        vj = *jl.offset(j as isize);
                        loop {
                            m = qm;
                            qm = *q.offset(m as isize);
                            if !(qm < vj) {
                                break;
                            }
                        }
                        if qm != vj {
                            luk += 1;
                            luk;
                            *q.offset(m as isize) = vj;
                            *q.offset(vj as isize) = qm;
                            qm = vj;
                        }
                        j += 1;
                        j;
                    }
                }
                qm = *q.offset(np1 as isize);
                if qm != k {
                    current_block = 16222062389406139910;
                    break;
                }
                if !(luk == 0 as libc::c_int as libc::c_long) {
                    if lastid != luk {
                        if jlmin > jlptr {
                            current_block = 9665328865370312925;
                        } else {
                            qm = *q.offset(qm as isize);
                            j = jlmin;
                            loop {
                                if !(j <= jlptr) {
                                    current_block = 9665328865370312925;
                                    break;
                                }
                                i__3 = *jl.offset(j as isize) - qm;
                                if i__3 >= 0 as libc::c_int as libc::c_long {
                                    if i__3 == 0 as libc::c_int as libc::c_long {
                                        current_block = 4535172136950873432;
                                        break;
                                    } else {
                                        current_block = 9665328865370312925;
                                        break;
                                    }
                                } else {
                                    j += 1;
                                    j;
                                }
                            }
                            match current_block {
                                9665328865370312925 => {}
                                _ => {
                                    *ijl.offset(k as isize) = j;
                                    i = j;
                                    loop {
                                        if !(i <= jlptr) {
                                            current_block = 11869735117417356968;
                                            break;
                                        }
                                        if *jl.offset(i as isize) != qm {
                                            current_block = 9665328865370312925;
                                            break;
                                        }
                                        qm = *q.offset(qm as isize);
                                        if qm > *n {
                                            current_block = 7556977570016938016;
                                            break;
                                        }
                                        i += 1;
                                        i;
                                    }
                                    match current_block {
                                        7556977570016938016 => {}
                                        9665328865370312925 => {}
                                        _ => {
                                            jlptr = j - 1 as libc::c_int as libc::c_long;
                                            current_block = 9665328865370312925;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            7556977570016938016 => {}
                            _ => {
                                jlmin = jlptr + 1 as libc::c_int as libc::c_long;
                                *ijl.offset(k as isize) = jlmin;
                                if !(luk == 0 as libc::c_int as libc::c_long) {
                                    jlptr += luk;
                                    if jlptr > *jlmax {
                                        current_block = 8751598319676287042;
                                        break;
                                    }
                                    qm = *q.offset(np1 as isize);
                                    i__2 = jlptr;
                                    j = jlmin;
                                    while j <= i__2 {
                                        qm = *q.offset(qm as isize);
                                        *jl.offset(j as isize) = qm;
                                        j += 1;
                                        j;
                                    }
                                }
                            }
                        }
                    } else {
                        irll = *irl.offset(lasti as isize);
                        *ijl
                            .offset(
                                k as isize,
                            ) = irll + 1 as libc::c_int as libc::c_long;
                        if *jl.offset(irll as isize) != k {
                            let ref mut fresh0 = *ijl.offset(k as isize);
                            *fresh0 -= 1;
                            *fresh0;
                        }
                    }
                }
                *irl.offset(k as isize) = *ijl.offset(k as isize);
                *il
                    .offset(
                        (k + 1 as libc::c_int as libc::c_long) as isize,
                    ) = *il.offset(k as isize) + luk;
                *q.offset(np1 as isize) = np1;
                luk = -(1 as libc::c_int) as libc::c_long;
                rk = *r.offset(k as isize);
                jmin = *ira.offset(k as isize);
                jmax = *ia.offset((rk + 1 as libc::c_int as libc::c_long) as isize)
                    - 1 as libc::c_int as libc::c_long;
                if !(jmin > jmax) {
                    i__2 = jmax;
                    j = jmin;
                    while j <= i__2 {
                        vj = *ic.offset(*ja.offset(j as isize) as isize);
                        qm = np1;
                        loop {
                            m = qm;
                            qm = *q.offset(m as isize);
                            if !(qm < vj) {
                                break;
                            }
                        }
                        if qm == vj {
                            current_block = 8532763467788927469;
                            break 's_151;
                        }
                        luk += 1;
                        luk;
                        *q.offset(m as isize) = vj;
                        *q.offset(vj as isize) = qm;
                        j += 1;
                        j;
                    }
                }
                lastid = 0 as libc::c_int as libc::c_long;
                lasti = 0 as libc::c_int as libc::c_long;
                *iju.offset(k as isize) = juptr;
                i = k;
                i1 = *jrl.offset(k as isize);
                loop {
                    i = i1;
                    if i == 0 as libc::c_int as libc::c_long {
                        break;
                    }
                    i1 = *jrl.offset(i as isize);
                    qm = np1;
                    jmin = *iru.offset(i as isize);
                    jmax = *iju.offset(i as isize)
                        + *iu.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                        - *iu.offset(i as isize) - 1 as libc::c_int as libc::c_long;
                    long_ = jmax - jmin;
                    if long_ < 0 as libc::c_int as libc::c_long {
                        continue;
                    }
                    jtmp = *ju.offset(jmin as isize);
                    if !(jtmp == k) {
                        long_ += 1;
                        long_;
                        cend = *ijl.offset(i as isize)
                            + *il.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                            - *il.offset(i as isize);
                        let ref mut fresh1 = *irl.offset(i as isize);
                        *fresh1 += 1;
                        *fresh1;
                        if !(*irl.offset(i as isize) >= cend) {
                            j = *jl.offset(*irl.offset(i as isize) as isize);
                            *jrl.offset(i as isize) = *jrl.offset(j as isize);
                            *jrl.offset(j as isize) = i;
                        }
                    }
                    if !(lastid >= long_) {
                        lasti = i;
                        lastid = long_;
                    }
                    i__2 = jmax;
                    j = jmin;
                    while j <= i__2 {
                        vj = *ju.offset(j as isize);
                        loop {
                            m = qm;
                            qm = *q.offset(m as isize);
                            if !(qm < vj) {
                                break;
                            }
                        }
                        if !(qm == vj) {
                            luk += 1;
                            luk;
                            *q.offset(m as isize) = vj;
                            *q.offset(vj as isize) = qm;
                            qm = vj;
                        }
                        j += 1;
                        j;
                    }
                }
                if !(*il.offset((k + 1 as libc::c_int as libc::c_long) as isize)
                    <= *il.offset(k as isize))
                {
                    j = *jl.offset(*irl.offset(k as isize) as isize);
                    *jrl.offset(k as isize) = *jrl.offset(j as isize);
                    *jrl.offset(j as isize) = k;
                }
                qm = *q.offset(np1 as isize);
                if qm != k {
                    current_block = 16222062389406139910;
                    break;
                }
                if !(luk == 0 as libc::c_int as libc::c_long) {
                    if lastid != luk {
                        if jumin > juptr {
                            current_block = 12326984230270240492;
                        } else {
                            qm = *q.offset(qm as isize);
                            j = jumin;
                            loop {
                                if !(j <= juptr) {
                                    current_block = 12326984230270240492;
                                    break;
                                }
                                i__3 = *ju.offset(j as isize) - qm;
                                if i__3 < 0 as libc::c_int as libc::c_long {
                                    j += 1;
                                    j;
                                } else if i__3 == 0 as libc::c_int as libc::c_long {
                                    current_block = 4480436065447190508;
                                    break;
                                } else {
                                    current_block = 12326984230270240492;
                                    break;
                                }
                            }
                            match current_block {
                                12326984230270240492 => {}
                                _ => {
                                    *iju.offset(k as isize) = j;
                                    i = j;
                                    loop {
                                        if !(i <= juptr) {
                                            current_block = 13718575627189773797;
                                            break;
                                        }
                                        if *ju.offset(i as isize) != qm {
                                            current_block = 12326984230270240492;
                                            break;
                                        }
                                        qm = *q.offset(qm as isize);
                                        if qm > *n {
                                            current_block = 13562299916484619002;
                                            break;
                                        }
                                        i += 1;
                                        i;
                                    }
                                    match current_block {
                                        13562299916484619002 => {}
                                        12326984230270240492 => {}
                                        _ => {
                                            juptr = j - 1 as libc::c_int as libc::c_long;
                                            current_block = 12326984230270240492;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            13562299916484619002 => {}
                            _ => {
                                jumin = juptr + 1 as libc::c_int as libc::c_long;
                                *iju.offset(k as isize) = jumin;
                                if !(luk == 0 as libc::c_int as libc::c_long) {
                                    juptr += luk;
                                    if juptr > *jumax {
                                        current_block = 1944307598241570661;
                                        break;
                                    }
                                    qm = *q.offset(np1 as isize);
                                    j = jumin;
                                    while j <= juptr {
                                        qm = *q.offset(qm as isize);
                                        *ju.offset(j as isize) = qm;
                                        j += 1;
                                        j;
                                    }
                                }
                            }
                        }
                    } else {
                        irul = *iru.offset(lasti as isize);
                        *iju
                            .offset(
                                k as isize,
                            ) = irul + 1 as libc::c_int as libc::c_long;
                        if *ju.offset(irul as isize) != k {
                            let ref mut fresh2 = *iju.offset(k as isize);
                            *fresh2 -= 1;
                            *fresh2;
                        }
                    }
                }
                *iru.offset(k as isize) = *iju.offset(k as isize);
                *iu
                    .offset(
                        (k + 1 as libc::c_int as libc::c_long) as isize,
                    ) = *iu.offset(k as isize) + luk;
                i = k;
                loop {
                    i1 = *jru.offset(i as isize);
                    if *r.offset(i as isize) < 0 as libc::c_int as libc::c_long {
                        *r.offset(i as isize) = -*r.offset(i as isize);
                    } else {
                        rend = *iju.offset(i as isize)
                            + *iu.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                            - *iu.offset(i as isize);
                        if !(*iru.offset(i as isize) >= rend) {
                            j = *ju.offset(*iru.offset(i as isize) as isize);
                            *jru.offset(i as isize) = *jru.offset(j as isize);
                            *jru.offset(j as isize) = i;
                        }
                    }
                    i = i1;
                    if i == 0 as libc::c_int as libc::c_long {
                        break;
                    }
                    let ref mut fresh3 = *iru.offset(i as isize);
                    *fresh3 += 1;
                    *fresh3;
                }
                i = *irac.offset(k as isize);
                if i == 0 as libc::c_int as libc::c_long {
                    current_block = 17788412896529399552;
                } else {
                    current_block = 16258209370360230652;
                }
                loop {
                    match current_block {
                        17788412896529399552 => {
                            k += 1;
                            k;
                            break;
                        }
                        _ => {
                            i1 = *jra.offset(i as isize);
                            let ref mut fresh4 = *ira.offset(i as isize);
                            *fresh4 += 1;
                            *fresh4;
                            if !(*ira.offset(i as isize)
                                >= *ia
                                    .offset(
                                        (*r.offset(i as isize) + 1 as libc::c_int as libc::c_long)
                                            as isize,
                                    ))
                            {
                                irai = *ira.offset(i as isize);
                                jairai = *ic.offset(*ja.offset(irai as isize) as isize);
                                if !(jairai > i) {
                                    *jra.offset(i as isize) = *irac.offset(jairai as isize);
                                    *irac.offset(jairai as isize) = i;
                                }
                            }
                            i = i1;
                            if i != 0 as libc::c_int as libc::c_long {
                                current_block = 16258209370360230652;
                            } else {
                                current_block = 17788412896529399552;
                            }
                        }
                    }
                }
            }
            match current_block {
                16222062389406139910 => {}
                _ => {
                    match current_block {
                        1944307598241570661 => {
                            *flag_ = *n * 6 as libc::c_int as libc::c_long + k;
                            return 0 as libc::c_int;
                        }
                        8532763467788927469 => {
                            *flag_ = (*n << 1 as libc::c_int) + rk;
                            return 0 as libc::c_int;
                        }
                        8751598319676287042 => {
                            *flag_ = *n * 3 as libc::c_int as libc::c_long + k;
                            return 0 as libc::c_int;
                        }
                        _ => {
                            *ijl.offset(*n as isize) = jlptr;
                            *iju.offset(*n as isize) = juptr;
                            *flag_ = 0 as libc::c_int as libc::c_long;
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
        9913419768449505477 => {
            *flag_ = *n + rk;
            return 0 as libc::c_int;
        }
        _ => {}
    }
    *flag_ = *n * 5 as libc::c_int as libc::c_long + k;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn nroc_(
    mut n: *mut libc::c_long,
    mut ic: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut a: *mut libc::c_double,
    mut jar: *mut libc::c_long,
    mut ar: *mut libc::c_double,
    mut p: *mut libc::c_long,
    mut flag_: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    let mut jmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut newj: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    p = p.offset(-1);
    p;
    ar = ar.offset(-1);
    ar;
    jar = jar.offset(-1);
    jar;
    a = a.offset(-1);
    a;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    ic = ic.offset(-1);
    ic;
    i__1 = *n;
    k = 1 as libc::c_int as libc::c_long;
    's_30: loop {
        if !(k <= i__1) {
            current_block = 4488286894823169796;
            break;
        }
        jmin = *ia.offset(k as isize);
        jmax = *ia.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        if !(jmin > jmax) {
            *p
                .offset(
                    (*n + 1 as libc::c_int as libc::c_long) as isize,
                ) = *n + 1 as libc::c_int as libc::c_long;
            i__2 = jmax;
            j = jmin;
            while j <= i__2 {
                newj = *ic.offset(*ja.offset(j as isize) as isize);
                i = *n + 1 as libc::c_int as libc::c_long;
                while !(*p.offset(i as isize) >= newj) {
                    i = *p.offset(i as isize);
                }
                if *p.offset(i as isize) == newj {
                    current_block = 14702335084483398507;
                    break 's_30;
                }
                *p.offset(newj as isize) = *p.offset(i as isize);
                *p.offset(i as isize) = newj;
                *jar.offset(newj as isize) = *ja.offset(j as isize);
                *ar.offset(newj as isize) = *a.offset(j as isize);
                j += 1;
                j;
            }
            i = *n + 1 as libc::c_int as libc::c_long;
            i__2 = jmax;
            j = jmin;
            while j <= i__2 {
                i = *p.offset(i as isize);
                *ja.offset(j as isize) = *jar.offset(i as isize);
                *a.offset(j as isize) = *ar.offset(i as isize);
                j += 1;
                j;
            }
        }
        k += 1;
        k;
    }
    match current_block {
        14702335084483398507 => {
            *flag_ = *n + k;
            return 0 as libc::c_int;
        }
        _ => {
            *flag_ = 0 as libc::c_int as libc::c_long;
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn odrv_(
    mut n: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut a: *mut libc::c_double,
    mut p: *mut libc::c_long,
    mut ip: *mut libc::c_long,
    mut nsp: *mut libc::c_long,
    mut isp: *mut libc::c_long,
    mut path: libc::c_long,
    mut flag_: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut head: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut dflag: libc::c_long = 0;
    let mut q: libc::c_long = 0;
    let mut v: libc::c_long = 0;
    let mut max_: libc::c_long = 0;
    let mut tmp: libc::c_long = 0;
    #[export_name = "sro_"]
    pub unsafe extern "C" fn sro__0(
        mut n_0: *mut libc::c_long,
        mut ip_0: *mut libc::c_long,
        mut ia_0: *mut libc::c_long,
        mut ja_0: *mut libc::c_long,
        mut a_0: *mut libc::c_double,
        mut q_0: *mut libc::c_long,
        mut r: *mut libc::c_long,
        mut dflag_0: *mut libc::c_long,
    ) -> libc::c_int {
        let mut i__1: libc::c_long = 0;
        let mut i__2: libc::c_long = 0;
        let mut jmin: libc::c_long = 0;
        let mut jmax: libc::c_long = 0;
        let mut i: libc::c_long = 0;
        let mut j: libc::c_long = 0;
        let mut k: libc::c_long = 0;
        let mut ilast: libc::c_long = 0;
        let mut ak: libc::c_double = 0.;
        let mut jdummy: libc::c_long = 0;
        let mut jak: libc::c_long = 0;
        r = r.offset(-1);
        r;
        q_0 = q_0.offset(-1);
        q_0;
        a_0 = a_0.offset(-1);
        a_0;
        ja_0 = ja_0.offset(-1);
        ja_0;
        ia_0 = ia_0.offset(-1);
        ia_0;
        ip_0 = ip_0.offset(-1);
        ip_0;
        i__1 = *n_0;
        i = 1 as libc::c_int as libc::c_long;
        while i <= i__1 {
            *q_0.offset(i as isize) = 0 as libc::c_int as libc::c_long;
            i += 1;
            i;
        }
        i__1 = *n_0;
        i = 1 as libc::c_int as libc::c_long;
        while i <= i__1 {
            jmin = *ia_0.offset(i as isize);
            jmax = *ia_0.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                - 1 as libc::c_int as libc::c_long;
            if !(jmin > jmax) {
                i__2 = jmax;
                j = jmin;
                while j <= i__2 {
                    k = *ja_0.offset(j as isize);
                    if *ip_0.offset(k as isize) < *ip_0.offset(i as isize) {
                        *ja_0.offset(j as isize) = i;
                    }
                    if *ip_0.offset(k as isize) >= *ip_0.offset(i as isize) {
                        k = i;
                    }
                    *r.offset(j as isize) = k;
                    let ref mut fresh5 = *q_0.offset(k as isize);
                    *fresh5 += 1;
                    *fresh5;
                    j += 1;
                    j;
                }
            }
            i += 1;
            i;
        }
        i__1 = *n_0;
        i = 1 as libc::c_int as libc::c_long;
        while i <= i__1 {
            *ia_0
                .offset(
                    (i + 1 as libc::c_int as libc::c_long) as isize,
                ) = *ia_0.offset(i as isize) + *q_0.offset(i as isize);
            *q_0
                .offset(
                    i as isize,
                ) = *ia_0.offset((i + 1 as libc::c_int as libc::c_long) as isize);
            i += 1;
            i;
        }
        ilast = 0 as libc::c_int as libc::c_long;
        jmin = *ia_0.offset(1 as libc::c_int as isize);
        jmax = *ia_0.offset((*n_0 + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        j = jmax;
        i__1 = jmax;
        jdummy = jmin;
        while jdummy <= i__1 {
            i = *r.offset(j as isize);
            if *dflag_0 == 0 || *ja_0.offset(j as isize) != i || i == ilast {
                let ref mut fresh6 = *q_0.offset(i as isize);
                *fresh6 -= 1;
                *fresh6;
                *r.offset(j as isize) = *q_0.offset(i as isize);
            } else {
                *r.offset(j as isize) = *ia_0.offset(i as isize);
                ilast = i;
            }
            j -= 1;
            j;
            jdummy += 1;
            jdummy;
        }
        i__1 = jmax;
        j = jmin;
        while j <= i__1 {
            while !(*r.offset(j as isize) == j) {
                k = *r.offset(j as isize);
                *r.offset(j as isize) = *r.offset(k as isize);
                *r.offset(k as isize) = k;
                jak = *ja_0.offset(k as isize);
                *ja_0.offset(k as isize) = *ja_0.offset(j as isize);
                *ja_0.offset(j as isize) = jak;
                ak = *a_0.offset(k as isize);
                *a_0.offset(k as isize) = *a_0.offset(j as isize);
                *a_0.offset(j as isize) = ak;
            }
            j += 1;
            j;
        }
        return 0 as libc::c_int;
    }
    isp = isp.offset(-1);
    isp;
    ip = ip.offset(-1);
    ip;
    p = p.offset(-1);
    p;
    a = a.offset(-1);
    a;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    *flag_ = 0 as libc::c_int as libc::c_long;
    if path < 1 as libc::c_int as libc::c_long
        || (5 as libc::c_int as libc::c_long) < path
    {
        *flag_ = *n * 11 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        return 0 as libc::c_int;
    } else {
        if (path - 1 as libc::c_int as libc::c_long)
            * (path - 2 as libc::c_int as libc::c_long)
            * (path - 4 as libc::c_int as libc::c_long)
            != 0 as libc::c_int as libc::c_long
        {
            current_block = 9230604390755232537;
        } else {
            max_ = (*nsp - *n) / 2 as libc::c_int as libc::c_long;
            v = 1 as libc::c_int as libc::c_long;
            l = v + max_;
            head = l + max_;
            if max_ < *n {
                current_block = 18025120349589395083;
            } else {
                md_(
                    n,
                    &mut *ia.offset(1 as libc::c_int as isize),
                    &mut *ja.offset(1 as libc::c_int as isize),
                    &mut max_,
                    &mut *isp.offset(v as isize),
                    &mut *isp.offset(l as isize),
                    &mut *isp.offset(head as isize),
                    &mut *p.offset(1 as libc::c_int as isize),
                    &mut *ip.offset(1 as libc::c_int as isize),
                    &mut *isp.offset(v as isize),
                    flag_,
                );
                if *flag_ != 0 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int
                } else {
                    current_block = 9230604390755232537;
                }
            }
        }
        match current_block {
            9230604390755232537 => {
                if (path - 2 as libc::c_int as libc::c_long)
                    * (path - 3 as libc::c_int as libc::c_long)
                    * (path - 4 as libc::c_int as libc::c_long)
                    * (path - 5 as libc::c_int as libc::c_long)
                    != 0 as libc::c_int as libc::c_long
                {
                    current_block = 3169937384973398221;
                } else {
                    tmp = *nsp + 1 as libc::c_int as libc::c_long - *n;
                    q = tmp
                        - (*ia.offset((*n + 1 as libc::c_int as libc::c_long) as isize)
                            - 1 as libc::c_int as libc::c_long);
                    if q < 1 as libc::c_int as libc::c_long {
                        current_block = 18025120349589395083;
                    } else {
                        dflag = (path == 4 as libc::c_int as libc::c_long
                            || path == 5 as libc::c_int as libc::c_long) as libc::c_int
                            as libc::c_long;
                        sro__0(
                            n,
                            &mut *ip.offset(1 as libc::c_int as isize),
                            &mut *ia.offset(1 as libc::c_int as isize),
                            &mut *ja.offset(1 as libc::c_int as isize),
                            &mut *a.offset(1 as libc::c_int as isize),
                            &mut *isp.offset(tmp as isize),
                            &mut *isp.offset(q as isize),
                            &mut dflag,
                        );
                        current_block = 3169937384973398221;
                    }
                }
                match current_block {
                    18025120349589395083 => {}
                    _ => return 0 as libc::c_int,
                }
            }
            _ => {}
        }
        *flag_ = *n * 10 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        return 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn md_(
    mut n: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut max_: *mut libc::c_long,
    mut v: *mut libc::c_long,
    mut l: *mut libc::c_long,
    mut head: *mut libc::c_long,
    mut last: *mut libc::c_long,
    mut next: *mut libc::c_long,
    mut mark: *mut libc::c_long,
    mut flag_: *mut libc::c_long,
) -> libc::c_int {
    let mut i__1: libc::c_long = 0;
    static mut equiv_0: [libc::c_long; 1] = [0; 1];
    let mut dmin_: libc::c_long = 0;
    let mut tail: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut tag: libc::c_long = 0;
    mark = mark.offset(-1);
    mark;
    next = next.offset(-1);
    next;
    last = last.offset(-1);
    last;
    head = head.offset(-1);
    head;
    l = l.offset(-1);
    l;
    v = v.offset(-1);
    v;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    tag = 0 as libc::c_int as libc::c_long;
    mdi_(
        n,
        &mut *ia.offset(1 as libc::c_int as isize),
        &mut *ja.offset(1 as libc::c_int as isize),
        max_,
        &mut *v.offset(1 as libc::c_int as isize),
        &mut *l.offset(1 as libc::c_int as isize),
        &mut *head.offset(1 as libc::c_int as isize),
        &mut *last.offset(1 as libc::c_int as isize),
        &mut *next.offset(1 as libc::c_int as isize),
        &mut *mark.offset(1 as libc::c_int as isize),
        &mut tag,
        flag_,
    );
    if *flag_ != 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    k = 0 as libc::c_int as libc::c_long;
    dmin_ = 1 as libc::c_int as libc::c_long;
    while !(k >= *n) {
        while !(*head.offset(dmin_ as isize) > 0 as libc::c_int as libc::c_long) {
            dmin_ += 1;
            dmin_;
        }
        *equiv_0.as_mut_ptr() = *head.offset(dmin_ as isize);
        *head.offset(dmin_ as isize) = *next.offset(*equiv_0.as_mut_ptr() as isize);
        if *head.offset(dmin_ as isize) > 0 as libc::c_int as libc::c_long {
            *last.offset(*head.offset(dmin_ as isize) as isize) = -dmin_;
        }
        k += 1;
        k;
        *next.offset(*equiv_0.as_mut_ptr() as isize) = -k;
        *last
            .offset(
                *equiv_0.as_mut_ptr() as isize,
            ) = dmin_ - 1 as libc::c_int as libc::c_long;
        tag += *last.offset(*equiv_0.as_mut_ptr() as isize);
        *mark.offset(*equiv_0.as_mut_ptr() as isize) = tag;
        mdm_(
            equiv_0.as_mut_ptr(),
            &mut tail,
            &mut *v.offset(1 as libc::c_int as isize),
            &mut *l.offset(1 as libc::c_int as isize),
            &mut *last.offset(1 as libc::c_int as isize),
            &mut *next.offset(1 as libc::c_int as isize),
            &mut *mark.offset(1 as libc::c_int as isize),
        );
        mdp_(
            &mut k,
            equiv_0.as_mut_ptr(),
            &mut tail,
            &mut *v.offset(1 as libc::c_int as isize),
            &mut *l.offset(1 as libc::c_int as isize),
            &mut *head.offset(1 as libc::c_int as isize),
            &mut *last.offset(1 as libc::c_int as isize),
            &mut *next.offset(1 as libc::c_int as isize),
            &mut *mark.offset(1 as libc::c_int as isize),
        );
        mdu_(
            equiv_0.as_mut_ptr(),
            &mut dmin_,
            &mut *v.offset(1 as libc::c_int as isize),
            &mut *l.offset(1 as libc::c_int as isize),
            &mut *head.offset(1 as libc::c_int as isize),
            &mut *last.offset(1 as libc::c_int as isize),
            &mut *next.offset(1 as libc::c_int as isize),
            &mut *mark.offset(1 as libc::c_int as isize),
        );
    }
    i__1 = *n;
    k = 1 as libc::c_int as libc::c_long;
    while k <= i__1 {
        *next.offset(k as isize) = -*next.offset(k as isize);
        *last.offset(*next.offset(k as isize) as isize) = k;
        k += 1;
        k;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdu_(
    mut ek_0: *mut libc::c_long,
    mut dmin_: *mut libc::c_long,
    mut v: *mut libc::c_long,
    mut l: *mut libc::c_long,
    mut head: *mut libc::c_long,
    mut last: *mut libc::c_long,
    mut next: *mut libc::c_long,
    mut mark: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    static mut equiv_0: [libc::c_long; 1] = [0; 1];
    let mut b: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut vb: libc::c_long = 0;
    let mut vi: libc::c_long = 0;
    let mut blpmax: libc::c_long = 0;
    let mut ilpmax: libc::c_long = 0;
    let mut tag: libc::c_long = 0;
    let mut blp: libc::c_long = 0;
    let mut dvi: libc::c_long = 0;
    let mut evi: libc::c_long = 0;
    let mut ilp: libc::c_long = 0;
    mark = mark.offset(-1);
    mark;
    next = next.offset(-1);
    next;
    last = last.offset(-1);
    last;
    head = head.offset(-1);
    head;
    l = l.offset(-1);
    l;
    v = v.offset(-1);
    v;
    tag = *mark.offset(*ek_0 as isize) - *last.offset(*ek_0 as isize);
    i = *ek_0;
    ilpmax = *last.offset(*ek_0 as isize);
    if !(ilpmax <= 0 as libc::c_int as libc::c_long) {
        i__1 = ilpmax;
        ilp = 1 as libc::c_int as libc::c_long;
        while ilp <= i__1 {
            i = *l.offset(i as isize);
            vi = *v.offset(i as isize);
            i__2 = *last.offset(vi as isize);
            if i__2 < 0 as libc::c_int as libc::c_long {
                tag += 1;
                tag;
                dvi = *last.offset(*ek_0 as isize);
                s = *l.offset(vi as isize);
                loop {
                    s = *l.offset(s as isize);
                    if s == 0 as libc::c_int as libc::c_long {
                        current_block = 13172515638809264422;
                        break;
                    }
                    *equiv_0.as_mut_ptr() = *v.offset(s as isize);
                    if *next.offset(*equiv_0.as_mut_ptr() as isize)
                        < 0 as libc::c_int as libc::c_long
                    {
                        if *mark.offset(*equiv_0.as_mut_ptr() as isize)
                            < 0 as libc::c_int as libc::c_long
                        {
                            *last.offset(vi as isize) = 0 as libc::c_int as libc::c_long;
                            let ref mut fresh7 = *mark
                                .offset(*equiv_0.as_mut_ptr() as isize);
                            *fresh7 -= 1;
                            *fresh7;
                            current_block = 2011660552326064184;
                            break;
                        } else {
                            b = *equiv_0.as_mut_ptr();
                            blpmax = *last.offset(*equiv_0.as_mut_ptr() as isize);
                            i__2 = blpmax;
                            blp = 1 as libc::c_int as libc::c_long;
                            while blp <= i__2 {
                                b = *l.offset(b as isize);
                                vb = *v.offset(b as isize);
                                if !(*mark.offset(vb as isize) >= tag) {
                                    *mark.offset(vb as isize) = tag;
                                    dvi += 1;
                                    dvi;
                                }
                                blp += 1;
                                blp;
                            }
                        }
                    } else {
                        *mark.offset(*equiv_0.as_mut_ptr() as isize) = tag;
                        dvi += 1;
                        dvi;
                    }
                }
                match current_block {
                    13172515638809264422 => {}
                    _ => {
                        loop {
                            s = *l.offset(s as isize);
                            if s == 0 as libc::c_int as libc::c_long {
                                break;
                            }
                            *equiv_0.as_mut_ptr() = *v.offset(s as isize);
                            if *mark.offset(*equiv_0.as_mut_ptr() as isize)
                                < 0 as libc::c_int as libc::c_long
                            {
                                let ref mut fresh8 = *mark
                                    .offset(*equiv_0.as_mut_ptr() as isize);
                                *fresh8 -= 1;
                                *fresh8;
                            }
                        }
                        current_block = 11812396948646013369;
                    }
                }
            } else if i__2 == 0 as libc::c_int as libc::c_long {
                current_block = 11812396948646013369;
            } else {
                evi = *last.offset(vi as isize);
                dvi = *last.offset(*ek_0 as isize) + *last.offset(evi as isize)
                    + *mark.offset(evi as isize);
                *mark.offset(evi as isize) = 0 as libc::c_int as libc::c_long;
                current_block = 13172515638809264422;
            }
            match current_block {
                13172515638809264422 => {
                    *next.offset(vi as isize) = *head.offset(dvi as isize);
                    *head.offset(dvi as isize) = vi;
                    *last.offset(vi as isize) = -dvi;
                    if *next.offset(vi as isize) > 0 as libc::c_int as libc::c_long {
                        *last.offset(*next.offset(vi as isize) as isize) = vi;
                    }
                    if dvi < *dmin_ {
                        *dmin_ = dvi;
                    }
                }
                _ => {}
            }
            ilp += 1;
            ilp;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdp_(
    mut k: *mut libc::c_long,
    mut ek_0: *mut libc::c_long,
    mut tail: *mut libc::c_long,
    mut v: *mut libc::c_long,
    mut l: *mut libc::c_long,
    mut head: *mut libc::c_long,
    mut last: *mut libc::c_long,
    mut next: *mut libc::c_long,
    mut mark: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut li: libc::c_long = 0;
    let mut es_1: libc::c_long = 0;
    let mut vi: libc::c_long = 0;
    let mut ls: libc::c_long = 0;
    let mut ilpmax: libc::c_long = 0;
    let mut tag: libc::c_long = 0;
    let mut evi: libc::c_long = 0;
    let mut ilp: libc::c_long = 0;
    let mut lvi: libc::c_long = 0;
    let mut free: libc::c_long = 0 as libc::c_int as libc::c_long;
    mark = mark.offset(-1);
    mark;
    next = next.offset(-1);
    next;
    last = last.offset(-1);
    last;
    head = head.offset(-1);
    head;
    l = l.offset(-1);
    l;
    v = v.offset(-1);
    v;
    tag = *mark.offset(*ek_0 as isize);
    li = *ek_0;
    ilpmax = *last.offset(*ek_0 as isize);
    if !(ilpmax <= 0 as libc::c_int as libc::c_long) {
        i__1 = ilpmax;
        ilp = 1 as libc::c_int as libc::c_long;
        while ilp <= i__1 {
            i = li;
            li = *l.offset(i as isize);
            vi = *v.offset(li as isize);
            if !(*last.offset(vi as isize) == 0 as libc::c_int as libc::c_long) {
                if *last.offset(vi as isize) > 0 as libc::c_int as libc::c_long {
                    *next
                        .offset(
                            *last.offset(vi as isize) as isize,
                        ) = *next.offset(vi as isize);
                } else {
                    *head
                        .offset(
                            -*last.offset(vi as isize) as isize,
                        ) = *next.offset(vi as isize);
                }
                if *next.offset(vi as isize) > 0 as libc::c_int as libc::c_long {
                    *last
                        .offset(
                            *next.offset(vi as isize) as isize,
                        ) = *last.offset(vi as isize);
                }
            }
            ls = vi;
            loop {
                s = ls;
                ls = *l.offset(s as isize);
                if ls == 0 as libc::c_int as libc::c_long {
                    break;
                }
                es_1 = *v.offset(ls as isize);
                if *mark.offset(es_1 as isize) < tag {
                    continue;
                }
                free = ls;
                *l.offset(s as isize) = *l.offset(ls as isize);
                ls = s;
            }
            lvi = *l.offset(vi as isize);
            if lvi != 0 as libc::c_int as libc::c_long {
                if *l.offset(lvi as isize) != 0 as libc::c_int as libc::c_long {
                    current_block = 12027707678851985974;
                } else {
                    evi = *v.offset(lvi as isize);
                    if *next.offset(evi as isize) >= 0 as libc::c_int as libc::c_long {
                        current_block = 12027707678851985974;
                    } else {
                        if *mark.offset(evi as isize) < 0 as libc::c_int as libc::c_long
                        {
                            *last.offset(vi as isize) = 0 as libc::c_int as libc::c_long;
                            let ref mut fresh10 = *mark.offset(evi as isize);
                            *fresh10 -= 1;
                            *fresh10;
                        } else {
                            *last.offset(vi as isize) = evi;
                            *mark
                                .offset(evi as isize) = -(1 as libc::c_int) as libc::c_long;
                            *l.offset(*tail as isize) = li;
                            *tail = li;
                            *l.offset(i as isize) = *l.offset(li as isize);
                            li = i;
                        }
                        current_block = 12799175880562102251;
                    }
                }
                match current_block {
                    12027707678851985974 => {
                        *last.offset(vi as isize) = -*ek_0;
                    }
                    _ => {}
                }
                *v.offset(free as isize) = *ek_0;
                *l.offset(free as isize) = *l.offset(vi as isize);
                *l.offset(vi as isize) = free;
            } else {
                *l.offset(i as isize) = *l.offset(li as isize);
                li = i;
                *k += 1;
                *k;
                *next.offset(vi as isize) = -*k;
                let ref mut fresh9 = *last.offset(*ek_0 as isize);
                *fresh9 -= 1;
                *fresh9;
            }
            ilp += 1;
            ilp;
        }
    }
    *l.offset(*tail as isize) = 0 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdm_(
    mut vk_0: *mut libc::c_long,
    mut tail: *mut libc::c_long,
    mut v: *mut libc::c_long,
    mut l: *mut libc::c_long,
    mut last: *mut libc::c_long,
    mut next: *mut libc::c_long,
    mut mark: *mut libc::c_long,
) -> libc::c_int {
    let mut i__1: libc::c_long = 0;
    static mut equiv_0: [libc::c_long; 1] = [0; 1];
    let mut b: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut lb: libc::c_long = 0;
    let mut vb: libc::c_long = 0;
    let mut ls: libc::c_long = 0;
    let mut blpmax: libc::c_long = 0;
    let mut tag: libc::c_long = 0;
    let mut blp: libc::c_long = 0;
    mark = mark.offset(-1);
    mark;
    next = next.offset(-1);
    next;
    last = last.offset(-1);
    last;
    l = l.offset(-1);
    l;
    v = v.offset(-1);
    v;
    tag = *mark.offset(*vk_0 as isize);
    *tail = *vk_0;
    ls = *l.offset(*vk_0 as isize);
    loop {
        s = ls;
        if s == 0 as libc::c_int as libc::c_long {
            break;
        }
        ls = *l.offset(s as isize);
        *equiv_0.as_mut_ptr() = *v.offset(s as isize);
        if *next.offset(*equiv_0.as_mut_ptr() as isize)
            < 0 as libc::c_int as libc::c_long
        {
            lb = *l.offset(*equiv_0.as_mut_ptr() as isize);
            blpmax = *last.offset(*equiv_0.as_mut_ptr() as isize);
            i__1 = blpmax;
            blp = 1 as libc::c_int as libc::c_long;
            while blp <= i__1 {
                b = lb;
                lb = *l.offset(b as isize);
                vb = *v.offset(b as isize);
                if !(*mark.offset(vb as isize) >= tag) {
                    *mark.offset(vb as isize) = tag;
                    *l.offset(*tail as isize) = b;
                    *tail = b;
                }
                blp += 1;
                blp;
            }
            *mark.offset(*equiv_0.as_mut_ptr() as isize) = tag;
        } else {
            *mark.offset(*equiv_0.as_mut_ptr() as isize) = tag;
            *l.offset(*tail as isize) = s;
            *tail = s;
        }
    }
    *l.offset(*tail as isize) = 0 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdi_(
    mut n: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut max_: *mut libc::c_long,
    mut v: *mut libc::c_long,
    mut l: *mut libc::c_long,
    mut head: *mut libc::c_long,
    mut last: *mut libc::c_long,
    mut next: *mut libc::c_long,
    mut mark: *mut libc::c_long,
    mut tag: *mut libc::c_long,
    mut flag_: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    let mut i__3: libc::c_long = 0;
    let mut jmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut kmax: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut vi: libc::c_long = 0;
    let mut vj: libc::c_long = 0;
    let mut nextvi: libc::c_long = 0;
    let mut dvi: libc::c_long = 0;
    let mut sfs: libc::c_long = 0;
    let mut lvk: libc::c_long = 0;
    mark = mark.offset(-1);
    mark;
    next = next.offset(-1);
    next;
    last = last.offset(-1);
    last;
    head = head.offset(-1);
    head;
    l = l.offset(-1);
    l;
    v = v.offset(-1);
    v;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    i__1 = *n;
    vi = 1 as libc::c_int as libc::c_long;
    while vi <= i__1 {
        *mark.offset(vi as isize) = 1 as libc::c_int as libc::c_long;
        *l.offset(vi as isize) = 0 as libc::c_int as libc::c_long;
        *head.offset(vi as isize) = 0 as libc::c_int as libc::c_long;
        vi += 1;
        vi;
    }
    sfs = *n + 1 as libc::c_int as libc::c_long;
    i__1 = *n;
    vi = 1 as libc::c_int as libc::c_long;
    's_63: loop {
        if !(vi <= i__1) {
            current_block = 10758786907990354186;
            break;
        }
        jmin = *ia.offset(vi as isize);
        jmax = *ia.offset((vi + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        if !(jmin > jmax) {
            i__2 = jmax;
            j = jmin;
            while j <= i__2 {
                vj = *ja.offset(j as isize);
                i__3 = vj - vi;
                if i__3 < 0 as libc::c_int as libc::c_long {
                    lvk = vi;
                    kmax = *mark.offset(vi as isize) - 1 as libc::c_int as libc::c_long;
                    if kmax == 0 as libc::c_int as libc::c_long {
                        current_block = 6838970441529464609;
                    } else {
                        i__3 = kmax;
                        k = 1 as libc::c_int as libc::c_long;
                        loop {
                            if !(k <= i__3) {
                                current_block = 6838970441529464609;
                                break;
                            }
                            lvk = *l.offset(lvk as isize);
                            if *v.offset(lvk as isize) == vj {
                                current_block = 8457315219000651999;
                                break;
                            }
                            k += 1;
                            k;
                        }
                    }
                } else if i__3 == 0 as libc::c_int as libc::c_long {
                    current_block = 8457315219000651999;
                } else {
                    current_block = 6838970441529464609;
                }
                match current_block {
                    6838970441529464609 => {
                        if sfs >= *max_ {
                            current_block = 4029375612062182056;
                            break 's_63;
                        }
                        let ref mut fresh11 = *mark.offset(vi as isize);
                        *fresh11 += 1;
                        *fresh11;
                        *v.offset(sfs as isize) = vj;
                        *l.offset(sfs as isize) = *l.offset(vi as isize);
                        *l.offset(vi as isize) = sfs;
                        sfs += 1;
                        sfs;
                        let ref mut fresh12 = *mark.offset(vj as isize);
                        *fresh12 += 1;
                        *fresh12;
                        *v.offset(sfs as isize) = vi;
                        *l.offset(sfs as isize) = *l.offset(vj as isize);
                        *l.offset(vj as isize) = sfs;
                        sfs += 1;
                        sfs;
                    }
                    _ => {}
                }
                j += 1;
                j;
            }
        }
        vi += 1;
        vi;
    }
    match current_block {
        4029375612062182056 => {
            *flag_ = *n * 9 as libc::c_int as libc::c_long + vi;
            return 0 as libc::c_int;
        }
        _ => {
            i__1 = *n;
            vi = 1 as libc::c_int as libc::c_long;
            while vi <= i__1 {
                dvi = *mark.offset(vi as isize);
                *next.offset(vi as isize) = *head.offset(dvi as isize);
                *head.offset(dvi as isize) = vi;
                *last.offset(vi as isize) = -dvi;
                nextvi = *next.offset(vi as isize);
                if nextvi > 0 as libc::c_int as libc::c_long {
                    *last.offset(nextvi as isize) = vi;
                }
                *mark.offset(vi as isize) = *tag;
                vi += 1;
                vi;
            }
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn jgroup_(
    mut n: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut maxg: *mut libc::c_long,
    mut ngrp: *mut libc::c_long,
    mut igp: *mut libc::c_long,
    mut jgp: *mut libc::c_long,
    mut incl: *mut libc::c_long,
    mut jdone: *mut libc::c_long,
    mut ier: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ncol: libc::c_long = 0;
    let mut kmin: libc::c_long = 0;
    let mut kmax: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut ng: libc::c_long = 0;
    jdone = jdone.offset(-1);
    jdone;
    incl = incl.offset(-1);
    incl;
    jgp = jgp.offset(-1);
    jgp;
    igp = igp.offset(-1);
    igp;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    *ier = 0 as libc::c_int as libc::c_long;
    j = 1 as libc::c_int as libc::c_long;
    while j <= *n {
        *jdone.offset(j as isize) = 0 as libc::c_int as libc::c_long;
        j += 1;
        j;
    }
    ncol = 1 as libc::c_int as libc::c_long;
    ng = 1 as libc::c_int as libc::c_long;
    loop {
        if !(ng <= *maxg) {
            current_block = 11459959175219260272;
            break;
        }
        *igp.offset(ng as isize) = ncol;
        i = 1 as libc::c_int as libc::c_long;
        while i <= *n {
            *incl.offset(i as isize) = 0 as libc::c_int as libc::c_long;
            i += 1;
            i;
        }
        j = 1 as libc::c_int as libc::c_long;
        while j <= *n {
            if !(*jdone.offset(j as isize) == 1 as libc::c_int as libc::c_long) {
                kmin = *ia.offset(j as isize);
                kmax = *ia.offset((j + 1 as libc::c_int as libc::c_long) as isize)
                    - 1 as libc::c_int as libc::c_long;
                k = kmin;
                loop {
                    if !(k <= kmax) {
                        current_block = 5601891728916014340;
                        break;
                    }
                    i = *ja.offset(k as isize);
                    if *incl.offset(i as isize) == 1 as libc::c_int as libc::c_long {
                        current_block = 4166486009154926805;
                        break;
                    }
                    k += 1;
                    k;
                }
                match current_block {
                    4166486009154926805 => {}
                    _ => {
                        *jgp.offset(ncol as isize) = j;
                        ncol += 1;
                        ncol;
                        *jdone.offset(j as isize) = 1 as libc::c_int as libc::c_long;
                        k = kmin;
                        while k <= kmax {
                            i = *ja.offset(k as isize);
                            *incl.offset(i as isize) = 1 as libc::c_int as libc::c_long;
                            k += 1;
                            k;
                        }
                    }
                }
            }
            j += 1;
            j;
        }
        if ncol == *igp.offset(ng as isize) {
            current_block = 10418282001537845225;
            break;
        }
        ng += 1;
        ng;
    }
    match current_block {
        11459959175219260272 => {
            if ncol <= *n {
                *ier = 1 as libc::c_int as libc::c_long;
                return 0 as libc::c_int;
            } else {
                ng = *maxg;
            }
        }
        _ => {}
    }
    *ngrp = ng - 1 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ewset_(
    mut n: *mut libc::c_long,
    mut itol: *mut libc::c_long,
    mut rtol: *mut libc::c_double,
    mut atol: *mut libc::c_double,
    mut ycur: *mut libc::c_double,
    mut ewt: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    match *itol {
        1 => {
            i = 0 as libc::c_int as libc::c_long;
            while i < *n {
                *ewt
                    .offset(
                        i as isize,
                    ) = *rtol.offset(0 as libc::c_int as isize)
                    * fabs(*ycur.offset(i as isize))
                    + *atol.offset(0 as libc::c_int as isize);
                i += 1;
                i;
            }
        }
        2 => {
            i = 0 as libc::c_int as libc::c_long;
            while i < *n {
                *ewt
                    .offset(
                        i as isize,
                    ) = *rtol.offset(0 as libc::c_int as isize)
                    * fabs(*ycur.offset(i as isize)) + *atol.offset(i as isize);
                i += 1;
                i;
            }
        }
        3 => {
            i = 0 as libc::c_int as libc::c_long;
            while i < *n {
                *ewt
                    .offset(
                        i as isize,
                    ) = *rtol.offset(i as isize) * fabs(*ycur.offset(i as isize))
                    + *atol.offset(0 as libc::c_int as isize);
                i += 1;
                i;
            }
        }
        4 => {
            i = 0 as libc::c_int as libc::c_long;
            while i < *n {
                *ewt
                    .offset(
                        i as isize,
                    ) = *rtol.offset(i as isize) * fabs(*ycur.offset(i as isize))
                    + *atol.offset(i as isize);
                i += 1;
                i;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
