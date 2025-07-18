use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub unsafe extern "C" fn compute_lba_geom_from_tot_sectors(
    mut dev: *mut device,
) -> libc::c_int {
    let mut sect_per_track: libc::c_uint = 0;
    let mut tracks: uint32_t = 0;
    if (*dev).heads as libc::c_int != 0 && (*dev).sectors as libc::c_int != 0
        && (*dev).tracks != 0
    {
        return 0 as libc::c_int;
    }
    if (*dev).tot_sectors == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*dev).tot_sectors != 0
        && (*dev).tot_sectors <= 8640 as libc::c_int as libc::c_uint
        && ((*dev).tot_sectors).wrapping_rem(40 as libc::c_int as libc::c_uint)
            == 0 as libc::c_int as libc::c_uint
    {
        if (*dev).tot_sectors <= 540 as libc::c_int as libc::c_uint {
            (*dev).tracks = 40 as libc::c_int as libc::c_uint;
            (*dev).heads = 1 as libc::c_int as uint16_t;
        } else if (*dev).tot_sectors <= 1080 as libc::c_int as libc::c_uint {
            if (*dev).heads as libc::c_int == 1 as libc::c_int {
                (*dev).tracks = 80 as libc::c_int as libc::c_uint;
            } else {
                (*dev).tracks = 40 as libc::c_int as libc::c_uint;
                (*dev).heads = 2 as libc::c_int as uint16_t;
            }
        } else {
            (*dev).tracks = 80 as libc::c_int as libc::c_uint;
            (*dev).heads = 2 as libc::c_int as uint16_t;
        }
        (*dev)
            .sectors = ((*dev).tot_sectors)
            .wrapping_div((*dev).heads as libc::c_uint)
            .wrapping_div((*dev).tracks) as uint16_t;
    }
    if (*dev).sectors == 0 || (*dev).heads == 0 {
        (*dev).sectors = 63 as libc::c_int as uint16_t;
        if (*dev).tot_sectors
            < (16 as libc::c_uint)
                .wrapping_mul((*dev).sectors as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint)
        {
            (*dev).heads = 16 as libc::c_int as uint16_t;
        } else if (*dev).tot_sectors
            < (32 as libc::c_uint)
                .wrapping_mul((*dev).sectors as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint)
        {
            (*dev).heads = 32 as libc::c_int as uint16_t;
        } else if (*dev).tot_sectors
            < (64 as libc::c_uint)
                .wrapping_mul((*dev).sectors as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint)
        {
            (*dev).heads = 64 as libc::c_int as uint16_t;
        } else if (*dev).tot_sectors
            < (128 as libc::c_uint)
                .wrapping_mul((*dev).sectors as libc::c_uint)
                .wrapping_mul(1024 as libc::c_int as libc::c_uint)
        {
            (*dev).heads = 128 as libc::c_int as uint16_t;
        } else {
            (*dev).heads = 255 as libc::c_int as uint16_t;
        }
    }
    if (*dev).tracks == 0 {
        sect_per_track = ((*dev).heads as libc::c_int * (*dev).sectors as libc::c_int)
            as libc::c_uint;
        tracks = ((*dev).tot_sectors)
            .wrapping_add(sect_per_track)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(sect_per_track);
        (*dev).tracks = tracks;
    }
    return 0 as libc::c_int;
}
