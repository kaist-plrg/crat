use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashtable {
    pub f1: T_HashFunc,
    pub f2: T_HashFunc,
    pub compar: T_ComparFunc,
    pub size: size_t,
    pub fill: size_t,
    pub inuse: size_t,
    pub max: size_t,
    pub entries: *mut *mut libc::c_void,
}
pub type T_ComparFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
pub type T_HashFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> uint32_t>;
pub type T_HashTable = hashtable;
static mut sizes: [size_t; 30] = [
    5 as libc::c_int as size_t,
    11 as libc::c_int as size_t,
    23 as libc::c_int as size_t,
    47 as libc::c_int as size_t,
    97 as libc::c_int as size_t,
    197 as libc::c_int as size_t,
    397 as libc::c_int as size_t,
    797 as libc::c_int as size_t,
    1597 as libc::c_int as size_t,
    3203 as libc::c_int as size_t,
    6421 as libc::c_int as size_t,
    12853 as libc::c_int as size_t,
    25717 as libc::c_int as size_t,
    51437 as libc::c_int as size_t,
    102877 as libc::c_int as size_t,
    205759 as libc::c_int as size_t,
    411527 as libc::c_int as size_t,
    823117 as libc::c_int as size_t,
    1646237 as libc::c_int as size_t,
    3292489 as libc::c_int as size_t,
    6584983 as libc::c_int as size_t,
    13169977 as libc::c_int as size_t,
    26339969 as libc::c_int as size_t,
    52679969 as libc::c_int as size_t,
    105359939 as libc::c_int as size_t,
    210719881 as libc::c_int as size_t,
    421439783 as libc::c_int as size_t,
    842879579 as libc::c_int as size_t,
    1685759167 as libc::c_int as size_t,
    0 as libc::c_int as size_t,
];
static mut deleted: libc::c_int = 0 as libc::c_int;
static mut unallocated: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn alloc_ht(mut H: *mut T_HashTable, mut size: size_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ii: size_t = 0;
    i = 0 as libc::c_int;
    while sizes[i as usize] != 0 {
        if sizes[i as usize] > size.wrapping_mul(4 as libc::c_int as libc::c_ulong) {
            break;
        }
        i += 1;
        i;
    }
    if sizes[i as usize] == 0 {
        i = 0 as libc::c_int;
        while sizes[i as usize] != 0 {
            if sizes[i as usize] > size.wrapping_mul(2 as libc::c_int as libc::c_ulong) {
                break;
            }
            i += 1;
            i;
        }
    }
    if sizes[i as usize] == 0 {
        i = 0 as libc::c_int;
        while sizes[i as usize] != 0 {
            if sizes[i as usize] > size {
                break;
            }
            i += 1;
            i;
        }
    }
    if sizes[i as usize] == 0 {
        return -(1 as libc::c_int);
    }
    size = sizes[i as usize];
    if size < (*H).size {
        size = (*H).size;
    }
    (*H)
        .max = size
        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        .wrapping_div(5 as libc::c_int as libc::c_ulong)
        .wrapping_sub(2 as libc::c_int as libc::c_ulong);
    (*H).size = size;
    (*H).fill = 0 as libc::c_int as size_t;
    (*H).inuse = 0 as libc::c_int as size_t;
    (*H)
        .entries = calloc(
        size,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    ) as *mut *mut libc::c_void;
    if ((*H).entries).is_null() {
        return -(1 as libc::c_int);
    }
    ii = 0 as libc::c_int as size_t;
    while ii < size {
        let ref mut fresh0 = *((*H).entries).offset(ii as isize);
        *fresh0 = &mut unallocated as *mut libc::c_int as *mut libc::c_void;
        ii = ii.wrapping_add(1);
        ii;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn make_ht(
    mut f1: T_HashFunc,
    mut f2: T_HashFunc,
    mut c: T_ComparFunc,
    mut size: size_t,
    mut H: *mut *mut T_HashTable,
) -> libc::c_int {
    *H = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<T_HashTable>() as libc::c_ulong,
    ) as *mut T_HashTable;
    if (*H).is_null() {
        return -(1 as libc::c_int);
    }
    (**H).f1 = f1;
    (**H).f2 = f2;
    (**H).compar = c;
    (**H).size = 0 as libc::c_int as size_t;
    if alloc_ht(*H, size) != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn free_ht(
    mut H: *mut T_HashTable,
    mut entry_free: T_HashFunc,
) -> libc::c_int {
    let mut i: size_t = 0;
    if entry_free.is_some() {
        i = 0 as libc::c_int as size_t;
        while i < (*H).size {
            if *((*H).entries).offset(i as isize)
                != &mut unallocated as *mut libc::c_int as *mut libc::c_void
                && *((*H).entries).offset(i as isize)
                    != &mut deleted as *mut libc::c_int as *mut libc::c_void
            {
                entry_free.unwrap()(*((*H).entries).offset(i as isize));
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    free((*H).entries as *mut libc::c_char as *mut libc::c_void);
    free(H as *mut libc::c_char as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn _hash_add(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut hint: *mut size_t,
) -> libc::c_int {
    let mut f2: size_t = 0;
    let mut pos: size_t = 0;
    let mut ctr: libc::c_int = 0;
    pos = (((*H).f1).unwrap()(E) as libc::c_ulong).wrapping_rem((*H).size);
    f2 = -(1 as libc::c_int) as size_t;
    ctr = 0 as libc::c_int;
    while *((*H).entries).offset(pos as isize)
        != &mut unallocated as *mut libc::c_int as *mut libc::c_void
        && *((*H).entries).offset(pos as isize)
            != &mut deleted as *mut libc::c_int as *mut libc::c_void
    {
        if f2 == -(1 as libc::c_int) as size_t {
            f2 = (((*H).f2).unwrap()(E) as libc::c_ulong)
                .wrapping_rem(
                    ((*H).size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        pos = pos
            .wrapping_add(f2)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_rem((*H).size);
        ctr += 1;
        ctr;
    }
    if *((*H).entries).offset(pos as isize)
        == &mut unallocated as *mut libc::c_int as *mut libc::c_void
    {
        (*H).fill = ((*H).fill).wrapping_add(1);
        (*H).fill;
    }
    (*H).inuse = ((*H).inuse).wrapping_add(1);
    (*H).inuse;
    let ref mut fresh1 = *((*H).entries).offset(pos as isize);
    *fresh1 = E;
    if !hint.is_null() {
        *hint = pos;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn rehash(mut H: *mut T_HashTable) -> libc::c_int {
    let mut size: size_t = 0;
    let mut i: size_t = 0;
    let mut oldentries: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    size = (*H).size;
    oldentries = (*H).entries;
    if alloc_ht(
        H,
        ((*H).inuse)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_add((*H).fill)
            .wrapping_div(5 as libc::c_int as libc::c_ulong),
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        if *oldentries.offset(i as isize)
            != &mut unallocated as *mut libc::c_int as *mut libc::c_void
            && *oldentries.offset(i as isize)
                != &mut deleted as *mut libc::c_int as *mut libc::c_void
        {
            _hash_add(H, *oldentries.offset(i as isize), 0 as *mut size_t);
        }
        i = i.wrapping_add(1);
        i;
    }
    free(oldentries as *mut libc::c_char as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hash_add(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut hint: *mut size_t,
) -> libc::c_int {
    if (*H).fill >= (*H).max {
        rehash(H);
    }
    if (*H).fill == (*H).size {
        return -(1 as libc::c_int);
    }
    return _hash_add(H, E, hint);
}
unsafe extern "C" fn _hash_lookup(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut E2: *mut *mut libc::c_void,
    mut hint: *mut size_t,
    mut isIdentity: libc::c_int,
) -> libc::c_int {
    let mut f2: size_t = 0;
    let mut pos: size_t = 0;
    let mut upos: size_t = 0;
    let mut ttl: size_t = 0;
    pos = (((*H).f1).unwrap()(E) as libc::c_ulong).wrapping_rem((*H).size);
    ttl = (*H).size;
    f2 = -(1 as libc::c_int) as size_t;
    upos = -(1 as libc::c_int) as size_t;
    while ttl != 0
        && *((*H).entries).offset(pos as isize)
            != &mut unallocated as *mut libc::c_int as *mut libc::c_void
        && (*((*H).entries).offset(pos as isize)
            == &mut deleted as *mut libc::c_int as *mut libc::c_void
            || (isIdentity != 0
                || ((*H).compar).unwrap()(*((*H).entries).offset(pos as isize), E)
                    != 0 as libc::c_int)
                && (isIdentity == 0 || *((*H).entries).offset(pos as isize) != E))
    {
        if f2 == -(1 as libc::c_int) as size_t {
            f2 = (((*H).f2).unwrap()(E) as libc::c_ulong)
                .wrapping_rem(
                    ((*H).size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
        }
        if upos == -(1 as libc::c_int) as size_t
            && *((*H).entries).offset(pos as isize)
                == &mut deleted as *mut libc::c_int as *mut libc::c_void
        {
            upos = pos;
        }
        pos = pos
            .wrapping_add(f2)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_rem((*H).size);
        ttl = ttl.wrapping_sub(1);
        ttl;
    }
    if *((*H).entries).offset(pos as isize)
        == &mut unallocated as *mut libc::c_int as *mut libc::c_void || ttl == 0
    {
        return -(1 as libc::c_int);
    }
    if upos != -(1 as libc::c_int) as size_t {
        let ref mut fresh2 = *((*H).entries).offset(upos as isize);
        *fresh2 = *((*H).entries).offset(pos as isize);
        let ref mut fresh3 = *((*H).entries).offset(pos as isize);
        *fresh3 = &mut deleted as *mut libc::c_int as *mut libc::c_void;
        pos = upos;
    }
    if !hint.is_null() {
        *hint = pos;
    }
    *E2 = *((*H).entries).offset(pos as isize);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn hash_lookup(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut E2: *mut *mut libc::c_void,
    mut hint: *mut size_t,
) -> libc::c_int {
    return _hash_lookup(H, E, E2, hint, 0 as libc::c_int);
}
pub unsafe extern "C" fn hash_remove(
    mut H: *mut T_HashTable,
    mut E: *mut libc::c_void,
    mut hint: size_t,
) -> libc::c_int {
    let mut E2: *mut libc::c_void = 0 as *mut libc::c_void;
    if hint < (*H).size && *((*H).entries).offset(hint as isize) == E {
        (*H).inuse = ((*H).inuse).wrapping_sub(1);
        (*H).inuse;
        let ref mut fresh4 = *((*H).entries).offset(hint as isize);
        *fresh4 = &mut deleted as *mut libc::c_int as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    if _hash_lookup(H, E, &mut E2, &mut hint, 1 as libc::c_int) != 0 {
        fprintf(
            stderr,
            b"Removing non-existent entry\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*H).inuse = ((*H).inuse).wrapping_sub(1);
    (*H).inuse;
    let ref mut fresh5 = *((*H).entries).offset(hint as isize);
    *fresh5 = &mut deleted as *mut libc::c_int as *mut libc::c_void;
    return 0 as libc::c_int;
}
