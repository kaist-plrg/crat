use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlULong = uintptr_t;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub __domainname: [libc::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileInf {
    pub name: *mut libc::c_char,
    pub suffix: *mut libc::c_char,
    pub file_part: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub work_name1: *mut libc::c_char,
    pub work_name2: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmdInf {
    pub exe_name: *mut libc::c_char,
    pub opt: [libc::c_char; 8192],
    pub out_opt: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
static mut pl_escape_symbol: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"abfnrtv\0")
};
static mut pl_escape_char: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<
        &[u8; 8],
        &mut [libc::c_char; 8],
    >(b"\x07\x08\x0C\n\r\t\x0B\0")
};
pub unsafe extern "C" fn Encode_Hexa(
    mut module: *mut libc::c_char,
    mut pred: *mut libc::c_char,
    mut arity: libc::c_int,
    mut str: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut module_encode: libc::c_int = if module.is_null()
        || *module as libc::c_int == '\0' as i32
    {
        0 as libc::c_int
    } else {
        String_Needs_Encoding(module) + 1 as libc::c_int
    };
    let mut pred_encode: libc::c_int = String_Needs_Encoding(pred);
    let fresh0 = str;
    str = str.offset(1);
    *fresh0 = 'X' as i32 as libc::c_char;
    let fresh1 = str;
    str = str.offset(1);
    *fresh1 = ('0' as i32 + (module_encode << 1 as libc::c_int | pred_encode))
        as libc::c_char;
    let fresh2 = str;
    str = str.offset(1);
    *fresh2 = '_' as i32 as libc::c_char;
    if module_encode == 1 as libc::c_int {
        str = str
            .offset(
                sprintf(str, b"%s__\0" as *const u8 as *const libc::c_char, module)
                    as isize,
            );
    } else if module_encode == 2 as libc::c_int {
        str = Encode_String(module, str);
        let fresh3 = str;
        str = str.offset(1);
        *fresh3 = '_' as i32 as libc::c_char;
        let fresh4 = str;
        str = str.offset(1);
        *fresh4 = '_' as i32 as libc::c_char;
    }
    if pred_encode == 0 as libc::c_int {
        str = str
            .offset(
                sprintf(str, b"%s\0" as *const u8 as *const libc::c_char, pred) as isize,
            );
    } else {
        str = Encode_String(pred, str);
    }
    if arity >= 0 as libc::c_int {
        str = str
            .offset(
                sprintf(str, b"__a%d\0" as *const u8 as *const libc::c_char, arity)
                    as isize,
            );
    }
    return str;
}
pub unsafe extern "C" fn Encode_Hexa_Line(
    mut str: *mut libc::c_char,
    mut format: *mut libc::c_char,
    mut strict: libc::c_int,
) -> *mut libc::c_char {
    static mut result: [libc::c_char; 4096] = [0; 4096];
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut module: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pred: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut free_in_buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arity: libc::c_int = 0;
    if format.is_null() {
        format = b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    src = str;
    dst = result.as_mut_ptr();
    loop {
        while *(*__ctype_b_loc()).offset(*src as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let fresh5 = src;
            src = src.offset(1);
            let fresh6 = dst;
            dst = dst.offset(1);
            *fresh6 = *fresh5;
        }
        if *src as libc::c_int == '\0' as i32 {
            break;
        }
        p = Parse_Atom(src, buff.as_mut_ptr());
        if p.is_null() {
            let fresh7 = src;
            src = src.offset(1);
            let fresh8 = dst;
            dst = dst.offset(1);
            *fresh8 = *fresh7;
        } else {
            if *p as libc::c_int == ':' as i32 {
                module = buff.as_mut_ptr();
                pred = buff
                    .as_mut_ptr()
                    .offset(strlen(module) as isize)
                    .offset(1 as libc::c_int as isize);
                q = p.offset(1 as libc::c_int as isize);
                p = Parse_Atom(q, pred);
                if p.is_null() {
                    while src < q {
                        let fresh9 = src;
                        src = src.offset(1);
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = *fresh9;
                    }
                    continue;
                }
            } else {
                module = 0 as *mut libc::c_char;
                pred = buff.as_mut_ptr();
            }
            arity = strtol(
                p.offset(1 as libc::c_int as isize),
                &mut q,
                10 as libc::c_int,
            ) as libc::c_int;
            if *pred as libc::c_int == '\0' as i32
                || *p as libc::c_int != '/' as i32 && strict != 0
                || *p as libc::c_int == '/' as i32
                    && (arity < 0 as libc::c_int || arity > 1024 as libc::c_int
                        || *(*__ctype_b_loc()).offset(*q as libc::c_int as isize)
                            as libc::c_int
                            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                            != 0 || *q as libc::c_int == '_' as i32)
            {
                while src < q {
                    let fresh11 = src;
                    src = src.offset(1);
                    let fresh12 = dst;
                    dst = dst.offset(1);
                    *fresh12 = *fresh11;
                }
            } else {
                if *p as libc::c_int != '/' as i32 {
                    arity = -(1 as libc::c_int);
                    src = p;
                } else {
                    src = q;
                }
                free_in_buff = pred
                    .offset(strlen(pred) as isize)
                    .offset(1 as libc::c_int as isize);
                Encode_Hexa(module, pred, arity, free_in_buff);
                dst = dst.offset(sprintf(dst, format, free_in_buff) as isize);
            }
        }
    }
    let fresh13 = dst;
    dst = dst.offset(1);
    *fresh13 = '\0' as i32 as libc::c_char;
    return result.as_mut_ptr();
}
unsafe extern "C" fn String_Needs_Encoding(mut str: *mut libc::c_char) -> libc::c_int {
    if *str as libc::c_int == '\0' as i32
        || !(*str as libc::c_int >= 'a' as i32 && *str as libc::c_int <= 'z' as i32
            || *str as libc::c_int >= 'A' as i32 && *str as libc::c_int <= 'Z' as i32
            || *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        return 1 as libc::c_int;
    }
    loop {
        str = str.offset(1);
        if !(*str as libc::c_int != '\0' as i32) {
            break;
        }
        if *str as libc::c_int == '_' as i32 {
            if *str.offset(-(1 as libc::c_int) as isize) as libc::c_int == '_' as i32
                || *str.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                return 1 as libc::c_int;
            }
        } else if !(*str as libc::c_int >= 'a' as i32
            && *str as libc::c_int <= 'z' as i32
            || *str as libc::c_int >= 'A' as i32 && *str as libc::c_int <= 'Z' as i32
            || *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Encode_String(
    mut str: *mut libc::c_char,
    mut buff: *mut libc::c_char,
) -> *mut libc::c_char {
    while *str != 0 {
        sprintf(
            buff,
            b"%02X\0" as *const u8 as *const libc::c_char,
            *str as libc::c_uchar as libc::c_uint,
        );
        str = str.offset(1);
        str;
        buff = buff.offset(2 as libc::c_int as isize);
    }
    return buff;
}
pub unsafe extern "C" fn Decode_Hexa(
    mut str: *mut libc::c_char,
    mut strict: libc::c_int,
    mut quote: libc::c_int,
    mut decode_aux: libc::c_int,
    mut module: *mut libc::c_char,
    mut pred: *mut libc::c_char,
    mut arity: *mut libc::c_int,
    mut aux_no: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut module_encode: libc::c_int = 0;
    let mut pred_encode: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let fresh14 = str;
    str = str.offset(1);
    if *fresh14 as libc::c_int != 'X' as i32 || (*str as libc::c_int) < '0' as i32
        || *str as libc::c_int >= '5' as i32
    {
        return 0 as *mut libc::c_char;
    }
    let fresh15 = str;
    str = str.offset(1);
    n = *fresh15 as libc::c_int - '0' as i32;
    module_encode = n >> 1 as libc::c_int;
    pred_encode = n & 1 as libc::c_int;
    let fresh16 = str;
    str = str.offset(1);
    if *fresh16 as libc::c_int != '_' as i32 {
        return 0 as *mut libc::c_char;
    }
    if module_encode == 0 as libc::c_int {
        *module = '\0' as i32 as libc::c_char;
    } else {
        str = if module_encode == 1 as libc::c_int {
            Copy_Not_Encoded_String(str, module)
        } else {
            Decode_String(str, module)
        };
        if str.is_null()
            || {
                let fresh17 = str;
                str = str.offset(1);
                *fresh17 as libc::c_int != '_' as i32
            }
            || {
                let fresh18 = str;
                str = str.offset(1);
                *fresh18 as libc::c_int != '_' as i32
            }
        {
            return 0 as *mut libc::c_char;
        }
        if quote != 0 {
            Quote_If_Needed(module);
        }
    }
    str = if pred_encode == 0 as libc::c_int {
        Copy_Not_Encoded_String(str, pred)
    } else {
        Decode_String(str, pred)
    };
    if str.is_null() || *pred as libc::c_int == '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    *arity = -(1 as libc::c_int);
    if *str as libc::c_int == '_' as i32
        && *str.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *str.offset(2 as libc::c_int as isize) as libc::c_int == 'a' as i32
    {
        *arity = strtoul(
            str.offset(3 as libc::c_int as isize),
            &mut p,
            10 as libc::c_int,
        ) as libc::c_int;
        if p == str.offset(3 as libc::c_int as isize) {
            *arity = -(1 as libc::c_int);
        }
        str = p;
    }
    if *arity < 0 as libc::c_int || *arity > 1024 as libc::c_int
        || *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *p as libc::c_int == '_' as i32
    {
        if strict != 0 {
            return 0 as *mut libc::c_char;
        }
        *arity = -(1 as libc::c_int);
    }
    *aux_no = -(1 as libc::c_int);
    if decode_aux != 0 && *pred as libc::c_int == '$' as i32
        && {
            p = strstr(pred, b"_$aux\0" as *const u8 as *const libc::c_char);
            !p.is_null()
        }
    {
        n = strtol(
            p
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize,
                )
                .offset(-(1 as libc::c_int as isize)),
            &mut q,
            10 as libc::c_int,
        ) as libc::c_int;
        if *q as libc::c_int == '\0' as i32 {
            loop {
                p = p.offset(-1);
                if !(p > pred
                    && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
                {
                    break;
                }
            }
            if p >= pred && *p as libc::c_int == '/' as i32 {
                *aux_no = n;
                *p = '\0' as i32 as libc::c_char;
                *arity = strtol(
                    p.offset(1 as libc::c_int as isize),
                    &mut p,
                    10 as libc::c_int,
                ) as libc::c_int;
                p = pred;
                loop {
                    *p = *p.offset(1 as libc::c_int as isize);
                    let fresh19 = p;
                    p = p.offset(1);
                    if !(*fresh19 != 0) {
                        break;
                    }
                }
            }
        }
    }
    if quote != 0 {
        Quote_If_Needed(pred);
    }
    return str;
}
pub unsafe extern "C" fn Decode_Hexa_Line(
    mut str: *mut libc::c_char,
    mut format: *mut libc::c_char,
    mut strict: libc::c_int,
    mut quote: libc::c_int,
    mut decode_aux: libc::c_int,
) -> *mut libc::c_char {
    static mut result: [libc::c_char; 4096] = [0; 4096];
    static mut module: [libc::c_char; 2048] = [0; 2048];
    static mut pred: [libc::c_char; 1024] = [0; 1024];
    let mut arity: libc::c_int = 0;
    let mut aux_no: libc::c_int = 0;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    if format.is_null() {
        format = b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    src = str;
    dst = result.as_mut_ptr();
    while *src != 0 {
        p = src;
        if (*src as libc::c_int == 'X' as i32
            || *src as libc::c_int == '_' as i32
                && {
                    p = src.offset(1 as libc::c_int as isize);
                    *p as libc::c_int == 'X' as i32
                })
            && (src == str
                || *(*__ctype_b_loc())
                    .offset(
                        *src.offset(-(1 as libc::c_int) as isize) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0)
            && {
                p = Decode_Hexa(
                    p,
                    strict,
                    quote,
                    decode_aux,
                    module.as_mut_ptr(),
                    pred.as_mut_ptr(),
                    &mut arity,
                    &mut aux_no,
                );
                !p.is_null()
            }
        {
            src = p;
            n = strlen(module.as_mut_ptr()) as libc::c_int;
            if n != 0 {
                let fresh20 = n;
                n = n + 1;
                module[fresh20 as usize] = ':' as i32 as libc::c_char;
            }
            n
                += sprintf(
                    module.as_mut_ptr().offset(n as isize),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    pred.as_mut_ptr(),
                );
            if arity >= 0 as libc::c_int {
                n
                    += sprintf(
                        module.as_mut_ptr().offset(n as isize),
                        b"/%d\0" as *const u8 as *const libc::c_char,
                        arity,
                    );
            }
            if decode_aux == 2 as libc::c_int && aux_no >= 0 as libc::c_int {
                n
                    += sprintf(
                        module.as_mut_ptr().offset(n as isize),
                        b"(aux %d)\0" as *const u8 as *const libc::c_char,
                        aux_no,
                    );
            }
            dst = dst.offset(sprintf(dst, format, module.as_mut_ptr()) as isize);
        } else {
            let fresh21 = src;
            src = src.offset(1);
            let fresh22 = dst;
            dst = dst.offset(1);
            *fresh22 = *fresh21;
        }
    }
    *dst = '\0' as i32 as libc::c_char;
    return result.as_mut_ptr();
}
unsafe extern "C" fn Copy_Not_Encoded_String(
    mut str: *mut libc::c_char,
    mut buff: *mut libc::c_char,
) -> *mut libc::c_char {
    if !(*str as libc::c_int >= 'a' as i32 && *str as libc::c_int <= 'z' as i32
        || *str as libc::c_int >= 'A' as i32 && *str as libc::c_int <= 'Z' as i32
        || *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
    {
        return 0 as *mut libc::c_char;
    }
    let fresh23 = str;
    str = str.offset(1);
    let fresh24 = buff;
    buff = buff.offset(1);
    *fresh24 = *fresh23;
    while *str as libc::c_int != '\0' as i32 {
        if *str as libc::c_int == '_' as i32 {
            if *str.offset(-(1 as libc::c_int) as isize) as libc::c_int == '_' as i32
                || *str.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                str = str.offset(-1);
                str;
                buff = buff.offset(-1);
                buff;
                break;
            }
        } else if !(*str as libc::c_int >= 'a' as i32
            && *str as libc::c_int <= 'z' as i32
            || *str as libc::c_int >= 'A' as i32 && *str as libc::c_int <= 'Z' as i32
            || *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            break;
        }
        let fresh25 = str;
        str = str.offset(1);
        let fresh26 = buff;
        buff = buff.offset(1);
        *fresh26 = *fresh25;
    }
    *buff = '\0' as i32 as libc::c_char;
    return str;
}
unsafe extern "C" fn Decode_String(
    mut str: *mut libc::c_char,
    mut buff: *mut libc::c_char,
) -> *mut libc::c_char {
    while (*(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *str as libc::c_int >= 'A' as i32 && *str as libc::c_int <= 'F' as i32)
        && (*(*__ctype_b_loc())
            .offset(*str.offset(1 as libc::c_int as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || *str.offset(1 as libc::c_int as isize) as libc::c_int >= 'A' as i32
                && *str.offset(1 as libc::c_int as isize) as libc::c_int <= 'F' as i32)
    {
        let fresh27 = buff;
        buff = buff.offset(1);
        *fresh27 = ((if *(*__ctype_b_loc()).offset(*str as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *str as libc::c_int - '0' as i32
        } else {
            *str as libc::c_int - 'A' as i32 + 10 as libc::c_int
        }) * 16 as libc::c_int
            + (if *(*__ctype_b_loc())
                .offset(*str.offset(1 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                *str.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32
            } else {
                *str.offset(1 as libc::c_int as isize) as libc::c_int - 'A' as i32
                    + 10 as libc::c_int
            })) as libc::c_char;
        str = str.offset(2 as libc::c_int as isize);
    }
    *buff = '\0' as i32 as libc::c_char;
    return str;
}
unsafe extern "C" fn Quote_If_Needed(mut str: *mut libc::c_char) {
    static mut buff: [libc::c_char; 2048] = [0; 2048];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    if *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
        & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = str;
        while *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *p as libc::c_int == '_' as i32
        {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            return;
        }
    }
    p = str;
    q = buff.as_mut_ptr();
    let fresh28 = q;
    q = q.offset(1);
    *fresh28 = '\'' as i32 as libc::c_char;
    p = str;
    while *p != 0 {
        r = strchr(pl_escape_char.as_mut_ptr(), *p as libc::c_int);
        if !r.is_null() {
            let fresh29 = q;
            q = q.offset(1);
            *fresh29 = '\\' as i32 as libc::c_char;
            let fresh30 = q;
            q = q.offset(1);
            *fresh30 = pl_escape_symbol[r.offset_from(pl_escape_char.as_mut_ptr())
                as libc::c_long as usize];
        } else if *p as libc::c_int == '\'' as i32 || *p as libc::c_int == '\\' as i32 {
            let fresh31 = q;
            q = q.offset(1);
            *fresh31 = *p;
            let fresh32 = q;
            q = q.offset(1);
            *fresh32 = *p;
        } else if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            q = q
                .offset(
                    sprintf(
                        q,
                        b"\\x%x\\\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_uchar as libc::c_uint,
                    ) as isize,
                );
        } else {
            let fresh33 = q;
            q = q.offset(1);
            *fresh33 = *p;
        }
        p = p.offset(1);
        p;
    }
    let fresh34 = q;
    q = q.offset(1);
    *fresh34 = '\'' as i32 as libc::c_char;
    *q = '\0' as i32 as libc::c_char;
    strcpy(str, buff.as_mut_ptr());
}
unsafe extern "C" fn Parse_Atom(
    mut src: *mut libc::c_char,
    mut dst: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut delim: libc::c_char = 0;
    if *src as libc::c_int == '\'' as i32 || *src as libc::c_int == '"' as i32 {
        let fresh35 = src;
        src = src.offset(1);
        delim = *fresh35;
        while *src as libc::c_int != 0
            && (*src as libc::c_int != delim as libc::c_int
                || *src.offset(1 as libc::c_int as isize) as libc::c_int
                    == delim as libc::c_int)
        {
            if *src as libc::c_int == delim as libc::c_int {
                let fresh36 = dst;
                dst = dst.offset(1);
                *fresh36 = delim;
                src = src.offset(2 as libc::c_int as isize);
            } else if *src as libc::c_int == '\\' as i32 {
                src = src.offset(1);
                src;
                if !(strchr(
                    b"\\'\"`\0" as *const u8 as *const libc::c_char,
                    *src as libc::c_int,
                ))
                    .is_null()
                {
                    let fresh37 = src;
                    src = src.offset(1);
                    let fresh38 = dst;
                    dst = dst.offset(1);
                    *fresh38 = *fresh37;
                } else {
                    p = strchr(pl_escape_symbol.as_mut_ptr(), *src as libc::c_int);
                    if !p.is_null() {
                        let fresh39 = dst;
                        dst = dst.offset(1);
                        *fresh39 = pl_escape_char[p
                            .offset_from(pl_escape_symbol.as_mut_ptr()) as libc::c_long
                            as usize];
                    } else {
                        if *src as libc::c_int == 'x' as i32 {
                            src = src.offset(1);
                            src;
                            i = 16 as libc::c_int;
                        } else {
                            i = 8 as libc::c_int;
                        }
                        i = strtol(src, &mut p, i) as libc::c_int;
                        if p == src
                            || *(*__ctype_b_loc()).offset(*src as libc::c_int as isize)
                                as libc::c_int
                                & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0 || *p as libc::c_int != '\\' as i32
                        {
                            return 0 as *mut libc::c_char;
                        }
                        let fresh40 = dst;
                        dst = dst.offset(1);
                        *fresh40 = i as libc::c_char;
                        src = p.offset(1 as libc::c_int as isize);
                    }
                }
            } else {
                let fresh41 = src;
                src = src.offset(1);
                let fresh42 = dst;
                dst = dst.offset(1);
                *fresh42 = *fresh41;
            }
        }
        if *src as libc::c_int != delim as libc::c_int {
            return 0 as *mut libc::c_char;
        }
        src = src.offset(1);
        src;
    } else {
        if *(*__ctype_b_loc()).offset(*src as libc::c_int as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
            && *src as libc::c_int != '_' as i32 && *src as libc::c_int != '$' as i32
        {
            return 0 as *mut libc::c_char;
        }
        while *(*__ctype_b_loc()).offset(*src as libc::c_int as isize) as libc::c_int
            & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *src as libc::c_int == '_' as i32 || *src as libc::c_int == '$' as i32
        {
            let fresh43 = src;
            src = src.offset(1);
            let fresh44 = dst;
            dst = dst.offset(1);
            *fresh44 = *fresh43;
        }
    }
    *dst = '\0' as i32 as libc::c_char;
    return src;
}
unsafe extern "C" fn Mk_Copying_Message(
    mut sub_part: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut buff: [libc::c_char; 256] = [0; 256];
    if !sub_part.is_null() {
        sprintf(
            buff.as_mut_ptr(),
            b"%s (%s) %s\n\0" as *const u8 as *const libc::c_char,
            sub_part,
            b"GNU Prolog\0" as *const u8 as *const libc::c_char,
            b"1.5.0\0" as *const u8 as *const libc::c_char,
        );
    } else {
        sprintf(
            buff.as_mut_ptr(),
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            b"GNU Prolog\0" as *const u8 as *const libc::c_char,
            b"1.5.0\0" as *const u8 as *const libc::c_char,
        );
    }
    strcat(
        buff.as_mut_ptr(),
        b"Copyright (C) 1999-2025 Daniel Diaz\n\nGNU Prolog comes with ABSOLUTELY NO WARRANTY.\nThis is free software; see the source or the file\nnamed COPYING for copying conditions.\n\0"
            as *const u8 as *const libc::c_char,
    );
    return buff.as_mut_ptr();
}
unsafe extern "C" fn Display_Copying(mut sub_part: *mut libc::c_char) {
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        Mk_Copying_Message(sub_part),
    );
}
unsafe extern "C" fn Get_Prolog_Path(
    mut argv0: *mut libc::c_char,
    mut devel_mode_0: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    static mut prolog_path_cache: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut devel_mode_cache: libc::c_int = 0 as libc::c_int;
    static mut resolved: [libc::c_char; 4096] = [0; 4096];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !prolog_path_cache.is_null() {
        *devel_mode_0 = devel_mode_cache;
        return prolog_path_cache;
    }
    p = getenv(b"PL_PATH\0" as *const u8 as *const libc::c_char);
    if !p.is_null() {
        strcpy(resolved.as_mut_ptr(), p);
        p = Is_A_Valid_Root(resolved.as_mut_ptr(), devel_mode_0);
        if !p.is_null() {
            current_block = 3678226613078732738;
        } else {
            current_block = 10879442775620481940;
        }
    } else {
        current_block = 10879442775620481940;
    }
    match current_block {
        10879442775620481940 => {
            if !(!argv0.is_null()
                && {
                    p = Get_Prolog_Path_From_Exec(argv0, devel_mode_0);
                    !p.is_null()
                })
            {
                p = Search_Path(
                    b"gplc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if p.is_null() {
                    return 0 as *mut libc::c_char;
                }
                p = Get_Prolog_Path_From_Exec(p, devel_mode_0);
                if p.is_null() {
                    return 0 as *mut libc::c_char;
                }
            }
        }
        _ => {}
    }
    devel_mode_cache = *devel_mode_0;
    prolog_path_cache = strdup(p);
    return p;
}
unsafe extern "C" fn Get_Prolog_Path_From_Exec(
    mut str: *mut libc::c_char,
    mut devel_mode_0: *mut libc::c_int,
) -> *mut libc::c_char {
    static mut resolved: [libc::c_char; 4096] = [0; 4096];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if (realpath(str, resolved.as_mut_ptr())).is_null() {
        return 0 as *mut libc::c_char;
    }
    p = resolved
        .as_mut_ptr()
        .offset(strlen(resolved.as_mut_ptr()) as isize)
        .offset(-(1 as libc::c_int as isize));
    while p > resolved.as_mut_ptr()
        && !(*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if p == resolved.as_mut_ptr() {
        return 0 as *mut libc::c_char;
    }
    while p > resolved.as_mut_ptr()
        && (*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if p == resolved.as_mut_ptr() {
        return 0 as *mut libc::c_char;
    }
    while p > resolved.as_mut_ptr()
        && !(*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if p == resolved.as_mut_ptr() {
        return 0 as *mut libc::c_char;
    }
    *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return Is_A_Valid_Root(resolved.as_mut_ptr(), devel_mode_0);
}
unsafe extern "C" fn Is_A_Valid_Root(
    mut str: *mut libc::c_char,
    mut devel_mode_0: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = str.offset(strlen(str) as isize).offset(-(1 as libc::c_int as isize));
    while p >= str
        && (*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if !(p < str) {
        p = p.offset(1);
        p;
        strcpy(p, b"/bin/gplc\0" as *const u8 as *const libc::c_char);
        if access(str, 1 as libc::c_int) == 0 as libc::c_int {
            *p = '\0' as i32 as libc::c_char;
            *devel_mode_0 = 0 as libc::c_int;
            current_block = 6240996679092828214;
        } else {
            strcpy(p, b"/TopComp\0" as *const u8 as *const libc::c_char);
            if access(str, 0 as libc::c_int) == 0 as libc::c_int {
                *p = '\0' as i32 as libc::c_char;
                *devel_mode_0 = 1 as libc::c_int;
                current_block = 6240996679092828214;
            } else {
                current_block = 8123227981631641850;
            }
        }
        match current_block {
            8123227981631641850 => {}
            _ => return str,
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn Search_Path(mut file: *mut libc::c_char) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = getenv(
        b"PATH\0" as *const u8 as *const libc::c_char,
    );
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    if path.is_null() {
        return 0 as *mut libc::c_char;
    }
    p = path;
    loop {
        p = strchr(path, ':' as i32);
        if !p.is_null() {
            l = p.offset_from(path) as libc::c_long as libc::c_int;
            strncpy(buff.as_mut_ptr(), path, l as libc::c_ulong);
        } else {
            strcpy(buff.as_mut_ptr(), path);
            l = strlen(buff.as_mut_ptr()) as libc::c_int;
        }
        let fresh45 = l;
        l = l + 1;
        buff[fresh45 as usize] = '/' as i32 as libc::c_char;
        strcpy(buff.as_mut_ptr().offset(l as isize), file);
        if access(buff.as_mut_ptr(), 1 as libc::c_int) == 0 as libc::c_int {
            return buff.as_mut_ptr();
        }
        if p.is_null() {
            break;
        }
        path = p.offset(1 as libc::c_int as isize);
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_M_Tempnam(
    mut dir: *mut libc::c_char,
    mut pfx: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut tmpl: [libc::c_char; 4096] = [0; 4096];
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dlen: libc::c_int = 0;
    let mut plen: libc::c_int = 0;
    let mut buf: stat = stat {
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
    if pfx.is_null() || *pfx.offset(0 as libc::c_int as isize) == 0 {
        pfx = b"file\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        plen = 4 as libc::c_int;
    } else {
        plen = strlen(pfx) as libc::c_int;
        if plen > 5 as libc::c_int {
            plen = 5 as libc::c_int;
        }
    }
    d = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
    if !d.is_null()
        && (stat(d, &mut buf) == 0 as libc::c_int
            && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
    {
        dir = d;
    } else if !(!dir.is_null()
        && (stat(dir, &mut buf) == 0 as libc::c_int
            && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint))
    {
        dir = 0 as *mut libc::c_char;
    }
    if dir.is_null() {
        if stat(b"/tmp\0" as *const u8 as *const libc::c_char, &mut buf)
            == 0 as libc::c_int
            && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint
        {
            dir = b"/tmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if strcmp(
            b"/tmp\0" as *const u8 as *const libc::c_char,
            b"/tmp\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
            && (stat(b"/tmp\0" as *const u8 as *const libc::c_char, &mut buf)
                == 0 as libc::c_int
                && buf.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o40000 as libc::c_int as libc::c_uint)
        {
            dir = b"/tmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            *__errno_location() = 2 as libc::c_int;
            return 0 as *mut libc::c_char;
        }
    }
    dlen = strlen(dir) as libc::c_int;
    while dlen > 1 as libc::c_int
        && *dir.offset((dlen - 1 as libc::c_int) as isize) as libc::c_int == '/' as i32
    {
        dlen -= 1;
        dlen;
    }
    if (4096 as libc::c_int)
        < dlen + 1 as libc::c_int + plen + 6 as libc::c_int + 1 as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    sprintf(
        tmpl.as_mut_ptr(),
        b"%.*s/%.*sXXXXXX\0" as *const u8 as *const libc::c_char,
        dlen,
        dir,
        plen,
        pfx,
    );
    d = Pl_M_Mktemp(tmpl.as_mut_ptr());
    if !d.is_null() {
        d = strdup(d);
    }
    return d;
}
pub unsafe extern "C" fn Pl_M_Mktemp(mut tmpl: *mut libc::c_char) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut XXXXXX: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut value: PlULong = 0;
    let mut count: libc::c_int = 0;
    let mut buf: stat = stat {
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
    static mut letters: [libc::c_char; 63] = unsafe {
        *::std::mem::transmute::<
            &[u8; 63],
            &[libc::c_char; 63],
        >(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0")
    };
    len = strlen(tmpl) as libc::c_int;
    if len < 6 as libc::c_int
        || strcmp(
            &mut *tmpl.offset((len - 6 as libc::c_int) as isize),
            b"XXXXXX\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    XXXXXX = &mut *tmpl.offset((len - 6 as libc::c_int) as isize) as *mut libc::c_char;
    value = (value as libc::c_ulong)
        .wrapping_add(time(0 as *mut time_t) as PlULong ^ getpid() as libc::c_ulong)
        as PlULong as PlULong;
    count = 0 as libc::c_int;
    while count < 238328 as libc::c_int {
        let mut v: PlULong = value;
        *XXXXXX
            .offset(
                0 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                1 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                2 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                3 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                4 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        v = (v as libc::c_ulong).wrapping_div(62 as libc::c_int as libc::c_ulong)
            as PlULong as PlULong;
        *XXXXXX
            .offset(
                5 as libc::c_int as isize,
            ) = letters[v.wrapping_rem(62 as libc::c_int as libc::c_ulong) as usize];
        if lstat(tmpl, &mut buf) < 0 as libc::c_int {
            if *__errno_location() == 2 as libc::c_int {
                *__errno_location() = 0 as libc::c_int;
                return tmpl;
            } else {
                return 0 as *mut libc::c_char
            }
        }
        value = (value as libc::c_ulong)
            .wrapping_add(7777 as libc::c_int as libc::c_ulong) as PlULong as PlULong;
        count += 1;
        count;
    }
    *__errno_location() = 17 as libc::c_int;
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_M_Spawn_Redirect(
    mut arg: *mut *mut libc::c_char,
    mut detach: libc::c_int,
    mut f_in: *mut *mut FILE,
    mut f_out: *mut *mut FILE,
    mut f_err: *mut *mut FILE,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pipe_in: [libc::c_int; 2] = [0; 2];
    let mut pipe_out: [libc::c_int; 2] = [0; 2];
    let mut pipe_err: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    fflush(stdout);
    fflush(stderr);
    if *arg.offset(1 as libc::c_int as isize) == 1 as libc::c_int as *mut libc::c_char {
        arg = Pl_M_Cmd_Line_To_Argv(
            *arg.offset(0 as libc::c_int as isize),
            0 as *mut libc::c_int,
        );
    }
    if !(!f_in.is_null() && pipe(pipe_in.as_mut_ptr()) != 0
        || !f_out.is_null() && pipe(pipe_out.as_mut_ptr()) != 0
        || !f_err.is_null() && f_err != f_out && pipe(pipe_err.as_mut_ptr()) != 0)
    {
        pid = fork();
        if !(pid == -(1 as libc::c_int)) {
            if pid == 0 as libc::c_int {
                if detach == 0 || fork() == 0 as libc::c_int {
                    if !(!f_in.is_null()
                        && (close(pipe_in[1 as libc::c_int as usize]) != 0
                            || pipe_in[0 as libc::c_int as usize] != 0 as libc::c_int
                                && (dup2(
                                    pipe_in[0 as libc::c_int as usize],
                                    0 as libc::c_int,
                                ) == -(1 as libc::c_int)
                                    || close(pipe_in[0 as libc::c_int as usize]) != 0)))
                    {
                        if !(!f_out.is_null()
                            && (close(pipe_out[0 as libc::c_int as usize]) != 0
                                || pipe_out[1 as libc::c_int as usize] != 1 as libc::c_int
                                    && (dup2(
                                        pipe_out[1 as libc::c_int as usize],
                                        1 as libc::c_int,
                                    ) == -(1 as libc::c_int)
                                        || close(pipe_out[1 as libc::c_int as usize]) != 0)))
                        {
                            if !f_err.is_null() {
                                if f_err != f_out {
                                    if close(pipe_err[0 as libc::c_int as usize]) != 0
                                        || pipe_err[1 as libc::c_int as usize] != 2 as libc::c_int
                                            && (dup2(
                                                pipe_err[1 as libc::c_int as usize],
                                                2 as libc::c_int,
                                            ) == -(1 as libc::c_int)
                                                || close(pipe_err[1 as libc::c_int as usize]) != 0)
                                    {
                                        current_block = 16473627950311267560;
                                    } else {
                                        current_block = 12349973810996921269;
                                    }
                                } else if dup2(1 as libc::c_int, 2 as libc::c_int)
                                    == -(1 as libc::c_int)
                                {
                                    current_block = 16473627950311267560;
                                } else {
                                    current_block = 12349973810996921269;
                                }
                            } else {
                                current_block = 12349973810996921269;
                            }
                            match current_block {
                                16473627950311267560 => {}
                                _ => {
                                    execvp(
                                        *arg.offset(0 as libc::c_int as isize),
                                        arg as *const *mut libc::c_char,
                                    );
                                    exit(
                                        if *__errno_location() == 2 as libc::c_int
                                            || *__errno_location() == 20 as libc::c_int
                                        {
                                            126 as libc::c_int
                                        } else {
                                            127 as libc::c_int
                                        },
                                    );
                                }
                            }
                        }
                    }
                } else {
                    exit(0 as libc::c_int);
                }
            } else {
                if detach != 0 {
                    if waitpid(pid, &mut status, 0 as libc::c_int) < 0 as libc::c_int {
                        current_block = 16473627950311267560;
                    } else {
                        pid = 0 as libc::c_int;
                        current_block = 4808432441040389987;
                    }
                } else {
                    current_block = 4808432441040389987;
                }
                match current_block {
                    16473627950311267560 => {}
                    _ => {
                        if !(!f_in.is_null()
                            && (close(pipe_in[0 as libc::c_int as usize]) != 0
                                || {
                                    *f_in = fdopen(
                                        pipe_in[1 as libc::c_int as usize],
                                        b"wt\0" as *const u8 as *const libc::c_char,
                                    );
                                    (*f_in).is_null()
                                }))
                        {
                            if !(!f_out.is_null()
                                && (close(pipe_out[1 as libc::c_int as usize]) != 0
                                    || {
                                        *f_out = fdopen(
                                            pipe_out[0 as libc::c_int as usize],
                                            b"rt\0" as *const u8 as *const libc::c_char,
                                        );
                                        (*f_out).is_null()
                                    }))
                            {
                                if !(!f_err.is_null() && f_err != f_out
                                    && (close(pipe_err[1 as libc::c_int as usize]) != 0
                                        || {
                                            *f_err = fdopen(
                                                pipe_err[0 as libc::c_int as usize],
                                                b"rt\0" as *const u8 as *const libc::c_char,
                                            );
                                            (*f_err).is_null()
                                        }))
                                {
                                    return pid;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_M_Get_Status(mut pid: libc::c_int) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    if waitpid(pid, &mut status, 0 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        status = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        if status == 127 as libc::c_int {
            status = -(2 as libc::c_int);
        } else if status == 126 as libc::c_int {
            status = -(1 as libc::c_int);
            *__errno_location() = 2 as libc::c_int;
        }
    }
    return status;
}
pub unsafe extern "C" fn Pl_M_Spawn(mut arg: *mut *mut libc::c_char) -> libc::c_int {
    let mut pid: libc::c_int = 0;
    fflush(stdout);
    fflush(stderr);
    if *arg.offset(1 as libc::c_int as isize) == 1 as libc::c_int as *mut libc::c_char {
        arg = Pl_M_Cmd_Line_To_Argv(
            *arg.offset(0 as libc::c_int as isize),
            0 as *mut libc::c_int,
        );
    }
    pid = fork();
    if pid == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if pid == 0 as libc::c_int {
        execvp(*arg.offset(0 as libc::c_int as isize), arg as *const *mut libc::c_char);
        exit(
            if *__errno_location() == 2 as libc::c_int
                || *__errno_location() == 20 as libc::c_int
            {
                126 as libc::c_int
            } else {
                127 as libc::c_int
            },
        );
    }
    return Pl_M_Get_Status(pid);
}
pub unsafe extern "C" fn Pl_M_Shell(mut cmd: *mut libc::c_char) -> libc::c_int {
    return Pl_M_Spawn(Pl_M_Create_Shell_Command(cmd));
}
pub unsafe extern "C" fn Pl_M_Cmd_Line_To_Argv(
    mut cmd: *mut libc::c_char,
    mut argc: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    static mut arg: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    static mut nb_arg: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = cmd;
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        if i >= nb_arg {
            nb_arg += 64 as libc::c_int;
            arg = (if arg.is_null() {
                malloc(
                    (nb_arg as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                )
            } else {
                realloc(
                    arg as *mut libc::c_void,
                    (nb_arg as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                )
            }) as *mut *mut libc::c_char;
        }
        let fresh46 = i;
        i = i + 1;
        let ref mut fresh47 = *arg.offset(fresh46 as isize);
        *fresh47 = p;
        while *p as libc::c_int != ' ' as i32 && *p as libc::c_int != '\t' as i32
            && *p as libc::c_int != '\0' as i32
        {
            if *p as libc::c_int == '"' as i32 {
                loop {
                    p = p.offset(1);
                    p;
                    if !(*p as libc::c_int != '"' as i32
                        && *p as libc::c_int != '\0' as i32)
                    {
                        break;
                    }
                }
                if *p as libc::c_int == '"' as i32 {
                    p = p.offset(1);
                    p;
                }
            } else {
                p = p.offset(1);
                p;
            }
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        let fresh48 = p;
        p = p.offset(1);
        *fresh48 = '\0' as i32 as libc::c_char;
    }
    let ref mut fresh49 = *arg.offset(i as isize);
    *fresh49 = 0 as *mut libc::c_char;
    if !argc.is_null() {
        *argc = i;
    }
    return arg;
}
pub static mut pl_m_os_type: libc::c_int = 0;
pub static mut pl_m_architecture: [libc::c_char; 32] = [0; 32];
pub static mut pl_m_os_version: [libc::c_char; 256] = [0; 256];
pub unsafe extern "C" fn Pl_Init_Machine1() {
    let mut uname_info: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    pl_m_os_type = 0 as libc::c_int;
    if uname(&mut uname_info) < 0 as libc::c_int {
        strcpy(
            pl_m_architecture.as_mut_ptr(),
            b"unknown architecture\0" as *const u8 as *const libc::c_char,
        );
        strcpy(
            pl_m_os_version.as_mut_ptr(),
            b"unknown OS version\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    strcpy(pl_m_architecture.as_mut_ptr(), (uname_info.machine).as_mut_ptr());
    sprintf(
        pl_m_os_version.as_mut_ptr(),
        b"%s %s\0" as *const u8 as *const libc::c_char,
        (uname_info.sysname).as_mut_ptr(),
        (uname_info.release).as_mut_ptr(),
    );
}
pub unsafe extern "C" fn Pl_M_Create_Shell_Command(
    mut cmd: *mut libc::c_char,
) -> *mut *mut libc::c_char {
    static mut arg: [*mut libc::c_char; 4] = [0 as *const libc::c_char
        as *mut libc::c_char; 4];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = getenv(b"SHELL\0" as *const u8 as *const libc::c_char);
    arg[0 as libc::c_int
        as usize] = (if !p.is_null() {
        p as *const libc::c_char
    } else {
        b"/bin/sh\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    arg[1 as libc::c_int
        as usize] = b"-c\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if !cmd.is_null() {
        arg[2 as libc::c_int as usize] = cmd;
        arg[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
    } else {
        arg[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    }
    return arg.as_mut_ptr();
}
pub static mut start_path: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut devel_mode: libc::c_int = 0 as libc::c_int;
pub static mut devel_dir: [*mut libc::c_char; 8] = [
    b"EnginePl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"BipsPl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EngineFD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"BipsFD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Linedit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"W32GUICons\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TopComp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut file_lopt: *mut FileInf = 0 as *const FileInf as *mut FileInf;
pub static mut nb_file_lopt: libc::c_int = 0 as libc::c_int;
pub static mut stop_after: libc::c_int = 7 as libc::c_int;
pub static mut verbose: libc::c_int = 0 as libc::c_int;
pub static mut file_name_out: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut pl_def_local_size: libc::c_int = -(1 as libc::c_int);
pub static mut pl_def_global_size: libc::c_int = -(1 as libc::c_int);
pub static mut pl_def_trail_size: libc::c_int = -(1 as libc::c_int);
pub static mut pl_def_cstr_size: libc::c_int = -(1 as libc::c_int);
pub static mut pl_def_max_atom: libc::c_int = -(1 as libc::c_int);
pub static mut pl_fixed_sizes: libc::c_int = 0 as libc::c_int;
pub static mut needs_stack_file: libc::c_int = 0 as libc::c_int;
pub static mut bc_mode: libc::c_int = 0 as libc::c_int;
pub static mut gui_console: libc::c_int = 0 as libc::c_int;
pub static mut new_top_level: libc::c_int = 0 as libc::c_int;
pub static mut no_top_level: libc::c_int = 0 as libc::c_int;
pub static mut min_pl_bips: libc::c_int = 0 as libc::c_int;
pub static mut min_fd_bips: libc::c_int = 0 as libc::c_int;
pub static mut no_debugger: libc::c_int = 0 as libc::c_int;
pub static mut no_pl_lib: libc::c_int = 0 as libc::c_int;
pub static mut no_fd_lib: libc::c_int = 0 as libc::c_int;
pub static mut no_fd_lib_warn: libc::c_int = 0 as libc::c_int;
pub static mut strip: libc::c_int = 0 as libc::c_int;
pub static mut no_decode_hex: libc::c_int = 0 as libc::c_int;
pub static mut warn_str: [libc::c_char; 1024] = unsafe {
    *::std::mem::transmute::<
        &[u8; 1024],
        &mut [libc::c_char; 1024],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
pub static mut temp_dir: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut no_del_temp_files: libc::c_int = 0 as libc::c_int;
pub static mut cmd_pl2wam: CmdInf = unsafe {
    {
        let mut init = CmdInf {
            exe_name: b"pl2wam\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            opt: *::std::mem::transmute::<
                &[u8; 8192],
                &mut [libc::c_char; 8192],
            >(
                b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            out_opt: b"-o \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
pub static mut cmd_wam2ma: CmdInf = unsafe {
    {
        let mut init = CmdInf {
            exe_name: b"wam2ma\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            opt: *::std::mem::transmute::<
                &[u8; 8192],
                &mut [libc::c_char; 8192],
            >(
                b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            out_opt: b"-o \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
pub static mut cmd_ma2asm: CmdInf = unsafe {
    {
        let mut init = CmdInf {
            exe_name: b"ma2asm\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            opt: *::std::mem::transmute::<
                &[u8; 8192],
                &mut [libc::c_char; 8192],
            >(
                b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            out_opt: b"-o \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
pub static mut cmd_asm: CmdInf = unsafe {
    {
        let mut init = CmdInf {
            exe_name: b"as\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            opt: *::std::mem::transmute::<
                &[u8; 8192],
                &mut [libc::c_char; 8192],
            >(
                b" --64 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            out_opt: b"-o \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
pub static mut cmd_fd2c: CmdInf = unsafe {
    {
        let mut init = CmdInf {
            exe_name: b"fd2c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            opt: *::std::mem::transmute::<
                &[u8; 8192],
                &mut [libc::c_char; 8192],
            >(
                b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            out_opt: b"-o \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
pub static mut cmd_cc: CmdInf = unsafe {
    {
        let mut init = CmdInf {
            exe_name: b"gcc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            opt: *::std::mem::transmute::<
                &[u8; 8192],
                &mut [libc::c_char; 8192],
            >(
                b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            out_opt: b"-o \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
pub static mut cmd_link: CmdInf = unsafe {
    {
        let mut init = CmdInf {
            exe_name: b"gcc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            opt: *::std::mem::transmute::<
                &[u8; 8192],
                &mut [libc::c_char; 8192],
            >(
                b"  -fno-strict-aliasing -fcommon \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            out_opt: b"-o \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
pub static mut cc_fd2c_flags: *mut libc::c_char = b"-O3 -fomit-frame-pointer -Wno-char-subscripts \0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
pub static mut suffixes: [*mut libc::c_char; 8] = [
    b".pl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".wam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".ma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".fd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b".c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut last_opt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pdev: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    file_lopt = calloc(
        (argc + 1 as libc::c_int) as libc::c_ulong,
        ::std::mem::size_of::<FileInf>() as libc::c_ulong,
    ) as *mut FileInf;
    if file_lopt.is_null() {
        Pl_Fatal_Error(
            b"memory allocation fault\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Parse_Arguments(argc, argv);
    if verbose != 0 {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    start_path = Get_Prolog_Path(
        *argv.offset(0 as libc::c_int as isize),
        &mut devel_mode,
    );
    if start_path.is_null() {
        Pl_Fatal_Error(
            b"cannot find the path for %s, set environment variable %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            b"GNU Prolog\0" as *const u8 as *const libc::c_char,
            b"PL_PATH\0" as *const u8 as *const libc::c_char,
        );
    }
    strcat(
        (cmd_cc.opt).as_mut_ptr(),
        b" -fno-strict-aliasing -fcommon -ffixed-r12 -ffixed-r13 -ffixed-r14 -ffixed-r15 -c \0"
            as *const u8 as *const libc::c_char,
    );
    if devel_mode != 0 {
        pdev = devel_dir.as_mut_ptr();
        while !(*pdev).is_null() {
            sprintf(
                (cmd_cc.opt)
                    .as_mut_ptr()
                    .offset(strlen((cmd_cc.opt).as_mut_ptr()) as isize),
                b"%s%s/%s \0" as *const u8 as *const libc::c_char,
                b"-I\0" as *const u8 as *const libc::c_char,
                start_path,
                *pdev,
            );
            pdev = pdev.offset(1);
            pdev;
        }
    } else {
        sprintf(
            (cmd_cc.opt).as_mut_ptr().offset(strlen((cmd_cc.opt).as_mut_ptr()) as isize),
            b"%s%s/include \0" as *const u8 as *const libc::c_char,
            b"-I\0" as *const u8 as *const libc::c_char,
            start_path,
        );
    }
    strcat((cmd_link.opt).as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    if verbose != 0 {
        fprintf(
            stderr,
            b"Path used: %s %s\n\0" as *const u8 as *const libc::c_char,
            start_path,
            if devel_mode != 0 {
                b"(development mode)\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    Compile_Files();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Compile_Files() {
    let mut current_block: u64;
    let mut f: *mut FileInf = 0 as *mut FileInf;
    let mut stage: libc::c_int = 0;
    let mut stage_end: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut fd: *mut FILE = 0 as *mut FILE;
    if stop_after < 7 as libc::c_int {
        if *warn_str.as_mut_ptr() != 0 {
            fprintf(
                stderr,
                b"link not done - ignored option(s): %s\n\0" as *const u8
                    as *const libc::c_char,
                warn_str.as_mut_ptr(),
            );
        }
        stage_end = stop_after;
        needs_stack_file = 0 as libc::c_int;
        if bc_mode != 0 {
            suffixes[1 as libc::c_int
                as usize] = b".wbc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            strcat(
                (cmd_pl2wam.opt).as_mut_ptr(),
                b"--wam-for-byte-code \0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        stage_end = 3 as libc::c_int;
    }
    if needs_stack_file != 0 {
        f = file_lopt.offset(nb_file_lopt as isize);
        (*f).work_name2 = 0 as *mut libc::c_char;
        New_Work_File(f, 1 as libc::c_int, 10000 as libc::c_int);
        (*f).name = (*f).work_name2;
        (*f)
            .suffix = ((*f).name)
            .offset(strlen((*f).name) as isize)
            .offset(-(strlen(suffixes[2 as libc::c_int as usize]) as isize));
        (*f).type_0 = 2 as libc::c_int;
        (*f).work_name1 = (*f).name;
        (*f).work_name2 = 0 as *mut libc::c_char;
        if verbose != 0 {
            fprintf(
                stderr,
                b"creating stack size file: %s\n\0" as *const u8 as *const libc::c_char,
                (*f).name,
            );
        }
        fd = fopen((*f).name, b"wt\0" as *const u8 as *const libc::c_char);
        if fd.is_null() {
            Pl_Fatal_Error(
                b"cannot open stack size file (%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*f).name,
            );
        }
        if pl_def_local_size >= 0 as libc::c_int {
            fprintf(
                fd,
                b"long global pl_def_local_size = %d\n\0" as *const u8
                    as *const libc::c_char,
                pl_def_local_size,
            );
        }
        if pl_def_global_size >= 0 as libc::c_int {
            fprintf(
                fd,
                b"long global pl_def_global_size = %d\n\0" as *const u8
                    as *const libc::c_char,
                pl_def_global_size,
            );
        }
        if pl_def_trail_size >= 0 as libc::c_int {
            fprintf(
                fd,
                b"long global pl_def_trail_size = %d\n\0" as *const u8
                    as *const libc::c_char,
                pl_def_trail_size,
            );
        }
        if pl_def_cstr_size >= 0 as libc::c_int {
            fprintf(
                fd,
                b"long global pl_def_cstr_size = %d\n\0" as *const u8
                    as *const libc::c_char,
                pl_def_cstr_size,
            );
        }
        if pl_def_max_atom >= 0 as libc::c_int {
            fprintf(
                fd,
                b"long global pl_def_max_atom = %d\n\0" as *const u8
                    as *const libc::c_char,
                pl_def_max_atom,
            );
        }
        if pl_fixed_sizes != 0 {
            fprintf(
                fd,
                b"long global pl_fixed_sizes = 1\n\0" as *const u8 as *const libc::c_char,
            );
        }
        fclose(fd);
    }
    if verbose != 0 {
        fprintf(stderr, b"\n*** Compiling\n\0" as *const u8 as *const libc::c_char);
    }
    f = file_lopt;
    while !((*f).name).is_null() {
        if !((*f).type_0 == 8 as libc::c_int) {
            if verbose != 0
                && ((*f).type_0 == 5 as libc::c_int || (*f).type_0 == 6 as libc::c_int
                    || (*f).type_0 <= stage_end)
            {
                fprintf(
                    stderr,
                    b"\n--- file: %s\n\0" as *const u8 as *const libc::c_char,
                    (*f).name,
                );
            }
            if (*f).type_0 == 5 as libc::c_int && stop_after >= 3 as libc::c_int {
                stage = 5 as libc::c_int;
                New_Work_File(
                    f,
                    stage,
                    if stop_after == 5 as libc::c_int {
                        stop_after
                    } else {
                        10000 as libc::c_int
                    },
                );
                Compile_Cmd(&mut cmd_fd2c, f);
                if stop_after != 5 as libc::c_int {
                    stage = 3 as libc::c_int;
                    New_Work_File(f, stage, stop_after);
                    l = strlen((cmd_cc.opt).as_mut_ptr()) as libc::c_int;
                    strcpy((cmd_cc.opt).as_mut_ptr().offset(l as isize), cc_fd2c_flags);
                    Compile_Cmd(&mut cmd_cc, f);
                    cmd_cc.opt[l as usize] = '\0' as i32 as libc::c_char;
                }
                current_block = 390734930780304639;
            } else if (*f).type_0 == 6 as libc::c_int && stop_after >= 3 as libc::c_int
                && stop_after != 5 as libc::c_int
            {
                stage = 3 as libc::c_int;
                New_Work_File(f, stage, stop_after);
                Compile_Cmd(&mut cmd_cc, f);
                current_block = 390734930780304639;
            } else if (*f).type_0 == 5 as libc::c_int || (*f).type_0 == 6 as libc::c_int
                || stop_after == 5 as libc::c_int || (*f).type_0 > stop_after
            {
                fprintf(
                    stderr,
                    b"unused input file: %s\n\0" as *const u8 as *const libc::c_char,
                    (*f).name,
                );
                current_block = 2891135413264362348;
            } else {
                stage = (*f).type_0;
                while stage <= stage_end {
                    New_Work_File(f, stage, stop_after);
                    match stage {
                        0 => {
                            Compile_Cmd(&mut cmd_pl2wam, f);
                        }
                        1 => {
                            Compile_Cmd(&mut cmd_wam2ma, f);
                        }
                        2 => {
                            Compile_Cmd(&mut cmd_ma2asm, f);
                            if needs_stack_file != 0
                                && f == file_lopt.offset(nb_file_lopt as isize)
                                && no_del_temp_files == 0
                            {
                                if verbose != 0 {
                                    fprintf(
                                        stderr,
                                        b"deleting stack size file\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                Delete_Temp_File((*f).name);
                            }
                        }
                        3 => {
                            Compile_Cmd(&mut cmd_asm, f);
                        }
                        _ => {}
                    }
                    stage += 1;
                    stage;
                }
                current_block = 390734930780304639;
            }
            match current_block {
                2891135413264362348 => {}
                _ => {
                    Free_Work_File2(f);
                }
            }
        }
        f = f.offset(1);
        f;
    }
    if stop_after < 7 as libc::c_int {
        return;
    }
    if verbose != 0 {
        fprintf(stderr, b"\n*** Linking\n\n\0" as *const u8 as *const libc::c_char);
    }
    Link_Cmd();
    f = file_lopt;
    while !((*f).name).is_null() {
        if (*f).work_name1 != (*f).name {
            Delete_Temp_File((*f).work_name1);
        }
        f = f.offset(1);
        f;
    }
}
pub unsafe extern "C" fn Create_Output_File_Name(
    mut f: *mut FileInf,
    mut buff: *mut libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    static mut counter: libc::c_int = 0 as libc::c_int;
    p = file_name_out;
    while *p != 0 {
        if *p as libc::c_int != '%' as i32 {
            let fresh50 = buff;
            buff = buff.offset(1);
            *fresh50 = *p;
        } else {
            p = p.offset(1);
            match *p as libc::c_int {
                100 => {
                    strcpy(buff, (*f).name);
                    l = ((*f).file_part).offset_from((*f).name) as libc::c_long
                        as libc::c_int;
                    buff = buff.offset(l as isize);
                }
                102 => {
                    strcpy(buff, (*f).name);
                    buff = buff.offset(strlen(buff) as isize);
                }
                70 => {
                    strcpy(buff, (*f).file_part);
                    buff = buff.offset(strlen(buff) as isize);
                }
                112 => {
                    strcpy(buff, (*f).name);
                    l = ((*f).suffix).offset_from((*f).name) as libc::c_long
                        as libc::c_int;
                    buff = buff.offset(l as isize);
                }
                80 => {
                    strcpy(buff, (*f).file_part);
                    l = ((*f).suffix).offset_from((*f).file_part) as libc::c_long
                        as libc::c_int;
                    buff = buff.offset(l as isize);
                }
                115 => {
                    strcpy(buff, (*f).suffix);
                    buff = buff.offset(strlen(buff) as isize);
                }
                99 => {
                    counter += 1;
                    sprintf(buff, b"%d\0" as *const u8 as *const libc::c_char, counter);
                    buff = buff.offset(strlen(buff) as isize);
                }
                _ => {
                    let fresh51 = buff;
                    buff = buff.offset(1);
                    *fresh51 = '%' as i32 as libc::c_char;
                    let fresh52 = buff;
                    buff = buff.offset(1);
                    *fresh52 = *p;
                }
            }
        }
        p = p.offset(1);
        p;
    }
    *buff = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn New_Work_File(
    mut f: *mut FileInf,
    mut stage: libc::c_int,
    mut stop_after_0: libc::c_int,
) {
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if stage < stop_after_0 {
        p = Pl_M_Tempnam(
            temp_dir,
            b"gplc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        sprintf(
            buff.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            p,
            suffixes[(stage + 1 as libc::c_int) as usize],
        );
        free(p as *mut libc::c_void);
    } else if !file_name_out.is_null() {
        Create_Output_File_Name(f, buff.as_mut_ptr());
    } else {
        strcpy(buff.as_mut_ptr(), (*f).name);
        strcpy(
            buff
                .as_mut_ptr()
                .offset(((*f).suffix).offset_from((*f).name) as libc::c_long as isize),
            suffixes[(stage + 1 as libc::c_int) as usize],
        );
    }
    Free_Work_File2(f);
    (*f).work_name2 = strdup(buff.as_mut_ptr());
}
pub unsafe extern "C" fn Free_Work_File2(mut f: *mut FileInf) {
    if !((*f).work_name2).is_null() {
        if (*f).work_name1 != (*f).name {
            Delete_Temp_File((*f).work_name1);
        }
        (*f).work_name1 = (*f).work_name2;
    }
}
pub unsafe extern "C" fn Compile_Cmd(mut c: *mut CmdInf, mut f: *mut FileInf) {
    static mut buff: [libc::c_char; 12289] = [0; 12289];
    sprintf(
        buff.as_mut_ptr(),
        b"%s%s%s%s %s\0" as *const u8 as *const libc::c_char,
        (*c).exe_name,
        ((*c).opt).as_mut_ptr(),
        (*c).out_opt,
        (*f).work_name2,
        (*f).work_name1,
    );
    Exec_One_Cmd(buff.as_mut_ptr(), 1 as libc::c_int);
}
pub unsafe extern "C" fn Link_Cmd() {
    static mut file_out: [libc::c_char; 4096] = [0; 4096];
    static mut buff: [libc::c_char; 12289] = [0; 12289];
    let mut f: *mut FileInf = 0 as *mut FileInf;
    if no_fd_lib == 0 as libc::c_int && no_fd_lib_warn != 0 {
        if Find_File(
            b"libbips_fd.a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr(),
            1 as libc::c_int,
        ) == 0
            || Find_File(
                b"libengine_fd.a\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
                1 as libc::c_int,
            ) == 0
        {
            min_fd_bips = 1 as libc::c_int;
            no_fd_lib = min_fd_bips;
        }
    }
    if file_name_out.is_null() {
        file_name_out = b"%p\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    f = file_lopt;
    while (*f).type_0 == 8 as libc::c_int {
        f = f.offset(1);
        f;
    }
    Create_Output_File_Name(f, file_out.as_mut_ptr());
    file_name_out = file_out.as_mut_ptr();
    sprintf(
        buff.as_mut_ptr(),
        b"%s%s%s%s \0" as *const u8 as *const libc::c_char,
        cmd_link.exe_name,
        (cmd_link.opt).as_mut_ptr(),
        cmd_link.out_opt,
        file_name_out,
    );
    f = file_lopt;
    while !((*f).name).is_null() {
        sprintf(
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            b"%s \0" as *const u8 as *const libc::c_char,
            (*f).work_name1,
        );
        f = f.offset(1);
        f;
    }
    if min_pl_bips == 0 {
        Find_File(
            b"all_pl_bips\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    if min_fd_bips == 0 {
        Find_File(
            b"all_fd_bips\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    if new_top_level != 0 {
        Find_File(
            b"top_level_main\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    if no_top_level == 0 {
        Find_File(
            b"top_level\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    if no_debugger == 0 {
        Find_File(
            b"debugger\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    if no_fd_lib == 0 {
        Find_File(
            b"libbips_fd.a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        Find_File(
            b"libengine_fd.a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    if no_pl_lib == 0 {
        Find_File(
            b"libbips_pl.a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    Find_File(
        b"libengine_pl.a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
        0 as libc::c_int,
    );
    strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    Find_File(
        b"liblinedit.a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
        0 as libc::c_int,
    );
    strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    strcat(buff.as_mut_ptr(), b"-lm \0" as *const u8 as *const libc::c_char);
    if no_pl_lib == 0 && gui_console != 0 {
        Find_File(
            b"w32gc_interf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            0 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        Find_File(
            b"win_exe_icon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b".res\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            1 as libc::c_int,
        );
        strcat(buff.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    }
    Exec_One_Cmd(buff.as_mut_ptr(), no_decode_hex);
    if strip != 0
        && *(b"strip\0" as *const u8 as *const libc::c_char) as libc::c_int != ':' as i32
        && *(b"strip\0" as *const u8 as *const libc::c_char) as libc::c_int
            != '\0' as i32
    {
        sprintf(
            buff.as_mut_ptr(),
            b"%s %s%s\0" as *const u8 as *const libc::c_char,
            b"strip\0" as *const u8 as *const libc::c_char,
            file_name_out,
            b"\0" as *const u8 as *const libc::c_char,
        );
        Exec_One_Cmd(buff.as_mut_ptr(), 1 as libc::c_int);
    }
}
pub unsafe extern "C" fn Exec_One_Cmd(
    mut cmd: *mut libc::c_char,
    mut no_decode_hex_0: libc::c_int,
) {
    let mut status: libc::c_int = 0;
    static mut arg: [*mut libc::c_char; 2] = unsafe {
        [
            0 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int as *mut libc::c_char,
        ]
    };
    arg[0 as libc::c_int as usize] = cmd;
    if verbose != 0 {
        fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, cmd);
    }
    if no_decode_hex_0 == 1 as libc::c_int {
        status = Pl_M_Spawn(arg.as_mut_ptr());
    } else {
        status = Spawn_Decode_Hex(arg.as_mut_ptr());
    }
    if status == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"error trying to execute \0" as *const u8 as *const libc::c_char,
        );
        perror(arg[0 as libc::c_int as usize]);
    }
    if status == -(2 as libc::c_int) {
        fprintf(
            stderr,
            b"error trying to execute %s: unknown error\0" as *const u8
                as *const libc::c_char,
            arg[0 as libc::c_int as usize],
        );
    }
    if status != 0 {
        Pl_Fatal_Error(
            b"compilation failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
pub unsafe extern "C" fn Spawn_Decode_Hex(
    mut arg: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pid: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut f_out: *mut FILE = 0 as *mut FILE;
    static mut buff: [libc::c_char; 12289] = [0; 12289];
    pid = Pl_M_Spawn_Redirect(
        arg,
        0 as libc::c_int,
        0 as *mut *mut FILE,
        &mut f_out,
        &mut f_out,
    );
    if pid == -(1 as libc::c_int) || pid == -(2 as libc::c_int) {
        return pid;
    }
    loop {
        !(fgets(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12289]>() as libc::c_ulong
                as libc::c_int,
            f_out,
        ))
            .is_null();
        if feof(f_out) != 0 {
            break;
        }
        fputs(
            Decode_Hexa_Line(
                buff.as_mut_ptr(),
                b"predicate(%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
            ),
            stderr,
        );
    }
    if fclose(f_out) != 0 {
        return -(1 as libc::c_int);
    }
    status = Pl_M_Get_Status(pid);
    return status;
}
pub unsafe extern "C" fn Delete_Temp_File(mut name: *mut libc::c_char) {
    if no_del_temp_files != 0 {
        return;
    }
    if verbose != 0 {
        fprintf(stderr, b"delete %s\n\0" as *const u8 as *const libc::c_char, name);
    }
    unlink(name);
}
pub unsafe extern "C" fn Find_File(
    mut file: *mut libc::c_char,
    mut suff: *mut libc::c_char,
    mut file_path: *mut libc::c_char,
    mut ignore_error: libc::c_int,
) -> libc::c_int {
    let mut name: [libc::c_char; 4096] = [0; 4096];
    let mut pdev: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cur_end: *mut libc::c_char = file_path;
    sprintf(
        name.as_mut_ptr(),
        b"%s%s\0" as *const u8 as *const libc::c_char,
        file,
        suff,
    );
    if devel_mode == 0 {
        sprintf(
            file_path,
            b"%s/lib/%s\0" as *const u8 as *const libc::c_char,
            start_path,
            name.as_mut_ptr(),
        );
        if access(file_path, 0 as libc::c_int) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    } else {
        pdev = devel_dir.as_mut_ptr();
        while !(*pdev).is_null() {
            sprintf(
                file_path,
                b"%s/%s/%s\0" as *const u8 as *const libc::c_char,
                start_path,
                *pdev,
                name.as_mut_ptr(),
            );
            if access(file_path, 0 as libc::c_int) == 0 as libc::c_int {
                return 1 as libc::c_int;
            }
            pdev = pdev.offset(1);
            pdev;
        }
    }
    if ignore_error == 0 {
        Pl_Fatal_Error(
            b"cannot locate file %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
    }
    *cur_end = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Find_Suffix(
    mut suffixes_0: *mut libc::c_char,
    mut suffix: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strstr(suffixes_0, suffix);
    if !p.is_null()
        && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '|' as i32
        && *p.offset(strlen(suffix) as isize) as libc::c_int == '|' as i32
    {
        return p;
    }
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_Fatal_Error(mut format: *mut libc::c_char, mut args: ...) {
    let mut f: *mut FileInf = 0 as *mut FileInf;
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    vfprintf(stderr, format, arg_ptr.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    if no_del_temp_files != 0 {
        exit(1 as libc::c_int);
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"deleting temporary files before exit\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    f = file_lopt;
    while !((*f).name).is_null() {
        if !((*f).work_name1).is_null() && (*f).work_name1 != (*f).name
            && (file_name_out.is_null()
                || strcasecmp((*f).work_name1, file_name_out) != 0 as libc::c_int)
        {
            Delete_Temp_File((*f).work_name1);
        }
        if !((*f).work_name2).is_null() && (*f).work_name2 != (*f).name
            && (file_name_out.is_null()
                || strcasecmp((*f).work_name2, file_name_out) != 0 as libc::c_int)
        {
            Delete_Temp_File((*f).work_name2);
        }
        f = f.offset(1);
        f;
    }
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn Parse_Arguments(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut file_name_out_i: libc::c_int = 0;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FileInf = file_lopt;
    let mut nb_file: libc::c_int = 0 as libc::c_int;
    let mut current_block_168: u64;
    i = 1 as libc::c_int;
    while i < argc {
        if **argv.offset(i as isize) as libc::c_int == '-' as i32
            && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
        {
            last_opt = b"-o\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            if strncmp(
                *argv.offset(i as isize),
                b"-o\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
                || {
                    last_opt = b"--output\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    strncmp(
                        *argv.offset(i as isize),
                        b"--output\0" as *const u8 as *const libc::c_char,
                        strlen(*argv.offset(i as isize)),
                    ) == 0 as libc::c_int
                }
            {
                file_name_out_i = i;
                i += 1;
                if i >= argc {
                    Pl_Fatal_Error(
                        b"FILE missing after %s option\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        last_opt,
                    );
                }
                file_name_out = *argv.offset(i as isize);
                current_block_168 = 8258075665625361029;
            } else {
                last_opt = b"-W\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                if strncmp(
                    *argv.offset(i as isize),
                    b"-W\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                    || {
                        last_opt = b"--wam-for-native\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        strncmp(
                            *argv.offset(i as isize),
                            b"--wam-for-native\0" as *const u8 as *const libc::c_char,
                            strlen(*argv.offset(i as isize)),
                        ) == 0 as libc::c_int
                    }
                {
                    stop_after = 0 as libc::c_int;
                    bc_mode = 0 as libc::c_int;
                    current_block_168 = 8258075665625361029;
                } else {
                    last_opt = b"-w\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    if strncmp(
                        *argv.offset(i as isize),
                        b"-w\0" as *const u8 as *const libc::c_char,
                        strlen(*argv.offset(i as isize)),
                    ) == 0 as libc::c_int
                        || {
                            last_opt = b"--wam-for-byte-code\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            strncmp(
                                *argv.offset(i as isize),
                                b"--wam-for-byte-code\0" as *const u8
                                    as *const libc::c_char,
                                strlen(*argv.offset(i as isize)),
                            ) == 0 as libc::c_int
                        }
                    {
                        stop_after = 0 as libc::c_int;
                        bc_mode = 1 as libc::c_int;
                        current_block_168 = 8258075665625361029;
                    } else {
                        last_opt = b"-M\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        if strncmp(
                            *argv.offset(i as isize),
                            b"-M\0" as *const u8 as *const libc::c_char,
                            strlen(*argv.offset(i as isize)),
                        ) == 0 as libc::c_int
                            || {
                                last_opt = b"--mini-assembly\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char;
                                strncmp(
                                    *argv.offset(i as isize),
                                    b"--mini-assembly\0" as *const u8 as *const libc::c_char,
                                    strlen(*argv.offset(i as isize)),
                                ) == 0 as libc::c_int
                            }
                        {
                            stop_after = 1 as libc::c_int;
                            bc_mode = 0 as libc::c_int;
                            current_block_168 = 8258075665625361029;
                        } else {
                            last_opt = b"-S\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            if strncmp(
                                *argv.offset(i as isize),
                                b"-S\0" as *const u8 as *const libc::c_char,
                                strlen(*argv.offset(i as isize)),
                            ) == 0 as libc::c_int
                                || {
                                    last_opt = b"--assembly\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char;
                                    strncmp(
                                        *argv.offset(i as isize),
                                        b"--assembly\0" as *const u8 as *const libc::c_char,
                                        strlen(*argv.offset(i as isize)),
                                    ) == 0 as libc::c_int
                                }
                            {
                                stop_after = 2 as libc::c_int;
                                bc_mode = 0 as libc::c_int;
                                current_block_168 = 8258075665625361029;
                            } else {
                                last_opt = b"-c\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                                if strncmp(
                                    *argv.offset(i as isize),
                                    b"-c\0" as *const u8 as *const libc::c_char,
                                    strlen(*argv.offset(i as isize)),
                                ) == 0 as libc::c_int
                                    || {
                                        last_opt = b"--object\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char;
                                        strncmp(
                                            *argv.offset(i as isize),
                                            b"--object\0" as *const u8 as *const libc::c_char,
                                            strlen(*argv.offset(i as isize)),
                                        ) == 0 as libc::c_int
                                    }
                                {
                                    stop_after = 3 as libc::c_int;
                                    bc_mode = 0 as libc::c_int;
                                    current_block_168 = 8258075665625361029;
                                } else {
                                    last_opt = b"-F\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char;
                                    if strncmp(
                                        *argv.offset(i as isize),
                                        b"-F\0" as *const u8 as *const libc::c_char,
                                        strlen(*argv.offset(i as isize)),
                                    ) == 0 as libc::c_int
                                        || {
                                            last_opt = b"--fd-to-c\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char;
                                            strncmp(
                                                *argv.offset(i as isize),
                                                b"--fd-to-c\0" as *const u8 as *const libc::c_char,
                                                strlen(*argv.offset(i as isize)),
                                            ) == 0 as libc::c_int
                                        }
                                    {
                                        stop_after = 5 as libc::c_int;
                                        bc_mode = 0 as libc::c_int;
                                        current_block_168 = 8258075665625361029;
                                    } else {
                                        last_opt = b"--comment\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char;
                                        if strncmp(
                                            *argv.offset(i as isize),
                                            b"--comment\0" as *const u8 as *const libc::c_char,
                                            strlen(*argv.offset(i as isize)),
                                        ) == 0 as libc::c_int
                                        {
                                            sprintf(
                                                (cmd_wam2ma.opt)
                                                    .as_mut_ptr()
                                                    .offset(strlen((cmd_wam2ma.opt).as_mut_ptr()) as isize),
                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                last_opt,
                                            );
                                            sprintf(
                                                (cmd_ma2asm.opt)
                                                    .as_mut_ptr()
                                                    .offset(strlen((cmd_ma2asm.opt).as_mut_ptr()) as isize),
                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                last_opt,
                                            );
                                            current_block_168 = 8258075665625361029;
                                        } else {
                                            last_opt = b"--pic\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char;
                                            if strncmp(
                                                *argv.offset(i as isize),
                                                b"--pic\0" as *const u8 as *const libc::c_char,
                                                strlen(*argv.offset(i as isize)),
                                            ) == 0 as libc::c_int
                                                || {
                                                    last_opt = b"-fPIC\0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char;
                                                    strncmp(
                                                        *argv.offset(i as isize),
                                                        b"-fPIC\0" as *const u8 as *const libc::c_char,
                                                        strlen(*argv.offset(i as isize)),
                                                    ) == 0 as libc::c_int
                                                }
                                            {
                                                sprintf(
                                                    (cmd_ma2asm.opt)
                                                        .as_mut_ptr()
                                                        .offset(strlen((cmd_ma2asm.opt).as_mut_ptr()) as isize),
                                                    b"%s \0" as *const u8 as *const libc::c_char,
                                                    last_opt,
                                                );
                                                current_block_168 = 8258075665625361029;
                                            } else {
                                                last_opt = b"--temp-dir\0" as *const u8
                                                    as *const libc::c_char as *mut libc::c_char;
                                                if strncmp(
                                                    *argv.offset(i as isize),
                                                    b"--temp-dir\0" as *const u8 as *const libc::c_char,
                                                    strlen(*argv.offset(i as isize)),
                                                ) == 0 as libc::c_int
                                                {
                                                    i += 1;
                                                    if i >= argc {
                                                        Pl_Fatal_Error(
                                                            b"PATH missing after %s option\0" as *const u8
                                                                as *const libc::c_char as *mut libc::c_char,
                                                            last_opt,
                                                        );
                                                    }
                                                    temp_dir = *argv.offset(i as isize);
                                                    current_block_168 = 8258075665625361029;
                                                } else {
                                                    last_opt = b"--no-del-temp-files\0" as *const u8
                                                        as *const libc::c_char as *mut libc::c_char;
                                                    if strncmp(
                                                        *argv.offset(i as isize),
                                                        b"--no-del-temp-files\0" as *const u8
                                                            as *const libc::c_char,
                                                        strlen(*argv.offset(i as isize)),
                                                    ) == 0 as libc::c_int
                                                    {
                                                        no_del_temp_files = 1 as libc::c_int;
                                                        current_block_168 = 8258075665625361029;
                                                    } else {
                                                        last_opt = b"--no-decode-hexa\0" as *const u8
                                                            as *const libc::c_char as *mut libc::c_char;
                                                        if strncmp(
                                                            *argv.offset(i as isize),
                                                            b"--no-decode-hexa\0" as *const u8 as *const libc::c_char,
                                                            strlen(*argv.offset(i as isize)),
                                                        ) == 0 as libc::c_int
                                                            || {
                                                                last_opt = b"--no-demangling\0" as *const u8
                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                strncmp(
                                                                    *argv.offset(i as isize),
                                                                    b"--no-demangling\0" as *const u8 as *const libc::c_char,
                                                                    strlen(*argv.offset(i as isize)),
                                                                ) == 0 as libc::c_int
                                                            }
                                                        {
                                                            no_decode_hex = 1 as libc::c_int;
                                                            current_block_168 = 8258075665625361029;
                                                        } else {
                                                            last_opt = b"--version\0" as *const u8
                                                                as *const libc::c_char as *mut libc::c_char;
                                                            if strncmp(
                                                                *argv.offset(i as isize),
                                                                b"--version\0" as *const u8 as *const libc::c_char,
                                                                strlen(*argv.offset(i as isize)),
                                                            ) == 0 as libc::c_int
                                                                || {
                                                                    last_opt = b"-v\0" as *const u8 as *const libc::c_char
                                                                        as *mut libc::c_char;
                                                                    strncmp(
                                                                        *argv.offset(i as isize),
                                                                        b"-v\0" as *const u8 as *const libc::c_char,
                                                                        strlen(*argv.offset(i as isize)),
                                                                    ) == 0 as libc::c_int
                                                                }
                                                                || {
                                                                    last_opt = b"--verbose\0" as *const u8
                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                    strncmp(
                                                                        *argv.offset(i as isize),
                                                                        b"--verbose\0" as *const u8 as *const libc::c_char,
                                                                        strlen(*argv.offset(i as isize)),
                                                                    ) == 0 as libc::c_int
                                                                }
                                                            {
                                                                Display_Copying(
                                                                    b"Prolog compiler\0" as *const u8 as *const libc::c_char
                                                                        as *mut libc::c_char,
                                                                );
                                                                last_opt = b"--version\0" as *const u8
                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                if strncmp(
                                                                    *argv.offset(i as isize),
                                                                    b"--version\0" as *const u8 as *const libc::c_char,
                                                                    strlen(*argv.offset(i as isize)),
                                                                ) == 0 as libc::c_int
                                                                {
                                                                    exit(0 as libc::c_int);
                                                                }
                                                                verbose = 1 as libc::c_int;
                                                                current_block_168 = 8258075665625361029;
                                                            } else {
                                                                last_opt = b"-h\0" as *const u8 as *const libc::c_char
                                                                    as *mut libc::c_char;
                                                                if strncmp(
                                                                    *argv.offset(i as isize),
                                                                    b"-h\0" as *const u8 as *const libc::c_char,
                                                                    strlen(*argv.offset(i as isize)),
                                                                ) == 0 as libc::c_int
                                                                    || {
                                                                        last_opt = b"--help\0" as *const u8 as *const libc::c_char
                                                                            as *mut libc::c_char;
                                                                        strncmp(
                                                                            *argv.offset(i as isize),
                                                                            b"--help\0" as *const u8 as *const libc::c_char,
                                                                            strlen(*argv.offset(i as isize)),
                                                                        ) == 0 as libc::c_int
                                                                    }
                                                                {
                                                                    Display_Help();
                                                                    exit(0 as libc::c_int);
                                                                }
                                                                last_opt = b"--pl-state\0" as *const u8
                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                if strncmp(
                                                                    *argv.offset(i as isize),
                                                                    b"--pl-state\0" as *const u8 as *const libc::c_char,
                                                                    strlen(*argv.offset(i as isize)),
                                                                ) == 0 as libc::c_int
                                                                {
                                                                    i += 1;
                                                                    if i >= argc {
                                                                        Pl_Fatal_Error(
                                                                            b"FILE missing after %s option\0" as *const u8
                                                                                as *const libc::c_char as *mut libc::c_char,
                                                                            last_opt,
                                                                        );
                                                                    }
                                                                    if access(*argv.offset(i as isize), 4 as libc::c_int)
                                                                        != 0 as libc::c_int
                                                                    {
                                                                        perror(*argv.offset(i as isize));
                                                                        exit(1 as libc::c_int);
                                                                    }
                                                                    sprintf(
                                                                        (cmd_pl2wam.opt)
                                                                            .as_mut_ptr()
                                                                            .offset(strlen((cmd_pl2wam.opt).as_mut_ptr()) as isize),
                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                        last_opt,
                                                                    );
                                                                    last_opt = *argv.offset(i as isize);
                                                                    sprintf(
                                                                        (cmd_pl2wam.opt)
                                                                            .as_mut_ptr()
                                                                            .offset(strlen((cmd_pl2wam.opt).as_mut_ptr()) as isize),
                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                        last_opt,
                                                                    );
                                                                    current_block_168 = 8258075665625361029;
                                                                } else {
                                                                    last_opt = b"--wam-comment\0" as *const u8
                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                    if strncmp(
                                                                        *argv.offset(i as isize),
                                                                        b"--wam-comment\0" as *const u8 as *const libc::c_char,
                                                                        strlen(*argv.offset(i as isize)),
                                                                    ) == 0 as libc::c_int
                                                                    {
                                                                        i += 1;
                                                                        if i >= argc {
                                                                            Pl_Fatal_Error(
                                                                                b"COMMENT missing after %s option\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char,
                                                                                last_opt,
                                                                            );
                                                                        }
                                                                        sprintf(
                                                                            (cmd_pl2wam.opt)
                                                                                .as_mut_ptr()
                                                                                .offset(strlen((cmd_pl2wam.opt).as_mut_ptr()) as isize),
                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                            last_opt,
                                                                        );
                                                                        last_opt = *argv.offset(i as isize);
                                                                        sprintf(
                                                                            (cmd_pl2wam.opt)
                                                                                .as_mut_ptr()
                                                                                .offset(strlen((cmd_pl2wam.opt).as_mut_ptr()) as isize),
                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                            last_opt,
                                                                        );
                                                                        current_block_168 = 8258075665625361029;
                                                                    } else {
                                                                        last_opt = b"--no-susp-warn\0" as *const u8
                                                                            as *const libc::c_char as *mut libc::c_char;
                                                                        if strncmp(
                                                                            *argv.offset(i as isize),
                                                                            b"--no-susp-warn\0" as *const u8 as *const libc::c_char,
                                                                            strlen(*argv.offset(i as isize)),
                                                                        ) == 0 as libc::c_int
                                                                            || {
                                                                                last_opt = b"--no-singl-warn\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--no-singl-warn\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--no-redef-error\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--no-redef-error\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--foreign-only\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--foreign-only\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--no-call-c\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--no-call-c\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--no-inline\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--no-inline\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--no-reorder\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--no-reorder\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--no-reg-opt\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--no-reg-opt\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--min-reg-opt\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--min-reg-opt\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--no-opt-last-subterm\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--no-opt-last-subterm\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--fast-math\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--fast-math\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--keep-void-inst\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--keep-void-inst\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--compile-msg\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--compile-msg\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                            || {
                                                                                last_opt = b"--statistics\0" as *const u8
                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--statistics\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                            }
                                                                        {
                                                                            sprintf(
                                                                                (cmd_pl2wam.opt)
                                                                                    .as_mut_ptr()
                                                                                    .offset(strlen((cmd_pl2wam.opt).as_mut_ptr()) as isize),
                                                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                                                last_opt,
                                                                            );
                                                                            current_block_168 = 8258075665625361029;
                                                                        } else {
                                                                            last_opt = b"--c-compiler\0" as *const u8
                                                                                as *const libc::c_char as *mut libc::c_char;
                                                                            if strncmp(
                                                                                *argv.offset(i as isize),
                                                                                b"--c-compiler\0" as *const u8 as *const libc::c_char,
                                                                                strlen(*argv.offset(i as isize)),
                                                                            ) == 0 as libc::c_int
                                                                            {
                                                                                i += 1;
                                                                                if i >= argc {
                                                                                    Pl_Fatal_Error(
                                                                                        b"FILE missing after %s option\0" as *const u8
                                                                                            as *const libc::c_char as *mut libc::c_char,
                                                                                        last_opt,
                                                                                    );
                                                                                }
                                                                                cmd_cc.exe_name = *argv.offset(i as isize);
                                                                                if strcmp(
                                                                                    cmd_link.exe_name,
                                                                                    b"gcc\0" as *const u8 as *const libc::c_char,
                                                                                ) == 0 as libc::c_int
                                                                                {
                                                                                    cmd_link.exe_name = *argv.offset(i as isize);
                                                                                }
                                                                                current_block_168 = 8258075665625361029;
                                                                            } else {
                                                                                last_opt = b"--linker\0" as *const u8 as *const libc::c_char
                                                                                    as *mut libc::c_char;
                                                                                if strncmp(
                                                                                    *argv.offset(i as isize),
                                                                                    b"--linker\0" as *const u8 as *const libc::c_char,
                                                                                    strlen(*argv.offset(i as isize)),
                                                                                ) == 0 as libc::c_int
                                                                                {
                                                                                    i += 1;
                                                                                    if i >= argc {
                                                                                        Pl_Fatal_Error(
                                                                                            b"FILE missing after %s option\0" as *const u8
                                                                                                as *const libc::c_char as *mut libc::c_char,
                                                                                            last_opt,
                                                                                        );
                                                                                    }
                                                                                    cmd_link.exe_name = *argv.offset(i as isize);
                                                                                    current_block_168 = 8258075665625361029;
                                                                                } else {
                                                                                    last_opt = b"-C\0" as *const u8 as *const libc::c_char
                                                                                        as *mut libc::c_char;
                                                                                    if strncmp(
                                                                                        *argv.offset(i as isize),
                                                                                        b"-C\0" as *const u8 as *const libc::c_char,
                                                                                        strlen(*argv.offset(i as isize)),
                                                                                    ) == 0 as libc::c_int
                                                                                    {
                                                                                        i += 1;
                                                                                        if i >= argc {
                                                                                            Pl_Fatal_Error(
                                                                                                b"OPTION missing after %s option\0" as *const u8
                                                                                                    as *const libc::c_char as *mut libc::c_char,
                                                                                                last_opt,
                                                                                            );
                                                                                        }
                                                                                        sprintf(
                                                                                            (cmd_cc.opt)
                                                                                                .as_mut_ptr()
                                                                                                .offset(strlen((cmd_cc.opt).as_mut_ptr()) as isize),
                                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                                            *argv.offset(i as isize),
                                                                                        );
                                                                                        cc_fd2c_flags = b"\0" as *const u8 as *const libc::c_char
                                                                                            as *mut libc::c_char;
                                                                                        current_block_168 = 8258075665625361029;
                                                                                    } else {
                                                                                        last_opt = b"-A\0" as *const u8 as *const libc::c_char
                                                                                            as *mut libc::c_char;
                                                                                        if strncmp(
                                                                                            *argv.offset(i as isize),
                                                                                            b"-A\0" as *const u8 as *const libc::c_char,
                                                                                            strlen(*argv.offset(i as isize)),
                                                                                        ) == 0 as libc::c_int
                                                                                        {
                                                                                            i += 1;
                                                                                            if i >= argc {
                                                                                                Pl_Fatal_Error(
                                                                                                    b"OPTION missing after %s option\0" as *const u8
                                                                                                        as *const libc::c_char as *mut libc::c_char,
                                                                                                    last_opt,
                                                                                                );
                                                                                            }
                                                                                            sprintf(
                                                                                                (cmd_asm.opt)
                                                                                                    .as_mut_ptr()
                                                                                                    .offset(strlen((cmd_asm.opt).as_mut_ptr()) as isize),
                                                                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                *argv.offset(i as isize),
                                                                                            );
                                                                                            current_block_168 = 8258075665625361029;
                                                                                        } else {
                                                                                            last_opt = b"--local-size\0" as *const u8
                                                                                                as *const libc::c_char as *mut libc::c_char;
                                                                                            if strncmp(
                                                                                                *argv.offset(i as isize),
                                                                                                b"--local-size\0" as *const u8 as *const libc::c_char,
                                                                                                strlen(*argv.offset(i as isize)),
                                                                                            ) == 0 as libc::c_int
                                                                                            {
                                                                                                sprintf(
                                                                                                    warn_str
                                                                                                        .as_mut_ptr()
                                                                                                        .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                    b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                    *argv.offset(i as isize),
                                                                                                );
                                                                                                i += 1;
                                                                                                if i >= argc {
                                                                                                    Pl_Fatal_Error(
                                                                                                        b"SIZE missing after %s option\0" as *const u8
                                                                                                            as *const libc::c_char as *mut libc::c_char,
                                                                                                        last_opt,
                                                                                                    );
                                                                                                }
                                                                                                pl_def_local_size = strtol(
                                                                                                    *argv.offset(i as isize),
                                                                                                    &mut q,
                                                                                                    10 as libc::c_int,
                                                                                                ) as libc::c_int;
                                                                                                if *q as libc::c_int != 0
                                                                                                    || pl_def_local_size < 0 as libc::c_int
                                                                                                {
                                                                                                    Pl_Fatal_Error(
                                                                                                        b"invalid stack size (%s)\0" as *const u8
                                                                                                            as *const libc::c_char as *mut libc::c_char,
                                                                                                        *argv.offset(i as isize),
                                                                                                    );
                                                                                                }
                                                                                                sprintf(
                                                                                                    warn_str
                                                                                                        .as_mut_ptr()
                                                                                                        .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                    b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                    *argv.offset(i as isize),
                                                                                                );
                                                                                                needs_stack_file = 1 as libc::c_int;
                                                                                                current_block_168 = 8258075665625361029;
                                                                                            } else {
                                                                                                last_opt = b"--global-size\0" as *const u8
                                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                                if strncmp(
                                                                                                    *argv.offset(i as isize),
                                                                                                    b"--global-size\0" as *const u8 as *const libc::c_char,
                                                                                                    strlen(*argv.offset(i as isize)),
                                                                                                ) == 0 as libc::c_int
                                                                                                {
                                                                                                    sprintf(
                                                                                                        warn_str
                                                                                                            .as_mut_ptr()
                                                                                                            .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                        *argv.offset(i as isize),
                                                                                                    );
                                                                                                    i += 1;
                                                                                                    if i >= argc {
                                                                                                        Pl_Fatal_Error(
                                                                                                            b"SIZE missing after %s option\0" as *const u8
                                                                                                                as *const libc::c_char as *mut libc::c_char,
                                                                                                            last_opt,
                                                                                                        );
                                                                                                    }
                                                                                                    pl_def_global_size = strtol(
                                                                                                        *argv.offset(i as isize),
                                                                                                        &mut q,
                                                                                                        10 as libc::c_int,
                                                                                                    ) as libc::c_int;
                                                                                                    if *q as libc::c_int != 0
                                                                                                        || pl_def_global_size < 0 as libc::c_int
                                                                                                    {
                                                                                                        Pl_Fatal_Error(
                                                                                                            b"invalid stack size (%s)\0" as *const u8
                                                                                                                as *const libc::c_char as *mut libc::c_char,
                                                                                                            *argv.offset(i as isize),
                                                                                                        );
                                                                                                    }
                                                                                                    sprintf(
                                                                                                        warn_str
                                                                                                            .as_mut_ptr()
                                                                                                            .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                        *argv.offset(i as isize),
                                                                                                    );
                                                                                                    needs_stack_file = 1 as libc::c_int;
                                                                                                    current_block_168 = 8258075665625361029;
                                                                                                } else {
                                                                                                    last_opt = b"--trail-size\0" as *const u8
                                                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                                                    if strncmp(
                                                                                                        *argv.offset(i as isize),
                                                                                                        b"--trail-size\0" as *const u8 as *const libc::c_char,
                                                                                                        strlen(*argv.offset(i as isize)),
                                                                                                    ) == 0 as libc::c_int
                                                                                                    {
                                                                                                        sprintf(
                                                                                                            warn_str
                                                                                                                .as_mut_ptr()
                                                                                                                .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                            *argv.offset(i as isize),
                                                                                                        );
                                                                                                        i += 1;
                                                                                                        if i >= argc {
                                                                                                            Pl_Fatal_Error(
                                                                                                                b"SIZE missing after %s option\0" as *const u8
                                                                                                                    as *const libc::c_char as *mut libc::c_char,
                                                                                                                last_opt,
                                                                                                            );
                                                                                                        }
                                                                                                        pl_def_trail_size = strtol(
                                                                                                            *argv.offset(i as isize),
                                                                                                            &mut q,
                                                                                                            10 as libc::c_int,
                                                                                                        ) as libc::c_int;
                                                                                                        if *q as libc::c_int != 0
                                                                                                            || pl_def_trail_size < 0 as libc::c_int
                                                                                                        {
                                                                                                            Pl_Fatal_Error(
                                                                                                                b"invalid stack size (%s)\0" as *const u8
                                                                                                                    as *const libc::c_char as *mut libc::c_char,
                                                                                                                *argv.offset(i as isize),
                                                                                                            );
                                                                                                        }
                                                                                                        sprintf(
                                                                                                            warn_str
                                                                                                                .as_mut_ptr()
                                                                                                                .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                            *argv.offset(i as isize),
                                                                                                        );
                                                                                                        needs_stack_file = 1 as libc::c_int;
                                                                                                        current_block_168 = 8258075665625361029;
                                                                                                    } else {
                                                                                                        last_opt = b"--cstr-size\0" as *const u8
                                                                                                            as *const libc::c_char as *mut libc::c_char;
                                                                                                        if strncmp(
                                                                                                            *argv.offset(i as isize),
                                                                                                            b"--cstr-size\0" as *const u8 as *const libc::c_char,
                                                                                                            strlen(*argv.offset(i as isize)),
                                                                                                        ) == 0 as libc::c_int
                                                                                                        {
                                                                                                            sprintf(
                                                                                                                warn_str
                                                                                                                    .as_mut_ptr()
                                                                                                                    .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                *argv.offset(i as isize),
                                                                                                            );
                                                                                                            i += 1;
                                                                                                            if i >= argc {
                                                                                                                Pl_Fatal_Error(
                                                                                                                    b"SIZE missing after %s option\0" as *const u8
                                                                                                                        as *const libc::c_char as *mut libc::c_char,
                                                                                                                    last_opt,
                                                                                                                );
                                                                                                            }
                                                                                                            pl_def_cstr_size = strtol(
                                                                                                                *argv.offset(i as isize),
                                                                                                                &mut q,
                                                                                                                10 as libc::c_int,
                                                                                                            ) as libc::c_int;
                                                                                                            if *q as libc::c_int != 0
                                                                                                                || pl_def_cstr_size < 0 as libc::c_int
                                                                                                            {
                                                                                                                Pl_Fatal_Error(
                                                                                                                    b"invalid stack size (%s)\0" as *const u8
                                                                                                                        as *const libc::c_char as *mut libc::c_char,
                                                                                                                    *argv.offset(i as isize),
                                                                                                                );
                                                                                                            }
                                                                                                            sprintf(
                                                                                                                warn_str
                                                                                                                    .as_mut_ptr()
                                                                                                                    .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                *argv.offset(i as isize),
                                                                                                            );
                                                                                                            needs_stack_file = 1 as libc::c_int;
                                                                                                            current_block_168 = 8258075665625361029;
                                                                                                        } else {
                                                                                                            last_opt = b"--max-atom\0" as *const u8
                                                                                                                as *const libc::c_char as *mut libc::c_char;
                                                                                                            if strncmp(
                                                                                                                *argv.offset(i as isize),
                                                                                                                b"--max-atom\0" as *const u8 as *const libc::c_char,
                                                                                                                strlen(*argv.offset(i as isize)),
                                                                                                            ) == 0 as libc::c_int
                                                                                                            {
                                                                                                                sprintf(
                                                                                                                    warn_str
                                                                                                                        .as_mut_ptr()
                                                                                                                        .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                    b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                    *argv.offset(i as isize),
                                                                                                                );
                                                                                                                i += 1;
                                                                                                                if i >= argc {
                                                                                                                    Pl_Fatal_Error(
                                                                                                                        b"SIZE missing after %s option\0" as *const u8
                                                                                                                            as *const libc::c_char as *mut libc::c_char,
                                                                                                                        last_opt,
                                                                                                                    );
                                                                                                                }
                                                                                                                pl_def_max_atom = strtol(
                                                                                                                    *argv.offset(i as isize),
                                                                                                                    &mut q,
                                                                                                                    10 as libc::c_int,
                                                                                                                ) as libc::c_int;
                                                                                                                if *q as libc::c_int != 0
                                                                                                                    || pl_def_max_atom < 0 as libc::c_int
                                                                                                                {
                                                                                                                    Pl_Fatal_Error(
                                                                                                                        b"invalid max atom (%s)\0" as *const u8
                                                                                                                            as *const libc::c_char as *mut libc::c_char,
                                                                                                                        *argv.offset(i as isize),
                                                                                                                    );
                                                                                                                }
                                                                                                                sprintf(
                                                                                                                    warn_str
                                                                                                                        .as_mut_ptr()
                                                                                                                        .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                    b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                    *argv.offset(i as isize),
                                                                                                                );
                                                                                                                needs_stack_file = 1 as libc::c_int;
                                                                                                                current_block_168 = 8258075665625361029;
                                                                                                            } else {
                                                                                                                last_opt = b"--fixed-sizes\0" as *const u8
                                                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                                                if strncmp(
                                                                                                                    *argv.offset(i as isize),
                                                                                                                    b"--fixed-sizes\0" as *const u8 as *const libc::c_char,
                                                                                                                    strlen(*argv.offset(i as isize)),
                                                                                                                ) == 0 as libc::c_int
                                                                                                                {
                                                                                                                    sprintf(
                                                                                                                        warn_str
                                                                                                                            .as_mut_ptr()
                                                                                                                            .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                        *argv.offset(i as isize),
                                                                                                                    );
                                                                                                                    pl_fixed_sizes = 1 as libc::c_int;
                                                                                                                    needs_stack_file = 1 as libc::c_int;
                                                                                                                    current_block_168 = 8258075665625361029;
                                                                                                                } else {
                                                                                                                    last_opt = b"--new-top-level\0" as *const u8
                                                                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                                                                    if strncmp(
                                                                                                                        *argv.offset(i as isize),
                                                                                                                        b"--new-top-level\0" as *const u8 as *const libc::c_char,
                                                                                                                        strlen(*argv.offset(i as isize)),
                                                                                                                    ) == 0 as libc::c_int
                                                                                                                    {
                                                                                                                        sprintf(
                                                                                                                            warn_str
                                                                                                                                .as_mut_ptr()
                                                                                                                                .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                            *argv.offset(i as isize),
                                                                                                                        );
                                                                                                                        no_top_level = 0 as libc::c_int;
                                                                                                                        no_debugger = 0 as libc::c_int;
                                                                                                                        new_top_level = 1 as libc::c_int;
                                                                                                                        current_block_168 = 8258075665625361029;
                                                                                                                    } else {
                                                                                                                        last_opt = b"--no-top-level\0" as *const u8
                                                                                                                            as *const libc::c_char as *mut libc::c_char;
                                                                                                                        if strncmp(
                                                                                                                            *argv.offset(i as isize),
                                                                                                                            b"--no-top-level\0" as *const u8 as *const libc::c_char,
                                                                                                                            strlen(*argv.offset(i as isize)),
                                                                                                                        ) == 0 as libc::c_int
                                                                                                                        {
                                                                                                                            sprintf(
                                                                                                                                warn_str
                                                                                                                                    .as_mut_ptr()
                                                                                                                                    .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                *argv.offset(i as isize),
                                                                                                                            );
                                                                                                                            no_top_level = 1 as libc::c_int;
                                                                                                                            no_debugger = 1 as libc::c_int;
                                                                                                                            new_top_level = 0 as libc::c_int;
                                                                                                                            current_block_168 = 8258075665625361029;
                                                                                                                        } else {
                                                                                                                            last_opt = b"--gui-console\0" as *const u8
                                                                                                                                as *const libc::c_char as *mut libc::c_char;
                                                                                                                            if strncmp(
                                                                                                                                *argv.offset(i as isize),
                                                                                                                                b"--gui-console\0" as *const u8 as *const libc::c_char,
                                                                                                                                strlen(*argv.offset(i as isize)),
                                                                                                                            ) == 0 as libc::c_int
                                                                                                                            {
                                                                                                                                fprintf(
                                                                                                                                    stderr,
                                                                                                                                    b"Warning: Win32 GUI Console not available\n\0" as *const u8
                                                                                                                                        as *const libc::c_char,
                                                                                                                                );
                                                                                                                                current_block_168 = 8258075665625361029;
                                                                                                                            } else {
                                                                                                                                last_opt = b"--no-debugger\0" as *const u8
                                                                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                                                                if strncmp(
                                                                                                                                    *argv.offset(i as isize),
                                                                                                                                    b"--no-debugger\0" as *const u8 as *const libc::c_char,
                                                                                                                                    strlen(*argv.offset(i as isize)),
                                                                                                                                ) == 0 as libc::c_int
                                                                                                                                {
                                                                                                                                    sprintf(
                                                                                                                                        warn_str
                                                                                                                                            .as_mut_ptr()
                                                                                                                                            .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                    );
                                                                                                                                    no_debugger = 1 as libc::c_int;
                                                                                                                                    current_block_168 = 8258075665625361029;
                                                                                                                                } else {
                                                                                                                                    last_opt = b"--min-pl-bips\0" as *const u8
                                                                                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                                                                                    if strncmp(
                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                        b"--min-pl-bips\0" as *const u8 as *const libc::c_char,
                                                                                                                                        strlen(*argv.offset(i as isize)),
                                                                                                                                    ) == 0 as libc::c_int
                                                                                                                                    {
                                                                                                                                        sprintf(
                                                                                                                                            warn_str
                                                                                                                                                .as_mut_ptr()
                                                                                                                                                .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                            *argv.offset(i as isize),
                                                                                                                                        );
                                                                                                                                        min_pl_bips = 1 as libc::c_int;
                                                                                                                                        current_block_168 = 8258075665625361029;
                                                                                                                                    } else {
                                                                                                                                        last_opt = b"--min-fd-bips\0" as *const u8
                                                                                                                                            as *const libc::c_char as *mut libc::c_char;
                                                                                                                                        if strncmp(
                                                                                                                                            *argv.offset(i as isize),
                                                                                                                                            b"--min-fd-bips\0" as *const u8 as *const libc::c_char,
                                                                                                                                            strlen(*argv.offset(i as isize)),
                                                                                                                                        ) == 0 as libc::c_int
                                                                                                                                        {
                                                                                                                                            sprintf(
                                                                                                                                                warn_str
                                                                                                                                                    .as_mut_ptr()
                                                                                                                                                    .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                *argv.offset(i as isize),
                                                                                                                                            );
                                                                                                                                            min_fd_bips = 1 as libc::c_int;
                                                                                                                                            current_block_168 = 8258075665625361029;
                                                                                                                                        } else {
                                                                                                                                            last_opt = b"--min-bips\0" as *const u8
                                                                                                                                                as *const libc::c_char as *mut libc::c_char;
                                                                                                                                            if strncmp(
                                                                                                                                                *argv.offset(i as isize),
                                                                                                                                                b"--min-bips\0" as *const u8 as *const libc::c_char,
                                                                                                                                                strlen(*argv.offset(i as isize)),
                                                                                                                                            ) == 0 as libc::c_int
                                                                                                                                                || {
                                                                                                                                                    last_opt = b"--min-size\0" as *const u8
                                                                                                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                                                                                                    strncmp(
                                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                                        b"--min-size\0" as *const u8 as *const libc::c_char,
                                                                                                                                                        strlen(*argv.offset(i as isize)),
                                                                                                                                                    ) == 0 as libc::c_int
                                                                                                                                                }
                                                                                                                                            {
                                                                                                                                                sprintf(
                                                                                                                                                    warn_str
                                                                                                                                                        .as_mut_ptr()
                                                                                                                                                        .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                    b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                    *argv.offset(i as isize),
                                                                                                                                                );
                                                                                                                                                min_fd_bips = 1 as libc::c_int;
                                                                                                                                                min_pl_bips = min_fd_bips;
                                                                                                                                                no_debugger = min_pl_bips;
                                                                                                                                                no_top_level = no_debugger;
                                                                                                                                                new_top_level = 0 as libc::c_int;
                                                                                                                                                last_opt = b"--min-size\0" as *const u8
                                                                                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                                                                                if strncmp(
                                                                                                                                                    *argv.offset(i as isize),
                                                                                                                                                    b"--min-size\0" as *const u8 as *const libc::c_char,
                                                                                                                                                    strlen(*argv.offset(i as isize)),
                                                                                                                                                ) == 0 as libc::c_int
                                                                                                                                                {
                                                                                                                                                    strip = 1 as libc::c_int;
                                                                                                                                                }
                                                                                                                                                current_block_168 = 8258075665625361029;
                                                                                                                                            } else {
                                                                                                                                                last_opt = b"--no-pl-lib\0" as *const u8
                                                                                                                                                    as *const libc::c_char as *mut libc::c_char;
                                                                                                                                                if strncmp(
                                                                                                                                                    *argv.offset(i as isize),
                                                                                                                                                    b"--no-pl-lib\0" as *const u8 as *const libc::c_char,
                                                                                                                                                    strlen(*argv.offset(i as isize)),
                                                                                                                                                ) == 0 as libc::c_int
                                                                                                                                                {
                                                                                                                                                    sprintf(
                                                                                                                                                        warn_str
                                                                                                                                                            .as_mut_ptr()
                                                                                                                                                            .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                                    );
                                                                                                                                                    no_fd_lib = 1 as libc::c_int;
                                                                                                                                                    no_pl_lib = no_fd_lib;
                                                                                                                                                    min_fd_bips = 1 as libc::c_int;
                                                                                                                                                    min_pl_bips = min_fd_bips;
                                                                                                                                                    no_debugger = min_pl_bips;
                                                                                                                                                    no_top_level = no_debugger;
                                                                                                                                                    new_top_level = 0 as libc::c_int;
                                                                                                                                                    current_block_168 = 8258075665625361029;
                                                                                                                                                } else {
                                                                                                                                                    last_opt = b"--no-fd-lib\0" as *const u8
                                                                                                                                                        as *const libc::c_char as *mut libc::c_char;
                                                                                                                                                    if strncmp(
                                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                                        b"--no-fd-lib\0" as *const u8 as *const libc::c_char,
                                                                                                                                                        strlen(*argv.offset(i as isize)),
                                                                                                                                                    ) == 0 as libc::c_int
                                                                                                                                                    {
                                                                                                                                                        sprintf(
                                                                                                                                                            warn_str
                                                                                                                                                                .as_mut_ptr()
                                                                                                                                                                .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                            b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                            *argv.offset(i as isize),
                                                                                                                                                        );
                                                                                                                                                        min_fd_bips = 1 as libc::c_int;
                                                                                                                                                        no_fd_lib = min_fd_bips;
                                                                                                                                                        current_block_168 = 8258075665625361029;
                                                                                                                                                    } else {
                                                                                                                                                        last_opt = b"--no-fd-lib-warn\0" as *const u8
                                                                                                                                                            as *const libc::c_char as *mut libc::c_char;
                                                                                                                                                        if strncmp(
                                                                                                                                                            *argv.offset(i as isize),
                                                                                                                                                            b"--no-fd-lib-warn\0" as *const u8 as *const libc::c_char,
                                                                                                                                                            strlen(*argv.offset(i as isize)),
                                                                                                                                                        ) == 0 as libc::c_int
                                                                                                                                                        {
                                                                                                                                                            sprintf(
                                                                                                                                                                warn_str
                                                                                                                                                                    .as_mut_ptr()
                                                                                                                                                                    .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                                b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                                *argv.offset(i as isize),
                                                                                                                                                            );
                                                                                                                                                            no_fd_lib_warn = 1 as libc::c_int;
                                                                                                                                                            current_block_168 = 8258075665625361029;
                                                                                                                                                        } else {
                                                                                                                                                            last_opt = b"-s\0" as *const u8 as *const libc::c_char
                                                                                                                                                                as *mut libc::c_char;
                                                                                                                                                            if strncmp(
                                                                                                                                                                *argv.offset(i as isize),
                                                                                                                                                                b"-s\0" as *const u8 as *const libc::c_char,
                                                                                                                                                                strlen(*argv.offset(i as isize)),
                                                                                                                                                            ) == 0 as libc::c_int
                                                                                                                                                                || {
                                                                                                                                                                    last_opt = b"--strip\0" as *const u8 as *const libc::c_char
                                                                                                                                                                        as *mut libc::c_char;
                                                                                                                                                                    strncmp(
                                                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                                                        b"--strip\0" as *const u8 as *const libc::c_char,
                                                                                                                                                                        strlen(*argv.offset(i as isize)),
                                                                                                                                                                    ) == 0 as libc::c_int
                                                                                                                                                                }
                                                                                                                                                            {
                                                                                                                                                                sprintf(
                                                                                                                                                                    warn_str
                                                                                                                                                                        .as_mut_ptr()
                                                                                                                                                                        .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                                    b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                                    *argv.offset(i as isize),
                                                                                                                                                                );
                                                                                                                                                                strip = 1 as libc::c_int;
                                                                                                                                                                current_block_168 = 8258075665625361029;
                                                                                                                                                            } else {
                                                                                                                                                                last_opt = b"-L\0" as *const u8 as *const libc::c_char
                                                                                                                                                                    as *mut libc::c_char;
                                                                                                                                                                if strncmp(
                                                                                                                                                                    *argv.offset(i as isize),
                                                                                                                                                                    b"-L\0" as *const u8 as *const libc::c_char,
                                                                                                                                                                    strlen(*argv.offset(i as isize)),
                                                                                                                                                                ) == 0 as libc::c_int
                                                                                                                                                                {
                                                                                                                                                                    sprintf(
                                                                                                                                                                        warn_str
                                                                                                                                                                            .as_mut_ptr()
                                                                                                                                                                            .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                                                    );
                                                                                                                                                                    i += 1;
                                                                                                                                                                    if i >= argc {
                                                                                                                                                                        Pl_Fatal_Error(
                                                                                                                                                                            b"OPTION missing after %s option\0" as *const u8
                                                                                                                                                                                as *const libc::c_char as *mut libc::c_char,
                                                                                                                                                                            last_opt,
                                                                                                                                                                        );
                                                                                                                                                                    }
                                                                                                                                                                    sprintf(
                                                                                                                                                                        warn_str
                                                                                                                                                                            .as_mut_ptr()
                                                                                                                                                                            .offset(strlen(warn_str.as_mut_ptr()) as isize),
                                                                                                                                                                        b"%s \0" as *const u8 as *const libc::c_char,
                                                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                                                    );
                                                                                                                                                                    (*f).work_name1 = *argv.offset(i as isize);
                                                                                                                                                                    (*f).name = (*f).work_name1;
                                                                                                                                                                    (*f).type_0 = 8 as libc::c_int;
                                                                                                                                                                    nb_file_lopt += 1;
                                                                                                                                                                    nb_file_lopt;
                                                                                                                                                                    f = f.offset(1);
                                                                                                                                                                    f;
                                                                                                                                                                    current_block_168 = 8258075665625361029;
                                                                                                                                                                } else {
                                                                                                                                                                    Pl_Fatal_Error(
                                                                                                                                                                        b"unknown option %s - try %s --help\0" as *const u8
                                                                                                                                                                            as *const libc::c_char as *mut libc::c_char,
                                                                                                                                                                        *argv.offset(i as isize),
                                                                                                                                                                        b"gplc\0" as *const u8 as *const libc::c_char,
                                                                                                                                                                    );
                                                                                                                                                                    current_block_168 = 2544535129495155983;
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current_block_168 = 2544535129495155983;
        }
        match current_block_168 {
            2544535129495155983 => {
                (*f).name = *argv.offset(i as isize);
                (*f).suffix = strrchr(*argv.offset(i as isize), '.' as i32);
                if ((*f).suffix).is_null() {
                    (*f)
                        .suffix = (*argv.offset(i as isize))
                        .offset(strlen(*argv.offset(i as isize)) as isize);
                }
                q = (*f).suffix;
                while q >= (*f).name {
                    if *q as libc::c_int == '/' as i32 {
                        break;
                    }
                    q = q.offset(-1);
                    q;
                }
                (*f).file_part = q.offset(1 as libc::c_int as isize);
                if !(Find_Suffix(
                    b"|.pro|.prolog|\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*f).suffix,
                ))
                    .is_null()
                {
                    (*f).type_0 = 0 as libc::c_int;
                } else if !(Find_Suffix(
                    b"|.pro|.prolog|\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*f).suffix,
                ))
                    .is_null()
                {
                    (*f).type_0 = 6 as libc::c_int;
                } else {
                    (*f).type_0 = 7 as libc::c_int;
                    p = suffixes.as_mut_ptr();
                    while !(*p).is_null() {
                        if strcasecmp(*p, (*f).suffix) == 0 as libc::c_int {
                            (*f)
                                .type_0 = p.offset_from(suffixes.as_mut_ptr())
                                as libc::c_long as libc::c_int;
                            break;
                        } else {
                            p = p.offset(1);
                            p;
                        }
                    }
                }
                (*f).work_name1 = (*f).name;
                (*f).work_name2 = 0 as *mut libc::c_char;
                if (*f).type_0 != 7 as libc::c_int
                    && access((*f).name, 4 as libc::c_int) != 0 as libc::c_int
                {
                    perror((*f).name);
                    exit(1 as libc::c_int);
                }
                nb_file_lopt += 1;
                nb_file_lopt;
                nb_file += 1;
                nb_file;
                f = f.offset(1);
                f;
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if no_top_level != 0 {
        new_top_level = 0 as libc::c_int;
    }
    if f == file_lopt {
        if verbose != 0 {
            exit(0 as libc::c_int);
        } else {
            Pl_Fatal_Error(
                b"no input file specified\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    (*f).name = 0 as *mut libc::c_char;
    if nb_file > 1 as libc::c_int && stop_after < 7 as libc::c_int
        && !file_name_out.is_null() && (strchr(file_name_out, '%' as i32)).is_null()
    {
        fprintf(
            stderr,
            b"named output file ignored with multiples output (or use meta-characters, e.g. %%p)\n\0"
                as *const u8 as *const libc::c_char,
        );
        sprintf(
            warn_str.as_mut_ptr().offset(strlen(warn_str.as_mut_ptr()) as isize),
            b"%s \0" as *const u8 as *const libc::c_char,
            *argv.offset(file_name_out_i as isize),
        );
        sprintf(
            warn_str.as_mut_ptr().offset(strlen(warn_str.as_mut_ptr()) as isize),
            b"%s \0" as *const u8 as *const libc::c_char,
            *argv.offset((file_name_out_i + 1 as libc::c_int) as isize),
        );
        file_name_out = 0 as *mut libc::c_char;
    }
}
pub unsafe extern "C" fn Display_Help() {
    fprintf(
        stderr,
        b"Usage: %s [OPTION]... FILE...\n\0" as *const u8 as *const libc::c_char,
        b"gplc\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"General options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -o FILE, --output FILE      set output file name (see below)\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -W, --wam-for-native        stop after producing WAM file(s)\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -w, --wam-for-byte-code     stop after producing WAM for byte-code file(s) (force --no-call-c)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -M, --mini-assembly         stop after producing mini-assembly file(s)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -S, --assembly              stop after producing assembly file(s)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -F, --fd-to-c               stop after producing C file(s) from FD file(s)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -c, --object                stop after producing object file(s)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --temp-dir PATH             use PATH as directory for temporary files\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-del-temp-files         do not delete temporary files\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-demangling             do not decode hexadecimal predicate names\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-decode-hexa            same as --no-demanling (deprecated)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -v, --verbose               print executed commands\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -h, --help                  print this help and exit\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --version                   print version number and exit\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Prolog to WAM compiler options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --pl-state FILE             read FILE to set the initial Prolog state\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --wam-comment COMMENT       emit COMMENT as a comment in the WAM file\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-susp-warn              do not show warnings for suspicious predicates\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-singl-warn             do not show warnings for named singleton variables\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-redef-error            do not show errors for built-in redefinitions\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --foreign-only              only compile foreign/1-2 directives\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-call-c                 do not allow the use of fd_tell, '$call_c',...\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-inline                 do not inline predicates\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-reorder                do not reorder predicate arguments\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-reg-opt                do not optimize registers\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --min-reg-opt               minimally optimize registers\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-opt-last-subterm       do not optimize last subterm compilation\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --fast-math                 fast mathematical mode (assume integer arithmetics)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --keep-void-inst            keep void instructions in the output file\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --compile-msg               print a compile message\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --statistics                print statistics information\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"WAM to mini-assembly translator options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --comment                   include comments in the output file\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Mini-assembly to assembly translator options:\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --comment                   include comments in the output file\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --pic                       produce position independent code (PIC)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"C Compiler options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --c-compiler FILE           use FILE as C compiler/linker\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -C OPTION                   pass OPTION to the C compiler\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Assembler options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -A OPTION                   pass OPTION to the assembler\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Linker options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --linker FILE               use FILE as linker\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --local-size N              set default local  stack size to N Kb\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --global-size N             set default global stack size to N Kb\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --trail-size N              set default trail  stack size to N Kb\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --cstr-size N               set default cstr   stack size to N Kb\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --max-atom N                set default atom   table size to N atoms\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --fixed-sizes               do not consult environment variables at run-time\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --gui-console               link the Win32 GUI console\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --new-top-level             link the top-level main (to recognize top-level command-line options)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-top-level              do not link the top-level (force --no-debugger)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-debugger               do not link the Prolog/WAM debugger\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --min-pl-bips               link only used Prolog built-in predicates\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --min-fd-bips               link only used FD solver built-in predicates\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --min-bips                  same as: --no-top-level --min-pl-bips --min-fd-bips --no-debugger\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --min-size                  same as: --min-bips --strip\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-pl-lib                 do not look for the Prolog and FD libraries (maintenance only)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-fd-lib                 do not look for the FD library (maintenance only)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-fd-lib-warn            do not warn about inexistent FD library (maintenance only)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -s, --strip                 strip the executable\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -L OPTION                   pass OPTION to the linker\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"The file name specified after --output can include meta-characters:\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  %f for the whole input file name, %F same as %f without directory\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  %p for the whole prefix name, %P same as %p without directory\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  %s for the suffix (or empty if not specified)\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  %d for the directory part (or empty if not specified)\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  %c for a auto-increment counter\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Report bugs to bug-prolog@gnu.org.\0" as *const u8 as *const libc::c_char,
    );
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
