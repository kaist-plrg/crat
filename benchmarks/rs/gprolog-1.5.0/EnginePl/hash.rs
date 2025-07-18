use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn Pl_Calloc_Check(
        nb: libc::c_uint,
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashScan {
    pub endt: *mut libc::c_char,
    pub cur_t: *mut libc::c_char,
    pub cur_p: *mut libc::c_char,
}
pub type HashNode = *mut hash_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_node {
    pub next: HashNode,
    pub key: PlLong,
}
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub unsafe extern "C" fn Pl_Hash_Alloc_Table(
    mut tbl_size: libc::c_int,
    mut elem_size: libc::c_int,
) -> *mut libc::c_char {
    let mut tbl: *mut libc::c_char = 0 as *mut libc::c_char;
    tbl = Pl_Calloc_Check(
        (3 as libc::c_int + tbl_size) as libc::c_uint,
        ::std::mem::size_of::<HashNode>() as libc::c_ulong as libc::c_uint,
        b"hash.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        130 as libc::c_int,
    );
    if tbl_size < 1 as libc::c_int {
        tbl_size = 1 as libc::c_int;
    }
    *(tbl as *mut libc::c_int).offset(0 as libc::c_int as isize) = tbl_size;
    *(tbl as *mut libc::c_int).offset(1 as libc::c_int as isize) = elem_size;
    *(tbl as *mut libc::c_int).offset(2 as libc::c_int as isize) = 0 as libc::c_int;
    return tbl;
}
pub unsafe extern "C" fn Pl_Hash_Realloc_Table(
    mut tbl: *mut libc::c_char,
    mut new_tbl_size: libc::c_int,
) -> *mut libc::c_char {
    let mut tbl_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut elem_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(1 as libc::c_int as isize);
    let mut t: *mut HashNode = (tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    let mut endt: *mut HashNode = t.offset(tbl_size as isize);
    let mut p: HashNode = 0 as *mut hash_node;
    let mut p1: HashNode = 0 as *mut hash_node;
    let mut prev: *mut HashNode = 0 as *mut HashNode;
    let mut new_tbl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_t: *mut HashNode = 0 as *mut HashNode;
    new_tbl = Pl_Hash_Alloc_Table(new_tbl_size, elem_size);
    if new_tbl.is_null() {
        return 0 as *mut libc::c_char;
    }
    *(new_tbl as *mut libc::c_int)
        .offset(
            2 as libc::c_int as isize,
        ) = *(tbl as *mut libc::c_int).offset(2 as libc::c_int as isize);
    new_t = (new_tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    loop {
        p = *t;
        while !p.is_null() {
            prev = Hash_Locate(new_t, new_tbl_size, (*p).key);
            p1 = p;
            p = (*p).next;
            *prev = p1;
            (*p1).next = 0 as HashNode;
        }
        t = t.offset(1);
        if !(t < endt) {
            break;
        }
    }
    free(tbl as *mut libc::c_void);
    return new_tbl;
}
pub unsafe extern "C" fn Pl_Hash_Free_Table(mut tbl: *mut libc::c_char) {
    let mut tbl_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut t: *mut HashNode = (tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    let mut endt: *mut HashNode = t.offset(tbl_size as isize);
    let mut p: HashNode = 0 as *mut hash_node;
    let mut p1: HashNode = 0 as *mut hash_node;
    loop {
        p = *t;
        while !p.is_null() {
            p1 = p;
            p = (*p).next;
            free(p1 as *mut libc::c_void);
        }
        t = t.offset(1);
        if !(t < endt) {
            break;
        }
    }
    free(tbl as *mut libc::c_void);
}
pub unsafe extern "C" fn Pl_Hash_Delete_All(mut tbl: *mut libc::c_char) {
    let mut tbl_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut t: *mut HashNode = (tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    let mut endt: *mut HashNode = t.offset(tbl_size as isize);
    let mut p: HashNode = 0 as *mut hash_node;
    let mut p1: HashNode = 0 as *mut hash_node;
    loop {
        p = *t;
        while !p.is_null() {
            p1 = p;
            p = (*p).next;
            free(p1 as *mut libc::c_void);
        }
        *t = 0 as HashNode;
        t = t.offset(1);
        if !(t < endt) {
            break;
        }
    }
    *(tbl as *mut libc::c_int).offset(2 as libc::c_int as isize) = 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Hash_Insert(
    mut tbl: *mut libc::c_char,
    mut elem: *mut libc::c_char,
    mut replace: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut tbl_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut elem_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(1 as libc::c_int as isize);
    let mut t: *mut HashNode = (tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    let mut key: PlLong = *(elem as *mut PlLong);
    let mut prev: *mut HashNode = 0 as *mut HashNode;
    let mut p: HashNode = 0 as *mut hash_node;
    prev = Hash_Locate(t, tbl_size, key);
    p = *prev;
    if p.is_null() {
        p = Pl_Malloc_Check(
            (::std::mem::size_of::<hash_node>() as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<PlLong>() as libc::c_ulong)
                .wrapping_add(elem_size as libc::c_ulong) as libc::c_uint,
            b"hash.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            276 as libc::c_int,
        ) as HashNode;
        (*p).next = 0 as HashNode;
        let ref mut fresh0 = *(tbl as *mut libc::c_int)
            .offset(2 as libc::c_int as isize);
        *fresh0 += 1;
        *fresh0;
        *prev = p;
        current_block = 8515828400728868193;
    } else if replace == 0 {
        current_block = 13210747023348852869;
    } else {
        current_block = 8515828400728868193;
    }
    match current_block {
        8515828400728868193 => {
            memcpy(
                &mut (*p).key as *mut PlLong as *mut libc::c_char as *mut libc::c_void,
                elem as *const libc::c_void,
                elem_size as libc::c_ulong,
            );
        }
        _ => {}
    }
    return &mut (*p).key as *mut PlLong as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_Hash_Find(
    mut tbl: *mut libc::c_char,
    mut key: PlLong,
) -> *mut libc::c_char {
    let mut tbl_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut t: *mut HashNode = (tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    let mut prev: *mut HashNode = 0 as *mut HashNode;
    let mut p: HashNode = 0 as *mut hash_node;
    prev = Hash_Locate(t, tbl_size, key);
    p = *prev;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    return &mut (*p).key as *mut PlLong as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_Hash_Delete(
    mut tbl: *mut libc::c_char,
    mut key: PlLong,
) -> *mut libc::c_char {
    let mut tbl_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut t: *mut HashNode = (tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    let mut prev: *mut HashNode = 0 as *mut HashNode;
    let mut p: HashNode = 0 as *mut hash_node;
    prev = Hash_Locate(t, tbl_size, key);
    p = *prev;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    *prev = (*p).next;
    free(p as *mut libc::c_void);
    let ref mut fresh1 = *(tbl as *mut libc::c_int).offset(2 as libc::c_int as isize);
    *fresh1 -= 1;
    *fresh1;
    return tbl;
}
unsafe extern "C" fn Hash_Locate(
    mut t: *mut HashNode,
    mut tbl_size: libc::c_int,
    mut key: PlLong,
) -> *mut HashNode {
    let mut n: libc::c_int = (key as PlULong).wrapping_rem(tbl_size as libc::c_ulong)
        as libc::c_int;
    let mut p: HashNode = 0 as *mut hash_node;
    t = t.offset(n as isize);
    p = *t;
    while !p.is_null() {
        if (*p).key == key {
            break;
        }
        t = &mut (*p).next;
        p = (*p).next;
    }
    return t;
}
pub unsafe extern "C" fn Pl_Hash_First(
    mut tbl: *mut libc::c_char,
    mut scan: *mut HashScan,
) -> *mut libc::c_char {
    let mut tbl_size: libc::c_int = *(tbl as *mut libc::c_int)
        .offset(0 as libc::c_int as isize);
    let mut t: *mut HashNode = (tbl as *mut HashNode).offset(3 as libc::c_int as isize);
    let mut endt: *mut HashNode = t.offset(tbl_size as isize);
    (*scan).endt = endt as *mut libc::c_char;
    (*scan).cur_t = t as *mut libc::c_char;
    (*scan).cur_p = *t as *mut libc::c_char;
    return Pl_Hash_Next(scan);
}
pub unsafe extern "C" fn Pl_Hash_Next(mut scan: *mut HashScan) -> *mut libc::c_char {
    let mut t: *mut HashNode = 0 as *mut HashNode;
    let mut endt: *mut HashNode = 0 as *mut HashNode;
    let mut p: HashNode = 0 as *mut hash_node;
    p = (*scan).cur_p as HashNode;
    if !p.is_null() {
        (*scan).cur_p = (*p).next as *mut libc::c_char;
        return &mut (*p).key as *mut PlLong as *mut libc::c_char;
    }
    t = (*scan).cur_t as *mut HashNode;
    endt = (*scan).endt as *mut HashNode;
    loop {
        t = t.offset(1);
        if !(t < endt) {
            break;
        }
        p = *t;
        if !p.is_null() {
            (*scan).cur_t = t as *mut libc::c_char;
            (*scan).cur_p = (*p).next as *mut libc::c_char;
            return &mut (*p).key as *mut PlLong as *mut libc::c_char;
        }
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_Hash_Table_Size(mut tbl: *mut libc::c_char) -> libc::c_int {
    return *(tbl as *mut libc::c_int).offset(0 as libc::c_int as isize);
}
pub unsafe extern "C" fn Pl_Hash_Nb_Elements(mut tbl: *mut libc::c_char) -> libc::c_int {
    return *(tbl as *mut libc::c_int).offset(2 as libc::c_int as isize);
}
