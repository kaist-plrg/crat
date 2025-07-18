use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn wcsdup(__s: *const wchar_t) -> *mut wchar_t;
    fn towupper(__wc: wint_t) -> wint_t;
    fn low_level_dir_write_end(Dir: *mut Stream_t, entry: libc::c_int);
    fn getDirCacheP(Stream: *mut Stream_t) -> *mut *mut dirCache_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type wchar_t = libc::c_int;
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
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub attr: libc::c_uchar,
    pub Case: libc::c_uchar,
    pub ctime_ms: libc::c_uchar,
    pub ctime: [libc::c_uchar; 2],
    pub cdate: [libc::c_uchar; 2],
    pub adate: [libc::c_uchar; 2],
    pub startHi: [libc::c_uchar; 2],
    pub time: [libc::c_uchar; 2],
    pub date: [libc::c_uchar; 2],
    pub start: [libc::c_uchar; 2],
    pub size: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: libc::c_int,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub pread: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub pwrite: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub freeFunc: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub set_geom: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> libc::c_int,
    >,
    pub get_data: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut libc::c_int,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub pre_allocate: Option::<
        unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
    >,
    pub get_dosConvert: Option::<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
}
pub type mt_off_t = off_t;
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const libc::c_char,
    pub drive: libc::c_char,
    pub fat_bits: libc::c_int,
    pub mode: libc::c_int,
    pub tracks: libc::c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: libc::c_uint,
    pub offset: off_t,
    pub partition: libc::c_uint,
    pub misc_flags: libc::c_uint,
    pub ssize: uint8_t,
    pub use_2m: libc::c_uint,
    pub precmd: *mut libc::c_char,
    pub file_nr: libc::c_int,
    pub blocksize: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data_map: *const libc::c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut libc::c_char,
    pub cfg_filename: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirCache_t {
    pub entries: *mut *mut dirCacheEntry_t,
    pub nr_entries: libc::c_uint,
    pub nrHashed: libc::c_uint,
    pub bm0: [libc::c_uint; 128],
    pub bm1: [libc::c_uint; 128],
    pub bm2: [libc::c_uint; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirCacheEntry_t {
    pub type_0: dirCacheEntryType_t,
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
    pub shortName: *mut wchar_t,
    pub longName: *mut wchar_t,
    pub dir: directory,
    pub endMarkPos: libc::c_int,
}
pub type dirCacheEntryType_t = libc::c_uint;
pub const DCET_END: dirCacheEntryType_t = 2;
pub const DCET_USED: dirCacheEntryType_t = 1;
pub const DCET_FREE: dirCacheEntryType_t = 0;
#[inline]
unsafe extern "C" fn rol(mut arg: uint32_t, mut shift: libc::c_int) -> uint32_t {
    arg &= 0xffffffff as libc::c_uint;
    return arg << shift | arg >> 32 as libc::c_int - shift;
}
unsafe extern "C" fn calcHash(mut name: *mut wchar_t) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    let mut c: wint_t = 0;
    hash = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as libc::c_uint;
    while *name != 0 {
        hash = rol(hash, 5 as libc::c_int);
        c = towupper(*name as wint_t);
        hash
            ^= c.wrapping_mul(c.wrapping_add(2 as libc::c_int as libc::c_uint))
                ^ i.wrapping_mul(i.wrapping_add(2 as libc::c_int as libc::c_uint));
        hash &= 0xffffffff as libc::c_uint;
        i = i.wrapping_add(1);
        i;
        name = name.offset(1);
        name;
    }
    hash = hash.wrapping_mul(hash.wrapping_add(2 as libc::c_int as libc::c_uint));
    hash ^= (hash & 0xfff as libc::c_int as libc::c_uint) << 12 as libc::c_int;
    hash ^= (hash & 0xff000 as libc::c_int as libc::c_uint) << 24 as libc::c_int;
    return hash;
}
unsafe extern "C" fn addBit(
    mut bitmap: *mut libc::c_uint,
    mut hash: libc::c_uint,
    mut checkOnly: libc::c_int,
) -> libc::c_uint {
    let mut bit: libc::c_uint = 0;
    let mut entry: libc::c_int = 0;
    bit = (1 as libc::c_uint)
        << (hash as libc::c_ulong)
            .wrapping_rem(
                (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            );
    entry = (hash as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_rem(128 as libc::c_int as libc::c_ulong) as libc::c_int;
    if checkOnly != 0 {
        return *bitmap.offset(entry as isize) & bit
    } else {
        *bitmap.offset(entry as isize) |= bit;
        return 1 as libc::c_int as libc::c_uint;
    };
}
unsafe extern "C" fn _addHash(
    mut cache: *mut dirCache_t,
    mut hash: libc::c_uint,
    mut checkOnly: libc::c_int,
) -> libc::c_int {
    return (addBit(((*cache).bm0).as_mut_ptr(), hash, checkOnly) != 0
        && addBit(((*cache).bm1).as_mut_ptr(), rol(hash, 12 as libc::c_int), checkOnly)
            != 0
        && addBit(((*cache).bm2).as_mut_ptr(), rol(hash, 24 as libc::c_int), checkOnly)
            != 0) as libc::c_int;
}
unsafe extern "C" fn addNameToHash(mut cache: *mut dirCache_t, mut name: *mut wchar_t) {
    _addHash(cache, calcHash(name), 0 as libc::c_int);
}
unsafe extern "C" fn hashDce(mut cache: *mut dirCache_t, mut dce: *mut dirCacheEntry_t) {
    if (*dce).beginSlot != (*cache).nrHashed {
        return;
    }
    (*cache).nrHashed = (*dce).endSlot;
    if !((*dce).longName).is_null() {
        addNameToHash(cache, (*dce).longName);
    }
    addNameToHash(cache, (*dce).shortName);
}
pub unsafe extern "C" fn isHashed(
    mut cache: *mut dirCache_t,
    mut name: *mut wchar_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = _addHash(cache, calcHash(name), 1 as libc::c_int);
    return ret;
}
pub unsafe extern "C" fn growDirCache(
    mut cache: *mut dirCache_t,
    mut slot: libc::c_uint,
) -> libc::c_int {
    if (slot as libc::c_int) < 0 as libc::c_int {
        fprintf(stderr, b"Bad slot %d\n\0" as *const u8 as *const libc::c_char, slot);
        exit(1 as libc::c_int);
    }
    if (*cache).nr_entries <= slot {
        let mut i: libc::c_uint = 0;
        (*cache)
            .entries = realloc(
            (*cache).entries as *mut libc::c_void,
            (slot
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut dirCacheEntry_t>() as libc::c_ulong,
                ),
        ) as *mut *mut dirCacheEntry_t;
        if ((*cache).entries).is_null() {
            return -(1 as libc::c_int);
        }
        i = (*cache).nr_entries;
        while i
            < slot
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
        {
            let ref mut fresh0 = *((*cache).entries).offset(i as isize);
            *fresh0 = 0 as *mut dirCacheEntry_t;
            i = i.wrapping_add(1);
            i;
        }
        (*cache)
            .nr_entries = slot
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn allocDirCache(
    mut Stream: *mut Stream_t,
    mut slot: libc::c_uint,
) -> *mut dirCache_t {
    let mut dcp: *mut *mut dirCache_t = 0 as *mut *mut dirCache_t;
    if (slot as libc::c_int) < 0 as libc::c_int {
        fprintf(stderr, b"Bad slot %d\n\0" as *const u8 as *const libc::c_char, slot);
        exit(1 as libc::c_int);
    }
    dcp = getDirCacheP(Stream);
    if (*dcp).is_null() {
        *dcp = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<dirCache_t>() as libc::c_ulong,
        ) as *mut dirCache_t;
        if (*dcp).is_null() {
            return 0 as *mut dirCache_t;
        }
        (**dcp)
            .entries = calloc(
            slot
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(5 as libc::c_int as libc::c_uint) as libc::c_ulong,
            ::std::mem::size_of::<*mut dirCacheEntry_t>() as libc::c_ulong,
        ) as *mut *mut dirCacheEntry_t;
        if ((**dcp).entries).is_null() {
            free(*dcp as *mut libc::c_void);
            return 0 as *mut dirCache_t;
        }
        (**dcp)
            .nr_entries = slot
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint);
        memset(
            ((**dcp).bm0).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_uint; 128]>() as libc::c_ulong,
        );
        memset(
            ((**dcp).bm1).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_uint; 128]>() as libc::c_ulong,
        );
        memset(
            ((**dcp).bm2).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_uint; 128]>() as libc::c_ulong,
        );
        (**dcp).nrHashed = 0 as libc::c_int as libc::c_uint;
    } else if growDirCache(*dcp, slot) < 0 as libc::c_int {
        return 0 as *mut dirCache_t
    }
    return *dcp;
}
unsafe extern "C" fn freeDirCacheRange(
    mut cache: *mut dirCache_t,
    mut beginSlot: libc::c_uint,
    mut endSlot: libc::c_uint,
) -> libc::c_int {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut clearBegin: libc::c_uint = 0;
    let mut clearEnd: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    if endSlot < beginSlot {
        fprintf(
            stderr,
            b"Bad slots %d %d in free range\n\0" as *const u8 as *const libc::c_char,
            beginSlot,
            endSlot,
        );
        exit(1 as libc::c_int);
    }
    while beginSlot < endSlot {
        entry = *((*cache).entries).offset(beginSlot as isize);
        if entry.is_null() {
            beginSlot = beginSlot.wrapping_add(1);
            beginSlot;
        } else {
            if (*entry).beginSlot == beginSlot {} else {
                __assert_fail(
                    b"entry->beginSlot == beginSlot\0" as *const u8
                        as *const libc::c_char,
                    b"dirCache.c\0" as *const u8 as *const libc::c_char,
                    197 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 64],
                        &[libc::c_char; 64],
                    >(
                        b"int freeDirCacheRange(dirCache_t *, unsigned int, unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7996: {
                if (*entry).beginSlot == beginSlot {} else {
                    __assert_fail(
                        b"entry->beginSlot == beginSlot\0" as *const u8
                            as *const libc::c_char,
                        b"dirCache.c\0" as *const u8 as *const libc::c_char,
                        197 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 64],
                            &[libc::c_char; 64],
                        >(
                            b"int freeDirCacheRange(dirCache_t *, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            clearEnd = (*entry).endSlot;
            if clearEnd > endSlot {
                clearEnd = endSlot;
            }
            clearBegin = beginSlot;
            i = clearBegin;
            while i < clearEnd {
                let ref mut fresh1 = *((*cache).entries).offset(i as isize);
                *fresh1 = 0 as *mut dirCacheEntry_t;
                i = i.wrapping_add(1);
                i;
            }
            (*entry).beginSlot = clearEnd;
            if (*entry).beginSlot == (*entry).endSlot {
                let mut needWriteEnd: libc::c_int = 0 as libc::c_int;
                if (*entry).endMarkPos != -(1 as libc::c_int)
                    && (*entry).endMarkPos < beginSlot as libc::c_int
                {
                    needWriteEnd = 1 as libc::c_int;
                }
                if !((*entry).longName).is_null() {
                    free((*entry).longName as *mut libc::c_void);
                }
                if !((*entry).shortName).is_null() {
                    free((*entry).shortName as *mut libc::c_void);
                }
                free(entry as *mut libc::c_void);
                if needWriteEnd != 0 {
                    return beginSlot as libc::c_int;
                }
            }
            beginSlot = clearEnd;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn allocDirCacheEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: libc::c_uint,
    mut endSlot: libc::c_uint,
    mut type_0: dirCacheEntryType_t,
) -> *mut dirCacheEntry_t {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut i: libc::c_uint = 0;
    if growDirCache(cache, endSlot) < 0 as libc::c_int {
        return 0 as *mut dirCacheEntry_t;
    }
    entry = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<dirCacheEntry_t>() as libc::c_ulong,
    ) as *mut dirCacheEntry_t;
    if entry.is_null() {
        return 0 as *mut dirCacheEntry_t;
    }
    (*entry).type_0 = type_0;
    (*entry).longName = 0 as *mut wchar_t;
    (*entry).shortName = 0 as *mut wchar_t;
    (*entry).beginSlot = beginSlot;
    (*entry).endSlot = endSlot;
    (*entry).endMarkPos = -(1 as libc::c_int);
    freeDirCacheRange(cache, beginSlot, endSlot);
    i = beginSlot;
    while i < endSlot {
        let ref mut fresh2 = *((*cache).entries).offset(i as isize);
        *fresh2 = entry;
        i = i.wrapping_add(1);
        i;
    }
    return entry;
}
pub unsafe extern "C" fn addUsedEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: libc::c_uint,
    mut endSlot: libc::c_uint,
    mut longName: *mut wchar_t,
    mut shortName: *mut wchar_t,
    mut dir: *mut directory,
) -> *mut dirCacheEntry_t {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    if endSlot < beginSlot {
        fprintf(
            stderr,
            b"Bad slots %d %d in add used entry\n\0" as *const u8 as *const libc::c_char,
            beginSlot,
            endSlot,
        );
        exit(1 as libc::c_int);
    }
    entry = allocDirCacheEntry(cache, beginSlot, endSlot, DCET_USED);
    if entry.is_null() {
        return 0 as *mut dirCacheEntry_t;
    }
    (*entry).beginSlot = beginSlot;
    (*entry).endSlot = endSlot;
    if !longName.is_null() {
        (*entry).longName = wcsdup(longName);
    }
    (*entry).shortName = wcsdup(shortName);
    (*entry).dir = *dir;
    hashDce(cache, entry);
    return entry;
}
unsafe extern "C" fn mergeFreeSlots(mut cache: *mut dirCache_t, mut slot: libc::c_uint) {
    let mut previous: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut next: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut i: libc::c_uint = 0;
    if slot == 0 as libc::c_int as libc::c_uint {
        return;
    }
    previous = *((*cache).entries)
        .offset(slot.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
    next = *((*cache).entries).offset(slot as isize);
    if !next.is_null()
        && (*next).type_0 as libc::c_uint == DCET_FREE as libc::c_int as libc::c_uint
        && !previous.is_null()
        && (*previous).type_0 as libc::c_uint == DCET_FREE as libc::c_int as libc::c_uint
    {
        i = (*next).beginSlot;
        while i < (*next).endSlot {
            let ref mut fresh3 = *((*cache).entries).offset(i as isize);
            *fresh3 = previous;
            i = i.wrapping_add(1);
            i;
        }
        (*previous).endSlot = (*next).endSlot;
        (*previous).endMarkPos = (*next).endMarkPos;
        free(next as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn addFreeEndEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: libc::c_uint,
    mut endSlot: libc::c_uint,
    mut isAtEnd: libc::c_int,
) -> *mut dirCacheEntry_t {
    let mut entry: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    if beginSlot < (*cache).nrHashed {
        (*cache).nrHashed = beginSlot;
    }
    if endSlot < beginSlot {
        fprintf(
            stderr,
            b"Bad slots %d %d in add free entry\n\0" as *const u8 as *const libc::c_char,
            beginSlot,
            endSlot,
        );
        exit(1 as libc::c_int);
    }
    if endSlot == beginSlot {
        return 0 as *mut dirCacheEntry_t;
    }
    entry = allocDirCacheEntry(cache, beginSlot, endSlot, DCET_FREE);
    if isAtEnd != 0 {
        (*entry).endMarkPos = beginSlot as libc::c_int;
    }
    mergeFreeSlots(cache, beginSlot);
    mergeFreeSlots(cache, endSlot);
    return *((*cache).entries).offset(beginSlot as isize);
}
pub unsafe extern "C" fn addFreeEntry(
    mut cache: *mut dirCache_t,
    mut beginSlot: libc::c_uint,
    mut endSlot: libc::c_uint,
) -> *mut dirCacheEntry_t {
    return addFreeEndEntry(cache, beginSlot, endSlot, 0 as libc::c_int);
}
pub unsafe extern "C" fn addEndEntry(
    mut cache: *mut dirCache_t,
    mut pos: libc::c_uint,
) -> *mut dirCacheEntry_t {
    return allocDirCacheEntry(cache, pos, pos.wrapping_add(1 as libc::c_uint), DCET_END);
}
pub unsafe extern "C" fn lookupInDircache(
    mut cache: *mut dirCache_t,
    mut pos: libc::c_uint,
) -> *mut dirCacheEntry_t {
    if growDirCache(cache, pos.wrapping_add(1 as libc::c_int as libc::c_uint))
        < 0 as libc::c_int
    {
        return 0 as *mut dirCacheEntry_t;
    }
    return *((*cache).entries).offset(pos as isize);
}
pub unsafe extern "C" fn freeDirCache(mut Stream: *mut Stream_t) {
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut dcp: *mut *mut dirCache_t = 0 as *mut *mut dirCache_t;
    dcp = getDirCacheP(Stream);
    cache = *dcp;
    if !cache.is_null() {
        let mut n: libc::c_int = 0;
        n = freeDirCacheRange(
            cache,
            0 as libc::c_int as libc::c_uint,
            (*cache).nr_entries,
        );
        if n >= 0 as libc::c_int {
            low_level_dir_write_end(Stream, n);
        }
        free(cache as *mut libc::c_void);
        *dcp = 0 as *mut dirCache_t;
    }
}
