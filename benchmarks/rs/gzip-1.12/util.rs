use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn rpl_fprintf(fp: *mut FILE, format: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: libc::c_uint;
    static mut inptr: libc::c_uint;
    static mut outcnt: libc::c_uint;
    static mut bytes_in: off_t;
    static mut bytes_out: off_t;
    static mut ifd: libc::c_int;
    static mut ofd: libc::c_int;
    static mut ifname: [libc::c_char; 0];
    static mut ofname: [libc::c_char; 0];
    static mut program_name: *mut libc::c_char;
    static mut exit_code: libc::c_int;
    static mut quiet: libc::c_int;
    static mut test: libc::c_int;
    fn abort_gzip();
    fn perror(__s: *const libc::c_char);
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type voidp = *mut libc::c_void;
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
pub type uch = libc::c_uchar;
pub type ulg = libc::c_ulong;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut crc_32_tab: [ulg; 256] = [
    0 as libc::c_long as ulg,
    0x77073096 as libc::c_long as ulg,
    0xee0e612c as libc::c_long as ulg,
    0x990951ba as libc::c_long as ulg,
    0x76dc419 as libc::c_long as ulg,
    0x706af48f as libc::c_long as ulg,
    0xe963a535 as libc::c_long as ulg,
    0x9e6495a3 as libc::c_long as ulg,
    0xedb8832 as libc::c_long as ulg,
    0x79dcb8a4 as libc::c_long as ulg,
    0xe0d5e91e as libc::c_long as ulg,
    0x97d2d988 as libc::c_long as ulg,
    0x9b64c2b as libc::c_long as ulg,
    0x7eb17cbd as libc::c_long as ulg,
    0xe7b82d07 as libc::c_long as ulg,
    0x90bf1d91 as libc::c_long as ulg,
    0x1db71064 as libc::c_long as ulg,
    0x6ab020f2 as libc::c_long as ulg,
    0xf3b97148 as libc::c_long as ulg,
    0x84be41de as libc::c_long as ulg,
    0x1adad47d as libc::c_long as ulg,
    0x6ddde4eb as libc::c_long as ulg,
    0xf4d4b551 as libc::c_long as ulg,
    0x83d385c7 as libc::c_long as ulg,
    0x136c9856 as libc::c_long as ulg,
    0x646ba8c0 as libc::c_long as ulg,
    0xfd62f97a as libc::c_long as ulg,
    0x8a65c9ec as libc::c_long as ulg,
    0x14015c4f as libc::c_long as ulg,
    0x63066cd9 as libc::c_long as ulg,
    0xfa0f3d63 as libc::c_long as ulg,
    0x8d080df5 as libc::c_long as ulg,
    0x3b6e20c8 as libc::c_long as ulg,
    0x4c69105e as libc::c_long as ulg,
    0xd56041e4 as libc::c_long as ulg,
    0xa2677172 as libc::c_long as ulg,
    0x3c03e4d1 as libc::c_long as ulg,
    0x4b04d447 as libc::c_long as ulg,
    0xd20d85fd as libc::c_long as ulg,
    0xa50ab56b as libc::c_long as ulg,
    0x35b5a8fa as libc::c_long as ulg,
    0x42b2986c as libc::c_long as ulg,
    0xdbbbc9d6 as libc::c_long as ulg,
    0xacbcf940 as libc::c_long as ulg,
    0x32d86ce3 as libc::c_long as ulg,
    0x45df5c75 as libc::c_long as ulg,
    0xdcd60dcf as libc::c_long as ulg,
    0xabd13d59 as libc::c_long as ulg,
    0x26d930ac as libc::c_long as ulg,
    0x51de003a as libc::c_long as ulg,
    0xc8d75180 as libc::c_long as ulg,
    0xbfd06116 as libc::c_long as ulg,
    0x21b4f4b5 as libc::c_long as ulg,
    0x56b3c423 as libc::c_long as ulg,
    0xcfba9599 as libc::c_long as ulg,
    0xb8bda50f as libc::c_long as ulg,
    0x2802b89e as libc::c_long as ulg,
    0x5f058808 as libc::c_long as ulg,
    0xc60cd9b2 as libc::c_long as ulg,
    0xb10be924 as libc::c_long as ulg,
    0x2f6f7c87 as libc::c_long as ulg,
    0x58684c11 as libc::c_long as ulg,
    0xc1611dab as libc::c_long as ulg,
    0xb6662d3d as libc::c_long as ulg,
    0x76dc4190 as libc::c_long as ulg,
    0x1db7106 as libc::c_long as ulg,
    0x98d220bc as libc::c_long as ulg,
    0xefd5102a as libc::c_long as ulg,
    0x71b18589 as libc::c_long as ulg,
    0x6b6b51f as libc::c_long as ulg,
    0x9fbfe4a5 as libc::c_long as ulg,
    0xe8b8d433 as libc::c_long as ulg,
    0x7807c9a2 as libc::c_long as ulg,
    0xf00f934 as libc::c_long as ulg,
    0x9609a88e as libc::c_long as ulg,
    0xe10e9818 as libc::c_long as ulg,
    0x7f6a0dbb as libc::c_long as ulg,
    0x86d3d2d as libc::c_long as ulg,
    0x91646c97 as libc::c_long as ulg,
    0xe6635c01 as libc::c_long as ulg,
    0x6b6b51f4 as libc::c_long as ulg,
    0x1c6c6162 as libc::c_long as ulg,
    0x856530d8 as libc::c_long as ulg,
    0xf262004e as libc::c_long as ulg,
    0x6c0695ed as libc::c_long as ulg,
    0x1b01a57b as libc::c_long as ulg,
    0x8208f4c1 as libc::c_long as ulg,
    0xf50fc457 as libc::c_long as ulg,
    0x65b0d9c6 as libc::c_long as ulg,
    0x12b7e950 as libc::c_long as ulg,
    0x8bbeb8ea as libc::c_long as ulg,
    0xfcb9887c as libc::c_long as ulg,
    0x62dd1ddf as libc::c_long as ulg,
    0x15da2d49 as libc::c_long as ulg,
    0x8cd37cf3 as libc::c_long as ulg,
    0xfbd44c65 as libc::c_long as ulg,
    0x4db26158 as libc::c_long as ulg,
    0x3ab551ce as libc::c_long as ulg,
    0xa3bc0074 as libc::c_long as ulg,
    0xd4bb30e2 as libc::c_long as ulg,
    0x4adfa541 as libc::c_long as ulg,
    0x3dd895d7 as libc::c_long as ulg,
    0xa4d1c46d as libc::c_long as ulg,
    0xd3d6f4fb as libc::c_long as ulg,
    0x4369e96a as libc::c_long as ulg,
    0x346ed9fc as libc::c_long as ulg,
    0xad678846 as libc::c_long as ulg,
    0xda60b8d0 as libc::c_long as ulg,
    0x44042d73 as libc::c_long as ulg,
    0x33031de5 as libc::c_long as ulg,
    0xaa0a4c5f as libc::c_long as ulg,
    0xdd0d7cc9 as libc::c_long as ulg,
    0x5005713c as libc::c_long as ulg,
    0x270241aa as libc::c_long as ulg,
    0xbe0b1010 as libc::c_long as ulg,
    0xc90c2086 as libc::c_long as ulg,
    0x5768b525 as libc::c_long as ulg,
    0x206f85b3 as libc::c_long as ulg,
    0xb966d409 as libc::c_long as ulg,
    0xce61e49f as libc::c_long as ulg,
    0x5edef90e as libc::c_long as ulg,
    0x29d9c998 as libc::c_long as ulg,
    0xb0d09822 as libc::c_long as ulg,
    0xc7d7a8b4 as libc::c_long as ulg,
    0x59b33d17 as libc::c_long as ulg,
    0x2eb40d81 as libc::c_long as ulg,
    0xb7bd5c3b as libc::c_long as ulg,
    0xc0ba6cad as libc::c_long as ulg,
    0xedb88320 as libc::c_long as ulg,
    0x9abfb3b6 as libc::c_long as ulg,
    0x3b6e20c as libc::c_long as ulg,
    0x74b1d29a as libc::c_long as ulg,
    0xead54739 as libc::c_long as ulg,
    0x9dd277af as libc::c_long as ulg,
    0x4db2615 as libc::c_long as ulg,
    0x73dc1683 as libc::c_long as ulg,
    0xe3630b12 as libc::c_long as ulg,
    0x94643b84 as libc::c_long as ulg,
    0xd6d6a3e as libc::c_long as ulg,
    0x7a6a5aa8 as libc::c_long as ulg,
    0xe40ecf0b as libc::c_long as ulg,
    0x9309ff9d as libc::c_long as ulg,
    0xa00ae27 as libc::c_long as ulg,
    0x7d079eb1 as libc::c_long as ulg,
    0xf00f9344 as libc::c_long as ulg,
    0x8708a3d2 as libc::c_long as ulg,
    0x1e01f268 as libc::c_long as ulg,
    0x6906c2fe as libc::c_long as ulg,
    0xf762575d as libc::c_long as ulg,
    0x806567cb as libc::c_long as ulg,
    0x196c3671 as libc::c_long as ulg,
    0x6e6b06e7 as libc::c_long as ulg,
    0xfed41b76 as libc::c_long as ulg,
    0x89d32be0 as libc::c_long as ulg,
    0x10da7a5a as libc::c_long as ulg,
    0x67dd4acc as libc::c_long as ulg,
    0xf9b9df6f as libc::c_long as ulg,
    0x8ebeeff9 as libc::c_long as ulg,
    0x17b7be43 as libc::c_long as ulg,
    0x60b08ed5 as libc::c_long as ulg,
    0xd6d6a3e8 as libc::c_long as ulg,
    0xa1d1937e as libc::c_long as ulg,
    0x38d8c2c4 as libc::c_long as ulg,
    0x4fdff252 as libc::c_long as ulg,
    0xd1bb67f1 as libc::c_long as ulg,
    0xa6bc5767 as libc::c_long as ulg,
    0x3fb506dd as libc::c_long as ulg,
    0x48b2364b as libc::c_long as ulg,
    0xd80d2bda as libc::c_long as ulg,
    0xaf0a1b4c as libc::c_long as ulg,
    0x36034af6 as libc::c_long as ulg,
    0x41047a60 as libc::c_long as ulg,
    0xdf60efc3 as libc::c_long as ulg,
    0xa867df55 as libc::c_long as ulg,
    0x316e8eef as libc::c_long as ulg,
    0x4669be79 as libc::c_long as ulg,
    0xcb61b38c as libc::c_long as ulg,
    0xbc66831a as libc::c_long as ulg,
    0x256fd2a0 as libc::c_long as ulg,
    0x5268e236 as libc::c_long as ulg,
    0xcc0c7795 as libc::c_long as ulg,
    0xbb0b4703 as libc::c_long as ulg,
    0x220216b9 as libc::c_long as ulg,
    0x5505262f as libc::c_long as ulg,
    0xc5ba3bbe as libc::c_long as ulg,
    0xb2bd0b28 as libc::c_long as ulg,
    0x2bb45a92 as libc::c_long as ulg,
    0x5cb36a04 as libc::c_long as ulg,
    0xc2d7ffa7 as libc::c_long as ulg,
    0xb5d0cf31 as libc::c_long as ulg,
    0x2cd99e8b as libc::c_long as ulg,
    0x5bdeae1d as libc::c_long as ulg,
    0x9b64c2b0 as libc::c_long as ulg,
    0xec63f226 as libc::c_long as ulg,
    0x756aa39c as libc::c_long as ulg,
    0x26d930a as libc::c_long as ulg,
    0x9c0906a9 as libc::c_long as ulg,
    0xeb0e363f as libc::c_long as ulg,
    0x72076785 as libc::c_long as ulg,
    0x5005713 as libc::c_long as ulg,
    0x95bf4a82 as libc::c_long as ulg,
    0xe2b87a14 as libc::c_long as ulg,
    0x7bb12bae as libc::c_long as ulg,
    0xcb61b38 as libc::c_long as ulg,
    0x92d28e9b as libc::c_long as ulg,
    0xe5d5be0d as libc::c_long as ulg,
    0x7cdcefb7 as libc::c_long as ulg,
    0xbdbdf21 as libc::c_long as ulg,
    0x86d3d2d4 as libc::c_long as ulg,
    0xf1d4e242 as libc::c_long as ulg,
    0x68ddb3f8 as libc::c_long as ulg,
    0x1fda836e as libc::c_long as ulg,
    0x81be16cd as libc::c_long as ulg,
    0xf6b9265b as libc::c_long as ulg,
    0x6fb077e1 as libc::c_long as ulg,
    0x18b74777 as libc::c_long as ulg,
    0x88085ae6 as libc::c_long as ulg,
    0xff0f6a70 as libc::c_long as ulg,
    0x66063bca as libc::c_long as ulg,
    0x11010b5c as libc::c_long as ulg,
    0x8f659eff as libc::c_long as ulg,
    0xf862ae69 as libc::c_long as ulg,
    0x616bffd3 as libc::c_long as ulg,
    0x166ccf45 as libc::c_long as ulg,
    0xa00ae278 as libc::c_long as ulg,
    0xd70dd2ee as libc::c_long as ulg,
    0x4e048354 as libc::c_long as ulg,
    0x3903b3c2 as libc::c_long as ulg,
    0xa7672661 as libc::c_long as ulg,
    0xd06016f7 as libc::c_long as ulg,
    0x4969474d as libc::c_long as ulg,
    0x3e6e77db as libc::c_long as ulg,
    0xaed16a4a as libc::c_long as ulg,
    0xd9d65adc as libc::c_long as ulg,
    0x40df0b66 as libc::c_long as ulg,
    0x37d83bf0 as libc::c_long as ulg,
    0xa9bcae53 as libc::c_long as ulg,
    0xdebb9ec5 as libc::c_long as ulg,
    0x47b2cf7f as libc::c_long as ulg,
    0x30b5ffe9 as libc::c_long as ulg,
    0xbdbdf21c as libc::c_long as ulg,
    0xcabac28a as libc::c_long as ulg,
    0x53b39330 as libc::c_long as ulg,
    0x24b4a3a6 as libc::c_long as ulg,
    0xbad03605 as libc::c_long as ulg,
    0xcdd70693 as libc::c_long as ulg,
    0x54de5729 as libc::c_long as ulg,
    0x23d967bf as libc::c_long as ulg,
    0xb3667a2e as libc::c_long as ulg,
    0xc4614ab8 as libc::c_long as ulg,
    0x5d681b02 as libc::c_long as ulg,
    0x2a6f2b94 as libc::c_long as ulg,
    0xb40bbe37 as libc::c_long as ulg,
    0xc30c8ea1 as libc::c_long as ulg,
    0x5a05df1b as libc::c_long as ulg,
    0x2d02ef8d as libc::c_long as ulg,
];
static mut crc: ulg = 0xffffffff as libc::c_long as ulg;
pub unsafe extern "C" fn copy(
    mut in_0: libc::c_int,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut got: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    while insize > inptr {
        write_buf(
            out,
            (inbuf.as_mut_ptr() as *mut libc::c_char).offset(inptr as isize) as voidp,
            insize.wrapping_sub(inptr),
        );
        got = read_buffer(
            in_0,
            inbuf.as_mut_ptr() as *mut libc::c_char as voidp,
            0x40000 as libc::c_int as libc::c_uint,
        );
        if got == -(1 as libc::c_int) {
            read_error();
        }
        bytes_in += got as libc::c_long;
        insize = got as libc::c_uint;
        inptr = 0 as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn updcrc(mut s: *const uch, mut n: libc::c_uint) -> ulg {
    let mut c: ulg = 0;
    if s.is_null() {
        c = 0xffffffff as libc::c_long as ulg;
    } else {
        c = crc;
        if n != 0 {
            loop {
                let fresh0 = s;
                s = s.offset(1);
                c = crc_32_tab[((c as libc::c_int ^ *fresh0 as libc::c_int)
                    & 0xff as libc::c_int) as usize] ^ c >> 8 as libc::c_int;
                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
        }
    }
    crc = c;
    return c ^ 0xffffffff as libc::c_long as libc::c_ulong;
}
pub unsafe extern "C" fn getcrc() -> ulg {
    return crc ^ 0xffffffff as libc::c_long as libc::c_ulong;
}
pub unsafe extern "C" fn clear_bufs() {
    outcnt = 0 as libc::c_int as libc::c_uint;
    inptr = 0 as libc::c_int as libc::c_uint;
    insize = inptr;
    bytes_out = 0 as libc::c_long;
    bytes_in = bytes_out;
}
pub unsafe extern "C" fn fill_inbuf(mut eof_ok: libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = 0;
    insize = 0 as libc::c_int as libc::c_uint;
    loop {
        len = read_buffer(
            ifd,
            (inbuf.as_mut_ptr() as *mut libc::c_char).offset(insize as isize) as voidp,
            (0x40000 as libc::c_int as libc::c_uint).wrapping_sub(insize),
        );
        if len == 0 as libc::c_int {
            break;
        }
        if len == -(1 as libc::c_int) {
            read_error();
            break;
        } else {
            insize = insize.wrapping_add(len as libc::c_uint);
            if !(insize < 0x40000 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    if insize == 0 as libc::c_int as libc::c_uint {
        if eof_ok != 0 {
            return -(1 as libc::c_int);
        }
        flush_window();
        *__errno_location() = 0 as libc::c_int;
        read_error();
    }
    bytes_in += insize as off_t;
    inptr = 1 as libc::c_int as libc::c_uint;
    return *inbuf.as_mut_ptr().offset(0 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn read_buffer(
    mut fd: libc::c_int,
    mut buf: voidp,
    mut cnt: libc::c_uint,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    if (2147483647 as libc::c_int as libc::c_uint) < cnt {
        cnt = 2147483647 as libc::c_int as libc::c_uint;
    }
    len = read(fd, buf, cnt as size_t) as libc::c_int;
    if len < 0 as libc::c_int && *__errno_location() == 11 as libc::c_int {
        let mut flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
        if 0 as libc::c_int <= flags {
            if flags & 0o4000 as libc::c_int == 0 {
                *__errno_location() = 11 as libc::c_int;
            } else if rpl_fcntl(fd, 4 as libc::c_int, flags & !(0o4000 as libc::c_int))
                != -(1 as libc::c_int)
            {
                len = read(fd, buf, cnt as size_t) as libc::c_int;
            }
        }
    }
    return len;
}
unsafe extern "C" fn write_buffer(
    mut fd: libc::c_int,
    mut buf: voidp,
    mut cnt: libc::c_uint,
) -> libc::c_int {
    if (2147483647 as libc::c_int as libc::c_uint) < cnt {
        cnt = 2147483647 as libc::c_int as libc::c_uint;
    }
    return write(fd, buf as *const libc::c_void, cnt as size_t) as libc::c_int;
}
pub unsafe extern "C" fn flush_outbuf() {
    if outcnt == 0 as libc::c_int as libc::c_uint {
        return;
    }
    write_buf(ofd, outbuf.as_mut_ptr() as voidp, outcnt);
    outcnt = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn flush_window() {
    if outcnt == 0 as libc::c_int as libc::c_uint {
        return;
    }
    updcrc(window.as_mut_ptr(), outcnt);
    write_buf(ofd, window.as_mut_ptr() as voidp, outcnt);
    outcnt = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn write_buf(
    mut fd: libc::c_int,
    mut buf: voidp,
    mut cnt: libc::c_uint,
) {
    let mut n: libc::c_uint = 0;
    bytes_out += cnt as libc::c_long;
    if test != 0 {
        return;
    }
    loop {
        n = write_buffer(fd, buf, cnt) as libc::c_uint;
        if !(n != cnt) {
            break;
        }
        if n == -(1 as libc::c_int) as libc::c_uint {
            write_error();
        }
        cnt = cnt.wrapping_sub(n);
        buf = (buf as *mut libc::c_char).offset(n as isize) as voidp;
    };
}
pub unsafe extern "C" fn strlwr(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    t = s;
    while *t != 0 {
        *t = (if *(*__ctype_b_loc()).offset(*t as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *t as libc::c_uchar as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(*t as libc::c_uchar as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*t as libc::c_uchar as libc::c_int as isize);
                }
                __res
            })
        } else {
            *t as libc::c_uchar as libc::c_int
        }) as libc::c_char;
        t = t.offset(1);
        t;
    }
    return s;
}
pub unsafe extern "C" fn gzip_base_name(
    mut fname: *mut libc::c_char,
) -> *mut libc::c_char {
    fname = last_component(fname);
    if 'A' as i32 == 'a' as i32 {
        strlwr(fname);
    }
    return fname;
}
pub unsafe extern "C" fn xunlink(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut r: libc::c_int = unlink(filename);
    return r;
}
pub unsafe extern "C" fn add_envopt(
    mut argcp: *mut libc::c_int,
    mut argvp: *mut *mut *mut libc::c_char,
    mut envvar_name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nargc: libc::c_int = 0 as libc::c_int;
    let mut env_val: *mut libc::c_char = 0 as *mut libc::c_char;
    env_val = getenv(envvar_name);
    if env_val.is_null() {
        return 0 as *mut libc::c_char;
    }
    env_val = xstrdup(env_val);
    p = env_val;
    while *p != 0 {
        p = p.offset(strspn(p, b" \t\0" as *const u8 as *const libc::c_char) as isize);
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        p = p.offset(strcspn(p, b" \t\0" as *const u8 as *const libc::c_char) as isize);
        if *p != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
        }
        nargc += 1;
        nargc;
    }
    if nargc == 0 as libc::c_int {
        rpl_free(env_val as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *argcp = nargc + 1 as libc::c_int;
    nargv = xcalloc(
        (*argcp + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    oargv = *argvp;
    *argvp = nargv;
    let fresh2 = nargv;
    nargv = nargv.offset(1);
    *fresh2 = *oargv;
    p = env_val;
    while nargc > 0 as libc::c_int {
        p = p.offset(strspn(p, b" \t\0" as *const u8 as *const libc::c_char) as isize);
        let fresh3 = nargv;
        nargv = nargv.offset(1);
        *fresh3 = p;
        loop {
            let fresh4 = p;
            p = p.offset(1);
            if !(*fresh4 != 0) {
                break;
            }
        }
        nargc -= 1;
        nargc;
    }
    *nargv = 0 as *mut libc::c_char;
    return env_val;
}
pub unsafe extern "C" fn gzip_error(mut m: *const libc::c_char) {
    rpl_fprintf(
        stderr,
        b"\n%s: %s: %s\n\0" as *const u8 as *const libc::c_char,
        program_name,
        ifname.as_mut_ptr(),
        m,
    );
    abort_gzip();
}
pub unsafe extern "C" fn xalloc_die() {
    rpl_fprintf(
        stderr,
        b"\n%s: memory_exhausted\n\0" as *const u8 as *const libc::c_char,
        program_name,
    );
    abort_gzip();
}
pub unsafe extern "C" fn warning(mut m: *const libc::c_char) {
    if quiet == 0 {
        rpl_fprintf(
            stderr,
            b"%s: %s: warning: %s\n\0" as *const u8 as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
            m,
        );
    }
    if exit_code == 0 as libc::c_int {
        exit_code = 2 as libc::c_int;
    }
}
pub unsafe extern "C" fn read_error() {
    let mut e: libc::c_int = *__errno_location();
    rpl_fprintf(stderr, b"\n%s: \0" as *const u8 as *const libc::c_char, program_name);
    if e != 0 as libc::c_int {
        *__errno_location() = e;
        perror(ifname.as_mut_ptr());
    } else {
        rpl_fprintf(
            stderr,
            b"%s: unexpected end of file\n\0" as *const u8 as *const libc::c_char,
            ifname.as_mut_ptr(),
        );
    }
    abort_gzip();
}
pub unsafe extern "C" fn write_error() {
    let mut e: libc::c_int = *__errno_location();
    rpl_fprintf(stderr, b"\n%s: \0" as *const u8 as *const libc::c_char, program_name);
    *__errno_location() = e;
    perror(ofname.as_mut_ptr());
    abort_gzip();
}
pub unsafe extern "C" fn display_ratio(
    mut num: off_t,
    mut den: off_t,
    mut file: *mut FILE,
) {
    rpl_fprintf(
        file,
        b"%5.1f%%\0" as *const u8 as *const libc::c_char,
        if den == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int as libc::c_double
        } else {
            100.0f64 * num as libc::c_double / den as libc::c_double
        },
    );
}
pub unsafe extern "C" fn fprint_off(
    mut file: *mut FILE,
    mut offset: off_t,
    mut width: libc::c_int,
) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut p: *mut libc::c_char = buf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as isize);
    if offset < 0 as libc::c_int as libc::c_long {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as libc::c_long
                - offset % 10 as libc::c_int as libc::c_long) as libc::c_char;
            offset /= 10 as libc::c_int as libc::c_long;
            if !(offset != 0 as libc::c_int as libc::c_long) {
                break;
            }
        }
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char;
    } else {
        loop {
            p = p.offset(-1);
            *p = ('0' as i32 as libc::c_long
                + offset % 10 as libc::c_int as libc::c_long) as libc::c_char;
            offset /= 10 as libc::c_int as libc::c_long;
            if !(offset != 0 as libc::c_int as libc::c_long) {
                break;
            }
        }
    }
    width = (width as libc::c_long
        - buf
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as isize,
            )
            .offset_from(p) as libc::c_long) as libc::c_int;
    loop {
        let fresh5 = width;
        width = width - 1;
        if !((0 as libc::c_int) < fresh5) {
            break;
        }
        putc(' ' as i32, file);
    }
    while p
        < buf
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as isize,
            )
    {
        putc(*p as libc::c_int, file);
        p = p.offset(1);
        p;
    }
}
