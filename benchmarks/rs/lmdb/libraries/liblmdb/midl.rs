use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type mdb_size_t = size_t;
pub type MDB_ID = mdb_size_t;
pub type MDB_IDL = *mut MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ID2 {
    pub mid: MDB_ID,
    pub mptr: *mut libc::c_void,
}
pub type MDB_ID2L = *mut MDB_ID2;
pub unsafe extern "C" fn mdb_midl_search(
    mut ids: MDB_IDL,
    mut id: MDB_ID,
) -> libc::c_uint {
    let mut base: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cursor: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_uint = *ids.offset(0 as libc::c_int as isize) as libc::c_uint;
    while (0 as libc::c_int as libc::c_uint) < n {
        let mut pivot: libc::c_uint = n >> 1 as libc::c_int;
        cursor = base.wrapping_add(pivot).wrapping_add(1 as libc::c_int as libc::c_uint);
        val = if *ids.offset(cursor as isize) < id {
            -(1 as libc::c_int)
        } else {
            (*ids.offset(cursor as isize) > id) as libc::c_int
        };
        if val < 0 as libc::c_int {
            n = pivot;
        } else if val > 0 as libc::c_int {
            base = cursor;
            n = n.wrapping_sub(pivot.wrapping_add(1 as libc::c_int as libc::c_uint));
        } else {
            return cursor
        }
    }
    if val > 0 as libc::c_int {
        cursor = cursor.wrapping_add(1);
        cursor;
    }
    return cursor;
}
pub unsafe extern "C" fn mdb_midl_alloc(mut num: libc::c_int) -> MDB_IDL {
    let mut ids: MDB_IDL = malloc(
        ((num + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
    ) as MDB_IDL;
    if !ids.is_null() {
        let fresh0 = ids;
        ids = ids.offset(1);
        *fresh0 = num as MDB_ID;
        *ids = 0 as libc::c_int as MDB_ID;
    }
    return ids;
}
pub unsafe extern "C" fn mdb_midl_free(mut ids: MDB_IDL) {
    if !ids.is_null() {
        free(ids.offset(-(1 as libc::c_int as isize)) as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn mdb_midl_shrink(mut idp: *mut MDB_IDL) {
    let mut ids: MDB_IDL = *idp;
    ids = ids.offset(-1);
    if *ids
        > (((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_ulong
        && {
            ids = realloc(
                ids as *mut libc::c_void,
                ((((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                    - 1 as libc::c_int + 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
            ) as MDB_IDL;
            !ids.is_null()
        }
    {
        let fresh1 = ids;
        ids = ids.offset(1);
        *fresh1 = (((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
            - 1 as libc::c_int) as MDB_ID;
        *idp = ids;
    }
}
unsafe extern "C" fn mdb_midl_grow(
    mut idp: *mut MDB_IDL,
    mut num: libc::c_int,
) -> libc::c_int {
    let mut idn: MDB_IDL = (*idp).offset(-(1 as libc::c_int as isize));
    idn = realloc(
        idn as *mut libc::c_void,
        (*idn)
            .wrapping_add(num as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
    ) as MDB_IDL;
    if idn.is_null() {
        return 12 as libc::c_int;
    }
    let fresh2 = idn;
    idn = idn.offset(1);
    *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(num as libc::c_ulong) as MDB_ID
        as MDB_ID;
    *idp = idn;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_need(
    mut idp: *mut MDB_IDL,
    mut num: libc::c_uint,
) -> libc::c_int {
    let mut ids: MDB_IDL = *idp;
    num = (num as libc::c_ulong).wrapping_add(*ids.offset(0 as libc::c_int as isize))
        as libc::c_uint as libc::c_uint;
    if num as libc::c_ulong > *ids.offset(-(1 as libc::c_int) as isize) {
        num = num
            .wrapping_add(num.wrapping_div(4 as libc::c_int as libc::c_uint))
            .wrapping_add((256 as libc::c_int + 2 as libc::c_int) as libc::c_uint)
            & -(256 as libc::c_int) as libc::c_uint;
        ids = realloc(
            ids.offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            (num as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
        ) as MDB_IDL;
        if ids.is_null() {
            return 12 as libc::c_int;
        }
        let fresh3 = ids;
        ids = ids.offset(1);
        *fresh3 = num.wrapping_sub(2 as libc::c_int as libc::c_uint) as MDB_ID;
        *idp = ids;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_append(
    mut idp: *mut MDB_IDL,
    mut id: MDB_ID,
) -> libc::c_int {
    let mut ids: MDB_IDL = *idp;
    if *ids.offset(0 as libc::c_int as isize)
        >= *ids.offset(-(1 as libc::c_int) as isize)
    {
        if mdb_midl_grow(
            idp,
            ((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                - 1 as libc::c_int,
        ) != 0
        {
            return 12 as libc::c_int;
        }
        ids = *idp;
    }
    let ref mut fresh4 = *ids.offset(0 as libc::c_int as isize);
    *fresh4 = (*fresh4).wrapping_add(1);
    *fresh4;
    *ids.offset(*ids.offset(0 as libc::c_int as isize) as isize) = id;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_append_list(
    mut idp: *mut MDB_IDL,
    mut app: MDB_IDL,
) -> libc::c_int {
    let mut ids: MDB_IDL = *idp;
    if (*ids.offset(0 as libc::c_int as isize))
        .wrapping_add(*app.offset(0 as libc::c_int as isize))
        >= *ids.offset(-(1 as libc::c_int) as isize)
    {
        if mdb_midl_grow(idp, *app.offset(0 as libc::c_int as isize) as libc::c_int) != 0
        {
            return 12 as libc::c_int;
        }
        ids = *idp;
    }
    memcpy(
        &mut *ids
            .offset(
                (*ids.offset(0 as libc::c_int as isize))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut MDB_ID as *mut libc::c_void,
        &mut *app.offset(1 as libc::c_int as isize) as *mut MDB_ID
            as *const libc::c_void,
        (*app.offset(0 as libc::c_int as isize))
            .wrapping_mul(::std::mem::size_of::<MDB_ID>() as libc::c_ulong),
    );
    let ref mut fresh5 = *ids.offset(0 as libc::c_int as isize);
    *fresh5 = (*fresh5 as libc::c_ulong)
        .wrapping_add(*app.offset(0 as libc::c_int as isize)) as MDB_ID as MDB_ID;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_append_range(
    mut idp: *mut MDB_IDL,
    mut id: MDB_ID,
    mut n: libc::c_uint,
) -> libc::c_int {
    let mut ids: *mut MDB_ID = *idp;
    let mut len: MDB_ID = *ids.offset(0 as libc::c_int as isize);
    if len.wrapping_add(n as libc::c_ulong) > *ids.offset(-(1 as libc::c_int) as isize) {
        if mdb_midl_grow(
            idp,
            (n
                | (((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
                    - 1 as libc::c_int) as libc::c_uint) as libc::c_int,
        ) != 0
        {
            return 12 as libc::c_int;
        }
        ids = *idp;
    }
    *ids.offset(0 as libc::c_int as isize) = len.wrapping_add(n as libc::c_ulong);
    ids = ids.offset(len as isize);
    while n != 0 {
        let fresh6 = id;
        id = id.wrapping_add(1);
        let fresh7 = n;
        n = n.wrapping_sub(1);
        *ids.offset(fresh7 as isize) = fresh6;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_midl_xmerge(mut idl: MDB_IDL, mut merge: MDB_IDL) {
    let mut old_id: MDB_ID = 0;
    let mut merge_id: MDB_ID = 0;
    let mut i: MDB_ID = *merge.offset(0 as libc::c_int as isize);
    let mut j: MDB_ID = *idl.offset(0 as libc::c_int as isize);
    let mut k: MDB_ID = i.wrapping_add(j);
    let mut total: MDB_ID = k;
    *idl.offset(0 as libc::c_int as isize) = -(1 as libc::c_int) as MDB_ID;
    old_id = *idl.offset(j as isize);
    while i != 0 {
        let fresh8 = i;
        i = i.wrapping_sub(1);
        merge_id = *merge.offset(fresh8 as isize);
        while old_id < merge_id {
            let fresh9 = k;
            k = k.wrapping_sub(1);
            *idl.offset(fresh9 as isize) = old_id;
            j = j.wrapping_sub(1);
            old_id = *idl.offset(j as isize);
        }
        let fresh10 = k;
        k = k.wrapping_sub(1);
        *idl.offset(fresh10 as isize) = merge_id;
    }
    *idl.offset(0 as libc::c_int as isize) = total;
}
pub unsafe extern "C" fn mdb_midl_sort(mut ids: MDB_IDL) {
    let mut istack: [libc::c_int; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ir: libc::c_int = 0;
    let mut jstack: libc::c_int = 0;
    let mut a: MDB_ID = 0;
    let mut itmp: MDB_ID = 0;
    ir = *ids.offset(0 as libc::c_int as isize) as libc::c_int;
    l = 1 as libc::c_int;
    jstack = 0 as libc::c_int;
    loop {
        if ir - l < 8 as libc::c_int {
            j = l + 1 as libc::c_int;
            while j <= ir {
                a = *ids.offset(j as isize);
                i = j - 1 as libc::c_int;
                while i >= 1 as libc::c_int {
                    if *ids.offset(i as isize) >= a {
                        break;
                    }
                    *ids
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ) = *ids.offset(i as isize);
                    i -= 1;
                    i;
                }
                *ids.offset((i + 1 as libc::c_int) as isize) = a;
                j += 1;
                j;
            }
            if jstack == 0 as libc::c_int {
                break;
            }
            let fresh11 = jstack;
            jstack = jstack - 1;
            ir = istack[fresh11 as usize];
            let fresh12 = jstack;
            jstack = jstack - 1;
            l = istack[fresh12 as usize];
        } else {
            k = l + ir >> 1 as libc::c_int;
            itmp = *ids.offset(k as isize);
            *ids.offset(k as isize) = *ids.offset((l + 1 as libc::c_int) as isize);
            *ids.offset((l + 1 as libc::c_int) as isize) = itmp;
            if *ids.offset(l as isize) < *ids.offset(ir as isize) {
                itmp = *ids.offset(l as isize);
                *ids.offset(l as isize) = *ids.offset(ir as isize);
                *ids.offset(ir as isize) = itmp;
            }
            if *ids.offset((l + 1 as libc::c_int) as isize) < *ids.offset(ir as isize) {
                itmp = *ids.offset((l + 1 as libc::c_int) as isize);
                *ids.offset((l + 1 as libc::c_int) as isize) = *ids.offset(ir as isize);
                *ids.offset(ir as isize) = itmp;
            }
            if *ids.offset(l as isize) < *ids.offset((l + 1 as libc::c_int) as isize) {
                itmp = *ids.offset(l as isize);
                *ids.offset(l as isize) = *ids.offset((l + 1 as libc::c_int) as isize);
                *ids.offset((l + 1 as libc::c_int) as isize) = itmp;
            }
            i = l + 1 as libc::c_int;
            j = ir;
            a = *ids.offset((l + 1 as libc::c_int) as isize);
            loop {
                loop {
                    i += 1;
                    i;
                    if !(*ids.offset(i as isize) > a) {
                        break;
                    }
                }
                loop {
                    j -= 1;
                    j;
                    if !(*ids.offset(j as isize) < a) {
                        break;
                    }
                }
                if j < i {
                    break;
                }
                itmp = *ids.offset(i as isize);
                *ids.offset(i as isize) = *ids.offset(j as isize);
                *ids.offset(j as isize) = itmp;
            }
            *ids.offset((l + 1 as libc::c_int) as isize) = *ids.offset(j as isize);
            *ids.offset(j as isize) = a;
            jstack += 2 as libc::c_int;
            if ir - i + 1 as libc::c_int >= j - l {
                istack[jstack as usize] = ir;
                istack[(jstack - 1 as libc::c_int) as usize] = i;
                ir = j - 1 as libc::c_int;
            } else {
                istack[jstack as usize] = j - 1 as libc::c_int;
                istack[(jstack - 1 as libc::c_int) as usize] = l;
                l = i;
            }
        }
    };
}
pub unsafe extern "C" fn mdb_mid2l_search(
    mut ids: MDB_ID2L,
    mut id: MDB_ID,
) -> libc::c_uint {
    let mut base: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cursor: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_uint = (*ids.offset(0 as libc::c_int as isize)).mid
        as libc::c_uint;
    while (0 as libc::c_int as libc::c_uint) < n {
        let mut pivot: libc::c_uint = n >> 1 as libc::c_int;
        cursor = base.wrapping_add(pivot).wrapping_add(1 as libc::c_int as libc::c_uint);
        val = if id < (*ids.offset(cursor as isize)).mid {
            -(1 as libc::c_int)
        } else {
            (id > (*ids.offset(cursor as isize)).mid) as libc::c_int
        };
        if val < 0 as libc::c_int {
            n = pivot;
        } else if val > 0 as libc::c_int {
            base = cursor;
            n = n.wrapping_sub(pivot.wrapping_add(1 as libc::c_int as libc::c_uint));
        } else {
            return cursor
        }
    }
    if val > 0 as libc::c_int {
        cursor = cursor.wrapping_add(1);
        cursor;
    }
    return cursor;
}
pub unsafe extern "C" fn mdb_mid2l_insert(
    mut ids: MDB_ID2L,
    mut id: *mut MDB_ID2,
) -> libc::c_int {
    let mut x: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    x = mdb_mid2l_search(ids, (*id).mid);
    if x < 1 as libc::c_int as libc::c_uint {
        return -(2 as libc::c_int);
    }
    if x as libc::c_ulong <= (*ids.offset(0 as libc::c_int as isize)).mid
        && (*ids.offset(x as isize)).mid == (*id).mid
    {
        return -(1 as libc::c_int);
    }
    if (*ids.offset(0 as libc::c_int as isize)).mid
        >= (((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_ulong
    {
        return -(2 as libc::c_int)
    } else {
        let ref mut fresh13 = (*ids.offset(0 as libc::c_int as isize)).mid;
        *fresh13 = (*fresh13).wrapping_add(1);
        *fresh13;
        i = (*ids.offset(0 as libc::c_int as isize)).mid as libc::c_uint;
        while i > x {
            *ids
                .offset(
                    i as isize,
                ) = *ids
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
            i = i.wrapping_sub(1);
            i;
        }
        *ids.offset(x as isize) = *id;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mdb_mid2l_append(
    mut ids: MDB_ID2L,
    mut id: *mut MDB_ID2,
) -> libc::c_int {
    if (*ids.offset(0 as libc::c_int as isize)).mid
        >= (((1 as libc::c_int) << 16 as libc::c_int + 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_ulong
    {
        return -(2 as libc::c_int);
    }
    let ref mut fresh14 = (*ids.offset(0 as libc::c_int as isize)).mid;
    *fresh14 = (*fresh14).wrapping_add(1);
    *fresh14;
    *ids.offset((*ids.offset(0 as libc::c_int as isize)).mid as isize) = *id;
    return 0 as libc::c_int;
}
