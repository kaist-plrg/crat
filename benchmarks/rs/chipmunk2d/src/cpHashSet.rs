use ::libc;
extern "C" {
    fn abort() -> !;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn cpMessage(
        condition: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        isError: libc::c_int,
        isHardError: libc::c_int,
        message: *const libc::c_char,
        _: ...
    );
    fn cpArrayNew(size: libc::c_int) -> *mut cpArray;
    fn cpArrayFree(arr: *mut cpArray);
    fn cpArrayPush(arr: *mut cpArray, object: *mut libc::c_void);
    fn cpArrayFreeEach(
        arr: *mut cpArray,
        freeFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
}
pub type uintptr_t = libc::c_ulong;
pub type cpHashValue = uintptr_t;
pub type cpBool = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArray {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub arr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpHashSet {
    pub entries: libc::c_uint,
    pub size: libc::c_uint,
    pub eql: cpHashSetEqlFunc,
    pub default_value: *mut libc::c_void,
    pub table: *mut *mut cpHashSetBin,
    pub pooledBins: *mut cpHashSetBin,
    pub allocatedBuffers: *mut cpArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpHashSetBin {
    pub elt: *mut libc::c_void,
    pub hash: cpHashValue,
    pub next: *mut cpHashSetBin,
}
pub type cpHashSetEqlFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
>;
pub type cpHashSetTransFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
pub type cpHashSetIteratorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpHashSetFilterFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
>;
static mut primes: [libc::c_int; 30] = [
    5 as libc::c_int,
    13 as libc::c_int,
    23 as libc::c_int,
    47 as libc::c_int,
    97 as libc::c_int,
    193 as libc::c_int,
    389 as libc::c_int,
    769 as libc::c_int,
    1543 as libc::c_int,
    3079 as libc::c_int,
    6151 as libc::c_int,
    12289 as libc::c_int,
    24593 as libc::c_int,
    49157 as libc::c_int,
    98317 as libc::c_int,
    196613 as libc::c_int,
    393241 as libc::c_int,
    786433 as libc::c_int,
    1572869 as libc::c_int,
    3145739 as libc::c_int,
    6291469 as libc::c_int,
    12582917 as libc::c_int,
    25165843 as libc::c_int,
    50331653 as libc::c_int,
    100663319 as libc::c_int,
    201326611 as libc::c_int,
    402653189 as libc::c_int,
    805306457 as libc::c_int,
    1610612741 as libc::c_int,
    0 as libc::c_int,
];
#[inline]
unsafe extern "C" fn next_prime(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while n > primes[i as usize] {
        i += 1;
        i;
        if primes[i as usize] == 0 {
            cpMessage(
                b"primes[i]\0" as *const u8 as *const libc::c_char,
                b"../../src/prime.h\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Tried to resize a hash table to a size greater than 1610612741 O_o\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    }
    return primes[i as usize];
}
pub unsafe extern "C" fn cpHashSetFree(mut set: *mut cpHashSet) {
    if !set.is_null() {
        free((*set).table as *mut libc::c_void);
        cpArrayFreeEach(
            (*set).allocatedBuffers,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        cpArrayFree((*set).allocatedBuffers);
        free(set as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpHashSetNew(
    mut size: libc::c_int,
    mut eqlFunc: cpHashSetEqlFunc,
) -> *mut cpHashSet {
    let mut set: *mut cpHashSet = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpHashSet>() as libc::c_ulong,
    ) as *mut cpHashSet;
    (*set).size = next_prime(size) as libc::c_uint;
    (*set).entries = 0 as libc::c_int as libc::c_uint;
    (*set).eql = eqlFunc;
    (*set).default_value = 0 as *mut libc::c_void;
    (*set)
        .table = calloc(
        (*set).size as libc::c_ulong,
        ::std::mem::size_of::<*mut cpHashSetBin>() as libc::c_ulong,
    ) as *mut *mut cpHashSetBin;
    (*set).pooledBins = 0 as *mut cpHashSetBin;
    (*set).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    return set;
}
pub unsafe extern "C" fn cpHashSetSetDefaultValue(
    mut set: *mut cpHashSet,
    mut default_value: *mut libc::c_void,
) {
    (*set).default_value = default_value;
}
unsafe extern "C" fn setIsFull(mut set: *mut cpHashSet) -> libc::c_int {
    return ((*set).entries >= (*set).size) as libc::c_int;
}
unsafe extern "C" fn cpHashSetResize(mut set: *mut cpHashSet) {
    let mut newSize: libc::c_uint = next_prime(
        ((*set).size).wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
    ) as libc::c_uint;
    let mut newTable: *mut *mut cpHashSetBin = calloc(
        newSize as libc::c_ulong,
        ::std::mem::size_of::<*mut cpHashSetBin>() as libc::c_ulong,
    ) as *mut *mut cpHashSetBin;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*set).size {
        let mut bin: *mut cpHashSetBin = *((*set).table).offset(i as isize);
        while !bin.is_null() {
            let mut next: *mut cpHashSetBin = (*bin).next;
            let mut idx: cpHashValue = ((*bin).hash)
                .wrapping_rem(newSize as libc::c_ulong);
            (*bin).next = *newTable.offset(idx as isize);
            let ref mut fresh0 = *newTable.offset(idx as isize);
            *fresh0 = bin;
            bin = next;
        }
        i = i.wrapping_add(1);
        i;
    }
    free((*set).table as *mut libc::c_void);
    (*set).table = newTable;
    (*set).size = newSize;
}
#[inline]
unsafe extern "C" fn recycleBin(mut set: *mut cpHashSet, mut bin: *mut cpHashSetBin) {
    (*bin).next = (*set).pooledBins;
    (*set).pooledBins = bin;
    (*bin).elt = 0 as *mut libc::c_void;
}
unsafe extern "C" fn getUnusedBin(mut set: *mut cpHashSet) -> *mut cpHashSetBin {
    let mut bin: *mut cpHashSetBin = (*set).pooledBins;
    if !bin.is_null() {
        (*set).pooledBins = (*bin).next;
        return bin;
    } else {
        let mut count: libc::c_int = ((32 as libc::c_int * 1024 as libc::c_int)
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpHashSetBin>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../../src/cpHashSet.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        let mut buffer: *mut cpHashSetBin = calloc(
            1 as libc::c_int as libc::c_ulong,
            (32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
        ) as *mut cpHashSetBin;
        cpArrayPush((*set).allocatedBuffers, buffer as *mut libc::c_void);
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < count {
            recycleBin(set, buffer.offset(i as isize));
            i += 1;
            i;
        }
        return buffer;
    };
}
pub unsafe extern "C" fn cpHashSetCount(mut set: *mut cpHashSet) -> libc::c_int {
    return (*set).entries as libc::c_int;
}
pub unsafe extern "C" fn cpHashSetInsert(
    mut set: *mut cpHashSet,
    mut hash: cpHashValue,
    mut ptr: *const libc::c_void,
    mut trans: cpHashSetTransFunc,
    mut data: *mut libc::c_void,
) -> *const libc::c_void {
    let mut idx: cpHashValue = hash.wrapping_rem((*set).size as libc::c_ulong);
    let mut bin: *mut cpHashSetBin = *((*set).table).offset(idx as isize);
    while !bin.is_null() && ((*set).eql).unwrap()(ptr, (*bin).elt) == 0 {
        bin = (*bin).next;
    }
    if bin.is_null() {
        bin = getUnusedBin(set);
        (*bin).hash = hash;
        (*bin).elt = if trans.is_some() { trans.unwrap()(ptr, data) } else { data };
        (*bin).next = *((*set).table).offset(idx as isize);
        let ref mut fresh1 = *((*set).table).offset(idx as isize);
        *fresh1 = bin;
        (*set).entries = ((*set).entries).wrapping_add(1);
        (*set).entries;
        if setIsFull(set) != 0 {
            cpHashSetResize(set);
        }
    }
    return (*bin).elt;
}
pub unsafe extern "C" fn cpHashSetRemove(
    mut set: *mut cpHashSet,
    mut hash: cpHashValue,
    mut ptr: *const libc::c_void,
) -> *const libc::c_void {
    let mut idx: cpHashValue = hash.wrapping_rem((*set).size as libc::c_ulong);
    let mut prev_ptr: *mut *mut cpHashSetBin = &mut *((*set).table).offset(idx as isize)
        as *mut *mut cpHashSetBin;
    let mut bin: *mut cpHashSetBin = *((*set).table).offset(idx as isize);
    while !bin.is_null() && ((*set).eql).unwrap()(ptr, (*bin).elt) == 0 {
        prev_ptr = &mut (*bin).next;
        bin = (*bin).next;
    }
    if !bin.is_null() {
        *prev_ptr = (*bin).next;
        (*set).entries = ((*set).entries).wrapping_sub(1);
        (*set).entries;
        let mut elt: *const libc::c_void = (*bin).elt;
        recycleBin(set, bin);
        return elt;
    }
    return 0 as *const libc::c_void;
}
pub unsafe extern "C" fn cpHashSetFind(
    mut set: *mut cpHashSet,
    mut hash: cpHashValue,
    mut ptr: *const libc::c_void,
) -> *const libc::c_void {
    let mut idx: cpHashValue = hash.wrapping_rem((*set).size as libc::c_ulong);
    let mut bin: *mut cpHashSetBin = *((*set).table).offset(idx as isize);
    while !bin.is_null() && ((*set).eql).unwrap()(ptr, (*bin).elt) == 0 {
        bin = (*bin).next;
    }
    return if !bin.is_null() { (*bin).elt } else { (*set).default_value };
}
pub unsafe extern "C" fn cpHashSetEach(
    mut set: *mut cpHashSet,
    mut func: cpHashSetIteratorFunc,
    mut data: *mut libc::c_void,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*set).size {
        let mut bin: *mut cpHashSetBin = *((*set).table).offset(i as isize);
        while !bin.is_null() {
            let mut next: *mut cpHashSetBin = (*bin).next;
            func.unwrap()((*bin).elt, data);
            bin = next;
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn cpHashSetFilter(
    mut set: *mut cpHashSet,
    mut func: cpHashSetFilterFunc,
    mut data: *mut libc::c_void,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*set).size {
        let mut prev_ptr: *mut *mut cpHashSetBin = &mut *((*set).table)
            .offset(i as isize) as *mut *mut cpHashSetBin;
        let mut bin: *mut cpHashSetBin = *((*set).table).offset(i as isize);
        while !bin.is_null() {
            let mut next: *mut cpHashSetBin = (*bin).next;
            if func.unwrap()((*bin).elt, data) != 0 {
                prev_ptr = &mut (*bin).next;
            } else {
                *prev_ptr = next;
                (*set).entries = ((*set).entries).wrapping_sub(1);
                (*set).entries;
                recycleBin(set, bin);
            }
            bin = next;
        }
        i = i.wrapping_add(1);
        i;
    }
}
