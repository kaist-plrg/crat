use ::libc;
extern "C" {
    fn rand() -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union htab_value {
    pub p: *mut libc::c_void,
    pub n: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htab {
    pub elems: *mut elem,
    pub mask: size_t,
    pub used: size_t,
    pub seed: size_t,
    pub dead: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elem {
    pub item: htab_entry,
    pub hash: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htab_entry {
    pub key: *mut libc::c_char,
    pub data: htab_value,
}
unsafe extern "C" fn keyhash(mut k: *const libc::c_char, mut seed: size_t) -> size_t {
    let mut p: *const libc::c_uchar = k as *const libc::c_void as *const libc::c_uchar;
    let mut h: size_t = seed;
    while *p != 0 {
        let fresh0 = p;
        p = p.offset(1);
        h = (31 as libc::c_int as libc::c_ulong)
            .wrapping_mul(h)
            .wrapping_add(tolower(*fresh0 as libc::c_int) as libc::c_ulong);
    }
    return h;
}
unsafe extern "C" fn resize(mut htab: *mut htab, mut nel: size_t) -> libc::c_int {
    let mut newsize: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut oldmask: size_t = (*htab).mask;
    let mut e: *mut elem = 0 as *mut elem;
    let mut newe: *mut elem = 0 as *mut elem;
    let mut oldtab: *mut elem = (*htab).elems;
    let mut oldend: *mut elem = 0 as *mut elem;
    if nel
        > (-(1 as libc::c_int) as size_t)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        nel = (-(1 as libc::c_int) as size_t)
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    newsize = 8 as libc::c_int as size_t;
    while newsize < nel {
        newsize = (newsize as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    (*htab)
        .elems = calloc(newsize, ::std::mem::size_of::<elem>() as libc::c_ulong)
        as *mut elem;
    if ((*htab).elems).is_null() {
        (*htab).elems = oldtab;
        return 0 as libc::c_int;
    }
    (*htab).mask = newsize.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if oldtab.is_null() {
        return 1 as libc::c_int;
    }
    oldend = oldtab.offset(oldmask as isize).offset(1 as libc::c_int as isize);
    e = oldtab;
    while e < oldend {
        if !((*e).item.key).is_null() {
            i = (*e).hash;
            j = 1 as libc::c_int as size_t;
            loop {
                newe = ((*htab).elems).offset((i & (*htab).mask) as isize);
                if ((*newe).item.key).is_null() {
                    break;
                }
                let fresh1 = j;
                j = j.wrapping_add(1);
                i = (i as libc::c_ulong).wrapping_add(fresh1) as size_t as size_t;
            }
            *newe = *e;
        }
        e = e.offset(1);
        e;
    }
    free(oldtab as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn lookup(
    mut htab: *mut htab,
    mut key: *const libc::c_char,
    mut hash: size_t,
    mut dead: size_t,
) -> *mut elem {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut e: *mut elem = 0 as *mut elem;
    i = hash;
    j = 1 as libc::c_int as size_t;
    loop {
        e = ((*htab).elems).offset((i & (*htab).mask) as isize);
        if ((*e).item.key).is_null() && ((*e).hash == 0 || (*e).hash == dead)
            || (*e).hash == hash && strcasecmp((*e).item.key, key) == 0 as libc::c_int
        {
            break;
        }
        let fresh2 = j;
        j = j.wrapping_add(1);
        i = (i as libc::c_ulong).wrapping_add(fresh2) as size_t as size_t;
    }
    return e;
}
pub unsafe extern "C" fn htab_create(mut nel: size_t) -> *mut htab {
    let mut r: *mut htab = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htab>() as libc::c_ulong,
    ) as *mut htab;
    if !r.is_null() && resize(r, nel) == 0 {
        free(r as *mut libc::c_void);
        r = 0 as *mut htab;
    }
    (*r).seed = rand() as size_t;
    return r;
}
pub unsafe extern "C" fn htab_destroy(mut htab: *mut htab) {
    free((*htab).elems as *mut libc::c_void);
    free(htab as *mut libc::c_void);
}
unsafe extern "C" fn htab_find_elem(
    mut htab: *mut htab,
    mut key: *const libc::c_char,
) -> *mut elem {
    let mut hash: size_t = keyhash(key, (*htab).seed);
    let mut e: *mut elem = lookup(htab, key, hash, 0 as libc::c_int as size_t);
    if !((*e).item.key).is_null() {
        return e;
    }
    return 0 as *mut elem;
}
pub unsafe extern "C" fn htab_find(
    mut htab: *mut htab,
    mut key: *const libc::c_char,
) -> *mut htab_value {
    let mut e: *mut elem = htab_find_elem(htab, key);
    if e.is_null() {
        return 0 as *mut htab_value;
    }
    return &mut (*e).item.data;
}
pub unsafe extern "C" fn htab_find2(
    mut htab: *mut htab,
    mut key: *const libc::c_char,
    mut saved_key: *mut *mut libc::c_char,
) -> *mut htab_value {
    let mut e: *mut elem = htab_find_elem(htab, key);
    if e.is_null() {
        return 0 as *mut htab_value;
    }
    *saved_key = (*e).item.key;
    return &mut (*e).item.data;
}
pub unsafe extern "C" fn htab_delete(
    mut htab: *mut htab,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut e: *mut elem = htab_find_elem(htab, key);
    if e.is_null() {
        return 0 as libc::c_int;
    }
    (*e).item.key = 0 as *mut libc::c_char;
    (*e).hash = 0xdeadc0de as libc::c_uint as size_t;
    (*htab).used = ((*htab).used).wrapping_sub(1);
    (*htab).used;
    (*htab).dead = ((*htab).dead).wrapping_add(1);
    (*htab).dead;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn htab_insert(
    mut htab: *mut htab,
    mut key: *mut libc::c_char,
    mut value: htab_value,
) -> libc::c_int {
    let mut hash: size_t = keyhash(key, (*htab).seed);
    let mut oh: size_t = 0;
    let mut e: *mut elem = lookup(htab, key, hash, 0xdeadc0de as libc::c_uint as size_t);
    if !((*e).item.key).is_null() {
        return 0 as libc::c_int;
    }
    oh = (*e).hash;
    (*e).item.key = key;
    (*e).item.data = value;
    (*e).hash = hash;
    (*htab).used = ((*htab).used).wrapping_add(1);
    if ((*htab).used).wrapping_add((*htab).dead)
        > ((*htab).mask)
            .wrapping_sub(((*htab).mask).wrapping_div(4 as libc::c_int as libc::c_ulong))
    {
        if resize(htab, (2 as libc::c_int as libc::c_ulong).wrapping_mul((*htab).used))
            == 0
        {
            (*htab).used = ((*htab).used).wrapping_sub(1);
            (*htab).used;
            (*e).item.key = 0 as *mut libc::c_char;
            (*e).hash = oh;
            return 0 as libc::c_int;
        }
        (*htab).dead = 0 as libc::c_int as size_t;
    } else if oh == 0xdeadc0de as libc::c_uint as libc::c_ulong {
        (*htab).dead = ((*htab).dead).wrapping_sub(1);
        (*htab).dead;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn htab_next(
    mut htab: *mut htab,
    mut iterator: size_t,
    mut key: *mut *mut libc::c_char,
    mut v: *mut *mut htab_value,
) -> size_t {
    let mut i: size_t = 0;
    i = iterator;
    while i < ((*htab).mask).wrapping_add(1 as libc::c_int as libc::c_ulong) {
        let mut e: *mut elem = ((*htab).elems).offset(i as isize);
        if !((*e).item.key).is_null() {
            *key = (*e).item.key;
            *v = &mut (*e).item.data;
            return i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as size_t;
}
