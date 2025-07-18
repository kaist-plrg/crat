use ::libc;
extern "C" {
    fn Kf(x: F) -> K;
    fn Ki(x: I) -> K;
    fn newK(t: I, n: I) -> K;
    fn ci(a: K) -> K;
    fn cd(a: K) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn FC(a: F, b: F) -> I;
    fn vitter(a: *mut I, n: I, N: I);
    fn RF() -> F;
    fn kerr(s: cS) -> K;
    fn vf_ex(q: V, g: K) -> K;
    fn atomI(a: K) -> I;
    fn at_verb(a: K, b: K) -> K;
    fn countI(a: K) -> I;
    fn take(a: K, b: K) -> K;
    fn matchI(a: K, b: K) -> I;
    fn hash_find(a: K, b: K) -> K;
}
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k0 {
    pub _c: I,
    pub t: I,
    pub n: I,
    pub k: [*mut k0; 1],
}
pub type K = *mut k0;
pub unsafe extern "C" fn find(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    if at > 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    if -(4 as libc::c_int) as libc::c_longlong == at
        && 4 as libc::c_int as libc::c_longlong == bt
    {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = an;
        while i < _i {
            if *(((*a).k).as_mut_ptr() as *mut S).offset(i as isize)
                == *(((*b).k).as_mut_ptr() as *mut S)
            {
                return Ki(i);
            }
            i += 1;
            i;
        }
    }
    if -(3 as libc::c_int) as libc::c_longlong == at
        && 3 as libc::c_int as libc::c_longlong == bt
    {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = an;
        while i_0 < _i_0 {
            if *(((*a).k).as_mut_ptr() as *mut C).offset(i_0 as isize) as libc::c_int
                == *(((*b).k).as_mut_ptr() as *mut C) as libc::c_int
            {
                return Ki(i_0);
            }
            i_0 += 1;
            i_0;
        }
    }
    if -(2 as libc::c_int) as libc::c_longlong == at
        && 2 as libc::c_int as libc::c_longlong == bt
    {
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = an;
        while i_1 < _i_1 {
            if FC(
                *(((*a).k).as_mut_ptr() as *mut F).offset(i_1 as isize),
                *(((*b).k).as_mut_ptr() as *mut F),
            ) == 0
            {
                return Ki(i_1);
            }
            i_1 += 1;
            i_1;
        }
    }
    if -(2 as libc::c_int) as libc::c_longlong == at
        && 1 as libc::c_int as libc::c_longlong == bt
    {
        let mut fb: F = if 9223372036854775807 as libc::c_longlong
            == *(((*b).k).as_mut_ptr() as *mut I)
        {
            1 as libc::c_int as libc::c_double / 0.0f64
        } else if -(9223372036854775807 as libc::c_longlong)
            == *(((*b).k).as_mut_ptr() as *mut I)
        {
            -(1 as libc::c_int as libc::c_double / 0.0f64)
        } else if -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
            == *(((*b).k).as_mut_ptr() as *mut I)
        {
            0 as libc::c_int as libc::c_double / 0.0f64
        } else {
            *(((*b).k).as_mut_ptr() as *mut I) as libc::c_double
        };
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = an;
        while i_2 < _i_2 {
            if FC(*(((*a).k).as_mut_ptr() as *mut F).offset(i_2 as isize), fb) == 0 {
                return Ki(i_2);
            }
            i_2 += 1;
            i_2;
        }
    }
    if -(1 as libc::c_int) as libc::c_longlong == at
        && 2 as libc::c_int as libc::c_longlong == bt
    {
        let mut i_3: I = 0 as libc::c_int as I;
        let mut _i_3: I = an;
        while i_3 < _i_3 {
            if FC(
                if 9223372036854775807 as libc::c_longlong
                    == *(((*a).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                {
                    1 as libc::c_int as libc::c_double / 0.0f64
                } else if -(9223372036854775807 as libc::c_longlong)
                    == *(((*a).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                {
                    -(1 as libc::c_int as libc::c_double / 0.0f64)
                } else if -(9223372036854775807 as libc::c_longlong)
                    - 1 as libc::c_longlong
                    == *(((*a).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                {
                    0 as libc::c_int as libc::c_double / 0.0f64
                } else {
                    *(((*a).k).as_mut_ptr() as *mut I).offset(i_3 as isize)
                        as libc::c_double
                },
                *(((*b).k).as_mut_ptr() as *mut F),
            ) == 0
            {
                return Ki(i_3);
            }
            i_3 += 1;
            i_3;
        }
    }
    if -(1 as libc::c_int) as libc::c_longlong == at
        && 1 as libc::c_int as libc::c_longlong == bt
    {
        let mut i_4: I = 0 as libc::c_int as I;
        let mut _i_4: I = an;
        while i_4 < _i_4 {
            if *(((*a).k).as_mut_ptr() as *mut I).offset(i_4 as isize)
                == *(((*b).k).as_mut_ptr() as *mut I)
            {
                return Ki(i_4);
            }
            i_4 += 1;
            i_4;
        }
    }
    if at == 0 {
        if 2 as libc::c_int as libc::c_longlong == an
            && -(5 as libc::c_int) as libc::c_longlong
                == (**((*a).k).as_mut_ptr().offset(1 as libc::c_int as isize)).t
        {
            return hash_find(a, b);
        }
        let mut i_5: I = 0 as libc::c_int as I;
        let mut _i_5: I = an;
        while i_5 < _i_5 {
            if matchI(*((*a).k).as_mut_ptr().offset(i_5 as isize), b) != 0 {
                return Ki(i_5);
            }
            i_5 += 1;
            i_5;
        }
    }
    return Ki(an);
}
unsafe extern "C" fn num_ex(mut a: K, mut x: F) -> F {
    let mut y: F = 0 as libc::c_int as F;
    let mut b: K = 0 as *mut k0;
    let mut g: K = 0 as *mut k0;
    b = Kf(x);
    if b.is_null() {
        return 0 as libc::c_int as libc::c_double / 0.0f64;
    }
    g = newK(0 as libc::c_int as I, 1 as libc::c_int as I);
    if g.is_null() {
        cd(b);
        return 0 as libc::c_int as libc::c_double / 0.0f64;
    }
    let ref mut fresh0 = *((*g).k).as_mut_ptr();
    *fresh0 = ci(b);
    let mut k: K = vf_ex(&mut a as *mut K as V, g);
    if k.is_null()
        || (*k).t != 1 as libc::c_int as libc::c_longlong
            && (*k).t != 2 as libc::c_int as libc::c_longlong
    {
        y = 0 as libc::c_int as libc::c_double / 0.0f64;
    } else if (*k).t == 1 as libc::c_int as libc::c_longlong {
        y = *(((*k).k).as_mut_ptr() as *mut I) as F;
    } else {
        y = *(((*k).k).as_mut_ptr() as *mut F);
    }
    cd(b);
    cd(k);
    cd(g);
    return y;
}
unsafe extern "C" fn isShallowNumeric(mut k: K) -> I {
    if (if (*k).t < 0 as libc::c_int as libc::c_longlong { -(*k).t } else { (*k).t })
        > 2 as libc::c_int as libc::c_longlong
    {
        return 0 as libc::c_int as I;
    }
    if 0 as libc::c_int as libc::c_longlong == (*k).t {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (*k).n;
        while i < _i {
            let mut t: I = (**((*k).k).as_mut_ptr().offset(i as isize)).t;
            if t != 1 as libc::c_int as libc::c_longlong
                && t != 2 as libc::c_int as libc::c_longlong
            {
                return 0 as libc::c_int as I;
            }
            i += 1;
            i;
        }
    }
    return 1 as libc::c_int as I;
}
unsafe extern "C" fn ithFloat(mut k: K, mut i: I) -> F {
    if k.is_null() {
        return 0 as libc::c_int as F;
    }
    let mut n: I = (*k).n;
    if (*k).t == 0 {
        k = *((*k).k).as_mut_ptr().offset((i % n) as isize);
        i = 0 as libc::c_int as I;
    }
    if 1 as libc::c_int as libc::c_longlong
        == (if (*k).t < 0 as libc::c_int as libc::c_longlong { -(*k).t } else { (*k).t })
    {
        return *(((*k).k).as_mut_ptr() as *mut I).offset((i % n) as isize) as F;
    }
    return *(((*k).k).as_mut_ptr() as *mut F).offset((i % n) as isize);
}
unsafe extern "C" fn inverter(mut a: K, mut b: K, mut c: K, mut index: I) -> F {
    let mut y: F = ithFloat(b, index);
    let mut i: I = 0;
    let mut m: I = 20 as libc::c_int as I;
    let vla = (m + 2 as libc::c_int as libc::c_longlong) as usize;
    let mut x: Vec::<F> = ::std::vec::from_elem(0., vla);
    let vla_0 = (m + 2 as libc::c_int as libc::c_longlong) as usize;
    let mut f: Vec::<F> = ::std::vec::from_elem(0., vla_0);
    *x.as_mut_ptr().offset(0 as libc::c_int as isize) = 0.9998f64;
    *x.as_mut_ptr().offset(1 as libc::c_int as isize) = 0.9999f64;
    if !c.is_null() {
        let mut r: F = ithFloat(c, index);
        *x.as_mut_ptr().offset(0 as libc::c_int as isize) = 0.9999f64 * r;
        *x.as_mut_ptr().offset(1 as libc::c_int as isize) = r;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i: I = 2 as libc::c_int as I;
    while i_0 < _i {
        *f
            .as_mut_ptr()
            .offset(i_0 as isize) = num_ex(a, *x.as_mut_ptr().offset(i_0 as isize)) - y;
        i_0 += 1;
        i_0;
    }
    let mut d: F = 0.;
    let mut e: F = if y != 0. { y * 0.000001f64 } else { 0.000001f64 };
    i = 0 as libc::c_int as I;
    while i < m {
        d = (*x.as_mut_ptr().offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
            - *x.as_mut_ptr().offset(i as isize))
            / (*f
                .as_mut_ptr()
                .offset((i + 1 as libc::c_int as libc::c_longlong) as isize)
                - *f.as_mut_ptr().offset(i as isize))
            * *f
                .as_mut_ptr()
                .offset((i + 1 as libc::c_int as libc::c_longlong) as isize);
        *x
            .as_mut_ptr()
            .offset(
                (i + 2 as libc::c_int as libc::c_longlong) as isize,
            ) = *x
            .as_mut_ptr()
            .offset((i + 1 as libc::c_int as libc::c_longlong) as isize) - d;
        *f
            .as_mut_ptr()
            .offset(
                (i + 2 as libc::c_int as libc::c_longlong) as isize,
            ) = num_ex(
            a,
            *x.as_mut_ptr().offset((i + 2 as libc::c_int as libc::c_longlong) as isize),
        ) - y;
        if (if d < 0 as libc::c_int as libc::c_double { -d } else { d }) < e
            || FC(
                *f
                    .as_mut_ptr()
                    .offset((i + 2 as libc::c_int as libc::c_longlong) as isize),
                0.0f64,
            ) == 0
        {
            break;
        }
        i += 1;
        i;
    }
    if i >= m {
        kerr(b"limit\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int as F;
    }
    return *x.as_mut_ptr().offset((i + 2 as libc::c_int as libc::c_longlong) as isize);
}
pub unsafe extern "C" fn what_triadic(mut a: K, mut b: K, mut c: K) -> K {
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if isShallowNumeric(b) == 0 || !c.is_null() && isShallowNumeric(c) == 0 {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if bt == 0 && bn == 0 || !c.is_null() && (*c).t == 0 && (*c).n == 0 {
        return newK(0 as libc::c_int as I, 0 as libc::c_int as I);
    }
    if 0 as libc::c_int as libc::c_longlong == bn
        || !c.is_null() && 0 as libc::c_int as libc::c_longlong == (*c).n
    {
        return newK(-(2 as libc::c_int) as I, 0 as libc::c_int as I);
    }
    if !c.is_null() && (*c).t < 1 as libc::c_int as libc::c_longlong
        && bt < 1 as libc::c_int as libc::c_longlong && (*c).n != (*b).n
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    let mut zn: I = bn;
    let mut zt: I = 2 as libc::c_int as I;
    if bt < 1 as libc::c_int as libc::c_longlong
        || !c.is_null() && (*c).t < 1 as libc::c_int as libc::c_longlong
    {
        zt = -(2 as libc::c_int) as I;
    }
    if !c.is_null() {
        zn = if zn > (*c).n { zn } else { (*c).n };
    }
    let mut z: K = newK(zt, zn);
    if z.is_null() {
        return 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = zn;
    while i < _i {
        *(((*z).k).as_mut_ptr() as *mut F).offset(i as isize) = inverter(a, b, c, i);
        i += 1;
        i;
    }
    return z;
}
unsafe extern "C" fn qrand(mut a: K, mut b: K) -> K {
    let mut at: I = (*a).t;
    let mut bt: I = (*b).t;
    let mut y: K = 0 as *mut k0;
    if 1 as libc::c_int as libc::c_longlong
        != (if at < 0 as libc::c_int as libc::c_longlong { -at } else { at })
        || 1 as libc::c_int as libc::c_longlong != bt
            && 2 as libc::c_int as libc::c_longlong != bt
    {
        return kerr(b"int\0" as *const u8 as *const libc::c_char);
    }
    let mut c: I = *(((*a).k).as_mut_ptr() as *mut I);
    let mut n: I = if c < 0 as libc::c_int as libc::c_longlong { -c } else { c };
    if 1 as libc::c_int as libc::c_longlong == bt
        && c < 0 as libc::c_int as libc::c_longlong
        && *(((*b).k).as_mut_ptr() as *mut I) < -c
    {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if 1 as libc::c_int as libc::c_longlong == bt
        && *(((*b).k).as_mut_ptr() as *mut I) < 0 as libc::c_int as libc::c_longlong
    {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut j: I = 0 as libc::c_int as I;
    let mut k: I = 0;
    let mut s: I = 0;
    y = newK(
        (if 1 as libc::c_int as libc::c_longlong == bt {
            -(1 as libc::c_int)
        } else {
            -(2 as libc::c_int)
        }) as I,
        n,
    );
    if y.is_null() {
        return 0 as K;
    }
    if 2 as libc::c_int as libc::c_longlong == bt {
        let mut f: F = *(((*b).k).as_mut_ptr() as *mut F);
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            *(((*y).k).as_mut_ptr() as *mut F).offset(i as isize) = RF() * f;
            i += 1;
            i;
        }
        return y;
    }
    let mut d: I = *(((*b).k).as_mut_ptr() as *mut I);
    if c >= 0 as libc::c_int as libc::c_longlong {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            *(((*y).k).as_mut_ptr() as *mut I)
                .offset(i_0 as isize) = (d as libc::c_double * RF()) as I;
            i_0 += 1;
            i_0;
        }
    } else {
        vitter(((*y).k).as_mut_ptr() as *mut I, (*y).n, d);
        j = n - 1 as libc::c_int as libc::c_longlong;
        while j > 0 as libc::c_int as libc::c_longlong {
            k = ((1 as libc::c_int as libc::c_longlong + j) as libc::c_double * RF())
                as I;
            s = *(((*y).k).as_mut_ptr() as *mut I).offset(j as isize);
            *(((*y).k).as_mut_ptr() as *mut I)
                .offset(
                    j as isize,
                ) = *(((*y).k).as_mut_ptr() as *mut I).offset(k as isize);
            *(((*y).k).as_mut_ptr() as *mut I).offset(k as isize) = s;
            j -= 1;
            j;
        }
    }
    return y;
}
pub unsafe extern "C" fn sample(mut x: K, mut y: K) -> K {
    let mut a: K = 0 as *mut k0;
    let mut b: K = 0 as *mut k0;
    let mut z: K = 0 as *mut k0;
    if (*y).n == 0 {
        return take(x, y);
    }
    b = Ki(countI(y));
    if b.is_null() {
        return 0 as K;
    }
    a = qrand(x, b);
    if OOM_CD(0 as libc::c_int as I, a, b, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    cd(b);
    z = at_verb(y, a);
    cd(a);
    return z;
}
pub unsafe extern "C" fn what(mut x: K, mut y: K) -> K {
    if 1 as libc::c_int as libc::c_longlong == (*x).t {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    if 7 as libc::c_int as libc::c_longlong == (*x).t {
        return what_triadic(x, y, 0 as K);
    }
    if 1 as libc::c_int as libc::c_longlong == (*x).t {
        return if atomI(y) != 0 { qrand(x, y) } else { sample(x, y) };
    }
    return find(x, y);
}
