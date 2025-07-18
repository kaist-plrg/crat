use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
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
pub struct OldDos_t {
    pub tracks: libc::c_uint,
    pub sectors: uint16_t,
    pub heads: uint16_t,
    pub dir_len: uint16_t,
    pub cluster_size: uint8_t,
    pub fat_len: uint32_t,
    pub media: uint8_t,
}
static mut old_dos: [OldDos_t; 11] = [
    {
        let mut init = OldDos_t {
            tracks: 40 as libc::c_int as libc::c_uint,
            sectors: 9 as libc::c_int as uint16_t,
            heads: 1 as libc::c_int as uint16_t,
            dir_len: 4 as libc::c_int as uint16_t,
            cluster_size: 1 as libc::c_int as uint8_t,
            fat_len: 2 as libc::c_int as uint32_t,
            media: 0xfc as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 40 as libc::c_int as libc::c_uint,
            sectors: 9 as libc::c_int as uint16_t,
            heads: 2 as libc::c_int as uint16_t,
            dir_len: 7 as libc::c_int as uint16_t,
            cluster_size: 2 as libc::c_int as uint8_t,
            fat_len: 2 as libc::c_int as uint32_t,
            media: 0xfd as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 40 as libc::c_int as libc::c_uint,
            sectors: 8 as libc::c_int as uint16_t,
            heads: 1 as libc::c_int as uint16_t,
            dir_len: 4 as libc::c_int as uint16_t,
            cluster_size: 1 as libc::c_int as uint8_t,
            fat_len: 1 as libc::c_int as uint32_t,
            media: 0xfe as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 40 as libc::c_int as libc::c_uint,
            sectors: 8 as libc::c_int as uint16_t,
            heads: 2 as libc::c_int as uint16_t,
            dir_len: 7 as libc::c_int as uint16_t,
            cluster_size: 2 as libc::c_int as uint8_t,
            fat_len: 1 as libc::c_int as uint32_t,
            media: 0xff as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as libc::c_int as libc::c_uint,
            sectors: 9 as libc::c_int as uint16_t,
            heads: 2 as libc::c_int as uint16_t,
            dir_len: 7 as libc::c_int as uint16_t,
            cluster_size: 2 as libc::c_int as uint8_t,
            fat_len: 3 as libc::c_int as uint32_t,
            media: 0xf9 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as libc::c_int as libc::c_uint,
            sectors: 15 as libc::c_int as uint16_t,
            heads: 2 as libc::c_int as uint16_t,
            dir_len: 14 as libc::c_int as uint16_t,
            cluster_size: 1 as libc::c_int as uint8_t,
            fat_len: 7 as libc::c_int as uint32_t,
            media: 0xf9 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as libc::c_int as libc::c_uint,
            sectors: 18 as libc::c_int as uint16_t,
            heads: 2 as libc::c_int as uint16_t,
            dir_len: 14 as libc::c_int as uint16_t,
            cluster_size: 1 as libc::c_int as uint8_t,
            fat_len: 9 as libc::c_int as uint32_t,
            media: 0xf0 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as libc::c_int as libc::c_uint,
            sectors: 36 as libc::c_int as uint16_t,
            heads: 2 as libc::c_int as uint16_t,
            dir_len: 15 as libc::c_int as uint16_t,
            cluster_size: 2 as libc::c_int as uint8_t,
            fat_len: 9 as libc::c_int as uint32_t,
            media: 0xf0 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as libc::c_int as libc::c_uint,
            sectors: 8 as libc::c_int as uint16_t,
            heads: 2 as libc::c_int as uint16_t,
            dir_len: 7 as libc::c_int as uint16_t,
            cluster_size: 2 as libc::c_int as uint8_t,
            fat_len: 2 as libc::c_int as uint32_t,
            media: 0xfb as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as libc::c_int as libc::c_uint,
            sectors: 8 as libc::c_int as uint16_t,
            heads: 1 as libc::c_int as uint16_t,
            dir_len: 7 as libc::c_int as uint16_t,
            cluster_size: 2 as libc::c_int as uint8_t,
            fat_len: 2 as libc::c_int as uint32_t,
            media: 0xfa as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = OldDos_t {
            tracks: 80 as libc::c_int as libc::c_uint,
            sectors: 9 as libc::c_int as uint16_t,
            heads: 1 as libc::c_int as uint16_t,
            dir_len: 7 as libc::c_int as uint16_t,
            cluster_size: 2 as libc::c_int as uint8_t,
            fat_len: 2 as libc::c_int as uint32_t,
            media: 0xf8 as libc::c_int as uint8_t,
        };
        init
    },
];
pub unsafe extern "C" fn getOldDosBySize(mut size: size_t) -> *mut OldDos_t {
    let mut i: size_t = 0;
    size = size.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[OldDos_t; 11]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<OldDos_t>() as libc::c_ulong)
    {
        if (old_dos[i as usize].sectors as libc::c_uint)
            .wrapping_mul(old_dos[i as usize].tracks)
            .wrapping_mul(old_dos[i as usize].heads as libc::c_uint) as libc::c_ulong
            == size
        {
            return &mut *old_dos.as_mut_ptr().offset(i as isize) as *mut OldDos_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut OldDos_t;
}
pub unsafe extern "C" fn getOldDosByMedia(mut media: libc::c_int) -> *mut OldDos_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[OldDos_t; 11]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<OldDos_t>() as libc::c_ulong)
    {
        if old_dos[i as usize].media as libc::c_int == media {
            return &mut *old_dos.as_mut_ptr().offset(i as isize) as *mut OldDos_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(
        stderr,
        b"Unknown media type %02x\n\0" as *const u8 as *const libc::c_char,
        media,
    );
    return 0 as *mut OldDos_t;
}
pub unsafe extern "C" fn getOldDosByParams(
    mut tracks: libc::c_uint,
    mut heads: libc::c_uint,
    mut sectors: libc::c_uint,
    mut dir_len: libc::c_uint,
    mut cluster_size: libc::c_uint,
) -> *mut OldDos_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[OldDos_t; 11]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<OldDos_t>() as libc::c_ulong)
    {
        if sectors == old_dos[i as usize].sectors as libc::c_uint
            && tracks == old_dos[i as usize].tracks
            && heads == old_dos[i as usize].heads as libc::c_uint
            && (dir_len == 0 as libc::c_int as libc::c_uint
                || dir_len == old_dos[i as usize].dir_len as libc::c_uint)
            && (cluster_size == 0 as libc::c_int as libc::c_uint
                || cluster_size == old_dos[i as usize].cluster_size as libc::c_uint)
        {
            return &mut *old_dos.as_mut_ptr().offset(i as isize) as *mut OldDos_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut OldDos_t;
}
pub unsafe extern "C" fn setDeviceFromOldDos(
    mut media: libc::c_int,
    mut dev: *mut device,
) -> libc::c_int {
    let mut params: *mut OldDos_t = getOldDosByMedia(media);
    if params.is_null() {
        return -(1 as libc::c_int);
    }
    (*dev).heads = (*params).heads;
    (*dev).tracks = (*params).tracks;
    (*dev).sectors = (*params).sectors;
    (*dev).ssize = 0x80 as libc::c_int as uint8_t;
    (*dev).use_2m = !(1 as libc::c_uint);
    return 0 as libc::c_int;
}
