use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut inbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: libc::c_uint;
    static mut inptr: libc::c_uint;
    static mut outcnt: libc::c_uint;
    fn fill_inbuf(eof_ok: libc::c_int) -> libc::c_int;
    fn flush_window();
}
pub type voidp = *mut libc::c_void;
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type ulg = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huft {
    pub e: uch,
    pub b: uch,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub n: ush,
    pub t: *mut huft,
}
static mut border: [libc::c_uint; 19] = [
    16 as libc::c_int as libc::c_uint,
    17 as libc::c_int as libc::c_uint,
    18 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    8 as libc::c_int as libc::c_uint,
    7 as libc::c_int as libc::c_uint,
    9 as libc::c_int as libc::c_uint,
    6 as libc::c_int as libc::c_uint,
    10 as libc::c_int as libc::c_uint,
    5 as libc::c_int as libc::c_uint,
    11 as libc::c_int as libc::c_uint,
    4 as libc::c_int as libc::c_uint,
    12 as libc::c_int as libc::c_uint,
    3 as libc::c_int as libc::c_uint,
    13 as libc::c_int as libc::c_uint,
    2 as libc::c_int as libc::c_uint,
    14 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    15 as libc::c_int as libc::c_uint,
];
static mut cplens: [ush; 31] = [
    3 as libc::c_int as ush,
    4 as libc::c_int as ush,
    5 as libc::c_int as ush,
    6 as libc::c_int as ush,
    7 as libc::c_int as ush,
    8 as libc::c_int as ush,
    9 as libc::c_int as ush,
    10 as libc::c_int as ush,
    11 as libc::c_int as ush,
    13 as libc::c_int as ush,
    15 as libc::c_int as ush,
    17 as libc::c_int as ush,
    19 as libc::c_int as ush,
    23 as libc::c_int as ush,
    27 as libc::c_int as ush,
    31 as libc::c_int as ush,
    35 as libc::c_int as ush,
    43 as libc::c_int as ush,
    51 as libc::c_int as ush,
    59 as libc::c_int as ush,
    67 as libc::c_int as ush,
    83 as libc::c_int as ush,
    99 as libc::c_int as ush,
    115 as libc::c_int as ush,
    131 as libc::c_int as ush,
    163 as libc::c_int as ush,
    195 as libc::c_int as ush,
    227 as libc::c_int as ush,
    258 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
];
static mut cplext: [ush; 31] = [
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    1 as libc::c_int as ush,
    1 as libc::c_int as ush,
    1 as libc::c_int as ush,
    1 as libc::c_int as ush,
    2 as libc::c_int as ush,
    2 as libc::c_int as ush,
    2 as libc::c_int as ush,
    2 as libc::c_int as ush,
    3 as libc::c_int as ush,
    3 as libc::c_int as ush,
    3 as libc::c_int as ush,
    3 as libc::c_int as ush,
    4 as libc::c_int as ush,
    4 as libc::c_int as ush,
    4 as libc::c_int as ush,
    4 as libc::c_int as ush,
    5 as libc::c_int as ush,
    5 as libc::c_int as ush,
    5 as libc::c_int as ush,
    5 as libc::c_int as ush,
    0 as libc::c_int as ush,
    99 as libc::c_int as ush,
    99 as libc::c_int as ush,
];
static mut cpdist: [ush; 30] = [
    1 as libc::c_int as ush,
    2 as libc::c_int as ush,
    3 as libc::c_int as ush,
    4 as libc::c_int as ush,
    5 as libc::c_int as ush,
    7 as libc::c_int as ush,
    9 as libc::c_int as ush,
    13 as libc::c_int as ush,
    17 as libc::c_int as ush,
    25 as libc::c_int as ush,
    33 as libc::c_int as ush,
    49 as libc::c_int as ush,
    65 as libc::c_int as ush,
    97 as libc::c_int as ush,
    129 as libc::c_int as ush,
    193 as libc::c_int as ush,
    257 as libc::c_int as ush,
    385 as libc::c_int as ush,
    513 as libc::c_int as ush,
    769 as libc::c_int as ush,
    1025 as libc::c_int as ush,
    1537 as libc::c_int as ush,
    2049 as libc::c_int as ush,
    3073 as libc::c_int as ush,
    4097 as libc::c_int as ush,
    6145 as libc::c_int as ush,
    8193 as libc::c_int as ush,
    12289 as libc::c_int as ush,
    16385 as libc::c_int as ush,
    24577 as libc::c_int as ush,
];
static mut cpdext: [ush; 30] = [
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    0 as libc::c_int as ush,
    1 as libc::c_int as ush,
    1 as libc::c_int as ush,
    2 as libc::c_int as ush,
    2 as libc::c_int as ush,
    3 as libc::c_int as ush,
    3 as libc::c_int as ush,
    4 as libc::c_int as ush,
    4 as libc::c_int as ush,
    5 as libc::c_int as ush,
    5 as libc::c_int as ush,
    6 as libc::c_int as ush,
    6 as libc::c_int as ush,
    7 as libc::c_int as ush,
    7 as libc::c_int as ush,
    8 as libc::c_int as ush,
    8 as libc::c_int as ush,
    9 as libc::c_int as ush,
    9 as libc::c_int as ush,
    10 as libc::c_int as ush,
    10 as libc::c_int as ush,
    11 as libc::c_int as ush,
    11 as libc::c_int as ush,
    12 as libc::c_int as ush,
    12 as libc::c_int as ush,
    13 as libc::c_int as ush,
    13 as libc::c_int as ush,
];
static mut bb: ulg = 0;
static mut bk: libc::c_uint = 0;
static mut mask_bits: [ush; 17] = [
    0 as libc::c_int as ush,
    0x1 as libc::c_int as ush,
    0x3 as libc::c_int as ush,
    0x7 as libc::c_int as ush,
    0xf as libc::c_int as ush,
    0x1f as libc::c_int as ush,
    0x3f as libc::c_int as ush,
    0x7f as libc::c_int as ush,
    0xff as libc::c_int as ush,
    0x1ff as libc::c_int as ush,
    0x3ff as libc::c_int as ush,
    0x7ff as libc::c_int as ush,
    0xfff as libc::c_int as ush,
    0x1fff as libc::c_int as ush,
    0x3fff as libc::c_int as ush,
    0x7fff as libc::c_int as ush,
    0xffff as libc::c_int as ush,
];
static mut lbits: libc::c_int = 9 as libc::c_int;
static mut dbits: libc::c_int = 6 as libc::c_int;
static mut hufts: libc::c_uint = 0;
unsafe extern "C" fn huft_build(
    mut b: *mut libc::c_uint,
    mut n: libc::c_uint,
    mut s: libc::c_uint,
    mut d: *mut ush,
    mut e: *mut ush,
    mut t: *mut *mut huft,
    mut m: *mut libc::c_int,
) -> libc::c_int {
    let mut a: libc::c_uint = 0;
    let mut c: [libc::c_uint; 17] = [0; 17];
    let mut f: libc::c_uint = 0;
    let mut g: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut p: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut q: *mut huft = 0 as *mut huft;
    let mut r: huft = huft {
        e: 0,
        b: 0,
        v: C2RustUnnamed { n: 0 },
    };
    let mut u: [*mut huft; 16] = [0 as *mut huft; 16];
    let mut v: [libc::c_uint; 288] = [0; 288];
    let mut w: libc::c_int = 0;
    let mut x: [libc::c_uint; 17] = [0; 17];
    let mut xp: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_uint = 0;
    memset(
        c.as_mut_ptr() as voidp,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uint; 17]>() as libc::c_ulong,
    );
    p = b;
    i = n;
    loop {
        c[*p as usize] = (c[*p as usize]).wrapping_add(1);
        c[*p as usize];
        p = p.offset(1);
        p;
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
    }
    if c[0 as libc::c_int as usize] == n {
        q = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<huft>() as libc::c_ulong),
        ) as *mut huft;
        if q.is_null() {
            return 3 as libc::c_int;
        }
        hufts = hufts.wrapping_add(3 as libc::c_int as libc::c_uint);
        let ref mut fresh0 = (*q.offset(0 as libc::c_int as isize)).v.t;
        *fresh0 = 0 as *mut libc::c_void as *mut huft;
        (*q.offset(1 as libc::c_int as isize)).e = 99 as libc::c_int as uch;
        (*q.offset(1 as libc::c_int as isize)).b = 1 as libc::c_int as uch;
        (*q.offset(2 as libc::c_int as isize)).e = 99 as libc::c_int as uch;
        (*q.offset(2 as libc::c_int as isize)).b = 1 as libc::c_int as uch;
        *t = q.offset(1 as libc::c_int as isize);
        *m = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    l = *m;
    j = 1 as libc::c_int as libc::c_uint;
    while j <= 16 as libc::c_int as libc::c_uint {
        if c[j as usize] != 0 {
            break;
        }
        j = j.wrapping_add(1);
        j;
    }
    k = j as libc::c_int;
    if (l as libc::c_uint) < j {
        l = j as libc::c_int;
    }
    i = 16 as libc::c_int as libc::c_uint;
    while i != 0 {
        if c[i as usize] != 0 {
            break;
        }
        i = i.wrapping_sub(1);
        i;
    }
    g = i as libc::c_int;
    if l as libc::c_uint > i {
        l = i as libc::c_int;
    }
    *m = l;
    y = (1 as libc::c_int) << j;
    while j < i {
        y = (y as libc::c_uint).wrapping_sub(c[j as usize]) as libc::c_int
            as libc::c_int;
        if y < 0 as libc::c_int {
            return 2 as libc::c_int;
        }
        j = j.wrapping_add(1);
        j;
        y <<= 1 as libc::c_int;
    }
    y = (y as libc::c_uint).wrapping_sub(c[i as usize]) as libc::c_int as libc::c_int;
    if y < 0 as libc::c_int {
        return 2 as libc::c_int;
    }
    c[i as usize] = (c[i as usize]).wrapping_add(y as libc::c_uint);
    j = 0 as libc::c_int as libc::c_uint;
    x[1 as libc::c_int as usize] = j;
    p = c.as_mut_ptr().offset(1 as libc::c_int as isize);
    xp = x.as_mut_ptr().offset(2 as libc::c_int as isize);
    loop {
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
        let fresh1 = p;
        p = p.offset(1);
        j = j.wrapping_add(*fresh1);
        let fresh2 = xp;
        xp = xp.offset(1);
        *fresh2 = j;
    }
    p = b;
    i = 0 as libc::c_int as libc::c_uint;
    loop {
        let fresh3 = p;
        p = p.offset(1);
        j = *fresh3;
        if j != 0 as libc::c_int as libc::c_uint {
            let fresh4 = x[j as usize];
            x[j as usize] = (x[j as usize]).wrapping_add(1);
            v[fresh4 as usize] = i;
        }
        i = i.wrapping_add(1);
        if !(i < n) {
            break;
        }
    }
    n = x[g as usize];
    i = 0 as libc::c_int as libc::c_uint;
    x[0 as libc::c_int as usize] = i;
    p = v.as_mut_ptr();
    h = -(1 as libc::c_int);
    w = -l;
    u[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut huft;
    q = 0 as *mut libc::c_void as *mut huft;
    z = 0 as libc::c_int as libc::c_uint;
    while k <= g {
        a = c[k as usize];
        loop {
            let fresh5 = a;
            a = a.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            while k > w + l {
                h += 1;
                h;
                w += l;
                z = (g - w) as libc::c_uint;
                z = if z > l as libc::c_uint { l as libc::c_uint } else { z };
                j = (k - w) as libc::c_uint;
                f = ((1 as libc::c_int) << j) as libc::c_uint;
                if f > a.wrapping_add(1 as libc::c_int as libc::c_uint) {
                    f = f.wrapping_sub(a.wrapping_add(1 as libc::c_int as libc::c_uint));
                    xp = c.as_mut_ptr().offset(k as isize);
                    if j < z {
                        loop {
                            j = j.wrapping_add(1);
                            if !(j < z) {
                                break;
                            }
                            f <<= 1 as libc::c_int;
                            xp = xp.offset(1);
                            if f <= *xp {
                                break;
                            }
                            f = f.wrapping_sub(*xp);
                        }
                    }
                }
                z = ((1 as libc::c_int) << j) as libc::c_uint;
                q = malloc(
                    (z.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<huft>() as libc::c_ulong),
                ) as *mut huft;
                if q.is_null() {
                    if h != 0 {
                        huft_free(u[0 as libc::c_int as usize]);
                    }
                    return 3 as libc::c_int;
                }
                hufts = hufts
                    .wrapping_add(z.wrapping_add(1 as libc::c_int as libc::c_uint));
                *t = q.offset(1 as libc::c_int as isize);
                t = &mut (*q).v.t;
                *t = 0 as *mut libc::c_void as *mut huft;
                q = q.offset(1);
                u[h as usize] = q;
                if h != 0 {
                    x[h as usize] = i;
                    r.b = l as uch;
                    r.e = (16 as libc::c_int as libc::c_uint).wrapping_add(j) as uch;
                    r.v.t = q;
                    j = i >> w - l;
                    *(u[(h - 1 as libc::c_int) as usize]).offset(j as isize) = r;
                }
            }
            r.b = (k - w) as uch;
            if p >= v.as_mut_ptr().offset(n as isize) {
                r.e = 99 as libc::c_int as uch;
            } else if *p < s {
                r
                    .e = (if *p < 256 as libc::c_int as libc::c_uint {
                    16 as libc::c_int
                } else {
                    15 as libc::c_int
                }) as uch;
                r.v.n = *p as ush;
                p = p.offset(1);
                p;
            } else {
                r.e = *e.offset((*p).wrapping_sub(s) as isize) as uch;
                let fresh6 = p;
                p = p.offset(1);
                r.v.n = *d.offset((*fresh6).wrapping_sub(s) as isize);
            }
            f = ((1 as libc::c_int) << k - w) as libc::c_uint;
            j = i >> w;
            while j < z {
                *q.offset(j as isize) = r;
                j = j.wrapping_add(f);
            }
            j = ((1 as libc::c_int) << k - 1 as libc::c_int) as libc::c_uint;
            while i & j != 0 {
                i ^= j;
                j >>= 1 as libc::c_int;
            }
            i ^= j;
            while i & (((1 as libc::c_int) << w) - 1 as libc::c_int) as libc::c_uint
                != x[h as usize]
            {
                h -= 1;
                h;
                w -= l;
            }
        }
        k += 1;
        k;
    }
    return (y != 0 as libc::c_int && g != 1 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn huft_free(mut t: *mut huft) -> libc::c_int {
    let mut p: *mut huft = 0 as *mut huft;
    let mut q: *mut huft = 0 as *mut huft;
    p = t;
    while !p.is_null() {
        p = p.offset(-1);
        q = (*p).v.t;
        rpl_free(p as *mut libc::c_void);
        p = q;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn inflate_codes(
    mut tl: *mut huft,
    mut td: *mut huft,
    mut bl: libc::c_int,
    mut bd: libc::c_int,
) -> libc::c_int {
    let mut e: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    let mut w: libc::c_uint = 0;
    let mut t: *mut huft = 0 as *mut huft;
    let mut ml: libc::c_uint = 0;
    let mut md: libc::c_uint = 0;
    let mut b: ulg = 0;
    let mut k: libc::c_uint = 0;
    b = bb;
    k = bk;
    w = outcnt;
    ml = mask_bits[bl as usize] as libc::c_uint;
    md = mask_bits[bd as usize] as libc::c_uint;
    loop {
        while k < bl as libc::c_uint {
            b
                |= ((if inptr < insize {
                    let fresh7 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh7 as isize) as libc::c_int
                } else {
                    outcnt = w;
                    fill_inbuf(0 as libc::c_int)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        t = tl.offset((b as libc::c_uint & ml) as isize);
        e = (*t).e as libc::c_uint;
        if e > 16 as libc::c_int as libc::c_uint {
            loop {
                if e == 99 as libc::c_int as libc::c_uint {
                    return 1 as libc::c_int;
                }
                b >>= (*t).b as libc::c_int;
                k = k.wrapping_sub((*t).b as libc::c_uint);
                e = e.wrapping_sub(16 as libc::c_int as libc::c_uint);
                while k < e {
                    b
                        |= ((if inptr < insize {
                            let fresh8 = inptr;
                            inptr = inptr.wrapping_add(1);
                            *inbuf.as_mut_ptr().offset(fresh8 as isize) as libc::c_int
                        } else {
                            outcnt = w;
                            fill_inbuf(0 as libc::c_int)
                        }) as uch as ulg) << k;
                    k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                t = ((*t).v.t)
                    .offset(
                        (b as libc::c_uint & mask_bits[e as usize] as libc::c_uint)
                            as isize,
                    );
                e = (*t).e as libc::c_uint;
                if !(e > 16 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
        }
        b >>= (*t).b as libc::c_int;
        k = k.wrapping_sub((*t).b as libc::c_uint);
        if e == 16 as libc::c_int as libc::c_uint {
            let fresh9 = w;
            w = w.wrapping_add(1);
            *window.as_mut_ptr().offset(fresh9 as isize) = (*t).v.n as uch;
            if w == 0x8000 as libc::c_int as libc::c_uint {
                outcnt = w;
                flush_window();
                w = 0 as libc::c_int as libc::c_uint;
            }
        } else {
            if e == 15 as libc::c_int as libc::c_uint {
                break;
            }
            while k < e {
                b
                    |= ((if inptr < insize {
                        let fresh10 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh10 as isize) as libc::c_int
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as libc::c_int)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
            n = ((*t).v.n as libc::c_uint)
                .wrapping_add(b as libc::c_uint & mask_bits[e as usize] as libc::c_uint);
            b >>= e;
            k = k.wrapping_sub(e);
            while k < bd as libc::c_uint {
                b
                    |= ((if inptr < insize {
                        let fresh11 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh11 as isize) as libc::c_int
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as libc::c_int)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
            t = td.offset((b as libc::c_uint & md) as isize);
            e = (*t).e as libc::c_uint;
            if e > 16 as libc::c_int as libc::c_uint {
                loop {
                    if e == 99 as libc::c_int as libc::c_uint {
                        return 1 as libc::c_int;
                    }
                    b >>= (*t).b as libc::c_int;
                    k = k.wrapping_sub((*t).b as libc::c_uint);
                    e = e.wrapping_sub(16 as libc::c_int as libc::c_uint);
                    while k < e {
                        b
                            |= ((if inptr < insize {
                                let fresh12 = inptr;
                                inptr = inptr.wrapping_add(1);
                                *inbuf.as_mut_ptr().offset(fresh12 as isize) as libc::c_int
                            } else {
                                outcnt = w;
                                fill_inbuf(0 as libc::c_int)
                            }) as uch as ulg) << k;
                        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    t = ((*t).v.t)
                        .offset(
                            (b as libc::c_uint & mask_bits[e as usize] as libc::c_uint)
                                as isize,
                        );
                    e = (*t).e as libc::c_uint;
                    if !(e > 16 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
            }
            b >>= (*t).b as libc::c_int;
            k = k.wrapping_sub((*t).b as libc::c_uint);
            while k < e {
                b
                    |= ((if inptr < insize {
                        let fresh13 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh13 as isize) as libc::c_int
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as libc::c_int)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
            d = w
                .wrapping_sub((*t).v.n as libc::c_uint)
                .wrapping_sub(b as libc::c_uint & mask_bits[e as usize] as libc::c_uint);
            b >>= e;
            k = k.wrapping_sub(e);
            loop {
                d &= (0x8000 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
                e = (0x8000 as libc::c_int as libc::c_uint)
                    .wrapping_sub((if d > w { d } else { w }));
                e = if e > n { n } else { e };
                n = n.wrapping_sub(e);
                if e <= (if d < w { w.wrapping_sub(d) } else { d.wrapping_sub(w) }) {
                    memcpy(
                        window.as_mut_ptr().offset(w as isize) as *mut libc::c_void,
                        window.as_mut_ptr().offset(d as isize) as *const libc::c_void,
                        e as libc::c_ulong,
                    );
                    w = w.wrapping_add(e);
                    d = d.wrapping_add(e);
                } else {
                    loop {
                        let fresh14 = d;
                        d = d.wrapping_add(1);
                        let fresh15 = w;
                        w = w.wrapping_add(1);
                        *window
                            .as_mut_ptr()
                            .offset(
                                fresh15 as isize,
                            ) = *window.as_mut_ptr().offset(fresh14 as isize);
                        e = e.wrapping_sub(1);
                        if !(e != 0) {
                            break;
                        }
                    }
                }
                if w == 0x8000 as libc::c_int as libc::c_uint {
                    outcnt = w;
                    flush_window();
                    w = 0 as libc::c_int as libc::c_uint;
                }
                if !(n != 0) {
                    break;
                }
            }
        }
    }
    outcnt = w;
    bb = b;
    bk = k;
    return 0 as libc::c_int;
}
unsafe extern "C" fn inflate_stored() -> libc::c_int {
    let mut n: libc::c_uint = 0;
    let mut w: libc::c_uint = 0;
    let mut b: ulg = 0;
    let mut k: libc::c_uint = 0;
    b = bb;
    k = bk;
    w = outcnt;
    n = k & 7 as libc::c_int as libc::c_uint;
    b >>= n;
    k = k.wrapping_sub(n);
    while k < 16 as libc::c_int as libc::c_uint {
        b
            |= ((if inptr < insize {
                let fresh16 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh16 as isize) as libc::c_int
            } else {
                outcnt = w;
                fill_inbuf(0 as libc::c_int)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    n = b as libc::c_uint & 0xffff as libc::c_int as libc::c_uint;
    b >>= 16 as libc::c_int;
    k = k.wrapping_sub(16 as libc::c_int as libc::c_uint);
    while k < 16 as libc::c_int as libc::c_uint {
        b
            |= ((if inptr < insize {
                let fresh17 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh17 as isize) as libc::c_int
            } else {
                outcnt = w;
                fill_inbuf(0 as libc::c_int)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    if n != (!b & 0xffff as libc::c_int as libc::c_ulong) as libc::c_uint {
        return 1 as libc::c_int;
    }
    b >>= 16 as libc::c_int;
    k = k.wrapping_sub(16 as libc::c_int as libc::c_uint);
    loop {
        let fresh18 = n;
        n = n.wrapping_sub(1);
        if !(fresh18 != 0) {
            break;
        }
        while k < 8 as libc::c_int as libc::c_uint {
            b
                |= ((if inptr < insize {
                    let fresh19 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh19 as isize) as libc::c_int
                } else {
                    outcnt = w;
                    fill_inbuf(0 as libc::c_int)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        let fresh20 = w;
        w = w.wrapping_add(1);
        *window.as_mut_ptr().offset(fresh20 as isize) = b as uch;
        if w == 0x8000 as libc::c_int as libc::c_uint {
            outcnt = w;
            flush_window();
            w = 0 as libc::c_int as libc::c_uint;
        }
        b >>= 8 as libc::c_int;
        k = k.wrapping_sub(8 as libc::c_int as libc::c_uint);
    }
    outcnt = w;
    bb = b;
    bk = k;
    return 0 as libc::c_int;
}
unsafe extern "C" fn inflate_fixed() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tl: *mut huft = 0 as *mut huft;
    let mut td: *mut huft = 0 as *mut huft;
    let mut bl: libc::c_int = 0;
    let mut bd: libc::c_int = 0;
    let mut l: [libc::c_uint; 288] = [0; 288];
    i = 0 as libc::c_int;
    while i < 144 as libc::c_int {
        l[i as usize] = 8 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    while i < 256 as libc::c_int {
        l[i as usize] = 9 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    while i < 280 as libc::c_int {
        l[i as usize] = 7 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    while i < 288 as libc::c_int {
        l[i as usize] = 8 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    bl = 7 as libc::c_int;
    i = huft_build(
        l.as_mut_ptr(),
        288 as libc::c_int as libc::c_uint,
        257 as libc::c_int as libc::c_uint,
        cplens.as_mut_ptr(),
        cplext.as_mut_ptr(),
        &mut tl,
        &mut bl,
    );
    if i != 0 as libc::c_int {
        return i;
    }
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        l[i as usize] = 5 as libc::c_int as libc::c_uint;
        i += 1;
        i;
    }
    bd = 5 as libc::c_int;
    i = huft_build(
        l.as_mut_ptr(),
        30 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        cpdist.as_mut_ptr(),
        cpdext.as_mut_ptr(),
        &mut td,
        &mut bd,
    );
    if i > 1 as libc::c_int {
        huft_free(tl);
        return i;
    }
    if inflate_codes(tl, td, bl, bd) != 0 {
        return 1 as libc::c_int;
    }
    huft_free(tl);
    huft_free(td);
    return 0 as libc::c_int;
}
unsafe extern "C" fn inflate_dynamic() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    let mut l: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut w: libc::c_uint = 0;
    let mut tl: *mut huft = 0 as *mut huft;
    let mut td: *mut huft = 0 as *mut huft;
    let mut bl: libc::c_int = 0;
    let mut bd: libc::c_int = 0;
    let mut nb: libc::c_uint = 0;
    let mut nl: libc::c_uint = 0;
    let mut nd: libc::c_uint = 0;
    let mut ll: [libc::c_uint; 316] = [0; 316];
    let mut b: ulg = 0;
    let mut k: libc::c_uint = 0;
    b = bb;
    k = bk;
    w = outcnt;
    while k < 5 as libc::c_int as libc::c_uint {
        b
            |= ((if inptr < insize {
                let fresh21 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh21 as isize) as libc::c_int
            } else {
                outcnt = w;
                fill_inbuf(0 as libc::c_int)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    nl = (257 as libc::c_int as libc::c_uint)
        .wrapping_add(b as libc::c_uint & 0x1f as libc::c_int as libc::c_uint);
    b >>= 5 as libc::c_int;
    k = k.wrapping_sub(5 as libc::c_int as libc::c_uint);
    while k < 5 as libc::c_int as libc::c_uint {
        b
            |= ((if inptr < insize {
                let fresh22 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh22 as isize) as libc::c_int
            } else {
                outcnt = w;
                fill_inbuf(0 as libc::c_int)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    nd = (1 as libc::c_int as libc::c_uint)
        .wrapping_add(b as libc::c_uint & 0x1f as libc::c_int as libc::c_uint);
    b >>= 5 as libc::c_int;
    k = k.wrapping_sub(5 as libc::c_int as libc::c_uint);
    while k < 4 as libc::c_int as libc::c_uint {
        b
            |= ((if inptr < insize {
                let fresh23 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh23 as isize) as libc::c_int
            } else {
                outcnt = w;
                fill_inbuf(0 as libc::c_int)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    nb = (4 as libc::c_int as libc::c_uint)
        .wrapping_add(b as libc::c_uint & 0xf as libc::c_int as libc::c_uint);
    b >>= 4 as libc::c_int;
    k = k.wrapping_sub(4 as libc::c_int as libc::c_uint);
    if nl > 286 as libc::c_int as libc::c_uint || nd > 30 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    j = 0 as libc::c_int as libc::c_uint;
    while j < nb {
        while k < 3 as libc::c_int as libc::c_uint {
            b
                |= ((if inptr < insize {
                    let fresh24 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh24 as isize) as libc::c_int
                } else {
                    outcnt = w;
                    fill_inbuf(0 as libc::c_int)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        ll[border[j as usize]
            as usize] = b as libc::c_uint & 7 as libc::c_int as libc::c_uint;
        b >>= 3 as libc::c_int;
        k = k.wrapping_sub(3 as libc::c_int as libc::c_uint);
        j = j.wrapping_add(1);
        j;
    }
    while j < 19 as libc::c_int as libc::c_uint {
        ll[border[j as usize] as usize] = 0 as libc::c_int as libc::c_uint;
        j = j.wrapping_add(1);
        j;
    }
    bl = 7 as libc::c_int;
    i = huft_build(
        ll.as_mut_ptr(),
        19 as libc::c_int as libc::c_uint,
        19 as libc::c_int as libc::c_uint,
        0 as *mut ush,
        0 as *mut ush,
        &mut tl,
        &mut bl,
    );
    if i != 0 as libc::c_int {
        if i == 1 as libc::c_int {
            huft_free(tl);
        }
        return i;
    }
    if tl.is_null() {
        return 2 as libc::c_int;
    }
    n = nl.wrapping_add(nd);
    m = mask_bits[bl as usize] as libc::c_uint;
    l = 0 as libc::c_int as libc::c_uint;
    i = l as libc::c_int;
    while (i as libc::c_uint) < n {
        while k < bl as libc::c_uint {
            b
                |= ((if inptr < insize {
                    let fresh25 = inptr;
                    inptr = inptr.wrapping_add(1);
                    *inbuf.as_mut_ptr().offset(fresh25 as isize) as libc::c_int
                } else {
                    outcnt = w;
                    fill_inbuf(0 as libc::c_int)
                }) as uch as ulg) << k;
            k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        td = tl.offset((b as libc::c_uint & m) as isize);
        j = (*td).b as libc::c_uint;
        b >>= j;
        k = k.wrapping_sub(j);
        if (*td).e as libc::c_int == 99 as libc::c_int {
            huft_free(tl);
            return 2 as libc::c_int;
        }
        j = (*td).v.n as libc::c_uint;
        if j < 16 as libc::c_int as libc::c_uint {
            l = j;
            let fresh26 = i;
            i = i + 1;
            ll[fresh26 as usize] = l;
        } else if j == 16 as libc::c_int as libc::c_uint {
            while k < 2 as libc::c_int as libc::c_uint {
                b
                    |= ((if inptr < insize {
                        let fresh27 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh27 as isize) as libc::c_int
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as libc::c_int)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
            j = (3 as libc::c_int as libc::c_uint)
                .wrapping_add(b as libc::c_uint & 3 as libc::c_int as libc::c_uint);
            b >>= 2 as libc::c_int;
            k = k.wrapping_sub(2 as libc::c_int as libc::c_uint);
            if (i as libc::c_uint).wrapping_add(j) > n {
                return 1 as libc::c_int;
            }
            loop {
                let fresh28 = j;
                j = j.wrapping_sub(1);
                if !(fresh28 != 0) {
                    break;
                }
                let fresh29 = i;
                i = i + 1;
                ll[fresh29 as usize] = l;
            }
        } else if j == 17 as libc::c_int as libc::c_uint {
            while k < 3 as libc::c_int as libc::c_uint {
                b
                    |= ((if inptr < insize {
                        let fresh30 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh30 as isize) as libc::c_int
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as libc::c_int)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
            j = (3 as libc::c_int as libc::c_uint)
                .wrapping_add(b as libc::c_uint & 7 as libc::c_int as libc::c_uint);
            b >>= 3 as libc::c_int;
            k = k.wrapping_sub(3 as libc::c_int as libc::c_uint);
            if (i as libc::c_uint).wrapping_add(j) > n {
                return 1 as libc::c_int;
            }
            loop {
                let fresh31 = j;
                j = j.wrapping_sub(1);
                if !(fresh31 != 0) {
                    break;
                }
                let fresh32 = i;
                i = i + 1;
                ll[fresh32 as usize] = 0 as libc::c_int as libc::c_uint;
            }
            l = 0 as libc::c_int as libc::c_uint;
        } else {
            while k < 7 as libc::c_int as libc::c_uint {
                b
                    |= ((if inptr < insize {
                        let fresh33 = inptr;
                        inptr = inptr.wrapping_add(1);
                        *inbuf.as_mut_ptr().offset(fresh33 as isize) as libc::c_int
                    } else {
                        outcnt = w;
                        fill_inbuf(0 as libc::c_int)
                    }) as uch as ulg) << k;
                k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
            }
            j = (11 as libc::c_int as libc::c_uint)
                .wrapping_add(b as libc::c_uint & 0x7f as libc::c_int as libc::c_uint);
            b >>= 7 as libc::c_int;
            k = k.wrapping_sub(7 as libc::c_int as libc::c_uint);
            if (i as libc::c_uint).wrapping_add(j) > n {
                return 1 as libc::c_int;
            }
            loop {
                let fresh34 = j;
                j = j.wrapping_sub(1);
                if !(fresh34 != 0) {
                    break;
                }
                let fresh35 = i;
                i = i + 1;
                ll[fresh35 as usize] = 0 as libc::c_int as libc::c_uint;
            }
            l = 0 as libc::c_int as libc::c_uint;
        }
    }
    huft_free(tl);
    bb = b;
    bk = k;
    bl = lbits;
    i = huft_build(
        ll.as_mut_ptr(),
        nl,
        257 as libc::c_int as libc::c_uint,
        cplens.as_mut_ptr(),
        cplext.as_mut_ptr(),
        &mut tl,
        &mut bl,
    );
    if i != 0 as libc::c_int {
        if i == 1 as libc::c_int {
            huft_free(tl);
        }
        return i;
    }
    bd = dbits;
    i = huft_build(
        ll.as_mut_ptr().offset(nl as isize),
        nd,
        0 as libc::c_int as libc::c_uint,
        cpdist.as_mut_ptr(),
        cpdext.as_mut_ptr(),
        &mut td,
        &mut bd,
    );
    if i != 0 as libc::c_int {
        if i == 1 as libc::c_int {
            huft_free(td);
        }
        huft_free(tl);
        return i;
    }
    let mut err: libc::c_int = if inflate_codes(tl, td, bl, bd) != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    huft_free(tl);
    huft_free(td);
    return err;
}
unsafe extern "C" fn inflate_block(mut e: *mut libc::c_int) -> libc::c_int {
    let mut t: libc::c_uint = 0;
    let mut w: libc::c_uint = 0;
    let mut b: ulg = 0;
    let mut k: libc::c_uint = 0;
    b = bb;
    k = bk;
    w = outcnt;
    while k < 1 as libc::c_int as libc::c_uint {
        b
            |= ((if inptr < insize {
                let fresh36 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh36 as isize) as libc::c_int
            } else {
                outcnt = w;
                fill_inbuf(0 as libc::c_int)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    *e = b as libc::c_int & 1 as libc::c_int;
    b >>= 1 as libc::c_int;
    k = k.wrapping_sub(1 as libc::c_int as libc::c_uint);
    while k < 2 as libc::c_int as libc::c_uint {
        b
            |= ((if inptr < insize {
                let fresh37 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh37 as isize) as libc::c_int
            } else {
                outcnt = w;
                fill_inbuf(0 as libc::c_int)
            }) as uch as ulg) << k;
        k = k.wrapping_add(8 as libc::c_int as libc::c_uint);
    }
    t = b as libc::c_uint & 3 as libc::c_int as libc::c_uint;
    b >>= 2 as libc::c_int;
    k = k.wrapping_sub(2 as libc::c_int as libc::c_uint);
    bb = b;
    bk = k;
    if t == 2 as libc::c_int as libc::c_uint {
        return inflate_dynamic();
    }
    if t == 0 as libc::c_int as libc::c_uint {
        return inflate_stored();
    }
    if t == 1 as libc::c_int as libc::c_uint {
        return inflate_fixed();
    }
    return 2 as libc::c_int;
}
pub unsafe extern "C" fn inflate() -> libc::c_int {
    let mut e: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut h: libc::c_uint = 0;
    outcnt = 0 as libc::c_int as libc::c_uint;
    bk = 0 as libc::c_int as libc::c_uint;
    bb = 0 as libc::c_int as ulg;
    h = 0 as libc::c_int as libc::c_uint;
    loop {
        hufts = 0 as libc::c_int as libc::c_uint;
        r = inflate_block(&mut e);
        if r != 0 as libc::c_int {
            return r;
        }
        if hufts > h {
            h = hufts;
        }
        if !(e == 0) {
            break;
        }
    }
    while bk >= 8 as libc::c_int as libc::c_uint {
        bk = bk.wrapping_sub(8 as libc::c_int as libc::c_uint);
        inptr = inptr.wrapping_sub(1);
        inptr;
    }
    outcnt = outcnt;
    flush_window();
    return 0 as libc::c_int;
}
