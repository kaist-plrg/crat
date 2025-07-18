use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    pub type FatMap_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn fatDeallocate(This: *mut Fs_t, pos: libc::c_uint);
    fn fatDecode(This: *mut Fs_t, pos: libc::c_uint) -> libc::c_uint;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
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
pub type fatAccessMode_t = libc::c_uint;
pub const FAT_ACCESS_WRITE: fatAccessMode_t = 1;
pub const FAT_ACCESS_READ: fatAccessMode_t = 0;
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
pub unsafe extern "C" fn fat_free(
    mut Dir: *mut Stream_t,
    mut fat: libc::c_uint,
) -> libc::c_int {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut next_no_step: libc::c_uint = 0;
    if fat == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    while (*This).fat_error == 0 {
        next_no_step = fatDecode(This, fat);
        fatDeallocate(This, fat);
        if next_no_step >= (*This).last_fat {
            break;
        }
        fat = next_no_step;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fatFreeWithDir(
    mut Dir: *mut Stream_t,
    mut dir: *mut directory,
) -> libc::c_int {
    let mut first: uint32_t = 0;
    if (strncmp(
        ((*dir).name).as_mut_ptr(),
        b".      \0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as libc::c_ulong,
    ) == 0
        || strncmp(
            ((*dir).name).as_mut_ptr(),
            b"..     \0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0)
        && strncmp(
            ((*dir).ext).as_mut_ptr(),
            b"   \0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        fprintf(
            stderr,
            b"Trying to remove . or .. entry\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    first = ((*dir).start[0 as libc::c_int as usize] as libc::c_int
        + (((*dir).start[1 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int))
        as uint16_t as uint32_t;
    if fat32RootCluster(Dir) != 0 {
        first
            |= (((*dir).startHi[0 as libc::c_int as usize] as libc::c_int
                + (((*dir).startHi[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as uint32_t) << 16 as libc::c_int;
    }
    return fat_free(Dir, first);
}
pub unsafe extern "C" fn fatFreeWithDirentry(mut entry: *mut direntry_t) -> libc::c_int {
    return fatFreeWithDir((*entry).Dir, &mut (*entry).dir);
}
