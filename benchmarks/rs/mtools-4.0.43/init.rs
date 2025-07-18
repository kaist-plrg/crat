use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    pub type FatMap_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
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
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn getfreeMinClusters(Stream: *mut Stream_t, ref_0: uint32_t) -> libc::c_int;
    static mut mtools_skip_check: libc::c_uint;
    fn log_2(_: libc::c_uint) -> libc::c_uint;
    fn expand(_: *const libc::c_char, _: *mut libc::c_char) -> *const libc::c_char;
    static mut devices: *mut device;
    fn check_if_sectors_fit(
        tot_sectors: uint32_t,
        maxBytes: mt_off_t,
        sectorSize: uint32_t,
        errmsg: *mut libc::c_char,
    ) -> libc::c_int;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn get_data_pass_through(
        Stream: *mut Stream_t,
        date: *mut time_t,
        size: *mut mt_off_t,
        type_0: *mut libc::c_int,
        address: *mut uint32_t,
    ) -> libc::c_int;
    fn pread_pass_through(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn pwrite_pass_through(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn getOldDosByMedia(media: libc::c_int) -> *mut OldDos_t;
    fn setDeviceFromOldDos(media: libc::c_int, dev: *mut device) -> libc::c_int;
    fn fat_write(This: *mut Fs_t);
    fn fs_free(Stream: *mut Stream_t) -> libc::c_int;
    fn fat_read(
        This: *mut Fs_t,
        boot: *mut bootsector,
        nodups: libc::c_int,
    ) -> libc::c_int;
    fn set_fat(This: *mut Fs_t, haveBigFatLen: bool);
    fn buf_init(
        Next: *mut Stream_t,
        size: size_t,
        cylinderSize: size_t,
        sectorSize: size_t,
    ) -> *mut Stream_t;
    fn cp_open(codepage: libc::c_uint) -> *mut doscp_t;
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
pub struct OldDos_t {
    pub tracks: libc::c_uint,
    pub sectors: uint16_t,
    pub heads: uint16_t,
    pub dir_len: uint16_t,
    pub cluster_size: uint8_t,
    pub fat_len: uint32_t,
    pub media: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: libc::c_uint,
    pub RootDirSize: uint16_t,
    pub BadSectors: libc::c_uint,
}
pub unsafe extern "C" fn sectorsToBytes(
    mut This: *mut Fs_t,
    mut off: uint32_t,
) -> mt_off_t {
    return (off as mt_off_t) << (*This).sectorShift;
}
unsafe extern "C" fn read_boot(
    mut Stream: *mut Stream_t,
    mut boot: *mut bootsector,
    mut size: size_t,
) -> libc::c_int {
    let mut boot_sector_size: size_t = 0;
    if size == 0 {
        size = 256 as libc::c_int as size_t;
    }
    if size > 4096 as libc::c_int as libc::c_ulong {
        size = 4096 as libc::c_int as size_t;
    }
    if force_pread(
        Stream,
        ((*boot).characters).as_mut_ptr(),
        0 as libc::c_int as mt_off_t,
        size,
    ) != size as ssize_t
    {
        return -(1 as libc::c_int);
    }
    boot_sector_size = ((*boot).boot.secsiz[0 as libc::c_int as usize] as libc::c_int
        + (((*boot).boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
            << 8 as libc::c_int)) as uint16_t as size_t;
    if boot_sector_size < ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong
    {
        memset(
            ((*boot).bytes).as_mut_ptr().offset(boot_sector_size as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong)
                .wrapping_sub(boot_sector_size),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fs_flush(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    fat_write(This);
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_dosConvert(mut Stream: *mut Stream_t) -> *mut doscp_t {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    return (*This).cp;
}
pub static mut FsClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                pread_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                pwrite_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(fs_flush as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int),
            freeFunc: Some(
                fs_free as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
            set_geom: None,
            get_data: Some(
                get_data_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut libc::c_int,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            pre_allocate: None,
            get_dosConvert: Some(
                get_dosConvert as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
unsafe extern "C" fn get_media_type(
    mut St: *mut Stream_t,
    mut boot: *mut bootsector,
) -> libc::c_int {
    let mut media: libc::c_int = 0;
    media = (*boot).boot.descr as libc::c_int;
    if media < 0xf0 as libc::c_int {
        let mut temp: [libc::c_char; 512] = [0; 512];
        if force_pread(
            St,
            temp.as_mut_ptr(),
            512 as libc::c_int as mt_off_t,
            512 as libc::c_int as size_t,
        ) == 512 as libc::c_int as libc::c_long
        {
            media = temp[0 as libc::c_int as usize] as libc::c_uchar as libc::c_int;
        } else {
            media = 0 as libc::c_int;
        }
    } else {
        media += 0x100 as libc::c_int;
    }
    return media;
}
pub unsafe extern "C" fn GetFs(mut Fs: *mut Stream_t) -> *mut Stream_t {
    while !Fs.is_null() && (*Fs).Class != &mut FsClass as *mut Class_t {
        Fs = (*Fs).Next;
    }
    return Fs;
}
unsafe extern "C" fn boot_to_geom(
    mut dev: *mut device,
    mut media: libc::c_int,
    mut boot: *mut bootsector,
) {
    let mut tot_sectors: uint32_t = 0;
    let mut BootP: libc::c_int = 0;
    let mut Infp0: libc::c_int = 0;
    let mut InfpX: libc::c_int = 0;
    let mut InfTm: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_uchar = 0;
    let mut sect_per_track: uint16_t = 0;
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    (*dev).ssize = 2 as libc::c_int as uint8_t;
    (*dev).use_2m = 0x80 as libc::c_int as libc::c_uint;
    if media == 0xf0 as libc::c_int || media >= 0x100 as libc::c_int {
        (*dev)
            .heads = ((*boot).boot.nheads[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nheads[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t;
        (*dev)
            .sectors = ((*boot).boot.nsect[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nsect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t;
        tot_sectors = (((*boot).boot.bigsect[0 as libc::c_int as usize] as libc::c_int
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
                << 16 as libc::c_int)) as uint32_t;
        if ((*boot).boot.psect[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.psect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t != 0
        {
            tot_sectors = ((*boot).boot.psect[0 as libc::c_int as usize] as libc::c_int
                + (((*boot).boot.psect[1 as libc::c_int as usize] as libc::c_int)
                    << 8 as libc::c_int)) as uint16_t as uint32_t;
        }
        sect_per_track = ((*dev).heads as libc::c_int * (*dev).sectors as libc::c_int)
            as uint16_t;
        if sect_per_track as libc::c_int == 0 as libc::c_int {
            if mtools_skip_check != 0 {
                (*dev).heads = 1 as libc::c_int as uint16_t;
                (*dev).sectors = 1 as libc::c_int as uint16_t;
                sect_per_track = 1 as libc::c_int as uint16_t;
            } else {
                fprintf(
                    stderr,
                    b"The devil is in the details: zero number of heads or sectors\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        (*dev).tracks = tot_sectors.wrapping_div(sect_per_track as libc::c_uint);
        if tot_sectors.wrapping_rem(sect_per_track as libc::c_uint) != 0 {
            (*dev).tracks = ((*dev).tracks).wrapping_add(1);
            (*dev).tracks;
        }
        BootP = ((*boot).boot.ext.old.BootP[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.ext.old.BootP[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int;
        Infp0 = ((*boot).boot.ext.old.Infp0[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.ext.old.Infp0[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int;
        InfpX = ((*boot).boot.ext.old.InfpX[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.ext.old.InfpX[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int;
        InfTm = ((*boot).boot.ext.old.InfTm[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.ext.old.InfTm[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int;
        if ((*boot).boot.fatlen[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.fatlen[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t != 0
        {
            labelBlock = &mut (*boot).boot.ext.old.labelBlock;
        } else {
            labelBlock = &mut (*boot).boot.ext.fat32.labelBlock;
        }
        if (*boot).boot.descr as libc::c_int >= 0xf0 as libc::c_int
            && ((*labelBlock).dos4 as libc::c_int == 0x28 as libc::c_int
                || (*labelBlock).dos4 as libc::c_int == 0x29 as libc::c_int)
            && strncmp(
                ((*boot).boot.banner).as_mut_ptr(),
                b"2M\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int && BootP < 512 as libc::c_int
            && Infp0 < 512 as libc::c_int && InfpX < 512 as libc::c_int
            && InfTm < 512 as libc::c_int && BootP >= InfTm + 2 as libc::c_int
            && InfTm >= InfpX && InfpX >= Infp0 && Infp0 >= 76 as libc::c_int
        {
            sum = 0 as libc::c_int as libc::c_uchar;
            j = 63 as libc::c_int;
            while j < BootP {
                sum = (sum as libc::c_int + (*boot).bytes[j as usize] as libc::c_int)
                    as libc::c_uchar;
                j += 1;
                j;
            }
            (*dev).ssize = (*boot).bytes[InfTm as usize];
            if sum == 0 && (*dev).ssize as libc::c_int <= 7 as libc::c_int {
                (*dev).use_2m = 0xff as libc::c_int as libc::c_uint;
                (*dev)
                    .ssize = ((*dev).ssize as libc::c_int | 0x80 as libc::c_int)
                    as uint8_t;
            }
        }
        (*dev)
            .sector_size = ((*boot).boot.secsiz[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t;
    } else if setDeviceFromOldDos(media, dev) < 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn try_device(
    mut dev: *mut device,
    mut mode: libc::c_int,
    mut out_dev: *mut device,
    mut boot: *mut bootsector,
    mut name: *mut libc::c_char,
    mut media: *mut libc::c_int,
    mut maxSize: *mut mt_off_t,
    mut isRop: *mut libc::c_int,
    mut try_writable: libc::c_int,
    mut errmsg: *mut libc::c_char,
) -> *mut Stream_t {
    let mut retry_write: libc::c_int = 0;
    let mut have_read_bootsector: libc::c_int = 0 as libc::c_int;
    let mut modeFlags: libc::c_int = mode & !(0o3 as libc::c_int);
    let mut openMode: libc::c_int = 0;
    let mut lockMode: libc::c_int = 0;
    *out_dev = *dev;
    expand((*dev).name, name);
    if try_writable != 0 {
        openMode = 0o2 as libc::c_int | modeFlags;
    } else {
        openMode = mode;
    }
    lockMode = openMode;
    retry_write = 0 as libc::c_int;
    while retry_write < 2 as libc::c_int {
        let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
        let mut r: libc::c_int = 0;
        let mut geomFailure: libc::c_int = 0 as libc::c_int;
        if retry_write != 0 {
            mode |= 0o2 as libc::c_int;
        }
        Stream = OpenImage(
            out_dev,
            dev,
            name,
            openMode,
            errmsg,
            0 as libc::c_int,
            lockMode,
            maxSize,
            &mut geomFailure,
            0 as *mut xdf_info,
        );
        if Stream.is_null() {
            if geomFailure != 0 && mode & 0o3 as libc::c_int == 0 as libc::c_int {
                openMode = modeFlags | 0o2 as libc::c_int;
            } else if try_writable != 0
                && (*__errno_location() == 1 as libc::c_int
                    || *__errno_location() == 13 as libc::c_int
                    || *__errno_location() == 30 as libc::c_int)
            {
                openMode = modeFlags | 0 as libc::c_int;
                lockMode = openMode;
            } else {
                return 0 as *mut Stream_t
            }
        } else {
            if have_read_bootsector == 0 {
                r = read_boot(Stream, boot, (*out_dev).blocksize as size_t);
                if r < 0 as libc::c_int {
                    sprintf(
                        errmsg,
                        b"init %c: could not read boot sector\0" as *const u8
                            as *const libc::c_char,
                        (*dev).drive as libc::c_int,
                    );
                    free_stream(&mut Stream);
                    return 0 as *mut Stream_t;
                }
                *media = get_media_type(Stream, boot);
                if *media <= 0xf0 as libc::c_int {
                    if (*boot).boot.jump[2 as libc::c_int as usize] as libc::c_int
                        == 'L' as i32
                    {
                        sprintf(
                            errmsg,
                            b"diskette %c: is Linux LILO, not DOS\0" as *const u8
                                as *const libc::c_char,
                            (*dev).drive as libc::c_int,
                        );
                    } else {
                        sprintf(
                            errmsg,
                            b"init %c: non DOS media\0" as *const u8
                                as *const libc::c_char,
                            (*dev).drive as libc::c_int,
                        );
                    }
                    free_stream(&mut Stream);
                    return 0 as *mut Stream_t;
                }
                have_read_bootsector = 1 as libc::c_int;
            }
            *__errno_location() = 0 as libc::c_int;
            boot_to_geom(out_dev, *media, boot);
            if ((*(*Stream).Class).set_geom).unwrap()(Stream, out_dev, dev) != 0 {
                if *__errno_location() == 9 as libc::c_int
                    || *__errno_location() == 1 as libc::c_int
                {
                    free_stream(&mut Stream);
                    openMode = modeFlags | 0o2 as libc::c_int;
                } else {
                    if *__errno_location() != 0 {
                        snprintf(
                            errmsg,
                            199 as libc::c_int as libc::c_ulong,
                            b"Can't set disk parameters for %c: %s\0" as *const u8
                                as *const libc::c_char,
                            (*dev).drive as libc::c_int,
                            strerror(*__errno_location()),
                        );
                    } else {
                        sprintf(
                            errmsg,
                            b"Can't set disk parameters for %c\0" as *const u8
                                as *const libc::c_char,
                            (*dev).drive as libc::c_int,
                        );
                    }
                    free_stream(&mut Stream);
                    return 0 as *mut Stream_t;
                }
            } else {
                if !isRop.is_null() {
                    *isRop = (openMode & 0o3 as libc::c_int == 0 as libc::c_int)
                        as libc::c_int;
                }
                return Stream;
            }
        }
        retry_write += 1;
        retry_write;
    }
    return 0 as *mut Stream_t;
}
pub unsafe extern "C" fn calc_clus_start(mut Fs: *mut Fs_t) -> uint32_t {
    return ((*Fs).fat_start as libc::c_uint)
        .wrapping_add(((*Fs).fat_len).wrapping_mul((*Fs).num_fat as libc::c_uint))
        .wrapping_add((*Fs).dir_len as libc::c_uint);
}
pub unsafe extern "C" fn calc_num_clus(
    mut Fs: *mut Fs_t,
    mut tot_sectors: uint32_t,
) -> libc::c_int {
    (*Fs).clus_start = calc_clus_start(Fs);
    if tot_sectors <= (*Fs).clus_start {
        return -(1 as libc::c_int);
    }
    (*Fs)
        .num_clus = tot_sectors
        .wrapping_sub((*Fs).clus_start)
        .wrapping_div((*Fs).cluster_size as libc::c_uint);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn find_device(
    mut drive: libc::c_char,
    mut mode: libc::c_int,
    mut out_dev: *mut device,
    mut boot: *mut bootsector,
    mut name: *mut libc::c_char,
    mut media: *mut libc::c_int,
    mut maxSize: *mut mt_off_t,
    mut isRop: *mut libc::c_int,
) -> *mut Stream_t {
    let mut errmsg: [libc::c_char; 200] = [0; 200];
    let mut dev: *mut device = 0 as *mut device;
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const libc::c_char,
        drive as libc::c_int,
    );
    dev = devices;
    while !((*dev).name).is_null() {
        let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
        let mut isRo: libc::c_int = 0;
        isRo = 0 as libc::c_int;
        if !((*dev).drive as libc::c_int != drive as libc::c_int) {
            Stream = try_device(
                dev,
                mode,
                out_dev,
                boot,
                name,
                media,
                maxSize,
                &mut isRo,
                (isRop != 0 as *mut libc::c_void as *mut libc::c_int) as libc::c_int,
                errmsg.as_mut_ptr(),
            );
            if !Stream.is_null() {
                if !isRop.is_null() {
                    *isRop = isRo;
                }
                return Stream;
            }
        }
        dev = dev.offset(1);
        dev;
    }
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, errmsg.as_mut_ptr());
    return 0 as *mut Stream_t;
}
pub unsafe extern "C" fn parseFsParams(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
    mut media: libc::c_int,
    mut cylinder_size: libc::c_uint,
) -> uint32_t {
    let mut tot_sectors: uint32_t = 0;
    let mut haveBigFatLen: libc::c_int = 0 as libc::c_int;
    if media & !(7 as libc::c_int) == 0xf8 as libc::c_int {
        let mut params: *mut OldDos_t = getOldDosByMedia(media);
        if params.is_null() {
            fprintf(
                stderr,
                b"Unknown media byte %02x\n\0" as *const u8 as *const libc::c_char,
                media,
            );
            return 0 as libc::c_int as uint32_t;
        }
        (*This).cluster_size = (*params).cluster_size;
        tot_sectors = cylinder_size.wrapping_mul((*params).tracks);
        (*This).fat_start = 1 as libc::c_int as uint16_t;
        (*This).fat_len = (*params).fat_len;
        (*This).dir_len = (*params).dir_len;
        (*This).num_fat = 2 as libc::c_int as uint8_t;
        (*This).sector_size = 512 as libc::c_int as uint16_t;
        (*This).sectorShift = 9 as libc::c_int as libc::c_uint;
        (*This).sectorMask = 511 as libc::c_int as libc::c_uint;
    } else {
        let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
        let mut i: libc::c_uint = 0;
        (*This)
            .sector_size = ((*boot).boot.secsiz[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.secsiz[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t;
        if (*This).sector_size as libc::c_int > 8192 as libc::c_int {
            fprintf(
                stderr,
                b"init: sector size too big\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int as uint32_t;
        }
        i = log_2((*This).sector_size as libc::c_uint);
        if i == 24 as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                b"init: sector size (%d) not a small power of two\n\0" as *const u8
                    as *const libc::c_char,
                (*This).sector_size as libc::c_int,
            );
            return 0 as libc::c_int as uint32_t;
        }
        (*This).sectorShift = i;
        (*This)
            .sectorMask = ((*This).sector_size as libc::c_int - 1 as libc::c_int)
            as libc::c_uint;
        tot_sectors = ((*boot).boot.psect[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.psect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as uint32_t;
        if tot_sectors == 0 {
            tot_sectors = (((*boot).boot.bigsect[0 as libc::c_int as usize]
                as libc::c_int
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
                    << 16 as libc::c_int)) as uint32_t;
        }
        (*This).cluster_size = (*boot).boot.clsiz;
        (*This)
            .fat_start = ((*boot).boot.nrsvsect[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.nrsvsect[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t;
        (*This)
            .fat_len = ((*boot).boot.fatlen[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.fatlen[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as uint32_t;
        (*This)
            .dir_len = (((*boot).boot.dirents[0 as libc::c_int as usize] as libc::c_int
            + (((*boot).boot.dirents[1 as libc::c_int as usize] as libc::c_int)
                << 8 as libc::c_int)) as uint16_t as libc::c_int * 32 as libc::c_int
            / (*This).sector_size as libc::c_int) as uint16_t;
        (*This).num_fat = (*boot).boot.nfat;
        if (*This).fat_len != 0 {
            labelBlock = &mut (*boot).boot.ext.old.labelBlock;
        } else {
            labelBlock = &mut (*boot).boot.ext.fat32.labelBlock;
            (*This)
                .fat_len = (((*boot).boot.ext.fat32.bigFat[0 as libc::c_int as usize]
                as libc::c_int
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
                    << 16 as libc::c_int)) as uint32_t;
            haveBigFatLen = 1 as libc::c_int;
            (*This)
                .backupBoot = ((*boot)
                .boot
                .ext
                .fat32
                .backupBoot[0 as libc::c_int as usize] as libc::c_int
                + (((*boot).boot.ext.fat32.backupBoot[1 as libc::c_int as usize]
                    as libc::c_int) << 8 as libc::c_int)) as uint16_t;
        }
        if (*labelBlock).dos4 as libc::c_int == 0x28 as libc::c_int
            || (*labelBlock).dos4 as libc::c_int == 0x29 as libc::c_int
        {
            (*This).serialized = 1 as libc::c_int;
            (*This)
                .serial_number = (((*labelBlock).serial[0 as libc::c_int as usize]
                as libc::c_int
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
                    << 16 as libc::c_int)) as uint32_t as libc::c_ulong;
        }
    }
    if calc_num_clus(This, tot_sectors) < 0 as libc::c_int {
        return 0 as libc::c_int as uint32_t;
    }
    set_fat(This, haveBigFatLen != 0);
    return tot_sectors;
}
pub unsafe extern "C" fn fs_init(
    mut drive: libc::c_char,
    mut mode: libc::c_int,
    mut isRop: *mut libc::c_int,
) -> *mut Stream_t {
    let mut blocksize: uint32_t = 0;
    let mut media: libc::c_int = 0;
    let mut disk_size: size_t = 0 as libc::c_int as size_t;
    let mut tot_sectors: uint32_t = 0;
    let mut name: [libc::c_char; 2048] = [0; 2048];
    let mut cylinder_size: libc::c_uint = 0;
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
    let mut maxSize: mt_off_t = 0;
    let mut errmsg: [libc::c_char; 81] = [0; 81];
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut This: *mut Fs_t = 0 as *mut Fs_t;
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Fs_t>() as libc::c_ulong,
    ) as *mut Fs_t;
    if This.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut FsClass, 0 as *mut Stream_t);
    (*This).preallocatedClusters = 0 as libc::c_int as libc::c_uint;
    (*This).lastFatSectorNr = 0 as libc::c_int as uint32_t;
    (*This).lastFatAccessMode = FAT_ACCESS_READ;
    (*This).lastFatSectorData = 0 as *mut libc::c_uchar;
    (*This).drive = drive;
    (*This).last = 0 as libc::c_int as uint32_t;
    (*This)
        .head
        .Next = find_device(
        drive,
        mode,
        &mut dev,
        &mut boot,
        name.as_mut_ptr(),
        &mut media,
        &mut maxSize,
        isRop,
    );
    if ((*This).head.Next).is_null() {
        return 0 as *mut Stream_t;
    }
    cylinder_size = (dev.heads as libc::c_int * dev.sectors as libc::c_int)
        as libc::c_uint;
    (*This).serialized = 0 as libc::c_int;
    tot_sectors = parseFsParams(This, &mut boot, media, cylinder_size);
    if tot_sectors == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut Stream_t;
    }
    if check_if_sectors_fit(
        tot_sectors,
        maxSize,
        (*This).sector_size as uint32_t,
        errmsg.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"%s\0" as *const u8 as *const libc::c_char,
            errmsg.as_mut_ptr(),
        );
        return 0 as *mut Stream_t;
    }
    disk_size = (if dev.tracks != 0 {
        cylinder_size
    } else {
        512 as libc::c_int as libc::c_uint
    }) as size_t;
    disk_size = cylinder_size as size_t;
    if disk_size > 256 as libc::c_int as libc::c_ulong {
        disk_size = dev.sectors as size_t;
        if dev.sectors as libc::c_int % 2 as libc::c_int != 0 {
            disk_size <<= 1 as libc::c_int;
        }
    }
    if disk_size.wrapping_rem(2 as libc::c_int as libc::c_ulong) != 0 {
        disk_size = (disk_size as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    if dev.blocksize == 0 || dev.blocksize < (*This).sector_size as libc::c_uint {
        blocksize = (*This).sector_size as uint32_t;
    } else {
        blocksize = dev.blocksize;
    }
    if disk_size != 0 {
        let mut Buffer: *mut Stream_t = buf_init(
            (*This).head.Next,
            disk_size.wrapping_mul(blocksize as libc::c_ulong),
            disk_size.wrapping_mul(blocksize as libc::c_ulong),
            (*This).sector_size as size_t,
        );
        if !Buffer.is_null() {
            (*This).head.Next = Buffer;
        } else {
            perror(b"init: allocate buffer\0" as *const u8 as *const libc::c_char);
        }
    }
    if fat_read(
        This,
        &mut boot,
        (dev.use_2m & 0x7f as libc::c_int as libc::c_uint) as libc::c_int,
    ) != 0
    {
        fprintf(stderr, b"Error reading FAT\n\0" as *const u8 as *const libc::c_char);
        (*This).num_fat = 1 as libc::c_int as uint8_t;
        free_stream(&mut (*This).head.Next);
        free((*This).head.Next as *mut libc::c_char as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    (*This).cp = cp_open(dev.codepage);
    if ((*This).cp).is_null() {
        fprintf(
            stderr,
            b"Error setting code page\n\0" as *const u8 as *const libc::c_char,
        );
        fs_free(This as *mut Stream_t);
        free_stream(&mut (*This).head.Next);
        free((*This).head.Next as *mut libc::c_char as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    return This as *mut Stream_t;
}
pub unsafe extern "C" fn getDrive(mut Stream: *mut Stream_t) -> libc::c_char {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).head.Class != &mut FsClass as *mut Class_t {
        return getDrive(GetFs(Stream))
    } else {
        return (*This).drive
    };
}
pub unsafe extern "C" fn fsPreallocateClusters(
    mut Fs: *mut Fs_t,
    mut size: uint32_t,
) -> libc::c_int {
    if size > 0 as libc::c_int as libc::c_uint
        && getfreeMinClusters(Fs as *mut Stream_t, size) != 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*Fs).preallocatedClusters = ((*Fs).preallocatedClusters).wrapping_add(size);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fsReleasePreallocateClusters(
    mut Fs: *mut Fs_t,
    mut size: uint32_t,
) {
    (*Fs).preallocatedClusters = ((*Fs).preallocatedClusters).wrapping_sub(size);
}
