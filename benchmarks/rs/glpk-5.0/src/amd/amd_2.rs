use ::libc;
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn _glp_amd_postorder(
        nn: libc::c_int,
        Parent: *mut libc::c_int,
        Npiv: *mut libc::c_int,
        Fsize: *mut libc::c_int,
        Order: *mut libc::c_int,
        Child: *mut libc::c_int,
        Sibling: *mut libc::c_int,
        Stack: *mut libc::c_int,
    );
}
unsafe extern "C" fn clear_flag(
    mut wflg: libc::c_int,
    mut wbig: libc::c_int,
    mut W: *mut libc::c_int,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    if wflg < 2 as libc::c_int || wflg >= wbig {
        x = 0 as libc::c_int;
        while x < n {
            if *W.offset(x as isize) != 0 as libc::c_int {
                *W.offset(x as isize) = 1 as libc::c_int;
            }
            x += 1;
            x;
        }
        wflg = 2 as libc::c_int;
    }
    return wflg;
}
pub unsafe extern "C" fn _glp_amd_2(
    mut n: libc::c_int,
    mut Pe: *mut libc::c_int,
    mut Iw: *mut libc::c_int,
    mut Len: *mut libc::c_int,
    mut iwlen: libc::c_int,
    mut pfree: libc::c_int,
    mut Nv: *mut libc::c_int,
    mut Next: *mut libc::c_int,
    mut Last: *mut libc::c_int,
    mut Head: *mut libc::c_int,
    mut Elen: *mut libc::c_int,
    mut Degree: *mut libc::c_int,
    mut W: *mut libc::c_int,
    mut Control: *mut libc::c_double,
    mut Info: *mut libc::c_double,
) {
    let mut deg: libc::c_int = 0;
    let mut degme: libc::c_int = 0;
    let mut dext: libc::c_int = 0;
    let mut lemax: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut elenme: libc::c_int = 0;
    let mut eln: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ilast: libc::c_int = 0;
    let mut inext: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jlast: libc::c_int = 0;
    let mut jnext: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut knt1: libc::c_int = 0;
    let mut knt2: libc::c_int = 0;
    let mut knt3: libc::c_int = 0;
    let mut lenj: libc::c_int = 0;
    let mut ln: libc::c_int = 0;
    let mut me: libc::c_int = 0;
    let mut mindeg: libc::c_int = 0;
    let mut nel: libc::c_int = 0;
    let mut nleft: libc::c_int = 0;
    let mut nvi: libc::c_int = 0;
    let mut nvj: libc::c_int = 0;
    let mut nvpiv: libc::c_int = 0;
    let mut slenme: libc::c_int = 0;
    let mut wbig: libc::c_int = 0;
    let mut we: libc::c_int = 0;
    let mut wflg: libc::c_int = 0;
    let mut wnvi: libc::c_int = 0;
    let mut ok: libc::c_int = 0;
    let mut ndense: libc::c_int = 0;
    let mut ncmpa: libc::c_int = 0;
    let mut dense: libc::c_int = 0;
    let mut aggressive: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut f: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut ndiv: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut nms_lu: libc::c_double = 0.;
    let mut nms_ldl: libc::c_double = 0.;
    let mut dmax: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut lnz: libc::c_double = 0.;
    let mut lnzme: libc::c_double = 0.;
    let mut p: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut p3: libc::c_int = 0;
    let mut p4: libc::c_int = 0;
    let mut pdst: libc::c_int = 0;
    let mut pend: libc::c_int = 0;
    let mut pj: libc::c_int = 0;
    let mut pme: libc::c_int = 0;
    let mut pme1: libc::c_int = 0;
    let mut pme2: libc::c_int = 0;
    let mut pn: libc::c_int = 0;
    let mut psrc: libc::c_int = 0;
    lnz = 0 as libc::c_int as libc::c_double;
    ndiv = 0 as libc::c_int as libc::c_double;
    nms_lu = 0 as libc::c_int as libc::c_double;
    nms_ldl = 0 as libc::c_int as libc::c_double;
    dmax = 1 as libc::c_int as libc::c_double;
    me = -(1 as libc::c_int);
    mindeg = 0 as libc::c_int;
    ncmpa = 0 as libc::c_int;
    nel = 0 as libc::c_int;
    lemax = 0 as libc::c_int;
    if !Control.is_null() {
        alpha = *Control.offset(0 as libc::c_int as isize);
        aggressive = (*Control.offset(1 as libc::c_int as isize)
            != 0 as libc::c_int as libc::c_double) as libc::c_int;
    } else {
        alpha = 10.0f64;
        aggressive = 1 as libc::c_int;
    }
    if alpha < 0 as libc::c_int as libc::c_double {
        dense = n - 2 as libc::c_int;
    } else {
        dense = (alpha * sqrt(n as libc::c_double)) as libc::c_int;
    }
    dense = if 16 as libc::c_int > dense { 16 as libc::c_int } else { dense };
    dense = if n < dense { n } else { dense };
    i = 0 as libc::c_int;
    while i < n {
        *Last.offset(i as isize) = -(1 as libc::c_int);
        *Head.offset(i as isize) = -(1 as libc::c_int);
        *Next.offset(i as isize) = -(1 as libc::c_int);
        *Nv.offset(i as isize) = 1 as libc::c_int;
        *W.offset(i as isize) = 1 as libc::c_int;
        *Elen.offset(i as isize) = 0 as libc::c_int;
        *Degree.offset(i as isize) = *Len.offset(i as isize);
        i += 1;
        i;
    }
    wbig = 2147483647 as libc::c_int - n;
    wflg = clear_flag(0 as libc::c_int, wbig, W, n);
    ndense = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        deg = *Degree.offset(i as isize);
        if deg == 0 as libc::c_int {
            *Elen.offset(i as isize) = -(1 as libc::c_int) - 2 as libc::c_int;
            nel += 1;
            nel;
            *Pe.offset(i as isize) = -(1 as libc::c_int);
            *W.offset(i as isize) = 0 as libc::c_int;
        } else if deg > dense {
            ndense += 1;
            ndense;
            *Nv.offset(i as isize) = 0 as libc::c_int;
            *Elen.offset(i as isize) = -(1 as libc::c_int);
            nel += 1;
            nel;
            *Pe.offset(i as isize) = -(1 as libc::c_int);
        } else {
            inext = *Head.offset(deg as isize);
            if inext != -(1 as libc::c_int) {
                *Last.offset(inext as isize) = i;
            }
            *Next.offset(i as isize) = inext;
            *Head.offset(deg as isize) = i;
        }
        i += 1;
        i;
    }
    while nel < n {
        deg = mindeg;
        while deg < n {
            me = *Head.offset(deg as isize);
            if me != -(1 as libc::c_int) {
                break;
            }
            deg += 1;
            deg;
        }
        mindeg = deg;
        inext = *Next.offset(me as isize);
        if inext != -(1 as libc::c_int) {
            *Last.offset(inext as isize) = -(1 as libc::c_int);
        }
        *Head.offset(deg as isize) = inext;
        elenme = *Elen.offset(me as isize);
        nvpiv = *Nv.offset(me as isize);
        nel += nvpiv;
        *Nv.offset(me as isize) = -nvpiv;
        degme = 0 as libc::c_int;
        if elenme == 0 as libc::c_int {
            pme1 = *Pe.offset(me as isize);
            pme2 = pme1 - 1 as libc::c_int;
            p = pme1;
            while p <= pme1 + *Len.offset(me as isize) - 1 as libc::c_int {
                i = *Iw.offset(p as isize);
                nvi = *Nv.offset(i as isize);
                if nvi > 0 as libc::c_int {
                    degme += nvi;
                    *Nv.offset(i as isize) = -nvi;
                    pme2 += 1;
                    *Iw.offset(pme2 as isize) = i;
                    ilast = *Last.offset(i as isize);
                    inext = *Next.offset(i as isize);
                    if inext != -(1 as libc::c_int) {
                        *Last.offset(inext as isize) = ilast;
                    }
                    if ilast != -(1 as libc::c_int) {
                        *Next.offset(ilast as isize) = inext;
                    } else {
                        *Head.offset(*Degree.offset(i as isize) as isize) = inext;
                    }
                }
                p += 1;
                p;
            }
        } else {
            p = *Pe.offset(me as isize);
            pme1 = pfree;
            slenme = *Len.offset(me as isize) - elenme;
            knt1 = 1 as libc::c_int;
            while knt1 <= elenme + 1 as libc::c_int {
                if knt1 > elenme {
                    e = me;
                    pj = p;
                    ln = slenme;
                } else {
                    let fresh0 = p;
                    p = p + 1;
                    e = *Iw.offset(fresh0 as isize);
                    pj = *Pe.offset(e as isize);
                    ln = *Len.offset(e as isize);
                }
                knt2 = 1 as libc::c_int;
                while knt2 <= ln {
                    let fresh1 = pj;
                    pj = pj + 1;
                    i = *Iw.offset(fresh1 as isize);
                    nvi = *Nv.offset(i as isize);
                    if nvi > 0 as libc::c_int {
                        if pfree >= iwlen {
                            *Pe.offset(me as isize) = p;
                            *Len.offset(me as isize) -= knt1;
                            if *Len.offset(me as isize) == 0 as libc::c_int {
                                *Pe.offset(me as isize) = -(1 as libc::c_int);
                            }
                            *Pe.offset(e as isize) = pj;
                            *Len.offset(e as isize) = ln - knt2;
                            if *Len.offset(e as isize) == 0 as libc::c_int {
                                *Pe.offset(e as isize) = -(1 as libc::c_int);
                            }
                            ncmpa += 1;
                            ncmpa;
                            j = 0 as libc::c_int;
                            while j < n {
                                pn = *Pe.offset(j as isize);
                                if pn >= 0 as libc::c_int {
                                    *Pe.offset(j as isize) = *Iw.offset(pn as isize);
                                    *Iw.offset(pn as isize) = -j - 2 as libc::c_int;
                                }
                                j += 1;
                                j;
                            }
                            psrc = 0 as libc::c_int;
                            pdst = 0 as libc::c_int;
                            pend = pme1 - 1 as libc::c_int;
                            while psrc <= pend {
                                let fresh2 = psrc;
                                psrc = psrc + 1;
                                j = -*Iw.offset(fresh2 as isize) - 2 as libc::c_int;
                                if j >= 0 as libc::c_int {
                                    *Iw.offset(pdst as isize) = *Pe.offset(j as isize);
                                    let fresh3 = pdst;
                                    pdst = pdst + 1;
                                    *Pe.offset(j as isize) = fresh3;
                                    lenj = *Len.offset(j as isize);
                                    knt3 = 0 as libc::c_int;
                                    while knt3 <= lenj - 2 as libc::c_int {
                                        let fresh4 = psrc;
                                        psrc = psrc + 1;
                                        let fresh5 = pdst;
                                        pdst = pdst + 1;
                                        *Iw.offset(fresh5 as isize) = *Iw.offset(fresh4 as isize);
                                        knt3 += 1;
                                        knt3;
                                    }
                                }
                            }
                            p1 = pdst;
                            psrc = pme1;
                            while psrc <= pfree - 1 as libc::c_int {
                                let fresh6 = pdst;
                                pdst = pdst + 1;
                                *Iw.offset(fresh6 as isize) = *Iw.offset(psrc as isize);
                                psrc += 1;
                                psrc;
                            }
                            pme1 = p1;
                            pfree = pdst;
                            pj = *Pe.offset(e as isize);
                            p = *Pe.offset(me as isize);
                        }
                        degme += nvi;
                        *Nv.offset(i as isize) = -nvi;
                        let fresh7 = pfree;
                        pfree = pfree + 1;
                        *Iw.offset(fresh7 as isize) = i;
                        ilast = *Last.offset(i as isize);
                        inext = *Next.offset(i as isize);
                        if inext != -(1 as libc::c_int) {
                            *Last.offset(inext as isize) = ilast;
                        }
                        if ilast != -(1 as libc::c_int) {
                            *Next.offset(ilast as isize) = inext;
                        } else {
                            *Head.offset(*Degree.offset(i as isize) as isize) = inext;
                        }
                    }
                    knt2 += 1;
                    knt2;
                }
                if e != me {
                    *Pe.offset(e as isize) = -me - 2 as libc::c_int;
                    *W.offset(e as isize) = 0 as libc::c_int;
                }
                knt1 += 1;
                knt1;
            }
            pme2 = pfree - 1 as libc::c_int;
        }
        *Degree.offset(me as isize) = degme;
        *Pe.offset(me as isize) = pme1;
        *Len.offset(me as isize) = pme2 - pme1 + 1 as libc::c_int;
        *Elen.offset(me as isize) = -(nvpiv + degme) - 2 as libc::c_int;
        wflg = clear_flag(wflg, wbig, W, n);
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            eln = *Elen.offset(i as isize);
            if eln > 0 as libc::c_int {
                nvi = -*Nv.offset(i as isize);
                wnvi = wflg - nvi;
                p = *Pe.offset(i as isize);
                while p <= *Pe.offset(i as isize) + eln - 1 as libc::c_int {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we >= wflg {
                        we -= nvi;
                    } else if we != 0 as libc::c_int {
                        we = *Degree.offset(e as isize) + wnvi;
                    }
                    *W.offset(e as isize) = we;
                    p += 1;
                    p;
                }
            }
            pme += 1;
            pme;
        }
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            p1 = *Pe.offset(i as isize);
            p2 = p1 + *Elen.offset(i as isize) - 1 as libc::c_int;
            pn = p1;
            hash = 0 as libc::c_int as libc::c_uint;
            deg = 0 as libc::c_int;
            if aggressive != 0 {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as libc::c_int {
                        dext = we - wflg;
                        if dext > 0 as libc::c_int {
                            deg += dext;
                            let fresh8 = pn;
                            pn = pn + 1;
                            *Iw.offset(fresh8 as isize) = e;
                            hash = hash.wrapping_add(e as libc::c_uint);
                        } else {
                            *Pe.offset(e as isize) = -me - 2 as libc::c_int;
                            *W.offset(e as isize) = 0 as libc::c_int;
                        }
                    }
                    p += 1;
                    p;
                }
            } else {
                p = p1;
                while p <= p2 {
                    e = *Iw.offset(p as isize);
                    we = *W.offset(e as isize);
                    if we != 0 as libc::c_int {
                        dext = we - wflg;
                        deg += dext;
                        let fresh9 = pn;
                        pn = pn + 1;
                        *Iw.offset(fresh9 as isize) = e;
                        hash = hash.wrapping_add(e as libc::c_uint);
                    }
                    p += 1;
                    p;
                }
            }
            *Elen.offset(i as isize) = pn - p1 + 1 as libc::c_int;
            p3 = pn;
            p4 = p1 + *Len.offset(i as isize);
            p = p2 + 1 as libc::c_int;
            while p < p4 {
                j = *Iw.offset(p as isize);
                nvj = *Nv.offset(j as isize);
                if nvj > 0 as libc::c_int {
                    deg += nvj;
                    let fresh10 = pn;
                    pn = pn + 1;
                    *Iw.offset(fresh10 as isize) = j;
                    hash = hash.wrapping_add(j as libc::c_uint);
                }
                p += 1;
                p;
            }
            if *Elen.offset(i as isize) == 1 as libc::c_int && p3 == pn {
                *Pe.offset(i as isize) = -me - 2 as libc::c_int;
                nvi = -*Nv.offset(i as isize);
                degme -= nvi;
                nvpiv += nvi;
                nel += nvi;
                *Nv.offset(i as isize) = 0 as libc::c_int;
                *Elen.offset(i as isize) = -(1 as libc::c_int);
            } else {
                *Degree
                    .offset(
                        i as isize,
                    ) = if *Degree.offset(i as isize) < deg {
                    *Degree.offset(i as isize)
                } else {
                    deg
                };
                *Iw.offset(pn as isize) = *Iw.offset(p3 as isize);
                *Iw.offset(p3 as isize) = *Iw.offset(p1 as isize);
                *Iw.offset(p1 as isize) = me;
                *Len.offset(i as isize) = pn - p1 + 1 as libc::c_int;
                hash = hash.wrapping_rem(n as libc::c_uint);
                j = *Head.offset(hash as isize);
                if j <= -(1 as libc::c_int) {
                    *Next.offset(i as isize) = -j - 2 as libc::c_int;
                    *Head.offset(hash as isize) = -i - 2 as libc::c_int;
                } else {
                    *Next.offset(i as isize) = *Last.offset(j as isize);
                    *Last.offset(j as isize) = i;
                }
                *Last.offset(i as isize) = hash as libc::c_int;
            }
            pme += 1;
            pme;
        }
        *Degree.offset(me as isize) = degme;
        lemax = if lemax > degme { lemax } else { degme };
        wflg += lemax;
        wflg = clear_flag(wflg, wbig, W, n);
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            if *Nv.offset(i as isize) < 0 as libc::c_int {
                hash = *Last.offset(i as isize) as libc::c_uint;
                j = *Head.offset(hash as isize);
                if j == -(1 as libc::c_int) {
                    i = -(1 as libc::c_int);
                } else if j < -(1 as libc::c_int) {
                    i = -j - 2 as libc::c_int;
                    *Head.offset(hash as isize) = -(1 as libc::c_int);
                } else {
                    i = *Last.offset(j as isize);
                    *Last.offset(j as isize) = -(1 as libc::c_int);
                }
                while i != -(1 as libc::c_int)
                    && *Next.offset(i as isize) != -(1 as libc::c_int)
                {
                    ln = *Len.offset(i as isize);
                    eln = *Elen.offset(i as isize);
                    p = *Pe.offset(i as isize) + 1 as libc::c_int;
                    while p <= *Pe.offset(i as isize) + ln - 1 as libc::c_int {
                        *W.offset(*Iw.offset(p as isize) as isize) = wflg;
                        p += 1;
                        p;
                    }
                    jlast = i;
                    j = *Next.offset(i as isize);
                    while j != -(1 as libc::c_int) {
                        ok = (*Len.offset(j as isize) == ln
                            && *Elen.offset(j as isize) == eln) as libc::c_int;
                        p = *Pe.offset(j as isize) + 1 as libc::c_int;
                        while ok != 0
                            && p <= *Pe.offset(j as isize) + ln - 1 as libc::c_int
                        {
                            if *W.offset(*Iw.offset(p as isize) as isize) != wflg {
                                ok = 0 as libc::c_int;
                            }
                            p += 1;
                            p;
                        }
                        if ok != 0 {
                            *Pe.offset(j as isize) = -i - 2 as libc::c_int;
                            *Nv.offset(i as isize) += *Nv.offset(j as isize);
                            *Nv.offset(j as isize) = 0 as libc::c_int;
                            *Elen.offset(j as isize) = -(1 as libc::c_int);
                            j = *Next.offset(j as isize);
                            *Next.offset(jlast as isize) = j;
                        } else {
                            jlast = j;
                            j = *Next.offset(j as isize);
                        }
                    }
                    wflg += 1;
                    wflg;
                    i = *Next.offset(i as isize);
                }
            }
            pme += 1;
            pme;
        }
        p = pme1;
        nleft = n - nel;
        pme = pme1;
        while pme <= pme2 {
            i = *Iw.offset(pme as isize);
            nvi = -*Nv.offset(i as isize);
            if nvi > 0 as libc::c_int {
                *Nv.offset(i as isize) = nvi;
                deg = *Degree.offset(i as isize) + degme - nvi;
                deg = if deg < nleft - nvi { deg } else { nleft - nvi };
                inext = *Head.offset(deg as isize);
                if inext != -(1 as libc::c_int) {
                    *Last.offset(inext as isize) = i;
                }
                *Next.offset(i as isize) = inext;
                *Last.offset(i as isize) = -(1 as libc::c_int);
                *Head.offset(deg as isize) = i;
                mindeg = if mindeg < deg { mindeg } else { deg };
                *Degree.offset(i as isize) = deg;
                let fresh11 = p;
                p = p + 1;
                *Iw.offset(fresh11 as isize) = i;
            }
            pme += 1;
            pme;
        }
        *Nv.offset(me as isize) = nvpiv;
        *Len.offset(me as isize) = p - pme1;
        if *Len.offset(me as isize) == 0 as libc::c_int {
            *Pe.offset(me as isize) = -(1 as libc::c_int);
            *W.offset(me as isize) = 0 as libc::c_int;
        }
        if elenme != 0 as libc::c_int {
            pfree = p;
        }
        if !Info.is_null() {
            f = nvpiv as libc::c_double;
            r = (degme + ndense) as libc::c_double;
            dmax = if dmax > f + r { dmax } else { f + r };
            lnzme = f * r
                + (f - 1 as libc::c_int as libc::c_double) * f
                    / 2 as libc::c_int as libc::c_double;
            lnz += lnzme;
            ndiv += lnzme;
            s = f * r * r + r * (f - 1 as libc::c_int as libc::c_double) * f
                + (f - 1 as libc::c_int as libc::c_double) * f
                    * (2 as libc::c_int as libc::c_double * f
                        - 1 as libc::c_int as libc::c_double)
                    / 6 as libc::c_int as libc::c_double;
            nms_lu += s;
            nms_ldl += (s + lnzme) / 2 as libc::c_int as libc::c_double;
        }
    }
    if !Info.is_null() {
        f = ndense as libc::c_double;
        dmax = if dmax > ndense as libc::c_double {
            dmax
        } else {
            ndense as libc::c_double
        };
        lnzme = (f - 1 as libc::c_int as libc::c_double) * f
            / 2 as libc::c_int as libc::c_double;
        lnz += lnzme;
        ndiv += lnzme;
        s = (f - 1 as libc::c_int as libc::c_double) * f
            * (2 as libc::c_int as libc::c_double * f
                - 1 as libc::c_int as libc::c_double)
            / 6 as libc::c_int as libc::c_double;
        nms_lu += s;
        nms_ldl += (s + lnzme) / 2 as libc::c_int as libc::c_double;
        *Info.offset(9 as libc::c_int as isize) = lnz;
        *Info.offset(10 as libc::c_int as isize) = ndiv;
        *Info.offset(11 as libc::c_int as isize) = nms_ldl;
        *Info.offset(12 as libc::c_int as isize) = nms_lu;
        *Info.offset(6 as libc::c_int as isize) = ndense as libc::c_double;
        *Info.offset(13 as libc::c_int as isize) = dmax;
        *Info.offset(8 as libc::c_int as isize) = ncmpa as libc::c_double;
        *Info.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < n {
        *Pe.offset(i as isize) = -*Pe.offset(i as isize) - 2 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n {
        *Elen.offset(i as isize) = -*Elen.offset(i as isize) - 2 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n {
        if *Nv.offset(i as isize) == 0 as libc::c_int {
            j = *Pe.offset(i as isize);
            if !(j == -(1 as libc::c_int)) {
                while *Nv.offset(j as isize) == 0 as libc::c_int {
                    j = *Pe.offset(j as isize);
                }
                e = j;
                j = i;
                while *Nv.offset(j as isize) == 0 as libc::c_int {
                    jnext = *Pe.offset(j as isize);
                    *Pe.offset(j as isize) = e;
                    j = jnext;
                }
            }
        }
        i += 1;
        i;
    }
    _glp_amd_postorder(n, Pe, Nv, Elen, W, Head, Next, Last);
    k = 0 as libc::c_int;
    while k < n {
        *Head.offset(k as isize) = -(1 as libc::c_int);
        *Next.offset(k as isize) = -(1 as libc::c_int);
        k += 1;
        k;
    }
    e = 0 as libc::c_int;
    while e < n {
        k = *W.offset(e as isize);
        if k != -(1 as libc::c_int) {
            *Head.offset(k as isize) = e;
        }
        e += 1;
        e;
    }
    nel = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < n {
        e = *Head.offset(k as isize);
        if e == -(1 as libc::c_int) {
            break;
        }
        *Next.offset(e as isize) = nel;
        nel += *Nv.offset(e as isize);
        k += 1;
        k;
    }
    i = 0 as libc::c_int;
    while i < n {
        if *Nv.offset(i as isize) == 0 as libc::c_int {
            e = *Pe.offset(i as isize);
            if e != -(1 as libc::c_int) {
                *Next.offset(i as isize) = *Next.offset(e as isize);
                let ref mut fresh12 = *Next.offset(e as isize);
                *fresh12 += 1;
                *fresh12;
            } else {
                let fresh13 = nel;
                nel = nel + 1;
                *Next.offset(i as isize) = fresh13;
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n {
        k = *Next.offset(i as isize);
        *Last.offset(k as isize) = i;
        i += 1;
        i;
    }
}
