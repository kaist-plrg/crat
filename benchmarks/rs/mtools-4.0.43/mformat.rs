use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    pub type FatMap_t;
    fn random() -> libc::c_long;
    fn srandom(__seed: libc::c_uint);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    fn label_name_uc(
        cp: *mut doscp_t,
        filename: *const libc::c_char,
        verbose: libc::c_int,
        mangled: *mut libc::c_int,
        ans: *mut dos_name_t,
    );
    fn label_name_pc(
        cp: *mut doscp_t,
        filename: *const libc::c_char,
        verbose: libc::c_int,
        mangled: *mut libc::c_int,
        ans: *mut dos_name_t,
    );
    static mut mtools_rate_0: uint8_t;
    static mut mtools_rate_any: uint8_t;
    fn get_default_drive() -> libc::c_char;
    fn set_cmd_line_image(img: *mut libc::c_char);
    fn check_number_parse_errno(
        c: libc::c_char,
        optarg_0: *const libc::c_char,
        endptr: *mut libc::c_char,
    );
    fn parseSize(sizeStr: *mut libc::c_char) -> uint32_t;
    fn strtoui(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_uint;
    fn atoui(nptr: *const libc::c_char) -> libc::c_uint;
    fn atoul(nptr: *const libc::c_char) -> libc::c_ulong;
    fn strtou8(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint8_t;
    fn atou8(str: *const libc::c_char) -> uint8_t;
    fn strtou16(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint16_t;
    fn atou16(str: *const libc::c_char) -> uint16_t;
    fn strtou32(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint32_t;
    static mut progname: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    fn expand(_: *const libc::c_char, _: *mut libc::c_char) -> *const libc::c_char;
    static mut mversion: *const libc::c_char;
    static mut mdate: *const libc::c_char;
    static mut mformat_banner: *const libc::c_char;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    static mut devices: *mut device;
    fn check_if_sectors_fit(
        tot_sectors: uint32_t,
        maxBytes: mt_off_t,
        sectorSize: uint32_t,
        errmsg: *mut libc::c_char,
    ) -> libc::c_int;
    fn chs_to_totsectors(dev: *mut device, errmsg: *mut libc::c_char) -> libc::c_int;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn getOldDosBySize(size: size_t) -> *mut OldDos_t;
    fn getOldDosByParams(
        tracks: libc::c_uint,
        heads: libc::c_uint,
        sectors: libc::c_uint,
        dir_len: libc::c_uint,
        cluster_size: libc::c_uint,
    ) -> *mut OldDos_t;
    fn calc_clus_start(Fs: *mut Fs_t) -> uint32_t;
    static mut FsClass: Class_t;
    fn calc_num_clus(Fs: *mut Fs_t, tot_sectors: uint32_t) -> libc::c_int;
    fn set_fat(This: *mut Fs_t, haveBigFatLen: bool);
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
    fn fatAllocate(This: *mut Fs_t, pos: libc::c_uint, value: libc::c_uint);
    fn fatEncode(This: *mut Fs_t, pos: libc::c_uint, value: libc::c_uint);
    fn zero_fat(Fs: *mut Fs_t, media_descriptor: uint8_t) -> libc::c_int;
    fn OpenRoot(Dir: *mut Stream_t) -> *mut Stream_t;
    fn labelit(
        dosname: *mut dos_name_t,
        longname: *mut libc::c_char,
        arg0: *mut libc::c_void,
        entry: *mut direntry_t,
    ) -> libc::c_int;
    fn mwrite_one(
        Dir: *mut Stream_t,
        argname: *const libc::c_char,
        shortname: *const libc::c_char,
        cb: Option::<write_data_callback>,
        arg: *mut libc::c_void,
        ch: *mut ClashHandling_t,
    ) -> libc::c_int;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn buf_init(
        Next: *mut Stream_t,
        size: size_t,
        cylinderSize: size_t,
        sectorSize: size_t,
    ) -> *mut Stream_t;
    fn setBeginEnd(
        partTable: *mut partition,
        begin: uint32_t,
        end: uint32_t,
        iheads: uint16_t,
        isectors: uint16_t,
        activate: libc::c_int,
        type_0: uint8_t,
        fat_bits: libc::c_uint,
    );
    fn OpenImage(
        out_dev: *mut device,
        dev: *mut device,
        name: *const libc::c_char,
        mode: libc::c_int,
        errmsg: *mut libc::c_char,
        flags: libc::c_int,
        lockMode: libc::c_int,
        maxSize: *mut mt_off_t,
        geomFailureP: *mut libc::c_int,
        xdf_info: *mut xdf_info,
    ) -> *mut Stream_t;
    fn cp_open(codepage: libc::c_uint) -> *mut doscp_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
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
pub union C2RustUnnamed {
    pub fat32: fat32_t,
    pub old: oldboot_t,
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
pub union bootsector {
    pub bytes: [libc::c_uchar; 4096],
    pub characters: [libc::c_char; 4096],
    pub boot: bootsector_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClashHandling_t {
    pub action: [clash_action; 2],
    pub namematch_default: [clash_action; 2],
    pub nowarn: libc::c_int,
    pub got_slots: libc::c_int,
    pub mod_time: libc::c_int,
    pub myname: *mut libc::c_char,
    pub dosname: *mut libc::c_uchar,
    pub single: libc::c_int,
    pub use_longname: libc::c_int,
    pub ignore_entry: libc::c_int,
    pub source: libc::c_int,
    pub source_entry: libc::c_int,
    pub name_converter: Option::<
        unsafe extern "C" fn(
            *mut doscp_t,
            *const libc::c_char,
            libc::c_int,
            *mut libc::c_int,
            *mut dos_name_t,
        ) -> (),
    >,
    pub is_label: libc::c_int,
}
pub type clash_action = libc::c_uint;
pub const NAMEMATCH_GREW: clash_action = 9;
pub const NAMEMATCH_SUCCESS: clash_action = 8;
pub const NAMEMATCH_ERROR: clash_action = 7;
pub const NAMEMATCH_OVERWRITE: clash_action = 6;
pub const NAMEMATCH_PRENAME: clash_action = 5;
pub const NAMEMATCH_RENAME: clash_action = 4;
pub const NAMEMATCH_SKIP: clash_action = 3;
pub const NAMEMATCH_QUIT: clash_action = 2;
pub const NAMEMATCH_AUTORENAME: clash_action = 1;
pub const NAMEMATCH_NONE: clash_action = 0;
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
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut libc::c_char,
    *mut libc::c_void,
    *mut direntry_t,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: libc::c_uint,
    pub RootDirSize: uint16_t,
    pub BadSectors: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partition {
    pub start: hsc,
    pub end: hsc,
    pub start_sect: [libc::c_uchar; 4],
    pub nr_sects: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsc {
    pub byte0: libc::c_uchar,
    pub head: libc::c_uchar,
    pub sector: libc::c_uchar,
    pub cyl: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OldDos_t {
    pub tracks: libc::c_uint,
    pub sectors: uint16_t,
    pub heads: uint16_t,
    pub dir_len: uint16_t,
    pub cluster_size: uint8_t,
    pub fat_len: uint32_t,
    pub media: uint8_t,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn ch_toupper(mut ch: libc::c_char) -> libc::c_char {
    return ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = ch as libc::c_uchar as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(ch as libc::c_uchar as libc::c_int);
            }
        } else {
            __res = *(*__ctype_toupper_loc())
                .offset(ch as libc::c_uchar as libc::c_int as isize);
        }
        __res
    }) as libc::c_char;
}
#[inline]
unsafe extern "C" fn init_random() {
    srandom(time(0 as *mut time_t) as libc::c_uint);
}
#[inline]
unsafe extern "C" fn ptrdiff(
    mut end: *const libc::c_char,
    mut begin: *const libc::c_char,
) -> size_t {
    return end.offset_from(begin) as libc::c_long as size_t;
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
unsafe extern "C" fn init_geometry_boot(
    mut boot: *mut bootsector,
    mut dev: *mut device,
    mut sectors0: uint8_t,
    mut rate_0: uint8_t,
    mut rate_any: uint8_t,
    mut tot_sectors: *mut uint32_t,
    mut keepBoot: libc::c_int,
) -> uint16_t {
    let mut nb_renum: libc::c_int = 0;
    let mut sector2: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    set_word(((*boot).boot.nsect).as_mut_ptr(), (*dev).sectors);
    set_word(((*boot).boot.nheads).as_mut_ptr(), (*dev).heads);
    if *tot_sectors != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"*tot_sectors != 0\0" as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 109],
                &[libc::c_char; 109],
            >(
                b"uint16_t init_geometry_boot(union bootsector *, struct device *, uint8_t, uint8_t, uint8_t, uint32_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9026: {
        if *tot_sectors != 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"*tot_sectors != 0\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 109],
                    &[libc::c_char; 109],
                >(
                    b"uint16_t init_geometry_boot(union bootsector *, struct device *, uint8_t, uint8_t, uint8_t, uint32_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if *tot_sectors <= 65535 as libc::c_int as libc::c_uint
        && (*dev).hidden <= 65535 as libc::c_int as libc::c_uint
    {
        set_word(((*boot).boot.psect).as_mut_ptr(), *tot_sectors as uint16_t);
        set_dword(((*boot).boot.bigsect).as_mut_ptr(), 0 as libc::c_int as uint32_t);
        set_word(((*boot).boot.nhs).as_mut_ptr(), (*dev).hidden as uint16_t);
    } else {
        set_word(((*boot).boot.psect).as_mut_ptr(), 0 as libc::c_int as libc::c_ushort);
        set_dword(((*boot).boot.bigsect).as_mut_ptr(), *tot_sectors);
        set_dword(((*boot).boot.nhs).as_mut_ptr(), (*dev).hidden);
    }
    if (*dev).use_2m & 0x7f as libc::c_int as libc::c_uint != 0 {
        let mut bootOffset: uint16_t = 0;
        let mut j: uint8_t = 0;
        let mut size2: uint8_t = 0;
        let mut i: uint16_t = 0;
        strncpy(
            ((*boot).boot.banner).as_mut_ptr(),
            b"2M-STV04\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        );
        (*boot).boot.ext.old.res_2m = 0 as libc::c_int as libc::c_uchar;
        (*boot).boot.ext.old.fmt_2mf = 6 as libc::c_int as libc::c_uchar;
        if (*dev).sectors as libc::c_int
            % (((1 as libc::c_int) << (*dev).ssize as libc::c_int) + 3 as libc::c_int
                >> 2 as libc::c_int) != 0
        {
            (*boot).boot.ext.old.wt = 1 as libc::c_int as libc::c_uchar;
        } else {
            (*boot).boot.ext.old.wt = 0 as libc::c_int as libc::c_uchar;
        }
        (*boot).boot.ext.old.rate_0 = rate_0;
        (*boot).boot.ext.old.rate_any = rate_any;
        if (*boot).boot.ext.old.rate_any as libc::c_int == 2 as libc::c_int {
            (*boot).boot.ext.old.rate_any = 1 as libc::c_int as libc::c_uchar;
        }
        i = 76 as libc::c_int as uint16_t;
        set_word(((*boot).boot.ext.old.Infp0).as_mut_ptr(), i);
        let fresh0 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh0 as usize] = sectors0;
        let fresh1 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh1 as usize] = 108 as libc::c_int as libc::c_uchar;
        j = 1 as libc::c_int as uint8_t;
        while j as libc::c_int <= sectors0 as libc::c_int {
            let fresh2 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh2 as usize] = j;
            j = j.wrapping_add(1);
            j;
        }
        set_word(((*boot).boot.ext.old.InfpX).as_mut_ptr(), i);
        let fresh3 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh3 as usize] = 64 as libc::c_int as libc::c_uchar;
        let fresh4 = i;
        i = i.wrapping_add(1);
        (*boot).bytes[fresh4 as usize] = 3 as libc::c_int as libc::c_uchar;
        let fresh5 = i;
        i = i.wrapping_add(1);
        nb_renum = fresh5 as libc::c_int;
        sector2 = (*dev).sectors as libc::c_int;
        size2 = (*dev).ssize;
        j = 1 as libc::c_int as uint8_t;
        while sector2 != 0 {
            while sector2
                < (1 as libc::c_int) << size2 as libc::c_int >> 2 as libc::c_int
            {
                size2 = size2.wrapping_sub(1);
                size2;
            }
            let fresh6 = i;
            i = i.wrapping_add(1);
            (*boot)
                .bytes[fresh6
                as usize] = (128 as libc::c_int + j as libc::c_int) as libc::c_uchar;
            let fresh7 = j;
            j = j.wrapping_add(1);
            let fresh8 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh8 as usize] = fresh7;
            let fresh9 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh9 as usize] = size2;
            sector2 -= (1 as libc::c_int) << size2 as libc::c_int >> 2 as libc::c_int;
        }
        (*boot)
            .bytes[nb_renum
            as usize] = ((i as libc::c_int - nb_renum - 1 as libc::c_int)
            / 3 as libc::c_int) as uint8_t;
        set_word(((*boot).boot.ext.old.InfTm).as_mut_ptr(), i);
        sector2 = (*dev).sectors as libc::c_int;
        size2 = (*dev).ssize;
        while sector2 != 0 {
            while sector2 < (1 as libc::c_int) << size2 as libc::c_int - 2 as libc::c_int
            {
                size2 = size2.wrapping_sub(1);
                size2;
            }
            let fresh10 = i;
            i = i.wrapping_add(1);
            (*boot).bytes[fresh10 as usize] = size2;
            sector2 -= (1 as libc::c_int) << size2 as libc::c_int - 2 as libc::c_int;
        }
        set_word(((*boot).boot.ext.old.BootP).as_mut_ptr(), i);
        bootOffset = i;
        sum = 0 as libc::c_int;
        j = 64 as libc::c_int as uint8_t;
        while (j as libc::c_int) < i as libc::c_int {
            sum += (*boot).bytes[j as usize] as libc::c_int;
            j = j.wrapping_add(1);
            j;
        }
        (*boot).boot.ext.old.CheckSum = -sum as libc::c_uchar;
        return bootOffset;
    } else {
        if keepBoot == 0 {
            (*boot)
                .boot
                .jump[0 as libc::c_int as usize] = 0xeb as libc::c_int as libc::c_uchar;
            (*boot)
                .boot
                .jump[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
            (*boot)
                .boot
                .jump[2 as libc::c_int as usize] = 0x90 as libc::c_int as libc::c_uchar;
            strncpy(
                ((*boot).boot.banner).as_mut_ptr(),
                mformat_banner,
                8 as libc::c_int as libc::c_ulong,
            );
        }
        return 0 as libc::c_int as uint16_t;
    };
}
static mut bootprog: [libc::c_uchar; 47] = [
    0xfa as libc::c_int as libc::c_uchar,
    0x31 as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0xd8 as libc::c_int as libc::c_uchar,
    0x8e as libc::c_int as libc::c_uchar,
    0xc0 as libc::c_int as libc::c_uchar,
    0xfc as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0xbe as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xbf as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0xf3 as libc::c_int as libc::c_uchar,
    0xa5 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x8 as libc::c_int as libc::c_uchar,
    0xb8 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0x2 as libc::c_int as libc::c_uchar,
    0xbb as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0xba as libc::c_int as libc::c_uchar,
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xb9 as libc::c_int as libc::c_uchar,
    0x1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x13 as libc::c_int as libc::c_uchar,
    0x72 as libc::c_int as libc::c_uchar,
    0x5 as libc::c_int as libc::c_uchar,
    0xea as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0x7c as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0xcd as libc::c_int as libc::c_uchar,
    0x19 as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern "C" fn inst_boot_prg(mut boot: *mut bootsector, mut offset: uint16_t) {
    memcpy(
        ((*boot).bytes).as_mut_ptr().offset(offset as libc::c_int as isize)
            as *mut libc::c_void,
        bootprog.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 47]>() as libc::c_ulong,
    );
    if (offset as libc::c_int - 2 as libc::c_int) < 0x80 as libc::c_int {
        (*boot)
            .boot
            .jump[0 as libc::c_int as usize] = 0xeb as libc::c_int as libc::c_uchar;
        (*boot)
            .boot
            .jump[1 as libc::c_int
            as usize] = (offset as libc::c_int - 2 as libc::c_int) as uint8_t;
        (*boot)
            .boot
            .jump[2 as libc::c_int as usize] = 0x90 as libc::c_int as libc::c_uchar;
    } else {
        (*boot)
            .boot
            .jump[0 as libc::c_int as usize] = 0xe9 as libc::c_int as libc::c_uchar;
        (*boot)
            .boot
            .jump[1 as libc::c_int
            as usize] = (offset as libc::c_int - 3 as libc::c_int) as uint8_t;
        (*boot)
            .boot
            .jump[2 as libc::c_int
            as usize] = (offset as libc::c_int - 3 as libc::c_int >> 8 as libc::c_int)
            as uint8_t;
    }
    set_word(
        ((*boot).boot.jump)
            .as_mut_ptr()
            .offset(offset as libc::c_int as isize)
            .offset(20 as libc::c_int as isize),
        (offset as libc::c_int + 24 as libc::c_int) as libc::c_ushort,
    );
}
#[inline]
unsafe extern "C" fn format_root(
    mut Fs: *mut Fs_t,
    mut label: *mut libc::c_char,
    mut boot: *mut bootsector,
) {
    let mut RootDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut ch: ClashHandling_t = ClashHandling_t {
        action: [NAMEMATCH_NONE; 2],
        namematch_default: [NAMEMATCH_NONE; 2],
        nowarn: 0,
        got_slots: 0,
        mod_time: 0,
        myname: 0 as *mut libc::c_char,
        dosname: 0 as *mut libc::c_uchar,
        single: 0,
        use_longname: 0,
        ignore_entry: 0,
        source: 0,
        source_entry: 0,
        name_converter: None,
        is_label: 0,
    };
    let mut dirlen: libc::c_uint = 0;
    init_clash_handling(&mut ch);
    ch
        .name_converter = Some(
        label_name_uc
            as unsafe extern "C" fn(
                *mut doscp_t,
                *const libc::c_char,
                libc::c_int,
                *mut libc::c_int,
                *mut dos_name_t,
            ) -> (),
    );
    ch.ignore_entry = -(2 as libc::c_int);
    buf = safe_malloc((*Fs).sector_size as size_t) as *mut libc::c_char;
    RootDir = OpenRoot(Fs as *mut Stream_t);
    if RootDir.is_null() {
        fprintf(
            stderr,
            b"Could not open root directory\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    memset(buf as *mut libc::c_void, '\0' as i32, (*Fs).sector_size as libc::c_ulong);
    if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
        dirlen = (*Fs).cluster_size as libc::c_uint;
        fatAllocate(Fs, (*Fs).rootCluster, (*Fs).end_fat);
    } else {
        dirlen = (*Fs).dir_len as libc::c_uint;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < dirlen {
        ((*(*RootDir).Class).pwrite)
            .unwrap()(RootDir, buf, sectorsToBytes(Fs, i), (*Fs).sector_size as size_t);
        i = i.wrapping_add(1);
        i;
    }
    ch.ignore_entry = 1 as libc::c_int;
    if *label.offset(0 as libc::c_int as isize) != 0 {
        mwrite_one(
            RootDir,
            label,
            0 as *const libc::c_char,
            Some(
                labelit
                    as unsafe extern "C" fn(
                        *mut dos_name_t,
                        *mut libc::c_char,
                        *mut libc::c_void,
                        *mut direntry_t,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
            &mut ch,
        );
    }
    free_stream(&mut RootDir);
    if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
        set_word(
            ((*boot).boot.dirents).as_mut_ptr(),
            0 as libc::c_int as libc::c_ushort,
        );
    } else {
        set_word(
            ((*boot).boot.dirents).as_mut_ptr(),
            ((*Fs).dir_len as libc::c_int
                * ((*Fs).sector_size as libc::c_int / 32 as libc::c_int)) as uint16_t,
        );
    }
    free(buf as *mut libc::c_void);
}
unsafe extern "C" fn calc_fat_len(
    mut Fs: *mut Fs_t,
    mut tot_sectors: uint32_t,
) -> libc::c_int {
    let mut rem_sect: uint32_t = 0;
    let mut numerator: uint32_t = 0;
    let mut denominator: uint32_t = 0;
    let mut corr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut clus_start: uint32_t = 0;
    let mut fat_nybbles: libc::c_uint = 0;
    if (*Fs).fat_bits != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"Fs->fat_bits != 0\0" as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"int calc_fat_len(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_11407: {
        if (*Fs).fat_bits != 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"Fs->fat_bits != 0\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"int calc_fat_len(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    (*Fs).fat_len = 0 as libc::c_int as uint32_t;
    clus_start = calc_clus_start(Fs);
    if tot_sectors < clus_start {
        return -(2 as libc::c_int);
    }
    rem_sect = tot_sectors.wrapping_sub(clus_start);
    if rem_sect.wrapping_rem(2 as libc::c_int as libc::c_uint)
        == 1 as libc::c_int as libc::c_uint
        && (*Fs).num_fat as libc::c_int % 2 as libc::c_int == 0 as libc::c_int
        && (*Fs).cluster_size as libc::c_int % 2 as libc::c_int == 0 as libc::c_int
    {
        rem_sect = rem_sect.wrapping_sub(1);
        rem_sect;
    }
    fat_nybbles = ((*Fs).fat_bits).wrapping_div(4 as libc::c_int as libc::c_uint);
    numerator = rem_sect
        .wrapping_add(
            (2 as libc::c_int * (*Fs).cluster_size as libc::c_int) as libc::c_uint,
        );
    denominator = (((*Fs).cluster_size as libc::c_int * (*Fs).sector_size as libc::c_int
        * 2 as libc::c_int) as libc::c_uint)
        .wrapping_add(((*Fs).num_fat as libc::c_uint).wrapping_mul(fat_nybbles));
    if fat_nybbles == 3 as libc::c_int as libc::c_uint {
        if rem_sect > (256 as libc::c_int * 0xff5 as libc::c_int) as libc::c_uint {
            return 1 as libc::c_int;
        }
        numerator = (numerator as libc::c_uint).wrapping_mul(fat_nybbles) as uint32_t
            as uint32_t;
    } else {
        denominator = denominator.wrapping_div(fat_nybbles);
    }
    if rem_sect > denominator {
        numerator = (numerator as libc::c_uint).wrapping_sub(denominator) as uint32_t
            as uint32_t;
        corr = corr.wrapping_add(1);
        corr;
    }
    (*Fs)
        .fat_len = numerator
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(denominator)
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_add(corr);
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn clusters_fit_into_fat(mut Fs: *mut Fs_t) -> bool {
    return ((*Fs).num_clus)
        .wrapping_add(2 as libc::c_int as libc::c_uint)
        .wrapping_mul(((*Fs).fat_bits).wrapping_div(4 as libc::c_int as libc::c_uint))
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(
            ((*Fs).sector_size as libc::c_int * 2 as libc::c_int) as libc::c_uint,
        ) < (*Fs).fat_len;
}
unsafe extern "C" fn check_fs_params_and_set_fat(
    mut Fs: *mut Fs_t,
    mut tot_sectors: uint32_t,
) {
    let mut provisional_fat_bits: libc::c_uint = 0;
    if (if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
        ((*Fs).dir_len as libc::c_int == 0 as libc::c_int) as libc::c_int
    } else {
        ((*Fs).dir_len as libc::c_int != 0 as libc::c_int) as libc::c_int
    }) != 0
    {} else {
        __assert_fail(
            b"Fs->fat_bits == 32 ? (Fs->dir_len == 0) : (Fs->dir_len != 0)\0"
                as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_10224: {
        if (if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
            ((*Fs).dir_len as libc::c_int == 0 as libc::c_int) as libc::c_int
        } else {
            ((*Fs).dir_len as libc::c_int != 0 as libc::c_int) as libc::c_int
        }) != 0
        {} else {
            __assert_fail(
                b"Fs->fat_bits == 32 ? (Fs->dir_len == 0) : (Fs->dir_len != 0)\0"
                    as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                341 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if tot_sectors
        >= ((*Fs).clus_start)
            .wrapping_add(
                ((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as libc::c_uint),
            )
    {} else {
        __assert_fail(
            b"tot_sectors >= Fs->clus_start + Fs->num_clus * Fs->cluster_size\0"
                as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_10159: {
        if tot_sectors
            >= ((*Fs).clus_start)
                .wrapping_add(
                    ((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as libc::c_uint),
                )
        {} else {
            __assert_fail(
                b"tot_sectors >= Fs->clus_start + Fs->num_clus * Fs->cluster_size\0"
                    as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if tot_sectors
        <= ((*Fs).clus_start)
            .wrapping_add(
                ((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as libc::c_uint),
            )
            .wrapping_add((*Fs).cluster_size as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"tot_sectors <= Fs->clus_start + Fs->num_clus * Fs->cluster_size + Fs->cluster_size - 1\0"
                as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_10076: {
        if tot_sectors
            <= ((*Fs).clus_start)
                .wrapping_add(
                    ((*Fs).num_clus).wrapping_mul((*Fs).cluster_size as libc::c_uint),
                )
                .wrapping_add((*Fs).cluster_size as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"tot_sectors <= Fs->clus_start + Fs->num_clus * Fs->cluster_size + Fs->cluster_size - 1\0"
                    as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                349 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if clusters_fit_into_fat(Fs) {} else {
        __assert_fail(
            b"clusters_fit_into_fat(Fs)\0" as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            352 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_9993: {
        if clusters_fit_into_fat(Fs) {} else {
            __assert_fail(
                b"clusters_fit_into_fat(Fs)\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                352 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    provisional_fat_bits = (*Fs).fat_bits;
    set_fat(Fs, provisional_fat_bits == 32 as libc::c_int as libc::c_uint);
    if provisional_fat_bits == (*Fs).fat_bits {} else {
        __assert_fail(
            b"provisional_fat_bits == Fs->fat_bits\0" as *const u8
                as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            357 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                .as_ptr(),
        );
    }
    'c_9927: {
        if provisional_fat_bits == (*Fs).fat_bits {} else {
            __assert_fail(
                b"provisional_fat_bits == Fs->fat_bits\0" as *const u8
                    as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                357 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"void check_fs_params_and_set_fat(Fs_t *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn fat32_specific_init(mut Fs: *mut Fs_t) {
    (*Fs).primaryFat = 0 as libc::c_int as uint32_t;
    (*Fs).writeAllFats = 1 as libc::c_int as uint32_t;
    if (*Fs).backupBoot == 0 {
        if (*Fs).fat_start as libc::c_int <= 6 as libc::c_int {
            (*Fs)
                .backupBoot = ((*Fs).fat_start as libc::c_int - 1 as libc::c_int)
                as uint16_t;
        } else {
            (*Fs).backupBoot = 6 as libc::c_int as uint16_t;
        }
    }
    if ((*Fs).fat_start as libc::c_int) < 3 as libc::c_int {
        fprintf(
            stderr,
            b"For FAT 32, reserved sectors need to be at least 3\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if (*Fs).fat_start as libc::c_int <= (*Fs).backupBoot as libc::c_int {
        fprintf(
            stderr,
            b"Reserved sectors (%d) must be more than backupBoot (%d)\n\0" as *const u8
                as *const libc::c_char,
            (*Fs).fat_start as libc::c_int,
            (*Fs).backupBoot as libc::c_int,
        );
        (*Fs).backupBoot = 0 as libc::c_int as uint16_t;
    }
}
unsafe extern "C" fn try_cluster_size(
    mut Fs: *mut Fs_t,
    mut tot_sectors: uint32_t,
    mut may_change_boot_size: bool,
    mut may_change_fat_len: bool,
    mut may_change_root_size: bool,
    mut may_pad: bool,
) -> libc::c_int {
    let mut maxClus: uint32_t = 0;
    let mut minClus: uint32_t = 0;
    match (*Fs).fat_bits {
        12 => {
            minClus = 1 as libc::c_int as uint32_t;
            maxClus = 0xff5 as libc::c_int as uint32_t;
        }
        16 => {
            minClus = 4096 as libc::c_int as uint32_t;
            maxClus = 0xfff5 as libc::c_int as uint32_t;
        }
        32 => {
            minClus = 0xfff5 as libc::c_int as uint32_t;
            maxClus = 0xffffff5 as libc::c_int as uint32_t;
        }
        _ => {
            if 0 as libc::c_int != 0
                && !(b"Bad number of FAT bits\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"false && \"Bad number of FAT bits\"\0" as *const u8
                        as *const libc::c_char,
                    b"mformat.c\0" as *const u8 as *const libc::c_char,
                    438 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 67],
                        &[libc::c_char; 67],
                    >(
                        b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_11502: {
                if 0 as libc::c_int != 0
                    && !(b"Bad number of FAT bits\0" as *const u8 as *const libc::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"false && \"Bad number of FAT bits\"\0" as *const u8
                            as *const libc::c_char,
                        b"mformat.c\0" as *const u8 as *const libc::c_char,
                        438 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 67],
                            &[libc::c_char; 67],
                        >(
                            b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            return -(2 as libc::c_int);
        }
    }
    if !(getenv(b"MTOOLS_DEBUG_FAT\0" as *const u8 as *const libc::c_char)).is_null() {
        fprintf(
            stderr,
            b"FAT=%d Cluster=%d%s\n\0" as *const u8 as *const libc::c_char,
            (*Fs).fat_bits,
            (*Fs).cluster_size as libc::c_int,
            if may_pad as libc::c_int != 0 {
                b" may_pad\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if may_change_fat_len {
        let mut fit: libc::c_int = calc_fat_len(Fs, tot_sectors);
        if fit != 0 as libc::c_int {
            return fit;
        }
    }
    loop {
        let mut bwaste: uint32_t = 0;
        let mut waste: uint16_t = 0;
        let mut dir_grow: uint16_t = 0 as libc::c_int as uint16_t;
        if calc_num_clus(Fs, tot_sectors) < 0 as libc::c_int {
            return -(2 as libc::c_int);
        }
        if (*Fs).num_clus < minClus {
            return -(1 as libc::c_int);
        }
        if !may_change_fat_len {
            if (*Fs).num_clus >= 0xffffff5 as libc::c_int as libc::c_uint
                || !clusters_fit_into_fat(Fs)
            {
                return 2 as libc::c_int;
            }
        }
        if (*Fs).num_clus < maxClus {
            break;
        }
        if !may_pad {
            return 1 as libc::c_int;
        }
        bwaste = tot_sectors
            .wrapping_sub((*Fs).clus_start)
            .wrapping_sub(maxClus.wrapping_mul((*Fs).cluster_size as libc::c_uint))
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        if bwaste <= 65535 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"bwaste <= UINT16_MAX\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                510 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_11062: {
            if bwaste <= 65535 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"bwaste <= UINT16_MAX\0" as *const u8 as *const libc::c_char,
                    b"mformat.c\0" as *const u8 as *const libc::c_char,
                    510 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 67],
                        &[libc::c_char; 67],
                    >(
                        b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        waste = bwaste as uint16_t;
        if may_change_root_size {
            dir_grow = (32 as libc::c_int - (*Fs).dir_len as libc::c_int) as uint16_t;
            if dir_grow as libc::c_int > waste as libc::c_int {
                dir_grow = waste;
            }
            waste = (waste as libc::c_int - dir_grow as libc::c_int) as uint16_t;
        }
        if may_change_fat_len as libc::c_int != 0
            && (!may_change_boot_size
                || (*Fs).fat_bits == 12 as libc::c_int as libc::c_uint)
        {
            let mut fat_grow: uint16_t = ((waste as libc::c_int
                + (*Fs).num_fat as libc::c_int - 1 as libc::c_int)
                / (*Fs).num_fat as libc::c_int) as uint16_t;
            let mut dir_shrink: uint16_t = 0 as libc::c_int as uint16_t;
            (*Fs)
                .fat_len = ((*Fs).fat_len as libc::c_uint)
                .wrapping_add(fat_grow as libc::c_uint) as uint32_t as uint32_t;
            dir_shrink = (waste as libc::c_int
                - fat_grow as libc::c_int * (*Fs).num_fat as libc::c_int) as uint16_t;
            if dir_shrink as libc::c_int > dir_grow as libc::c_int {
                dir_shrink = dir_grow;
            }
            dir_grow = (dir_grow as libc::c_int - dir_shrink as libc::c_int) as uint16_t;
        } else if may_change_boot_size {
            (*Fs)
                .fat_start = ((*Fs).fat_start as libc::c_int + waste as libc::c_int)
                as uint16_t;
        }
        (*Fs)
            .dir_len = ((*Fs).dir_len as libc::c_int + dir_grow as libc::c_int)
            as uint16_t;
        may_pad = 0 as libc::c_int != 0;
    }
    if (*Fs).num_clus >= minClus {} else {
        __assert_fail(
            b"Fs->num_clus >= minClus\0" as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            544 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_10849: {
        if (*Fs).num_clus >= minClus {} else {
            __assert_fail(
                b"Fs->num_clus >= minClus\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                544 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*Fs).num_clus < maxClus {} else {
        __assert_fail(
            b"Fs->num_clus < maxClus\0" as *const u8 as *const libc::c_char,
            b"mformat.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_10804: {
        if (*Fs).num_clus < maxClus {} else {
            __assert_fail(
                b"Fs->num_clus < maxClus\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                545 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 67],
                    &[libc::c_char; 67],
                >(
                    b"int try_cluster_size(Fs_t *, uint32_t, _Bool, _Bool, _Bool, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn calc_fs_parameters(
    mut dev: *mut device,
    mut fat32Requested: bool,
    mut tot_sectors: uint32_t,
    mut Fs: *mut Fs_t,
    mut descr: *mut uint8_t,
) -> libc::c_int {
    let mut may_change_boot_size: bool = (*Fs).fat_start as libc::c_int
        == 0 as libc::c_int;
    let mut may_change_fat_bits: bool = (*dev).fat_bits == 0 as libc::c_int
        && !fat32Requested;
    let mut may_change_cluster_size: bool = (*Fs).cluster_size as libc::c_int
        == 0 as libc::c_int;
    let mut may_change_root_size: bool = (*Fs).dir_len as libc::c_int
        == 0 as libc::c_int;
    let mut may_change_fat_len: bool = (*Fs).fat_len == 0 as libc::c_int as libc::c_uint;
    let mut may_pad: bool = 0 as libc::c_int != 0;
    let mut saved_dir_len: uint16_t = 0;
    let mut params: *mut OldDos_t = 0 as *mut OldDos_t;
    if fat32Requested {
        if (*dev).fat_bits != 0 && (*dev).fat_bits != 32 as libc::c_int {
            fprintf(
                stderr,
                b"Fat bits 32 requested on command line, but %d in device description\n\0"
                    as *const u8 as *const libc::c_char,
                (*dev).fat_bits,
            );
            exit(1 as libc::c_int);
        }
        (*dev).fat_bits = 32 as libc::c_int;
    }
    (*Fs).infoSectorLoc = 0 as libc::c_int as uint32_t;
    if (may_change_fat_bits as libc::c_int != 0
        || (if (*dev).fat_bits > 0 as libc::c_int {
            (*dev).fat_bits
        } else {
            -(*dev).fat_bits
        }) as libc::c_uint == 12 as libc::c_int as libc::c_uint)
        && (may_change_boot_size as libc::c_int != 0
            || (*Fs).fat_start as libc::c_int == 1 as libc::c_int)
    {
        params = getOldDosByParams(
            (*dev).tracks,
            (*dev).heads as libc::c_uint,
            (*dev).sectors as libc::c_uint,
            (*Fs).dir_len as libc::c_uint,
            (*Fs).cluster_size as libc::c_uint,
        );
    }
    if !params.is_null() {
        let mut num_clus_valid: libc::c_int = 0;
        *descr = (*params).media;
        (*Fs).fat_start = 1 as libc::c_int as uint16_t;
        (*Fs).cluster_size = (*params).cluster_size;
        (*Fs).dir_len = (*params).dir_len;
        (*Fs).fat_len = (*params).fat_len;
        (*Fs).fat_bits = 12 as libc::c_int as libc::c_uint;
        num_clus_valid = calc_num_clus(Fs, tot_sectors);
        if num_clus_valid >= 0 as libc::c_int {} else {
            __assert_fail(
                b"num_clus_valid >= 0\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                597 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_11923: {
            if num_clus_valid >= 0 as libc::c_int {} else {
                __assert_fail(
                    b"num_clus_valid >= 0\0" as *const u8 as *const libc::c_char,
                    b"mformat.c\0" as *const u8 as *const libc::c_char,
                    597 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        check_fs_params_and_set_fat(Fs, tot_sectors);
        return 0 as libc::c_int;
    }
    if (*dev).hidden != 0
        || tot_sectors
            .wrapping_rem(
                ((*dev).sectors as libc::c_int * (*dev).heads as libc::c_int)
                    as libc::c_uint,
            ) != 0
    {
        *descr = 0xf8 as libc::c_int as uint8_t;
    } else {
        *descr = 0xf0 as libc::c_int as uint8_t;
    }
    (*Fs)
        .fat_bits = (if (*dev).fat_bits > 0 as libc::c_int {
        (*dev).fat_bits
    } else {
        -(*dev).fat_bits
    }) as libc::c_uint;
    if (*Fs).fat_bits == 0 as libc::c_int as libc::c_uint {
        (*Fs).fat_bits = 12 as libc::c_int as libc::c_uint;
    }
    if (*Fs).cluster_size == 0 {
        if tot_sectors < 2400 as libc::c_int as libc::c_uint
            && (*dev).heads as libc::c_int == 2 as libc::c_int
        {
            (*Fs).cluster_size = 2 as libc::c_int as uint8_t;
        } else if may_change_fat_len as libc::c_int != 0
            && (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint
        {
            (*Fs).cluster_size = 8 as libc::c_int as uint8_t;
        } else {
            (*Fs).cluster_size = 1 as libc::c_int as uint8_t;
        }
    }
    if (*Fs).dir_len == 0 {
        if tot_sectors < 1200 as libc::c_int as libc::c_uint {
            if (*dev).heads as libc::c_int == 1 as libc::c_int {
                (*Fs).dir_len = 4 as libc::c_int as uint16_t;
            } else {
                (*Fs).dir_len = 7 as libc::c_int as uint16_t;
            }
        } else if tot_sectors <= 3840 as libc::c_int as libc::c_uint {
            (*Fs).dir_len = 14 as libc::c_int as uint16_t;
        } else if tot_sectors <= 7680 as libc::c_int as libc::c_uint {
            (*Fs).dir_len = 15 as libc::c_int as uint16_t;
        } else {
            (*Fs).dir_len = 32 as libc::c_int as uint16_t;
        }
    }
    saved_dir_len = (*Fs).dir_len;
    loop {
        let mut fit: libc::c_int = 0;
        if may_change_boot_size {
            if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
                (*Fs).fat_start = 32 as libc::c_int as uint16_t;
            } else {
                (*Fs).fat_start = 1 as libc::c_int as uint16_t;
            }
        }
        if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
            (*Fs).dir_len = 0 as libc::c_int as uint16_t;
        } else if (*Fs).dir_len as libc::c_int == 0 as libc::c_int {
            (*Fs).dir_len = saved_dir_len;
        }
        if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint
            && may_change_cluster_size as libc::c_int != 0
            && may_change_fat_len as libc::c_int != 0
        {
            (*Fs)
                .cluster_size = (if tot_sectors
                >= (32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                    * 2 as libc::c_int) as libc::c_uint
            {
                64 as libc::c_int
            } else if tot_sectors
                >= (16 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                    * 2 as libc::c_int) as libc::c_uint
            {
                32 as libc::c_int
            } else if tot_sectors
                >= (8 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
                    * 2 as libc::c_int) as libc::c_uint
            {
                16 as libc::c_int
            } else {
                (*Fs).cluster_size as libc::c_int
            }) as uint8_t;
        }
        fit = try_cluster_size(
            Fs,
            tot_sectors,
            may_change_boot_size,
            may_change_fat_len,
            may_change_root_size,
            may_pad,
        );
        if !(getenv(b"MTOOLS_DEBUG_FAT\0" as *const u8 as *const libc::c_char)).is_null()
        {
            fprintf(stderr, b" fit=%d\n\0" as *const u8 as *const libc::c_char, fit);
        }
        if fit == 0 as libc::c_int {
            break;
        }
        if fit == -(2 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if fit != 2 as libc::c_int || !may_change_fat_len {} else {
            __assert_fail(
                b"fit != 2 || !may_change_fat_len\0" as *const u8 as *const libc::c_char,
                b"mformat.c\0" as *const u8 as *const libc::c_char,
                695 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 83],
                    &[libc::c_char; 83],
                >(
                    b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_10682: {
            if fit != 2 as libc::c_int || !may_change_fat_len {} else {
                __assert_fail(
                    b"fit != 2 || !may_change_fat_len\0" as *const u8
                        as *const libc::c_char,
                    b"mformat.c\0" as *const u8 as *const libc::c_char,
                    695 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 83],
                        &[libc::c_char; 83],
                    >(
                        b"int calc_fs_parameters(struct device *, _Bool, uint32_t, struct Fs_t *, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if fit < 0 as libc::c_int {
            if may_change_cluster_size as libc::c_int != 0
                && may_change_fat_len as libc::c_int != 0
                && (*Fs).cluster_size as libc::c_int > 1 as libc::c_int
            {
                (*Fs)
                    .cluster_size = ((*Fs).cluster_size as libc::c_int
                    / 2 as libc::c_int) as uint8_t;
            } else {
                if fat32Requested {
                    break;
                }
                if !may_change_fat_bits
                    && (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint
                {
                    fat32Requested = 1 as libc::c_int != 0;
                    break;
                } else {
                    if !may_change_fat_bits
                        || (*Fs).fat_bits == 12 as libc::c_int as libc::c_uint
                    {
                        return -(2 as libc::c_int);
                    }
                    match (*Fs).fat_bits {
                        16 => {
                            (*Fs).fat_bits = 12 as libc::c_int as libc::c_uint;
                        }
                        32 => {
                            (*Fs).fat_bits = 16 as libc::c_int as libc::c_uint;
                        }
                        _ => {}
                    }
                    may_pad = 1 as libc::c_int != 0;
                }
            }
        } else {
            if fit == 1 as libc::c_int && may_change_fat_bits as libc::c_int != 0
                && !may_pad
            {
                if (*Fs).fat_bits == 12 as libc::c_int as libc::c_uint
                    && (!may_change_cluster_size
                        || (*Fs).cluster_size as libc::c_int >= 8 as libc::c_int)
                {
                    (*Fs).fat_bits = 16 as libc::c_int as libc::c_uint;
                    if may_change_cluster_size {
                        (*Fs).cluster_size = 1 as libc::c_int as uint8_t;
                    }
                    continue;
                } else if (*Fs).fat_bits == 16 as libc::c_int as libc::c_uint
                    && (!may_change_cluster_size
                        || (*Fs).cluster_size as libc::c_int >= 64 as libc::c_int)
                {
                    (*Fs).fat_bits = 32 as libc::c_int as libc::c_uint;
                    if may_change_cluster_size {
                        (*Fs)
                            .cluster_size = (if may_change_fat_len as libc::c_int != 0 {
                            8 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) as uint8_t;
                    }
                    continue;
                }
            }
            if may_change_cluster_size as libc::c_int != 0
                && ((*Fs).cluster_size as libc::c_int) < 128 as libc::c_int
            {
                (*Fs)
                    .cluster_size = (2 as libc::c_int
                    * (*Fs).cluster_size as libc::c_int) as uint8_t;
            } else if fit == 2 as libc::c_int && may_change_fat_bits as libc::c_int != 0
                && may_change_root_size as libc::c_int != 0
                && (*Fs).fat_bits == 16 as libc::c_int as libc::c_uint
            {
                (*Fs).fat_bits = 12 as libc::c_int as libc::c_uint;
                may_pad = 1 as libc::c_int != 0;
            } else {
                return if fit == 2 as libc::c_int {
                    -(4 as libc::c_int)
                } else {
                    -(3 as libc::c_int)
                }
            }
        }
    }
    if !(getenv(b"MTOOLS_DEBUG_FAT\0" as *const u8 as *const libc::c_char)).is_null()
        || !(getenv(b"MTOOLS_DEBUG_FAT_SUMMARY\0" as *const u8 as *const libc::c_char))
            .is_null()
    {
        fprintf(
            stderr,
            b" FAT%d Cluster_size=%d %d clusters FAT_LEN=%d\n\0" as *const u8
                as *const libc::c_char,
            (*Fs).fat_bits,
            (*Fs).cluster_size as libc::c_int,
            (*Fs).num_clus,
            (*Fs).fat_len,
        );
    }
    check_fs_params_and_set_fat(Fs, tot_sectors);
    if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
        fat32_specific_init(Fs);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn initFsForFormat(mut Fs: *mut Fs_t) {
    memset(
        Fs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Fs_t>() as libc::c_ulong,
    );
    init_head(&mut (*Fs).head, &mut FsClass, 0 as *mut Stream_t);
    (*Fs).cluster_size = 0 as libc::c_int as uint8_t;
    (*Fs).dir_len = 0 as libc::c_int as uint16_t;
    (*Fs).fat_len = 0 as libc::c_int as uint32_t;
    (*Fs).num_fat = 2 as libc::c_int as uint8_t;
    (*Fs).backupBoot = 0 as libc::c_int as uint16_t;
}
pub unsafe extern "C" fn setFsSectorSize(
    mut Fs: *mut Fs_t,
    mut dev: *mut device,
    mut msize: uint16_t,
) {
    let mut j: libc::c_uint = 0;
    (*Fs).sector_size = 512 as libc::c_int as uint16_t;
    if (*dev).use_2m & 0x7f as libc::c_int as libc::c_uint == 0 {
        (*Fs)
            .sector_size = ((128 as libc::c_uint)
            << ((*dev).ssize as libc::c_int & 0x7f as libc::c_int)) as uint16_t;
    }
    if msize != 0 {
        (*Fs).sector_size = msize;
    }
    j = 0 as libc::c_int as libc::c_uint;
    while j < 31 as libc::c_int as libc::c_uint {
        if (*Fs).sector_size as libc::c_uint == ((1 as libc::c_int) << j) as libc::c_uint
        {
            (*Fs).sectorShift = j;
            break;
        } else {
            j = j.wrapping_add(1);
            j;
        }
    }
    (*Fs)
        .sectorMask = ((*Fs).sector_size as libc::c_int - 1 as libc::c_int)
        as libc::c_uint;
}
unsafe extern "C" fn old_dos_size_to_geom(
    mut size: size_t,
    mut cyls: *mut libc::c_uint,
    mut heads: *mut libc::c_ushort,
    mut sects: *mut libc::c_ushort,
) -> libc::c_int {
    let mut params: *mut OldDos_t = getOldDosBySize(size);
    if !params.is_null() {
        *cyls = (*params).tracks;
        *heads = (*params).heads;
        *sects = (*params).sectors;
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn usage(mut ret: libc::c_int) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-V] [-t tracks] [-h heads] [-n sectors] [-v label] [-1] [-4] [-8] [-f size] [-N serialnumber] [-k] [-B bootsector] [-r root_dir_len] [-L fat_len] [-F] [-I fsVersion] [-C] [-c cluster_size] [-H hidden_sectors] [-X] [-S hardsectorsize] [-M softsectorsize] [-3] [-2 track0sectors] [-0 rate0] [-A rateany] [-a]device\n\0"
            as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
pub unsafe extern "C" fn mformat(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut dummy: libc::c_int,
) {
    let mut r: libc::c_int = 0;
    let mut Fs: *mut Fs_t = 0 as *mut Fs_t;
    let mut hs: libc::c_uint = 0;
    let mut hs_set: libc::c_int = 0;
    let mut arguse_2m: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut sectors0: uint8_t = 18 as libc::c_int as uint8_t;
    let mut create: libc::c_int = 0 as libc::c_int;
    let mut rate_0: uint8_t = 0;
    let mut rate_any: uint8_t = 0;
    let mut mangled: libc::c_int = 0;
    let mut argssize: uint8_t = 0 as libc::c_int as uint8_t;
    let mut msize: uint16_t = 0 as libc::c_int as uint16_t;
    let mut fat32: libc::c_int = 0 as libc::c_int;
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    let mut bootOffset: size_t = 0;
    let mut i: libc::c_uint = 0;
    let mut format_xdf: libc::c_int = 0 as libc::c_int;
    let mut info: xdf_info = xdf_info {
        FatSize: 0,
        RootDirSize: 0,
        BadSectors: 0,
    };
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut bootSector: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut keepBoot: libc::c_int = 0 as libc::c_int;
    let mut used_dev: device = device {
        name: 0 as *const libc::c_char,
        drive: 0,
        fat_bits: 0,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0,
        ssize: 0,
        use_2m: 0,
        precmd: 0 as *mut libc::c_char,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: 0 as *const libc::c_char,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: 0 as *mut libc::c_char,
        cfg_filename: 0 as *const libc::c_char,
    };
    let mut argtracks: libc::c_uint = 0;
    let mut argheads: uint16_t = 0;
    let mut argsectors: uint16_t = 0;
    let mut tot_sectors: uint32_t = 0 as libc::c_int as uint32_t;
    let mut blocksize: uint32_t = 0;
    let mut drive: libc::c_char = 0;
    let mut name: [libc::c_char; 2048] = [0; 2048];
    let mut label: [libc::c_char; 261] = [0; 261];
    let mut shortlabel: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    let mut dev: *mut device = 0 as *mut device;
    let mut errmsg: [libc::c_char; 2100] = [0; 2100];
    let mut serial: uint32_t = 0;
    let mut serial_set: libc::c_int = 0;
    let mut fsVersion: uint16_t = 0;
    let mut mediaDesc: uint8_t = 0 as libc::c_int as uint8_t;
    let mut haveMediaDesc: bool = 0 as libc::c_int != 0;
    let mut biosDisk: uint8_t = 0 as libc::c_int as uint8_t;
    let mut haveBiosDisk: bool = 0 as libc::c_int != 0;
    let mut maxSize: mt_off_t = 0;
    let mut Atari: libc::c_int = 0 as libc::c_int;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    hs_set = 0 as libc::c_int;
    hs = hs_set as libc::c_uint;
    argtracks = 0 as libc::c_int as libc::c_uint;
    argheads = 0 as libc::c_int as uint16_t;
    argsectors = 0 as libc::c_int as uint16_t;
    arguse_2m = 0 as libc::c_int as libc::c_uint;
    argssize = 0x2 as libc::c_int as uint8_t;
    label[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    serial_set = 0 as libc::c_int;
    serial = 0 as libc::c_int as uint32_t;
    fsVersion = 0 as libc::c_int as uint16_t;
    Fs = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Fs_t>() as libc::c_ulong,
    ) as *mut Fs_t;
    if Fs.is_null() {
        fprintf(stderr, b"Out of memory\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    initFsForFormat(Fs);
    if !(getenv(b"MTOOLS_DIR_LEN\0" as *const u8 as *const libc::c_char)).is_null() {
        (*Fs)
            .dir_len = atou16(
            getenv(b"MTOOLS_DIR_LEN\0" as *const u8 as *const libc::c_char),
        );
        if (*Fs).dir_len as libc::c_int <= 0 as libc::c_int {
            (*Fs).dir_len = 0 as libc::c_int as uint16_t;
        }
    }
    if !(getenv(b"MTOOLS_NFATS\0" as *const u8 as *const libc::c_char)).is_null() {
        (*Fs)
            .num_fat = atou8(
            getenv(b"MTOOLS_NFATS\0" as *const u8 as *const libc::c_char),
        );
        if (*Fs).num_fat as libc::c_int <= 0 as libc::c_int {
            (*Fs).num_fat = 2 as libc::c_int as uint8_t;
        }
    }
    rate_0 = mtools_rate_0;
    rate_any = mtools_rate_any;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(
            argc,
            argv,
            b"i:148f:t:n:v:qub:kK:R:B:r:L:I:FCc:Xh:s:T:l:N:H:M:S:2:30:Aad:m:\0"
                as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        *__errno_location() = 0 as libc::c_int;
        endptr = 0 as *mut libc::c_char;
        let mut current_block_99: u64;
        match c {
            105 => {
                set_cmd_line_image(optarg);
                current_block_99 = 7639320476250304355;
            }
            49 => {
                argheads = 1 as libc::c_int as uint16_t;
                current_block_99 = 7639320476250304355;
            }
            52 => {
                argsectors = 9 as libc::c_int as uint16_t;
                argtracks = 40 as libc::c_int as libc::c_uint;
                current_block_99 = 7639320476250304355;
            }
            56 => {
                argsectors = 8 as libc::c_int as uint16_t;
                argtracks = 40 as libc::c_int as libc::c_uint;
                current_block_99 = 7639320476250304355;
            }
            102 => {
                r = old_dos_size_to_geom(
                    atoul(optarg),
                    &mut argtracks,
                    &mut argheads,
                    &mut argsectors,
                );
                if r != 0 {
                    fprintf(
                        stderr,
                        b"Bad size %s\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    exit(1 as libc::c_int);
                }
                current_block_99 = 7639320476250304355;
            }
            116 => {
                argtracks = atou16(optarg) as libc::c_uint;
                current_block_99 = 7639320476250304355;
            }
            84 => {
                tot_sectors = parseSize(optarg);
                current_block_99 = 7639320476250304355;
            }
            110 | 115 => {
                argsectors = atou16(optarg);
                current_block_99 = 7639320476250304355;
            }
            108 | 118 => {
                strncpy(
                    label.as_mut_ptr(),
                    optarg,
                    (20 as libc::c_int * 13 as libc::c_int + 1 as libc::c_int
                        - 1 as libc::c_int) as libc::c_ulong,
                );
                label[(20 as libc::c_int * 13 as libc::c_int + 1 as libc::c_int
                    - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
                current_block_99 = 7639320476250304355;
            }
            113 | 117 => {
                fprintf(
                    stderr,
                    b"Flag %c not supported by mtools\n\0" as *const u8
                        as *const libc::c_char,
                    c,
                );
                exit(1 as libc::c_int);
            }
            98 => {
                haveBiosDisk = 1 as libc::c_int != 0;
                biosDisk = atou8(optarg);
                current_block_99 = 5843925138985246205;
            }
            70 => {
                current_block_99 = 5843925138985246205;
            }
            83 => {
                argssize = (atou8(optarg) as libc::c_int | 0x80 as libc::c_int)
                    as uint8_t;
                if (argssize as libc::c_int) < 0x80 as libc::c_int {
                    usage(1 as libc::c_int);
                }
                if argssize as libc::c_int >= 0x87 as libc::c_int {
                    fprintf(
                        stderr,
                        b"argssize must be less than 6\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                current_block_99 = 7639320476250304355;
            }
            88 => {
                format_xdf = 1 as libc::c_int;
                current_block_99 = 7639320476250304355;
            }
            50 => {
                arguse_2m = 0xff as libc::c_int as libc::c_uint;
                sectors0 = atou8(optarg);
                current_block_99 = 7639320476250304355;
            }
            51 => {
                arguse_2m = 0x80 as libc::c_int as libc::c_uint;
                current_block_99 = 7639320476250304355;
            }
            48 => {
                rate_0 = atou8(optarg);
                current_block_99 = 7639320476250304355;
            }
            65 => {
                rate_any = atou8(optarg);
                current_block_99 = 7639320476250304355;
            }
            77 => {
                msize = atou16(optarg);
                if msize as libc::c_int != 512 as libc::c_int
                    && msize as libc::c_int != 1024 as libc::c_int
                    && msize as libc::c_int != 2048 as libc::c_int
                    && msize as libc::c_int != 4096 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"Only sector sizes of 512, 1024, 2048 or 4096 bytes are allowed\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    usage(1 as libc::c_int);
                }
                current_block_99 = 7639320476250304355;
            }
            78 => {
                serial = strtou32(optarg, &mut endptr, 16 as libc::c_int);
                serial_set = 1 as libc::c_int;
                current_block_99 = 7639320476250304355;
            }
            97 => {
                Atari = 1 as libc::c_int;
                current_block_99 = 7639320476250304355;
            }
            67 => {
                create = 0o100 as libc::c_int | 0o1000 as libc::c_int;
                current_block_99 = 7639320476250304355;
            }
            72 => {
                hs = atoui(optarg);
                hs_set = 1 as libc::c_int;
                current_block_99 = 7639320476250304355;
            }
            73 => {
                fsVersion = strtou16(optarg, &mut endptr, 0 as libc::c_int);
                current_block_99 = 7639320476250304355;
            }
            99 => {
                (*Fs).cluster_size = atou8(optarg);
                current_block_99 = 7639320476250304355;
            }
            114 => {
                (*Fs).dir_len = strtou16(optarg, &mut endptr, 0 as libc::c_int);
                current_block_99 = 7639320476250304355;
            }
            76 => {
                (*Fs).fat_len = strtoui(optarg, &mut endptr, 0 as libc::c_int);
                current_block_99 = 7639320476250304355;
            }
            66 => {
                bootSector = optarg;
                current_block_99 = 7639320476250304355;
            }
            107 => {
                keepBoot = 1 as libc::c_int;
                current_block_99 = 7639320476250304355;
            }
            75 => {
                (*Fs).backupBoot = atou16(optarg);
                if ((*Fs).backupBoot as libc::c_int) < 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Backupboot must be greater than 2\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                current_block_99 = 7639320476250304355;
            }
            82 => {
                (*Fs).fat_start = atou8(optarg) as uint16_t;
                current_block_99 = 7639320476250304355;
            }
            104 => {
                argheads = atou16(optarg);
                current_block_99 = 7639320476250304355;
            }
            100 => {
                (*Fs).num_fat = atou8(optarg);
                current_block_99 = 7639320476250304355;
            }
            109 => {
                mediaDesc = strtou8(optarg, &mut endptr, 0 as libc::c_int);
                if *endptr != 0 {
                    mediaDesc = strtou8(optarg, &mut endptr, 16 as libc::c_int);
                }
                if optarg == endptr || *endptr as libc::c_int != 0 {
                    fprintf(
                        stderr,
                        b"Bad mediadesc %s\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    exit(1 as libc::c_int);
                }
                haveMediaDesc = 1 as libc::c_int != 0;
                current_block_99 = 7639320476250304355;
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
        match current_block_99 {
            5843925138985246205 => {
                fat32 = 1 as libc::c_int;
            }
            _ => {}
        }
        check_number_parse_errno(c as libc::c_char, optarg, endptr);
    }
    if argc - optind > 1 as libc::c_int {
        usage(1 as libc::c_int);
    }
    if argc - optind == 1 as libc::c_int {
        if *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize) == 0
            || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != ':' as i32
        {
            usage(1 as libc::c_int);
        }
        drive = ch_toupper(
            *(*argv.offset((argc - 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize),
        );
    } else {
        drive = get_default_drive();
        if drive as libc::c_int != ':' as i32 {
            fprintf(
                stderr,
                b"Drive letter missing\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if argtracks != 0 && tot_sectors != 0 {
        fprintf(
            stderr,
            b"Only one of -t or -T may be specified\n\0" as *const u8
                as *const libc::c_char,
        );
        usage(1 as libc::c_int);
    }
    if create != 0 && format_xdf != 0 {
        fprintf(
            stderr,
            b"Create and XDF can't be used together\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const libc::c_char,
        drive as libc::c_int,
    );
    blocksize = 0 as libc::c_int as uint32_t;
    dev = devices;
    while (*dev).drive != 0 {
        free_stream(&mut (*Fs).head.Next);
        if !((*dev).drive as libc::c_int != drive as libc::c_int) {
            used_dev = *dev;
            if argtracks != 0 {
                used_dev.tracks = argtracks;
            }
            if argheads != 0 {
                used_dev.heads = argheads;
            }
            if argsectors != 0 {
                used_dev.sectors = argsectors;
            }
            if arguse_2m != 0 {
                used_dev.use_2m = arguse_2m;
            }
            if argssize != 0 {
                used_dev.ssize = argssize;
            }
            if hs_set != 0 {
                used_dev.hidden = hs;
            }
            expand((*dev).name, name.as_mut_ptr());
            if format_xdf != 0 {
                used_dev.misc_flags |= 0x8 as libc::c_uint;
            }
            info.FatSize = 0 as libc::c_int as libc::c_uint;
            if tot_sectors != 0 {
                used_dev.tot_sectors = tot_sectors;
            }
            (*Fs)
                .head
                .Next = OpenImage(
                &mut used_dev,
                dev,
                name.as_mut_ptr(),
                0o2 as libc::c_int | create,
                errmsg.as_mut_ptr(),
                4 as libc::c_int,
                0o2 as libc::c_int,
                &mut maxSize,
                0 as *mut libc::c_int,
                &mut info,
            );
            if !((*Fs).head.Next).is_null() && info.FatSize != 0 {
                if (*Fs).fat_len == 0 {
                    (*Fs).fat_len = info.FatSize;
                }
                if (*Fs).dir_len == 0 {
                    (*Fs).dir_len = info.RootDirSize;
                }
            }
            if !((*Fs).head.Next).is_null() {
                if tot_sectors != 0 {
                    used_dev.tot_sectors = tot_sectors;
                }
                setFsSectorSize(Fs, &mut used_dev, msize);
                if used_dev.blocksize == 0
                    || used_dev.blocksize < (*Fs).sector_size as libc::c_uint
                {
                    blocksize = (*Fs).sector_size as uint32_t;
                } else {
                    blocksize = used_dev.blocksize;
                }
                if blocksize > 8192 as libc::c_int as libc::c_uint {
                    blocksize = 8192 as libc::c_int as uint32_t;
                }
                if chs_to_totsectors(&mut used_dev, errmsg.as_mut_ptr())
                    < 0 as libc::c_int
                    || check_if_sectors_fit(
                        (*dev).tot_sectors,
                        maxSize,
                        blocksize,
                        errmsg.as_mut_ptr(),
                    ) < 0 as libc::c_int
                {
                    free_stream(&mut (*Fs).head.Next);
                } else {
                    if tot_sectors == 0 {
                        tot_sectors = used_dev.tot_sectors;
                    }
                    if !(create == 0
                        && ((*(*(*Fs).head.Next).Class).pread)
                            .unwrap()(
                            (*Fs).head.Next,
                            &mut boot.characters as *mut [libc::c_char; 4096]
                                as *mut libc::c_char,
                            0 as libc::c_int as mt_off_t,
                            (*Fs).sector_size as size_t,
                        ) != (*Fs).sector_size as libc::c_int as libc::c_long)
                    {
                        break;
                    }
                    snprintf(
                        errmsg.as_mut_ptr(),
                        (::std::mem::size_of::<[libc::c_char; 2100]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"Error reading from '%s', wrong parameters?\0" as *const u8
                            as *const libc::c_char,
                        name.as_mut_ptr(),
                    );
                    free_stream(&mut (*Fs).head.Next);
                }
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if (*dev).drive as libc::c_int == 0 as libc::c_int {
        free_stream(&mut (*Fs).head.Next);
        fprintf(
            stderr,
            b"%s: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
            errmsg.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    if tot_sectors == 0 as libc::c_int as libc::c_uint {
        fprintf(stderr, b"Disk size not known\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if create != 0 {
        ((*(*(*Fs).head.Next).Class).pwrite)
            .unwrap()(
            (*Fs).head.Next,
            &mut boot.characters as *mut [libc::c_char; 4096] as *mut libc::c_char,
            sectorsToBytes(
                Fs,
                tot_sectors.wrapping_sub(1 as libc::c_int as libc::c_uint),
            ),
            (*Fs).sector_size as size_t,
        );
    }
    if !bootSector.is_null() {
        let mut fd: libc::c_int = 0;
        let mut ret: ssize_t = 0;
        fd = open(bootSector, 0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int);
        if fd < 0 as libc::c_int {
            perror(b"open boot sector\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        ret = read(
            fd,
            &mut boot.bytes as *mut [libc::c_uchar; 4096] as *mut libc::c_void,
            blocksize as size_t,
        );
        if ret < 0 as libc::c_int as libc::c_long
            || (ret as size_t) < blocksize as libc::c_ulong
        {
            perror(b"short read on boot sector\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        keepBoot = 1 as libc::c_int;
        close(fd);
    }
    if keepBoot == 0 && used_dev.use_2m & 0x7f as libc::c_int as libc::c_uint == 0 {
        memset(
            (boot.characters).as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            (*Fs).sector_size as libc::c_ulong,
        );
    }
    (*Fs)
        .head
        .Next = buf_init(
        (*Fs).head.Next,
        blocksize
            .wrapping_mul(used_dev.heads as libc::c_uint)
            .wrapping_mul(used_dev.sectors as libc::c_uint) as size_t,
        blocksize
            .wrapping_mul(used_dev.heads as libc::c_uint)
            .wrapping_mul(used_dev.sectors as libc::c_uint) as size_t,
        blocksize as size_t,
    );
    boot.boot.nfat = (*Fs).num_fat;
    if keepBoot == 0 {
        set_word(
            &mut *(boot.bytes).as_mut_ptr().offset(510 as libc::c_int as isize),
            0xaa55 as libc::c_int as libc::c_ushort,
        );
    }
    set_word((boot.boot.nsect).as_mut_ptr(), used_dev.sectors);
    set_word((boot.boot.nheads).as_mut_ptr(), used_dev.heads);
    match calc_fs_parameters(
        &mut used_dev,
        fat32 != 0,
        tot_sectors,
        Fs,
        &mut boot.boot.descr,
    ) {
        -1 => {
            fprintf(stderr, b"Too few sectors\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        -2 => {
            fprintf(
                stderr,
                b"Too few clusters for %d bit fat\n\0" as *const u8
                    as *const libc::c_char,
                (*Fs).fat_bits,
            );
            exit(1 as libc::c_int);
        }
        -3 => {
            fprintf(
                stderr,
                b"Too many clusters for %d bit FAT\n\0" as *const u8
                    as *const libc::c_char,
                (*Fs).fat_bits,
            );
            exit(1 as libc::c_int);
        }
        -4 => {
            fprintf(
                stderr,
                b"Too many clusters for fat length %d\n\0" as *const u8
                    as *const libc::c_char,
                (*Fs).fat_len,
            );
            exit(1 as libc::c_int);
        }
        _ => {}
    }
    if keepBoot == 0 && used_dev.use_2m & 0x7f as libc::c_int as libc::c_uint == 0 {
        if used_dev.partition == 0 {
            let mut partTable: *mut partition = &mut *(boot.bytes)
                .as_mut_ptr()
                .offset(0x1ae as libc::c_int as isize) as *mut libc::c_uchar
                as *mut partition;
            setBeginEnd(
                &mut *partTable.offset(1 as libc::c_int as isize),
                0 as libc::c_int as uint32_t,
                ((used_dev.heads as libc::c_int * used_dev.sectors as libc::c_int)
                    as libc::c_uint)
                    .wrapping_mul(used_dev.tracks),
                used_dev.heads as uint8_t as uint16_t,
                used_dev.sectors as uint8_t as uint16_t,
                1 as libc::c_int,
                0 as libc::c_int as uint8_t,
                (*Fs).fat_bits,
            );
        }
    }
    if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint {
        set_word((boot.boot.fatlen).as_mut_ptr(), 0 as libc::c_int as libc::c_ushort);
        set_dword((boot.boot.ext.fat32.bigFat).as_mut_ptr(), (*Fs).fat_len);
        (*Fs)
            .clus_start = ((*Fs).num_fat as libc::c_uint)
            .wrapping_mul((*Fs).fat_len)
            .wrapping_add((*Fs).fat_start as libc::c_uint);
        set_word(
            (boot.boot.ext.fat32.extFlags).as_mut_ptr(),
            0 as libc::c_int as libc::c_ushort,
        );
        set_word((boot.boot.ext.fat32.fsVersion).as_mut_ptr(), fsVersion);
        (*Fs).rootCluster = 2 as libc::c_int as uint32_t;
        set_dword((boot.boot.ext.fat32.rootCluster).as_mut_ptr(), (*Fs).rootCluster);
        (*Fs).infoSectorLoc = 1 as libc::c_int as uint32_t;
        set_word(
            (boot.boot.ext.fat32.infoSector).as_mut_ptr(),
            (*Fs).infoSectorLoc as libc::c_ushort,
        );
        (*Fs).infoSectorLoc = 1 as libc::c_int as uint32_t;
        set_word((boot.boot.ext.fat32.backupBoot).as_mut_ptr(), (*Fs).backupBoot);
        labelBlock = &mut boot.boot.ext.fat32.labelBlock;
    } else {
        set_word((boot.boot.fatlen).as_mut_ptr(), (*Fs).fat_len as uint16_t);
        (*Fs)
            .dir_start = ((*Fs).num_fat as libc::c_uint)
            .wrapping_mul((*Fs).fat_len)
            .wrapping_add((*Fs).fat_start as libc::c_uint);
        (*Fs).clus_start = ((*Fs).dir_start).wrapping_add((*Fs).dir_len as libc::c_uint);
        labelBlock = &mut boot.boot.ext.old.labelBlock;
    }
    (*Fs).cp = cp_open(used_dev.codepage);
    if ((*Fs).cp).is_null() {
        exit(1 as libc::c_int);
    }
    if haveMediaDesc {
        boot.boot.descr = mediaDesc;
    }
    if haveBiosDisk {
        (*labelBlock).physdrive = biosDisk;
    } else if keepBoot == 0 {
        (*labelBlock)
            .physdrive = (if boot.boot.descr as libc::c_int == 0xf8 as libc::c_int {
            0x80 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uchar;
    }
    (*labelBlock).reserved = 0 as libc::c_int as libc::c_uchar;
    (*labelBlock).dos4 = 0x29 as libc::c_int as libc::c_uchar;
    if serial_set == 0 || Atari != 0 {
        init_random();
    }
    if serial_set == 0 {
        serial = random() as uint32_t;
    }
    set_dword(((*labelBlock).serial).as_mut_ptr(), serial);
    label_name_pc(
        ((*(*(Fs as *mut Stream_t)).Class).get_dosConvert).unwrap()(Fs as *mut Stream_t),
        if label[0 as libc::c_int as usize] as libc::c_int != 0 {
            label.as_mut_ptr() as *const libc::c_char
        } else {
            b"NO NAME    \0" as *const u8 as *const libc::c_char
        },
        0 as libc::c_int,
        &mut mangled,
        &mut shortlabel,
    );
    strncpy(
        ((*labelBlock).label).as_mut_ptr(),
        (shortlabel.base).as_mut_ptr(),
        8 as libc::c_int as libc::c_ulong,
    );
    strncpy(
        ((*labelBlock).label).as_mut_ptr().offset(8 as libc::c_int as isize),
        (shortlabel.ext).as_mut_ptr(),
        3 as libc::c_int as libc::c_ulong,
    );
    sprintf(
        ((*labelBlock).fat_type).as_mut_ptr(),
        b"FAT%2.2d  \0" as *const u8 as *const libc::c_char,
        (*Fs).fat_bits,
    );
    (*labelBlock).fat_type[7 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    set_word((boot.boot.secsiz).as_mut_ptr(), (*Fs).sector_size);
    boot.boot.clsiz = (*Fs).cluster_size;
    set_word((boot.boot.nrsvsect).as_mut_ptr(), (*Fs).fat_start);
    bootOffset = init_geometry_boot(
        &mut boot,
        &mut used_dev,
        sectors0,
        rate_0,
        rate_any,
        &mut tot_sectors,
        keepBoot,
    ) as size_t;
    if bootOffset == 0 {
        bootOffset = (ptrdiff(
            labelBlock as *mut libc::c_char,
            (boot.bytes).as_mut_ptr() as *mut libc::c_char,
        ))
            .wrapping_add(::std::mem::size_of::<label_blk_t>() as libc::c_ulong);
    }
    if Atari != 0 {
        boot.boot.banner[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        boot.boot.banner[5 as libc::c_int as usize] = random() as libc::c_char;
        boot.boot.banner[6 as libc::c_int as usize] = random() as libc::c_char;
        boot.boot.banner[7 as libc::c_int as usize] = random() as libc::c_char;
    }
    if keepBoot == 0 && bootOffset <= 65535 as libc::c_int as libc::c_ulong {
        inst_boot_prg(&mut boot, bootOffset as uint16_t);
    }
    if used_dev.use_2m & 0x7f as libc::c_int as libc::c_uint != 0 {
        boot.boot.jump[0 as libc::c_int as usize] = 0xeb as libc::c_int as libc::c_uchar;
        boot.boot.jump[1 as libc::c_int as usize] = 0x80 as libc::c_int as libc::c_uchar;
        boot.boot.jump[2 as libc::c_int as usize] = 0x90 as libc::c_int as libc::c_uchar;
    }
    if used_dev.use_2m & 0x7f as libc::c_int as libc::c_uint != 0 {
        (*Fs).num_fat = 1 as libc::c_int as uint8_t;
    }
    (*Fs).lastFatSectorNr = 0 as libc::c_int as uint32_t;
    (*Fs).lastFatSectorData = 0 as *mut libc::c_uchar;
    zero_fat(Fs, boot.boot.descr);
    (*Fs).freeSpace = (*Fs).num_clus;
    (*Fs).last = 2 as libc::c_int as uint32_t;
    if used_dev.misc_flags & 0x8 as libc::c_uint != 0 {
        i = 0 as libc::c_int as libc::c_uint;
        while i
            < (info.BadSectors)
                .wrapping_add((*Fs).cluster_size as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_div((*Fs).cluster_size as libc::c_uint)
        {
            fatEncode(
                Fs,
                i.wrapping_add(2 as libc::c_int as libc::c_uint),
                0xfff7 as libc::c_int as libc::c_uint,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    format_root(Fs, label.as_mut_ptr(), &mut boot);
    if ((*(*(Fs as *mut Stream_t)).Class).pwrite)
        .unwrap()(
        Fs as *mut Stream_t,
        (boot.characters).as_mut_ptr(),
        0 as libc::c_int as mt_off_t,
        (*Fs).sector_size as size_t,
    ) < 0 as libc::c_int as libc::c_long
    {
        fprintf(
            stderr,
            b"Error writing boot sector\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if (*Fs).fat_bits == 32 as libc::c_int as libc::c_uint
        && (boot.boot.ext.fat32.backupBoot[0 as libc::c_int as usize] as libc::c_int
            + ((boot.boot.ext.fat32.backupBoot[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int != 0xffff as libc::c_int
    {
        if ((*(*(Fs as *mut Stream_t)).Class).pwrite)
            .unwrap()(
            Fs as *mut Stream_t,
            (boot.characters).as_mut_ptr(),
            sectorsToBytes(
                Fs,
                (boot.boot.ext.fat32.backupBoot[0 as libc::c_int as usize] as libc::c_int
                    + ((boot.boot.ext.fat32.backupBoot[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as uint32_t,
            ),
            (*Fs).sector_size as size_t,
        ) < 0 as libc::c_int as libc::c_long
        {
            fprintf(
                stderr,
                b"Error writing backup boot sector\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    free_stream(&mut Fs as *mut *mut Fs_t as *mut *mut Stream_t);
    if format_xdf != 0 && isatty(0 as libc::c_int) != 0
        && (getenv(b"MTOOLS_USE_XDF\0" as *const u8 as *const libc::c_char)).is_null()
    {
        fprintf(
            stderr,
            b"Note:\nRemember to set the \"MTOOLS_USE_XDF\" environmental\nvariable before accessing this disk\n\nBourne shell syntax (sh, ash, bash, ksh, zsh etc):\n export MTOOLS_USE_XDF=1\n\nC shell syntax (csh and tcsh):\n setenv MTOOLS_USE_XDF 1\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    exit(0 as libc::c_int);
}
