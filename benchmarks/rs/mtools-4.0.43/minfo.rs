use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    pub type FatMap_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    fn get_default_drive() -> libc::c_char;
    fn set_cmd_line_image(img: *mut libc::c_char);
    static mut progname: *const libc::c_char;
    fn print_sector(
        message: *const libc::c_char,
        data: *mut libc::c_uchar,
        size: libc::c_int,
    );
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn find_device(
        drive: libc::c_char,
        mode: libc::c_int,
        out_dev: *mut device,
        boot: *mut bootsector,
        name: *mut libc::c_char,
        media: *mut libc::c_int,
        maxSize: *mut mt_off_t,
        isRop: *mut libc::c_int,
    ) -> *mut Stream_t;
    static mut mdate: *const libc::c_char;
    static mut mversion: *const libc::c_char;
    fn calc_fs_parameters(
        dev: *mut device,
        fat32: bool,
        tot_sectors: uint32_t,
        Fs: *mut Fs_t,
        descr: *mut uint8_t,
    ) -> libc::c_int;
    fn initFsForFormat(Fs: *mut Fs_t);
    fn setFsSectorSize(Fs: *mut Fs_t, dev: *mut device, msize: uint16_t);
    fn parseFsParams(
        This: *mut Fs_t,
        boot: *mut bootsector,
        media: libc::c_int,
        cylinder_size: libc::c_uint,
    ) -> uint32_t;
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
pub type mt_off_t = off_t;
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
pub type device_t = device;
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
unsafe extern "C" fn usage(mut ret: libc::c_int) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-v] drive\n\0" as *const u8 as *const libc::c_char,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn displayInfosector(
    mut Stream: *mut Stream_t,
    mut boot: *mut bootsector,
) {
    let mut infosec: *mut InfoSector_t = 0 as *mut InfoSector_t;
    if ((*boot).boot.ext.fat32.infoSector[0 as libc::c_int as usize] as libc::c_int
        + (((*boot).boot.ext.fat32.infoSector[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_int == 0xffff as libc::c_int
    {
        return;
    }
    infosec = safe_malloc(
        ((*boot).boot.secsiz[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as size_t,
    ) as *mut InfoSector_t;
    force_pread(
        Stream,
        infosec as *mut libc::c_char,
        ((*boot).boot.secsiz[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as mt_off_t
            * ((*boot).boot.ext.fat32.infoSector[0 as libc::c_int as usize]
                as libc::c_int
                + (((*boot).boot.ext.fat32.infoSector[1 as libc::c_int as usize]
                    as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_long,
        ((*boot).boot.secsiz[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as size_t,
    );
    printf(b"\nInfosector:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"signature=0x%08x\n\0" as *const u8 as *const libc::c_char,
        (((*infosec).signature1[0 as libc::c_int as usize] as libc::c_int
            + (((*infosec).signature1[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int
            + (((*((*infosec).signature1)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int
                + ((*((*infosec).signature1)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                << 16 as libc::c_int)) as uint32_t,
    );
    if (((*infosec).count[0 as libc::c_int as usize] as libc::c_int
        + (((*infosec).count[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_int
        + (((*((*infosec).count)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int
            + ((*((*infosec).count)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
            as uint16_t as libc::c_int) << 16 as libc::c_int)) as uint32_t
        != 0xffffffff as libc::c_uint
    {
        printf(
            b"free clusters=%u\n\0" as *const u8 as *const libc::c_char,
            (((*infosec).count[0 as libc::c_int as usize] as libc::c_int
                + (((*infosec).count[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*infosec).count)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*infosec).count)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
        );
    }
    if (((*infosec).pos[0 as libc::c_int as usize] as libc::c_int
        + (((*infosec).pos[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_int
        + (((*((*infosec).pos)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as libc::c_int
            + ((*((*infosec).pos)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
            as uint16_t as libc::c_int) << 16 as libc::c_int)) as uint32_t
        != 0xffffffff as libc::c_uint
    {
        printf(
            b"last allocated cluster=%u\n\0" as *const u8 as *const libc::c_char,
            (((*infosec).pos[0 as libc::c_int as usize] as libc::c_int
                + (((*infosec).pos[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*infosec).pos)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*infosec).pos)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
        );
    }
}
unsafe extern "C" fn getHidden(mut boot: *mut bootsector) -> uint32_t {
    return if ((*boot).boot.psect[0 as libc::c_int as usize] as libc::c_int
        + (((*boot).boot.psect[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as libc::c_int != 0
    {
        ((*boot).boot.nhs[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nhs[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_uint
    } else {
        (((*boot).boot.nhs[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nhs[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int
            + (((*((*boot).boot.nhs)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int
                + ((*((*boot).boot.nhs)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int)
                << 16 as libc::c_int)) as uint32_t
    };
}
unsafe extern "C" fn displayBPB(mut Stream: *mut Stream_t, mut boot: *mut bootsector) {
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    printf(b"bootsector information\n\0" as *const u8 as *const libc::c_char);
    printf(b"======================\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"banner:\"%.8s\"\n\0" as *const u8 as *const libc::c_char,
        ((*boot).boot.banner).as_mut_ptr(),
    );
    printf(
        b"sector size: %d bytes\n\0" as *const u8 as *const libc::c_char,
        ((*boot).boot.secsiz[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int,
    );
    printf(
        b"cluster size: %d sectors\n\0" as *const u8 as *const libc::c_char,
        (*boot).boot.clsiz as libc::c_int,
    );
    printf(
        b"reserved (boot) sectors: %d\n\0" as *const u8 as *const libc::c_char,
        ((*boot).boot.nrsvsect[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nrsvsect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int,
    );
    printf(
        b"fats: %d\n\0" as *const u8 as *const libc::c_char,
        (*boot).boot.nfat as libc::c_int,
    );
    printf(
        b"max available root directory slots: %d\n\0" as *const u8
            as *const libc::c_char,
        ((*boot).boot.dirents[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.dirents[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int,
    );
    printf(
        b"small size: %d sectors\n\0" as *const u8 as *const libc::c_char,
        ((*boot).boot.psect[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.psect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int,
    );
    printf(
        b"media descriptor byte: 0x%x\n\0" as *const u8 as *const libc::c_char,
        (*boot).boot.descr as libc::c_int,
    );
    printf(
        b"sectors per fat: %d\n\0" as *const u8 as *const libc::c_char,
        ((*boot).boot.fatlen[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.fatlen[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int,
    );
    printf(
        b"sectors per track: %d\n\0" as *const u8 as *const libc::c_char,
        ((*boot).boot.nsect[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nsect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int,
    );
    printf(
        b"heads: %d\n\0" as *const u8 as *const libc::c_char,
        ((*boot).boot.nheads[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nheads[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int,
    );
    printf(
        b"hidden sectors: %d\n\0" as *const u8 as *const libc::c_char,
        getHidden(boot),
    );
    if ((*boot).boot.psect[0 as libc::c_int as usize] as libc::c_int
        + (((*boot).boot.psect[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t == 0
    {
        printf(
            b"big size: %u sectors\n\0" as *const u8 as *const libc::c_char,
            (((*boot).boot.bigsect[0 as libc::c_int as usize] as libc::c_int
                + (((*boot).boot.bigsect[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*boot).boot.bigsect)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*boot).boot.bigsect)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
        );
    }
    if ((*boot).boot.fatlen[0 as libc::c_int as usize] as libc::c_int
        + (((*boot).boot.fatlen[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t != 0
    {
        labelBlock = &mut (*boot).boot.ext.old.labelBlock;
    } else {
        labelBlock = &mut (*boot).boot.ext.fat32.labelBlock;
    }
    if (*labelBlock).dos4 as libc::c_int == 0x28 as libc::c_int
        || (*labelBlock).dos4 as libc::c_int == 0x29 as libc::c_int
    {
        printf(
            b"physical drive id: 0x%x\n\0" as *const u8 as *const libc::c_char,
            (*labelBlock).physdrive as libc::c_int,
        );
        printf(
            b"reserved=0x%x\n\0" as *const u8 as *const libc::c_char,
            (*labelBlock).reserved as libc::c_int,
        );
        printf(
            b"dos4=0x%x\n\0" as *const u8 as *const libc::c_char,
            (*labelBlock).dos4 as libc::c_int,
        );
        printf(
            b"serial number: %08X\n\0" as *const u8 as *const libc::c_char,
            (((*labelBlock).serial[0 as libc::c_int as usize] as libc::c_int
                + (((*labelBlock).serial[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*labelBlock).serial)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*labelBlock).serial)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
        );
        printf(
            b"disk label=\"%11.11s\"\n\0" as *const u8 as *const libc::c_char,
            ((*labelBlock).label).as_mut_ptr(),
        );
        printf(
            b"disk type=\"%8.8s\"\n\0" as *const u8 as *const libc::c_char,
            ((*labelBlock).fat_type).as_mut_ptr(),
        );
    }
    if ((*boot).boot.fatlen[0 as libc::c_int as usize] as libc::c_int
        + (((*boot).boot.fatlen[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t == 0
    {
        printf(
            b"Big fatlen=%u\n\0" as *const u8 as *const libc::c_char,
            (((*boot).boot.ext.fat32.bigFat[0 as libc::c_int as usize] as libc::c_int
                + (((*boot).boot.ext.fat32.bigFat[1 as libc::c_int as usize]
                    as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*boot).boot.ext.fat32.bigFat)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*boot).boot.ext.fat32.bigFat)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
        );
        printf(
            b"Extended flags=0x%04x\n\0" as *const u8 as *const libc::c_char,
            ((*boot).boot.ext.fat32.extFlags[0 as libc::c_int as usize] as libc::c_int
                + (((*boot).boot.ext.fat32.extFlags[1 as libc::c_int as usize]
                    as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int,
        );
        printf(
            b"FS version=0x%04x\n\0" as *const u8 as *const libc::c_char,
            ((*boot).boot.ext.fat32.fsVersion[0 as libc::c_int as usize] as libc::c_int
                + (((*boot).boot.ext.fat32.fsVersion[1 as libc::c_int as usize]
                    as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int,
        );
        printf(
            b"rootCluster=%u\n\0" as *const u8 as *const libc::c_char,
            (((*boot).boot.ext.fat32.rootCluster[0 as libc::c_int as usize]
                as libc::c_int
                + (((*boot).boot.ext.fat32.rootCluster[1 as libc::c_int as usize]
                    as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
                + (((*((*boot).boot.ext.fat32.rootCluster)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    + ((*((*boot).boot.ext.fat32.rootCluster)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t as libc::c_int)
                    << 16 as libc::c_int)) as uint32_t,
        );
        if ((*boot).boot.ext.fat32.infoSector[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.ext.fat32.infoSector[1 as libc::c_int as usize]
                as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
            != 0xffff as libc::c_int
        {
            printf(
                b"infoSector location=%d\n\0" as *const u8 as *const libc::c_char,
                ((*boot).boot.ext.fat32.infoSector[0 as libc::c_int as usize]
                    as libc::c_int
                    + (((*boot).boot.ext.fat32.infoSector[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int,
            );
        }
        if ((*boot).boot.ext.fat32.backupBoot[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.ext.fat32.backupBoot[1 as libc::c_int as usize]
                as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int
            != 0xffff as libc::c_int
        {
            printf(
                b"backup boot sector=%d\n\0" as *const u8 as *const libc::c_char,
                ((*boot).boot.ext.fat32.backupBoot[0 as libc::c_int as usize]
                    as libc::c_int
                    + (((*boot).boot.ext.fat32.backupBoot[1 as libc::c_int as usize]
                        as libc::c_int) << 8 as libc::c_int)) as uint16_t as libc::c_int,
            );
        }
        displayInfosector(Stream, boot);
    }
}
unsafe extern "C" fn try_0(
    mut tot_sectors: uint32_t,
    mut masterFs: *mut Fs_t,
    mut tryFs: *mut Fs_t,
    mut master_dev: *mut device,
    mut try_dev: *mut device,
    mut bootDescr: *mut uint8_t,
) -> libc::c_int {
    *tryFs = *masterFs;
    *try_dev = *master_dev;
    return calc_fs_parameters(
        try_dev,
        0 as libc::c_int != 0,
        tot_sectors,
        tryFs,
        bootDescr,
    );
}
unsafe extern "C" fn print_mformat_commandline(
    mut imgFile: *const libc::c_char,
    mut drive: libc::c_char,
    mut dev: *mut device,
    mut boot: *mut bootsector,
    mut media: libc::c_int,
    mut haveBPB: libc::c_int,
) {
    let mut size_code: uint8_t = 0;
    let mut sect_per_track: uint32_t = 0;
    let mut hidden: uint32_t = 0;
    let mut tot_sectors: uint32_t = 0;
    let mut tracks_match: libc::c_int = 0 as libc::c_int;
    let mut masterFs: Fs_t = Fs_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        serialized: 0,
        serial_number: 0,
        cluster_size: 0,
        sector_size: 0,
        fat_error: 0,
        fat_decode: None,
        fat_encode: None,
        fat_dirty: 0,
        fat_start: 0,
        fat_len: 0,
        num_fat: 0,
        end_fat: 0,
        last_fat: 0,
        fat_bits: 0,
        FatMap: 0 as *mut FatMap_t,
        dir_start: 0,
        dir_len: 0,
        clus_start: 0,
        num_clus: 0,
        drive: 0,
        primaryFat: 0,
        writeAllFats: 0,
        rootCluster: 0,
        infoSectorLoc: 0,
        backupBoot: 0,
        last: 0,
        freeSpace: 0,
        preallocatedClusters: 0,
        lastFatSectorNr: 0,
        lastFatSectorData: 0 as *mut libc::c_uchar,
        lastFatAccessMode: FAT_ACCESS_READ,
        sectorMask: 0,
        sectorShift: 0,
        cp: 0 as *mut doscp_t,
    };
    let mut tryFs: Fs_t = Fs_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        serialized: 0,
        serial_number: 0,
        cluster_size: 0,
        sector_size: 0,
        fat_error: 0,
        fat_decode: None,
        fat_encode: None,
        fat_dirty: 0,
        fat_start: 0,
        fat_len: 0,
        num_fat: 0,
        end_fat: 0,
        last_fat: 0,
        fat_bits: 0,
        FatMap: 0 as *mut FatMap_t,
        dir_start: 0,
        dir_len: 0,
        clus_start: 0,
        num_clus: 0,
        drive: 0,
        primaryFat: 0,
        writeAllFats: 0,
        rootCluster: 0,
        infoSectorLoc: 0,
        backupBoot: 0,
        last: 0,
        freeSpace: 0,
        preallocatedClusters: 0,
        lastFatSectorNr: 0,
        lastFatSectorData: 0 as *mut libc::c_uchar,
        lastFatAccessMode: FAT_ACCESS_READ,
        sectorMask: 0,
        sectorShift: 0,
        cp: 0 as *mut doscp_t,
    };
    let mut actual: Fs_t = Fs_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        serialized: 0,
        serial_number: 0,
        cluster_size: 0,
        sector_size: 0,
        fat_error: 0,
        fat_decode: None,
        fat_encode: None,
        fat_dirty: 0,
        fat_start: 0,
        fat_len: 0,
        num_fat: 0,
        end_fat: 0,
        last_fat: 0,
        fat_bits: 0,
        FatMap: 0 as *mut FatMap_t,
        dir_start: 0,
        dir_len: 0,
        clus_start: 0,
        num_clus: 0,
        drive: 0,
        primaryFat: 0,
        writeAllFats: 0,
        rootCluster: 0,
        infoSectorLoc: 0,
        backupBoot: 0,
        last: 0,
        freeSpace: 0,
        preallocatedClusters: 0,
        lastFatSectorNr: 0,
        lastFatSectorData: 0 as *mut libc::c_uchar,
        lastFatAccessMode: FAT_ACCESS_READ,
        sectorMask: 0,
        sectorShift: 0,
        cp: 0 as *mut doscp_t,
    };
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
    let mut tryMedia: uint8_t = 0;
    let mut bad: libc::c_int = 0;
    sect_per_track = ((*dev).sectors as libc::c_int * (*dev).heads as libc::c_int)
        as uint32_t;
    if sect_per_track == 0 as libc::c_int as libc::c_uint {
        return;
    }
    tot_sectors = parseFsParams(
        &mut actual,
        boot,
        media | (if haveBPB != 0 { 0x100 as libc::c_int } else { 0 as libc::c_int }),
        sect_per_track,
    );
    if tot_sectors == 0 as libc::c_int as libc::c_uint {
        return;
    }
    printf(b"mformat command line:\n  mformat \0" as *const u8 as *const libc::c_char);
    if haveBPB != 0 {
        if media == 0xf0 as libc::c_int {
            hidden = getHidden(boot);
        } else {
            hidden = 0 as libc::c_int as uint32_t;
        }
        size_code = (actual.sectorShift as uint8_t as libc::c_int - 7 as libc::c_int)
            as uint8_t;
    } else {
        size_code = 2 as libc::c_int as uint8_t;
        hidden = 0 as libc::c_int as uint32_t;
    }
    if tot_sectors
        == ((*dev).tracks)
            .wrapping_mul(sect_per_track)
            .wrapping_sub(hidden.wrapping_rem(sect_per_track))
    {
        tracks_match = 1 as libc::c_int;
        printf(b"-t %d \0" as *const u8 as *const libc::c_char, (*dev).tracks);
    } else {
        printf(b"-T %d \0" as *const u8 as *const libc::c_char, tot_sectors);
    }
    printf(
        b"-h %d -s %d \0" as *const u8 as *const libc::c_char,
        (*dev).heads as libc::c_int,
        (*dev).sectors as libc::c_int,
    );
    if haveBPB != 0 && (hidden != 0 || tracks_match == 0) {
        printf(b"-H %d \0" as *const u8 as *const libc::c_char, hidden);
    }
    used_dev = *dev;
    if size_code as libc::c_int != 2 as libc::c_int {
        printf(
            b"-S %d \0" as *const u8 as *const libc::c_char,
            size_code as libc::c_int,
        );
        used_dev.ssize = size_code;
    }
    initFsForFormat(&mut masterFs);
    setFsSectorSize(&mut masterFs, &mut used_dev, 0 as libc::c_int as uint16_t);
    if actual.num_fat as libc::c_int != 2 as libc::c_int {
        masterFs.num_fat = actual.num_fat;
        printf(
            b"-d %d \0" as *const u8 as *const libc::c_char,
            actual.num_fat as libc::c_int,
        );
    }
    bad = try_0(
        tot_sectors,
        &mut masterFs,
        &mut tryFs,
        dev,
        &mut used_dev,
        &mut tryMedia,
    );
    if bad != 0 || actual.dir_len as libc::c_int != tryFs.dir_len as libc::c_int {
        masterFs.dir_len = actual.dir_len;
        printf(
            b"-r %d \0" as *const u8 as *const libc::c_char,
            actual.dir_len as libc::c_int,
        );
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad != 0
        || actual.cluster_size as libc::c_int != tryFs.cluster_size as libc::c_int
    {
        masterFs.cluster_size = actual.cluster_size;
        printf(
            b"-c %d \0" as *const u8 as *const libc::c_char,
            actual.cluster_size as libc::c_int,
        );
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad != 0 || actual.fat_start as libc::c_int != tryFs.fat_start as libc::c_int {
        masterFs.fat_start = actual.fat_start;
        printf(
            b"-R %d \0" as *const u8 as *const libc::c_char,
            actual.fat_start as libc::c_int,
        );
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad != 0 || actual.fat_len != tryFs.fat_len {
        masterFs.fat_len = actual.fat_len;
        printf(b"-L %d \0" as *const u8 as *const libc::c_char, actual.fat_len);
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad == 0 {} else {
        __assert_fail(
            b"!bad\0" as *const u8 as *const libc::c_char,
            b"minfo.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void print_mformat_commandline(const char *, char, struct device *, union bootsector *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9375: {
        if bad == 0 {} else {
            __assert_fail(
                b"!bad\0" as *const u8 as *const libc::c_char,
                b"minfo.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void print_mformat_commandline(const char *, char, struct device *, union bootsector *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if media & 0xff as libc::c_int != tryMedia as libc::c_int & 0xff as libc::c_int {
        printf(
            b"-m %d \0" as *const u8 as *const libc::c_char,
            media & 0xff as libc::c_int,
        );
    }
    if actual.fat_bits == 32 as libc::c_int as libc::c_uint {
        if actual.backupBoot as libc::c_int != tryFs.backupBoot as libc::c_int {
            printf(
                b"-K %d \0" as *const u8 as *const libc::c_char,
                actual.backupBoot as libc::c_int,
            );
        }
    }
    if !imgFile.is_null() {
        printf(b"-i \"%s\" \0" as *const u8 as *const libc::c_char, imgFile);
    }
    printf(
        b"%c:\n\0" as *const u8 as *const libc::c_char,
        ch_toupper(drive) as libc::c_int,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn minfo(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut name: [libc::c_char; 2048] = [0; 2048];
    let mut dev: device = device {
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
    let mut drive: libc::c_char = 0;
    let mut verbose: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut have_drive: libc::c_int = 0 as libc::c_int;
    let mut ex: libc::c_int = 0 as libc::c_int;
    let mut imgFile: *mut libc::c_char = 0 as *mut libc::c_char;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:vh\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
                imgFile = optarg;
            }
            118 => {
                verbose = 1 as libc::c_int;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    while optind <= argc {
        let mut media: libc::c_int = 0;
        let mut haveBPB: libc::c_int = 0;
        if optind == argc {
            if have_drive != 0 {
                break;
            }
            drive = get_default_drive();
        } else {
            if *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize) == 0
                || *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int != ':' as i32
            {
                usage(1 as libc::c_int);
            }
            drive = ch_toupper(
                *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize),
            );
        }
        have_drive = 1 as libc::c_int;
        Stream = find_device(
            drive,
            0 as libc::c_int,
            &mut dev,
            &mut boot,
            name.as_mut_ptr(),
            &mut media,
            0 as *mut mt_off_t,
            0 as *mut libc::c_int,
        );
        if Stream.is_null() {
            fprintf(
                stderr,
                b"Could not open drive %c:\n\0" as *const u8 as *const libc::c_char,
                drive as libc::c_int,
            );
            ex = 1 as libc::c_int;
        } else {
            haveBPB = (media >= 0x100 as libc::c_int) as libc::c_int;
            media = media & 0xff as libc::c_int;
            printf(b"device information:\n\0" as *const u8 as *const libc::c_char);
            printf(b"===================\n\0" as *const u8 as *const libc::c_char);
            printf(
                b"filename=\"%s\"\n\0" as *const u8 as *const libc::c_char,
                name.as_mut_ptr(),
            );
            printf(
                b"sectors per track: %d\n\0" as *const u8 as *const libc::c_char,
                dev.sectors as libc::c_int,
            );
            printf(
                b"heads: %d\n\0" as *const u8 as *const libc::c_char,
                dev.heads as libc::c_int,
            );
            printf(
                b"cylinders: %d\n\n\0" as *const u8 as *const libc::c_char,
                dev.tracks,
            );
            printf(
                b"media byte: %02x\n\n\0" as *const u8 as *const libc::c_char,
                media & 0xff as libc::c_int,
            );
            print_mformat_commandline(
                imgFile,
                drive,
                &mut dev,
                &mut boot,
                media,
                haveBPB,
            );
            if haveBPB != 0 || verbose != 0 {
                displayBPB(Stream, &mut boot);
            }
            if verbose != 0 {
                let mut size: uint16_t = 0;
                let mut ssize: ssize_t = 0;
                let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                size = (boot.boot.secsiz[0 as libc::c_int as usize] as libc::c_int
                    + ((boot.boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                        << 8 as libc::c_int)) as uint16_t;
                buf = malloc(size as libc::c_ulong) as *mut libc::c_uchar;
                if buf.is_null() {
                    fprintf(
                        stderr,
                        b"Out of memory error\n\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                ssize = ((*(*Stream).Class).pread)
                    .unwrap()(
                    Stream,
                    buf as *mut libc::c_char,
                    0 as libc::c_int as mt_off_t,
                    size as size_t,
                );
                if ssize < 0 as libc::c_int as libc::c_long {
                    perror(b"read boot sector\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
                print_sector(
                    b"Boot sector hexdump\0" as *const u8 as *const libc::c_char,
                    buf,
                    ssize as uint16_t as libc::c_int,
                );
            }
        }
        optind += 1;
        optind;
    }
    free_stream(&mut Stream);
    exit(ex);
}
