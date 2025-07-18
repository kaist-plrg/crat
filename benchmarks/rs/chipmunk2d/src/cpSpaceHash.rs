use ::libc;
extern "C" {
    pub type cpHashSet;
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn cpHashSetNew(size: libc::c_int, eqlFunc: cpHashSetEqlFunc) -> *mut cpHashSet;
    fn cpArrayPush(arr: *mut cpArray, object: *mut libc::c_void);
    fn cpSpatialIndexCollideStatic(
        dynamicIndex: *mut cpSpatialIndex,
        staticIndex: *mut cpSpatialIndex,
        func: cpSpatialIndexQueryFunc,
        data: *mut libc::c_void,
    );
    fn cpHashSetEach(
        set: *mut cpHashSet,
        func: cpHashSetIteratorFunc,
        data: *mut libc::c_void,
    );
    fn cpArrayPop(arr: *mut cpArray) -> *mut libc::c_void;
    fn cpHashSetInsert(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
        trans: cpHashSetTransFunc,
        data: *mut libc::c_void,
    ) -> *const libc::c_void;
    fn cpHashSetRemove(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
    ) -> *const libc::c_void;
    fn cpHashSetFind(
        set: *mut cpHashSet,
        hash: cpHashValue,
        ptr: *const libc::c_void,
    ) -> *const libc::c_void;
    fn cpHashSetCount(set: *mut cpHashSet) -> libc::c_int;
    fn cpArrayFree(arr: *mut cpArray);
    fn cpArrayFreeEach(
        arr: *mut cpArray,
        freeFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn cpHashSetFree(set: *mut cpHashSet);
    fn cpSpatialIndexInit(
        index: *mut cpSpatialIndex,
        klass_0: *mut cpSpatialIndexClass,
        bbfunc: cpSpatialIndexBBFunc,
        staticIndex: *mut cpSpatialIndex,
    ) -> *mut cpSpatialIndex;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type cpFloat = libc::c_double;
pub type cpHashValue = uintptr_t;
pub type cpCollisionID = uint32_t;
pub type cpBool = libc::c_uchar;
pub type cpTimestamp = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArray {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub arr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndex {
    pub klass: *mut cpSpatialIndexClass,
    pub bbfunc: cpSpatialIndexBBFunc,
    pub staticIndex: *mut cpSpatialIndex,
    pub dynamicIndex: *mut cpSpatialIndex,
}
pub type cpSpatialIndexBBFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> cpBB,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndexClass {
    pub destroy: cpSpatialIndexDestroyImpl,
    pub count: cpSpatialIndexCountImpl,
    pub each: cpSpatialIndexEachImpl,
    pub contains: cpSpatialIndexContainsImpl,
    pub insert: cpSpatialIndexInsertImpl,
    pub remove: cpSpatialIndexRemoveImpl,
    pub reindex: cpSpatialIndexReindexImpl,
    pub reindexObject: cpSpatialIndexReindexObjectImpl,
    pub reindexQuery: cpSpatialIndexReindexQueryImpl,
    pub query: cpSpatialIndexQueryImpl,
    pub segmentQuery: cpSpatialIndexSegmentQueryImpl,
}
pub type cpSpatialIndexSegmentQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        *mut libc::c_void,
        cpVect,
        cpVect,
        cpFloat,
        cpSpatialIndexSegmentQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexSegmentQueryFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> cpFloat,
>;
pub type cpSpatialIndexQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        *mut libc::c_void,
        cpBB,
        cpSpatialIndexQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexQueryFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        cpCollisionID,
        *mut libc::c_void,
    ) -> cpCollisionID,
>;
pub type cpSpatialIndexReindexQueryImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        cpSpatialIndexQueryFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexReindexObjectImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexReindexImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> (),
>;
pub type cpSpatialIndexRemoveImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexInsertImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
>;
pub type cpSpatialIndexContainsImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> cpBool,
>;
pub type cpSpatialIndexEachImpl = Option::<
    unsafe extern "C" fn(
        *mut cpSpatialIndex,
        cpSpatialIndexIteratorFunc,
        *mut libc::c_void,
    ) -> (),
>;
pub type cpSpatialIndexIteratorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpSpatialIndexCountImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> libc::c_int,
>;
pub type cpSpatialIndexDestroyImpl = Option::<
    unsafe extern "C" fn(*mut cpSpatialIndex) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceHash {
    pub spatialIndex: cpSpatialIndex,
    pub numcells: libc::c_int,
    pub celldim: cpFloat,
    pub table: *mut *mut cpSpaceHashBin,
    pub handleSet: *mut cpHashSet,
    pub pooledBins: *mut cpSpaceHashBin,
    pub pooledHandles: *mut cpArray,
    pub allocatedBuffers: *mut cpArray,
    pub stamp: cpTimestamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceHashBin {
    pub handle: *mut cpHandle,
    pub next: *mut cpSpaceHashBin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpHandle {
    pub obj: *mut libc::c_void,
    pub retain: libc::c_int,
    pub stamp: cpTimestamp,
}
pub type cpHashSetEqlFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queryRehashContext {
    pub hash: *mut cpSpaceHash,
    pub func: cpSpatialIndexQueryFunc,
    pub data: *mut libc::c_void,
}
pub type cpHashSetIteratorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type cpHashSetTransFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eachContext {
    pub func: cpSpatialIndexIteratorFunc,
    pub data: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    return if f < 0 as libc::c_int as libc::c_double { -f } else { f };
}
#[inline]
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = {
        let mut init = cpVect { x: x, y: y };
        init
    };
    return v;
}
#[inline]
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
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
unsafe extern "C" fn cpHandleInit(
    mut hand: *mut cpHandle,
    mut obj: *mut libc::c_void,
) -> *mut cpHandle {
    (*hand).obj = obj;
    (*hand).retain = 0 as libc::c_int;
    (*hand).stamp = 0 as libc::c_int as cpTimestamp;
    return hand;
}
#[inline]
unsafe extern "C" fn cpHandleRetain(mut hand: *mut cpHandle) {
    (*hand).retain += 1;
    (*hand).retain;
}
#[inline]
unsafe extern "C" fn cpHandleRelease(
    mut hand: *mut cpHandle,
    mut pooledHandles: *mut cpArray,
) {
    (*hand).retain -= 1;
    (*hand).retain;
    if (*hand).retain == 0 as libc::c_int {
        cpArrayPush(pooledHandles, hand as *mut libc::c_void);
    }
}
unsafe extern "C" fn handleSetEql(
    mut obj: *mut libc::c_void,
    mut hand: *mut cpHandle,
) -> libc::c_int {
    return (obj == (*hand).obj) as libc::c_int;
}
unsafe extern "C" fn handleSetTrans(
    mut obj: *mut libc::c_void,
    mut hash: *mut cpSpaceHash,
) -> *mut libc::c_void {
    if (*(*hash).pooledHandles).num == 0 as libc::c_int {
        let mut count: libc::c_int = ((32 as libc::c_int * 1024 as libc::c_int)
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpHandle>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../../src/cpSpaceHash.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        let mut buffer: *mut cpHandle = calloc(
            1 as libc::c_int as libc::c_ulong,
            (32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
        ) as *mut cpHandle;
        cpArrayPush((*hash).allocatedBuffers, buffer as *mut libc::c_void);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < count {
            cpArrayPush(
                (*hash).pooledHandles,
                buffer.offset(i as isize) as *mut libc::c_void,
            );
            i += 1;
            i;
        }
    }
    let mut hand: *mut cpHandle = cpHandleInit(
        cpArrayPop((*hash).pooledHandles) as *mut cpHandle,
        obj,
    );
    cpHandleRetain(hand);
    return hand as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn recycleBin(
    mut hash: *mut cpSpaceHash,
    mut bin: *mut cpSpaceHashBin,
) {
    (*bin).next = (*hash).pooledBins;
    (*hash).pooledBins = bin;
}
#[inline]
unsafe extern "C" fn clearTableCell(mut hash: *mut cpSpaceHash, mut idx: libc::c_int) {
    let mut bin: *mut cpSpaceHashBin = *((*hash).table).offset(idx as isize);
    while !bin.is_null() {
        let mut next: *mut cpSpaceHashBin = (*bin).next;
        cpHandleRelease((*bin).handle, (*hash).pooledHandles);
        recycleBin(hash, bin);
        bin = next;
    }
    let ref mut fresh0 = *((*hash).table).offset(idx as isize);
    *fresh0 = 0 as *mut cpSpaceHashBin;
}
unsafe extern "C" fn clearTable(mut hash: *mut cpSpaceHash) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*hash).numcells {
        clearTableCell(hash, i);
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn getEmptyBin(mut hash: *mut cpSpaceHash) -> *mut cpSpaceHashBin {
    let mut bin: *mut cpSpaceHashBin = (*hash).pooledBins;
    if !bin.is_null() {
        (*hash).pooledBins = (*bin).next;
        return bin;
    } else {
        let mut count: libc::c_int = ((32 as libc::c_int * 1024 as libc::c_int)
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpSpaceHashBin>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../../src/cpSpaceHash.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        let mut buffer: *mut cpSpaceHashBin = calloc(
            1 as libc::c_int as libc::c_ulong,
            (32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong,
        ) as *mut cpSpaceHashBin;
        cpArrayPush((*hash).allocatedBuffers, buffer as *mut libc::c_void);
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < count {
            recycleBin(hash, buffer.offset(i as isize));
            i += 1;
            i;
        }
        return buffer;
    };
}
pub unsafe extern "C" fn cpSpaceHashAlloc() -> *mut cpSpaceHash {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpSpaceHash>() as libc::c_ulong,
    ) as *mut cpSpaceHash;
}
unsafe extern "C" fn cpSpaceHashAllocTable(
    mut hash: *mut cpSpaceHash,
    mut numcells: libc::c_int,
) {
    free((*hash).table as *mut libc::c_void);
    (*hash).numcells = numcells;
    (*hash)
        .table = calloc(
        numcells as libc::c_ulong,
        ::std::mem::size_of::<*mut cpSpaceHashBin>() as libc::c_ulong,
    ) as *mut *mut cpSpaceHashBin;
}
pub unsafe extern "C" fn cpSpaceHashInit(
    mut hash: *mut cpSpaceHash,
    mut celldim: cpFloat,
    mut numcells: libc::c_int,
    mut bbfunc: cpSpatialIndexBBFunc,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    cpSpatialIndexInit(hash as *mut cpSpatialIndex, Klass(), bbfunc, staticIndex);
    cpSpaceHashAllocTable(hash, next_prime(numcells));
    (*hash).celldim = celldim;
    (*hash)
        .handleSet = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut cpHandle) -> libc::c_int,
            >,
            cpHashSetEqlFunc,
        >(
            Some(
                handleSetEql
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cpHandle,
                    ) -> libc::c_int,
            ),
        ),
    );
    (*hash).pooledHandles = cpArrayNew(0 as libc::c_int);
    (*hash).pooledBins = 0 as *mut cpSpaceHashBin;
    (*hash).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    (*hash).stamp = 1 as libc::c_int as cpTimestamp;
    return hash as *mut cpSpatialIndex;
}
pub unsafe extern "C" fn cpSpaceHashNew(
    mut celldim: cpFloat,
    mut cells: libc::c_int,
    mut bbfunc: cpSpatialIndexBBFunc,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    return cpSpaceHashInit(cpSpaceHashAlloc(), celldim, cells, bbfunc, staticIndex);
}
unsafe extern "C" fn cpSpaceHashDestroy(mut hash: *mut cpSpaceHash) {
    if !((*hash).table).is_null() {
        clearTable(hash);
    }
    free((*hash).table as *mut libc::c_void);
    cpHashSetFree((*hash).handleSet);
    cpArrayFreeEach(
        (*hash).allocatedBuffers,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    cpArrayFree((*hash).allocatedBuffers);
    cpArrayFree((*hash).pooledHandles);
}
#[inline]
unsafe extern "C" fn containsHandle(
    mut bin: *mut cpSpaceHashBin,
    mut hand: *mut cpHandle,
) -> cpBool {
    while !bin.is_null() {
        if (*bin).handle == hand {
            return 1 as libc::c_int as cpBool;
        }
        bin = (*bin).next;
    }
    return 0 as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn hash_func(
    mut x: cpHashValue,
    mut y: cpHashValue,
    mut n: cpHashValue,
) -> cpHashValue {
    return (x.wrapping_mul(1640531513 as libc::c_ulong)
        ^ y.wrapping_mul(2654435789 as libc::c_ulong))
        .wrapping_rem(n);
}
#[inline]
unsafe extern "C" fn floor_int(mut f: cpFloat) -> libc::c_int {
    let mut i: libc::c_int = f as libc::c_int;
    return if f < 0.0f32 as libc::c_double && f != i as libc::c_double {
        i - 1 as libc::c_int
    } else {
        i
    };
}
#[inline]
unsafe extern "C" fn hashHandle(
    mut hash: *mut cpSpaceHash,
    mut hand: *mut cpHandle,
    mut bb: cpBB,
) {
    let mut dim: cpFloat = (*hash).celldim;
    let mut l: libc::c_int = floor_int(bb.l / dim);
    let mut r: libc::c_int = floor_int(bb.r / dim);
    let mut b: libc::c_int = floor_int(bb.b / dim);
    let mut t: libc::c_int = floor_int(bb.t / dim);
    let mut n: libc::c_int = (*hash).numcells;
    let mut i: libc::c_int = l;
    while i <= r {
        let mut j: libc::c_int = b;
        while j <= t {
            let mut idx: cpHashValue = hash_func(
                i as cpHashValue,
                j as cpHashValue,
                n as cpHashValue,
            );
            let mut bin: *mut cpSpaceHashBin = *((*hash).table).offset(idx as isize);
            if !(containsHandle(bin, hand) != 0) {
                cpHandleRetain(hand);
                let mut newBin: *mut cpSpaceHashBin = getEmptyBin(hash);
                (*newBin).handle = hand;
                (*newBin).next = bin;
                let ref mut fresh1 = *((*hash).table).offset(idx as isize);
                *fresh1 = newBin;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn cpSpaceHashInsert(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut hand: *mut cpHandle = cpHashSetInsert(
        (*hash).handleSet,
        hashid,
        obj,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cpSpaceHash,
                ) -> *mut libc::c_void,
            >,
            cpHashSetTransFunc,
        >(
            Some(
                handleSetTrans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cpSpaceHash,
                    ) -> *mut libc::c_void,
            ),
        ),
        hash as *mut libc::c_void,
    ) as *mut cpHandle;
    hashHandle(hash, hand, ((*hash).spatialIndex.bbfunc).unwrap()(obj));
}
unsafe extern "C" fn cpSpaceHashRehashObject(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut hand: *mut cpHandle = cpHashSetRemove((*hash).handleSet, hashid, obj)
        as *mut cpHandle;
    if !hand.is_null() {
        (*hand).obj = 0 as *mut libc::c_void;
        cpHandleRelease(hand, (*hash).pooledHandles);
        cpSpaceHashInsert(hash, obj, hashid);
    }
}
unsafe extern "C" fn rehash_helper(mut hand: *mut cpHandle, mut hash: *mut cpSpaceHash) {
    hashHandle(hash, hand, ((*hash).spatialIndex.bbfunc).unwrap()((*hand).obj));
}
unsafe extern "C" fn cpSpaceHashRehash(mut hash: *mut cpSpaceHash) {
    clearTable(hash);
    cpHashSetEach(
        (*hash).handleSet,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpHandle, *mut cpSpaceHash) -> ()>,
            cpHashSetIteratorFunc,
        >(
            Some(
                rehash_helper
                    as unsafe extern "C" fn(*mut cpHandle, *mut cpSpaceHash) -> (),
            ),
        ),
        hash as *mut libc::c_void,
    );
}
unsafe extern "C" fn cpSpaceHashRemove(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut hand: *mut cpHandle = cpHashSetRemove((*hash).handleSet, hashid, obj)
        as *mut cpHandle;
    if !hand.is_null() {
        (*hand).obj = 0 as *mut libc::c_void;
        cpHandleRelease(hand, (*hash).pooledHandles);
    }
}
unsafe extern "C" fn eachHelper(mut hand: *mut cpHandle, mut context: *mut eachContext) {
    ((*context).func).unwrap()((*hand).obj, (*context).data);
}
unsafe extern "C" fn cpSpaceHashEach(
    mut hash: *mut cpSpaceHash,
    mut func: cpSpatialIndexIteratorFunc,
    mut data: *mut libc::c_void,
) {
    let mut context: eachContext = {
        let mut init = eachContext {
            func: func,
            data: data,
        };
        init
    };
    cpHashSetEach(
        (*hash).handleSet,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpHandle, *mut eachContext) -> ()>,
            cpHashSetIteratorFunc,
        >(
            Some(
                eachHelper as unsafe extern "C" fn(*mut cpHandle, *mut eachContext) -> (),
            ),
        ),
        &mut context as *mut eachContext as *mut libc::c_void,
    );
}
unsafe extern "C" fn remove_orphaned_handles(
    mut hash: *mut cpSpaceHash,
    mut bin_ptr: *mut *mut cpSpaceHashBin,
) {
    let mut bin: *mut cpSpaceHashBin = *bin_ptr;
    while !bin.is_null() {
        let mut hand: *mut cpHandle = (*bin).handle;
        let mut next: *mut cpSpaceHashBin = (*bin).next;
        if ((*hand).obj).is_null() {
            *bin_ptr = (*bin).next;
            recycleBin(hash, bin);
            cpHandleRelease(hand, (*hash).pooledHandles);
        } else {
            bin_ptr = &mut (*bin).next;
        }
        bin = next;
    }
}
#[inline]
unsafe extern "C" fn query_helper(
    mut hash: *mut cpSpaceHash,
    mut bin_ptr: *mut *mut cpSpaceHashBin,
    mut obj: *mut libc::c_void,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    's_1: loop {
        let mut bin: *mut cpSpaceHashBin = *bin_ptr;
        loop {
            if bin.is_null() {
                break 's_1;
            }
            let mut hand: *mut cpHandle = (*bin).handle;
            let mut other: *mut libc::c_void = (*hand).obj;
            if !((*hand).stamp == (*hash).stamp || obj == other) {
                if !other.is_null() {
                    func.unwrap()(obj, other, 0 as libc::c_int as cpCollisionID, data);
                    (*hand).stamp = (*hash).stamp;
                } else {
                    remove_orphaned_handles(hash, bin_ptr);
                    break;
                }
            }
            bin = (*bin).next;
        }
    };
}
unsafe extern "C" fn cpSpaceHashQuery(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    let mut dim: cpFloat = (*hash).celldim;
    let mut l: libc::c_int = floor_int(bb.l / dim);
    let mut r: libc::c_int = floor_int(bb.r / dim);
    let mut b: libc::c_int = floor_int(bb.b / dim);
    let mut t: libc::c_int = floor_int(bb.t / dim);
    let mut n: libc::c_int = (*hash).numcells;
    let mut table: *mut *mut cpSpaceHashBin = (*hash).table;
    let mut i: libc::c_int = l;
    while i <= r {
        let mut j: libc::c_int = b;
        while j <= t {
            query_helper(
                hash,
                &mut *table
                    .offset(
                        (hash_func
                            as unsafe extern "C" fn(
                                cpHashValue,
                                cpHashValue,
                                cpHashValue,
                            ) -> cpHashValue)(
                            i as cpHashValue,
                            j as cpHashValue,
                            n as cpHashValue,
                        ) as isize,
                    ),
                obj,
                func,
                data,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*hash).stamp = ((*hash).stamp).wrapping_add(1);
    (*hash).stamp;
}
unsafe extern "C" fn queryRehash_helper(
    mut hand: *mut cpHandle,
    mut context: *mut queryRehashContext,
) {
    let mut hash: *mut cpSpaceHash = (*context).hash;
    let mut func: cpSpatialIndexQueryFunc = (*context).func;
    let mut data: *mut libc::c_void = (*context).data;
    let mut dim: cpFloat = (*hash).celldim;
    let mut n: libc::c_int = (*hash).numcells;
    let mut obj: *mut libc::c_void = (*hand).obj;
    let mut bb: cpBB = ((*hash).spatialIndex.bbfunc).unwrap()(obj);
    let mut l: libc::c_int = floor_int(bb.l / dim);
    let mut r: libc::c_int = floor_int(bb.r / dim);
    let mut b: libc::c_int = floor_int(bb.b / dim);
    let mut t: libc::c_int = floor_int(bb.t / dim);
    let mut table: *mut *mut cpSpaceHashBin = (*hash).table;
    let mut i: libc::c_int = l;
    while i <= r {
        let mut j: libc::c_int = b;
        while j <= t {
            let mut idx: cpHashValue = hash_func(
                i as cpHashValue,
                j as cpHashValue,
                n as cpHashValue,
            );
            let mut bin: *mut cpSpaceHashBin = *table.offset(idx as isize);
            if !(containsHandle(bin, hand) != 0) {
                cpHandleRetain(hand);
                query_helper(hash, &mut bin, obj, func, data);
                let mut newBin: *mut cpSpaceHashBin = getEmptyBin(hash);
                (*newBin).handle = hand;
                (*newBin).next = bin;
                let ref mut fresh2 = *table.offset(idx as isize);
                *fresh2 = newBin;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*hash).stamp = ((*hash).stamp).wrapping_add(1);
    (*hash).stamp;
}
unsafe extern "C" fn cpSpaceHashReindexQuery(
    mut hash: *mut cpSpaceHash,
    mut func: cpSpatialIndexQueryFunc,
    mut data: *mut libc::c_void,
) {
    clearTable(hash);
    let mut context: queryRehashContext = {
        let mut init = queryRehashContext {
            hash: hash,
            func: func,
            data: data,
        };
        init
    };
    cpHashSetEach(
        (*hash).handleSet,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpHandle, *mut queryRehashContext) -> ()>,
            cpHashSetIteratorFunc,
        >(
            Some(
                queryRehash_helper
                    as unsafe extern "C" fn(*mut cpHandle, *mut queryRehashContext) -> (),
            ),
        ),
        &mut context as *mut queryRehashContext as *mut libc::c_void,
    );
    cpSpatialIndexCollideStatic(
        hash as *mut cpSpatialIndex,
        (*hash).spatialIndex.staticIndex,
        func,
        data,
    );
}
#[inline]
unsafe extern "C" fn segmentQuery_helper(
    mut hash: *mut cpSpaceHash,
    mut bin_ptr: *mut *mut cpSpaceHashBin,
    mut obj: *mut libc::c_void,
    mut func: cpSpatialIndexSegmentQueryFunc,
    mut data: *mut libc::c_void,
) -> cpFloat {
    let mut t: cpFloat = 1.0f32 as cpFloat;
    's_3: loop {
        let mut bin: *mut cpSpaceHashBin = *bin_ptr;
        loop {
            if bin.is_null() {
                break 's_3;
            }
            let mut hand: *mut cpHandle = (*bin).handle;
            let mut other: *mut libc::c_void = (*hand).obj;
            if !((*hand).stamp == (*hash).stamp) {
                if !other.is_null() {
                    t = cpfmin(t, func.unwrap()(obj, other, data));
                    (*hand).stamp = (*hash).stamp;
                } else {
                    remove_orphaned_handles(hash, bin_ptr);
                    break;
                }
            }
            bin = (*bin).next;
        }
    }
    return t;
}
unsafe extern "C" fn cpSpaceHashSegmentQuery(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: cpSpatialIndexSegmentQueryFunc,
    mut data: *mut libc::c_void,
) {
    a = cpvmult(a, 1.0f32 as libc::c_double / (*hash).celldim);
    b = cpvmult(b, 1.0f32 as libc::c_double / (*hash).celldim);
    let mut cell_x: libc::c_int = floor_int(a.x);
    let mut cell_y: libc::c_int = floor_int(a.y);
    let mut t: cpFloat = 0 as libc::c_int as cpFloat;
    let mut x_inc: libc::c_int = 0;
    let mut y_inc: libc::c_int = 0;
    let mut temp_v: cpFloat = 0.;
    let mut temp_h: cpFloat = 0.;
    if b.x > a.x {
        x_inc = 1 as libc::c_int;
        temp_h = floor(a.x + 1.0f32 as libc::c_double) - a.x;
    } else {
        x_inc = -(1 as libc::c_int);
        temp_h = a.x - floor(a.x);
    }
    if b.y > a.y {
        y_inc = 1 as libc::c_int;
        temp_v = floor(a.y + 1.0f32 as libc::c_double) - a.y;
    } else {
        y_inc = -(1 as libc::c_int);
        temp_v = a.y - floor(a.y);
    }
    let mut dx: cpFloat = cpfabs(b.x - a.x);
    let mut dy: cpFloat = cpfabs(b.y - a.y);
    let mut dt_dx: cpFloat = if dx != 0. {
        1.0f32 as libc::c_double / dx
    } else {
        ::std::f32::INFINITY as libc::c_double
    };
    let mut dt_dy: cpFloat = if dy != 0. {
        1.0f32 as libc::c_double / dy
    } else {
        ::std::f32::INFINITY as libc::c_double
    };
    let mut next_h: cpFloat = if temp_h != 0. { temp_h * dt_dx } else { dt_dx };
    let mut next_v: cpFloat = if temp_v != 0. { temp_v * dt_dy } else { dt_dy };
    let mut n: libc::c_int = (*hash).numcells;
    let mut table: *mut *mut cpSpaceHashBin = (*hash).table;
    while t < t_exit {
        let mut idx: cpHashValue = hash_func(
            cell_x as cpHashValue,
            cell_y as cpHashValue,
            n as cpHashValue,
        );
        t_exit = cpfmin(
            t_exit,
            segmentQuery_helper(hash, &mut *table.offset(idx as isize), obj, func, data),
        );
        if next_v < next_h {
            cell_y += y_inc;
            t = next_v;
            next_v += dt_dy;
        } else {
            cell_x += x_inc;
            t = next_h;
            next_h += dt_dx;
        }
    }
    (*hash).stamp = ((*hash).stamp).wrapping_add(1);
    (*hash).stamp;
}
pub unsafe extern "C" fn cpSpaceHashResize(
    mut hash: *mut cpSpaceHash,
    mut celldim: cpFloat,
    mut numcells: libc::c_int,
) {
    if (*hash).spatialIndex.klass != Klass() {
        return;
    }
    clearTable(hash);
    (*hash).celldim = celldim;
    cpSpaceHashAllocTable(hash, next_prime(numcells));
}
unsafe extern "C" fn cpSpaceHashCount(mut hash: *mut cpSpaceHash) -> libc::c_int {
    return cpHashSetCount((*hash).handleSet);
}
unsafe extern "C" fn cpSpaceHashContains(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) -> libc::c_int {
    return (cpHashSetFind((*hash).handleSet, hashid, obj)
        != 0 as *mut libc::c_void as *const libc::c_void) as libc::c_int;
}
static mut klass: cpSpatialIndexClass = unsafe {
    {
        let mut init = cpSpatialIndexClass {
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSpaceHash) -> ()>,
                cpSpatialIndexDestroyImpl,
            >(Some(cpSpaceHashDestroy as unsafe extern "C" fn(*mut cpSpaceHash) -> ())),
            count: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSpaceHash) -> libc::c_int>,
                cpSpatialIndexCountImpl,
            >(
                Some(
                    cpSpaceHashCount
                        as unsafe extern "C" fn(*mut cpSpaceHash) -> libc::c_int,
                ),
            ),
            each: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        cpSpatialIndexIteratorFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexEachImpl,
            >(
                Some(
                    cpSpaceHashEach
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            cpSpatialIndexIteratorFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            contains: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> libc::c_int,
                >,
                cpSpatialIndexContainsImpl,
            >(
                Some(
                    cpSpaceHashContains
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> libc::c_int,
                ),
            ),
            insert: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexInsertImpl,
            >(
                Some(
                    cpSpaceHashInsert
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            remove: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexRemoveImpl,
            >(
                Some(
                    cpSpaceHashRemove
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindex: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSpaceHash) -> ()>,
                cpSpatialIndexReindexImpl,
            >(Some(cpSpaceHashRehash as unsafe extern "C" fn(*mut cpSpaceHash) -> ())),
            reindexObject: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                cpSpatialIndexReindexObjectImpl,
            >(
                Some(
                    cpSpaceHashRehashObject
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindexQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        cpSpatialIndexQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexReindexQueryImpl,
            >(
                Some(
                    cpSpaceHashReindexQuery
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            cpSpatialIndexQueryFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            query: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpBB,
                        cpSpatialIndexQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexQueryImpl,
            >(
                Some(
                    cpSpaceHashQuery
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpBB,
                            cpSpatialIndexQueryFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        cpSpatialIndexSegmentQueryFunc,
                        *mut libc::c_void,
                    ) -> (),
                >,
                cpSpatialIndexSegmentQueryImpl,
            >(
                Some(
                    cpSpaceHashSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpVect,
                            cpVect,
                            cpFloat,
                            cpSpatialIndexSegmentQueryFunc,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn Klass() -> *mut cpSpatialIndexClass {
    return &mut klass;
}
