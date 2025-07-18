use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __locale_data;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __toupper_l(__c: libc::c_int, __l: locale_t) -> libc::c_int;
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strncasecmp_l(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
        __loc: locale_t,
    ) -> libc::c_int;
    fn newlocale(
        __category_mask: libc::c_int,
        __locale: *const libc::c_char,
        __base: locale_t,
    ) -> locale_t;
    static mut const_devices: [device; 0];
    static nr_const_devices: libc::c_uint;
    fn strtosi(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_int;
    fn strtoui(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> libc::c_uint;
    fn strtou8(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint8_t;
    fn strtou16(
        nptr: *const libc::c_char,
        endptr: *mut *mut libc::c_char,
        base: libc::c_int,
    ) -> uint16_t;
    fn str_to_offset(str: *mut libc::c_char) -> off_t;
    fn get_homedir() -> *mut libc::c_char;
    fn printOom();
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type off_t = __off_t;
pub type caddr_t = __caddr_t;
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
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const libc::c_ushort,
    pub __ctype_tolower: *const libc::c_int,
    pub __ctype_toupper: *const libc::c_int,
    pub __names: [*const libc::c_char; 13],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
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
pub type switches_t = switches_l;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct switches_l {
    pub name: *const libc::c_char,
    pub address: caddr_t,
    pub type_0: C2RustUnnamed,
}
pub type C2RustUnnamed = libc::c_uint;
pub const T_UQSTRING: C2RustUnnamed = 5;
pub const T_UINT16: C2RustUnnamed = 4;
pub const T_UINT8: C2RustUnnamed = 3;
pub const T_UINT: C2RustUnnamed = 2;
pub const T_STRING: C2RustUnnamed = 1;
pub const T_INT: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub name: *const libc::c_char,
    pub fat_bits: libc::c_schar,
    pub tracks: libc::c_uint,
    pub heads: libc::c_ushort,
    pub sectors: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flags_t {
    pub name: *const libc::c_char,
    pub flag: libc::c_uint,
}
#[inline]
unsafe extern "C" fn ptrdiff(
    mut end: *const libc::c_char,
    mut begin: *const libc::c_char,
) -> size_t {
    return end.offset_from(begin) as libc::c_long as size_t;
}
static mut buffer: [libc::c_char; 257] = [0; 257];
static mut pos: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut token: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut token_length: size_t = 0;
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
static mut linenumber: libc::c_int = 0;
static mut lastTokenLinenumber: libc::c_int = 0;
static mut filename: *const libc::c_char = 0 as *const libc::c_char;
static mut file_nr: libc::c_int = 0 as libc::c_int;
static mut flag_mask: libc::c_uint = 0;
static mut cur_devs: libc::c_uint = 0;
static mut cur_dev: libc::c_int = 0;
static mut trusted: libc::c_int = 0 as libc::c_int;
static mut nr_dev: libc::c_uint = 0;
pub static mut devices: *mut device = 0 as *const device as *mut device;
static mut token_nr: libc::c_int = 0;
static mut default_drive: libc::c_char = '\0' as i32 as libc::c_char;
pub static mut mtools_skip_check: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut mtools_fat_compatibility: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut mtools_ignore_short_case: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut mtools_rate_0: uint8_t = 0 as libc::c_int as uint8_t;
pub static mut mtools_rate_any: uint8_t = 0 as libc::c_int as uint8_t;
pub static mut mtools_no_vfat: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut mtools_numeric_tail: libc::c_uint = 1 as libc::c_int as libc::c_uint;
pub static mut mtools_dotted_dir: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut mtools_twenty_four_hour_clock: libc::c_uint = 1 as libc::c_int
    as libc::c_uint;
pub static mut mtools_lock_timeout: libc::c_uint = 30 as libc::c_int as libc::c_uint;
pub static mut mtools_default_codepage: libc::c_uint = 850 as libc::c_int
    as libc::c_uint;
pub static mut mtools_date_string: *const libc::c_char = b"yyyy-mm-dd\0" as *const u8
    as *const libc::c_char;
static mut global_switches: [switches_t; 12] = unsafe {
    [
        {
            let mut init = switches_l {
                name: b"MTOOLS_LOWER_CASE\0" as *const u8 as *const libc::c_char,
                address: &mtools_ignore_short_case as *const libc::c_uint
                    as *mut libc::c_uint as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_FAT_COMPATIBILITY\0" as *const u8 as *const libc::c_char,
                address: &mtools_fat_compatibility as *const libc::c_uint
                    as *mut libc::c_uint as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_SKIP_CHECK\0" as *const u8 as *const libc::c_char,
                address: &mtools_skip_check as *const libc::c_uint as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_NO_VFAT\0" as *const u8 as *const libc::c_char,
                address: &mtools_no_vfat as *const libc::c_uint as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_RATE_0\0" as *const u8 as *const libc::c_char,
                address: &mtools_rate_0 as *const uint8_t as *mut uint8_t as caddr_t,
                type_0: T_UINT8,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_RATE_ANY\0" as *const u8 as *const libc::c_char,
                address: &mtools_rate_any as *const uint8_t as *mut uint8_t as caddr_t,
                type_0: T_UINT8,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_NAME_NUMERIC_TAIL\0" as *const u8 as *const libc::c_char,
                address: &mtools_numeric_tail as *const libc::c_uint as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_DOTTED_DIR\0" as *const u8 as *const libc::c_char,
                address: &mtools_dotted_dir as *const libc::c_uint as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_TWENTY_FOUR_HOUR_CLOCK\0" as *const u8
                    as *const libc::c_char,
                address: &mtools_twenty_four_hour_clock as *const libc::c_uint
                    as *mut libc::c_uint as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_DATE_STRING\0" as *const u8 as *const libc::c_char,
                address: &mtools_date_string as *const *const libc::c_char
                    as *mut *const libc::c_char as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MTOOLS_LOCK_TIMEOUT\0" as *const u8 as *const libc::c_char,
                address: &mtools_lock_timeout as *const libc::c_uint as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"DEFAULT_CODEPAGE\0" as *const u8 as *const libc::c_char,
                address: &mtools_default_codepage as *const libc::c_uint
                    as *mut libc::c_uint as caddr_t,
                type_0: T_UINT,
            };
            init
        },
    ]
};
static mut openflags: [flags_t; 4] = [
    {
        let mut init = flags_t {
            name: b"sync\0" as *const u8 as *const libc::c_char,
            flag: 0o4010000 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"nodelay\0" as *const u8 as *const libc::c_char,
            flag: 0o4000 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"exclusive\0" as *const u8 as *const libc::c_char,
            flag: 0o200 as libc::c_int as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"none\0" as *const u8 as *const libc::c_char,
            flag: 0 as libc::c_int as libc::c_uint,
        };
        init
    },
];
static mut misc_flags: [flags_t; 9] = [
    {
        let mut init = flags_t {
            name: b"use_xdf\0" as *const u8 as *const libc::c_char,
            flag: 0x8 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"scsi\0" as *const u8 as *const libc::c_char,
            flag: 0x1 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"nolock\0" as *const u8 as *const libc::c_char,
            flag: 0x4 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"mformat_only\0" as *const u8 as *const libc::c_char,
            flag: 0x10 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"filter\0" as *const u8 as *const libc::c_char,
            flag: 0x80 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"privileged\0" as *const u8 as *const libc::c_char,
            flag: 0x2 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"vold\0" as *const u8 as *const libc::c_char,
            flag: 0x20 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"remote\0" as *const u8 as *const libc::c_char,
            flag: 0x40 as libc::c_uint,
        };
        init
    },
    {
        let mut init = flags_t {
            name: b"swap\0" as *const u8 as *const libc::c_char,
            flag: 0x100 as libc::c_uint,
        };
        init
    },
];
static mut default_formats: [C2RustUnnamed_0; 15] = [
    {
        let mut init = C2RustUnnamed_0 {
            name: b"hd514\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 15 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"high-density-5-1/4\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 15 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"1.2m\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 15 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"hd312\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 18 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"high-density-3-1/2\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 18 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"1.44m\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 18 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"dd312\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 9 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"double-density-3-1/2\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 9 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"720k\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 80 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 9 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"dd514\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 40 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 9 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"double-density-5-1/4\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 40 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 9 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"360k\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 40 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 9 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"320k\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 40 as libc::c_int as libc::c_uint,
            heads: 2 as libc::c_int as libc::c_ushort,
            sectors: 8 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"180k\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 40 as libc::c_int as libc::c_uint,
            heads: 1 as libc::c_int as libc::c_ushort,
            sectors: 9 as libc::c_int as libc::c_ushort,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            name: b"160k\0" as *const u8 as *const libc::c_char,
            fat_bits: 12 as libc::c_int as libc::c_schar,
            tracks: 40 as libc::c_int as libc::c_uint,
            heads: 1 as libc::c_int as libc::c_ushort,
            sectors: 8 as libc::c_int as libc::c_ushort,
        };
        init
    },
];
static mut dswitches: [switches_t; 16] = [switches_t {
    name: 0 as *const libc::c_char,
    address: 0 as *const libc::c_char as *mut libc::c_char,
    type_0: T_INT,
}; 16];
static mut C: locale_t = 0 as *const __locale_struct as locale_t;
unsafe extern "C" fn init_canon() {
    if C.is_null() {
        C = newlocale(
            (1 as libc::c_int) << 0 as libc::c_int,
            b"C\0" as *const u8 as *const libc::c_char,
            0 as locale_t,
        );
    }
}
unsafe extern "C" fn canon_drv(mut drive: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    init_canon();
    ret = ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = drive;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *((*C).__ctype_toupper).offset(__c as isize)
                };
            } else {
                __res = __toupper_l(drive, C);
            }
        } else {
            __res = *((*C).__ctype_toupper).offset(drive as isize);
        }
        __res
    });
    return ret;
}
unsafe extern "C" fn cmp_tok(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    init_canon();
    return strncasecmp_l(a, b, len, C);
}
unsafe extern "C" fn ch_canon_drv(mut drive: libc::c_char) -> libc::c_char {
    return canon_drv(drive as libc::c_uchar as libc::c_int) as libc::c_char;
}
unsafe extern "C" fn maintain_default_drive(mut drive: libc::c_char) {
    if default_drive as libc::c_int == ':' as i32 {
        return;
    }
    if default_drive as libc::c_int == '\0' as i32
        || default_drive as libc::c_int > drive as libc::c_int
    {
        default_drive = drive;
    }
}
pub unsafe extern "C" fn get_default_drive() -> libc::c_char {
    if default_drive as libc::c_int != '\0' as i32 {
        return default_drive
    } else {
        return 'A' as i32 as libc::c_char
    };
}
unsafe extern "C" fn syntax(
    mut msg: *const libc::c_char,
    mut thisLine: libc::c_int,
) -> ! {
    let mut drive: libc::c_char = '\0' as i32 as libc::c_char;
    if thisLine != 0 {
        lastTokenLinenumber = linenumber;
    }
    if cur_dev >= 0 as libc::c_int {
        drive = (*devices.offset(cur_dev as isize)).drive;
    }
    fprintf(
        stderr,
        b"Syntax error at line %d \0" as *const u8 as *const libc::c_char,
        lastTokenLinenumber,
    );
    if drive != 0 {
        fprintf(
            stderr,
            b"for drive %c: \0" as *const u8 as *const libc::c_char,
            drive as libc::c_int,
        );
    }
    if !token.is_null() {
        fprintf(
            stderr,
            b"column %ld \0" as *const u8 as *const libc::c_char,
            token.offset_from(buffer.as_mut_ptr()) as libc::c_long,
        );
    }
    fprintf(
        stderr,
        b"in file %s: %s\0" as *const u8 as *const libc::c_char,
        filename,
        msg,
    );
    if *__errno_location() != 0 as libc::c_int {
        fprintf(
            stderr,
            b" (%s)\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn get_env_conf() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[switches_t; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<switches_t>() as libc::c_ulong)
    {
        s = getenv(global_switches[i as usize].name);
        if !s.is_null() {
            *__errno_location() = 0 as libc::c_int;
            match global_switches[i as usize].type_0 as libc::c_uint {
                0 => {
                    *(global_switches[i as usize].address
                        as *mut libc::c_int) = strtosi(
                        s,
                        0 as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    );
                }
                2 => {
                    *(global_switches[i as usize].address
                        as *mut libc::c_uint) = strtoui(
                        s,
                        0 as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    );
                }
                3 => {
                    *(global_switches[i as usize].address
                        as *mut uint8_t) = strtou8(
                        s,
                        0 as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    );
                }
                4 => {
                    *(global_switches[i as usize].address
                        as *mut uint16_t) = strtou16(
                        s,
                        0 as *mut *mut libc::c_char,
                        0 as libc::c_int,
                    );
                }
                1 | 5 => {
                    let ref mut fresh0 = *(global_switches[i as usize].address
                        as *mut *mut libc::c_char);
                    *fresh0 = s;
                }
                _ => {}
            }
            if *__errno_location() != 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"Bad number %s for %s (%s)\n\0" as *const u8 as *const libc::c_char,
                    s,
                    global_switches[i as usize].name,
                    strerror(*__errno_location()),
                );
                exit(1 as libc::c_int);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn mtools_getline() -> libc::c_int {
    if fp.is_null()
        || (fgets(buffer.as_mut_ptr(), 256 as libc::c_int + 1 as libc::c_int, fp))
            .is_null()
    {
        return -(1 as libc::c_int);
    }
    linenumber += 1;
    linenumber;
    pos = buffer.as_mut_ptr();
    token_nr = 0 as libc::c_int;
    buffer[256 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if strlen(buffer.as_mut_ptr()) == 256 as libc::c_int as libc::c_ulong {
        syntax(b"line too long\0" as *const u8 as *const libc::c_char, 1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn skip_junk(mut expect: libc::c_int) {
    lastTokenLinenumber = linenumber;
    while pos.is_null() || *pos == 0
        || !(strchr(
            b" #\n\t\0" as *const u8 as *const libc::c_char,
            *pos as libc::c_int,
        ))
            .is_null()
    {
        if pos.is_null() || *pos == 0 || *pos as libc::c_int == '#' as i32 {
            if mtools_getline() != 0 {
                pos = 0 as *mut libc::c_char;
                if expect != 0 {
                    syntax(
                        b"end of file unexpected\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    );
                }
                return;
            }
        } else {
            pos = pos.offset(1);
            pos;
        }
    }
    token_nr += 1;
    token_nr;
}
unsafe extern "C" fn get_next_token() -> *mut libc::c_char {
    skip_junk(0 as libc::c_int);
    if pos.is_null() {
        token_length = 0 as libc::c_int as size_t;
        token = 0 as *mut libc::c_char;
        return 0 as *mut libc::c_char;
    }
    token = pos;
    token_length = strcspn(token, b" \t\n#:=\0" as *const u8 as *const libc::c_char);
    pos = pos.offset(token_length as isize);
    return token;
}
unsafe extern "C" fn match_token(mut template: *const libc::c_char) -> libc::c_int {
    return (strlen(template) == token_length
        && cmp_tok(template, token, token_length) == 0) as libc::c_int;
}
unsafe extern "C" fn expect_char(mut c: libc::c_char) {
    let mut buf: [libc::c_char; 11] = [0; 11];
    skip_junk(1 as libc::c_int);
    if *pos as libc::c_int != c as libc::c_int {
        sprintf(
            buf.as_mut_ptr(),
            b"expected %c\0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
        );
        syntax(buf.as_mut_ptr(), 1 as libc::c_int);
    }
    pos = pos.offset(1);
    pos;
}
unsafe extern "C" fn get_string() -> *mut libc::c_char {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    skip_junk(1 as libc::c_int);
    if *pos as libc::c_int != '"' as i32 {
        syntax(b" \" expected\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    }
    str = pos.offset(1 as libc::c_int as isize);
    end = strchr(str, '"' as i32);
    if end.is_null() {
        syntax(
            b"unterminated string constant\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    str = strndup(str, ptrdiff(end, str));
    pos = end.offset(1 as libc::c_int as isize);
    return str;
}
unsafe extern "C" fn get_unquoted_string() -> *mut libc::c_char {
    if *pos as libc::c_int == '"' as i32 {
        return get_string()
    } else {
        let mut str: *mut libc::c_char = get_next_token();
        return strndup(str, token_length);
    };
}
unsafe extern "C" fn get_unumber(mut max: libc::c_ulong) -> libc::c_ulong {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_ulong = 0;
    skip_junk(1 as libc::c_int);
    last = pos;
    n = strtoul(pos, &mut pos, 0 as libc::c_int);
    if *__errno_location() != 0 {
        syntax(b"bad number\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    }
    if last == pos {
        syntax(
            b"numeral expected\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if n > max {
        syntax(
            b"number too big\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    pos = pos.offset(1);
    pos;
    token_nr += 1;
    token_nr;
    return n;
}
unsafe extern "C" fn get_number() -> libc::c_int {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    skip_junk(1 as libc::c_int);
    last = pos;
    n = strtol(pos, &mut pos, 0 as libc::c_int) as libc::c_int;
    if *__errno_location() != 0 {
        syntax(b"bad number\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    }
    if last == pos {
        syntax(
            b"numeral expected\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    pos = pos.offset(1);
    pos;
    token_nr += 1;
    token_nr;
    return n;
}
unsafe extern "C" fn purge(mut drive: libc::c_char, mut fn_0: libc::c_int) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    drive = ch_canon_drv(drive);
    j = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < cur_devs {
        if (*devices.offset(i as isize)).drive as libc::c_int != drive as libc::c_int
            || (*devices.offset(i as isize)).file_nr == fn_0
        {
            let fresh1 = j;
            j = j.wrapping_add(1);
            *devices.offset(fresh1 as isize) = *devices.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    cur_devs = j;
}
unsafe extern "C" fn grow() {
    if cur_devs >= nr_dev.wrapping_sub(2 as libc::c_int as libc::c_uint) {
        nr_dev = cur_devs.wrapping_add(2 as libc::c_int as libc::c_uint)
            << 1 as libc::c_int;
        devices = realloc(
            devices as *mut libc::c_char as *mut libc::c_void,
            (nr_dev as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<device>() as libc::c_ulong),
        ) as *mut device;
        if devices.is_null() {
            printOom();
            exit(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn init_drive() {
    memset(
        &mut *devices.offset(cur_dev as isize) as *mut device as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<device>() as libc::c_ulong,
    );
    (*devices.offset(cur_dev as isize)).ssize = 2 as libc::c_int as uint8_t;
}
unsafe extern "C" fn prepend() {
    let mut i: libc::c_uint = 0;
    grow();
    i = cur_devs;
    while i > 0 as libc::c_int as libc::c_uint {
        *devices
            .offset(
                i as isize,
            ) = *devices
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        i = i.wrapping_sub(1);
        i;
    }
    cur_dev = 0 as libc::c_int;
    cur_devs = cur_devs.wrapping_add(1);
    cur_devs;
    init_drive();
}
unsafe extern "C" fn append() {
    grow();
    cur_dev = cur_devs as libc::c_int;
    cur_devs = cur_devs.wrapping_add(1);
    cur_devs;
    init_drive();
}
unsafe extern "C" fn finish_drive_clause() {
    if cur_dev == -(1 as libc::c_int) {
        trusted = 0 as libc::c_int;
        return;
    }
    if ((*devices.offset(cur_dev as isize)).name).is_null() {
        syntax(
            b"missing filename\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if (*devices.offset(cur_dev as isize)).tracks != 0
        || (*devices.offset(cur_dev as isize)).heads as libc::c_int != 0
        || (*devices.offset(cur_dev as isize)).sectors as libc::c_int != 0
    {
        if (*devices.offset(cur_dev as isize)).tracks == 0
            || (*devices.offset(cur_dev as isize)).heads == 0
            || (*devices.offset(cur_dev as isize)).sectors == 0
        {
            syntax(
                b"incomplete geometry: either indicate all of track/heads/sectors or none of them\0"
                    as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        if (*devices.offset(cur_dev as isize)).misc_flags
            & (0x10 as libc::c_uint | 0x80 as libc::c_uint) == 0
        {
            syntax(
                b"if you supply a geometry, you also must supply one of the `mformat_only' or `filter' flags\0"
                    as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
    }
    (*devices.offset(cur_dev as isize)).file_nr = file_nr;
    let ref mut fresh2 = (*devices.offset(cur_dev as isize)).cfg_filename;
    *fresh2 = filename;
    if trusted == 0
        && (*devices.offset(cur_dev as isize)).misc_flags & 0x2 as libc::c_uint != 0
    {
        fprintf(
            stderr,
            b"Warning: privileged flag ignored for drive %c: defined in file %s\n\0"
                as *const u8 as *const libc::c_char,
            canon_drv((*devices.offset(cur_dev as isize)).drive as libc::c_int),
            filename,
        );
        (*devices.offset(cur_dev as isize)).misc_flags &= !(0x2 as libc::c_uint);
    }
    trusted = 0 as libc::c_int;
    cur_dev = -(1 as libc::c_int);
}
unsafe extern "C" fn set_var(
    mut switches: *mut switches_l,
    mut nr: libc::c_int,
    mut base_address: caddr_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nr {
        if match_token((*switches.offset(i as isize)).name) != 0 {
            expect_char('=' as i32 as libc::c_char);
            if (*switches.offset(i as isize)).type_0 as libc::c_uint
                == T_UINT as libc::c_int as libc::c_uint
            {
                *(base_address
                    .offset(
                        (*switches.offset(i as isize)).address as libc::c_long as isize,
                    )
                    as *mut libc::c_uint) = get_unumber(
                    (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint) as libc::c_ulong,
                ) as libc::c_uint;
            } else if (*switches.offset(i as isize)).type_0 as libc::c_uint
                == T_UINT8 as libc::c_int as libc::c_uint
            {
                *(base_address
                    .offset(
                        (*switches.offset(i as isize)).address as libc::c_long as isize,
                    )
                    as *mut uint8_t) = get_unumber(255 as libc::c_int as libc::c_ulong)
                    as uint8_t;
            } else if (*switches.offset(i as isize)).type_0 as libc::c_uint
                == T_UINT16 as libc::c_int as libc::c_uint
            {
                *(base_address
                    .offset(
                        (*switches.offset(i as isize)).address as libc::c_long as isize,
                    )
                    as *mut uint16_t) = get_unumber(
                    65535 as libc::c_int as libc::c_ulong,
                ) as uint16_t;
            } else if (*switches.offset(i as isize)).type_0 as libc::c_uint
                == T_INT as libc::c_int as libc::c_uint
            {
                *(base_address
                    .offset(
                        (*switches.offset(i as isize)).address as libc::c_long as isize,
                    ) as *mut libc::c_int) = get_number();
            } else if (*switches.offset(i as isize)).type_0 as libc::c_uint
                == T_STRING as libc::c_int as libc::c_uint
            {
                let ref mut fresh3 = *(base_address
                    .offset(
                        (*switches.offset(i as isize)).address as libc::c_long as isize,
                    ) as *mut *mut libc::c_char);
                *fresh3 = get_string();
            } else if (*switches.offset(i as isize)).type_0 as libc::c_uint
                == T_UQSTRING as libc::c_int as libc::c_uint
            {
                let ref mut fresh4 = *(base_address
                    .offset(
                        (*switches.offset(i as isize)).address as libc::c_long as isize,
                    ) as *mut *mut libc::c_char);
                *fresh4 = get_unquoted_string();
            }
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn set_openflags(mut dev: *mut device) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[flags_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<flags_t>() as libc::c_ulong)
    {
        if match_token(openflags[i as usize].name) != 0 {
            (*dev)
                .mode = ((*dev).mode as libc::c_uint | openflags[i as usize].flag)
                as libc::c_int;
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn set_misc_flags(mut dev: *mut device) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[flags_t; 9]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<flags_t>() as libc::c_ulong)
    {
        if match_token(misc_flags[i as usize].name) != 0 {
            flag_mask |= misc_flags[i as usize].flag;
            skip_junk(0 as libc::c_int);
            if !pos.is_null() && *pos as libc::c_int == '=' as i32 {
                pos = pos.offset(1);
                pos;
                match get_number() {
                    0 => return 0 as libc::c_int,
                    1 => {}
                    _ => {
                        syntax(
                            b"expected 0 or 1\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int,
                        );
                    }
                }
            }
            (*dev).misc_flags |= misc_flags[i as usize].flag;
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn set_def_format(mut dev: *mut device) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_0; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        if match_token(default_formats[i as usize].name) != 0 {
            if (*dev).ssize == 0 {
                (*dev).ssize = 2 as libc::c_int as uint8_t;
            }
            if (*dev).tracks == 0 {
                (*dev).tracks = default_formats[i as usize].tracks;
            }
            if (*dev).heads == 0 {
                (*dev).heads = default_formats[i as usize].heads;
            }
            if (*dev).sectors == 0 {
                (*dev).sectors = default_formats[i as usize].sectors;
            }
            if (*dev).fat_bits == 0 {
                (*dev).fat_bits = default_formats[i as usize].fat_bits as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn set_cmd_line_image(mut img: *mut libc::c_char) {
    let mut ofsp: *mut libc::c_char = 0 as *mut libc::c_char;
    prepend();
    (*devices.offset(cur_dev as isize)).drive = ':' as i32 as libc::c_char;
    default_drive = ':' as i32 as libc::c_char;
    ofsp = strstr(img, b"@@\0" as *const u8 as *const libc::c_char);
    if ofsp.is_null() {
        let ref mut fresh5 = (*devices.offset(cur_dev as isize)).name;
        *fresh5 = strdup(img);
        (*devices.offset(cur_dev as isize)).offset = 0 as libc::c_int as off_t;
    } else {
        let ref mut fresh6 = (*devices.offset(cur_dev as isize)).name;
        *fresh6 = strndup(img, ptrdiff(ofsp, img));
        (*devices.offset(cur_dev as isize))
            .offset = str_to_offset(ofsp.offset(2 as libc::c_int as isize));
    }
    (*devices.offset(cur_dev as isize)).fat_bits = 0 as libc::c_int;
    (*devices.offset(cur_dev as isize)).tracks = 0 as libc::c_int as libc::c_uint;
    (*devices.offset(cur_dev as isize)).heads = 0 as libc::c_int as uint16_t;
    (*devices.offset(cur_dev as isize)).sectors = 0 as libc::c_int as uint16_t;
    if !(strchr((*devices.offset(cur_dev as isize)).name, '|' as i32)).is_null() {
        let mut pipechar: *mut libc::c_char = strchr(
            (*devices.offset(cur_dev as isize)).name,
            '|' as i32,
        );
        *pipechar = 0 as libc::c_int as libc::c_char;
        strncpy(
            buffer.as_mut_ptr(),
            pipechar.offset(1 as libc::c_int as isize),
            256 as libc::c_int as libc::c_ulong,
        );
        buffer[256 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        fp = 0 as *mut FILE;
        filename = b"{command line}\0" as *const u8 as *const libc::c_char;
        linenumber = 0 as libc::c_int;
        lastTokenLinenumber = 0 as libc::c_int;
        pos = buffer.as_mut_ptr();
        token = 0 as *mut libc::c_char;
        parse_all(0 as libc::c_int);
    }
}
pub unsafe extern "C" fn check_number_parse_errno(
    mut c: libc::c_char,
    mut oarg: *const libc::c_char,
    mut endptr: *mut libc::c_char,
) {
    if !endptr.is_null() && *endptr as libc::c_int != 0 {
        fprintf(stderr, b"Bad number %s\n\0" as *const u8 as *const libc::c_char, oarg);
        exit(1 as libc::c_int);
    }
    if *__errno_location() != 0 {
        fprintf(
            stderr,
            b"Bad number %s for -%c (%s)\n\0" as *const u8 as *const libc::c_char,
            oarg,
            c as libc::c_int,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn tou16(
    mut in_0: libc::c_int,
    mut comment: *const libc::c_char,
) -> uint16_t {
    if in_0 > 65535 as libc::c_int {
        fprintf(
            stderr,
            b"Number of %s %d too big\n\0" as *const u8 as *const libc::c_char,
            comment,
            in_0,
        );
        exit(1 as libc::c_int);
    }
    if in_0 < 0 as libc::c_int {
        fprintf(
            stderr,
            b"Number of %s %d negative\n\0" as *const u8 as *const libc::c_char,
            comment,
            in_0,
        );
        exit(1 as libc::c_int);
    }
    return in_0 as uint16_t;
}
unsafe extern "C" fn parse_old_device_line(mut drive: libc::c_char) {
    let mut name: [libc::c_char; 4096] = [0; 4096];
    let mut items: libc::c_int = 0;
    let mut offset: libc::c_long = 0;
    let mut heads: libc::c_int = 0;
    let mut sectors: libc::c_int = 0;
    let mut tracks: libc::c_int = 0;
    finish_drive_clause();
    purge(drive, file_nr);
    append();
    items = sscanf(
        token,
        b"%c %s %i %i %i %i %li\0" as *const u8 as *const libc::c_char,
        &mut (*devices.offset(cur_dev as isize)).drive as *mut libc::c_char,
        name.as_mut_ptr(),
        &mut (*devices.offset(cur_dev as isize)).fat_bits as *mut libc::c_int,
        &mut tracks as *mut libc::c_int,
        &mut heads as *mut libc::c_int,
        &mut sectors as *mut libc::c_int,
        &mut offset as *mut libc::c_long,
    );
    (*devices.offset(cur_dev as isize))
        .heads = tou16(heads, b"heads\0" as *const u8 as *const libc::c_char);
    (*devices.offset(cur_dev as isize))
        .sectors = tou16(sectors, b"sectors\0" as *const u8 as *const libc::c_char);
    (*devices.offset(cur_dev as isize)).tracks = tracks as libc::c_uint;
    (*devices.offset(cur_dev as isize)).offset = offset;
    let mut current_block_13: u64;
    match items {
        2 => {
            (*devices.offset(cur_dev as isize)).fat_bits = 0 as libc::c_int;
            current_block_13 = 16131899736838168669;
        }
        3 => {
            current_block_13 = 16131899736838168669;
        }
        6 => {
            current_block_13 = 5049571061433982978;
        }
        0 | 1 | 4 | 5 => {
            syntax(
                b"bad number of parameters\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        _ => {
            current_block_13 = 13586036798005543211;
        }
    }
    match current_block_13 {
        16131899736838168669 => {
            (*devices.offset(cur_dev as isize)).sectors = 0 as libc::c_int as uint16_t;
            (*devices.offset(cur_dev as isize)).heads = 0 as libc::c_int as uint16_t;
            (*devices.offset(cur_dev as isize))
                .tracks = 0 as libc::c_int as libc::c_uint;
            current_block_13 = 5049571061433982978;
        }
        _ => {}
    }
    match current_block_13 {
        5049571061433982978 => {
            (*devices.offset(cur_dev as isize)).offset = 0 as libc::c_int as off_t;
        }
        _ => {}
    }
    if (*devices.offset(cur_dev as isize)).tracks == 0 {
        (*devices.offset(cur_dev as isize)).sectors = 0 as libc::c_int as uint16_t;
        (*devices.offset(cur_dev as isize)).heads = 0 as libc::c_int as uint16_t;
    }
    (*devices.offset(cur_dev as isize))
        .drive = ch_canon_drv((*devices.offset(cur_dev as isize)).drive);
    maintain_default_drive((*devices.offset(cur_dev as isize)).drive);
    let ref mut fresh7 = (*devices.offset(cur_dev as isize)).name;
    *fresh7 = strdup(name.as_mut_ptr());
    if (*fresh7).is_null() {
        printOom();
        exit(1 as libc::c_int);
    }
    (*devices.offset(cur_dev as isize)).misc_flags |= 0x10 as libc::c_uint;
    finish_drive_clause();
    pos = 0 as *mut libc::c_char;
}
unsafe extern "C" fn parse_one(mut privilege: libc::c_int) -> libc::c_int {
    let mut action: libc::c_int = 0 as libc::c_int;
    get_next_token();
    if token.is_null() {
        return 0 as libc::c_int;
    }
    if match_token(b"drive\0" as *const u8 as *const libc::c_char) != 0
        && {
            action = 1 as libc::c_int;
            action != 0
        }
        || match_token(b"drive+\0" as *const u8 as *const libc::c_char) != 0
            && {
                action = 2 as libc::c_int;
                action != 0
            }
        || match_token(b"+drive\0" as *const u8 as *const libc::c_char) != 0
            && {
                action = 3 as libc::c_int;
                action != 0
            }
        || match_token(b"clear_drive\0" as *const u8 as *const libc::c_char) != 0
            && {
                action = 4 as libc::c_int;
                action != 0
            }
    {
        finish_drive_clause();
        get_next_token();
        if token_length != 1 as libc::c_int as libc::c_ulong {
            syntax(
                b"drive letter expected\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        if action == 1 as libc::c_int || action == 4 as libc::c_int {
            purge(*token.offset(0 as libc::c_int as isize), file_nr);
        }
        if action == 4 as libc::c_int {
            return 1 as libc::c_int;
        }
        if action == 3 as libc::c_int {
            prepend();
        } else {
            append();
        }
        memset(
            devices.offset(cur_dev as isize) as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<device>() as libc::c_ulong,
        );
        trusted = privilege;
        flag_mask = 0 as libc::c_int as libc::c_uint;
        (*devices.offset(cur_dev as isize))
            .drive = ch_canon_drv(*token.offset(0 as libc::c_int as isize));
        maintain_default_drive((*devices.offset(cur_dev as isize)).drive);
        expect_char(':' as i32 as libc::c_char);
        return 1 as libc::c_int;
    }
    if token_nr == 1 as libc::c_int && token_length == 1 as libc::c_int as libc::c_ulong
    {
        parse_old_device_line(*token.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int;
    }
    if (cur_dev < 0 as libc::c_int
        || set_var(
            dswitches.as_mut_ptr(),
            (::std::mem::size_of::<[switches_t; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<switches_t>() as libc::c_ulong)
                as libc::c_int,
            &mut *devices.offset(cur_dev as isize) as *mut device as caddr_t,
        ) != 0 && set_openflags(&mut *devices.offset(cur_dev as isize)) != 0
            && set_misc_flags(&mut *devices.offset(cur_dev as isize)) != 0
            && set_def_format(&mut *devices.offset(cur_dev as isize)) != 0)
        && set_var(
            global_switches.as_mut_ptr(),
            (::std::mem::size_of::<[switches_t; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<switches_t>() as libc::c_ulong)
                as libc::c_int,
            0 as caddr_t,
        ) != 0
    {
        syntax(
            b"unrecognized keyword\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse_all(mut privilege: libc::c_int) {
    *__errno_location() = 0 as libc::c_int;
    while parse_one(privilege) != 0 {}
}
unsafe extern "C" fn parse(
    mut name: *const libc::c_char,
    mut privilege: libc::c_int,
) -> libc::c_int {
    if !fp.is_null() {
        fprintf(
            stderr,
            b"File descriptor already set!\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    fp = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    file_nr += 1;
    file_nr;
    filename = name;
    linenumber = 0 as libc::c_int;
    lastTokenLinenumber = 0 as libc::c_int;
    pos = 0 as *mut libc::c_char;
    token = 0 as *mut libc::c_char;
    cur_dev = -(1 as libc::c_int);
    parse_all(privilege);
    finish_drive_clause();
    fclose(fp);
    filename = 0 as *const libc::c_char;
    fp = 0 as *mut FILE;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn read_config() {
    let mut homedir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envConfFile: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut conf_file: [libc::c_char; 4107] = [0; 4107];
    file_nr = 0 as libc::c_int;
    cur_devs = nr_const_devices;
    nr_dev = nr_const_devices.wrapping_add(2 as libc::c_int as libc::c_uint);
    devices = calloc(
        nr_dev as libc::c_ulong,
        ::std::mem::size_of::<device>() as libc::c_ulong,
    ) as *mut device;
    if devices.is_null() {
        printOom();
        exit(1 as libc::c_int);
    }
    if nr_const_devices != 0 {
        memcpy(
            devices as *mut libc::c_void,
            const_devices.as_mut_ptr() as *const libc::c_void,
            (nr_const_devices as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<device>() as libc::c_ulong),
        );
    }
    (parse(b"/etc/mtools.conf\0" as *const u8 as *const libc::c_char, 1 as libc::c_int)
        | parse(
            b"/etc/default/mtools.conf\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        )
        | parse(
            b"/usr/local/etc/mtools.conf\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        ) != 0
        || parse(b"/etc/mtools\0" as *const u8 as *const libc::c_char, 1 as libc::c_int)
            | parse(
                b"/etc/default/mtools\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            ) != 0) as libc::c_int;
    homedir = get_homedir();
    if !homedir.is_null() {
        strncpy(conf_file.as_mut_ptr(), homedir, 4096 as libc::c_int as libc::c_ulong);
        conf_file[4096 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        strcat(
            conf_file.as_mut_ptr(),
            b"/.mtoolsrc\0" as *const u8 as *const libc::c_char,
        );
        parse(conf_file.as_mut_ptr(), 0 as libc::c_int);
    }
    memset(
        &mut *devices.offset(cur_devs as isize) as *mut device as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<device>() as libc::c_ulong,
    );
    envConfFile = getenv(b"MTOOLSRC\0" as *const u8 as *const libc::c_char);
    if !envConfFile.is_null() {
        parse(envConfFile, 0 as libc::c_int);
    }
    get_env_conf();
    if mtools_skip_check != 0 {
        mtools_fat_compatibility = 1 as libc::c_int as libc::c_uint;
    }
}
pub unsafe extern "C" fn mtoolstest(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut type_0: libc::c_int,
) {
    let mut dev: *mut device = 0 as *mut device;
    let mut drive: libc::c_char = '\0' as i32 as libc::c_char;
    if argc > 1 as libc::c_int
        && *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != 0
        && *(*argv.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == ':' as i32
    {
        drive = ch_canon_drv(
            *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize),
        );
    }
    dev = devices;
    while !((*dev).name).is_null() {
        if !(drive as libc::c_int != 0
            && drive as libc::c_int != (*dev).drive as libc::c_int)
        {
            printf(
                b"drive %c:\n\0" as *const u8 as *const libc::c_char,
                (*dev).drive as libc::c_int,
            );
            printf(
                b"\t#fn=%d mode=%d \0" as *const u8 as *const libc::c_char,
                (*dev).file_nr,
                (*dev).mode,
            );
            if !((*dev).cfg_filename).is_null() {
                printf(
                    b"defined in %s\n\0" as *const u8 as *const libc::c_char,
                    (*dev).cfg_filename,
                );
            } else {
                printf(b"builtin\n\0" as *const u8 as *const libc::c_char);
            }
            printf(
                b"\tfile=\"%s\" fat_bits=%d \n\0" as *const u8 as *const libc::c_char,
                (*dev).name,
                (*dev).fat_bits,
            );
            printf(
                b"\ttracks=%d heads=%d sectors=%d hidden=%d\n\0" as *const u8
                    as *const libc::c_char,
                (*dev).tracks,
                (*dev).heads as libc::c_int,
                (*dev).sectors as libc::c_int,
                (*dev).hidden,
            );
            printf(
                b"\toffset=0x%lx\n\0" as *const u8 as *const libc::c_char,
                (*dev).offset,
            );
            printf(
                b"\tpartition=%d\n\0" as *const u8 as *const libc::c_char,
                (*dev).partition,
            );
            if (*dev).misc_flags != 0 {
                printf(b"\t\0" as *const u8 as *const libc::c_char);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x100 as libc::c_uint != 0 {
                printf(b"swap \0" as *const u8 as *const libc::c_char);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x1 as libc::c_uint != 0 {
                printf(b"scsi \0" as *const u8 as *const libc::c_char);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x2 as libc::c_uint != 0 {
                printf(b"privileged\0" as *const u8 as *const libc::c_char);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x10 as libc::c_uint != 0 {
                printf(b"mformat_only \0" as *const u8 as *const libc::c_char);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x20 as libc::c_uint != 0 {
                printf(b"vold \0" as *const u8 as *const libc::c_char);
            }
            if !dev.is_null() && (*dev).misc_flags & 0x8 as libc::c_uint != 0 {
                printf(b"use_xdf \0" as *const u8 as *const libc::c_char);
            }
            if (*dev).misc_flags != 0 {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if (*dev).mode != 0 {
                printf(b"\t\0" as *const u8 as *const libc::c_char);
            }
            if (*dev).mode & 0o4010000 as libc::c_int != 0 {
                printf(b"sync \0" as *const u8 as *const libc::c_char);
            }
            if (*dev).mode & 0o4000 as libc::c_int != 0 {
                printf(b"nodelay \0" as *const u8 as *const libc::c_char);
            }
            if (*dev).mode & 0o200 as libc::c_int != 0 {
                printf(b"exclusive \0" as *const u8 as *const libc::c_char);
            }
            if (*dev).mode != 0 {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if !((*dev).precmd).is_null() {
                printf(
                    b"\tprecmd=%s\n\0" as *const u8 as *const libc::c_char,
                    (*dev).precmd,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        dev = dev.offset(1);
        dev;
    }
    printf(
        b"mtools_fat_compatibility=%d\n\0" as *const u8 as *const libc::c_char,
        mtools_fat_compatibility,
    );
    printf(
        b"mtools_skip_check=%d\n\0" as *const u8 as *const libc::c_char,
        mtools_skip_check,
    );
    printf(
        b"mtools_lower_case=%d\n\0" as *const u8 as *const libc::c_char,
        mtools_ignore_short_case,
    );
    exit(0 as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    dswitches = [
        {
            let mut init = switches_l {
                name: b"FILE\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).name as *mut *const libc::c_char
                    as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"OFFSET\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).offset as *mut off_t as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"PARTITION\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).partition as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"FAT\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).fat_bits as *mut libc::c_int
                    as caddr_t,
                type_0: T_INT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"FAT_BITS\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).fat_bits as *mut libc::c_int
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"MODE\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).mode as *mut libc::c_int as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"TRACKS\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).tracks as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"CYLINDERS\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).tracks as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"HEADS\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).heads as *mut uint16_t as caddr_t,
                type_0: T_UINT16,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"SECTORS\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).sectors as *mut uint16_t as caddr_t,
                type_0: T_UINT16,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"HIDDEN\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).hidden as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"PRECMD\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).precmd as *mut *mut libc::c_char
                    as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"POSTCMD\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).postcmd as *mut *mut libc::c_char
                    as caddr_t,
                type_0: T_STRING,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"BLOCKSIZE\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).blocksize as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"CODEPAGE\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).codepage as *mut libc::c_uint
                    as caddr_t,
                type_0: T_UINT,
            };
            init
        },
        {
            let mut init = switches_l {
                name: b"DATA_MAP\0" as *const u8 as *const libc::c_char,
                address: &mut (*(0 as *mut device)).data_map as *mut *const libc::c_char
                    as caddr_t,
                type_0: T_UQSTRING,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
