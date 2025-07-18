use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliHash {
    pub head: *mut libc::c_int,
    pub prev: *mut libc::c_ushort,
    pub hashval: *mut libc::c_int,
    pub val: libc::c_int,
    pub head2: *mut libc::c_int,
    pub prev2: *mut libc::c_ushort,
    pub hashval2: *mut libc::c_int,
    pub val2: libc::c_int,
    pub same: *mut libc::c_ushort,
}
pub unsafe extern "C" fn ZopfliAllocHash(
    mut window_size: size_t,
    mut h: *mut ZopfliHash,
) {
    (*h)
        .head = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    (*h)
        .prev = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul(window_size),
    ) as *mut libc::c_ushort;
    (*h)
        .hashval = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(window_size),
    ) as *mut libc::c_int;
    (*h)
        .same = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul(window_size),
    ) as *mut libc::c_ushort;
    (*h)
        .head2 = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(65536 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    (*h)
        .prev2 = malloc(
        (::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
            .wrapping_mul(window_size),
    ) as *mut libc::c_ushort;
    (*h)
        .hashval2 = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(window_size),
    ) as *mut libc::c_int;
}
pub unsafe extern "C" fn ZopfliResetHash(
    mut window_size: size_t,
    mut h: *mut ZopfliHash,
) {
    let mut i: size_t = 0;
    (*h).val = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 65536 as libc::c_int as libc::c_ulong {
        *((*h).head).offset(i as isize) = -(1 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < window_size {
        *((*h).prev).offset(i as isize) = i as libc::c_ushort;
        *((*h).hashval).offset(i as isize) = -(1 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < window_size {
        *((*h).same).offset(i as isize) = 0 as libc::c_int as libc::c_ushort;
        i = i.wrapping_add(1);
        i;
    }
    (*h).val2 = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 65536 as libc::c_int as libc::c_ulong {
        *((*h).head2).offset(i as isize) = -(1 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < window_size {
        *((*h).prev2).offset(i as isize) = i as libc::c_ushort;
        *((*h).hashval2).offset(i as isize) = -(1 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn ZopfliCleanHash(mut h: *mut ZopfliHash) {
    free((*h).head as *mut libc::c_void);
    free((*h).prev as *mut libc::c_void);
    free((*h).hashval as *mut libc::c_void);
    free((*h).head2 as *mut libc::c_void);
    free((*h).prev2 as *mut libc::c_void);
    free((*h).hashval2 as *mut libc::c_void);
    free((*h).same as *mut libc::c_void);
}
unsafe extern "C" fn UpdateHashValue(mut h: *mut ZopfliHash, mut c: libc::c_uchar) {
    (*h).val = ((*h).val << 5 as libc::c_int ^ c as libc::c_int) & 32767 as libc::c_int;
}
pub unsafe extern "C" fn ZopfliUpdateHash(
    mut array: *const libc::c_uchar,
    mut pos: size_t,
    mut end: size_t,
    mut h: *mut ZopfliHash,
) {
    let mut hpos: libc::c_ushort = (pos
        & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong) as libc::c_ushort;
    let mut amount: size_t = 0 as libc::c_int as size_t;
    UpdateHashValue(
        h,
        (if pos.wrapping_add(3 as libc::c_int as libc::c_ulong) <= end {
            *array
                .offset(
                    pos
                        .wrapping_add(3 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uchar,
    );
    *((*h).hashval).offset(hpos as isize) = (*h).val;
    if *((*h).head).offset((*h).val as isize) != -(1 as libc::c_int)
        && *((*h).hashval).offset(*((*h).head).offset((*h).val as isize) as isize)
            == (*h).val
    {
        *((*h).prev)
            .offset(
                hpos as isize,
            ) = *((*h).head).offset((*h).val as isize) as libc::c_ushort;
    } else {
        *((*h).prev).offset(hpos as isize) = hpos;
    }
    *((*h).head).offset((*h).val as isize) = hpos as libc::c_int;
    if *((*h).same)
        .offset(
            (pos.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong) as isize,
        ) as libc::c_int > 1 as libc::c_int
    {
        amount = (*((*h).same)
            .offset(
                (pos.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    & (32768 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                    as isize,
            ) as libc::c_int - 1 as libc::c_int) as size_t;
    }
    while pos.wrapping_add(amount).wrapping_add(1 as libc::c_int as libc::c_ulong) < end
        && *array.offset(pos as isize) as libc::c_int
            == *array
                .offset(
                    pos
                        .wrapping_add(amount)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int
        && amount < -(1 as libc::c_int) as libc::c_ushort as libc::c_ulong
    {
        amount = amount.wrapping_add(1);
        amount;
    }
    *((*h).same).offset(hpos as isize) = amount as libc::c_ushort;
    (*h)
        .val2 = *((*h).same).offset(hpos as isize) as libc::c_int - 3 as libc::c_int
        & 255 as libc::c_int ^ (*h).val;
    *((*h).hashval2).offset(hpos as isize) = (*h).val2;
    if *((*h).head2).offset((*h).val2 as isize) != -(1 as libc::c_int)
        && *((*h).hashval2).offset(*((*h).head2).offset((*h).val2 as isize) as isize)
            == (*h).val2
    {
        *((*h).prev2)
            .offset(
                hpos as isize,
            ) = *((*h).head2).offset((*h).val2 as isize) as libc::c_ushort;
    } else {
        *((*h).prev2).offset(hpos as isize) = hpos;
    }
    *((*h).head2).offset((*h).val2 as isize) = hpos as libc::c_int;
}
pub unsafe extern "C" fn ZopfliWarmupHash(
    mut array: *const libc::c_uchar,
    mut pos: size_t,
    mut end: size_t,
    mut h: *mut ZopfliHash,
) {
    UpdateHashValue(
        h,
        *array.offset(pos.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize),
    );
    if pos.wrapping_add(1 as libc::c_int as libc::c_ulong) < end {
        UpdateHashValue(
            h,
            *array.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
        );
    }
}
