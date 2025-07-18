use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strm_str_new(_: *const libc::c_char, _: strm_int) -> strm_string;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intptr_t = libc::c_long;
pub type int32_t = __int32_t;
pub type intptr_t = __intptr_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strm_state {
    pub env: *mut libc::c_void,
    pub prev: *mut strm_state,
    pub flags: uint32_t,
}
pub type strm_int = int32_t;
pub type strm_string = uint64_t;
pub type khiter_t = khint_t;
pub type khint_t = khint32_t;
pub type khint32_t = libc::c_uint;
pub type kh_ns_t = kh_ns_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_ns_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut khint64_t,
    pub vals: *mut *mut strm_state,
}
pub type khint64_t = libc::c_ulong;
static mut khash_ac_HASH_UPPER: libc::c_double = 0.77f64;
#[inline]
unsafe extern "C" fn kh_init_ns() -> *mut kh_ns_t {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<kh_ns_t>() as libc::c_ulong,
    ) as *mut kh_ns_t;
}
#[inline]
unsafe extern "C" fn kh_resize_ns(
    mut h: *mut kh_ns_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets;
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    new_n_buckets;
    if new_n_buckets < 4 as libc::c_int as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * khash_ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        new_flags = malloc(
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        ) as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        memset(
            new_flags as *mut libc::c_void,
            0xaa as libc::c_int,
            ((if new_n_buckets < 16 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_uint
            } else {
                new_n_buckets >> 4 as libc::c_int
            }) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            let mut new_keys: *mut khint64_t = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<khint64_t>() as libc::c_ulong),
            ) as *mut khint64_t;
            if new_keys.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            let mut new_vals: *mut *mut strm_state = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut strm_state>() as libc::c_ulong,
                    ),
            ) as *mut *mut strm_state;
            if new_vals.is_null() {
                free(new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            {
                let mut key: khint64_t = *((*h).keys).offset(j as isize);
                let mut val: *mut strm_state = 0 as *mut strm_state;
                let mut new_mask: khint_t = 0;
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_int as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                let ref mut fresh0 = *((*h).flags)
                    .offset((j >> 4 as libc::c_int) as isize);
                *fresh0 = (*fresh0 as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 0xf as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    let mut k: khint_t = 0;
                    let mut i: khint_t = 0;
                    let mut step: khint_t = 0 as libc::c_int as khint_t;
                    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int)
                        as khint32_t;
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_int as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    let ref mut fresh1 = *new_flags
                        .offset((i >> 4 as libc::c_int) as isize);
                    *fresh1 = (*fresh1 as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 0xf as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets
                        && *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_int as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        let mut tmp: khint64_t = *((*h).keys).offset(i as isize);
                        *((*h).keys).offset(i as isize) = key;
                        key = tmp;
                        let mut tmp_0: *mut strm_state = *((*h).vals).offset(i as isize);
                        let ref mut fresh2 = *((*h).vals).offset(i as isize);
                        *fresh2 = val;
                        val = tmp_0;
                        let ref mut fresh3 = *((*h).flags)
                            .offset((i >> 4 as libc::c_int) as isize);
                        *fresh3 = (*fresh3 as libc::c_ulong
                            | (1 as libc::c_ulong)
                                << ((i & 0xf as libc::c_uint) << 1 as libc::c_int))
                            as khint32_t;
                    } else {
                        *((*h).keys).offset(i as isize) = key;
                        let ref mut fresh4 = *((*h).vals).offset(i as isize);
                        *fresh4 = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if (*h).n_buckets > new_n_buckets {
            (*h)
                .keys = realloc(
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<khint64_t>() as libc::c_ulong),
            ) as *mut khint64_t;
            (*h)
                .vals = realloc(
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut strm_state>() as libc::c_ulong,
                    ),
            ) as *mut *mut strm_state;
        }
        free((*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * khash_ac_HASH_UPPER
            + 0.5f64) as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_get_ns(mut h: *const kh_ns_t, mut key: khint64_t) -> khint_t {
    if (*h).n_buckets != 0 {
        let mut k: khint_t = 0;
        let mut i: khint_t = 0;
        let mut last: khint_t = 0;
        let mut mask: khint_t = 0;
        let mut step: khint_t = 0 as libc::c_int as khint_t;
        mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint);
        k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int) as khint32_t;
        i = k & mask;
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) == key))
        {
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if i == last {
                return (*h).n_buckets;
            }
        }
        return if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 3 as libc::c_int as libc::c_uint != 0
        {
            (*h).n_buckets
        } else {
            i
        };
    } else {
        return 0 as libc::c_int as khint_t
    };
}
#[inline]
unsafe extern "C" fn kh_put_ns(
    mut h: *mut kh_ns_t,
    mut key: khint64_t,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            if kh_resize_ns(
                h,
                ((*h).n_buckets).wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) < 0 as libc::c_int
            {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else if kh_resize_ns(
            h,
            ((*h).n_buckets).wrapping_add(1 as libc::c_int as libc::c_uint),
        ) < 0 as libc::c_int
        {
            *ret = -(1 as libc::c_int);
            return (*h).n_buckets;
        }
    }
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = ((*h).n_buckets)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut step: khint_t = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = (key >> 33 as libc::c_int ^ key ^ key << 11 as libc::c_int) as khint32_t;
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 2 as libc::c_int as libc::c_uint == 0
            && (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
                || !(*((*h).keys).offset(i as isize) == key))
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 1 as libc::c_int as libc::c_uint != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 0xf as libc::c_uint) << 1 as libc::c_int)
                & 2 as libc::c_int as libc::c_uint != 0 && site != (*h).n_buckets
            {
                x = site;
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 2 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh5 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh5 = (*fresh5 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        (*h).n_occupied;
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 0xf as libc::c_uint) << 1 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        let ref mut fresh6 = *((*h).flags).offset((x >> 4 as libc::c_int) as isize);
        *fresh6 = (*fresh6 as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 0xf as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).size;
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
static mut nstbl: *mut kh_ns_t = 0 as *const kh_ns_t as *mut kh_ns_t;
static mut szero: strm_state = {
    let mut init = strm_state {
        env: 0 as *const libc::c_void as *mut libc::c_void,
        prev: 0 as *const strm_state as *mut strm_state,
        flags: 0,
    };
    init
};
pub unsafe extern "C" fn strm_ns_name(mut state: *mut strm_state) -> strm_string {
    let mut k: khiter_t = 0;
    if nstbl.is_null() {
        return 0 as libc::c_int as strm_string;
    }
    k = 0 as libc::c_int as khint_t;
    while k != (*nstbl).n_buckets {
        if *((*nstbl).flags).offset((k >> 4 as libc::c_int) as isize)
            >> ((k & 0xf as libc::c_uint) << 1 as libc::c_int)
            & 3 as libc::c_int as libc::c_uint == 0
        {
            if *((*nstbl).vals).offset(k as isize) == state {
                return *((*nstbl).keys).offset(k as isize);
            }
        }
        k = k.wrapping_add(1);
        k;
    }
    return 0 as libc::c_int as strm_string;
}
pub unsafe extern "C" fn strm_ns_get(mut name: strm_string) -> *mut strm_state {
    let mut k: khiter_t = 0;
    if nstbl.is_null() {
        return 0 as *mut strm_state;
    }
    k = kh_get_ns(nstbl, name as intptr_t as khint64_t);
    if k == (*nstbl).n_buckets {
        return 0 as *mut strm_state;
    }
    return *((*nstbl).vals).offset(k as isize);
}
pub unsafe extern "C" fn strm_ns_create(
    mut state: *mut strm_state,
    mut name: strm_string,
) -> *mut strm_state {
    let mut s: *mut strm_state = strm_ns_get(name);
    if s.is_null() {
        let mut r: libc::c_int = 0;
        let mut k: khiter_t = 0;
        if nstbl.is_null() {
            nstbl = kh_init_ns();
        }
        k = kh_put_ns(nstbl, name as intptr_t as khint64_t, &mut r);
        if r < 0 as libc::c_int {
            return 0 as *mut strm_state;
        }
        if r == 0 as libc::c_int {
            if !(*((*nstbl).vals).offset(k as isize)).is_null() {
                return 0 as *mut strm_state;
            }
        }
        s = malloc(::std::mem::size_of::<strm_state>() as libc::c_ulong)
            as *mut strm_state;
        if !s.is_null() {
            *s = szero;
            (*s).prev = state;
        }
        let ref mut fresh7 = *((*nstbl).vals).offset(k as isize);
        *fresh7 = s;
    }
    return s;
}
pub unsafe extern "C" fn strm_ns_new(
    mut state: *mut strm_state,
    mut name: *const libc::c_char,
) -> *mut strm_state {
    let mut s: strm_string = strm_str_new(name, strlen(name) as strm_int);
    return strm_ns_create(state, s);
}
