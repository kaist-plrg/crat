use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn wcscasecmp(__s1: *const wchar_t, __s2: *const wchar_t) -> libc::c_int;
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn isRootDir(Stream: *mut Stream_t) -> libc::c_int;
    static mut mtools_numeric_tail: libc::c_uint;
    fn dir_read(entry: *mut direntry_t, error: *mut libc::c_int) -> *mut directory;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn isNotFound(entry: *mut direntry_t) -> libc::c_int;
    fn isRootEntry(entry: *mut direntry_t) -> libc::c_int;
    fn setEntryForIteration(entry: *mut direntry_t, pos: libc::c_uint);
    fn setEntryToPos(entry: *mut direntry_t, pos: libc::c_uint);
    fn getEntryAsPos(entry: *mut direntry_t) -> libc::c_uint;
    fn getNextEntryAsPos(entry: *mut direntry_t) -> libc::c_uint;
    fn low_level_dir_write(entry: *mut direntry_t);
    fn growDirCache(cache: *mut dirCache_t, slot: libc::c_uint) -> libc::c_int;
    fn allocDirCache(Stream: *mut Stream_t, slot: libc::c_uint) -> *mut dirCache_t;
    fn addFreeEntry(
        Stream: *mut dirCache_t,
        begin: libc::c_uint,
        end: libc::c_uint,
    ) -> *mut dirCacheEntry_t;
    fn addFreeEndEntry(
        Stream: *mut dirCache_t,
        begin: libc::c_uint,
        end: libc::c_uint,
        isAtEnd: libc::c_int,
    ) -> *mut dirCacheEntry_t;
    fn addEndEntry(Stream: *mut dirCache_t, pos: libc::c_uint) -> *mut dirCacheEntry_t;
    fn isHashed(cache: *mut dirCache_t, name: *mut wchar_t) -> libc::c_int;
    fn addUsedEntry(
        Stream: *mut dirCache_t,
        begin: libc::c_uint,
        end: libc::c_uint,
        longName: *mut wchar_t,
        shortName: *mut wchar_t,
        dir: *mut directory,
    ) -> *mut dirCacheEntry_t;
    fn dos_to_wchar(
        fromDos: *mut doscp_t,
        dos: *const libc::c_char,
        wchar: *mut wchar_t,
        len: size_t,
    ) -> size_t;
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut libc::c_char,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
    fn native_to_wchar(
        native: *const libc::c_char,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const libc::c_char,
        mangled: *mut libc::c_int,
    ) -> size_t;
    #[link_name = "match"]
    fn match_0(
        _: *const wchar_t,
        _: *const wchar_t,
        _: *mut wchar_t,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn unix_name(
        fromDos: *mut doscp_t,
        base: *const libc::c_char,
        ext: *const libc::c_char,
        Case: uint8_t,
        answer: *mut wchar_t,
    ) -> *mut wchar_t;
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
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dos_name_t {
    pub base: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub sentinel: libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scan_state {
    pub match_free: libc::c_int,
    pub shortmatch: libc::c_int,
    pub longmatch: libc::c_int,
    pub free_start: libc::c_uint,
    pub free_end: libc::c_uint,
    pub slot: libc::c_uint,
    pub got_slots: libc::c_int,
    pub size_needed: libc::c_uint,
    pub max_entry: libc::c_uint,
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
pub const RES_MATCH: result_t = 1;
pub type result_t = libc::c_uint;
pub const RES_ERROR: result_t = 3;
pub const RES_END: result_t = 2;
pub const RES_NOMATCH: result_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfat_state {
    pub name: [wchar_t; 261],
    pub status: libc::c_int,
    pub subentries: libc::c_uint,
    pub sum: libc::c_uchar,
    pub present: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfat_subentry {
    pub id: libc::c_uchar,
    pub text1: [libc::c_uchar; 10],
    pub attribute: libc::c_uchar,
    pub hash1: libc::c_uchar,
    pub sum: libc::c_uchar,
    pub text2: [libc::c_uchar; 12],
    pub sector_l: libc::c_uchar,
    pub sector_u: libc::c_uchar,
    pub text3: [libc::c_uchar; 4],
}
pub static mut short_illegals: *const libc::c_char = b";+=[]',\"*\\<>/?:|\0" as *const u8
    as *const libc::c_char;
pub static mut long_illegals: *const libc::c_char = b"\"*\\<>/?:|\x05\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn autorename(
    mut name: *mut libc::c_char,
    mut tilda: libc::c_char,
    mut dot: libc::c_char,
    mut illegals: *const libc::c_char,
    mut limit: libc::c_int,
    mut bump: libc::c_int,
) {
    let mut tildapos: libc::c_int = 0;
    let mut dotpos: libc::c_int = 0;
    let mut seqnum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut maxseq: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tmp: libc::c_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    tildapos = -(1 as libc::c_int);
    p = name;
    while *p != 0 {
        if !(strchr(illegals, *p as libc::c_int)).is_null() {
            *p = '_' as i32 as libc::c_char;
            bump = 0 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    dotpos = 0 as libc::c_int;
    while *name.offset(dotpos as isize) as libc::c_int != 0 && dotpos < limit
        && *name.offset(dotpos as isize) as libc::c_int != dot as libc::c_int
    {
        if *name.offset(dotpos as isize) as libc::c_int == tilda as libc::c_int {
            tildapos = dotpos;
            seqnum = 0 as libc::c_int as libc::c_uint;
            maxseq = 1 as libc::c_int as libc::c_uint;
        } else if *name.offset(dotpos as isize) as libc::c_int >= '0' as i32
            && *name.offset(dotpos as isize) as libc::c_int <= '9' as i32
        {
            seqnum = seqnum
                .wrapping_mul(10 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (*name.offset(dotpos as isize) as libc::c_int - '0' as i32)
                        as uint8_t as libc::c_uint,
                );
            maxseq = maxseq.wrapping_mul(10 as libc::c_int as libc::c_uint);
        } else {
            tildapos = -(1 as libc::c_int);
        }
        dotpos += 1;
        dotpos;
    }
    if tildapos == -(1 as libc::c_int) {
        if dotpos > limit - 2 as libc::c_int {
            tildapos = limit - 2 as libc::c_int;
            dotpos = limit;
        } else {
            tildapos = dotpos;
            dotpos += 2 as libc::c_int;
        }
        seqnum = 1 as libc::c_int as libc::c_uint;
    } else {
        if bump != 0 {
            seqnum = seqnum.wrapping_add(1);
            seqnum;
        }
        if seqnum > 999999 as libc::c_int as libc::c_uint {
            seqnum = 1 as libc::c_int as libc::c_uint;
            tildapos = dotpos - 2 as libc::c_int;
        }
        if seqnum == maxseq {
            if dotpos >= limit {
                tildapos -= 1;
                tildapos;
            } else {
                dotpos += 1;
                dotpos;
            }
        }
    }
    tmp = *name.offset(dotpos as isize);
    if bump != 0 && seqnum == 1 as libc::c_int as libc::c_uint
        || seqnum > 1 as libc::c_int as libc::c_uint || mtools_numeric_tail != 0
    {
        sprintf(
            name.offset(tildapos as isize),
            b"%c%d\0" as *const u8 as *const libc::c_char,
            tilda as libc::c_int,
            seqnum,
        );
    }
    if dot != 0 {
        *name.offset(dotpos as isize) = tmp;
    }
}
pub unsafe extern "C" fn autorename_short(
    mut name: *mut dos_name_t,
    mut bump: libc::c_int,
) {
    autorename(
        ((*name).base).as_mut_ptr(),
        '~' as i32 as libc::c_char,
        ' ' as i32 as libc::c_char,
        short_illegals,
        8 as libc::c_int,
        bump,
    );
}
pub unsafe extern "C" fn autorename_long(
    mut name: *mut libc::c_char,
    mut bump: libc::c_int,
) {
    autorename(
        name,
        '-' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
        long_illegals,
        255 as libc::c_int,
        bump,
    );
}
unsafe extern "C" fn unicode_write(
    mut in_0: *mut wchar_t,
    mut out: *mut libc::c_uchar,
    mut num: libc::c_int,
    mut end_p: *mut libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < num {
        if *end_p != 0 {
            let ref mut fresh0 = *out.offset(1 as libc::c_int as isize);
            *fresh0 = 0xff as libc::c_int as libc::c_uchar;
            *out.offset(0 as libc::c_int as isize) = *fresh0;
        } else {
            *out
                .offset(
                    1 as libc::c_int as isize,
                ) = ((*in_0 & 0xffff as libc::c_int) >> 8 as libc::c_int)
                as libc::c_uchar;
            *out
                .offset(
                    0 as libc::c_int as isize,
                ) = (*in_0 & 0xff as libc::c_int) as libc::c_uchar;
            if *in_0 == 0 {
                *end_p = 0x40 as libc::c_int;
            }
        }
        out = out.offset(1);
        out;
        out = out.offset(1);
        out;
        in_0 = in_0.offset(1);
        in_0;
        j += 1;
        j;
    }
    return num;
}
#[inline]
unsafe extern "C" fn unicode_read(
    mut in_0: *mut libc::c_uchar,
    mut out: *mut wchar_t,
    mut num: libc::c_int,
) -> libc::c_int {
    let mut end_out: *mut wchar_t = out.offset(num as isize);
    while out < end_out {
        *out = *in_0.offset(0 as libc::c_int as isize) as libc::c_int
            | (*in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int;
        out = out.offset(1);
        out;
        in_0 = in_0.offset(1);
        in_0;
        in_0 = in_0.offset(1);
        in_0;
    }
    return num;
}
unsafe extern "C" fn clear_vfat(mut v: *mut vfat_state) {
    (*v).subentries = 0 as libc::c_int as libc::c_uint;
    (*v).status = 0 as libc::c_int;
    (*v).present = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn sum_shortname(mut dn: *const dos_name_t) -> libc::c_uchar {
    let mut sum: libc::c_uchar = 0;
    let mut name: *const libc::c_char = ((*dn).base).as_ptr();
    let mut end: *const libc::c_char = name.offset(11 as libc::c_int as isize);
    sum = 0 as libc::c_int as libc::c_uchar;
    while name < end {
        sum = ((if sum as libc::c_int & 1 as libc::c_int != 0 {
            0x80 as libc::c_int
        } else {
            0 as libc::c_int
        }) + (sum as libc::c_int >> 1 as libc::c_int) + *name as uint8_t as libc::c_int)
            as libc::c_uchar;
        name = name.offset(1);
        name;
    }
    return sum;
}
#[inline]
unsafe extern "C" fn check_vfat(mut v: *mut vfat_state, mut dir: *mut directory) {
    let mut dn: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    if (*v).subentries == 0 {
        return;
    }
    memcpy(
        (dn.base).as_mut_ptr() as *mut libc::c_void,
        ((*dir).name).as_mut_ptr() as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        (dn.ext).as_mut_ptr() as *mut libc::c_void,
        ((*dir).ext).as_mut_ptr() as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    );
    if (*v).sum as libc::c_int != sum_shortname(&mut dn) as libc::c_int {
        return;
    }
    if (*v).status & ((1 as libc::c_int) << (*v).subentries) - 1 as libc::c_int
        != ((1 as libc::c_int) << (*v).subentries) - 1 as libc::c_int
    {
        return;
    }
    (*v)
        .name[(13 as libc::c_int as libc::c_uint).wrapping_mul((*v).subentries)
        as usize] = 0 as libc::c_int;
    (*v).present = 1 as libc::c_int;
}
pub unsafe extern "C" fn write_vfat(
    mut Dir: *mut Stream_t,
    mut shortname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut start: libc::c_uint,
    mut mainEntry: *mut direntry_t,
) -> libc::c_uint {
    let mut vse: *mut vfat_subentry = 0 as *mut vfat_subentry;
    let mut vse_id: uint8_t = 0;
    let mut num_vses: uint8_t = 0;
    let mut c: *mut wchar_t = 0 as *mut wchar_t;
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
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut unixyName: [wchar_t; 13] = [0; 13];
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert).unwrap()(Dir);
    let mut wlongname: [wchar_t; 256] = [0; 256];
    let mut wlen: size_t = 0;
    if !longname.is_null() {
        entry.Dir = Dir;
        vse = &mut entry.dir as *mut directory as *mut vfat_subentry;
        (*vse).attribute = 0xf as libc::c_int as libc::c_uchar;
        (*vse).sector_u = 0 as libc::c_int as libc::c_uchar;
        (*vse).sector_l = (*vse).sector_u;
        (*vse).hash1 = (*vse).sector_l;
        (*vse).sum = sum_shortname(shortname);
        wlen = native_to_wchar(
            longname,
            wlongname.as_mut_ptr(),
            (255 as libc::c_int + 1 as libc::c_int) as size_t,
            0 as *const libc::c_char,
            0 as *mut libc::c_int,
        );
        num_vses = wlen
            .wrapping_add(13 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(13 as libc::c_int as libc::c_ulong) as uint8_t;
        vse_id = num_vses;
        while vse_id != 0 {
            let mut end: libc::c_int = 0 as libc::c_int;
            c = wlongname
                .as_mut_ptr()
                .offset(
                    ((vse_id as libc::c_int - 1 as libc::c_int) * 13 as libc::c_int)
                        as isize,
                );
            c = c
                .offset(
                    unicode_write(
                        c,
                        ((*vse).text1).as_mut_ptr(),
                        5 as libc::c_int,
                        &mut end,
                    ) as isize,
                );
            c = c
                .offset(
                    unicode_write(
                        c,
                        ((*vse).text2).as_mut_ptr(),
                        6 as libc::c_int,
                        &mut end,
                    ) as isize,
                );
            c = c
                .offset(
                    unicode_write(
                        c,
                        ((*vse).text3).as_mut_ptr(),
                        2 as libc::c_int,
                        &mut end,
                    ) as isize,
                );
            (*vse)
                .id = (if vse_id as libc::c_int == num_vses as libc::c_int {
                vse_id as libc::c_int | 0x40 as libc::c_int
            } else {
                vse_id as libc::c_int
            }) as libc::c_uchar;
            setEntryToPos(
                &mut entry,
                start
                    .wrapping_add(num_vses as libc::c_uint)
                    .wrapping_sub(vse_id as libc::c_uint),
            );
            low_level_dir_write(&mut entry);
            vse_id = vse_id.wrapping_sub(1);
            vse_id;
        }
    } else {
        num_vses = 0 as libc::c_int as uint8_t;
        wlongname[0 as libc::c_int as usize] = '\0' as i32;
    }
    cache = allocDirCache(
        Dir,
        start
            .wrapping_add(num_vses as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    if cache.is_null() {
        fprintf(stderr, b"Out of memory error\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    unix_name(
        cp,
        ((*shortname).base).as_mut_ptr(),
        ((*shortname).ext).as_mut_ptr(),
        0 as libc::c_int as uint8_t,
        unixyName.as_mut_ptr(),
    );
    addUsedEntry(
        cache,
        start,
        start
            .wrapping_add(num_vses as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
        wlongname.as_mut_ptr(),
        unixyName.as_mut_ptr(),
        &mut (*mainEntry).dir,
    );
    low_level_dir_write(mainEntry);
    return start.wrapping_add(num_vses as libc::c_uint);
}
pub unsafe extern "C" fn dir_write(mut entry: *mut direntry_t) {
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    if isRootEntry(entry) != 0 {
        fprintf(
            stderr,
            b"Attempt to write root directory pointer\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    cache = allocDirCache((*entry).Dir, getNextEntryAsPos(entry));
    if cache.is_null() {
        fprintf(
            stderr,
            b"Out of memory error in dir_write\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    dce = *((*cache).entries).offset((*entry).entry as isize);
    if !dce.is_null() {
        if (*entry).dir.name[0 as libc::c_int as usize] as libc::c_int
            == 0xe5 as libc::c_int as libc::c_char as libc::c_int
        {
            addFreeEntry(cache, (*dce).beginSlot, (*dce).endSlot);
        } else {
            (*dce).dir = (*entry).dir;
        }
    }
    low_level_dir_write(entry);
}
#[inline]
unsafe extern "C" fn parse_vses(mut entry: *mut direntry_t, mut v: *mut vfat_state) {
    let mut vse: *mut vfat_subentry = 0 as *mut vfat_subentry;
    let mut id: libc::c_uchar = 0;
    let mut last_flag: libc::c_uchar = 0;
    let mut c: *mut wchar_t = 0 as *mut wchar_t;
    vse = &mut (*entry).dir as *mut directory as *mut vfat_subentry;
    id = ((*vse).id as libc::c_int & 0x1f as libc::c_int) as libc::c_uchar;
    last_flag = ((*vse).id as libc::c_int & 0x40 as libc::c_int) as libc::c_uchar;
    if id as libc::c_int > 20 as libc::c_int {
        fprintf(
            stderr,
            b"parse_vses: invalid VSE ID %d at %d.\n\0" as *const u8
                as *const libc::c_char,
            id as libc::c_int,
            (*entry).entry,
        );
        return;
    }
    if (*v).sum as libc::c_int != (*vse).sum as libc::c_int {
        clear_vfat(v);
        (*v).sum = (*vse).sum;
    }
    (*v).status |= (1 as libc::c_int) << id as libc::c_int - 1 as libc::c_int;
    if last_flag != 0 {
        (*v).subentries = id as libc::c_uint;
    }
    c = &mut *((*v).name)
        .as_mut_ptr()
        .offset((13 as libc::c_int * (id as libc::c_int - 1 as libc::c_int)) as isize)
        as *mut wchar_t;
    c = c
        .offset(unicode_read(((*vse).text1).as_mut_ptr(), c, 5 as libc::c_int) as isize);
    c = c
        .offset(unicode_read(((*vse).text2).as_mut_ptr(), c, 6 as libc::c_int) as isize);
    c = c
        .offset(unicode_read(((*vse).text3).as_mut_ptr(), c, 2 as libc::c_int) as isize);
    if last_flag != 0 {
        *c = '\0' as i32;
    }
}
unsafe extern "C" fn vfat_lookup_loop_common(
    mut cp: *mut doscp_t,
    mut direntry: *mut direntry_t,
    mut cache: *mut dirCache_t,
    mut lookForFreeSpace: libc::c_int,
    mut io_error: *mut libc::c_int,
) -> *mut dirCacheEntry_t {
    let mut newfile: [wchar_t; 13] = [0; 13];
    let mut initpos: libc::c_uint = getNextEntryAsPos(direntry);
    let mut vfat: vfat_state = vfat_state {
        name: [0; 261],
        status: 0,
        subentries: 0,
        sum: 0,
        present: 0,
    };
    let mut longname: *mut wchar_t = 0 as *mut wchar_t;
    let mut error: libc::c_int = 0;
    let mut endmarkSeen: libc::c_int = 0 as libc::c_int;
    *io_error = 0 as libc::c_int;
    clear_vfat(&mut vfat);
    loop {
        (*direntry).entry += 1;
        (*direntry).entry;
        if (dir_read(direntry, &mut error)).is_null() {
            if error != 0 {
                *io_error = error;
                return 0 as *mut dirCacheEntry_t;
            }
            addFreeEndEntry(cache, initpos, getEntryAsPos(direntry), endmarkSeen);
            return addEndEntry(cache, getEntryAsPos(direntry));
        }
        if endmarkSeen != 0
            || (*direntry).dir.name[0 as libc::c_int as usize] as libc::c_int
                == 0 as libc::c_int as libc::c_char as libc::c_int
        {
            if lookForFreeSpace != 0 {
                endmarkSeen = 1 as libc::c_int;
            } else {
                return addEndEntry(cache, getEntryAsPos(direntry))
            }
        } else {
            if !((*direntry).dir.name[0 as libc::c_int as usize] as libc::c_int
                != 0xe5 as libc::c_int as libc::c_char as libc::c_int
                && (*direntry).dir.attr as libc::c_int == 0xf as libc::c_int)
            {
                break;
            }
            parse_vses(direntry, &mut vfat);
        }
    }
    if (*direntry).dir.name[0 as libc::c_int as usize] as libc::c_int
        == 0xe5 as libc::c_int as libc::c_char as libc::c_int
    {
        return addFreeEntry(cache, initpos, getNextEntryAsPos(direntry));
    }
    check_vfat(&mut vfat, &mut (*direntry).dir);
    if vfat.present == 0 {
        vfat.subentries = 0 as libc::c_int as libc::c_uint;
    }
    addFreeEntry(
        cache,
        initpos,
        (getEntryAsPos(direntry)).wrapping_sub(vfat.subentries),
    );
    if (*direntry).dir.attr as libc::c_int & 0x8 as libc::c_int != 0 {
        let mut ptr: *mut wchar_t = newfile.as_mut_ptr();
        if (*direntry).dir.name[0 as libc::c_int as usize] as libc::c_int
            == '\u{5}' as i32
        {
            ptr = ptr
                .offset(
                    dos_to_wchar(
                        cp,
                        b"\xE5\0" as *const u8 as *const libc::c_char,
                        ptr,
                        1 as libc::c_int as size_t,
                    ) as isize,
                );
            ptr = ptr
                .offset(
                    dos_to_wchar(
                        cp,
                        ((*direntry).dir.name)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize),
                        ptr,
                        7 as libc::c_int as size_t,
                    ) as isize,
                );
        } else {
            ptr = ptr
                .offset(
                    dos_to_wchar(
                        cp,
                        ((*direntry).dir.name).as_mut_ptr(),
                        ptr,
                        8 as libc::c_int as size_t,
                    ) as isize,
                );
        }
        ptr = ptr
            .offset(
                dos_to_wchar(
                    cp,
                    ((*direntry).dir.ext).as_mut_ptr(),
                    ptr,
                    3 as libc::c_int as size_t,
                ) as isize,
            );
        *ptr = '\0' as i32;
    } else {
        unix_name(
            cp,
            ((*direntry).dir.name).as_mut_ptr(),
            ((*direntry).dir.ext).as_mut_ptr(),
            (*direntry).dir.Case,
            newfile.as_mut_ptr(),
        );
    }
    if vfat.present != 0 {
        longname = (vfat.name).as_mut_ptr();
    } else {
        longname = 0 as *mut wchar_t;
    }
    return addUsedEntry(
        cache,
        (getEntryAsPos(direntry)).wrapping_sub(vfat.subentries),
        getNextEntryAsPos(direntry),
        longname,
        newfile.as_mut_ptr(),
        &mut (*direntry).dir,
    );
}
#[inline]
unsafe extern "C" fn vfat_lookup_loop_for_read(
    mut cp: *mut doscp_t,
    mut direntry: *mut direntry_t,
    mut cache: *mut dirCache_t,
    mut io_error: *mut libc::c_int,
) -> *mut dirCacheEntry_t {
    let mut initpos: libc::c_int = (*direntry).entry + 1 as libc::c_int;
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    *io_error = 0 as libc::c_int;
    dce = *((*cache).entries).offset(initpos as isize);
    if !dce.is_null() {
        setEntryToPos(
            direntry,
            ((*dce).endSlot).wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        return dce;
    } else {
        return vfat_lookup_loop_common(cp, direntry, cache, 0 as libc::c_int, io_error)
    };
}
unsafe extern "C" fn checkNameForMatch(
    mut direntry: *mut direntry_t,
    mut dce: *mut dirCacheEntry_t,
    mut filename: *const wchar_t,
    mut length: libc::c_int,
    mut flags: libc::c_int,
) -> result_t {
    match (*dce).type_0 as libc::c_uint {
        0 => return RES_NOMATCH,
        2 => return RES_END,
        1 | _ => {}
    }
    (*direntry).dir = (*dce).dir;
    if (*direntry).dir.attr as libc::c_int & 0x8 as libc::c_int != 0
        && flags & 0x8 as libc::c_int == 0
    {
        return RES_NOMATCH;
    }
    if !(flags & 0x40 as libc::c_int != 0
        || !((*dce).longName).is_null()
            && match_0(
                (*dce).longName,
                filename,
                ((*direntry).name).as_mut_ptr(),
                0 as libc::c_int,
                length,
            ) != 0
        || match_0(
            (*dce).shortName,
            filename,
            ((*direntry).name).as_mut_ptr(),
            1 as libc::c_int,
            length,
        ) != 0)
    {
        return RES_NOMATCH;
    }
    if (*direntry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0
        && flags & 0x10 as libc::c_int == 0
    {
        if flags & (0x8 as libc::c_int | 0x40 as libc::c_int | 0x80 as libc::c_int) == 0
        {
            let mut tmp: [libc::c_char; 53] = [0; 53];
            wchar_to_native(
                (*dce).shortName,
                tmp.as_mut_ptr(),
                13 as libc::c_int as size_t,
                ::std::mem::size_of::<[libc::c_char; 53]>() as libc::c_ulong,
            );
            fprintf(
                stderr,
                b"Skipping \"%s\", is a directory\n\0" as *const u8
                    as *const libc::c_char,
                tmp.as_mut_ptr(),
            );
        }
        return RES_NOMATCH;
    }
    if (*direntry).dir.attr as libc::c_int & (0x8 as libc::c_int | 0x10 as libc::c_int)
        == 0 && flags & 0x20 as libc::c_int == 0
    {
        if flags & (0x8 as libc::c_int | 0x40 as libc::c_int | 0x80 as libc::c_int) == 0
        {
            let mut tmp_0: [libc::c_char; 53] = [0; 53];
            wchar_to_native(
                (*dce).shortName,
                tmp_0.as_mut_ptr(),
                13 as libc::c_int as size_t,
                ::std::mem::size_of::<[libc::c_char; 53]>() as libc::c_ulong,
            );
            fprintf(
                stderr,
                b"Skipping \"%s\", is not a directory\n\0" as *const u8
                    as *const libc::c_char,
                tmp_0.as_mut_ptr(),
            );
        }
        return RES_NOMATCH;
    }
    return RES_MATCH;
}
pub unsafe extern "C" fn vfat_lookup_zt(
    mut direntry: *mut direntry_t,
    mut filename: *const libc::c_char,
    mut flags: libc::c_int,
    mut shortname: *mut libc::c_char,
    mut shortname_size: size_t,
    mut longname: *mut libc::c_char,
    mut longname_size: size_t,
) -> libc::c_int {
    return vfat_lookup(
        direntry,
        filename,
        strlen(filename),
        flags,
        shortname,
        shortname_size,
        longname,
        longname_size,
    );
}
pub unsafe extern "C" fn vfat_lookup(
    mut direntry: *mut direntry_t,
    mut filename: *const libc::c_char,
    mut length: size_t,
    mut flags: libc::c_int,
    mut shortname: *mut libc::c_char,
    mut shortname_size: size_t,
    mut longname: *mut libc::c_char,
    mut longname_size: size_t,
) -> libc::c_int {
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut result: result_t = RES_NOMATCH;
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut io_error: libc::c_int = 0;
    let mut wfilename: [wchar_t; 256] = [0; 256];
    let mut cp: *mut doscp_t = ((*(*(*direntry).Dir).Class).get_dosConvert)
        .unwrap()((*direntry).Dir);
    if !filename.is_null() {
        length = native_to_wchar(
            filename,
            wfilename.as_mut_ptr(),
            255 as libc::c_int as size_t,
            filename.offset(length as isize),
            0 as *mut libc::c_int,
        );
    } else {
        length = 0 as libc::c_int as size_t;
    }
    if isNotFound(direntry) != 0 {
        return -(1 as libc::c_int);
    }
    cache = allocDirCache((*direntry).Dir, getNextEntryAsPos(direntry));
    if cache.is_null() {
        fprintf(
            stderr,
            b"Out of memory error in vfat_lookup [0]\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    loop {
        dce = vfat_lookup_loop_for_read(cp, direntry, cache, &mut io_error);
        if dce.is_null() {
            if io_error != 0 {
                return -(2 as libc::c_int);
            }
            fprintf(
                stderr,
                b"Out of memory error in vfat_lookup\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        result = checkNameForMatch(
            direntry,
            dce,
            wfilename.as_mut_ptr(),
            length as libc::c_int,
            flags,
        );
        if !(result as libc::c_uint == RES_NOMATCH as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if result as libc::c_uint == RES_MATCH as libc::c_int as libc::c_uint {
        if !longname.is_null() {
            if !((*dce).longName).is_null() {
                wchar_to_native(
                    (*dce).longName,
                    longname,
                    255 as libc::c_int as size_t,
                    longname_size,
                );
            } else {
                *longname = '\0' as i32 as libc::c_char;
            }
        }
        if !shortname.is_null() {
            wchar_to_native(
                (*dce).shortName,
                shortname,
                12 as libc::c_int as size_t,
                shortname_size,
            );
        }
        (*direntry).beginSlot = (*dce).beginSlot;
        (*direntry)
            .endSlot = ((*dce).endSlot).wrapping_sub(1 as libc::c_int as libc::c_uint);
        return 0 as libc::c_int;
    } else {
        (*direntry).entry = -(2 as libc::c_int);
        return -(1 as libc::c_int);
    };
}
#[inline]
unsafe extern "C" fn vfat_lookup_loop_for_insert(
    mut cp: *mut doscp_t,
    mut direntry: *mut direntry_t,
    mut initpos: libc::c_uint,
    mut cache: *mut dirCache_t,
) -> *mut dirCacheEntry_t {
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut io_error: libc::c_int = 0;
    dce = *((*cache).entries).offset(initpos as isize);
    if !dce.is_null()
        && (*dce).type_0 as libc::c_uint != DCET_END as libc::c_int as libc::c_uint
    {
        return dce
    } else {
        setEntryForIteration(direntry, initpos);
        dce = vfat_lookup_loop_common(
            cp,
            direntry,
            cache,
            1 as libc::c_int,
            &mut io_error,
        );
        if dce.is_null() {
            if io_error != 0 {
                return 0 as *mut dirCacheEntry_t;
            }
            fprintf(
                stderr,
                b"Out of memory error in vfat_lookup_loop\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        return *((*cache).entries).offset(initpos as isize);
    };
}
unsafe extern "C" fn accountFreeSlots(
    mut ssp: *mut scan_state,
    mut dce: *mut dirCacheEntry_t,
) {
    if (*ssp).got_slots != 0 {
        return;
    }
    if (*ssp).free_end != (*dce).beginSlot {
        (*ssp).free_start = (*dce).beginSlot;
    }
    (*ssp).free_end = (*dce).endSlot;
    if ((*ssp).free_end).wrapping_sub((*ssp).free_start) >= (*ssp).size_needed {
        (*ssp).got_slots = 1 as libc::c_int;
        (*ssp)
            .slot = ((*ssp).free_start)
            .wrapping_add((*ssp).size_needed)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn clear_scan(
    mut longname: *mut wchar_t,
    mut use_longname: libc::c_int,
    mut s: *mut scan_state,
) {
    (*s).longmatch = -(1 as libc::c_int);
    (*s).shortmatch = (*s).longmatch;
    (*s).free_start = 0 as libc::c_int as libc::c_uint;
    (*s).got_slots = (*s).free_start as libc::c_int;
    (*s).free_end = (*s).got_slots as libc::c_uint;
    if use_longname & 1 as libc::c_int != 0 {
        (*s)
            .size_needed = (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                (wcslen(longname))
                    .wrapping_add(13 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(13 as libc::c_int as libc::c_ulong),
            ) as libc::c_uint;
    } else {
        (*s).size_needed = 1 as libc::c_int as libc::c_uint;
    };
}
pub unsafe extern "C" fn lookupForInsert(
    mut Dir: *mut Stream_t,
    mut direntry: *mut direntry_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut libc::c_char,
    mut ssp: *mut scan_state,
    mut ignore_entry: libc::c_int,
    mut source_entry: libc::c_int,
    mut pessimisticShortRename: libc::c_int,
    mut use_longname: libc::c_int,
) -> libc::c_int {
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
    let mut ignore_match: libc::c_int = 0;
    let mut dce: *mut dirCacheEntry_t = 0 as *mut dirCacheEntry_t;
    let mut cache: *mut dirCache_t = 0 as *mut dirCache_t;
    let mut pos: libc::c_uint = 0;
    let mut shortName: [wchar_t; 13] = [0; 13];
    let mut wlongname: [wchar_t; 256] = [0; 256];
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert).unwrap()(Dir);
    native_to_wchar(
        longname,
        wlongname.as_mut_ptr(),
        (255 as libc::c_int + 1 as libc::c_int) as size_t,
        0 as *const libc::c_char,
        0 as *mut libc::c_int,
    );
    clear_scan(wlongname.as_mut_ptr(), use_longname, ssp);
    ignore_match = (ignore_entry == -(2 as libc::c_int)) as libc::c_int;
    initializeDirentry(&mut entry, Dir);
    (*ssp).match_free = 0 as libc::c_int;
    cache = allocDirCache(Dir, 1 as libc::c_int as libc::c_uint);
    if cache.is_null() {
        fprintf(
            stderr,
            b"Out of memory error in lookupForInsert\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if ignore_match == 0 {
        unix_name(
            cp,
            ((*dosname).base).as_mut_ptr(),
            ((*dosname).ext).as_mut_ptr(),
            0 as libc::c_int as uint8_t,
            shortName.as_mut_ptr(),
        );
    }
    pos = (*cache).nrHashed;
    if source_entry >= 0 as libc::c_int
        || pos != 0 && isHashed(cache, wlongname.as_mut_ptr()) != 0
    {
        pos = 0 as libc::c_int as libc::c_uint;
    } else if pos != 0 && ignore_match == 0
        && isHashed(cache, shortName.as_mut_ptr()) != 0
    {
        if pessimisticShortRename != 0 {
            (*ssp).shortmatch = -(2 as libc::c_int);
            return 1 as libc::c_int;
        }
        pos = 0 as libc::c_int as libc::c_uint;
    } else if growDirCache(cache, pos) < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Out of memory error in vfat_looup [0]\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    loop {
        dce = vfat_lookup_loop_for_insert(cp, &mut entry, pos, cache);
        match (*dce).type_0 as libc::c_uint {
            0 => {
                accountFreeSlots(ssp, dce);
            }
            1 => {
                if (*dce).dir.attr as libc::c_int & 0x8 as libc::c_int == 0
                    && (*dce).endSlot as libc::c_int - 1 as libc::c_int == source_entry
                {
                    accountFreeSlots(ssp, dce);
                }
                if !((*dce).dir.attr as libc::c_int & 0x8 as libc::c_int != 0
                    || (*dce).endSlot as libc::c_int - 1 as libc::c_int == ignore_entry)
                {
                    if !((*dce).longName).is_null()
                        && wcscasecmp((*dce).longName, wlongname.as_mut_ptr()) == 0
                        || !((*dce).shortName).is_null()
                            && wcscasecmp((*dce).shortName, wlongname.as_mut_ptr()) == 0
                    {
                        (*ssp)
                            .longmatch = ((*dce).endSlot)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_int;
                        (*direntry).beginSlot = (*dce).beginSlot;
                        (*direntry)
                            .endSlot = ((*dce).endSlot)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint);
                        return 1 as libc::c_int;
                    }
                    if ignore_match == 0
                        && wcscasecmp(shortName.as_mut_ptr(), (*dce).shortName) == 0
                    {
                        (*ssp)
                            .shortmatch = ((*dce).endSlot)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_int;
                    }
                }
            }
            2 | _ => {}
        }
        pos = (*dce).endSlot;
        if !((*dce).type_0 as libc::c_uint != DCET_END as libc::c_int as libc::c_uint) {
            break;
        }
    }
    if (*ssp).shortmatch > -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    (*ssp).max_entry = (*dce).beginSlot;
    if (*ssp).got_slots != 0 {
        return 6 as libc::c_int;
    }
    if isRootDir(Dir) == 0 {
        return 5 as libc::c_int;
    }
    fprintf(stderr, b"No directory slots\n\0" as *const u8 as *const libc::c_char);
    return -(1 as libc::c_int);
}
