use ::libc;
extern "C" {
    pub type doscp_t;
    static max_off_t_31: mt_off_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    fn compute_lba_geom_from_tot_sectors(dev: *mut device) -> libc::c_int;
    fn XdfOpen(
        dev: *mut device,
        name: *const libc::c_char,
        mode: libc::c_int,
        errmsg: *mut libc::c_char,
        info: *mut xdf_info,
    ) -> *mut Stream_t;
    fn SimpleFileOpenWithLm(
        dev: *mut device,
        orig_dev: *mut device,
        name: *const libc::c_char,
        mode: libc::c_int,
        errmsg: *mut libc::c_char,
        mode2: libc::c_int,
        locked: libc::c_int,
        lockMode: libc::c_int,
        maxSize: *mut mt_off_t,
        geomFailure: *mut libc::c_int,
    ) -> *mut Stream_t;
    fn FloppydOpen(
        dev: *mut device,
        name: *const libc::c_char,
        mode: libc::c_int,
        errmsg: *mut libc::c_char,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn OpenScsi(
        dev: *mut device,
        name: *const libc::c_char,
        mode: libc::c_int,
        errmsg: *mut libc::c_char,
        mode2: libc::c_int,
        locked: libc::c_int,
        lockMode: libc::c_int,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn Remap(
        Next: *mut Stream_t,
        dev: *mut device,
        errmsg: *mut libc::c_char,
    ) -> *mut Stream_t;
    fn OpenPartition(
        Next: *mut Stream_t,
        dev: *mut device,
        errmsg: *mut libc::c_char,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn OpenOffset(
        Next: *mut Stream_t,
        dev: *mut device,
        offset: off_t,
        errmsg: *mut libc::c_char,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn OpenSwap(Next: *mut Stream_t) -> *mut Stream_t;
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
pub type mt_off_t = off_t;
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
pub struct xdf_info {
    pub FatSize: libc::c_uint,
    pub RootDirSize: uint16_t,
    pub BadSectors: libc::c_uint,
}
pub unsafe extern "C" fn OpenImage(
    mut out_dev: *mut device,
    mut dev: *mut device,
    mut name: *const libc::c_char,
    mut mode: libc::c_int,
    mut errmsg: *mut libc::c_char,
    mut flags: libc::c_int,
    mut lockMode: libc::c_int,
    mut maxSize: *mut mt_off_t,
    mut geomFailureP: *mut libc::c_int,
    mut xdf_info: *mut xdf_info,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut geomFailure: libc::c_int = 0 as libc::c_int;
    if (*out_dev).misc_flags & 0x40 as libc::c_uint != 0 {
        Stream = FloppydOpen(out_dev, name, mode, errmsg, maxSize);
    } else {
        Stream = XdfOpen(out_dev, name, mode, errmsg, xdf_info);
        if !Stream.is_null() {
            (*out_dev).use_2m = 0x7f as libc::c_int as libc::c_uint;
            if !maxSize.is_null() {
                *maxSize = max_off_t_31;
            }
        }
        if Stream.is_null() {
            Stream = OpenScsi(
                out_dev,
                name,
                mode,
                errmsg,
                flags,
                0 as libc::c_int,
                lockMode,
                maxSize,
            );
        }
        if Stream.is_null() {
            Stream = SimpleFileOpenWithLm(
                out_dev,
                dev,
                name,
                mode,
                errmsg,
                flags,
                0 as libc::c_int,
                lockMode,
                maxSize,
                &mut geomFailure,
            );
        }
        if geomFailure != 0 {
            if *geomFailureP != 0 {
                *geomFailureP = geomFailure;
            }
            return 0 as *mut Stream_t;
        }
    }
    if Stream.is_null() {
        return 0 as *mut Stream_t;
    }
    if !((*dev).data_map).is_null() {
        let mut Remapped: *mut Stream_t = Remap(Stream, out_dev, errmsg);
        if Remapped.is_null() {
            current_block = 5396729104290913487;
        } else {
            Stream = Remapped;
            current_block = 5783071609795492627;
        }
    } else {
        current_block = 5783071609795492627;
    }
    match current_block {
        5783071609795492627 => {
            if (*dev).offset != 0 {
                let mut Offset: *mut Stream_t = OpenOffset(
                    Stream,
                    out_dev,
                    (*dev).offset,
                    errmsg,
                    maxSize,
                );
                if Offset.is_null() {
                    current_block = 5396729104290913487;
                } else {
                    Stream = Offset;
                    current_block = 9828876828309294594;
                }
            } else {
                current_block = 9828876828309294594;
            }
            match current_block {
                5396729104290913487 => {}
                _ => {
                    if !dev.is_null() && (*dev).misc_flags & 0x100 as libc::c_uint != 0 {
                        let mut Swap: *mut Stream_t = OpenSwap(Stream);
                        if Swap.is_null() {
                            current_block = 5396729104290913487;
                        } else {
                            Stream = Swap;
                            current_block = 1608152415753874203;
                        }
                    } else {
                        current_block = 1608152415753874203;
                    }
                    match current_block {
                        5396729104290913487 => {}
                        _ => {
                            if !(flags & 4 as libc::c_int != 0
                                && compute_lba_geom_from_tot_sectors(out_dev)
                                    < 0 as libc::c_int)
                            {
                                if (*dev).partition != 0 && flags & 2 as libc::c_int == 0 {
                                    let mut Partition: *mut Stream_t = OpenPartition(
                                        Stream,
                                        out_dev,
                                        errmsg,
                                        maxSize,
                                    );
                                    if Partition.is_null() {
                                        current_block = 5396729104290913487;
                                    } else {
                                        Stream = Partition;
                                        current_block = 11913429853522160501;
                                    }
                                } else {
                                    current_block = 11913429853522160501;
                                }
                                match current_block {
                                    5396729104290913487 => {}
                                    _ => return Stream,
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free_stream(&mut Stream);
    return 0 as *mut Stream_t;
}
