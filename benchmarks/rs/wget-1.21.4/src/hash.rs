use ::libc;
extern "C" {
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rpl_free(_: *mut libc::c_void);
    fn abort() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub hash_function: hashfun_t,
    pub test_function: testfun_t,
    pub cells: *mut cell,
    pub size: libc::c_int,
    pub count: libc::c_int,
    pub resize_threshold: libc::c_int,
    pub prime_offset: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cell {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
}
pub type testfun_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hashfun_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_iterator {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub pos: *mut libc::c_void,
    pub end: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
unsafe extern "C" fn prime_size(
    mut size: libc::c_int,
    mut prime_offset: *mut libc::c_int,
) -> libc::c_int {
    static mut primes: [libc::c_int; 71] = [
        13 as libc::c_int,
        19 as libc::c_int,
        29 as libc::c_int,
        41 as libc::c_int,
        59 as libc::c_int,
        79 as libc::c_int,
        107 as libc::c_int,
        149 as libc::c_int,
        197 as libc::c_int,
        263 as libc::c_int,
        347 as libc::c_int,
        457 as libc::c_int,
        599 as libc::c_int,
        787 as libc::c_int,
        1031 as libc::c_int,
        1361 as libc::c_int,
        1777 as libc::c_int,
        2333 as libc::c_int,
        3037 as libc::c_int,
        3967 as libc::c_int,
        5167 as libc::c_int,
        6719 as libc::c_int,
        8737 as libc::c_int,
        11369 as libc::c_int,
        14783 as libc::c_int,
        19219 as libc::c_int,
        24989 as libc::c_int,
        32491 as libc::c_int,
        42257 as libc::c_int,
        54941 as libc::c_int,
        71429 as libc::c_int,
        92861 as libc::c_int,
        120721 as libc::c_int,
        156941 as libc::c_int,
        204047 as libc::c_int,
        265271 as libc::c_int,
        344857 as libc::c_int,
        448321 as libc::c_int,
        582821 as libc::c_int,
        757693 as libc::c_int,
        985003 as libc::c_int,
        1280519 as libc::c_int,
        1664681 as libc::c_int,
        2164111 as libc::c_int,
        2813353 as libc::c_int,
        3657361 as libc::c_int,
        4754591 as libc::c_int,
        6180989 as libc::c_int,
        8035301 as libc::c_int,
        10445899 as libc::c_int,
        13579681 as libc::c_int,
        17653589 as libc::c_int,
        22949669 as libc::c_int,
        29834603 as libc::c_int,
        38784989 as libc::c_int,
        50420551 as libc::c_int,
        65546729 as libc::c_int,
        85210757 as libc::c_int,
        110774011 as libc::c_int,
        144006217 as libc::c_int,
        187208107 as libc::c_int,
        243370577 as libc::c_int,
        316381771 as libc::c_int,
        411296309 as libc::c_int,
        534685237 as libc::c_int,
        695090819 as libc::c_int,
        903618083 as libc::c_int,
        1174703521 as libc::c_int,
        1527114613 as libc::c_int,
        1837299131 as libc::c_int,
        2147483647 as libc::c_int,
    ];
    let mut i: size_t = 0;
    i = *prime_offset as size_t;
    while i
        < (::std::mem::size_of::<[libc::c_int; 71]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        if primes[i as usize] >= size {
            *prime_offset = i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            return primes[i as usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    abort();
}
pub unsafe extern "C" fn hash_table_new(
    mut items: libc::c_int,
    mut hash_function: Option::<
        unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
    >,
    mut test_function: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) -> *mut hash_table {
    let mut size: libc::c_int = 0;
    let mut ht: *mut hash_table = xmalloc(
        ::std::mem::size_of::<hash_table>() as libc::c_ulong,
    ) as *mut hash_table;
    (*ht)
        .hash_function = if hash_function.is_some() {
        hash_function
    } else {
        Some(hash_pointer as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong)
    };
    (*ht)
        .test_function = if test_function.is_some() {
        test_function
    } else {
        Some(
            cmp_pointer
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        )
    };
    (*ht).prime_offset = 0 as libc::c_int;
    size = (1 as libc::c_int as libc::c_double + items as libc::c_double / 0.75f64)
        as libc::c_int;
    size = prime_size(size, &mut (*ht).prime_offset);
    (*ht).size = size;
    (*ht).resize_threshold = (size as libc::c_double * 0.75f64) as libc::c_int;
    (*ht)
        .cells = xmalloc(
        ((*ht).size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cell>() as libc::c_ulong),
    ) as *mut cell;
    memset(
        (*ht).cells as *mut libc::c_void,
        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cell>() as libc::c_ulong),
    );
    (*ht).count = 0 as libc::c_int;
    return ht;
}
pub unsafe extern "C" fn hash_table_destroy(mut ht: *mut hash_table) {
    rpl_free((*ht).cells as *mut libc::c_void);
    (*ht).cells = 0 as *mut cell;
    rpl_free(ht as *mut libc::c_void);
    ht = 0 as *mut hash_table;
}
#[inline]
unsafe extern "C" fn find_cell(
    mut ht: *const hash_table,
    mut key: *const libc::c_void,
) -> *mut cell {
    let mut cells: *mut cell = (*ht).cells;
    let mut size: libc::c_int = (*ht).size;
    let mut c: *mut cell = cells
        .offset(
            (((*ht).hash_function).unwrap()(key)).wrapping_rem(size as libc::c_ulong)
                as isize,
        );
    let mut equals: testfun_t = (*ht).test_function;
    while (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
        if equals.unwrap()(key, (*c).key) != 0 {
            break;
        }
        c = if c != cells.offset((size - 1 as libc::c_int) as isize) {
            c.offset(1 as libc::c_int as isize)
        } else {
            cells
        };
    }
    return c;
}
pub unsafe extern "C" fn hash_table_get(
    mut ht: *const hash_table,
    mut key: *const libc::c_void,
) -> *mut libc::c_void {
    let mut c: *mut cell = find_cell(ht, key);
    if (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
        return (*c).value
    } else {
        return 0 as *mut libc::c_void
    };
}
pub unsafe extern "C" fn hash_table_get_pair(
    mut ht: *const hash_table,
    mut lookup_key: *const libc::c_void,
    mut orig_key: *mut libc::c_void,
    mut value: *mut libc::c_void,
) -> libc::c_int {
    let mut c: *mut cell = find_cell(ht, lookup_key);
    if (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
        if !orig_key.is_null() {
            let ref mut fresh0 = *(orig_key as *mut *mut libc::c_void);
            *fresh0 = (*c).key;
        }
        if !value.is_null() {
            let ref mut fresh1 = *(value as *mut *mut libc::c_void);
            *fresh1 = (*c).value;
        }
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn hash_table_contains(
    mut ht: *const hash_table,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut c: *mut cell = find_cell(ht, key);
    return ((*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void)
        as libc::c_int;
}
unsafe extern "C" fn grow_hash_table(mut ht: *mut hash_table) {
    let mut hasher: hashfun_t = (*ht).hash_function;
    let mut old_cells: *mut cell = (*ht).cells;
    let mut old_end: *mut cell = ((*ht).cells).offset((*ht).size as isize);
    let mut c: *mut cell = 0 as *mut cell;
    let mut cells: *mut cell = 0 as *mut cell;
    let mut newsize: libc::c_int = 0;
    newsize = prime_size((*ht).size * 2 as libc::c_int, &mut (*ht).prime_offset);
    (*ht).size = newsize;
    (*ht).resize_threshold = (newsize as libc::c_double * 0.75f64) as libc::c_int;
    cells = xmalloc(
        (newsize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cell>() as libc::c_ulong),
    ) as *mut cell;
    memset(
        cells as *mut libc::c_void,
        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        (newsize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cell>() as libc::c_ulong),
    );
    (*ht).cells = cells;
    c = old_cells;
    while c < old_end {
        if (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
            let mut new_c: *mut cell = 0 as *mut cell;
            new_c = cells
                .offset(
                    (hasher.unwrap()((*c).key)).wrapping_rem(newsize as libc::c_ulong)
                        as isize,
                );
            while (*new_c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
                new_c = if new_c != cells.offset((newsize - 1 as libc::c_int) as isize) {
                    new_c.offset(1 as libc::c_int as isize)
                } else {
                    cells
                };
            }
            *new_c = *c;
        }
        c = c.offset(1);
        c;
    }
    rpl_free(old_cells as *mut libc::c_void);
    old_cells = 0 as *mut cell;
}
pub unsafe extern "C" fn hash_table_put(
    mut ht: *mut hash_table,
    mut key: *const libc::c_void,
    mut value: *const libc::c_void,
) {
    let mut c: *mut cell = find_cell(ht, key);
    if (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
        (*c).key = key as *mut libc::c_void;
        (*c).value = value as *mut libc::c_void;
        return;
    }
    if (*ht).count >= (*ht).resize_threshold {
        grow_hash_table(ht);
        c = find_cell(ht, key);
    }
    (*ht).count += 1;
    (*ht).count;
    (*c).key = key as *mut libc::c_void;
    (*c).value = value as *mut libc::c_void;
}
pub unsafe extern "C" fn hash_table_remove(
    mut ht: *mut hash_table,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut c: *mut cell = find_cell(ht, key);
    if !((*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void) {
        return 0 as libc::c_int
    } else {
        let mut size: libc::c_int = (*ht).size;
        let mut cells: *mut cell = (*ht).cells;
        let mut hasher: hashfun_t = (*ht).hash_function;
        (*c).key = !(0 as libc::c_int as uintptr_t) as *mut libc::c_void;
        (*ht).count -= 1;
        (*ht).count;
        c = if c != cells.offset((size - 1 as libc::c_int) as isize) {
            c.offset(1 as libc::c_int as isize)
        } else {
            cells
        };
        while (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
            let mut current_block_7: u64;
            let mut key2: *const libc::c_void = (*c).key;
            let mut c_new: *mut cell = 0 as *mut cell;
            c_new = cells
                .offset(
                    (hasher.unwrap()(key2)).wrapping_rem(size as libc::c_ulong) as isize,
                );
            loop {
                if !((*c_new).key
                    != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void)
                {
                    current_block_7 = 13183875560443969876;
                    break;
                }
                if key2 == (*c_new).key as *const libc::c_void {
                    current_block_7 = 12209867499936983673;
                    break;
                }
                c_new = if c_new != cells.offset((size - 1 as libc::c_int) as isize) {
                    c_new.offset(1 as libc::c_int as isize)
                } else {
                    cells
                };
            }
            match current_block_7 {
                13183875560443969876 => {
                    *c_new = *c;
                    (*c).key = !(0 as libc::c_int as uintptr_t) as *mut libc::c_void;
                }
                _ => {}
            }
            c = if c != cells.offset((size - 1 as libc::c_int) as isize) {
                c.offset(1 as libc::c_int as isize)
            } else {
                cells
            };
        }
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn hash_table_clear(mut ht: *mut hash_table) {
    memset(
        (*ht).cells as *mut libc::c_void,
        127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        ((*ht).size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cell>() as libc::c_ulong),
    );
    (*ht).count = 0 as libc::c_int;
}
pub unsafe extern "C" fn hash_table_for_each(
    mut ht: *mut hash_table,
    mut fn_0: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut cell = (*ht).cells;
    let mut end: *mut cell = ((*ht).cells).offset((*ht).size as isize);
    while c < end {
        if (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
            let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
            loop {
                key = (*c).key;
                if fn_0.unwrap()(key, (*c).value, arg) != 0 {
                    return;
                }
                if !((*c).key != key
                    && (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void)
                {
                    break;
                }
            }
        }
        c = c.offset(1);
        c;
    }
}
pub unsafe extern "C" fn hash_table_iterate(
    mut ht: *mut hash_table,
    mut iter: *mut hash_table_iterator,
) {
    (*iter).pos = (*ht).cells as *mut libc::c_void;
    (*iter).end = ((*ht).cells).offset((*ht).size as isize) as *mut libc::c_void;
}
pub unsafe extern "C" fn hash_table_iter_next(
    mut iter: *mut hash_table_iterator,
) -> libc::c_int {
    let mut c: *mut cell = (*iter).pos as *mut cell;
    let mut end: *mut cell = (*iter).end as *mut cell;
    while c < end {
        if (*c).key != !(0 as libc::c_int as uintptr_t) as *mut libc::c_void {
            (*iter).key = (*c).key;
            (*iter).value = (*c).value;
            (*iter).pos = c.offset(1 as libc::c_int as isize) as *mut libc::c_void;
            return 1 as libc::c_int;
        }
        c = c.offset(1);
        c;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hash_table_count(mut ht: *const hash_table) -> libc::c_int {
    return (*ht).count;
}
unsafe extern "C" fn hash_string(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut p: *const libc::c_char = key as *const libc::c_char;
    let mut h: libc::c_uint = *p as libc::c_uint;
    if h != 0 {
        p = p.offset(1 as libc::c_int as isize);
        while *p as libc::c_int != '\0' as i32 {
            h = (h << 5 as libc::c_int).wrapping_sub(h).wrapping_add(*p as libc::c_uint);
            p = p.offset(1);
            p;
        }
    }
    return h as libc::c_ulong;
}
unsafe extern "C" fn cmp_string(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    return (strcmp(s1 as *const libc::c_char, s2 as *const libc::c_char) == 0)
        as libc::c_int;
}
pub unsafe extern "C" fn make_string_hash_table(
    mut items: libc::c_int,
) -> *mut hash_table {
    return hash_table_new(
        items,
        Some(hash_string as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong),
        Some(
            cmp_string
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn hash_string_nocase(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut p: *const libc::c_char = key as *const libc::c_char;
    let mut h: libc::c_uint = c_tolower(*p as libc::c_int) as libc::c_uint;
    if h != 0 {
        p = p.offset(1 as libc::c_int as isize);
        while *p as libc::c_int != '\0' as i32 {
            h = (h << 5 as libc::c_int)
                .wrapping_sub(h)
                .wrapping_add(c_tolower(*p as libc::c_int) as libc::c_uint);
            p = p.offset(1);
            p;
        }
    }
    return h as libc::c_ulong;
}
unsafe extern "C" fn string_cmp_nocase(
    mut s1: *const libc::c_void,
    mut s2: *const libc::c_void,
) -> libc::c_int {
    return (strcasecmp(s1 as *const libc::c_char, s2 as *const libc::c_char) == 0)
        as libc::c_int;
}
pub unsafe extern "C" fn make_nocase_string_hash_table(
    mut items: libc::c_int,
) -> *mut hash_table {
    return hash_table_new(
        items,
        Some(
            hash_string_nocase
                as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            string_cmp_nocase
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn hash_pointer(mut ptr: *const libc::c_void) -> libc::c_ulong {
    let mut key: uintptr_t = ptr as uintptr_t;
    key = (key as libc::c_ulong).wrapping_add(key << 12 as libc::c_int) as uintptr_t
        as uintptr_t;
    key ^= key >> 22 as libc::c_int;
    key = (key as libc::c_ulong).wrapping_add(key << 4 as libc::c_int) as uintptr_t
        as uintptr_t;
    key ^= key >> 9 as libc::c_int;
    key = (key as libc::c_ulong).wrapping_add(key << 10 as libc::c_int) as uintptr_t
        as uintptr_t;
    key ^= key >> 2 as libc::c_int;
    key = (key as libc::c_ulong).wrapping_add(key << 7 as libc::c_int) as uintptr_t
        as uintptr_t;
    key ^= key >> 12 as libc::c_int;
    return key;
}
unsafe extern "C" fn cmp_pointer(
    mut ptr1: *const libc::c_void,
    mut ptr2: *const libc::c_void,
) -> libc::c_int {
    return (ptr1 == ptr2) as libc::c_int;
}
