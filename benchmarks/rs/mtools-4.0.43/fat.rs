use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    static mut got_signal: libc::c_int;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    static mut mtools_skip_check: libc::c_uint;
    static mut batchmode: libc::c_int;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
    fn cp_close(cp: *mut doscp_t);
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
pub struct InfoSector_t {
    pub signature1: [libc::c_uchar; 4],
    pub filler1: [libc::c_uchar; 480],
    pub signature2: [libc::c_uchar; 4],
    pub count: [libc::c_uchar; 4],
    pub pos: [libc::c_uchar; 4],
    pub filler2: [libc::c_uchar; 14],
    pub signature3: [libc::c_uchar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct label_blk_t {
    pub physdrive: libc::c_uchar,
    pub reserved: libc::c_uchar,
    pub dos4: libc::c_uchar,
    pub serial: [libc::c_uchar; 4],
    pub label: [libc::c_char; 11],
    pub fat_type: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fat32_t {
    pub bigFat: [libc::c_uchar; 4],
    pub extFlags: [libc::c_uchar; 2],
    pub fsVersion: [libc::c_uchar; 2],
    pub rootCluster: [libc::c_uchar; 4],
    pub infoSector: [libc::c_uchar; 2],
    pub backupBoot: [libc::c_uchar; 2],
    pub reserved: [libc::c_uchar; 6],
    pub reserved2: [libc::c_uchar; 6],
    pub labelBlock: label_blk_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldboot_t {
    pub labelBlock: label_blk_t,
    pub res_2m: libc::c_uchar,
    pub CheckSum: libc::c_uchar,
    pub fmt_2mf: libc::c_uchar,
    pub wt: libc::c_uchar,
    pub rate_0: libc::c_uchar,
    pub rate_any: libc::c_uchar,
    pub BootP: [libc::c_uchar; 2],
    pub Infp0: [libc::c_uchar; 2],
    pub InfpX: [libc::c_uchar; 2],
    pub InfTm: [libc::c_uchar; 2],
    pub DateF: [libc::c_uchar; 2],
    pub TimeF: [libc::c_uchar; 2],
    pub junk: [libc::c_uchar; 944],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bootsector_s {
    pub jump: [libc::c_uchar; 3],
    pub banner: [libc::c_char; 8],
    pub secsiz: [libc::c_uchar; 2],
    pub clsiz: libc::c_uchar,
    pub nrsvsect: [libc::c_uchar; 2],
    pub nfat: libc::c_uchar,
    pub dirents: [libc::c_uchar; 2],
    pub psect: [libc::c_uchar; 2],
    pub descr: libc::c_uchar,
    pub fatlen: [libc::c_uchar; 2],
    pub nsect: [libc::c_uchar; 2],
    pub nheads: [libc::c_uchar; 2],
    pub nhs: [libc::c_uchar; 4],
    pub bigsect: [libc::c_uchar; 4],
    pub ext: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fat32: fat32_t,
    pub old: oldboot_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union bootsector {
    pub bytes: [libc::c_uchar; 4096],
    pub characters: [libc::c_char; 4096],
    pub boot: bootsector_s,
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
pub type smt_off_t = mt_off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FatMap_t {
    pub data: *mut libc::c_uchar,
    pub dirty: fatBitMask,
    pub valid: fatBitMask,
}
pub type fatBitMask = libc::c_longlong;
#[inline]
unsafe extern "C" fn set_dword(mut data: *mut libc::c_uchar, mut value: uint32_t) {
    *data
        .offset(
            3 as libc::c_int as isize,
        ) = (value >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            2 as libc::c_int as isize,
        ) = (value >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            1 as libc::c_int as isize,
        ) = (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (value >> 0 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
}
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
#[inline]
unsafe extern "C" fn readSector(
    mut This: *mut Fs_t,
    mut buf: *mut libc::c_char,
    mut off: libc::c_uint,
    mut size: size_t,
) -> ssize_t {
    return ((*(*(*This).head.Next).Class).pread)
        .unwrap()(
        (*This).head.Next,
        buf,
        sectorsToBytes(This, off),
        size << (*This).sectorShift,
    );
}
#[inline]
unsafe extern "C" fn forceReadSector(
    mut This: *mut Fs_t,
    mut buf: *mut libc::c_char,
    mut off: libc::c_uint,
    mut size: size_t,
) -> ssize_t {
    return force_pread(
        (*This).head.Next,
        buf,
        sectorsToBytes(This, off),
        size << (*This).sectorShift,
    );
}
#[inline]
unsafe extern "C" fn forceWriteSector(
    mut This: *mut Fs_t,
    mut buf: *mut libc::c_char,
    mut off: libc::c_uint,
    mut size: size_t,
) -> ssize_t {
    return force_pwrite(
        (*This).head.Next,
        buf,
        sectorsToBytes(This, off),
        size << (*This).sectorShift,
    );
}
unsafe extern "C" fn GetFatMap(mut Stream: *mut Fs_t) -> *mut FatMap_t {
    let mut nr_entries: size_t = 0;
    let mut i: size_t = 0;
    let mut map: *mut FatMap_t = 0 as *mut FatMap_t;
    (*Stream).fat_error = 0 as libc::c_int;
    nr_entries = ((*Stream).fat_len as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        );
    map = calloc(nr_entries, ::std::mem::size_of::<FatMap_t>() as libc::c_ulong)
        as *mut FatMap_t;
    if map.is_null() {
        return 0 as *mut FatMap_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < nr_entries {
        let ref mut fresh0 = (*map.offset(i as isize)).data;
        *fresh0 = 0 as *mut libc::c_uchar;
        (*map.offset(i as isize)).valid = 0 as libc::c_int as fatBitMask;
        (*map.offset(i as isize)).dirty = 0 as libc::c_int as fatBitMask;
        i = i.wrapping_add(1);
        i;
    }
    return map;
}
#[inline]
unsafe extern "C" fn locate(
    mut Stream: *mut Fs_t,
    mut offset: uint32_t,
    mut slot: *mut uint32_t,
    mut bit: *mut uint32_t,
) -> libc::c_int {
    if offset >= (*Stream).fat_len {
        return -(1 as libc::c_int);
    }
    *slot = (offset as libc::c_ulong)
        .wrapping_div(
            (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as uint32_t;
    *bit = (offset as libc::c_ulong)
        .wrapping_rem(
            (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as uint32_t;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn fatReadSector(
    mut This: *mut Fs_t,
    mut sector: libc::c_uint,
    mut slot: libc::c_uint,
    mut bit: libc::c_uint,
    mut dupe: libc::c_uint,
    mut bitmap: fatBitMask,
) -> ssize_t {
    let mut fat_start: libc::c_uint = 0;
    let mut ret: ssize_t = 0;
    let mut nr_sectors: libc::c_uint = 0;
    dupe = dupe
        .wrapping_add((*This).primaryFat)
        .wrapping_rem((*This).num_fat as libc::c_uint);
    fat_start = ((*This).fat_start as libc::c_uint)
        .wrapping_add(((*This).fat_len).wrapping_mul(dupe));
    if bitmap == 0 as libc::c_int as libc::c_longlong {
        nr_sectors = (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                (bit as libc::c_ulong)
                    .wrapping_rem(
                        (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ),
            ) as libc::c_uint;
    } else {
        nr_sectors = 1 as libc::c_int as libc::c_uint;
    }
    ret = readSector(
        This,
        ((*((*This).FatMap).offset(slot as isize)).data)
            .offset((bit << (*This).sectorShift) as isize) as *mut libc::c_char,
        fat_start.wrapping_add(sector),
        nr_sectors as size_t,
    );
    if ret < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as ssize_t;
    }
    if (ret as size_t) < (*This).sector_size as libc::c_ulong {
        ret = forceReadSector(
            This,
            ((*((*This).FatMap).offset(slot as isize)).data)
                .offset((bit << (*This).sectorShift) as isize) as *mut libc::c_char,
            fat_start.wrapping_add(sector),
            1 as libc::c_int as size_t,
        );
        if ret < (*This).sector_size as libc::c_int as libc::c_long {
            return 0 as libc::c_int as ssize_t;
        }
        return 1 as libc::c_int as ssize_t;
    }
    return ret >> (*This).sectorShift;
}
unsafe extern "C" fn fatWriteSector(
    mut This: *mut Fs_t,
    mut sector: libc::c_uint,
    mut slot: libc::c_uint,
    mut bit: libc::c_uint,
    mut dupe: libc::c_uint,
) -> ssize_t {
    let mut fat_start: libc::c_uint = 0;
    dupe = dupe
        .wrapping_add((*This).primaryFat)
        .wrapping_rem((*This).num_fat as libc::c_uint);
    if dupe != 0 && (*This).writeAllFats == 0 {
        return (*This).sector_size as ssize_t;
    }
    fat_start = ((*This).fat_start as libc::c_uint)
        .wrapping_add(((*This).fat_len).wrapping_mul(dupe));
    return forceWriteSector(
        This,
        ((*((*This).FatMap).offset(slot as isize)).data)
            .offset(bit.wrapping_mul((*This).sector_size as libc::c_uint) as isize)
            as *mut libc::c_char,
        fat_start.wrapping_add(sector),
        1 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn loadSector(
    mut This: *mut Fs_t,
    mut sector: libc::c_uint,
    mut mode: fatAccessMode_t,
    mut recurs: libc::c_int,
) -> *mut libc::c_uchar {
    let mut slot: uint32_t = 0;
    let mut bit: uint32_t = 0;
    let mut ret: ssize_t = 0;
    if locate(This, sector, &mut slot, &mut bit) < 0 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    if ((*((*This).FatMap).offset(slot as isize)).data).is_null() {
        let ref mut fresh1 = (*((*This).FatMap).offset(slot as isize)).data;
        *fresh1 = malloc(
            ((*This).sector_size as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut libc::c_uchar;
        if ((*((*This).FatMap).offset(slot as isize)).data).is_null() {
            return 0 as *mut libc::c_uchar;
        }
        memset(
            (*((*This).FatMap).offset(slot as isize)).data as *mut libc::c_void,
            0xee as libc::c_int,
            ((*This).sector_size as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ),
        );
    }
    if (*((*This).FatMap).offset(slot as isize)).valid
        & (1 as libc::c_int as fatBitMask) << bit == 0
    {
        let mut i: libc::c_uint = 0;
        ret = -(1 as libc::c_int) as ssize_t;
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*This).num_fat as libc::c_uint {
            ret = fatReadSector(
                This,
                sector,
                slot,
                bit,
                i,
                (*((*This).FatMap).offset(slot as isize)).valid,
            );
            if ret == 0 as libc::c_int as libc::c_long {
                fprintf(
                    stderr,
                    b"Error reading fat number %d\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
                i = i.wrapping_add(1);
                i;
            } else {
                if (*((*This).FatMap).offset(slot as isize)).valid != 0 {
                    recurs = 1 as libc::c_int;
                }
                break;
            }
        }
        if ret == 0 as libc::c_int as libc::c_long {
            return 0 as *mut libc::c_uchar;
        }
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_int as libc::c_long) < ret {
            let ref mut fresh2 = (*((*This).FatMap).offset(slot as isize)).valid;
            *fresh2 |= (1 as libc::c_int as fatBitMask) << bit.wrapping_add(i);
            i = i.wrapping_add(1);
            i;
        }
        if recurs == 0 && ret == 1 as libc::c_int as libc::c_long {
            loadSector(
                This,
                sector.wrapping_add(1 as libc::c_int as libc::c_uint),
                mode,
                1 as libc::c_int,
            );
        }
        if recurs == 0 && batchmode != 0 {
            i = 0 as libc::c_int as libc::c_uint;
            while i < 1024 as libc::c_int as libc::c_uint {
                loadSector(This, sector.wrapping_add(i), mode, 1 as libc::c_int);
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    if mode as libc::c_uint == FAT_ACCESS_WRITE as libc::c_int as libc::c_uint {
        let ref mut fresh3 = (*((*This).FatMap).offset(slot as isize)).dirty;
        *fresh3 |= (1 as libc::c_int as fatBitMask) << bit;
        (*This).fat_dirty = 1 as libc::c_int;
    }
    return ((*((*This).FatMap).offset(slot as isize)).data)
        .offset((bit << (*This).sectorShift) as isize);
}
unsafe extern "C" fn getAddress(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
    mut mode: fatAccessMode_t,
) -> *mut libc::c_uchar {
    let mut ret: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sector: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    sector = num >> (*Stream).sectorShift;
    ret = 0 as *mut libc::c_uchar;
    if sector == (*Stream).lastFatSectorNr
        && (*Stream).lastFatAccessMode as libc::c_uint >= mode as libc::c_uint
    {
        ret = (*Stream).lastFatSectorData;
    }
    if ret.is_null() {
        ret = loadSector(Stream, sector, mode, 0 as libc::c_int);
        if ret.is_null() {
            return 0 as *mut libc::c_uchar;
        }
        (*Stream).lastFatSectorNr = sector;
        (*Stream).lastFatSectorData = ret;
        (*Stream).lastFatAccessMode = mode;
    }
    offset = num & (*Stream).sectorMask;
    return ret.offset(offset as isize);
}
unsafe extern "C" fn readByte(
    mut Stream: *mut Fs_t,
    mut start: libc::c_uint,
) -> libc::c_int {
    let mut address: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    address = getAddress(Stream, start, FAT_ACCESS_READ);
    if address.is_null() {
        return -(1 as libc::c_int);
    }
    return *address as libc::c_int;
}
unsafe extern "C" fn fat12_decode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
) -> libc::c_uint {
    let mut start: libc::c_uint = num
        .wrapping_mul(3 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    let mut byte0: libc::c_int = readByte(Stream, start);
    let mut byte1: libc::c_int = readByte(
        Stream,
        start.wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    if num < 2 as libc::c_int as libc::c_uint || byte0 < 0 as libc::c_int
        || byte1 < 0 as libc::c_int
        || num > ((*Stream).num_clus).wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"[1] Bad address %d\n\0" as *const u8 as *const libc::c_char,
            num,
        );
        return 1 as libc::c_int as libc::c_uint;
    }
    if num & 1 as libc::c_int as libc::c_uint != 0 {
        return (byte1 as uint32_t) << 4 as libc::c_int
            | (byte0 as uint32_t & 0xf0 as libc::c_int as libc::c_uint)
                >> 4 as libc::c_int
    } else {
        return (byte1 as uint32_t & 0xf as libc::c_int as libc::c_uint)
            << 8 as libc::c_int | byte0 as uint32_t
    };
}
unsafe extern "C" fn fat12_encode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
    mut code: libc::c_uint,
) {
    let mut start: libc::c_uint = num
        .wrapping_mul(3 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    let mut address0: *mut libc::c_uchar = getAddress(Stream, start, FAT_ACCESS_WRITE);
    let mut address1: *mut libc::c_uchar = getAddress(
        Stream,
        start.wrapping_add(1 as libc::c_int as libc::c_uint),
        FAT_ACCESS_WRITE,
    );
    if num & 1 as libc::c_int as libc::c_uint != 0 {
        *address0 = ((*address0 as libc::c_int & 0xf as libc::c_int) as libc::c_uint
            | code << 4 as libc::c_int & 0xf0 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *address1 = (code >> 4 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
    } else {
        *address0 = (code & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *address1 = ((*address1 as libc::c_int & 0xf0 as libc::c_int) as libc::c_uint
            | code >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
            as libc::c_uchar;
    };
}
unsafe extern "C" fn fat16_decode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
) -> libc::c_uint {
    let mut address: *mut libc::c_uchar = getAddress(
        Stream,
        num << 1 as libc::c_int,
        FAT_ACCESS_READ,
    );
    if address.is_null() {
        return 1 as libc::c_int as libc::c_uint;
    }
    return (*address.offset(0 as libc::c_int as isize) as libc::c_int
        + ((*address.offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_uint;
}
unsafe extern "C" fn fat16_encode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
    mut code: libc::c_uint,
) {
    let mut address: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if code > 65535 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"FAT16 code %x too big\n\0" as *const u8 as *const libc::c_char,
            code,
        );
        exit(1 as libc::c_int);
    }
    address = getAddress(Stream, num << 1 as libc::c_int, FAT_ACCESS_WRITE);
    set_word(address, code as uint16_t);
}
unsafe extern "C" fn fast_fat16_decode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
) -> libc::c_uint {
    let mut address: *mut libc::c_ushort = getAddress(
        Stream,
        num << 1 as libc::c_int,
        FAT_ACCESS_READ,
    ) as *mut libc::c_ushort;
    if address.is_null() {
        return 1 as libc::c_int as libc::c_uint;
    }
    return *address as libc::c_uint;
}
unsafe extern "C" fn fast_fat16_encode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
    mut code: libc::c_uint,
) {
    let mut address: *mut libc::c_ushort = getAddress(
        Stream,
        num << 1 as libc::c_int,
        FAT_ACCESS_WRITE,
    ) as *mut libc::c_ushort;
    if code > 65535 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"FAT16 code %x too big\n\0" as *const u8 as *const libc::c_char,
            code,
        );
        exit(1 as libc::c_int);
    }
    *address = code as uint16_t;
}
unsafe extern "C" fn fat32_decode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
) -> libc::c_uint {
    let mut address: *mut libc::c_uchar = getAddress(
        Stream,
        num << 2 as libc::c_int,
        FAT_ACCESS_READ,
    );
    if address.is_null() {
        return 1 as libc::c_int as libc::c_uint;
    }
    return ((*address.offset(0 as libc::c_int as isize) as libc::c_int
        + ((*address.offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_int
        + (((*address.offset(2 as libc::c_int as isize).offset(0 as libc::c_int as isize)
            as libc::c_int
            + ((*address
                .offset(2 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
            as uint16_t as libc::c_int) << 16 as libc::c_int)) as uint32_t
        & 0xfffffff as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn fat32_encode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
    mut code: libc::c_uint,
) {
    let mut address: *mut libc::c_uchar = getAddress(
        Stream,
        num << 2 as libc::c_int,
        FAT_ACCESS_WRITE,
    );
    set_dword(
        address,
        code & 0xfffffff as libc::c_int as libc::c_uint
            | ((*address.offset(0 as libc::c_int as isize) as libc::c_int
                + ((*address.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*address
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*address
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t & 0xf0000000 as libc::c_uint,
    );
}
unsafe extern "C" fn fast_fat32_decode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
) -> libc::c_uint {
    let mut address: *mut libc::c_uint = getAddress(
        Stream,
        num << 2 as libc::c_int,
        FAT_ACCESS_READ,
    ) as *mut libc::c_uint;
    if address.is_null() {
        return 1 as libc::c_int as libc::c_uint;
    }
    return *address & 0xfffffff as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn fast_fat32_encode(
    mut Stream: *mut Fs_t,
    mut num: libc::c_uint,
    mut code: libc::c_uint,
) {
    let mut address: *mut libc::c_uint = getAddress(
        Stream,
        num << 2 as libc::c_int,
        FAT_ACCESS_WRITE,
    ) as *mut libc::c_uint;
    *address = *address & 0xf0000000 as libc::c_uint
        | code & 0xfffffff as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn fat_write(mut This: *mut Fs_t) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut dups: libc::c_uint = 0;
    let mut bit: libc::c_uint = 0;
    let mut slot: libc::c_uint = 0;
    let mut ret: ssize_t = 0;
    if (*This).fat_dirty == 0 {
        return;
    }
    dups = (*This).num_fat as libc::c_uint;
    if (*This).fat_error != 0 {
        dups = 1 as libc::c_int as libc::c_uint;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < dups {
        j = 0 as libc::c_int as libc::c_uint;
        slot = 0 as libc::c_int as libc::c_uint;
        while j < (*This).fat_len {
            if (*((*This).FatMap).offset(slot as isize)).dirty == 0 {
                j = (j as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    ) as libc::c_uint as libc::c_uint;
            } else {
                bit = 0 as libc::c_int as libc::c_uint;
                while (bit as libc::c_ulong)
                    < (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    && j < (*This).fat_len
                {
                    if !((*((*This).FatMap).offset(slot as isize)).dirty
                        & (1 as libc::c_int as fatBitMask) << bit == 0)
                    {
                        ret = fatWriteSector(This, j, slot, bit, i);
                        if ret < (*This).sector_size as libc::c_int as libc::c_long {
                            if ret < 0 as libc::c_int as libc::c_long {
                                perror(
                                    b"error in fat_write\0" as *const u8 as *const libc::c_char,
                                );
                                exit(1 as libc::c_int);
                            } else {
                                fprintf(
                                    stderr,
                                    b"end of file in fat_write\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                exit(1 as libc::c_int);
                            }
                        }
                        if i == dups.wrapping_sub(1 as libc::c_int as libc::c_uint) {
                            let ref mut fresh4 = (*((*This).FatMap)
                                .offset(slot as isize))
                                .dirty;
                            *fresh4 &= !((1 as libc::c_int as fatBitMask) << bit);
                        }
                    }
                    bit = bit.wrapping_add(1);
                    bit;
                    j = j.wrapping_add(1);
                    j;
                }
            }
            slot = slot.wrapping_add(1);
            slot;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*This).infoSectorLoc != 0 && (*This).infoSectorLoc != 0xffffffff as libc::c_uint
    {
        let mut infoSector: *mut InfoSector_t = 0 as *mut InfoSector_t;
        infoSector = safe_malloc((*This).sector_size as size_t) as *mut InfoSector_t;
        if forceReadSector(
            This,
            infoSector as *mut libc::c_char,
            (*This).infoSectorLoc,
            1 as libc::c_int as size_t,
        ) != (*This).sector_size as libc::c_int as libc::c_long
        {
            fprintf(
                stderr,
                b"Trouble reading the info sector\n\0" as *const u8
                    as *const libc::c_char,
            );
            memset(
                ((*infoSector).filler1).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[libc::c_uchar; 480]>() as libc::c_ulong,
            );
            memset(
                ((*infoSector).filler2).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[libc::c_uchar; 14]>() as libc::c_ulong,
            );
        }
        set_dword(
            ((*infoSector).signature1).as_mut_ptr(),
            0x41615252 as libc::c_int as uint32_t,
        );
        set_dword(
            ((*infoSector).signature2).as_mut_ptr(),
            0x61417272 as libc::c_int as uint32_t,
        );
        set_dword(((*infoSector).pos).as_mut_ptr(), (*This).last);
        set_dword(((*infoSector).count).as_mut_ptr(), (*This).freeSpace);
        set_word(
            ((*infoSector).signature3).as_mut_ptr(),
            0xaa55 as libc::c_int as libc::c_ushort,
        );
        if forceWriteSector(
            This,
            infoSector as *mut libc::c_char,
            (*This).infoSectorLoc,
            1 as libc::c_int as size_t,
        ) != (*This).sector_size as libc::c_int as libc::c_long
        {
            fprintf(
                stderr,
                b"Trouble writing the info sector\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        free(infoSector as *mut libc::c_void);
    }
    (*This).fat_dirty = 0 as libc::c_int;
    (*This).lastFatAccessMode = FAT_ACCESS_READ;
}
pub unsafe extern "C" fn zero_fat(
    mut Stream: *mut Fs_t,
    mut media_descriptor: uint8_t,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut fat_start: libc::c_uint = 0;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    buf = malloc((*Stream).sector_size as libc::c_ulong) as *mut libc::c_uchar;
    if buf.is_null() {
        perror(b"alloc fat sector buffer\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*Stream).num_fat as libc::c_uint {
        fat_start = ((*Stream).fat_start as libc::c_uint)
            .wrapping_add(i.wrapping_mul((*Stream).fat_len));
        j = 0 as libc::c_int as libc::c_uint;
        while j < (*Stream).fat_len {
            if j <= 1 as libc::c_int as libc::c_uint {
                memset(
                    buf as *mut libc::c_void,
                    0 as libc::c_int,
                    (*Stream).sector_size as libc::c_ulong,
                );
            }
            if j == 0 {
                *buf.offset(0 as libc::c_int as isize) = media_descriptor;
                let ref mut fresh5 = *buf.offset(1 as libc::c_int as isize);
                *fresh5 = 0xff as libc::c_int as libc::c_uchar;
                *buf.offset(2 as libc::c_int as isize) = *fresh5;
                if (*Stream).fat_bits > 12 as libc::c_int as libc::c_uint {
                    *buf
                        .offset(
                            3 as libc::c_int as isize,
                        ) = 0xff as libc::c_int as libc::c_uchar;
                }
                if (*Stream).fat_bits > 16 as libc::c_int as libc::c_uint {
                    *buf
                        .offset(
                            3 as libc::c_int as isize,
                        ) = 0xf as libc::c_int as libc::c_uchar;
                    *buf
                        .offset(
                            4 as libc::c_int as isize,
                        ) = 0xff as libc::c_int as libc::c_uchar;
                    *buf
                        .offset(
                            5 as libc::c_int as isize,
                        ) = 0xff as libc::c_int as libc::c_uchar;
                    *buf
                        .offset(
                            6 as libc::c_int as isize,
                        ) = 0xff as libc::c_int as libc::c_uchar;
                    *buf
                        .offset(
                            7 as libc::c_int as isize,
                        ) = 0xff as libc::c_int as libc::c_uchar;
                }
            }
            if forceWriteSector(
                Stream,
                buf as *mut libc::c_char,
                fat_start.wrapping_add(j),
                1 as libc::c_int as size_t,
            ) != (*Stream).sector_size as libc::c_int as libc::c_long
            {
                fprintf(
                    stderr,
                    b"Trouble initializing a FAT sector\n\0" as *const u8
                        as *const libc::c_char,
                );
                free(buf as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(buf as *mut libc::c_void);
    (*Stream).FatMap = GetFatMap(Stream);
    if ((*Stream).FatMap).is_null() {
        perror(b"alloc fat map\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_fat12(mut This: *mut Fs_t) {
    (*This).fat_bits = 12 as libc::c_int as libc::c_uint;
    (*This).end_fat = 0xfff as libc::c_int as uint32_t;
    (*This).last_fat = 0xff6 as libc::c_int as uint32_t;
    (*This)
        .fat_decode = Some(
        fat12_decode as unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
    );
    (*This)
        .fat_encode = Some(
        fat12_encode as unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
    );
}
static mut word_endian_test: uint16_t = 0x1234 as libc::c_int as uint16_t;
unsafe extern "C" fn set_fat16(mut This: *mut Fs_t) {
    let mut t: *mut uint8_t = &mut word_endian_test as *mut uint16_t as *mut uint8_t;
    (*This).fat_bits = 16 as libc::c_int as libc::c_uint;
    (*This).end_fat = 0xffff as libc::c_int as uint32_t;
    (*This).last_fat = 0xfff6 as libc::c_int as uint32_t;
    if *t.offset(0 as libc::c_int as isize) as libc::c_int == 0x34 as libc::c_int
        && *t.offset(1 as libc::c_int as isize) as libc::c_int == 0x12 as libc::c_int
    {
        (*This)
            .fat_decode = Some(
            fast_fat16_decode
                as unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
        );
        (*This)
            .fat_encode = Some(
            fast_fat16_encode
                as unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
        );
    } else {
        (*This)
            .fat_decode = Some(
            fat16_decode as unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
        );
        (*This)
            .fat_encode = Some(
            fat16_encode
                as unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
        );
    };
}
static mut dword_endian_test: uint32_t = 0x12345678 as libc::c_int as uint32_t;
unsafe extern "C" fn set_fat32(mut This: *mut Fs_t) {
    let mut t: *mut uint8_t = &mut dword_endian_test as *mut uint32_t as *mut uint8_t;
    (*This).fat_bits = 32 as libc::c_int as libc::c_uint;
    (*This).end_fat = 0xfffffff as libc::c_int as uint32_t;
    (*This).last_fat = 0xffffff6 as libc::c_int as uint32_t;
    if *t.offset(0 as libc::c_int as isize) as libc::c_int == 0x78 as libc::c_int
        && *t.offset(1 as libc::c_int as isize) as libc::c_int == 0x56 as libc::c_int
        && *t.offset(2 as libc::c_int as isize) as libc::c_int == 0x34 as libc::c_int
        && *t.offset(3 as libc::c_int as isize) as libc::c_int == 0x12 as libc::c_int
    {
        (*This)
            .fat_decode = Some(
            fast_fat32_decode
                as unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
        );
        (*This)
            .fat_encode = Some(
            fast_fat32_encode
                as unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
        );
    } else {
        (*This)
            .fat_decode = Some(
            fat32_decode as unsafe extern "C" fn(*mut Fs_t, libc::c_uint) -> libc::c_uint,
        );
        (*This)
            .fat_encode = Some(
            fat32_encode
                as unsafe extern "C" fn(*mut Fs_t, libc::c_uint, libc::c_uint) -> (),
        );
    };
}
pub unsafe extern "C" fn set_fat(mut This: *mut Fs_t, mut haveBigFatLen: bool) {
    if haveBigFatLen {
        set_fat32(This);
    } else if (*This).num_clus < 0xff5 as libc::c_int as libc::c_uint {
        set_fat12(This);
    } else if (*This).num_clus < 0xfff5 as libc::c_int as libc::c_uint {
        set_fat16(This);
    } else {
        set_fat32(This);
    };
}
unsafe extern "C" fn check_fat(mut This: *mut Fs_t) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut f: libc::c_uint = 0;
    let mut tocheck: libc::c_uint = 0;
    if mtools_skip_check != 0 {
        return 0 as libc::c_int;
    }
    if (*This).fat_len
        < ((*This).num_clus)
            .wrapping_add(2 as libc::c_int as libc::c_uint)
            .wrapping_mul(
                ((*This).fat_bits).wrapping_div(4 as libc::c_int as libc::c_uint),
            )
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint)
            .wrapping_div((*This).sector_size as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        fprintf(
            stderr,
            b"Too few sectors in FAT\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    tocheck = (*This).num_clus;
    if tocheck.wrapping_add(1 as libc::c_int as libc::c_uint) >= (*This).last_fat {
        fprintf(
            stderr,
            b"Too many clusters in FAT\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if tocheck > 4096 as libc::c_int as libc::c_uint {
        tocheck = 4096 as libc::c_int as libc::c_uint;
    }
    i = 3 as libc::c_int as libc::c_uint;
    while i < tocheck {
        f = ((*This).fat_decode).unwrap()(This, i);
        if f == 1 as libc::c_int as libc::c_uint
            || f < (*This).last_fat && f > (*This).num_clus
        {
            fprintf(
                stderr,
                b"Cluster # at %d too big(%#x)\n\0" as *const u8 as *const libc::c_char,
                i,
                f,
            );
            fprintf(
                stderr,
                b"Probably non MS-DOS disk\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_media_type(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
) -> libc::c_int {
    let mut address: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    (*This).FatMap = GetFatMap(This);
    if ((*This).FatMap).is_null() {
        perror(b"alloc fat map\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    address = getAddress(This, 0 as libc::c_int as libc::c_uint, FAT_ACCESS_READ);
    if address.is_null() {
        fprintf(
            stderr,
            b"Could not read first FAT sector\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if mtools_skip_check != 0 {
        return 0 as libc::c_int;
    }
    if *address.offset(0 as libc::c_int as isize) == 0
        && *address.offset(1 as libc::c_int as isize) == 0
        && *address.offset(2 as libc::c_int as isize) == 0
    {
        return 0 as libc::c_int;
    }
    if *address.offset(0 as libc::c_int as isize) as libc::c_int
        != (*boot).boot.descr as libc::c_int
        && (*boot).boot.descr as libc::c_int >= 0xf0 as libc::c_int
        && (*address.offset(0 as libc::c_int as isize) as libc::c_int
            != 0xf9 as libc::c_int
            && *address.offset(0 as libc::c_int as isize) as libc::c_int
                != 0xf7 as libc::c_int
            || (*boot).boot.descr as libc::c_int != 0xf0 as libc::c_int)
        || (*address.offset(0 as libc::c_int as isize) as libc::c_int)
            < 0xf0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Bad media types %02x/%02x, probably non-MSDOS disk\n\0" as *const u8
                as *const libc::c_char,
            *address.offset(0 as libc::c_int as isize) as libc::c_int,
            (*boot).boot.descr as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if *address.offset(1 as libc::c_int as isize) as libc::c_int != 0xff as libc::c_int
        || *address.offset(2 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
    {
        fprintf(
            stderr,
            b"Initial bytes of fat is not 0xff\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fat_32_read(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
) -> libc::c_int {
    let mut size: size_t = 0;
    (*This)
        .fat_len = (((*boot).boot.ext.fat32.bigFat[0 as libc::c_int as usize]
        as libc::c_int
        + (((*boot).boot.ext.fat32.bigFat[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_int
        + (((*((*boot).boot.ext.fat32.bigFat)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int
            + ((*((*boot).boot.ext.fat32.bigFat)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
            as uint16_t as libc::c_int) << 16 as libc::c_int)) as uint32_t;
    (*This)
        .writeAllFats = ((*boot).boot.ext.fat32.extFlags[0 as libc::c_int as usize]
        as libc::c_int & 0x80 as libc::c_int == 0) as libc::c_int as uint32_t;
    (*This)
        .primaryFat = ((*boot).boot.ext.fat32.extFlags[0 as libc::c_int as usize]
        as libc::c_int & 0xf as libc::c_int) as uint32_t;
    (*This)
        .rootCluster = (((*boot).boot.ext.fat32.rootCluster[0 as libc::c_int as usize]
        as libc::c_int
        + (((*boot).boot.ext.fat32.rootCluster[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_int
        + (((*((*boot).boot.ext.fat32.rootCluster)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int
            + ((*((*boot).boot.ext.fat32.rootCluster)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
            as uint16_t as libc::c_int) << 16 as libc::c_int)) as uint32_t;
    size = (*This).sector_size as size_t;
    (*This)
        .infoSectorLoc = ((*boot).boot.ext.fat32.infoSector[0 as libc::c_int as usize]
        as libc::c_int
        + (((*boot).boot.ext.fat32.infoSector[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as uint32_t;
    if (*This).sector_size as libc::c_int >= 512 as libc::c_int
        && (*This).infoSectorLoc != 0
        && (*This).infoSectorLoc != 0xffffffff as libc::c_uint
    {
        let mut infoSector: *mut InfoSector_t = 0 as *mut InfoSector_t;
        infoSector = safe_malloc(size) as *mut InfoSector_t;
        if forceReadSector(
            This,
            infoSector as *mut libc::c_char,
            (*This).infoSectorLoc,
            1 as libc::c_int as size_t,
        ) == (*This).sector_size as libc::c_int as libc::c_long
            && (((*infoSector).signature1[0 as libc::c_int as usize] as libc::c_int
                + (((*infoSector).signature1[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*infoSector).signature1)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*infoSector).signature1)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t
                == 0x41615252 as libc::c_int as libc::c_uint
            && (((*infoSector).signature2[0 as libc::c_int as usize] as libc::c_int
                + (((*infoSector).signature2[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*infoSector).signature2)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*infoSector).signature2)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t
                == 0x61417272 as libc::c_int as libc::c_uint
        {
            (*This)
                .freeSpace = (((*infoSector).count[0 as libc::c_int as usize]
                as libc::c_int
                + (((*infoSector).count[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*infoSector).count)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*infoSector).count)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t;
            (*This)
                .last = (((*infoSector).pos[0 as libc::c_int as usize] as libc::c_int
                + (((*infoSector).pos[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*infoSector).pos)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*infoSector).pos)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t;
        }
        free(infoSector as *mut libc::c_void);
    }
    return (check_media_type(This, boot) != 0 || check_fat(This) != 0) as libc::c_int;
}
unsafe extern "C" fn old_fat_read(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
    mut nodups: libc::c_int,
) -> libc::c_int {
    (*This).writeAllFats = 1 as libc::c_int as uint32_t;
    (*This).primaryFat = 0 as libc::c_int as uint32_t;
    (*This)
        .dir_start = ((*This).fat_start as libc::c_uint)
        .wrapping_add(((*This).num_fat as libc::c_uint).wrapping_mul((*This).fat_len));
    (*This).infoSectorLoc = 0xffffffff as libc::c_uint;
    if nodups != 0 {
        (*This).num_fat = 1 as libc::c_int as uint8_t;
    }
    if check_media_type(This, boot) != 0 {
        return -(1 as libc::c_int);
    }
    if (*This).fat_bits == 16 as libc::c_int as libc::c_uint {
        if mtools_skip_check == 0
            && readByte(This, 3 as libc::c_int as libc::c_uint) != 0xff as libc::c_int
        {
            return -(1 as libc::c_int);
        }
    }
    return check_fat(This);
}
pub unsafe extern "C" fn fat_read(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
    mut nodups: libc::c_int,
) -> libc::c_int {
    (*This).fat_error = 0 as libc::c_int;
    (*This).fat_dirty = 0 as libc::c_int;
    (*This).last = 0xffffffff as libc::c_uint;
    (*This).freeSpace = 0xffffffff as libc::c_uint;
    (*This).lastFatSectorNr = 0 as libc::c_int as uint32_t;
    (*This).lastFatSectorData = 0 as *mut libc::c_uchar;
    if (*This).fat_bits >= 12 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"This->fat_bits >= 12\0" as *const u8 as *const libc::c_char,
            b"fat.c\0" as *const u8 as *const libc::c_char,
            783 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int fat_read(Fs_t *, union bootsector *, int)\0"))
                .as_ptr(),
        );
    }
    'c_11584: {
        if (*This).fat_bits >= 12 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"This->fat_bits >= 12\0" as *const u8 as *const libc::c_char,
                b"fat.c\0" as *const u8 as *const libc::c_char,
                783 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"int fat_read(Fs_t *, union bootsector *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*This).fat_bits <= 16 as libc::c_int as libc::c_uint {
        return old_fat_read(This, boot, nodups)
    } else {
        return fat_32_read(This, boot)
    };
}
pub unsafe extern "C" fn fatDecode(
    mut This: *mut Fs_t,
    mut pos: libc::c_uint,
) -> libc::c_uint {
    let mut ret: libc::c_uint = 0;
    ret = ((*This).fat_decode).unwrap()(This, pos);
    if ret != 0
        && (ret < 2 as libc::c_int as libc::c_uint
            || ret > ((*This).num_clus).wrapping_add(1 as libc::c_int as libc::c_uint))
        && ret < (*This).last_fat
    {
        fprintf(
            stderr,
            b"Bad FAT entry %d at %d\n\0" as *const u8 as *const libc::c_char,
            ret,
            pos,
        );
        (*This).fat_error += 1;
        (*This).fat_error;
    }
    return ret;
}
pub unsafe extern "C" fn fatAppend(
    mut This: *mut Fs_t,
    mut pos: libc::c_uint,
    mut newpos: libc::c_uint,
) {
    ((*This).fat_encode).unwrap()(This, pos, newpos);
    ((*This).fat_encode).unwrap()(This, newpos, (*This).end_fat);
    if (*This).freeSpace != 0xffffffff as libc::c_uint {
        (*This).freeSpace = ((*This).freeSpace).wrapping_sub(1);
        (*This).freeSpace;
    }
}
pub unsafe extern "C" fn fatDeallocate(mut This: *mut Fs_t, mut pos: libc::c_uint) {
    ((*This).fat_encode).unwrap()(This, pos, 0 as libc::c_int as libc::c_uint);
    if (*This).freeSpace != 0xffffffff as libc::c_uint {
        (*This).freeSpace = ((*This).freeSpace).wrapping_add(1);
        (*This).freeSpace;
    }
}
pub unsafe extern "C" fn fatAllocate(
    mut This: *mut Fs_t,
    mut pos: libc::c_uint,
    mut value: libc::c_uint,
) {
    ((*This).fat_encode).unwrap()(This, pos, value);
    if (*This).freeSpace != 0xffffffff as libc::c_uint {
        (*This).freeSpace = ((*This).freeSpace).wrapping_sub(1);
        (*This).freeSpace;
    }
}
pub unsafe extern "C" fn fatEncode(
    mut This: *mut Fs_t,
    mut pos: libc::c_uint,
    mut value: libc::c_uint,
) {
    let mut oldvalue: libc::c_uint = ((*This).fat_decode).unwrap()(This, pos);
    ((*This).fat_encode).unwrap()(This, pos, value);
    if (*This).freeSpace != 0xffffffff as libc::c_uint {
        if oldvalue != 0 {
            (*This).freeSpace = ((*This).freeSpace).wrapping_add(1);
            (*This).freeSpace;
        }
        if value != 0 {
            (*This).freeSpace = ((*This).freeSpace).wrapping_sub(1);
            (*This).freeSpace;
        }
    }
}
pub unsafe extern "C" fn get_next_free_cluster(
    mut This: *mut Fs_t,
    mut last: libc::c_uint,
) -> libc::c_uint {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    if (*This).last != 0xffffffff as libc::c_uint {
        last = (*This).last;
    }
    if last < 2 as libc::c_int as libc::c_uint
        || last >= ((*This).num_clus).wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        last = 1 as libc::c_int as libc::c_uint;
    }
    i = last.wrapping_add(1 as libc::c_int as libc::c_uint);
    loop {
        if !(i < ((*This).num_clus).wrapping_add(2 as libc::c_int as libc::c_uint)) {
            current_block = 1917311967535052937;
            break;
        }
        let mut r: libc::c_uint = fatDecode(This, i);
        if r == 1 as libc::c_int as libc::c_uint {
            current_block = 14055671384249344838;
            break;
        }
        if r == 0 {
            (*This).last = i;
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    match current_block {
        1917311967535052937 => {
            i = 2 as libc::c_int as libc::c_uint;
            loop {
                if !(i < last.wrapping_add(1 as libc::c_int as libc::c_uint)) {
                    current_block = 7149356873433890176;
                    break;
                }
                let mut r_0: libc::c_uint = fatDecode(This, i);
                if r_0 == 1 as libc::c_int as libc::c_uint {
                    current_block = 14055671384249344838;
                    break;
                }
                if r_0 == 0 {
                    (*This).last = i;
                    return i;
                }
                i = i.wrapping_add(1);
                i;
            }
            match current_block {
                14055671384249344838 => {}
                _ => {
                    fprintf(
                        stderr,
                        b"No free cluster %d %d\n\0" as *const u8 as *const libc::c_char,
                        (*This).preallocatedClusters,
                        (*This).last,
                    );
                    return 1 as libc::c_int as libc::c_uint;
                }
            }
        }
        _ => {}
    }
    fprintf(stderr, b"FAT error\n\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn getSerialized(mut Fs: *mut Fs_t) -> bool {
    return (*Fs).serialized != 0;
}
pub unsafe extern "C" fn getSerialNumber(mut Fs: *mut Fs_t) -> libc::c_ulong {
    return (*Fs).serial_number;
}
pub unsafe extern "C" fn getClusterBytes(mut Fs: *mut Fs_t) -> uint32_t {
    return ((*Fs).cluster_size as libc::c_int * (*Fs).sector_size as libc::c_int)
        as uint32_t;
}
pub unsafe extern "C" fn fat_error(mut Dir: *mut Stream_t) -> libc::c_int {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).fat_error != 0 {
        fprintf(stderr, b"Fat error detected\n\0" as *const u8 as *const libc::c_char);
    }
    return (*This).fat_error;
}
pub unsafe extern "C" fn fat32RootCluster(mut Dir: *mut Stream_t) -> uint32_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).fat_bits == 32 as libc::c_int as libc::c_uint {
        return (*This).rootCluster
    } else {
        return 0 as libc::c_int as uint32_t
    };
}
pub unsafe extern "C" fn getfree(mut Dir: *mut Stream_t) -> mt_off_t {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).freeSpace == 0xffffffff as libc::c_uint
        || (*This).freeSpace == 0 as libc::c_int as libc::c_uint
    {
        let mut i: libc::c_uint = 0;
        let mut total: uint32_t = 0;
        total = 0 as libc::c_long as uint32_t;
        i = 2 as libc::c_int as libc::c_uint;
        while i < ((*This).num_clus).wrapping_add(2 as libc::c_int as libc::c_uint) {
            let mut r: libc::c_uint = fatDecode(This, i);
            if r == 1 as libc::c_int as libc::c_uint {
                return -(1 as libc::c_int) as mt_off_t;
            }
            if r == 0 {
                total = total.wrapping_add(1);
                total;
            }
            i = i.wrapping_add(1);
            i;
        }
        (*This).freeSpace = total;
    }
    return sectorsToBytes(
        This,
        ((*This).freeSpace).wrapping_mul((*This).cluster_size as libc::c_uint),
    );
}
pub unsafe extern "C" fn getfreeMinClusters(
    mut Dir: *mut Stream_t,
    mut size: uint32_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut i: libc::c_uint = 0;
    let mut last: libc::c_uint = 0;
    let mut total: size_t = 0;
    if batchmode != 0 && (*This).freeSpace == 0xffffffff as libc::c_uint {
        getfree(Stream);
    }
    if (*This).freeSpace != 0xffffffff as libc::c_uint {
        if (*This).freeSpace >= size {
            return 1 as libc::c_int
        } else {
            fprintf(stderr, b"Disk full\n\0" as *const u8 as *const libc::c_char);
            got_signal = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    total = 0 as libc::c_long as size_t;
    last = (*This).last;
    if last < 2 as libc::c_int as libc::c_uint
        || last >= ((*This).num_clus).wrapping_add(2 as libc::c_int as libc::c_uint)
    {
        last = 1 as libc::c_int as libc::c_uint;
    }
    i = last.wrapping_add(1 as libc::c_int as libc::c_uint);
    loop {
        if !(i < ((*This).num_clus).wrapping_add(2 as libc::c_int as libc::c_uint)) {
            current_block = 15904375183555213903;
            break;
        }
        let mut r: libc::c_uint = fatDecode(This, i);
        if r == 1 as libc::c_int as libc::c_uint {
            current_block = 7486112596259618466;
            break;
        }
        if r == 0 {
            total = total.wrapping_add(1);
            total;
        }
        if total >= size as libc::c_ulong {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    match current_block {
        15904375183555213903 => {
            i = 2 as libc::c_int as libc::c_uint;
            loop {
                if !(i < last.wrapping_add(1 as libc::c_int as libc::c_uint)) {
                    current_block = 18386322304582297246;
                    break;
                }
                let mut r_0: libc::c_uint = fatDecode(This, i);
                if r_0 == 1 as libc::c_int as libc::c_uint {
                    current_block = 7486112596259618466;
                    break;
                }
                if r_0 == 0 {
                    total = total.wrapping_add(1);
                    total;
                }
                if total >= size as libc::c_ulong {
                    return 1 as libc::c_int;
                }
                i = i.wrapping_add(1);
                i;
            }
            match current_block {
                7486112596259618466 => {}
                _ => {
                    fprintf(
                        stderr,
                        b"Disk full\n\0" as *const u8 as *const libc::c_char,
                    );
                    got_signal = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    fprintf(stderr, b"FAT error\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn getfreeMinBytes(
    mut Dir: *mut Stream_t,
    mut size: mt_off_t,
) -> libc::c_int {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut size2: mt_off_t = 0;
    size2 = size
        / ((*This).sector_size as libc::c_int * (*This).cluster_size as libc::c_int)
            as libc::c_long;
    if size
        % ((*This).sector_size as libc::c_int * (*This).cluster_size as libc::c_int)
            as libc::c_long != 0
    {
        size2 += 1;
        size2;
    }
    if size2 > 4294967295 as libc::c_uint as libc::c_long {
        fprintf(
            stderr,
            b"Requested size too big\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return getfreeMinClusters(Dir, size2 as uint32_t);
}
pub unsafe extern "C" fn getStart(
    mut Dir: *mut Stream_t,
    mut dir: *mut directory,
) -> libc::c_uint {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut first: uint32_t = 0;
    first = ((*dir).start[0 as libc::c_int as usize] as libc::c_int
        + (((*dir).start[1 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int))
        as uint16_t as uint32_t;
    if fat32RootCluster(Stream) != 0 {
        first
            |= (((*dir).startHi[0 as libc::c_int as usize] as libc::c_int
                + (((*dir).startHi[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as uint32_t) << 16 as libc::c_int;
    }
    return first;
}
pub unsafe extern "C" fn fs_free(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if !((*This).FatMap).is_null() {
        let mut i: libc::c_int = 0;
        let mut nr_entries: libc::c_int = 0;
        nr_entries = ((*This).fat_len as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(
                (::std::mem::size_of::<fatBitMask>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
        i = 0 as libc::c_int;
        while i < nr_entries {
            if !((*((*This).FatMap).offset(i as isize)).data).is_null() {
                free((*((*This).FatMap).offset(i as isize)).data as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        free((*This).FatMap as *mut libc::c_void);
    }
    if !((*This).cp).is_null() {
        cp_close((*This).cp);
    }
    return 0 as libc::c_int;
}
