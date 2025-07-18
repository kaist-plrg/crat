use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn closeExec(fd: libc::c_int);
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn lock_dev(fd: libc::c_int, mode: libc::c_int, dev: *mut device) -> libc::c_int;
    fn precmd(dev: *mut device);
    fn postcmd(cmd: *const libc::c_char);
    fn truncBytes32(off: mt_off_t) -> off_t;
    fn send_one_cmd(
        fd: libc::c_int,
        raw_cmd: *mut RawRequest_t,
        message: *const libc::c_char,
    ) -> libc::c_int;
    fn analyze_one_reply(
        raw_cmd: *mut RawRequest_t,
        bytes: *mut libc::c_int,
        do_print: libc::c_int,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type caddr_t = __caddr_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub type mt_off_t = off_t;
pub type smt_off_t = mt_off_t;
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
pub struct floppy_raw_cmd {
    pub flags: libc::c_uint,
    pub data: *mut libc::c_void,
    pub kernel_data: *mut libc::c_char,
    pub next: *mut floppy_raw_cmd,
    pub length: libc::c_long,
    pub phys_length: libc::c_long,
    pub buffer_length: libc::c_int,
    pub rate: libc::c_uchar,
    pub cmd_count: libc::c_uchar,
    pub cmd: [libc::c_uchar; 16],
    pub reply_count: libc::c_uchar,
    pub reply: [libc::c_uchar; 16],
    pub track: libc::c_int,
    pub resultcode: libc::c_int,
    pub reserved1: libc::c_int,
    pub reserved2: libc::c_int,
}
pub type RawRequest_t = floppy_raw_cmd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: libc::c_uint,
    pub RootDirSize: uint16_t,
    pub BadSectors: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Xdf_t {
    pub head: Stream_t,
    pub fd: libc::c_int,
    pub buffer: *mut libc::c_char,
    pub track_valid: bool,
    pub current_track: uint8_t,
    pub map: *mut sector_map_t,
    pub track_size: uint32_t,
    pub track0_size: libc::c_int,
    pub sector_size: uint16_t,
    pub FatSize: uint8_t,
    pub RootDirSize: uint16_t,
    pub track_map: *mut TrackMap_t,
    pub last_sector: libc::c_uchar,
    pub rate: libc::c_uchar,
    #[bitfield(name = "stretch", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "rootskip", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "drive", ty = "libc::c_int", bits = "2..=5")]
    pub stretch_rootskip_drive: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub postcmd: *const libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct TrackMap_t {
    pub begin: libc::c_uchar,
    pub end: libc::c_uchar,
    pub sector: libc::c_uchar,
    pub sizecode: libc::c_uchar,
    #[bitfield(name = "dirty", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "phantom", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "valid", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "head", ty = "libc::c_uint", bits = "4..=4")]
    pub dirty_phantom_valid_head: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type sector_map_t = sector_map;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sector_map {
    #[bitfield(name = "head", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "size", ty = "libc::c_uint", bits = "1..=7")]
    pub head_size: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Compactify_t {
    pub head: libc::c_uchar,
    pub sector: libc::c_uchar,
    pub ptr: libc::c_uchar,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub track_size: libc::c_uchar,
    #[bitfield(name = "track0_size", ty = "libc::c_uint", bits = "0..=6")]
    #[bitfield(name = "rootskip", ty = "libc::c_uint", bits = "7..=7")]
    pub track0_size_rootskip: [u8; 1],
    pub rate: libc::c_uchar,
    pub map: [sector_map_t; 9],
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> libc::c_uint {
    let mut __minor: libc::c_uint = 0;
    __minor = ((__dev & 0xff as libc::c_uint as __dev_t) >> 0 as libc::c_int)
        as libc::c_uint;
    __minor = (__minor as libc::c_ulong
        | (__dev & 0xffffff00000 as libc::c_ulong) >> 12 as libc::c_int) as libc::c_uint;
    return __minor;
}
#[inline]
unsafe extern "C" fn RR_SETRATE(mut request: *mut floppy_raw_cmd, mut rate: uint8_t) {
    (*request).rate = rate;
}
#[inline]
unsafe extern "C" fn RR_SETDRIVE(
    mut request: *mut floppy_raw_cmd,
    mut drive: libc::c_int,
) {
    (*request)
        .cmd[1 as libc::c_int
        as usize] = ((*request).cmd[1 as libc::c_int as usize] as libc::c_int
        & !(3 as libc::c_int) | drive & 3 as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn RR_SETTRACK(mut request: *mut floppy_raw_cmd, mut track: uint8_t) {
    (*request).cmd[2 as libc::c_int as usize] = track;
}
#[inline]
unsafe extern "C" fn RR_SETPTRACK(
    mut request: *mut floppy_raw_cmd,
    mut track: libc::c_int,
) {
    (*request).track = track;
}
#[inline]
unsafe extern "C" fn RR_SETHEAD(mut request: *mut floppy_raw_cmd, mut head: uint8_t) {
    if head != 0 {
        (*request)
            .cmd[1 as libc::c_int
            as usize] = ((*request).cmd[1 as libc::c_int as usize] as libc::c_int
            | 4 as libc::c_int) as libc::c_uchar;
    } else {
        (*request)
            .cmd[1 as libc::c_int
            as usize] = ((*request).cmd[1 as libc::c_int as usize] as libc::c_int
            & !(4 as libc::c_int)) as libc::c_uchar;
    }
    (*request).cmd[3 as libc::c_int as usize] = head;
}
#[inline]
unsafe extern "C" fn RR_SETSECTOR(
    mut request: *mut floppy_raw_cmd,
    mut sector: uint8_t,
) {
    (*request).cmd[4 as libc::c_int as usize] = sector;
    (*request)
        .cmd[6 as libc::c_int
        as usize] = (sector as libc::c_int - 1 as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn RR_SETSIZECODE(
    mut request: *mut floppy_raw_cmd,
    mut sizecode: uint8_t,
) {
    (*request).cmd[5 as libc::c_int as usize] = sizecode;
    (*request)
        .cmd[6 as libc::c_int
        as usize] = ((*request).cmd[6 as libc::c_int as usize]).wrapping_add(1);
    (*request).cmd[6 as libc::c_int as usize];
    (*request).length
        += ((128 as libc::c_int) << sizecode as libc::c_int) as libc::c_long;
}
#[inline]
unsafe extern "C" fn RR_SETDIRECTION(
    mut request: *mut floppy_raw_cmd,
    mut direction: libc::c_int,
) {
    if direction == 1 as libc::c_int {
        (*request).flags |= 1 as libc::c_int as libc::c_uint;
        (*request)
            .cmd[0 as libc::c_int
            as usize] = (0xe6 as libc::c_int & !(0x80 as libc::c_int)) as libc::c_uchar;
    } else {
        (*request).flags |= 2 as libc::c_int as libc::c_uint;
        (*request)
            .cmd[0 as libc::c_int
            as usize] = (0xc5 as libc::c_int & !(0x80 as libc::c_int)) as libc::c_uchar;
    };
}
#[inline]
unsafe extern "C" fn RR_SETDATA(mut request: *mut floppy_raw_cmd, mut data: caddr_t) {
    (*request).data = data as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn RR_SETCONT(mut request: *mut floppy_raw_cmd) {
    (*request).flags |= 0x100 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn RR_SIZECODE(mut request: *mut floppy_raw_cmd) -> libc::c_int {
    return (*request).cmd[5 as libc::c_int as usize] as libc::c_int;
}
#[inline]
unsafe extern "C" fn GET_DRIVE(mut fd: libc::c_int) -> libc::c_int {
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if fstat(fd, &mut statbuf) < 0 as libc::c_int {
        perror(b"stat\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o60000 as libc::c_int as libc::c_uint)
        || gnu_dev_major(statbuf.st_rdev) != 2 as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    return gnu_dev_minor(statbuf.st_rdev) as libc::c_int;
}
#[inline]
unsafe extern "C" fn RR_INIT(mut request: *mut floppy_raw_cmd) {
    (*request).data = 0 as *mut libc::c_void;
    (*request).length = 0 as libc::c_int as libc::c_long;
    (*request).cmd_count = 9 as libc::c_int as libc::c_uchar;
    (*request)
        .flags = (8 as libc::c_int | 0x80 as libc::c_int | 0x40 as libc::c_int)
        as libc::c_uint;
    (*request).cmd[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    (*request).cmd[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    (*request).cmd[7 as libc::c_int as usize] = 0x1b as libc::c_int as libc::c_uchar;
    (*request).cmd[8 as libc::c_int as usize] = 0xff as libc::c_int as libc::c_uchar;
    (*request).reply_count = 0 as libc::c_int as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> libc::c_uint {
    let mut __major: libc::c_uint = 0;
    __major = ((__dev & 0xfff00 as libc::c_uint as __dev_t) >> 8 as libc::c_int)
        as libc::c_uint;
    __major = (__major as libc::c_ulong
        | (__dev & 0xfffff00000000000 as libc::c_ulong) >> 32 as libc::c_int)
        as libc::c_uint;
    return __major;
}
static mut xdf_table: [C2RustUnnamed_0; 5] = [C2RustUnnamed_0 {
    track_size: 0,
    track0_size_rootskip: [0; 1],
    rate: 0,
    map: [sector_map_t {
        head_size: [0; 1],
        c2rust_padding: [0; 3],
    }; 9],
}; 5];
unsafe extern "C" fn analyze_reply(
    mut raw_cmd: *mut RawRequest_t,
    mut do_print: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut newbytes: libc::c_int = 0;
    bytes = 0 as libc::c_int;
    loop {
        ret = analyze_one_reply(raw_cmd, &mut newbytes, do_print);
        bytes += newbytes;
        match ret {
            0 => return bytes,
            1 => {
                raw_cmd = raw_cmd.offset(1);
                raw_cmd;
            }
            -1 => if bytes != 0 { return bytes } else { return 0 as libc::c_int }
            _ => {}
        }
    };
}
unsafe extern "C" fn send_cmd(
    mut fd: libc::c_int,
    mut raw_cmd: *mut RawRequest_t,
    mut nr: libc::c_int,
    mut message: *const libc::c_char,
    mut retries: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if nr == 0 {
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < retries {
        match send_one_cmd(fd, raw_cmd, message) {
            -1 => return -(1 as libc::c_int),
            1 => {
                j += 1;
                j;
            }
            0 | _ => {
                ret = analyze_reply(raw_cmd, j);
                if ret > 0 as libc::c_int {
                    return ret;
                }
            }
        }
        j += 1;
        j;
    }
    if j > 1 as libc::c_int && j == retries {
        fprintf(
            stderr,
            b"Too many errors, giving up\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn add_to_request(
    mut This: *mut Xdf_t,
    mut ptr: libc::c_uchar,
    mut request: *mut RawRequest_t,
    mut nr: *mut libc::c_int,
    mut direction: libc::c_int,
    mut compactify: *mut Compactify_t,
) -> libc::c_int {
    if (*((*This).track_map).offset(ptr as isize)).phantom() != 0 {
        if direction == 1 as libc::c_int {
            memset(
                ((*This).buffer)
                    .offset(
                        (ptr as libc::c_int * (*This).sector_size as libc::c_int)
                            as isize,
                    ) as *mut libc::c_void,
                0 as libc::c_int,
                ((128 as libc::c_uint)
                    << (*((*This).track_map).offset(ptr as isize)).sizecode
                        as libc::c_int) as libc::c_ulong,
            );
        }
        return 0 as libc::c_int;
    }
    if *nr != 0
        && RR_SIZECODE(request.offset(*nr as isize).offset(-(1 as libc::c_int as isize)))
            == (*((*This).track_map).offset(ptr as isize)).sizecode as libc::c_int
        && (*compactify).head as libc::c_int
            == (*((*This).track_map).offset(ptr as isize)).head() as libc::c_int
        && (*compactify).ptr as libc::c_int + 1 as libc::c_int == ptr as libc::c_int
        && (*compactify).sector as libc::c_int + 1 as libc::c_int
            == (*((*This).track_map).offset(ptr as isize)).sector as libc::c_int
    {
        RR_SETSIZECODE(
            request.offset(*nr as isize).offset(-(1 as libc::c_int as isize)),
            (*((*This).track_map).offset(ptr as isize)).sizecode,
        );
    } else {
        if *nr != 0 {
            RR_SETCONT(
                request.offset(*nr as isize).offset(-(1 as libc::c_int as isize)),
            );
        }
        RR_INIT(request.offset(*nr as isize));
        RR_SETDRIVE(request.offset(*nr as isize), (*This).drive());
        RR_SETRATE(request.offset(*nr as isize), (*This).rate);
        RR_SETTRACK(request.offset(*nr as isize), (*This).current_track);
        RR_SETPTRACK(
            request.offset(*nr as isize),
            ((*This).current_track as libc::c_int) << (*This).stretch() as libc::c_int,
        );
        RR_SETHEAD(
            request.offset(*nr as isize),
            (*((*This).track_map).offset(ptr as isize)).head() as uint8_t,
        );
        RR_SETSECTOR(
            request.offset(*nr as isize),
            (*((*This).track_map).offset(ptr as isize)).sector,
        );
        RR_SETSIZECODE(
            request.offset(*nr as isize),
            (*((*This).track_map).offset(ptr as isize)).sizecode,
        );
        RR_SETDIRECTION(request.offset(*nr as isize), direction);
        RR_SETDATA(
            request.offset(*nr as isize),
            ((*This).buffer)
                .offset(
                    (ptr as libc::c_int * (*This).sector_size as libc::c_int) as isize,
                ),
        );
        *nr += 1;
        *nr;
    }
    (*compactify).ptr = ptr;
    (*compactify)
        .head = (*((*This).track_map).offset(ptr as isize)).head() as libc::c_uchar;
    (*compactify).sector = (*((*This).track_map).offset(ptr as isize)).sector;
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_to_request_if_invalid(
    mut This: *mut Xdf_t,
    mut ptr: libc::c_uchar,
    mut request: *mut RawRequest_t,
    mut nr: *mut libc::c_int,
    mut compactify: *mut Compactify_t,
) {
    if (*((*This).track_map).offset(ptr as isize)).valid() == 0 {
        add_to_request(This, ptr, request, nr, 1 as libc::c_int, compactify);
    }
}
unsafe extern "C" fn adjust_bounds(
    mut This: *mut Xdf_t,
    mut ibegin: uint32_t,
    mut iend: uint32_t,
    mut begin: *mut uint8_t,
    mut end: *mut uint8_t,
) {
    *begin = ibegin.wrapping_div((*This).sector_size as libc::c_uint) as uint8_t;
    *end = iend
        .wrapping_add((*This).sector_size as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div((*This).sector_size as libc::c_uint) as uint8_t;
}
#[inline]
unsafe extern "C" fn try_flush_dirty(mut This: *mut Xdf_t) -> libc::c_int {
    let mut ptr: libc::c_uchar = 0;
    let mut nr: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut requests: [RawRequest_t; 100] = [RawRequest_t {
        flags: 0,
        data: 0 as *mut libc::c_void,
        kernel_data: 0 as *mut libc::c_char,
        next: 0 as *mut floppy_raw_cmd,
        length: 0,
        phys_length: 0,
        buffer_length: 0,
        rate: 0,
        cmd_count: 0,
        cmd: [0; 16],
        reply_count: 0,
        reply: [0; 16],
        track: 0,
        resultcode: 0,
        reserved1: 0,
        reserved2: 0,
    }; 100];
    let mut compactify: Compactify_t = Compactify_t {
        head: 0,
        sector: 0,
        ptr: 0,
    };
    if !(*This).track_valid {
        return 0 as libc::c_int;
    }
    nr = 0 as libc::c_int;
    ptr = 0 as libc::c_int as libc::c_uchar;
    while (ptr as libc::c_int) < (*This).last_sector as libc::c_int {
        if (*((*This).track_map).offset(ptr as isize)).dirty() != 0 {
            add_to_request(
                This,
                ptr,
                requests.as_mut_ptr(),
                &mut nr,
                2 as libc::c_int,
                &mut compactify,
            );
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    bytes = send_cmd(
        (*This).fd,
        requests.as_mut_ptr(),
        nr,
        b"writing\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int,
    );
    if bytes < 0 as libc::c_int {
        return bytes;
    }
    ptr = 0 as libc::c_int as libc::c_uchar;
    while (ptr as libc::c_int) < (*This).last_sector as libc::c_int {
        if (*((*This).track_map).offset(ptr as isize)).dirty() != 0 {
            if bytes
                >= (*((*This).track_map).offset(ptr as isize)).end as libc::c_int
                    - (*((*This).track_map).offset(ptr as isize)).begin as libc::c_int
            {
                bytes
                    -= (*((*This).track_map).offset(ptr as isize)).end as libc::c_int
                        - (*((*This).track_map).offset(ptr as isize)).begin
                            as libc::c_int;
                let ref mut fresh0 = *((*This).track_map).offset(ptr as isize);
                (*fresh0).set_dirty(0 as libc::c_int as libc::c_uint);
            } else {
                return 1 as libc::c_int
            }
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn flush_dirty(mut This: *mut Xdf_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        ret = try_flush_dirty(This);
        if !(ret != 0) {
            break;
        }
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn load_data(
    mut This: *mut Xdf_t,
    mut ibegin: uint32_t,
    mut iend: uint32_t,
    mut retries: libc::c_int,
) -> ssize_t {
    let mut ptr: libc::c_uchar = 0;
    let mut nr: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut requests: [RawRequest_t; 100] = [RawRequest_t {
        flags: 0,
        data: 0 as *mut libc::c_void,
        kernel_data: 0 as *mut libc::c_char,
        next: 0 as *mut floppy_raw_cmd,
        length: 0,
        phys_length: 0,
        buffer_length: 0,
        rate: 0,
        cmd_count: 0,
        cmd: [0; 16],
        reply_count: 0,
        reply: [0; 16],
        track: 0,
        resultcode: 0,
        reserved1: 0,
        reserved2: 0,
    }; 100];
    let mut compactify: Compactify_t = Compactify_t {
        head: 0,
        sector: 0,
        ptr: 0,
    };
    let mut begin: libc::c_uchar = 0;
    let mut end: libc::c_uchar = 0;
    adjust_bounds(This, ibegin, iend, &mut begin, &mut end);
    ptr = begin;
    nr = 0 as libc::c_int;
    ptr = (*((*This).track_map).offset(ptr as isize)).begin;
    while (ptr as libc::c_int) < end as libc::c_int {
        add_to_request_if_invalid(
            This,
            ptr,
            requests.as_mut_ptr(),
            &mut nr,
            &mut compactify,
        );
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    bytes = send_cmd(
        (*This).fd,
        requests.as_mut_ptr(),
        nr,
        b"reading\0" as *const u8 as *const libc::c_char,
        retries,
    );
    if bytes < 0 as libc::c_int {
        return bytes as ssize_t;
    }
    ptr = begin;
    ptr = (*((*This).track_map).offset(ptr as isize)).begin;
    while (ptr as libc::c_int) < end as libc::c_int {
        if (*((*This).track_map).offset(ptr as isize)).valid() == 0 {
            if bytes
                >= (*((*This).track_map).offset(ptr as isize)).end as libc::c_int
                    - (*((*This).track_map).offset(ptr as isize)).begin as libc::c_int
            {
                bytes
                    -= (*((*This).track_map).offset(ptr as isize)).end as libc::c_int
                        - (*((*This).track_map).offset(ptr as isize)).begin
                            as libc::c_int;
                let ref mut fresh1 = *((*This).track_map).offset(ptr as isize);
                (*fresh1).set_valid(1 as libc::c_int as libc::c_uint);
            } else if ptr as libc::c_int > begin as libc::c_int {
                return (ptr as libc::c_int * (*This).sector_size as libc::c_int)
                    as ssize_t
            } else {
                return -(1 as libc::c_int) as ssize_t
            }
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    return (end as libc::c_int * (*This).sector_size as libc::c_int) as ssize_t;
}
unsafe extern "C" fn mark_dirty(
    mut This: *mut Xdf_t,
    mut ibegin: uint32_t,
    mut iend: uint32_t,
) {
    let mut ptr: libc::c_int = 0;
    let mut begin: libc::c_uchar = 0;
    let mut end: libc::c_uchar = 0;
    adjust_bounds(This, ibegin, iend, &mut begin, &mut end);
    ptr = begin as libc::c_int;
    ptr = (*((*This).track_map).offset(ptr as isize)).begin as libc::c_int;
    while ptr < end as libc::c_int {
        let ref mut fresh2 = *((*This).track_map).offset(ptr as isize);
        (*fresh2).set_valid(1 as libc::c_int as libc::c_uint);
        if (*((*This).track_map).offset(ptr as isize)).phantom() == 0 {
            let ref mut fresh3 = *((*This).track_map).offset(ptr as isize);
            (*fresh3).set_dirty(1 as libc::c_int as libc::c_uint);
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end as libc::c_int;
    }
}
unsafe extern "C" fn load_bounds(
    mut This: *mut Xdf_t,
    mut begin: uint32_t,
    mut end: uint32_t,
) -> ssize_t {
    let mut lbegin: libc::c_uchar = 0;
    let mut lend: libc::c_uchar = 0;
    adjust_bounds(This, begin, end, &mut lbegin, &mut lend);
    if begin
        != ((*((*This).track_map).offset(lbegin as isize)).begin as libc::c_int
            * (*This).sector_size as libc::c_int) as libc::c_uint
        && end
            != ((*((*This).track_map).offset(lend as isize)).begin as libc::c_int
                * (*This).sector_size as libc::c_int) as libc::c_uint
        && (lend as libc::c_int)
            < (*((*This).track_map)
                .offset((*((*This).track_map).offset(lbegin as isize)).end as isize))
                .end as libc::c_int
    {
        return load_data(This, begin, end, 4 as libc::c_int);
    }
    if begin
        != ((*((*This).track_map).offset(lbegin as isize)).begin as libc::c_int
            * (*This).sector_size as libc::c_int) as libc::c_uint
    {
        let mut ret: ssize_t = load_data(This, begin, begin, 4 as libc::c_int);
        if ret < 0 as libc::c_int as libc::c_long {
            return ret;
        }
    }
    if end
        != ((*((*This).track_map).offset(lend as isize)).begin as libc::c_int
            * (*This).sector_size as libc::c_int) as libc::c_uint
    {
        let mut ret_0: ssize_t = load_data(This, end, end, 4 as libc::c_int);
        if ret_0 < 0 as libc::c_int as libc::c_long {
            return ((*((*This).track_map).offset(lend as isize)).begin as libc::c_int
                * (*This).sector_size as libc::c_int) as ssize_t;
        }
    }
    return (lend as libc::c_int * (*This).sector_size as libc::c_int) as ssize_t;
}
unsafe extern "C" fn fill_boot(mut This: *mut Xdf_t) {
    let mut ptr: uint8_t = 0 as libc::c_int as uint8_t;
    let ref mut fresh4 = *((*This).track_map).offset(ptr as isize);
    (*fresh4).set_head(0 as libc::c_int as libc::c_uint);
    (*((*This).track_map).offset(ptr as isize))
        .sector = 129 as libc::c_int as libc::c_uchar;
    let ref mut fresh5 = *((*This).track_map).offset(ptr as isize);
    (*fresh5).set_phantom(0 as libc::c_int as libc::c_uint);
    (*((*This).track_map).offset(ptr as isize)).begin = ptr;
    (*((*This).track_map).offset(ptr as isize))
        .end = (ptr as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
    (*((*This).track_map).offset(ptr as isize))
        .sizecode = 2 as libc::c_int as libc::c_uchar;
    let ref mut fresh6 = *((*This).track_map).offset(ptr as isize);
    (*fresh6).set_valid(0 as libc::c_int as libc::c_uint);
    let ref mut fresh7 = *((*This).track_map).offset(ptr as isize);
    (*fresh7).set_dirty(0 as libc::c_int as libc::c_uint);
    (*This).last_sector = 1 as libc::c_int as libc::c_uchar;
    (*This).current_track = 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn fill_t0(
    mut This: *mut Xdf_t,
    mut ptr: uint8_t,
    mut size: libc::c_uint,
    mut sector: *mut uint8_t,
    mut head: *mut uint8_t,
) -> uint8_t {
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while n < size {
        let ref mut fresh8 = *((*This).track_map).offset(ptr as isize);
        (*fresh8).set_head(*head as libc::c_uint);
        (*((*This).track_map).offset(ptr as isize))
            .sector = (*sector as libc::c_int + 129 as libc::c_int) as libc::c_uchar;
        let ref mut fresh9 = *((*This).track_map).offset(ptr as isize);
        (*fresh9).set_phantom(0 as libc::c_int as libc::c_uint);
        *sector = (*sector).wrapping_add(1);
        *sector;
        if *head == 0 && *sector as libc::c_int >= (*This).track0_size - 8 as libc::c_int
        {
            *sector = 0 as libc::c_int as uint8_t;
            *head = 1 as libc::c_int as uint8_t;
        }
        ptr = ptr.wrapping_add(1);
        ptr;
        n = n.wrapping_add(1);
        n;
    }
    return ptr;
}
unsafe extern "C" fn fill_phantoms(
    mut This: *mut Xdf_t,
    mut ptr: uint8_t,
    mut size: uint8_t,
) -> uint8_t {
    let mut n: libc::c_uint = 0;
    n = 0 as libc::c_int as libc::c_uint;
    while n < size as libc::c_uint {
        let ref mut fresh10 = *((*This).track_map).offset(ptr as isize);
        (*fresh10).set_phantom(1 as libc::c_int as libc::c_uint);
        ptr = ptr.wrapping_add(1);
        ptr;
        n = n.wrapping_add(1);
        n;
    }
    return ptr;
}
unsafe extern "C" fn decompose(
    mut This: *mut Xdf_t,
    mut iwhere: mt_off_t,
    mut len: size_t,
    mut begin: *mut uint32_t,
    mut end: *mut uint32_t,
    mut boot: uint8_t,
) -> libc::c_int {
    let mut ptr: uint8_t = 0;
    let mut map: *mut sector_map_t = 0 as *mut sector_map_t;
    let mut lbegin: uint8_t = 0;
    let mut lend: uint8_t = 0;
    let mut track_size: uint32_t = ((*This).track_size)
        .wrapping_mul(1024 as libc::c_int as libc::c_uint);
    let mut track: smt_off_t = iwhere / track_size as libc::c_long;
    let mut where_0: uint32_t = (iwhere % track_size as libc::c_long) as uint32_t;
    *begin = where_0;
    if (where_0 as libc::c_ulong).wrapping_add(len) > track_size as libc::c_ulong {
        *end = track_size;
    } else {
        *end = (where_0 as libc::c_ulong).wrapping_add(len) as uint32_t;
    }
    if (*This).current_track as libc::c_long == track && boot == 0 {
        return 0 as libc::c_int;
    }
    if boot == 0 {
        flush_dirty(This);
    }
    if track >= 80 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    (*This).current_track = track as uint8_t;
    (*This).track_valid = 1 as libc::c_int != 0;
    if track != 0 {
        ptr = 0 as libc::c_int as uint8_t;
        map = (*This).map;
        while (*map).size() != 0 {
            lbegin = ptr;
            lend = (ptr as libc::c_int
                + ((128 as libc::c_uint) << (*map).size() as libc::c_int)
                    .wrapping_div((*This).sector_size as libc::c_uint) as uint8_t
                    as libc::c_int) as uint8_t;
            while (ptr as libc::c_int) < lend as libc::c_int {
                (*((*This).track_map).offset(ptr as isize)).begin = lbegin;
                (*((*This).track_map).offset(ptr as isize)).end = lend;
                let ref mut fresh11 = *((*This).track_map).offset(ptr as isize);
                (*fresh11).set_head((*map).head());
                (*((*This).track_map).offset(ptr as isize))
                    .sector = ((*map).size() as libc::c_int + 128 as libc::c_int)
                    as libc::c_uchar;
                (*((*This).track_map).offset(ptr as isize))
                    .sizecode = (*map).size() as libc::c_uchar;
                let ref mut fresh12 = *((*This).track_map).offset(ptr as isize);
                (*fresh12).set_valid(0 as libc::c_int as libc::c_uint);
                let ref mut fresh13 = *((*This).track_map).offset(ptr as isize);
                (*fresh13).set_dirty(0 as libc::c_int as libc::c_uint);
                let ref mut fresh14 = *((*This).track_map).offset(ptr as isize);
                (*fresh14).set_phantom(0 as libc::c_int as libc::c_uint);
                ptr = ptr.wrapping_add(1);
                ptr;
            }
            map = map.offset(1);
            map;
        }
        let ref mut fresh15 = (*((*This).track_map).offset(ptr as isize)).end;
        *fresh15 = ptr;
        (*((*This).track_map).offset(ptr as isize)).begin = *fresh15;
    } else {
        let mut sector: uint8_t = 0;
        let mut head: uint8_t = 0;
        head = 0 as libc::c_int as uint8_t;
        sector = 0 as libc::c_int as uint8_t;
        ptr = boot;
        while (ptr as libc::c_uint)
            < (2 as libc::c_int as libc::c_uint).wrapping_mul((*This).track_size)
        {
            (*((*This).track_map).offset(ptr as isize)).begin = ptr;
            (*((*This).track_map).offset(ptr as isize))
                .end = (ptr as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
            (*((*This).track_map).offset(ptr as isize))
                .sizecode = 2 as libc::c_int as libc::c_uchar;
            let ref mut fresh16 = *((*This).track_map).offset(ptr as isize);
            (*fresh16).set_valid(0 as libc::c_int as libc::c_uint);
            let ref mut fresh17 = *((*This).track_map).offset(ptr as isize);
            (*fresh17).set_dirty(0 as libc::c_int as libc::c_uint);
            ptr = ptr.wrapping_add(1);
            ptr;
        }
        ptr = fill_t0(
            This,
            0 as libc::c_int as uint8_t,
            (1 as libc::c_int + (*This).FatSize as libc::c_int) as libc::c_uint,
            &mut sector,
            &mut head,
        );
        ptr = fill_phantoms(This, ptr, (*This).FatSize);
        ptr = fill_t0(
            This,
            ptr,
            (*This).RootDirSize as libc::c_uint,
            &mut sector,
            &mut head,
        );
        ptr = fill_phantoms(This, ptr, 5 as libc::c_int as uint8_t);
        if (*This).rootskip() != 0 {
            sector = sector.wrapping_add(1);
            sector;
        }
        ptr = fill_t0(
            This,
            ptr,
            ((*This).track_size)
                .wrapping_sub((*This).FatSize as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_sub((*This).RootDirSize as libc::c_uint)
                .wrapping_sub(6 as libc::c_int as libc::c_uint),
            &mut sector,
            &mut head,
        );
    }
    (*This).last_sector = ptr;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xdf_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut begin: uint32_t = 0;
    let mut end: uint32_t = 0;
    let mut ret: ssize_t = 0;
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    if decompose(
        This,
        truncBytes32(where_0),
        len,
        &mut begin,
        &mut end,
        0 as libc::c_int as uint8_t,
    ) < 0 as libc::c_int
    {
        return 0 as libc::c_int as ssize_t;
    }
    ret = load_data(This, begin, end, 4 as libc::c_int);
    if ret < 0 as libc::c_int as libc::c_long || (ret as size_t) < begin as libc::c_ulong
    {
        return -(1 as libc::c_int) as ssize_t;
    }
    if len > (ret as size_t).wrapping_sub(begin as libc::c_ulong) {
        len = (ret as size_t).wrapping_sub(begin as libc::c_ulong);
    }
    memcpy(
        buf as *mut libc::c_void,
        ((*This).buffer).offset(begin as isize) as *const libc::c_void,
        len,
    );
    return end.wrapping_sub(begin) as ssize_t;
}
unsafe extern "C" fn xdf_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut begin: uint32_t = 0;
    let mut end: uint32_t = 0;
    let mut len2: ssize_t = 0;
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    if decompose(
        This,
        truncBytes32(where_0),
        len,
        &mut begin,
        &mut end,
        0 as libc::c_int as uint8_t,
    ) < 0 as libc::c_int
    {
        *__errno_location() = 27 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    len2 = load_bounds(This, begin, end);
    if len2 < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as ssize_t;
    }
    if end > len2 as uint32_t {
        end = len2 as uint32_t;
    }
    len2 -= begin as libc::c_long;
    if len > len2 as size_t {
        len = len2 as size_t;
    }
    memcpy(
        ((*This).buffer).offset(begin as isize) as *mut libc::c_void,
        buf as *const libc::c_void,
        len,
    );
    mark_dirty(This, begin, end);
    return end.wrapping_sub(begin) as ssize_t;
}
unsafe extern "C" fn xdf_flush(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    return flush_dirty(This);
}
unsafe extern "C" fn xdf_free(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    free((*This).track_map as *mut libc::c_char as *mut libc::c_void);
    free((*This).buffer as *mut libc::c_void);
    ret = close((*This).fd);
    postcmd((*This).postcmd);
    return ret;
}
unsafe extern "C" fn check_geom(
    mut This: *mut Xdf_t,
    mut dev: *mut device,
) -> libc::c_int {
    let mut sect: libc::c_uint = 0;
    if !(!dev.is_null() && (*dev).misc_flags & 0x10 as libc::c_uint != 0) {
        if (*dev).sectors as libc::c_int != 0
            && (*dev).sectors as libc::c_int != 19 as libc::c_int
            && ((*dev).sectors as libc::c_int != 0
                && (*dev).sectors as libc::c_int != 23 as libc::c_int)
            && ((*dev).sectors as libc::c_int != 0
                && (*dev).sectors as libc::c_int != 24 as libc::c_int)
            && ((*dev).sectors as libc::c_int != 0
                && (*dev).sectors as libc::c_int != 46 as libc::c_int)
            && ((*dev).sectors as libc::c_int != 0
                && (*dev).sectors as libc::c_int != 48 as libc::c_int)
        {
            return 1 as libc::c_int;
        }
        if (*dev).heads as libc::c_int != 0
            && (*dev).heads as libc::c_int != 2 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    if !This.is_null() {
        sect = (*This).track_size;
        if sect != 19 as libc::c_int as libc::c_uint
            && sect != 23 as libc::c_int as libc::c_uint
            && sect != 24 as libc::c_int as libc::c_uint
            && sect != 46 as libc::c_int as libc::c_uint
            && sect != 48 as libc::c_int as libc::c_uint
            || !(!dev.is_null() && (*dev).misc_flags & 0x10 as libc::c_uint != 0)
                && ((*dev).sectors as libc::c_int != 0
                    && (*dev).sectors as libc::c_uint != sect)
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_geom(mut This: *mut Xdf_t, mut dev: *mut device) {
    (*dev).heads = 2 as libc::c_int as uint16_t;
    (*dev).use_2m = 0xff as libc::c_int as libc::c_uint;
    (*dev).sectors = (*This).track_size as uint16_t;
    (*dev).tracks = 80 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn config_geom(
    mut Stream: *mut Stream_t,
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> libc::c_int {
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    if check_geom(This, dev) != 0 {
        return 1 as libc::c_int;
    }
    set_geom(This, dev);
    return 0 as libc::c_int;
}
static mut XdfClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                xdf_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                xdf_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(xdf_flush as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int),
            freeFunc: Some(
                xdf_free as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            set_geom: Some(
                config_geom
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device,
                        *mut device,
                    ) -> libc::c_int,
            ),
            get_data: None,
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
pub unsafe extern "C" fn XdfOpen(
    mut dev: *mut device,
    mut name: *const libc::c_char,
    mut mode: libc::c_int,
    mut errmsg: *mut libc::c_char,
    mut info: *mut xdf_info,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut This: *mut Xdf_t = 0 as *mut Xdf_t;
    let mut begin: uint32_t = 0;
    let mut end: uint32_t = 0;
    let mut boot: *mut bootsector = 0 as *mut bootsector;
    let mut type_0: libc::c_uint = 0;
    let mut fatSize: uint16_t = 0;
    if !dev.is_null()
        && (!(!dev.is_null() && (*dev).misc_flags & 0x8 as libc::c_uint != 0)
            && (getenv(b"MTOOLS_USE_XDF\0" as *const u8 as *const libc::c_char))
                .is_null() || check_geom(0 as *mut Xdf_t, dev) != 0)
    {
        return 0 as *mut Stream_t;
    }
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Xdf_t>() as libc::c_ulong,
    ) as *mut Xdf_t;
    if This.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut XdfClass, 0 as *mut Stream_t);
    (*This).sector_size = 512 as libc::c_int as uint16_t;
    (*This).set_stretch(0 as libc::c_int as libc::c_uint);
    (*This).postcmd = 0 as *const libc::c_char;
    precmd(dev);
    if !dev.is_null() {
        (*This).postcmd = (*dev).postcmd;
    }
    (*This)
        .fd = open(
        name,
        (mode | (*dev).mode) & !(0o3 as libc::c_int) | 0o200 as libc::c_int
            | 0o4000 as libc::c_int | 0o2 as libc::c_int,
    );
    if (*This).fd < 0 as libc::c_int {
        snprintf(
            errmsg,
            199 as libc::c_int as libc::c_ulong,
            b"xdf floppy: open: \"%s\"\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    } else {
        closeExec((*This).fd);
        (*This).set_drive(GET_DRIVE((*This).fd));
        if !((*This).drive() < 0 as libc::c_int) {
            (*This)
                .buffer = malloc(
                (96 as libc::c_int * 512 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
            if !((*This).buffer).is_null() {
                (*This).track_valid = 0 as libc::c_int != 0;
                (*This)
                    .track_map = calloc(
                    96 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<TrackMap_t>() as libc::c_ulong,
                ) as *mut TrackMap_t;
                if !((*This).track_map).is_null() {
                    if lock_dev(
                        (*This).fd,
                        (mode == 0o2 as libc::c_int) as libc::c_int,
                        dev,
                    ) != 0
                    {
                        snprintf(
                            errmsg,
                            199 as libc::c_int as libc::c_ulong,
                            b"xdf floppy: device \"%s\" busy:\0" as *const u8
                                as *const libc::c_char,
                            (*dev).name,
                        );
                    } else {
                        fill_boot(This);
                        (*This).rate = 0 as libc::c_int as libc::c_uchar;
                        if load_data(
                            This,
                            0 as libc::c_int as uint32_t,
                            1 as libc::c_int as uint32_t,
                            4 as libc::c_int,
                        ) < 0 as libc::c_int as libc::c_long
                        {
                            (*This).rate = 0x43 as libc::c_int as libc::c_uchar;
                            if load_data(
                                This,
                                0 as libc::c_int as uint32_t,
                                1 as libc::c_int as uint32_t,
                                4 as libc::c_int,
                            ) < 0 as libc::c_int as libc::c_long
                            {
                                current_block = 16003213496410897663;
                            } else {
                                current_block = 17478428563724192186;
                            }
                        } else {
                            current_block = 17478428563724192186;
                        }
                        match current_block {
                            16003213496410897663 => {}
                            _ => {
                                boot = (*This).buffer as *mut bootsector;
                                fatSize = ((*boot).boot.fatlen[0 as libc::c_int as usize]
                                    as libc::c_int
                                    + (((*boot).boot.fatlen[1 as libc::c_int as usize]
                                        as libc::c_int) << 8 as libc::c_int)) as uint16_t;
                                if fatSize as libc::c_int > 255 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"Fat size %d too large\n\0" as *const u8
                                            as *const libc::c_char,
                                        fatSize as libc::c_int,
                                    );
                                    exit(1 as libc::c_int);
                                }
                                (*This).FatSize = fatSize as uint8_t;
                                (*This)
                                    .RootDirSize = (((*boot)
                                    .boot
                                    .dirents[0 as libc::c_int as usize] as libc::c_int
                                    + (((*boot).boot.dirents[1 as libc::c_int as usize]
                                        as libc::c_int) << 8 as libc::c_int)) as uint16_t
                                    as libc::c_int / 16 as libc::c_int) as uint16_t;
                                (*This)
                                    .track_size = ((*boot).boot.nsect[0 as libc::c_int as usize]
                                    as libc::c_int
                                    + (((*boot).boot.nsect[1 as libc::c_int as usize]
                                        as libc::c_int) << 8 as libc::c_int)) as uint16_t
                                    as uint32_t;
                                type_0 = 0 as libc::c_int as libc::c_uint;
                                while (type_0 as libc::c_ulong)
                                    < (::std::mem::size_of::<[C2RustUnnamed_0; 5]>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong,
                                        )
                                {
                                    if xdf_table[type_0 as usize].track_size as libc::c_uint
                                        == (*This).track_size
                                    {
                                        (*This).map = (xdf_table[type_0 as usize].map).as_mut_ptr();
                                        (*This)
                                            .track0_size = (xdf_table[type_0 as usize]).track0_size()
                                            as libc::c_int;
                                        (*This)
                                            .set_rootskip((xdf_table[type_0 as usize]).rootskip());
                                        (*This).rate = xdf_table[type_0 as usize].rate;
                                        break;
                                    } else {
                                        type_0 = type_0.wrapping_add(1);
                                        type_0;
                                    }
                                }
                                if !(type_0 as libc::c_ulong
                                    == (::std::mem::size_of::<[C2RustUnnamed_0; 5]>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong,
                                        ))
                                {
                                    if !info.is_null() {
                                        (*info).RootDirSize = (*This).RootDirSize;
                                        (*info).FatSize = (*This).FatSize as libc::c_uint;
                                        (*info).BadSectors = 5 as libc::c_int as libc::c_uint;
                                    }
                                    decompose(
                                        This,
                                        0 as libc::c_int as mt_off_t,
                                        512 as libc::c_int as size_t,
                                        &mut begin,
                                        &mut end,
                                        1 as libc::c_int as uint8_t,
                                    );
                                    if !dev.is_null() {
                                        set_geom(This, dev);
                                    }
                                    return &mut (*This).head;
                                }
                            }
                        }
                    }
                    free((*This).track_map as *mut libc::c_char as *mut libc::c_void);
                }
                free((*This).buffer as *mut libc::c_void);
            }
        }
        close((*This).fd);
    }
    free(This as *mut libc::c_char as *mut libc::c_void);
    return 0 as *mut Stream_t;
}
unsafe extern "C" fn run_static_initializers() {
    xdf_table = [
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 19 as libc::c_int as libc::c_uchar,
                rate: 0 as libc::c_int as libc::c_uchar,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(3 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(2 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(2 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(3 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(0 as libc::c_int as libc::c_uint);
                        init
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                ],
            };
            init.set_track0_size(16 as libc::c_int as libc::c_uint);
            init.set_rootskip(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 23 as libc::c_int as libc::c_uchar,
                rate: 0 as libc::c_int as libc::c_uchar,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(3 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(4 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(2 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(2 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(4 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(3 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(0 as libc::c_int as libc::c_uint);
                        init
                    },
                ],
            };
            init.set_track0_size(19 as libc::c_int as libc::c_uint);
            init.set_rootskip(0 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 46 as libc::c_int as libc::c_uchar,
                rate: 0x43 as libc::c_int as libc::c_uchar,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(3 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(4 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(5 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(7 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(3 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(4 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(5 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(7 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(0 as libc::c_int as libc::c_uint);
                        init
                    },
                ],
            };
            init.set_track0_size(37 as libc::c_int as libc::c_uint);
            init.set_rootskip(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 24 as libc::c_int as libc::c_uchar,
                rate: 0 as libc::c_int as libc::c_uchar,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(5 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(5 as libc::c_int as libc::c_uint);
                        init
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                ],
            };
            init.set_track0_size(20 as libc::c_int as libc::c_uint);
            init.set_rootskip(1 as libc::c_int as libc::c_uint);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 48 as libc::c_int as libc::c_uchar,
                rate: 0 as libc::c_int as libc::c_uchar,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(7 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as libc::c_int as libc::c_uint);
                        init.set_size(7 as libc::c_int as libc::c_uint);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as libc::c_int as libc::c_uint);
                        init.set_size(6 as libc::c_int as libc::c_uint);
                        init
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                ],
            };
            init.set_track0_size(41 as libc::c_int as libc::c_uint);
            init.set_rootskip(1 as libc::c_int as libc::c_uint);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
