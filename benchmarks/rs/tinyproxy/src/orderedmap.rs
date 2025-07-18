use ::libc;
extern "C" {
    pub type htab;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn sblist_new(itemsize: size_t, blockitems: size_t) -> *mut sblist;
    fn sblist_free(l: *mut sblist);
    fn sblist_get(l: *mut sblist, item: size_t) -> *mut libc::c_void;
    fn sblist_add(l: *mut sblist, item: *mut libc::c_void) -> libc::c_int;
    fn sblist_delete(l: *mut sblist, item: size_t);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn htab_create(_: size_t) -> *mut htab;
    fn htab_destroy(_: *mut htab);
    fn htab_find(_: *mut htab, key: *const libc::c_char) -> *mut htab_value;
    fn htab_find2(
        htab: *mut htab,
        key: *const libc::c_char,
        saved_key: *mut *mut libc::c_char,
    ) -> *mut htab_value;
    fn htab_insert(_: *mut htab, _: *mut libc::c_char, _: htab_value) -> libc::c_int;
    fn htab_delete(htab: *mut htab, key: *const libc::c_char) -> libc::c_int;
    fn htab_next(
        _: *mut htab,
        iterator: size_t,
        key: *mut *mut libc::c_char,
        v: *mut *mut htab_value,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sblist {
    pub itemsize: size_t,
    pub blockitems: size_t,
    pub count: size_t,
    pub capa: size_t,
    pub items: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union htab_value {
    pub p: *mut libc::c_void,
    pub n: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct orderedmap {
    pub values: *mut sblist,
    pub map: *mut htab,
}
unsafe extern "C" fn orderedmap_destroy_contents(mut o: *mut orderedmap) {
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut v: *mut htab_value = 0 as *mut htab_value;
    if o.is_null() {
        return;
    }
    if !((*o).values).is_null() {
        while (*(*o).values).count != 0 {
            p = sblist_get((*o).values, 0 as libc::c_int as size_t)
                as *mut *mut libc::c_char;
            if !p.is_null() {
                free(*p as *mut libc::c_void);
            }
            sblist_delete((*o).values, 0 as libc::c_int as size_t);
        }
        sblist_free((*o).values);
    }
    if !((*o).map).is_null() {
        i = 0 as libc::c_int as size_t;
        loop {
            i = htab_next((*o).map, i, &mut q, &mut v);
            if !(i != 0) {
                break;
            }
            free(q as *mut libc::c_void);
        }
        htab_destroy((*o).map);
    }
}
pub unsafe extern "C" fn orderedmap_create(mut nbuckets: size_t) -> *mut orderedmap {
    let mut o: orderedmap = {
        let mut init = orderedmap {
            values: 0 as *mut sblist,
            map: 0 as *mut htab,
        };
        init
    };
    let mut new: *mut orderedmap = 0 as *mut orderedmap;
    o
        .values = sblist_new(
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        32 as libc::c_int as size_t,
    );
    if !(o.values).is_null() {
        o.map = htab_create(nbuckets);
        if !(o.map).is_null() {
            new = malloc(::std::mem::size_of::<orderedmap>() as libc::c_ulong)
                as *mut orderedmap;
            if !new.is_null() {
                memcpy(
                    new as *mut libc::c_void,
                    &mut o as *mut orderedmap as *const libc::c_void,
                    ::std::mem::size_of::<orderedmap>() as libc::c_ulong,
                );
                return new;
            }
        }
    }
    orderedmap_destroy_contents(&mut o);
    return 0 as *mut orderedmap;
}
pub unsafe extern "C" fn orderedmap_destroy(
    mut o: *mut orderedmap,
) -> *mut libc::c_void {
    orderedmap_destroy_contents(o);
    free(o as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn orderedmap_append(
    mut o: *mut orderedmap,
    mut key: *const libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    let mut index: size_t = 0;
    let mut nk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nv: *mut libc::c_char = 0 as *mut libc::c_char;
    nv = 0 as *mut libc::c_char;
    nk = nv;
    nk = strdup(key);
    nv = strdup(value);
    if !(nk.is_null() || nv.is_null()) {
        index = (*(*o).values).count;
        if !(sblist_add(
            (*o).values,
            &mut nv as *mut *mut libc::c_char as *mut libc::c_void,
        ) == 0)
        {
            if htab_insert((*o).map, nk, htab_value { n: index }) == 0 {
                sblist_delete((*o).values, index);
            } else {
                return 1 as libc::c_int
            }
        }
    }
    free(nk as *mut libc::c_void);
    free(nv as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn orderedmap_find(
    mut o: *mut orderedmap,
    mut key: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut v: *mut htab_value = htab_find((*o).map, key);
    if v.is_null() {
        return 0 as *mut libc::c_char;
    }
    p = sblist_get((*o).values, (*v).n) as *mut *mut libc::c_char;
    return if !p.is_null() { *p } else { 0 as *mut libc::c_char };
}
pub unsafe extern "C" fn orderedmap_remove(
    mut o: *mut orderedmap,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut lk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sk: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lv: *mut htab_value = 0 as *mut htab_value;
    let mut v: *mut htab_value = htab_find2((*o).map, key, &mut sk);
    if v.is_null() {
        return 0 as libc::c_int;
    }
    sv = sblist_get((*o).values, (*v).n) as *mut *mut libc::c_char;
    free(*sv as *mut libc::c_void);
    sblist_delete((*o).values, (*v).n);
    i = 0 as libc::c_int as size_t;
    loop {
        i = htab_next((*o).map, i, &mut lk, &mut lv);
        if !(i != 0) {
            break;
        }
        if (*lv).n > (*v).n {
            (*lv).n = ((*lv).n).wrapping_sub(1);
            (*lv).n;
        }
    }
    htab_delete((*o).map, key);
    free(sk as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn orderedmap_next(
    mut o: *mut orderedmap,
    mut iter: size_t,
    mut key: *mut *mut libc::c_char,
    mut value: *mut *mut libc::c_char,
) -> size_t {
    let mut h_iter: size_t = 0;
    let mut hval: *mut htab_value = 0 as *mut htab_value;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if iter < (*(*o).values).count {
        h_iter = 0 as libc::c_int as size_t;
        loop {
            h_iter = htab_next((*o).map, h_iter, key, &mut hval);
            if !(h_iter != 0) {
                break;
            }
            if (*hval).n == iter {
                p = sblist_get((*o).values, iter) as *mut *mut libc::c_char;
                *value = if !p.is_null() { *p } else { 0 as *mut libc::c_char };
                return iter.wrapping_add(1 as libc::c_int as libc::c_ulong);
            }
        }
    }
    return 0 as libc::c_int as size_t;
}
