use ::libc;
extern "C" {
    pub type doscp_t;
    pub type Fs_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn perror(__s: *const libc::c_char);
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn getfreeMinClusters(Stream: *mut Stream_t, ref_0: uint32_t) -> libc::c_int;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn dosnameToDirentry(n: *const dos_name_t, dir: *mut directory);
    fn getClusterBytes(File: *mut Fs_t) -> uint32_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
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
pub struct dos_name_t {
    pub base: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub sentinel: libc::c_char,
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
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: libc::c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
}
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
pub unsafe extern "C" fn dir_read(
    mut entry: *mut direntry_t,
    mut error: *mut libc::c_int,
) -> *mut directory {
    let mut n: ssize_t = 0;
    *error = 0 as libc::c_int;
    n = force_pread(
        (*entry).Dir,
        &mut (*entry).dir as *mut directory as *mut libc::c_char,
        (*entry).entry as mt_off_t * 32 as libc::c_int as libc::c_long,
        32 as libc::c_int as size_t,
    );
    if n != 32 as libc::c_int as libc::c_long {
        if n < 0 as libc::c_int as libc::c_long {
            *error = -(1 as libc::c_int);
        }
        return 0 as *mut directory;
    }
    return &mut (*entry).dir;
}
pub unsafe extern "C" fn dir_grow(
    mut Dir: *mut Stream_t,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut Stream: *mut Stream_t = GetFs(Dir);
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut ret: ssize_t = 0;
    let mut buflen: libc::c_uint = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    if getfreeMinClusters(Dir, 1 as libc::c_int as uint32_t) == 0 {
        return -(1 as libc::c_int);
    }
    buflen = getClusterBytes(This);
    buffer = malloc(buflen as libc::c_ulong) as *mut libc::c_char;
    if buffer.is_null() {
        perror(b"dir_grow: malloc\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(buffer as *mut libc::c_void, '\0' as i32, buflen as libc::c_ulong);
    ret = force_pwrite(
        Dir,
        buffer,
        size as mt_off_t * 32 as libc::c_int as libc::c_long,
        buflen as size_t,
    );
    free(buffer as *mut libc::c_void);
    if ret < buflen as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn low_level_dir_write(mut entry: *mut direntry_t) {
    force_pwrite(
        (*entry).Dir,
        &mut (*entry).dir as *mut directory as *mut libc::c_char,
        (*entry).entry as mt_off_t * 32 as libc::c_int as libc::c_long,
        32 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn low_level_dir_write_end(
    mut Dir: *mut Stream_t,
    mut entry: libc::c_int,
) {
    let mut zero: libc::c_char = 0 as libc::c_int as libc::c_char;
    force_pwrite(
        Dir,
        &mut zero,
        entry as mt_off_t * 32 as libc::c_int as libc::c_long,
        1 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn mk_entry(
    mut dn: *const dos_name_t,
    mut attr: libc::c_uchar,
    mut fat: libc::c_uint,
    mut size: uint32_t,
    mut date: time_t,
    mut ndir: *mut directory,
) -> *mut directory {
    let mut now: *mut tm = 0 as *mut tm;
    let mut date2: time_t = date;
    let mut hour: uint8_t = 0;
    let mut min_hi: uint8_t = 0;
    let mut min_low: uint8_t = 0;
    let mut sec: uint8_t = 0;
    let mut year: uint8_t = 0;
    let mut month_hi: uint8_t = 0;
    let mut month_low: uint8_t = 0;
    let mut day: uint8_t = 0;
    now = localtime(&mut date2);
    dosnameToDirentry(dn, ndir);
    (*ndir).attr = attr;
    (*ndir).ctime_ms = 0 as libc::c_int as libc::c_uchar;
    hour = ((*now).tm_hour << 3 as libc::c_int) as uint8_t;
    min_hi = ((*now).tm_min >> 3 as libc::c_int) as uint8_t;
    min_low = ((*now).tm_min << 5 as libc::c_int) as uint8_t;
    sec = ((*now).tm_sec / 2 as libc::c_int) as uint8_t;
    (*ndir)
        .time[1 as libc::c_int
        as usize] = (hour as libc::c_int + min_hi as libc::c_int) as libc::c_uchar;
    (*ndir).ctime[1 as libc::c_int as usize] = (*ndir).time[1 as libc::c_int as usize];
    (*ndir)
        .time[0 as libc::c_int
        as usize] = (min_low as libc::c_int + sec as libc::c_int) as libc::c_uchar;
    (*ndir).ctime[0 as libc::c_int as usize] = (*ndir).time[0 as libc::c_int as usize];
    year = (((*now).tm_year - 80 as libc::c_int) << 1 as libc::c_int) as uint8_t;
    month_hi = ((*now).tm_mon + 1 as libc::c_int >> 3 as libc::c_int) as uint8_t;
    month_low = (((*now).tm_mon + 1 as libc::c_int) << 5 as libc::c_int) as uint8_t;
    day = (*now).tm_mday as uint8_t;
    (*ndir)
        .date[1 as libc::c_int
        as usize] = (year as libc::c_int + month_hi as libc::c_int) as libc::c_uchar;
    (*ndir).cdate[1 as libc::c_int as usize] = (*ndir).date[1 as libc::c_int as usize];
    (*ndir).adate[1 as libc::c_int as usize] = (*ndir).cdate[1 as libc::c_int as usize];
    (*ndir)
        .date[0 as libc::c_int
        as usize] = (month_low as libc::c_int + day as libc::c_int) as libc::c_uchar;
    (*ndir).cdate[0 as libc::c_int as usize] = (*ndir).date[0 as libc::c_int as usize];
    (*ndir).adate[0 as libc::c_int as usize] = (*ndir).cdate[0 as libc::c_int as usize];
    set_word(
        ((*ndir).start).as_mut_ptr(),
        (fat & 0xffff as libc::c_int as libc::c_uint) as libc::c_ushort,
    );
    set_word(
        ((*ndir).startHi).as_mut_ptr(),
        (fat >> 16 as libc::c_int) as libc::c_ushort,
    );
    set_dword(((*ndir).size).as_mut_ptr(), size);
    return ndir;
}
pub unsafe extern "C" fn mk_entry_from_base(
    mut base: *const libc::c_char,
    mut attr: libc::c_uchar,
    mut fat: libc::c_uint,
    mut size: uint32_t,
    mut date: time_t,
    mut ndir: *mut directory,
) -> *mut directory {
    let mut entry: *mut directory = 0 as *mut directory;
    let mut dn: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    strncpy((dn.base).as_mut_ptr(), base, 8 as libc::c_int as libc::c_ulong);
    strncpy(
        (dn.ext).as_mut_ptr(),
        b"   \0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    );
    entry = mk_entry(&mut dn, attr, fat, size, date, ndir);
    (*entry).Case = 0 as libc::c_int as libc::c_uchar;
    return entry;
}
