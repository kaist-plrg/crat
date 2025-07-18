use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type mt_off_t = off_t;
pub type smt_off_t = mt_off_t;
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
pub unsafe extern "C" fn check_if_sectors_fit(
    mut tot_sectors: uint32_t,
    mut maxBytes: mt_off_t,
    mut sectorSize: uint32_t,
    mut errmsg: *mut libc::c_char,
) -> libc::c_int {
    if maxBytes == 0 {
        return 0 as libc::c_int;
    }
    if tot_sectors as libc::c_long > maxBytes / sectorSize as smt_off_t {
        sprintf(
            errmsg,
            b"%d sectors too large for this platform\n\0" as *const u8
                as *const libc::c_char,
            tot_sectors,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn chs_to_totsectors(
    mut dev: *mut device,
    mut errmsg: *mut libc::c_char,
) -> libc::c_int {
    let mut sect_per_track: uint32_t = 0;
    let mut tot_sectors: uint32_t = 0;
    if (*dev).tot_sectors != 0 {
        return 0 as libc::c_int;
    }
    if (*dev).heads == 0 || (*dev).sectors == 0 || (*dev).tracks == 0 {
        return 0 as libc::c_int;
    }
    sect_per_track = ((*dev).heads as libc::c_int * (*dev).sectors as libc::c_int)
        as uint32_t;
    if (*dev).tracks > (4294967295 as libc::c_uint).wrapping_div(sect_per_track) {
        if !errmsg.is_null() {
            sprintf(
                errmsg,
                b"Number of sectors larger than 2^32\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    tot_sectors = ((*dev).tracks).wrapping_mul(sect_per_track);
    if tot_sectors > ((*dev).hidden).wrapping_rem(sect_per_track) {
        tot_sectors = (tot_sectors as libc::c_uint)
            .wrapping_sub(((*dev).hidden).wrapping_rem(sect_per_track)) as uint32_t
            as uint32_t;
    }
    (*dev).tot_sectors = tot_sectors;
    return 0 as libc::c_int;
}
