use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    pub type dirCacheEntry_t;
    pub type FatMap_t;
    pub type hashtable;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn tzset();
    static mut timezone: libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn mk_entry_from_base(
        base: *const libc::c_char,
        attr: libc::c_uchar,
        fat: libc::c_uint,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn dir_grow(Dir: *mut Stream_t, size: libc::c_uint) -> libc::c_int;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    static mut batchmode: libc::c_int;
    fn getStart(Dir: *mut Stream_t, dir: *mut directory) -> libc::c_uint;
    fn truncMtOffTo32u(off: mt_off_t) -> uint32_t;
    fn truncSizeTo32u(siz: size_t) -> uint32_t;
    fn fatAppend(This: *mut Fs_t, pos: libc::c_uint, newpos: libc::c_uint);
    fn get_next_free_cluster(Fs: *mut Fs_t, last: libc::c_uint) -> libc::c_uint;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
    fn fatDecode(This: *mut Fs_t, pos: libc::c_uint) -> libc::c_uint;
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
    fn fatAllocate(This: *mut Fs_t, pos: libc::c_uint, value: libc::c_uint);
    fn fsPreallocateClusters(Fs: *mut Fs_t, _: uint32_t) -> libc::c_int;
    fn fsReleasePreallocateClusters(Fs: *mut Fs_t, _: uint32_t);
    fn dir_write(entry: *mut direntry_t);
    fn isRootEntry(entry: *mut direntry_t) -> libc::c_int;
    fn make_ht(
        f1: T_HashFunc,
        f2: T_HashFunc,
        c: T_ComparFunc,
        size: size_t,
        H: *mut *mut T_HashTable,
    ) -> libc::c_int;
    fn hash_add(
        H: *mut T_HashTable,
        E: *mut libc::c_void,
        hint: *mut size_t,
    ) -> libc::c_int;
    fn hash_remove(
        H: *mut T_HashTable,
        E: *mut libc::c_void,
        hint: size_t,
    ) -> libc::c_int;
    fn hash_lookup(
        H: *mut T_HashTable,
        E: *mut libc::c_void,
        E2: *mut *mut libc::c_void,
        hint: *mut size_t,
    ) -> libc::c_int;
    fn freeDirCache(Stream: *mut Stream_t);
    fn buf_init(
        Next: *mut Stream_t,
        size: size_t,
        cylinderSize: size_t,
        sectorSize: size_t,
    ) -> *mut Stream_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
pub struct File_t {
    pub head: Stream_t,
    pub Buffer: *mut Stream_t,
    pub map: Option::<
        unsafe extern "C" fn(
            *mut File_t,
            uint32_t,
            *mut uint32_t,
            libc::c_int,
            *mut mt_off_t,
        ) -> libc::c_int,
    >,
    pub FileSize: uint32_t,
    pub preallocatedSize: uint32_t,
    pub preallocatedClusters: uint32_t,
    pub FirstAbsCluNr: libc::c_uint,
    pub PreviousAbsCluNr: libc::c_uint,
    pub PreviousRelCluNr: libc::c_uint,
    pub direntry: direntry_t,
    pub hint: size_t,
    pub dcp: *mut dirCache_t,
    pub loopDetectRel: libc::c_uint,
    pub loopDetectAbs: libc::c_uint,
    pub where_0: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: libc::c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fs_t {
    pub head: Stream_t,
    pub serialized: libc::c_int,
    pub serial_number: libc::c_ulong,
    pub cluster_size: uint8_t,
    pub sector_size: uint16_t,
    pub fat_error: libc::c_int,
    pub fat_decode: Option::<
        unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
    >,
    pub fat_encode: Option::<
        unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
    >,
    pub fat_dirty: libc::c_int,
    pub fat_start: uint16_t,
    pub fat_len: uint32_t,
    pub num_fat: uint8_t,
    pub end_fat: uint32_t,
    pub last_fat: uint32_t,
    pub fat_bits: libc::c_uint,
    pub FatMap: *mut FatMap_t,
    pub dir_start: uint32_t,
    pub dir_len: uint16_t,
    pub clus_start: uint32_t,
    pub num_clus: uint32_t,
    pub drive: libc::c_char,
    pub primaryFat: uint32_t,
    pub writeAllFats: uint32_t,
    pub rootCluster: uint32_t,
    pub infoSectorLoc: uint32_t,
    pub backupBoot: uint16_t,
    pub last: uint32_t,
    pub freeSpace: uint32_t,
    pub preallocatedClusters: libc::c_uint,
    pub lastFatSectorNr: uint32_t,
    pub lastFatSectorData: *mut libc::c_uchar,
    pub lastFatAccessMode: fatAccessMode_t,
    pub sectorMask: libc::c_uint,
    pub sectorShift: libc::c_uint,
    pub cp: *mut doscp_t,
}
pub type fatAccessMode_t = libc::c_uint;
pub const FAT_ACCESS_WRITE: fatAccessMode_t = 1;
pub const FAT_ACCESS_READ: fatAccessMode_t = 0;
pub type T_HashTable = hashtable;
pub type T_ComparFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
pub type T_HashFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> uint32_t>;
#[inline]
unsafe extern "C" fn set_word(mut data: *mut libc::c_uchar, mut value: libc::c_ushort) {
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (value as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (value as libc::c_int >> 0 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
}
static mut filehash: *mut T_HashTable = 0 as *const T_HashTable as *mut T_HashTable;
unsafe extern "C" fn getUnbufferedFile(mut Stream: *mut Stream_t) -> *mut File_t {
    while (*Stream).Class != &mut FileClass as *mut Class_t {
        Stream = (*Stream).Next;
    }
    return Stream as *mut File_t;
}
#[inline]
unsafe extern "C" fn _getFs(mut File: *mut File_t) -> *mut Fs_t {
    return (*File).head.Next as *mut Fs_t;
}
pub unsafe extern "C" fn getFs(mut Stream: *mut Stream_t) -> *mut Fs_t {
    return (*getUnbufferedFile(Stream)).head.Next as *mut Fs_t;
}
pub unsafe extern "C" fn getDirCacheP(
    mut Stream: *mut Stream_t,
) -> *mut *mut dirCache_t {
    return &mut (*(getUnbufferedFile
        as unsafe extern "C" fn(*mut Stream_t) -> *mut File_t)(Stream))
        .dcp;
}
pub unsafe extern "C" fn getDirentry(mut Stream: *mut Stream_t) -> *mut direntry_t {
    return &mut (*(getUnbufferedFile
        as unsafe extern "C" fn(*mut Stream_t) -> *mut File_t)(Stream))
        .direntry;
}
unsafe extern "C" fn filebytesToClusters(
    mut bytes: uint32_t,
    mut clus_size: uint32_t,
) -> uint32_t {
    let mut ret: uint32_t = bytes.wrapping_div(clus_size);
    if bytes.wrapping_rem(clus_size) != 0 {
        ret = ret.wrapping_add(1);
        ret;
    }
    return ret;
}
unsafe extern "C" fn recalcPreallocSize(mut This: *mut File_t) -> libc::c_int {
    let mut currentClusters: uint32_t = 0;
    let mut neededClusters: uint32_t = 0;
    let mut clus_size: libc::c_uint = 0;
    let mut neededPrealloc: uint32_t = 0;
    let mut Fs: *mut Fs_t = _getFs(This);
    clus_size = ((*Fs).cluster_size as libc::c_int * (*Fs).sector_size as libc::c_int)
        as libc::c_uint;
    currentClusters = filebytesToClusters((*This).FileSize, clus_size);
    neededClusters = filebytesToClusters((*This).preallocatedSize, clus_size);
    if neededClusters < currentClusters {
        neededPrealloc = 0 as libc::c_int as uint32_t;
    } else {
        neededPrealloc = neededClusters.wrapping_sub(currentClusters);
    }
    if neededPrealloc > (*This).preallocatedClusters {
        let mut r: libc::c_int = fsPreallocateClusters(
            Fs,
            neededPrealloc.wrapping_sub((*This).preallocatedClusters),
        );
        if r != 0 {
            return r;
        }
    } else {
        fsReleasePreallocateClusters(
            Fs,
            ((*This).preallocatedClusters).wrapping_sub(neededPrealloc),
        );
    }
    (*This).preallocatedClusters = neededPrealloc;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _loopDetect(
    mut oldrel: *mut libc::c_uint,
    mut rel: libc::c_uint,
    mut oldabs: *mut libc::c_uint,
    mut absol: libc::c_uint,
) -> libc::c_int {
    if *oldrel != 0 && rel > *oldrel && absol == *oldabs {
        fprintf(
            stderr,
            b"loop detected! oldrel=%d newrel=%d abs=%d\n\0" as *const u8
                as *const libc::c_char,
            *oldrel,
            rel,
            absol,
        );
        return -(1 as libc::c_int);
    }
    if rel
        >= (2 as libc::c_int as libc::c_uint)
            .wrapping_mul(*oldrel)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        *oldrel = rel;
        *oldabs = absol;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn loopDetect(
    mut This: *mut File_t,
    mut rel: libc::c_uint,
    mut absol: libc::c_uint,
) -> libc::c_int {
    return _loopDetect(
        &mut (*This).loopDetectRel,
        rel,
        &mut (*This).loopDetectAbs,
        absol,
    );
}
unsafe extern "C" fn _countBlocks(
    mut This: *mut Fs_t,
    mut block: libc::c_uint,
) -> libc::c_uint {
    let mut blocks: libc::c_uint = 0;
    let mut rel: libc::c_uint = 0;
    let mut oldabs: libc::c_uint = 0;
    let mut oldrel: libc::c_uint = 0;
    blocks = 0 as libc::c_int as libc::c_uint;
    rel = 0 as libc::c_int as libc::c_uint;
    oldrel = rel;
    oldabs = oldrel;
    while block <= (*This).last_fat && block != 1 as libc::c_int as libc::c_uint
        && block != 0
    {
        blocks = blocks.wrapping_add(1);
        blocks;
        block = fatDecode(This, block);
        rel = rel.wrapping_add(1);
        rel;
        if _loopDetect(&mut oldrel, rel, &mut oldabs, block) < 0 as libc::c_int {
            block = 1 as libc::c_int as libc::c_uint;
        }
    }
    return blocks;
}
pub unsafe extern "C" fn countBlocks(
    mut Dir: *mut Stream_t,
    mut block: libc::c_uint,
) -> libc::c_uint {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    return _countBlocks(This, block);
}
unsafe extern "C" fn countBytes(
    mut Dir: *mut Stream_t,
    mut block: libc::c_uint,
) -> uint32_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    return (_countBlocks(This, block))
        .wrapping_mul((*This).sector_size as libc::c_uint)
        .wrapping_mul((*This).cluster_size as libc::c_uint);
}
pub unsafe extern "C" fn printFat(mut Stream: *mut Stream_t) {
    let mut This: *mut File_t = getUnbufferedFile(Stream);
    let mut n: uint32_t = 0;
    let mut rel: libc::c_uint = 0;
    let mut begin: libc::c_ulong = 0;
    let mut end: libc::c_ulong = 0;
    let mut first: libc::c_int = 0;
    n = (*This).FirstAbsCluNr;
    if n == 0 {
        printf(b"Root directory or empty file\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    rel = 0 as libc::c_int as libc::c_uint;
    first = 1 as libc::c_int;
    end = 0 as libc::c_int as libc::c_ulong;
    begin = end;
    loop {
        if first != 0
            || n as libc::c_ulong != end.wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            if first == 0 {
                if begin != end {
                    printf(b"-%lu\0" as *const u8 as *const libc::c_char, end);
                }
                printf(b"> \0" as *const u8 as *const libc::c_char);
            }
            end = n as libc::c_ulong;
            begin = end;
            printf(b"<%lu\0" as *const u8 as *const libc::c_char, begin);
        } else {
            end = end.wrapping_add(1);
            end;
        }
        first = 0 as libc::c_int;
        n = fatDecode(_getFs(This), n);
        rel = rel.wrapping_add(1);
        rel;
        if loopDetect(This, rel, n) < 0 as libc::c_int {
            n = 1 as libc::c_int as uint32_t;
        }
        if !(n <= (*_getFs(This)).last_fat && n != 1 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if first == 0 {
        if begin != end {
            printf(b"-%lu\0" as *const u8 as *const libc::c_char, end);
        }
        printf(b">\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn printFatWithOffset(
    mut Stream: *mut Stream_t,
    mut offset: off_t,
) {
    let mut This: *mut File_t = getUnbufferedFile(Stream);
    let mut n: uint32_t = 0;
    let mut rel: libc::c_uint = 0;
    let mut clusSize: off_t = 0;
    n = (*This).FirstAbsCluNr;
    if n == 0 {
        printf(b"Root directory or empty file\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    clusSize = ((*_getFs(This)).cluster_size as libc::c_int
        * (*_getFs(This)).sector_size as libc::c_int) as off_t;
    rel = 0 as libc::c_int as libc::c_uint;
    while offset >= clusSize {
        n = fatDecode(_getFs(This), n);
        rel = rel.wrapping_add(1);
        rel;
        if loopDetect(This, rel, n) < 0 as libc::c_int {
            return;
        }
        if n > (*_getFs(This)).last_fat {
            return;
        }
        offset -= clusSize;
    }
    printf(b"%lu\0" as *const u8 as *const libc::c_char, n as libc::c_ulong);
}
unsafe extern "C" fn normal_map(
    mut This: *mut File_t,
    mut where_0: uint32_t,
    mut len: *mut uint32_t,
    mut isReadonly: libc::c_int,
    mut res: *mut mt_off_t,
) -> libc::c_int {
    let mut offset: libc::c_uint = 0;
    let mut end: size_t = 0;
    let mut NrClu: uint32_t = 0;
    let mut RelCluNr: uint32_t = 0;
    let mut CurCluNr: uint32_t = 0;
    let mut NewCluNr: uint32_t = 0;
    let mut AbsCluNr: uint32_t = 0;
    let mut clus_size: uint32_t = 0;
    let mut Fs: *mut Fs_t = _getFs(This);
    *res = 0 as libc::c_int as mt_off_t;
    clus_size = ((*Fs).cluster_size as libc::c_int * (*Fs).sector_size as libc::c_int)
        as uint32_t;
    offset = where_0.wrapping_rem(clus_size);
    if isReadonly != 0 {
        if *len > ((*This).FileSize).wrapping_sub(where_0) {
            *len = ((*This).FileSize).wrapping_sub(where_0);
        }
    }
    if *len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*This).FirstAbsCluNr < 2 as libc::c_int as libc::c_uint {
        if isReadonly != 0 || *len == 0 as libc::c_int as libc::c_uint {
            *len = 0 as libc::c_int as uint32_t;
            return 0 as libc::c_int;
        }
        NewCluNr = get_next_free_cluster(_getFs(This), 1 as libc::c_int as libc::c_uint);
        if NewCluNr == 1 as libc::c_int as libc::c_uint {
            *__errno_location() = 28 as libc::c_int;
            return -(2 as libc::c_int);
        }
        hash_remove(filehash, This as *mut libc::c_void, (*This).hint);
        (*This).FirstAbsCluNr = NewCluNr;
        hash_add(filehash, This as *mut libc::c_void, &mut (*This).hint);
        fatAllocate(_getFs(This), NewCluNr, (*Fs).end_fat);
    }
    RelCluNr = where_0.wrapping_div(clus_size);
    if RelCluNr >= (*This).PreviousRelCluNr {
        CurCluNr = (*This).PreviousRelCluNr;
        AbsCluNr = (*This).PreviousAbsCluNr;
    } else {
        CurCluNr = 0 as libc::c_int as uint32_t;
        AbsCluNr = (*This).FirstAbsCluNr;
    }
    NrClu = offset
        .wrapping_add(*len)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(clus_size);
    while CurCluNr <= RelCluNr.wrapping_add(NrClu) {
        if CurCluNr == RelCluNr {
            (*This).PreviousRelCluNr = RelCluNr;
            (*This).PreviousAbsCluNr = AbsCluNr;
        }
        NewCluNr = fatDecode(_getFs(This), AbsCluNr);
        if NewCluNr == 1 as libc::c_int as libc::c_uint
            || NewCluNr == 0 as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"Fat problem while decoding %d %x\n\0" as *const u8
                    as *const libc::c_char,
                AbsCluNr,
                NewCluNr,
            );
            exit(1 as libc::c_int);
        }
        if CurCluNr == RelCluNr.wrapping_add(NrClu) {
            break;
        }
        if NewCluNr > (*Fs).last_fat && isReadonly == 0 {
            NewCluNr = get_next_free_cluster(_getFs(This), AbsCluNr);
            if NewCluNr == 1 as libc::c_int as libc::c_uint {
                *__errno_location() = 28 as libc::c_int;
                return -(2 as libc::c_int);
            }
            fatAppend(_getFs(This), AbsCluNr, NewCluNr);
        }
        if CurCluNr < RelCluNr && NewCluNr > (*Fs).last_fat {
            *len = 0 as libc::c_int as uint32_t;
            return 0 as libc::c_int;
        }
        if CurCluNr >= RelCluNr
            && NewCluNr != AbsCluNr.wrapping_add(1 as libc::c_int as libc::c_uint)
        {
            break;
        }
        CurCluNr = CurCluNr.wrapping_add(1);
        CurCluNr;
        AbsCluNr = NewCluNr;
        if loopDetect(This, CurCluNr, AbsCluNr) != 0 {
            *__errno_location() = 5 as libc::c_int;
            return -(2 as libc::c_int);
        }
    }
    if *len
        > (1 as libc::c_int as libc::c_uint)
            .wrapping_add(CurCluNr)
            .wrapping_sub(RelCluNr)
            .wrapping_mul(clus_size)
            .wrapping_sub(offset)
    {
        *len = (1 as libc::c_int as libc::c_uint)
            .wrapping_add(CurCluNr)
            .wrapping_sub(RelCluNr)
            .wrapping_mul(clus_size)
            .wrapping_sub(offset);
    }
    end = where_0.wrapping_add(*len) as size_t;
    if batchmode != 0 && isReadonly == 0 && end >= (*This).FileSize as libc::c_ulong {
        *len = (*len as libc::c_ulong)
            .wrapping_add(
                end
                    .wrapping_add(clus_size as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(
                        end
                            .wrapping_add(clus_size as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            .wrapping_rem(clus_size as libc::c_ulong),
                    )
                    .wrapping_sub(end),
            ) as uint32_t as uint32_t;
    }
    if (*len)
        .wrapping_add(offset)
        .wrapping_div(clus_size)
        .wrapping_add((*This).PreviousAbsCluNr)
        .wrapping_sub(2 as libc::c_int as libc::c_uint) > (*Fs).num_clus
    {
        fprintf(stderr, b"cluster too big\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    *res = sectorsToBytes(
        Fs,
        ((*This).PreviousAbsCluNr)
            .wrapping_sub(2 as libc::c_int as libc::c_uint)
            .wrapping_mul((*Fs).cluster_size as libc::c_uint)
            .wrapping_add((*Fs).clus_start),
    ) + offset as libc::c_long;
    return 1 as libc::c_int;
}
unsafe extern "C" fn root_map(
    mut This: *mut File_t,
    mut where_0: uint32_t,
    mut len: *mut uint32_t,
    mut mode: libc::c_int,
    mut res: *mut mt_off_t,
) -> libc::c_int {
    let mut Fs: *mut Fs_t = _getFs(This);
    if ((*Fs).dir_len as libc::c_uint)
        .wrapping_add(0 as libc::c_uint)
        .wrapping_mul((*Fs).sector_size as libc::c_uint) < where_0
    {
        *len = 0 as libc::c_int as uint32_t;
        *__errno_location() = 28 as libc::c_int;
        return -(2 as libc::c_int);
    }
    if *len
        > (((*Fs).dir_len as libc::c_int * (*Fs).sector_size as libc::c_int)
            as libc::c_uint)
            .wrapping_sub(where_0)
    {
        *len = (((*Fs).dir_len as libc::c_int * (*Fs).sector_size as libc::c_int)
            as libc::c_uint)
            .wrapping_sub(where_0);
    }
    if *len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    *res = sectorsToBytes(Fs, (*Fs).dir_start) + where_0 as libc::c_long;
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut pos: mt_off_t = 0;
    let mut err: libc::c_int = 0;
    let mut len: uint32_t = truncSizeTo32u(ilen);
    let mut ret: ssize_t = 0;
    let mut Disk: *mut Stream_t = (*_getFs(This)).head.Next;
    err = ((*This).map)
        .unwrap()(This, (*This).where_0, &mut len, 1 as libc::c_int, &mut pos);
    if err <= 0 as libc::c_int {
        return err as ssize_t;
    }
    ret = ((*(*Disk).Class).pread).unwrap()(Disk, buf, pos, len as size_t);
    if ret < 0 as libc::c_int as libc::c_long {
        return ret;
    }
    (*This)
        .where_0 = ((*This).where_0 as libc::c_ulong).wrapping_add(ret as size_t)
        as uint32_t as uint32_t;
    return ret;
}
unsafe extern "C" fn write_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut pos: mt_off_t = 0;
    let mut ret: ssize_t = 0;
    let mut requestedLen: uint32_t = 0;
    let mut bytesWritten: uint32_t = 0;
    let mut Disk: *mut Stream_t = (*_getFs(This)).head.Next;
    let mut maxLen: uint32_t = (4294967295 as libc::c_uint)
        .wrapping_sub((*This).where_0);
    let mut len: uint32_t = 0;
    let mut err: libc::c_int = 0;
    if ilen > maxLen as libc::c_ulong {
        len = maxLen;
    } else {
        len = ilen as uint32_t;
    }
    requestedLen = len;
    err = ((*This).map)
        .unwrap()(This, (*This).where_0, &mut len, 0 as libc::c_int, &mut pos);
    if err <= 0 as libc::c_int {
        return err as ssize_t;
    }
    if batchmode != 0 {
        ret = force_pwrite(Disk, buf, pos, len as size_t);
    } else {
        ret = ((*(*Disk).Class).pwrite).unwrap()(Disk, buf, pos, len as size_t);
    }
    if ret < 0 as libc::c_int as libc::c_long {
        return ret;
    }
    if ret as uint32_t > requestedLen {
        bytesWritten = requestedLen;
    } else {
        bytesWritten = ret as uint32_t;
    }
    (*This)
        .where_0 = ((*This).where_0 as libc::c_uint).wrapping_add(bytesWritten)
        as uint32_t as uint32_t;
    if (*This).where_0 > (*This).FileSize {
        (*This).FileSize = (*This).where_0;
    }
    recalcPreallocSize(This);
    return bytesWritten as ssize_t;
}
unsafe extern "C" fn pread_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    (*This).where_0 = truncMtOffTo32u(where_0);
    return read_file(Stream, buf, ilen);
}
unsafe extern "C" fn pwrite_file(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut ilen: size_t,
) -> ssize_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    (*This).where_0 = truncMtOffTo32u(where_0);
    return write_file(Stream, buf, ilen);
}
static mut month: [libc::c_int; 15] = [
    0 as libc::c_int,
    31 as libc::c_int,
    59 as libc::c_int,
    90 as libc::c_int,
    120 as libc::c_int,
    151 as libc::c_int,
    181 as libc::c_int,
    212 as libc::c_int,
    243 as libc::c_int,
    273 as libc::c_int,
    304 as libc::c_int,
    334 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
#[inline]
unsafe extern "C" fn conv_stamp(mut dir: *mut directory) -> time_t {
    let mut tmbuf: *mut tm = 0 as *mut tm;
    let mut tzone: libc::c_long = 0;
    let mut dst: libc::c_long = 0;
    let mut accum: time_t = 0;
    let mut tmp: time_t = 0;
    accum = (((*dir).date[1 as libc::c_int as usize] as libc::c_int >> 1 as libc::c_int)
        + 1980 as libc::c_int - 1970 as libc::c_int) as time_t;
    accum = accum * 365 as libc::c_long
        + month[((((*dir).date[1 as libc::c_int as usize] as libc::c_int
            & 0x1 as libc::c_int) << 3 as libc::c_int)
            + ((*dir).date[0 as libc::c_int as usize] as libc::c_int >> 5 as libc::c_int)
            - 1 as libc::c_int) as usize] as libc::c_long
        + ((*dir).date[0 as libc::c_int as usize] as libc::c_int & 0x1f as libc::c_int)
            as libc::c_long;
    accum
        += (((*dir).date[1 as libc::c_int as usize] as libc::c_int >> 1 as libc::c_int)
            + 1980 as libc::c_int - 1972 as libc::c_int) as libc::c_long
            / 4 as libc::c_long;
    if (((*dir).date[1 as libc::c_int as usize] as libc::c_int >> 1 as libc::c_int)
        + 1980 as libc::c_int) % 4 as libc::c_int == 0
        && (((*dir).date[1 as libc::c_int as usize] as libc::c_int & 0x1 as libc::c_int)
            << 3 as libc::c_int)
            + ((*dir).date[0 as libc::c_int as usize] as libc::c_int >> 5 as libc::c_int)
            < 3 as libc::c_int
    {
        accum -= 1;
        accum;
    }
    accum = accum * 24 as libc::c_long
        + ((*dir).time[1 as libc::c_int as usize] as libc::c_int >> 3 as libc::c_int)
            as libc::c_long;
    accum = accum * 60 as libc::c_long
        + ((((*dir).time[1 as libc::c_int as usize] as libc::c_int & 0x7 as libc::c_int)
            << 3 as libc::c_int)
            + ((*dir).time[0 as libc::c_int as usize] as libc::c_int
                >> 5 as libc::c_int)) as libc::c_long;
    accum = accum * 60 as libc::c_long
        + (((*dir).time[0 as libc::c_int as usize] as libc::c_int & 0x1f as libc::c_int)
            * 2 as libc::c_int) as libc::c_long;
    extern "C" {
        #[link_name = "timezone"]
        static mut timezone_0: libc::c_long;
    }
    tzset();
    tzone = timezone;
    accum += tzone;
    tmp = accum;
    tmbuf = localtime(&mut tmp);
    if !tmbuf.is_null() {
        dst = if (*tmbuf).tm_isdst != 0 {
            -(60 as libc::c_long) * 60 as libc::c_long
        } else {
            0 as libc::c_long
        };
        accum += dst;
    }
    return accum;
}
unsafe extern "C" fn get_file_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut libc::c_int,
    mut address: *mut uint32_t,
) -> libc::c_int {
    let mut This: *mut File_t = Stream as *mut File_t;
    if !date.is_null() {
        *date = conv_stamp(&mut (*This).direntry.dir);
    }
    if !size.is_null() {
        *size = (*This).FileSize as mt_off_t;
    }
    if !type_0.is_null() {
        *type_0 = (*This).direntry.dir.attr as libc::c_int & 0x10 as libc::c_int;
    }
    if !address.is_null() {
        *address = (*This).FirstAbsCluNr;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_file(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut Fs: *mut Fs_t = _getFs(This);
    fsReleasePreallocateClusters(Fs, (*This).preallocatedClusters);
    free_stream(&mut (*This).direntry.Dir);
    freeDirCache(Stream);
    return hash_remove(filehash, Stream as *mut libc::c_void, (*This).hint);
}
unsafe extern "C" fn flush_file(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut entry: *mut direntry_t = &mut (*This).direntry;
    if isRootDir(Stream) != 0 {
        return 0 as libc::c_int;
    }
    if (*This).FirstAbsCluNr != getStart((*entry).Dir, &mut (*entry).dir) {
        set_word(
            ((*entry).dir.start).as_mut_ptr(),
            ((*This).FirstAbsCluNr & 0xffff as libc::c_int as libc::c_uint)
                as libc::c_ushort,
        );
        set_word(
            ((*entry).dir.startHi).as_mut_ptr(),
            ((*This).FirstAbsCluNr >> 16 as libc::c_int) as libc::c_ushort,
        );
        dir_write(entry);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn pre_allocate_file(
    mut Stream: *mut Stream_t,
    mut isize: mt_off_t,
) -> libc::c_int {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut size: uint32_t = truncMtOffTo32u(isize);
    if size > (*This).FileSize && size > (*This).preallocatedSize {
        (*This).preallocatedSize = size;
        return recalcPreallocSize(This);
    } else {
        return 0 as libc::c_int
    };
}
static mut FileClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: Some(
                read_file
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        size_t,
                    ) -> ssize_t,
            ),
            write: Some(
                write_file
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        size_t,
                    ) -> ssize_t,
            ),
            pread: Some(
                pread_file
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                pwrite_file
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(
                flush_file as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            freeFunc: Some(
                free_file as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            set_geom: None,
            get_data: Some(
                get_file_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut libc::c_int,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            pre_allocate: Some(
                pre_allocate_file
                    as unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
            ),
            get_dosConvert: Some(
                get_dosConvert_pass_through
                    as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
unsafe extern "C" fn getAbsCluNr(mut This: *mut File_t) -> libc::c_uint {
    if (*This).FirstAbsCluNr != 0 {
        return (*This).FirstAbsCluNr;
    }
    if isRootDir(This as *mut Stream_t) != 0 {
        return 0 as libc::c_int as libc::c_uint;
    }
    return 1 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn func1(mut Stream: *mut libc::c_void) -> uint32_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    return getAbsCluNr(This) ^ (*This).head.Next as libc::c_ulong as uint32_t;
}
unsafe extern "C" fn func2(mut Stream: *mut libc::c_void) -> uint32_t {
    let mut This: *mut File_t = Stream as *mut File_t;
    return getAbsCluNr(This);
}
unsafe extern "C" fn comp(
    mut Stream: *mut libc::c_void,
    mut Stream2: *mut libc::c_void,
) -> libc::c_int {
    let mut This: *mut File_t = Stream as *mut File_t;
    let mut This2: *mut File_t = Stream2 as *mut File_t;
    return (_getFs(This) != _getFs(This2) || getAbsCluNr(This) != getAbsCluNr(This2))
        as libc::c_int;
}
unsafe extern "C" fn init_hash() {
    static mut is_initialised: libc::c_int = 0 as libc::c_int;
    if is_initialised == 0 {
        make_ht(
            Some(func1 as unsafe extern "C" fn(*mut libc::c_void) -> uint32_t),
            Some(func2 as unsafe extern "C" fn(*mut libc::c_void) -> uint32_t),
            Some(
                comp
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            20 as libc::c_int as size_t,
            &mut filehash,
        );
        is_initialised = 1 as libc::c_int;
    }
}
unsafe extern "C" fn _internalFileOpen(
    mut Dir: *mut Stream_t,
    mut first: libc::c_uint,
    mut size: uint32_t,
    mut entry: *mut direntry_t,
) -> *mut Stream_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut Pattern: File_t = File_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        Buffer: 0 as *mut Stream_t,
        map: None,
        FileSize: 0,
        preallocatedSize: 0,
        preallocatedClusters: 0,
        FirstAbsCluNr: 0,
        PreviousAbsCluNr: 0,
        PreviousRelCluNr: 0,
        direntry: direntry_t {
            Dir: 0 as *mut Stream_t,
            entry: 0,
            dir: directory {
                name: [0; 8],
                ext: [0; 3],
                attr: 0,
                Case: 0,
                ctime_ms: 0,
                ctime: [0; 2],
                cdate: [0; 2],
                adate: [0; 2],
                startHi: [0; 2],
                time: [0; 2],
                date: [0; 2],
                start: [0; 2],
                size: [0; 4],
            },
            name: [0; 256],
            beginSlot: 0,
            endSlot: 0,
        },
        hint: 0,
        dcp: 0 as *mut dirCache_t,
        loopDetectRel: 0,
        loopDetectAbs: 0,
        where_0: 0,
    };
    let mut File: *mut File_t = 0 as *mut File_t;
    init_hash();
    (*This).head.refs += 1;
    (*This).head.refs;
    if first != 1 as libc::c_int as libc::c_uint {
        let mut Result: *mut libc::c_void = 0 as *mut libc::c_void;
        init_head(&mut Pattern.head, &mut FileClass, &mut (*This).head);
        if first != 0
            || !entry.is_null()
                && (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int == 0
        {
            Pattern
                .map = Some(
                normal_map
                    as unsafe extern "C" fn(
                        *mut File_t,
                        uint32_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut mt_off_t,
                    ) -> libc::c_int,
            );
        } else {
            Pattern
                .map = Some(
                root_map
                    as unsafe extern "C" fn(
                        *mut File_t,
                        uint32_t,
                        *mut uint32_t,
                        libc::c_int,
                        *mut mt_off_t,
                    ) -> libc::c_int,
            );
        }
        Pattern.FirstAbsCluNr = first;
        Pattern.loopDetectRel = 0 as libc::c_int as libc::c_uint;
        Pattern.loopDetectAbs = first;
        if hash_lookup(
            filehash,
            &mut Pattern as *mut File_t as *mut libc::c_void,
            &mut Result,
            0 as *mut size_t,
        ) == 0
        {
            File = Result as *mut File_t;
            (*File).head.refs += 1;
            (*File).head.refs;
            (*This).head.refs -= 1;
            (*This).head.refs;
            return File as *mut Stream_t;
        }
    }
    File = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<File_t>() as libc::c_ulong,
    ) as *mut File_t;
    if File.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*File).head, &mut FileClass, &mut (*This).head);
    (*File).Buffer = 0 as *mut Stream_t;
    (*File).dcp = 0 as *mut dirCache_t;
    (*File).preallocatedClusters = 0 as libc::c_int as uint32_t;
    (*File).preallocatedSize = 0 as libc::c_int as uint32_t;
    (*File).direntry = *entry;
    if isRootEntry(entry) != 0 {
        (*File).direntry.Dir = File as *mut Stream_t;
    } else {
        copy_stream((*File).direntry.Dir);
    }
    (*File).where_0 = 0 as libc::c_int as uint32_t;
    if first != 0
        || !entry.is_null()
            && (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int == 0
    {
        (*File)
            .map = Some(
            normal_map
                as unsafe extern "C" fn(
                    *mut File_t,
                    uint32_t,
                    *mut uint32_t,
                    libc::c_int,
                    *mut mt_off_t,
                ) -> libc::c_int,
        );
    } else {
        (*File)
            .map = Some(
            root_map
                as unsafe extern "C" fn(
                    *mut File_t,
                    uint32_t,
                    *mut uint32_t,
                    libc::c_int,
                    *mut mt_off_t,
                ) -> libc::c_int,
        );
    }
    if first == 1 as libc::c_int as libc::c_uint {
        (*File).FirstAbsCluNr = 0 as libc::c_int as libc::c_uint;
    } else {
        (*File).FirstAbsCluNr = first;
    }
    (*File).loopDetectRel = 0 as libc::c_int as libc::c_uint;
    (*File).loopDetectAbs = 0 as libc::c_int as libc::c_uint;
    (*File).PreviousRelCluNr = 0xffff as libc::c_int as libc::c_uint;
    (*File).FileSize = size;
    hash_add(filehash, File as *mut libc::c_void, &mut (*File).hint);
    return File as *mut Stream_t;
}
unsafe extern "C" fn bufferize(mut Dir: *mut *mut Stream_t) {
    let mut BDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut file: *mut File_t = *Dir as *mut File_t;
    if (*Dir).is_null() {
        return;
    }
    if !((*file).Buffer).is_null() {
        (**Dir).refs -= 1;
        (**Dir).refs;
        (*(*file).Buffer).refs += 1;
        (*(*file).Buffer).refs;
        *Dir = (*file).Buffer;
        return;
    }
    BDir = buf_init(
        *Dir,
        16384 as libc::c_int as size_t,
        512 as libc::c_int as size_t,
        32 as libc::c_int as size_t,
    );
    if BDir.is_null() {
        free_stream(Dir);
        *Dir = 0 as *mut Stream_t;
    } else {
        (*file).Buffer = BDir;
        *Dir = BDir;
    };
}
pub unsafe extern "C" fn OpenRoot(mut Dir: *mut Stream_t) -> *mut Stream_t {
    let mut num: libc::c_uint = 0;
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    let mut size: uint32_t = 0;
    let mut file: *mut Stream_t = 0 as *mut Stream_t;
    memset(
        &mut entry as *mut direntry_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<direntry_t>() as libc::c_ulong,
    );
    num = fat32RootCluster(Dir);
    entry.entry = -(3 as libc::c_int);
    entry.name[0 as libc::c_int as usize] = '\0' as i32;
    mk_entry_from_base(
        b"/\0" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int as libc::c_uchar,
        num,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as time_t,
        &mut entry.dir,
    );
    if num != 0 {
        size = countBytes(Dir, num);
    } else {
        let mut Fs: *mut Fs_t = GetFs(Dir) as *mut Fs_t;
        size = ((*Fs).dir_len as libc::c_int * (*Fs).sector_size as libc::c_int)
            as uint32_t;
    }
    file = _internalFileOpen(Dir, num, size, &mut entry);
    bufferize(&mut file);
    return file;
}
pub unsafe extern "C" fn OpenFileByDirentry(
    mut entry: *mut direntry_t,
) -> *mut Stream_t {
    let mut file: *mut Stream_t = 0 as *mut Stream_t;
    let mut first: libc::c_uint = 0;
    let mut size: uint32_t = 0;
    first = getStart((*entry).Dir, &mut (*entry).dir);
    if first == 0 && (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        return OpenRoot((*entry).Dir);
    }
    if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        size = countBytes((*entry).Dir, first);
    } else {
        size = (((*entry).dir.size[0 as libc::c_int as usize] as libc::c_int
            + (((*entry).dir.size[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int
            + (((*((*entry).dir.size)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int
                + ((*((*entry).dir.size)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                << 16 as libc::c_int)) as uint32_t;
    }
    file = _internalFileOpen((*entry).Dir, first, size, entry);
    if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        bufferize(&mut file);
        if first == 1 as libc::c_int as libc::c_uint {
            dir_grow(file, 0 as libc::c_int as libc::c_uint);
        }
    }
    return file;
}
pub unsafe extern "C" fn isRootDir(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut File_t = getUnbufferedFile(Stream);
    return ((*This).map
        == Some(
            root_map
                as unsafe extern "C" fn(
                    *mut File_t,
                    uint32_t,
                    *mut uint32_t,
                    libc::c_int,
                    *mut mt_off_t,
                ) -> libc::c_int,
        )) as libc::c_int;
}
