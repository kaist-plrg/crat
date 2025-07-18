use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
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
    mut format_0: *mut libc::c_char,
    mut strict_0: libc::c_int,
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
    if format_0.is_null() {
        format_0 = b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
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
                || *p as libc::c_int != '/' as i32 && strict_0 != 0
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
                dst = dst.offset(sprintf(dst, format_0, free_in_buff) as isize);
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
    mut strict_0: libc::c_int,
    mut quote_0: libc::c_int,
    mut decode_aux_0: libc::c_int,
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
        if quote_0 != 0 {
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
        if strict_0 != 0 {
            return 0 as *mut libc::c_char;
        }
        *arity = -(1 as libc::c_int);
    }
    *aux_no = -(1 as libc::c_int);
    if decode_aux_0 != 0 && *pred as libc::c_int == '$' as i32
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
    if quote_0 != 0 {
        Quote_If_Needed(pred);
    }
    return str;
}
pub unsafe extern "C" fn Decode_Hexa_Line(
    mut str: *mut libc::c_char,
    mut format_0: *mut libc::c_char,
    mut strict_0: libc::c_int,
    mut quote_0: libc::c_int,
    mut decode_aux_0: libc::c_int,
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
    if format_0.is_null() {
        format_0 = b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
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
                    strict_0,
                    quote_0,
                    decode_aux_0,
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
            if decode_aux_0 == 2 as libc::c_int && aux_no >= 0 as libc::c_int {
                n
                    += sprintf(
                        module.as_mut_ptr().offset(n as isize),
                        b"(aux %d)\0" as *const u8 as *const libc::c_char,
                        aux_no,
                    );
            }
            dst = dst.offset(sprintf(dst, format_0, module.as_mut_ptr()) as isize);
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
pub static mut encode: libc::c_int = 0 as libc::c_int;
pub static mut strict: libc::c_int = 1 as libc::c_int;
pub static mut quote: libc::c_int = 1 as libc::c_int;
pub static mut enclose: libc::c_int = 1 as libc::c_int;
pub static mut decode_aux: libc::c_int = 0 as libc::c_int;
pub static mut cmd_line: libc::c_int = 0 as libc::c_int;
pub static mut format: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut fin: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut arg: [*mut libc::c_char; 1024] = [0 as *const libc::c_char
    as *mut libc::c_char; 1024];
pub static mut nb_arg: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    Parse_Arguments(argc, argv);
    if nb_arg == 0 as libc::c_int {
        if cmd_line != 0 {
            Pl_Fatal_Error(
                b"command-line is empty\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        One_File(stdin);
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < nb_arg {
        if cmd_line != 0 {
            One_Line(arg[i as usize]);
            putchar('\n' as i32);
        } else {
            f = fopen(arg[i as usize], b"rt\0" as *const u8 as *const libc::c_char);
            if f.is_null() {
                Pl_Fatal_Error(
                    b"cannot open %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    arg[i as usize],
                );
            }
            One_File(f);
            fclose(f);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn One_File(mut f: *mut FILE) {
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    while !(fgets(
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
    {
        One_Line(buff.as_mut_ptr());
    }
}
pub unsafe extern "C" fn One_Line(mut str: *mut libc::c_char) {
    if encode != 0 {
        fputs(Encode_Hexa_Line(str, format, strict), stdout);
    } else {
        fputs(Decode_Hexa_Line(str, format, strict, quote, decode_aux), stdout);
    };
}
pub unsafe extern "C" fn Parse_Arguments(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut current_block_30: u64;
    i = 1 as libc::c_int;
    while i < argc {
        if **argv.offset(i as isize) as libc::c_int == '-' as i32
            && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
        {
            if strncmp(
                *argv.offset(i as isize),
                b"--encode\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
                || strncmp(
                    *argv.offset(i as isize),
                    b"--mangling\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
            {
                encode = 1 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--decode\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
                || strncmp(
                    *argv.offset(i as isize),
                    b"--demangling\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
            {
                encode = 0 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--relax\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                strict = 0 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--strict\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                strict = 1 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--quote\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                quote = 1 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--no-quote\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                quote = 0 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--printf\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    Pl_Fatal_Error(
                        b"format missing after -printf option\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                format = *argv.offset(i as isize);
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--aux-father\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                decode_aux = 1 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--aux-father2\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                decode_aux = 2 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"--cmd-line\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                cmd_line = 1 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"-E\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
                || strncmp(
                    *argv.offset(i as isize),
                    b"-M\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
            {
                encode = 1 as libc::c_int;
                cmd_line = 1 as libc::c_int;
                strict = 0 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else if strncmp(
                *argv.offset(i as isize),
                b"-P\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
                || strncmp(
                    *argv.offset(i as isize),
                    b"-D\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
            {
                encode = 0 as libc::c_int;
                cmd_line = 1 as libc::c_int;
                strict = 0 as libc::c_int;
                decode_aux = 0 as libc::c_int;
                current_block_30 = 11174649648027449784;
            } else {
                if strncmp(
                    *argv.offset(i as isize),
                    b"--version\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"Prolog/Hexa Filter\0" as *const u8 as *const libc::c_char,
                    );
                    fprintf(
                        stderr,
                        b"%s version %s\n\0" as *const u8 as *const libc::c_char,
                        b"hexgplc\0" as *const u8 as *const libc::c_char,
                        b"1.1\0" as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                if strncmp(
                    *argv.offset(i as isize),
                    b"-h\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                    || strncmp(
                        *argv.offset(i as isize),
                        b"--help\0" as *const u8 as *const libc::c_char,
                        strlen(*argv.offset(i as isize)),
                    ) == 0 as libc::c_int
                {
                    Display_Help();
                    exit(0 as libc::c_int);
                }
                Pl_Fatal_Error(
                    b"unknown option %s - try %s --help\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    *argv.offset(i as isize),
                    b"hexgplc\0" as *const u8 as *const libc::c_char,
                );
                current_block_30 = 7659304154607701039;
            }
        } else {
            current_block_30 = 7659304154607701039;
        }
        match current_block_30 {
            7659304154607701039 => {
                let fresh45 = nb_arg;
                nb_arg = nb_arg + 1;
                arg[fresh45 as usize] = *argv.offset(i as isize);
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn Pl_Fatal_Error(mut format_0: *mut libc::c_char, mut args: ...) {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    vfprintf(stderr, format_0, arg_ptr.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
pub unsafe extern "C" fn Display_Help() {
    fprintf(
        stderr,
        b"Usage: %s [OPTION]... [FILE...]\0" as *const u8 as *const libc::c_char,
        b"hexgplc\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --mangling                  demangling mode (default)\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --decode                    same as --demangling\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --mangling                  mangling mode\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --encode                    same as --mangling\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --relax                     encode/decode also predicate names (not only predicate indicators)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --strict                    encode/decode only predicate indicators (default)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --quote                     quote decoded predicate names (as done by writeq)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --no-quote                  do not quote decoded predicate names\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --printf FORMAT             pass encoded/decoded strings to printf with FORMAT\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --aux-father                decode auxiliary predicate as its father\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --aux-father2               decode auxiliary predicate as its father + auxiliary number\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --cmd-line                  command-line mode: encode/decode each argument of the command-line\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -M or -H                    shortcut for --cmd-line --encode --relax\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -D or -P                    shortcut for --cmd-line --decode --relax --quote\0"
            as *const u8 as *const libc::c_char,
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
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for _arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(_arg.as_os_str()),
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
