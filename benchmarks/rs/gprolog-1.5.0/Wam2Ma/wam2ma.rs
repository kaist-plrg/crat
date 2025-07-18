use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn Parse_Wam_File(
        file_name_in_0: *mut libc::c_char,
        comment_0: libc::c_int,
    ) -> libc::c_int;
    fn Syntax_Error(s: *mut libc::c_char);
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
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type C2RustUnnamed = libc::c_uint;
pub const LIST_INST: C2RustUnnamed = 264;
pub const ANY: C2RustUnnamed = 263;
pub const LABEL: C2RustUnnamed = 262;
pub const MP_N: C2RustUnnamed = 261;
pub const F_N: C2RustUnnamed = 260;
pub const X_Y: C2RustUnnamed = 259;
pub const FLOAT: C2RustUnnamed = 258;
pub const INTEGER: C2RustUnnamed = 257;
pub const ATOM: C2RustUnnamed = 256;
pub type ArgVal = libc::c_double;
pub type Bool = libc::c_int;
pub type BTNode = btnode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct btnode {
    pub str_0: *mut libc::c_char,
    pub no: libc::c_int,
    pub filler: libc::c_int,
    pub info: [libc::c_char; 128],
    pub left: PBTNode,
    pub right: PBTNode,
}
pub type PBTNode = *mut btnode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BTString {
    pub tree: *mut BTNode,
    pub nb_elem: libc::c_int,
}
pub type Pred = predinf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predinf {
    pub module: *mut BTNode,
    pub functor: *mut BTNode,
    pub arity: libc::c_int,
    pub hexa: *mut libc::c_char,
    pub line_no: libc::c_int,
    pub prop: libc::c_int,
    pub pl_file: *mut BTNode,
    pub pl_line: libc::c_int,
    pub swt_tbl: [*mut SwtTbl; 3],
    pub next: PredP,
}
pub type PredP = *mut predinf;
pub type SwtTbl = swt_tbl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swt_tbl {
    pub type_0: C2RustUnnamed_0,
    pub tbl_no: libc::c_int,
    pub next: PSwtTbl,
    pub nb_elem: libc::c_int,
    pub elem: [SwtElt; 1],
}
pub type SwtElt = swt_elt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swt_elt {
    pub atom: *mut BTNode,
    pub n: PlLong,
    pub label: PlLong,
}
pub type PSwtTbl = *mut swt_tbl;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TBL_STC: C2RustUnnamed_0 = 2;
pub const TBL_INT: C2RustUnnamed_0 = 1;
pub const TBL_ATM: C2RustUnnamed_0 = 0;
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub type Direct = directinf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directinf {
    pub pl_file: *mut BTNode,
    pub pl_line: libc::c_int,
    pub system: Bool,
    pub next: DirectP,
}
pub type DirectP = *mut directinf;
pub type BTStrLstFct = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut libc::c_void) -> (),
>;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub unsafe extern "C" fn BT_String_Lookup(
    mut bt_str: *mut BTString,
    mut str: *mut libc::c_char,
) -> *mut BTNode {
    let mut bt_node: *mut BTNode = (*bt_str).tree;
    let mut cmp: libc::c_int = 0;
    while !bt_node.is_null() {
        cmp = strcmp(str, (*bt_node).str_0);
        if cmp == 0 as libc::c_int {
            return bt_node;
        }
        bt_node = if cmp < 0 as libc::c_int {
            (*bt_node).left
        } else {
            (*bt_node).right
        };
    }
    return 0 as *mut BTNode;
}
pub unsafe extern "C" fn BT_String_List(
    mut bt_str: *mut BTString,
    mut fct: BTStrLstFct,
) {
    BT_String_List_Rec((*bt_str).tree, fct);
}
unsafe extern "C" fn BT_String_List_Rec(mut bt_node: *mut BTNode, mut fct: BTStrLstFct) {
    if bt_node.is_null() {
        return;
    }
    BT_String_List_Rec((*bt_node).left, fct);
    (Some(fct.unwrap()))
        .unwrap()(
        (*bt_node).no,
        (*bt_node).str_0,
        ((*bt_node).info).as_mut_ptr() as *mut libc::c_void,
    );
    BT_String_List_Rec((*bt_node).right, fct);
}
pub unsafe extern "C" fn BT_String_Add(
    mut bt_str: *mut BTString,
    mut str: *mut libc::c_char,
) -> *mut BTNode {
    let mut pbt_node: *mut *mut BTNode = &mut (*bt_str).tree;
    let mut bt_node: *mut BTNode = 0 as *mut BTNode;
    let mut cmp: libc::c_int = 0;
    while !(*pbt_node).is_null() {
        bt_node = *pbt_node;
        cmp = strcmp(str, (*bt_node).str_0);
        if cmp == 0 as libc::c_int {
            return bt_node;
        }
        pbt_node = if cmp < 0 as libc::c_int {
            &mut (*bt_node).left
        } else {
            &mut (*bt_node).right
        };
    }
    bt_node = malloc(::std::mem::size_of::<BTNode>() as libc::c_ulong) as *mut BTNode;
    if bt_node.is_null() {
        fprintf(
            stderr,
            b"Cannot allocate memory for BT string: %s\n\0" as *const u8
                as *const libc::c_char,
            str,
        );
        exit(1 as libc::c_int);
    }
    (*bt_node).str_0 = str;
    let fresh0 = (*bt_str).nb_elem;
    (*bt_str).nb_elem = (*bt_str).nb_elem + 1;
    (*bt_node).no = fresh0;
    memset(
        ((*bt_node).info).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    );
    (*bt_node).right = 0 as PBTNode;
    (*bt_node).left = (*bt_node).right;
    *pbt_node = bt_node;
    return bt_node;
}
pub unsafe extern "C" fn BT_String_Init(mut bt_str: *mut BTString) {
    (*bt_str).tree = 0 as *mut BTNode;
    (*bt_str).nb_elem = 0 as libc::c_int;
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
    let fresh1 = str;
    str = str.offset(1);
    *fresh1 = 'X' as i32 as libc::c_char;
    let fresh2 = str;
    str = str.offset(1);
    *fresh2 = ('0' as i32 + (module_encode << 1 as libc::c_int | pred_encode))
        as libc::c_char;
    let fresh3 = str;
    str = str.offset(1);
    *fresh3 = '_' as i32 as libc::c_char;
    if module_encode == 1 as libc::c_int {
        str = str
            .offset(
                sprintf(str, b"%s__\0" as *const u8 as *const libc::c_char, module)
                    as isize,
            );
    } else if module_encode == 2 as libc::c_int {
        str = Encode_String(module, str);
        let fresh4 = str;
        str = str.offset(1);
        *fresh4 = '_' as i32 as libc::c_char;
        let fresh5 = str;
        str = str.offset(1);
        *fresh5 = '_' as i32 as libc::c_char;
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
            let fresh6 = src;
            src = src.offset(1);
            let fresh7 = dst;
            dst = dst.offset(1);
            *fresh7 = *fresh6;
        }
        if *src as libc::c_int == '\0' as i32 {
            break;
        }
        p = Parse_Atom(src, buff.as_mut_ptr());
        if p.is_null() {
            let fresh8 = src;
            src = src.offset(1);
            let fresh9 = dst;
            dst = dst.offset(1);
            *fresh9 = *fresh8;
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
                        let fresh10 = src;
                        src = src.offset(1);
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = *fresh10;
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
                    let fresh12 = src;
                    src = src.offset(1);
                    let fresh13 = dst;
                    dst = dst.offset(1);
                    *fresh13 = *fresh12;
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
    let fresh14 = dst;
    dst = dst.offset(1);
    *fresh14 = '\0' as i32 as libc::c_char;
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
    let fresh15 = str;
    str = str.offset(1);
    if *fresh15 as libc::c_int != 'X' as i32 || (*str as libc::c_int) < '0' as i32
        || *str as libc::c_int >= '5' as i32
    {
        return 0 as *mut libc::c_char;
    }
    let fresh16 = str;
    str = str.offset(1);
    n = *fresh16 as libc::c_int - '0' as i32;
    module_encode = n >> 1 as libc::c_int;
    pred_encode = n & 1 as libc::c_int;
    let fresh17 = str;
    str = str.offset(1);
    if *fresh17 as libc::c_int != '_' as i32 {
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
                let fresh18 = str;
                str = str.offset(1);
                *fresh18 as libc::c_int != '_' as i32
            }
            || {
                let fresh19 = str;
                str = str.offset(1);
                *fresh19 as libc::c_int != '_' as i32
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
                    let fresh20 = p;
                    p = p.offset(1);
                    if !(*fresh20 != 0) {
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
                let fresh21 = n;
                n = n + 1;
                module[fresh21 as usize] = ':' as i32 as libc::c_char;
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
            let fresh22 = src;
            src = src.offset(1);
            let fresh23 = dst;
            dst = dst.offset(1);
            *fresh23 = *fresh22;
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
    let fresh24 = str;
    str = str.offset(1);
    let fresh25 = buff;
    buff = buff.offset(1);
    *fresh25 = *fresh24;
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
        let fresh26 = str;
        str = str.offset(1);
        let fresh27 = buff;
        buff = buff.offset(1);
        *fresh27 = *fresh26;
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
        let fresh28 = buff;
        buff = buff.offset(1);
        *fresh28 = ((if *(*__ctype_b_loc()).offset(*str as libc::c_int as isize)
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
    let fresh29 = q;
    q = q.offset(1);
    *fresh29 = '\'' as i32 as libc::c_char;
    p = str;
    while *p != 0 {
        r = strchr(pl_escape_char.as_mut_ptr(), *p as libc::c_int);
        if !r.is_null() {
            let fresh30 = q;
            q = q.offset(1);
            *fresh30 = '\\' as i32 as libc::c_char;
            let fresh31 = q;
            q = q.offset(1);
            *fresh31 = pl_escape_symbol[r.offset_from(pl_escape_char.as_mut_ptr())
                as libc::c_long as usize];
        } else if *p as libc::c_int == '\'' as i32 || *p as libc::c_int == '\\' as i32 {
            let fresh32 = q;
            q = q.offset(1);
            *fresh32 = *p;
            let fresh33 = q;
            q = q.offset(1);
            *fresh33 = *p;
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
            let fresh34 = q;
            q = q.offset(1);
            *fresh34 = *p;
        }
        p = p.offset(1);
        p;
    }
    let fresh35 = q;
    q = q.offset(1);
    *fresh35 = '\'' as i32 as libc::c_char;
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
        let fresh36 = src;
        src = src.offset(1);
        delim = *fresh36;
        while *src as libc::c_int != 0
            && (*src as libc::c_int != delim as libc::c_int
                || *src.offset(1 as libc::c_int as isize) as libc::c_int
                    == delim as libc::c_int)
        {
            if *src as libc::c_int == delim as libc::c_int {
                let fresh37 = dst;
                dst = dst.offset(1);
                *fresh37 = delim;
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
                    let fresh38 = src;
                    src = src.offset(1);
                    let fresh39 = dst;
                    dst = dst.offset(1);
                    *fresh39 = *fresh38;
                } else {
                    p = strchr(pl_escape_symbol.as_mut_ptr(), *src as libc::c_int);
                    if !p.is_null() {
                        let fresh40 = dst;
                        dst = dst.offset(1);
                        *fresh40 = pl_escape_char[p
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
                        let fresh41 = dst;
                        dst = dst.offset(1);
                        *fresh41 = i as libc::c_char;
                        src = p.offset(1 as libc::c_int as isize);
                    }
                }
            } else {
                let fresh42 = src;
                src = src.offset(1);
                let fresh43 = dst;
                dst = dst.offset(1);
                *fresh43 = *fresh42;
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
            let fresh44 = src;
            src = src.offset(1);
            let fresh45 = dst;
            dst = dst.offset(1);
            *fresh45 = *fresh44;
        }
    }
    *dst = '\0' as i32 as libc::c_char;
    return src;
}
pub static mut file_name_in: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut file_name_out: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut comment: Bool = 0;
pub static mut file_out: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut bt_atom: BTString = BTString {
    tree: 0 as *const BTNode as *mut BTNode,
    nb_elem: 0,
};
pub static mut bt_tagged_atom: BTString = BTString {
    tree: 0 as *const BTNode as *mut BTNode,
    nb_elem: 0,
};
pub static mut bt_tagged_f_n: BTString = BTString {
    tree: 0 as *const BTNode as *mut BTNode,
    nb_elem: 0,
};
pub static mut cur_pl_file: *mut BTNode = 0 as *const BTNode as *mut BTNode;
pub static mut buff_hexa: [libc::c_char; 4114] = [0; 4114];
pub static mut dummy_pred_start: Pred = Pred {
    module: 0 as *const BTNode as *mut BTNode,
    functor: 0 as *const BTNode as *mut BTNode,
    arity: 0,
    hexa: 0 as *const libc::c_char as *mut libc::c_char,
    line_no: 0,
    prop: 0,
    pl_file: 0 as *const BTNode as *mut BTNode,
    pl_line: 0,
    swt_tbl: [0 as *const SwtTbl as *mut SwtTbl; 3],
    next: 0 as *const predinf as *mut predinf,
};
pub static mut pred_end: *mut Pred = unsafe {
    &dummy_pred_start as *const Pred as *mut Pred
};
pub static mut dummy_direct_start: Direct = Direct {
    pl_file: 0 as *const BTNode as *mut BTNode,
    pl_line: 0,
    system: 0,
    next: 0 as *const directinf as *mut directinf,
};
pub static mut direct_end: *mut Direct = unsafe {
    &dummy_direct_start as *const Direct as *mut Direct
};
pub static mut nb_swt_tbl: libc::c_int = 0 as libc::c_int;
pub static mut cur_pred: *mut Pred = 0 as *const Pred as *mut Pred;
pub static mut cur_pred_no: libc::c_int = 0 as libc::c_int;
pub static mut cur_arity: libc::c_int = 0;
pub static mut cur_sub_label: PlLong = 0;
pub static mut cur_direct_no: libc::c_int = 0 as libc::c_int;
pub static mut foreign_tbl: [*mut libc::c_char; 16] = [0 as *const libc::c_char
    as *mut libc::c_char; 16];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    Parse_Arguments(argc, argv);
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
    if file_name_out.is_null() {
        file_out = stdout;
    } else {
        file_out = fopen(file_name_out, b"wt\0" as *const u8 as *const libc::c_char);
        if file_out.is_null() {
            fprintf(
                stderr,
                b"cannot open output file %s\n\0" as *const u8 as *const libc::c_char,
                file_name_out,
            );
            exit(1 as libc::c_int);
        }
    }
    BT_String_Init(&mut bt_atom);
    BT_String_Init(&mut bt_tagged_atom);
    BT_String_Init(&mut bt_tagged_f_n);
    Init_Foreign_Table();
    dummy_pred_start.next = 0 as PredP;
    dummy_direct_start.next = 0 as DirectP;
    if Parse_Wam_File(file_name_in, comment) == 0 {
        fprintf(stderr, b"Translation aborted\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    Emit_Obj_Initializer();
    Emit_Exec_Directives();
    if file_out != stdout {
        fclose(file_out);
    }
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn Source_Line(
    mut line_no: libc::c_int,
    mut cmt: *mut libc::c_char,
) {
    Label_Printf(
        b"\t; %6d: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        line_no,
        cmt,
    );
}
pub unsafe extern "C" fn F_file_name(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut pl_file: *mut libc::c_char = 0 as *mut libc::c_char;
    pl_file = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    cur_pl_file = BT_String_Add(&mut bt_atom, pl_file);
}
pub unsafe extern "C" fn F_predicate(mut arg: *mut ArgVal) {
    let mut atom_module: *mut BTNode = 0 as *mut BTNode;
    let mut atom_functor: *mut BTNode = 0 as *mut BTNode;
    let mut module_user_system: Bool = 0 as libc::c_int;
    let mut prop: libc::c_int = 0 as libc::c_int;
    let mut local_symbol: Bool = 0 as libc::c_int;
    let mut top: *mut ArgVal = arg;
    let mut module: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut functor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arity: PlLong = 0;
    let mut pl_line: PlLong = 0;
    let mut static_dynamic: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut public_private: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mono_multi: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut built_in_local_global: *mut libc::c_char = 0 as *mut libc::c_char;
    module = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    functor = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    arity = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    pl_line = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    static_dynamic = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    public_private = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    mono_multi = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    built_in_local_global = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    if cur_pl_file.is_null() {
        Syntax_Error(
            b"file_name declaration missing\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    atom_functor = BT_String_Add(&mut bt_atom, functor);
    cur_arity = arity as libc::c_int;
    cur_sub_label = 0 as libc::c_int as PlLong;
    if strcmp(static_dynamic, b"dynamic\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        prop = 2 as libc::c_int;
    } else if strcmp(static_dynamic, b"static\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        prop = 1 as libc::c_int;
    } else {
        Syntax_Error(
            b"static or dynamic expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if strcmp(public_private, b"public\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        prop |= 4 as libc::c_int;
    } else if strcmp(public_private, b"private\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        Syntax_Error(
            b"public or private expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !(strcmp(mono_multi, b"monofile\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int)
    {
        if strcmp(mono_multi, b"multifile\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            prop |= 64 as libc::c_int;
            prop &= !(1 as libc::c_int);
        } else {
            Syntax_Error(
                b"multifile or multifile expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    if strcmp(built_in_local_global, b"built_in\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        prop |= 8 as libc::c_int;
    } else if strcmp(
        built_in_local_global,
        b"built_in_fd\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        prop |= 16 as libc::c_int;
    } else if strcmp(
        built_in_local_global,
        b"local\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        local_symbol = 1 as libc::c_int;
    } else if strcmp(
        built_in_local_global,
        b"user\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
        && strcmp(built_in_local_global, b"global\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        Syntax_Error(
            b"built_in, built_in_fd, local or global (or user) expected\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if local_symbol == 0 {
        prop |= 128 as libc::c_int;
    }
    cur_pred_no += 1;
    cur_pred_no;
    cur_pred = malloc(::std::mem::size_of::<Pred>() as libc::c_ulong) as *mut Pred;
    if cur_pred.is_null() {
        fprintf(
            stderr,
            b"Cannot allocate memory for predicate #%d (%s/%ld)\n\0" as *const u8
                as *const libc::c_char,
            cur_pred_no,
            functor,
            arity,
        );
        exit(1 as libc::c_int);
    }
    if module.is_null() || *module as libc::c_int == '\0' as i32 {
        if prop & 8 as libc::c_int != 0 || prop & 16 as libc::c_int != 0 {
            module = b"system\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            module = b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    atom_module = BT_String_Add(&mut bt_atom, module);
    if strcmp(module, b"user\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(module, b"system\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        module_user_system = 1 as libc::c_int;
    }
    (*cur_pred).module = atom_module;
    (*cur_pred).functor = atom_functor;
    (*cur_pred).arity = arity as libc::c_int;
    (*cur_pred).pl_file = cur_pl_file;
    (*cur_pred).pl_line = pl_line as libc::c_int;
    (*cur_pred).prop = prop;
    (*cur_pred).swt_tbl[0 as libc::c_int as usize] = 0 as *mut SwtTbl;
    (*cur_pred).swt_tbl[1 as libc::c_int as usize] = 0 as *mut SwtTbl;
    (*cur_pred).swt_tbl[2 as libc::c_int as usize] = 0 as *mut SwtTbl;
    (*cur_pred).next = 0 as PredP;
    (*pred_end).next = cur_pred;
    pred_end = cur_pred;
    if comment != 0 {
        Label_Printf(
            b"\n\n; *** Predicate: %s:%s/%d (%s:%d)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            module,
            functor,
            arity,
            (*cur_pl_file).str_0,
            pl_line,
        );
    }
    Encode_Hexa(
        if local_symbol != 0 || module_user_system != 0 {
            0 as *mut libc::c_char
        } else {
            module
        },
        functor,
        arity as libc::c_int,
        buff_hexa.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    *buff_hexa.as_mut_ptr() = '&' as i32 as libc::c_char;
    (*cur_pred).hexa = strdup(buff_hexa.as_mut_ptr());
    Label_Printf(
        b"\n\npl_code %s %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        if local_symbol != 0 {
            b"local\0" as *const u8 as *const libc::c_char
        } else {
            b"global\0" as *const u8 as *const libc::c_char
        },
        buff_hexa.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
}
pub unsafe extern "C" fn F_directive(mut arg: *mut ArgVal) {
    let mut p: *mut Direct = 0 as *mut Direct;
    let mut system: Bool = 0 as libc::c_int;
    let mut top: *mut ArgVal = arg;
    let mut pl_line: PlLong = 0;
    let mut user_system: *mut libc::c_char = 0 as *mut libc::c_char;
    pl_line = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    user_system = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    if cur_pl_file.is_null() {
        Syntax_Error(
            b"file_name declaration missing\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if strcmp(user_system, b"system\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        system = 1 as libc::c_int;
    } else if strcmp(user_system, b"user\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        system = 0 as libc::c_int;
    } else {
        Syntax_Error(
            b"user or system expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    cur_direct_no += 1;
    cur_direct_no;
    p = malloc(::std::mem::size_of::<Direct>() as libc::c_ulong) as *mut Direct;
    if p.is_null() {
        fprintf(
            stderr,
            b"Cannot allocate memory for directive #%d\n\0" as *const u8
                as *const libc::c_char,
            cur_direct_no,
        );
        exit(1 as libc::c_int);
    }
    (*p).pl_file = cur_pl_file;
    (*p).pl_line = pl_line as libc::c_int;
    (*p).system = system;
    (*p).next = 0 as DirectP;
    (*direct_end).next = p;
    direct_end = p;
    if comment != 0 {
        Label_Printf(
            b"\n\n; *** %s Directive (%s:%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            if system != 0 {
                b"System\0" as *const u8 as *const libc::c_char
            } else {
                b"User\0" as *const u8 as *const libc::c_char
            },
            (*cur_pl_file).str_0,
            pl_line,
        );
    }
    Label_Printf(
        b"\n\npl_code local directive_%d\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        cur_direct_no,
    );
}
pub unsafe extern "C" fn F_ensure_linked(mut arg: *mut ArgVal) {
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: PlLong = 0;
    let mut top: *mut ArgVal = arg;
    let mut nb_elem: PlLong = 0;
    nb_elem = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Label_Printf(
        b"\n\npl_code local ensure_linked\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    loop {
        let fresh46 = nb_elem;
        nb_elem = nb_elem - 1;
        if !(fresh46 != 0) {
            break;
        }
        m = *(top as *mut *mut libc::c_char);
        top = top.offset(1);
        top;
        p = *(top as *mut *mut libc::c_char);
        top = top.offset(1);
        top;
        n = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        Encode_Hexa(m, p, n as libc::c_int, buff_hexa.as_mut_ptr());
        Inst_Printf(
            b"pl_jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buff_hexa.as_mut_ptr(),
        );
    };
}
pub unsafe extern "C" fn F_get_variable(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut a: PlLong = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"move\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld),%c(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
        c as libc::c_int,
        xy,
    );
}
pub unsafe extern "C" fn F_get_value(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut a: PlLong = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify(%c(%ld),X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c as libc::c_int,
        xy,
        a,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_get_atom(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut atom: *mut BTNode = 0 as *mut BTNode;
    let mut str_atom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: PlLong = 0;
    str_atom = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    atom = BT_String_Add(&mut bt_tagged_atom, str_atom);
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Get_Atom_Tagged(ta(%d),X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*atom).no,
        a,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_get_integer(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut n: PlLong = 0;
    let mut a: PlLong = 0;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Get_Integer_Tagged(%ld,X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong,
        a,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_get_float(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut n: libc::c_double = 0.;
    let mut a: PlLong = 0;
    n = *(top as *mut libc::c_double);
    top = top.offset(1);
    top;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Get_Float(%1.20e,X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        n,
        a,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_get_nil(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut a: PlLong = 0;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Get_Nil(X(%ld))\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_get_list(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut a: PlLong = 0;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Get_List(X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        a,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_get_structure(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut str_atom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: PlLong = 0;
    let mut f_n_no: libc::c_int = 0;
    let mut a: PlLong = 0;
    str_atom = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    f_n_no = Add_F_N_Tagged(str_atom, n as libc::c_int);
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Get_Structure_Tagged(fn(%d),X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        f_n_no,
        a,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_put_variable(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut a: PlLong = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if c as libc::c_int == 'X' as i32 {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Put_X_Variable()\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Inst_Printf(
            b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            a,
        );
        Inst_Printf(
            b"move\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"X(%ld),X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            a,
            xy,
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Put_Y_Variable(&Y(%ld))\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            xy,
        );
        Inst_Printf(
            b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            a,
        );
    };
}
pub unsafe extern "C" fn F_put_void(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut a: PlLong = 0;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_X_Variable()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_value(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut a: PlLong = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"move\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%c(%ld),X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        c as libc::c_int,
        xy,
        a,
    );
}
pub unsafe extern "C" fn F_put_unsafe_value(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut a: PlLong = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_Unsafe_Value(%c(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c as libc::c_int,
        xy,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_atom(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut atom: *mut BTNode = 0 as *mut BTNode;
    let mut str_atom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: PlLong = 0;
    str_atom = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    atom = BT_String_Add(&mut bt_tagged_atom, str_atom);
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_Atom_Tagged(ta(%d))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*atom).no,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_integer(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut n: PlLong = 0;
    let mut a: PlLong = 0;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_Integer_Tagged(%ld)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_float(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut n: libc::c_double = 0.;
    let mut a: PlLong = 0;
    n = *(top as *mut libc::c_double);
    top = top.offset(1);
    top;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_Float(%1.20e)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        n,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_nil(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut a: PlLong = 0;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_Nil()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_list(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut a: PlLong = 0;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_List()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_structure(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut str_atom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: PlLong = 0;
    let mut f_n_no: libc::c_int = 0;
    let mut a: PlLong = 0;
    str_atom = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    f_n_no = Add_F_N_Tagged(str_atom, n as libc::c_int);
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_Structure_Tagged(fn(%d))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        f_n_no,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"X(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        a,
    );
}
pub unsafe extern "C" fn F_put_meta_term(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut module: *mut BTNode = 0 as *mut BTNode;
    let mut str_module: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: PlLong = 0;
    str_module = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    module = BT_String_Add(&mut bt_tagged_atom, str_module);
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Put_Meta_Term_Tagged(ta(%d), %ld)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*module).no,
        a,
    );
}
pub unsafe extern "C" fn F_math_load_value(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut a: PlLong = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Math_Load_Value(%c(%ld),&X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c as libc::c_int,
        xy,
        a,
    );
}
pub unsafe extern "C" fn F_math_fast_load_value(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut a: PlLong = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Math_Fast_Load_Value(%c(%ld),&X(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c as libc::c_int,
        xy,
        a,
    );
}
pub unsafe extern "C" fn F_unify_variable(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Variable()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%c(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        c as libc::c_int,
        xy,
    );
}
pub unsafe extern "C" fn F_unify_void(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut n: PlLong = 0;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Void(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        n,
    );
}
pub unsafe extern "C" fn F_unify_value(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Value(%c(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c as libc::c_int,
        xy,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_unify_local_value(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Local_Value(%c(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c as libc::c_int,
        xy,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_unify_atom(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut atom: *mut BTNode = 0 as *mut BTNode;
    let mut str_atom: *mut libc::c_char = 0 as *mut libc::c_char;
    str_atom = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    atom = BT_String_Add(&mut bt_tagged_atom, str_atom);
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Atom_Tagged(ta(%d))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*atom).no,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_unify_integer(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut n: PlLong = 0;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Integer_Tagged(%ld)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_unify_nil(mut arg: *mut ArgVal) {
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Nil()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_unify_list(mut arg: *mut ArgVal) {
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_List()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_unify_structure(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut str_atom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: PlLong = 0;
    let mut f_n_no: libc::c_int = 0;
    str_atom = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    f_n_no = Add_F_N_Tagged(str_atom, n as libc::c_int);
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Unify_Structure_Tagged(fn(%d))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        f_n_no,
    );
    Inst_Printf(
        b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_allocate(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut n: PlLong = 0;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Allocate(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        n,
    );
}
pub unsafe extern "C" fn F_deallocate(mut arg: *mut ArgVal) {
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Deallocate()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_call(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: PlLong = 0;
    m = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    p = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Encode_Hexa(m, p, n as libc::c_int, buff_hexa.as_mut_ptr());
    Inst_Printf(
        b"pl_call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buff_hexa.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn F_execute(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: PlLong = 0;
    m = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    p = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    n = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Encode_Hexa(m, p, n as libc::c_int, buff_hexa.as_mut_ptr());
    Inst_Printf(
        b"pl_jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buff_hexa.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn F_proceed(mut arg: *mut ArgVal) {
    Inst_Printf(
        b"pl_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_fail(mut arg: *mut ArgVal) {
    Inst_Printf(
        b"pl_fail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_label(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut val_l: PlLong = 0;
    val_l = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if val_l == -(1 as libc::c_int) as libc::c_long {
        strcpy(l.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(
            l.as_mut_ptr(),
            b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
            cur_pred_no,
            val_l,
        );
    }
    Label_Printf(
        b"\n%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        l.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn F_switch_on_term(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut val_label: PlLong = 0;
    static mut l: [[libc::c_char; 32]; 5] = [[0; 32]; 5];
    let mut mask: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        val_label = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        if val_label == -(1 as libc::c_int) as libc::c_long {
            strcpy(
                (l[i as usize]).as_mut_ptr(),
                b"0\0" as *const u8 as *const libc::c_char,
            );
        } else {
            sprintf(
                (l[i as usize]).as_mut_ptr(),
                b"&.pred%d_%ld\0" as *const u8 as *const libc::c_char,
                cur_pred_no,
                val_label,
            );
            mask |= (1 as libc::c_int) << i;
        }
        i += 1;
        i;
    }
    match mask {
        3 => {
            Inst_Printf(
                b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Pl_Switch_On_Term_Var_Atm(%s,%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (l[0 as libc::c_int as usize]).as_mut_ptr(),
                (l[1 as libc::c_int as usize]).as_mut_ptr(),
            );
        }
        17 => {
            Inst_Printf(
                b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Pl_Switch_On_Term_Var_Stc(%s,%s)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (l[0 as libc::c_int as usize]).as_mut_ptr(),
                (l[4 as libc::c_int as usize]).as_mut_ptr(),
            );
        }
        11 => {
            Inst_Printf(
                b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Pl_Switch_On_Term_Var_Atm_Lst(%s,%s,%s)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (l[0 as libc::c_int as usize]).as_mut_ptr(),
                (l[1 as libc::c_int as usize]).as_mut_ptr(),
                (l[3 as libc::c_int as usize]).as_mut_ptr(),
            );
        }
        19 => {
            Inst_Printf(
                b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Pl_Switch_On_Term_Var_Atm_Stc(%s,%s,%s)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (l[0 as libc::c_int as usize]).as_mut_ptr(),
                (l[1 as libc::c_int as usize]).as_mut_ptr(),
                (l[4 as libc::c_int as usize]).as_mut_ptr(),
            );
        }
        _ => {
            Inst_Printf(
                b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Pl_Switch_On_Term(%s,%s,%s,%s,%s)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (l[0 as libc::c_int as usize]).as_mut_ptr(),
                (l[1 as libc::c_int as usize]).as_mut_ptr(),
                (l[2 as libc::c_int as usize]).as_mut_ptr(),
                (l[3 as libc::c_int as usize]).as_mut_ptr(),
                (l[4 as libc::c_int as usize]).as_mut_ptr(),
            );
        }
    }
    Inst_Printf(
        b"jump_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Create_Switch_Table(
    mut type_0: libc::c_int,
    mut nb_elem: libc::c_int,
) -> *mut SwtTbl {
    let mut t: *mut SwtTbl = 0 as *mut SwtTbl;
    t = malloc(
        (::std::mem::size_of::<SwtTbl>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<SwtElt>() as libc::c_ulong)
                    .wrapping_mul((nb_elem - 1 as libc::c_int) as libc::c_ulong),
            ),
    ) as *mut SwtTbl;
    if t.is_null() {
        fprintf(
            stderr,
            b"Cannot allocate memory for switch table\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*t).type_0 = type_0 as C2RustUnnamed_0;
    let fresh47 = nb_swt_tbl;
    nb_swt_tbl = nb_swt_tbl + 1;
    (*t).tbl_no = fresh47;
    (*t).next = (*cur_pred).swt_tbl[type_0 as usize];
    (*t).nb_elem = nb_elem;
    (*cur_pred).swt_tbl[type_0 as usize] = t;
    return t;
}
pub unsafe extern "C" fn F_switch_on_atom(mut arg: *mut ArgVal) {
    let mut t: *mut SwtTbl = 0 as *mut SwtTbl;
    let mut elem: *mut SwtElt = 0 as *mut SwtElt;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut label: PlLong = 0;
    let mut top: *mut ArgVal = arg;
    let mut nb_elem: PlLong = 0;
    nb_elem = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    t = Create_Switch_Table(TBL_ATM as libc::c_int, nb_elem as libc::c_int);
    elem = ((*t).elem).as_mut_ptr();
    loop {
        let fresh48 = nb_elem;
        nb_elem = nb_elem - 1;
        if !(fresh48 != 0) {
            break;
        }
        str = *(top as *mut *mut libc::c_char);
        top = top.offset(1);
        top;
        label = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        (*elem).atom = BT_String_Add(&mut bt_atom, str);
        (*elem).label = label;
        elem = elem.offset(1);
        elem;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Switch_On_Atom(st(%d),%d)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        nb_swt_tbl - 1 as libc::c_int,
        (*t).nb_elem,
    );
    Inst_Printf(
        b"jump_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_switch_on_integer(mut arg: *mut ArgVal) {
    let mut c: libc::c_char = 0;
    let mut n: PlLong = 0;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut val_l: PlLong = 0;
    let mut top: *mut ArgVal = arg;
    let mut nb_elem: PlLong = 0;
    nb_elem = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Switch_On_Integer()\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Inst_Printf(
        b"switch_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    c = '(' as i32 as libc::c_char;
    loop {
        let fresh49 = nb_elem;
        nb_elem = nb_elem - 1;
        if !(fresh49 != 0) {
            break;
        }
        n = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        val_l = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        if val_l == -(1 as libc::c_int) as libc::c_long {
            strcpy(l.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
        } else {
            sprintf(
                l.as_mut_ptr(),
                b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
                cur_pred_no,
                val_l,
            );
        }
        fprintf(
            file_out,
            b"%c%ld=%s\0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
            n,
            l.as_mut_ptr(),
        );
        c = ',' as i32 as libc::c_char;
    }
    fprintf(file_out, b")\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn F_switch_on_structure(mut arg: *mut ArgVal) {
    let mut t: *mut SwtTbl = 0 as *mut SwtTbl;
    let mut elem: *mut SwtElt = 0 as *mut SwtElt;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arity: PlLong = 0;
    let mut label: PlLong = 0;
    let mut top: *mut ArgVal = arg;
    let mut nb_elem: PlLong = 0;
    nb_elem = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    t = Create_Switch_Table(TBL_STC as libc::c_int, nb_elem as libc::c_int);
    elem = ((*t).elem).as_mut_ptr();
    loop {
        let fresh50 = nb_elem;
        nb_elem = nb_elem - 1;
        if !(fresh50 != 0) {
            break;
        }
        str = *(top as *mut *mut libc::c_char);
        top = top.offset(1);
        top;
        arity = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        label = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        (*elem).atom = BT_String_Add(&mut bt_atom, str);
        (*elem).n = arity;
        (*elem).label = label;
        elem = elem.offset(1);
        elem;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Switch_On_Structure(st(%d),%d)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        nb_swt_tbl - 1 as libc::c_int,
        (*t).nb_elem,
    );
    Inst_Printf(
        b"jump_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn F_try_me_else(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut val_l: PlLong = 0;
    val_l = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if val_l == -(1 as libc::c_int) as libc::c_long {
        strcpy(l.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(
            l.as_mut_ptr(),
            b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
            cur_pred_no,
            val_l,
        );
    }
    if cur_arity >= 1 as libc::c_int && cur_arity <= 4 as libc::c_int {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Create_Choice_Point%d(&%s)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
            l.as_mut_ptr(),
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Create_Choice_Point(&%s,%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            l.as_mut_ptr(),
            cur_arity,
        );
    };
}
pub unsafe extern "C" fn F_retry_me_else(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut val_l: PlLong = 0;
    val_l = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if val_l == -(1 as libc::c_int) as libc::c_long {
        strcpy(l.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(
            l.as_mut_ptr(),
            b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
            cur_pred_no,
            val_l,
        );
    }
    if cur_arity >= 1 as libc::c_int && cur_arity <= 4 as libc::c_int {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Update_Choice_Point%d(&%s)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
            l.as_mut_ptr(),
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Update_Choice_Point(&%s,%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            l.as_mut_ptr(),
            cur_arity,
        );
    };
}
pub unsafe extern "C" fn F_trust_me_else_fail(mut arg: *mut ArgVal) {
    if cur_arity >= 1 as libc::c_int && cur_arity <= 4 as libc::c_int {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Delete_Choice_Point%d()\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Delete_Choice_Point(%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
        );
    };
}
pub unsafe extern "C" fn F_try(mut arg: *mut ArgVal) {
    let mut sl: [libc::c_char; 32] = [0; 32];
    let mut top: *mut ArgVal = arg;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut val_l: PlLong = 0;
    val_l = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if val_l == -(1 as libc::c_int) as libc::c_long {
        strcpy(l.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(
            l.as_mut_ptr(),
            b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
            cur_pred_no,
            val_l,
        );
    }
    let fresh51 = cur_sub_label;
    cur_sub_label = cur_sub_label + 1;
    sprintf(
        sl.as_mut_ptr(),
        b".pred%d_sub_%ld\0" as *const u8 as *const libc::c_char,
        cur_pred_no,
        fresh51,
    );
    if cur_arity >= 1 as libc::c_int && cur_arity <= 4 as libc::c_int {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Create_Choice_Point%d(&%s)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
            sl.as_mut_ptr(),
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Create_Choice_Point(&%s,%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            sl.as_mut_ptr(),
            cur_arity,
        );
    }
    Inst_Printf(
        b"jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        l.as_mut_ptr(),
    );
    Label_Printf(
        b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sl.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn F_retry(mut arg: *mut ArgVal) {
    let mut sl: [libc::c_char; 32] = [0; 32];
    let mut top: *mut ArgVal = arg;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut val_l: PlLong = 0;
    val_l = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if val_l == -(1 as libc::c_int) as libc::c_long {
        strcpy(l.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(
            l.as_mut_ptr(),
            b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
            cur_pred_no,
            val_l,
        );
    }
    let fresh52 = cur_sub_label;
    cur_sub_label = cur_sub_label + 1;
    sprintf(
        sl.as_mut_ptr(),
        b".pred%d_sub_%ld\0" as *const u8 as *const libc::c_char,
        cur_pred_no,
        fresh52,
    );
    if cur_arity >= 1 as libc::c_int && cur_arity <= 4 as libc::c_int {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Update_Choice_Point%d(&%s)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
            sl.as_mut_ptr(),
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Update_Choice_Point(&%s,%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            sl.as_mut_ptr(),
            cur_arity,
        );
    }
    Inst_Printf(
        b"jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        l.as_mut_ptr(),
    );
    Label_Printf(
        b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sl.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn F_trust(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut val_l: PlLong = 0;
    val_l = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if val_l == -(1 as libc::c_int) as libc::c_long {
        strcpy(l.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(
            l.as_mut_ptr(),
            b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
            cur_pred_no,
            val_l,
        );
    }
    if cur_arity >= 1 as libc::c_int && cur_arity <= 4 as libc::c_int {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Delete_Choice_Point%d()\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Delete_Choice_Point(%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cur_arity,
        );
    }
    Inst_Printf(
        b"jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        l.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn F_pragma_arity(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut a: PlLong = 0;
    a = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    cur_arity = a as libc::c_int;
}
pub unsafe extern "C" fn F_get_current_choice(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Get_Current_Choice()\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%c(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        c as libc::c_int,
        xy,
    );
}
pub unsafe extern "C" fn F_cut(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Cut(%c(%ld))\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        c as libc::c_int,
        xy,
    );
}
pub unsafe extern "C" fn F_soft_cut(mut arg: *mut ArgVal) {
    let mut top: *mut ArgVal = arg;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    xy = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if xy < 5000 as libc::c_int as libc::c_long {
        c = 'X' as i32 as libc::c_char;
    } else {
        xy -= 5000 as libc::c_int as libc::c_long;
        c = 'Y' as i32 as libc::c_char;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Soft_Cut(%c(%ld))\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c as libc::c_int,
        xy,
    );
}
pub unsafe extern "C" fn F_call_c(mut arg: *mut ArgVal) {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut fast_call: Bool = 0 as libc::c_int;
    let mut tagged: Bool = 0 as libc::c_int;
    let mut set_cp: Bool = 0 as libc::c_int;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut adr_of: Bool = 0;
    let mut ret_xy: PlLong = 0 as libc::c_int as PlLong;
    let mut ret_c: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut i: libc::c_int = 0;
    let mut c_option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg_type: PlLong = 0;
    let mut atom: *mut BTNode = 0 as *mut BTNode;
    let mut str_atom: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aux_functor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aux_arity: PlLong = 0;
    let mut n: PlLong = 0;
    let mut n1: libc::c_double = 0.;
    let mut xy: PlLong = 0;
    let mut c: libc::c_char = 0;
    let mut top: *mut ArgVal = arg;
    let mut fct_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nb_elem: PlLong = 0;
    fct_name = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    nb_elem = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < nb_elem {
        arg_type = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        if arg_type == X_Y as libc::c_int as libc::c_long {
            xy = *(top as *mut PlLong);
            top = top.offset(1);
            top;
            if xy < 5000 as libc::c_int as libc::c_long {
                c = 'X' as i32 as libc::c_char;
            } else {
                xy -= 5000 as libc::c_int as libc::c_long;
                c = 'Y' as i32 as libc::c_char;
            }
            ret = 3 as libc::c_int;
            ret_xy = xy;
            ret_c = c;
        } else {
            c_option = *(top as *mut *mut libc::c_char);
            top = top.offset(1);
            top;
            if strcmp(c_option, b"boolean\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = 1 as libc::c_int;
            } else if strcmp(c_option, b"jump\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ret = 2 as libc::c_int;
            } else if strcmp(
                c_option,
                b"fast_call\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                fast_call = 1 as libc::c_int;
            } else if strcmp(c_option, b"tagged\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                tagged = 1 as libc::c_int;
            } else if strcmp(c_option, b"set_cp\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                set_cp = 1 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    nb_elem = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    if set_cp != 0 {
        Inst_Printf(
            b"prep_cp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    if fast_call != 0 {
        fputs(b"\0" as *const u8 as *const libc::c_char, file_out);
    }
    fprintf(file_out, b"%s(\0" as *const u8 as *const libc::c_char, fct_name);
    i = 0 as libc::c_int;
    adr_of = 0 as libc::c_int;
    loop {
        arg_type = *(top as *mut PlLong);
        top = top.offset(1);
        top;
        match arg_type {
            256 => {
                str = *(top as *mut *mut libc::c_char);
                if *str as libc::c_int == '&' as i32
                    && *str.offset(1 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                {
                    if (i as libc::c_long) < nb_elem - 1 as libc::c_int as libc::c_long
                        && *(top.offset(1 as libc::c_int as isize) as *mut PlLong)
                            == X_Y as libc::c_int as libc::c_long
                        || (i as libc::c_long)
                            < nb_elem - 2 as libc::c_int as libc::c_long
                            && *(top.offset(1 as libc::c_int as isize) as *mut PlLong)
                                == ATOM as libc::c_int as libc::c_long
                            && *(top.offset(3 as libc::c_int as isize) as *mut PlLong)
                                == INTEGER as libc::c_int as libc::c_long
                    {
                        adr_of = 1 as libc::c_int;
                        i += 1;
                        i;
                        top = top.offset(1);
                        top;
                        continue;
                    }
                }
                if adr_of != 0 {
                    aux_functor = *(top as *mut *mut libc::c_char);
                    top = top.offset(1);
                    top;
                    i += 1;
                    i;
                    top = top.offset(1);
                    top;
                    aux_arity = *(top as *mut PlLong);
                    top = top.offset(1);
                    top;
                    Encode_Hexa(
                        0 as *mut libc::c_char,
                        aux_functor,
                        aux_arity as libc::c_int,
                        buff_hexa.as_mut_ptr(),
                    );
                    fprintf(
                        file_out,
                        b"&%s\0" as *const u8 as *const libc::c_char,
                        buff_hexa.as_mut_ptr(),
                    );
                    adr_of = 0 as libc::c_int;
                } else if tagged != 0 {
                    str_atom = *(top as *mut *mut libc::c_char);
                    top = top.offset(1);
                    top;
                    atom = BT_String_Add(&mut bt_tagged_atom, str_atom);
                    fprintf(
                        file_out,
                        b"ta(%d)\0" as *const u8 as *const libc::c_char,
                        (*atom).no,
                    );
                } else {
                    str_atom = *(top as *mut *mut libc::c_char);
                    top = top.offset(1);
                    top;
                    atom = BT_String_Add(&mut bt_atom, str_atom);
                    fprintf(
                        file_out,
                        b"at(%d)\0" as *const u8 as *const libc::c_char,
                        (*atom).no,
                    );
                }
            }
            257 => {
                n = *(top as *mut PlLong);
                top = top.offset(1);
                top;
                fprintf(
                    file_out,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    if tagged != 0 {
                        (n as PlULong) << 3 as libc::c_int
                            | 0x7 as libc::c_int as PlULong
                    } else {
                        n as libc::c_ulong
                    },
                );
            }
            258 => {
                n1 = *(top as *mut libc::c_double);
                top = top.offset(1);
                top;
                fprintf(file_out, b"%1.20e\0" as *const u8 as *const libc::c_char, n1);
            }
            259 => {
                xy = *(top as *mut PlLong);
                top = top.offset(1);
                top;
                if xy < 5000 as libc::c_int as libc::c_long {
                    c = 'X' as i32 as libc::c_char;
                } else {
                    xy -= 5000 as libc::c_int as libc::c_long;
                    c = 'Y' as i32 as libc::c_char;
                }
                if adr_of != 0 {
                    fprintf(file_out, b"&\0" as *const u8 as *const libc::c_char);
                    adr_of = 0 as libc::c_int;
                }
                fprintf(
                    file_out,
                    b"%c(%ld)\0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                    xy,
                );
            }
            260 => {
                if tagged != 0 {
                    let mut str_atom_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut n_0: PlLong = 0;
                    let mut f_n_no: libc::c_int = 0;
                    str_atom_0 = *(top as *mut *mut libc::c_char);
                    top = top.offset(1);
                    top;
                    n_0 = *(top as *mut PlLong);
                    top = top.offset(1);
                    top;
                    f_n_no = Add_F_N_Tagged(str_atom_0, n_0 as libc::c_int);
                    fprintf(
                        file_out,
                        b"fn(%d)\0" as *const u8 as *const libc::c_char,
                        f_n_no,
                    );
                } else {
                    let mut atom_0: *mut BTNode = 0 as *mut BTNode;
                    let mut str_atom_1: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut n_1: PlLong = 0;
                    str_atom_1 = *(top as *mut *mut libc::c_char);
                    top = top.offset(1);
                    top;
                    atom_0 = BT_String_Add(&mut bt_atom, str_atom_1);
                    n_1 = *(top as *mut PlLong);
                    top = top.offset(1);
                    top;
                    fprintf(
                        file_out,
                        b"at(%d),%ld\0" as *const u8 as *const libc::c_char,
                        (*atom_0).no,
                        n_1,
                    );
                }
            }
            _ => {}
        }
        i += 1;
        i;
        if !((i as libc::c_long) < nb_elem) {
            break;
        }
        fputc(',' as i32, file_out);
    }
    fprintf(file_out, b")\n\0" as *const u8 as *const libc::c_char);
    if ret == 1 as libc::c_int {
        Inst_Printf(
            b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if ret == 2 as libc::c_int {
        Inst_Printf(
            b"jump_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if ret == 3 as libc::c_int {
        Inst_Printf(
            b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%c(%ld)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ret_c as libc::c_int,
            ret_xy,
        );
    }
    if set_cp != 0 {
        Inst_Printf(
            b"here_cp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
pub unsafe extern "C" fn Init_Foreign_Table() {
    foreign_tbl[0 as libc::c_int
        as usize] = b"Integer\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    foreign_tbl[1 as libc::c_int
        as usize] = b"Positive\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    foreign_tbl[2 as libc::c_int
        as usize] = b"Float\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[3 as libc::c_int
        as usize] = b"Number\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[4 as libc::c_int
        as usize] = b"Atom\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[5 as libc::c_int
        as usize] = b"Boolean\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    foreign_tbl[6 as libc::c_int
        as usize] = b"Char\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[7 as libc::c_int
        as usize] = b"In_Char\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    foreign_tbl[8 as libc::c_int
        as usize] = b"Code\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[9 as libc::c_int
        as usize] = b"In_Code\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    foreign_tbl[10 as libc::c_int
        as usize] = b"Byte\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[11 as libc::c_int
        as usize] = b"In_Byte\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    foreign_tbl[12 as libc::c_int
        as usize] = b"String\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[13 as libc::c_int
        as usize] = b"Chars\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[14 as libc::c_int
        as usize] = b"Codes\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    foreign_tbl[15 as libc::c_int
        as usize] = b"Term\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn F_foreign_call_c(mut arg: *mut ArgVal) {
    static mut mode: [libc::c_int; 256] = [0; 256];
    static mut type_0: [libc::c_int; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut fio_arg_index: libc::c_int = 0 as libc::c_int;
    let mut nb_c_str: libc::c_int = 0 as libc::c_int;
    let mut s_dup: libc::c_int = 0;
    let mut complex_jump_ret: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut str_mode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut top: *mut ArgVal = arg;
    let mut fct_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret_mode: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bip_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bip_arity: PlLong = 0;
    let mut chc_size: PlLong = 0;
    let mut nb_elem: PlLong = 0;
    fct_name = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    ret_mode = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    bip_name = *(top as *mut *mut libc::c_char);
    top = top.offset(1);
    top;
    bip_arity = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    chc_size = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    nb_elem = *(top as *mut PlLong);
    top = top.offset(1);
    top;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < nb_elem {
        str_mode = *(top as *mut *mut libc::c_char);
        top = top.offset(1);
        top;
        str_type = *(top as *mut *mut libc::c_char);
        top = top.offset(1);
        top;
        if strcmp(str_mode, b"in\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode[i as usize] = 0 as libc::c_int;
        } else if strcmp(str_mode, b"out\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode[i as usize] = 1 as libc::c_int;
        } else if strcmp(str_mode, b"in_out\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            mode[i as usize] = 2 as libc::c_int;
        }
        j = 0 as libc::c_int;
        while !(strcasecmp(foreign_tbl[j as usize], str_type) == 0 as libc::c_int) {
            j += 1;
            if j >= 16 as libc::c_int {
                fprintf(
                    stderr,
                    b"invalid foreign type:%s\n\0" as *const u8 as *const libc::c_char,
                    str_type,
                );
                exit(1 as libc::c_int);
            }
        }
        type_0[i as usize] = j;
        if (mode[i as usize] == 0 as libc::c_int || mode[i as usize] == 2 as libc::c_int)
            && (j == 13 as libc::c_int || j == 14 as libc::c_int)
        {
            nb_c_str += 1;
            nb_c_str;
        }
        i += 1;
        i;
    }
    if chc_size >= 0 as libc::c_int as libc::c_long {
        sprintf(
            l.as_mut_ptr(),
            b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
            cur_pred_no,
            1 as libc::c_int as PlLong,
        );
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Foreign_Create_Choice(&%s,%d,%ld)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            l.as_mut_ptr(),
            cur_arity,
            chc_size,
        );
        Label_Printf(
            b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            l.as_mut_ptr(),
        );
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Foreign_Update_Choice(&%s,%d,%ld)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            l.as_mut_ptr(),
            cur_arity,
            chc_size,
        );
    }
    if *bip_name as libc::c_int != 0 || bip_arity != -(2 as libc::c_int) as libc::c_long
    {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Set_C_Bip_Name(\"%s\",%ld)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            bip_name,
            bip_arity,
        );
    }
    i = 0 as libc::c_int;
    while (i as libc::c_long) < nb_elem {
        n = type_0[i as usize];
        c = (if n == 2 as libc::c_int || n == 3 as libc::c_int {
            'D' as i32
        } else {
            'L' as i32
        }) as libc::c_char;
        s_dup = ((mode[i as usize] == 0 as libc::c_int
            || mode[i as usize] == 2 as libc::c_int)
            && (n == 13 as libc::c_int || n == 14 as libc::c_int)
            && {
                nb_c_str -= 1;
                nb_c_str != 0 as libc::c_int
            }) as libc::c_int;
        match mode[i as usize] {
            0 => {
                if n != 15 as libc::c_int {
                    Inst_Printf(
                        b"call_c\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"Pl_Rd_%s_Check(X(%d))\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        foreign_tbl[n as usize],
                        i,
                    );
                    Inst_Printf(
                        b"move_ret\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"F%c(%d)\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        c as libc::c_int,
                        i,
                    );
                    if s_dup != 0 {
                        Inst_Printf(
                            b"call_c\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            b"Pl_Strdup_Check(FL(%d),\"call generated by %s\",%d)\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            i,
                            b"wam2ma.c\0" as *const u8 as *const libc::c_char,
                            1923 as libc::c_int,
                        );
                        Inst_Printf(
                            b"move_ret\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            b"FL(%d)\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            i,
                        );
                    }
                }
            }
            1 => {
                complex_jump_ret = 1 as libc::c_int;
                if n != 15 as libc::c_int {
                    Inst_Printf(
                        b"call_c\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"Pl_Check_For_Un_%s(X(%d))\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        foreign_tbl[n as usize],
                        i,
                    );
                }
            }
            2 => {
                complex_jump_ret = 1 as libc::c_int;
                if n != 15 as libc::c_int {
                    let fresh53 = fio_arg_index;
                    fio_arg_index = fio_arg_index + 1;
                    Inst_Printf(
                        b"call_c\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"Pl_Foreign_Rd_IO_Arg(%d,X(%d),&Pl_Rd_%s_Check,%d)\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (c as libc::c_int == 'L' as i32) as libc::c_int + s_dup,
                        i,
                        foreign_tbl[n as usize],
                        fresh53,
                    );
                } else {
                    let fresh54 = fio_arg_index;
                    fio_arg_index = fio_arg_index + 1;
                    Inst_Printf(
                        b"call_c\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"Pl_Foreign_Rd_IO_Arg(1,X(%d),0,%d)\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        i,
                        fresh54,
                    );
                }
                Inst_Printf(
                    b"move_ret\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"FL(%d)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    i,
                );
            }
            _ => {}
        }
        i += 1;
        i;
    }
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    );
    fprintf(file_out, b"%s(\0" as *const u8 as *const libc::c_char, fct_name);
    i = 0 as libc::c_int;
    while (i as libc::c_long) < nb_elem {
        n = type_0[i as usize];
        c = (if mode[i as usize] == 2 as libc::c_int {
            'L' as i32
        } else if n == 2 as libc::c_int || n == 3 as libc::c_int {
            'D' as i32
        } else {
            'L' as i32
        }) as libc::c_char;
        if i > 0 as libc::c_int {
            fputc(',' as i32, file_out);
        }
        if mode[i as usize] == 1 as libc::c_int {
            fprintf(file_out, b"&\0" as *const u8 as *const libc::c_char);
        }
        if n == 15 as libc::c_int && mode[i as usize] == 0 as libc::c_int {
            fprintf(file_out, b"X(%d)\0" as *const u8 as *const libc::c_char, i);
        } else {
            fprintf(
                file_out,
                b"F%c(%d)\0" as *const u8 as *const libc::c_char,
                c as libc::c_int,
                i,
            );
        }
        i += 1;
        i;
    }
    fprintf(file_out, b")\n\0" as *const u8 as *const libc::c_char);
    if strcmp(ret_mode, b"jump\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if complex_jump_ret == 0 {
            Inst_Printf(
                b"jump_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            Inst_Printf(
                b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"FL(%d)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                256 as libc::c_int - 1 as libc::c_int,
            );
        }
    } else {
        complex_jump_ret = 0 as libc::c_int;
        if strcmp(ret_mode, b"boolean\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            Inst_Printf(
                b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    i = 0 as libc::c_int;
    while (i as libc::c_long) < nb_elem {
        n = type_0[i as usize];
        c = (if type_0[i as usize] == 2 as libc::c_int
            || type_0[i as usize] == 3 as libc::c_int
        {
            'D' as i32
        } else {
            'L' as i32
        }) as libc::c_char;
        match mode[i as usize] {
            1 => {
                if n != 15 as libc::c_int {
                    Inst_Printf(
                        b"call_c\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"Pl_Un_%s(F%c(%d),X(%d))\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        foreign_tbl[n as usize],
                        c as libc::c_int,
                        i,
                        i,
                    );
                } else {
                    Inst_Printf(
                        b"call_c\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"Pl_Unify(X(%d),FL(%d))\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        i,
                        i,
                    );
                }
                Inst_Printf(
                    b"fail_ret\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            2 => {
                Inst_Printf(
                    b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"Pl_Foreign_Un_IO_Arg(%d,&Pl_Un_%s,FL(%d),X(%d))\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (c as libc::c_int == 'L' as i32) as libc::c_int,
                    foreign_tbl[n as usize],
                    i,
                    i,
                );
                Inst_Printf(
                    b"fail_ret\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if complex_jump_ret != 0 {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Foreign_Jump_Ret(FL(%d))\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            256 as libc::c_int - 1 as libc::c_int,
        );
        Inst_Printf(
            b"jump_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
pub unsafe extern "C" fn Emit_Obj_Initializer() {
    let mut t: *mut SwtTbl = 0 as *mut SwtTbl;
    let mut p: *mut Pred = 0 as *mut Pred;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: [libc::c_char; 32] = [0; 32];
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    Label_Printf(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if bt_atom.nb_elem != 0 {
        Label_Printf(
            b"long local at(%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            bt_atom.nb_elem,
        );
    }
    if bt_tagged_atom.nb_elem != 0 {
        Label_Printf(
            b"long local ta(%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            bt_tagged_atom.nb_elem,
        );
    }
    if bt_tagged_f_n.nb_elem != 0 {
        Label_Printf(
            b"long local fn(%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            bt_tagged_f_n.nb_elem,
        );
    }
    if nb_swt_tbl != 0 {
        Label_Printf(
            b"long local st(%d)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            nb_swt_tbl,
        );
    }
    Label_Printf(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Label_Printf(
        b"c_code  initializer Object_Initializer\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_New_Object(&Prolog_Object_Initializer,&System_Directives,&User_Directives)\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"c_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Label_Printf(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Label_Printf(
        b"c_code  local Prolog_Object_Initializer\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    BT_String_List(
        &mut bt_atom,
        Some(
            Emit_One_Atom
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    BT_String_List(
        &mut bt_tagged_atom,
        Some(
            Emit_One_Atom_Tagged
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    BT_String_List(
        &mut bt_tagged_f_n,
        Some(
            Emit_One_F_N_Tagged
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut libc::c_char,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
    cur_pred_no = 0 as libc::c_int;
    p = dummy_pred_start.next;
    while !p.is_null() {
        fputc('\n' as i32, file_out);
        if (*p).prop & 1 as libc::c_int != 0 {
            q = (*p).hexa;
        } else {
            q = b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Create_Pred(at(%d),%d,at(%d),%d,%d,%s)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*(*p).functor).no,
            (*p).arity,
            (*(*p).pl_file).no,
            (*p).pl_line,
            (*p).prop,
            q,
        );
        cur_pred_no += 1;
        cur_pred_no;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            t = (*p).swt_tbl[i as usize];
            while !t.is_null() {
                Inst_Printf(
                    b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"Pl_Create_Swt_Table(%d)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*t).nb_elem,
                );
                Inst_Printf(
                    b"move_ret\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"st(%d)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*t).tbl_no,
                );
                match i {
                    0 => {
                        j = 0 as libc::c_int;
                        while j < (*t).nb_elem {
                            sprintf(
                                l.as_mut_ptr(),
                                b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
                                cur_pred_no,
                                (*((*t).elem).as_mut_ptr().offset(j as isize)).label,
                            );
                            Inst_Printf(
                                b"call_c\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                b"Pl_Create_Swt_Atm_Element(st(%d),%d,at(%d),&%s)\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                                (*t).tbl_no,
                                (*t).nb_elem,
                                (*(*((*t).elem).as_mut_ptr().offset(j as isize)).atom).no,
                                l.as_mut_ptr(),
                            );
                            j += 1;
                            j;
                        }
                    }
                    _ => {
                        j = 0 as libc::c_int;
                        while j < (*t).nb_elem {
                            sprintf(
                                l.as_mut_ptr(),
                                b".pred%d_%ld\0" as *const u8 as *const libc::c_char,
                                cur_pred_no,
                                (*((*t).elem).as_mut_ptr().offset(j as isize)).label,
                            );
                            Inst_Printf(
                                b"call_c\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                b"Pl_Create_Swt_Stc_Element(st(%d),%d,at(%d),%ld,&%s)\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                                (*t).tbl_no,
                                (*t).nb_elem,
                                (*(*((*t).elem).as_mut_ptr().offset(j as isize)).atom).no,
                                (*((*t).elem).as_mut_ptr().offset(j as isize)).n,
                                l.as_mut_ptr(),
                            );
                            j += 1;
                            j;
                        }
                    }
                }
                t = (*t).next;
            }
            i += 1;
            i;
        }
        p = (*p).next;
    }
    Inst_Printf(
        b"c_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Emit_Exec_Directives() {
    let mut i: libc::c_int = 0;
    let mut p: *mut Direct = 0 as *mut Direct;
    fputc('\n' as i32, file_out);
    Label_Printf(
        b"c_code  local System_Directives\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    p = dummy_direct_start.next;
    while !p.is_null() {
        i += 1;
        i;
        if !((*p).system == 0) {
            Inst_Printf(
                b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Pl_Execute_Directive(at(%d),%d,%d,&directive_%d)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*(*p).pl_file).no,
                (*p).pl_line,
                1 as libc::c_int,
                i,
            );
        }
        p = (*p).next;
    }
    Inst_Printf(
        b"c_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    fputc('\n' as i32, file_out);
    Label_Printf(
        b"c_code  local User_Directives\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    p = dummy_direct_start.next;
    while !p.is_null() {
        i += 1;
        i;
        if !((*p).system != 0) {
            Inst_Printf(
                b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Pl_Execute_Directive(at(%d),%d,%d,&directive_%d)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*(*p).pl_file).no,
                (*p).pl_line,
                0 as libc::c_int,
                i,
            );
        }
        p = (*p).next;
    }
    Inst_Printf(
        b"c_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Emit_One_Atom(
    mut no: libc::c_int,
    mut str: *mut libc::c_char,
    mut info: *mut libc::c_void,
) {
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Create_Atom(\"%s\")\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        str,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"at(%d)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        no,
    );
}
pub unsafe extern "C" fn Emit_One_Atom_Tagged(
    mut no: libc::c_int,
    mut str: *mut libc::c_char,
    mut info: *mut libc::c_void,
) {
    let mut atom: *mut BTNode = BT_String_Lookup(&mut bt_atom, str);
    if !atom.is_null() {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Put_Atom(at(%d))\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*atom).no,
        );
    } else {
        Inst_Printf(
            b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Pl_Create_Atom_Tagged(\"%s\")\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            str,
        );
    }
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ta(%d)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        no,
    );
}
pub unsafe extern "C" fn Add_F_N_Tagged(
    mut atom: *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = strlen(atom) as libc::c_int;
    atom = realloc(
        atom as *mut libc::c_void,
        (l + 5 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    sprintf(atom.offset(l as isize), b"/%d\0" as *const u8 as *const libc::c_char, n);
    return (*BT_String_Add(&mut bt_tagged_f_n, atom)).no;
}
pub unsafe extern "C" fn Emit_One_F_N_Tagged(
    mut no: libc::c_int,
    mut str: *mut libc::c_char,
    mut info: *mut libc::c_void,
) {
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_char = str
        .offset(strlen(str) as isize)
        .offset(-(1 as libc::c_int as isize));
    p = str.offset(strlen(str) as isize).offset(-(1 as libc::c_int as isize));
    while *p as libc::c_int != '/' as i32 {
        p = p.offset(-1);
        p;
    }
    n = atoi(p.offset(1 as libc::c_int as isize));
    *p = '\0' as i32 as libc::c_char;
    Inst_Printf(
        b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Pl_Create_Functor_Arity_Tagged(\"%s\",%d)\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        str,
        n,
    );
    Inst_Printf(
        b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"fn(%d)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        no,
    );
}
pub unsafe extern "C" fn Label_Printf(mut label: *mut libc::c_char, mut args: ...) {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    vfprintf(file_out, label, arg_ptr.as_va_list());
    fputc('\n' as i32, file_out);
}
pub unsafe extern "C" fn Inst_Printf(
    mut op: *mut libc::c_char,
    mut operands: *mut libc::c_char,
    mut args: ...
) {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    arg_ptr = args.clone();
    fprintf(file_out, b"\t%-10s \0" as *const u8 as *const libc::c_char, op);
    if !operands.is_null() {
        vfprintf(file_out, operands, arg_ptr.as_va_list());
        fputc('\n' as i32, file_out);
    }
}
pub unsafe extern "C" fn Parse_Arguments(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    static mut str: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    file_name_out = 0 as *mut libc::c_char;
    file_name_in = file_name_out;
    comment = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if **argv.offset(i as isize) as libc::c_int == '-' as i32
            && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
        {
            if strncmp(
                *argv.offset(i as isize),
                b"-o\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
                || strncmp(
                    *argv.offset(i as isize),
                    b"--output\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
            {
                i += 1;
                if i >= argc {
                    fprintf(
                        stderr,
                        b"FILE missing after %s option\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset((i - 1 as libc::c_int) as isize),
                    );
                    exit(1 as libc::c_int);
                }
                file_name_out = *argv.offset(i as isize);
            } else if strncmp(
                *argv.offset(i as isize),
                b"--comment\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                comment = 1 as libc::c_int;
            } else {
                if strncmp(
                    *argv.offset(i as isize),
                    b"--version\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                {
                    Display_Copying(
                        b"WAM to Mini-Assembly Compiler\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
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
                fprintf(
                    stderr,
                    b"unknown option %s - try wam2ma --help\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                );
                exit(1 as libc::c_int);
            }
        } else {
            if !file_name_in.is_null() {
                fprintf(
                    stderr,
                    b"input file already specified (%s)\n\0" as *const u8
                        as *const libc::c_char,
                    file_name_in,
                );
                exit(1 as libc::c_int);
            }
            file_name_in = *argv.offset(i as isize);
        }
        i += 1;
        i;
    }
    if !file_name_in.is_null()
        && strcmp(file_name_in, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        file_name_in = 0 as *mut libc::c_char;
    }
    if file_name_out.is_null() && !file_name_in.is_null() {
        strcpy(str.as_mut_ptr(), file_name_in);
        i = strlen(str.as_mut_ptr()) as libc::c_int;
        if strcmp(
            str.as_mut_ptr().offset(i as isize).offset(-(4 as libc::c_int as isize)),
            b".wam\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            strcpy(
                str.as_mut_ptr().offset(i as isize).offset(-(4 as libc::c_int as isize)),
                b".ma\0" as *const u8 as *const libc::c_char,
            );
        } else {
            strcpy(
                str.as_mut_ptr().offset(i as isize),
                b".ma\0" as *const u8 as *const libc::c_char,
            );
        }
        file_name_out = str.as_mut_ptr();
    }
    if !file_name_out.is_null()
        && strcmp(file_name_out, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        file_name_out = 0 as *mut libc::c_char;
    }
}
pub unsafe extern "C" fn Display_Help() {
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Usage: wam2ma [OPTION...] FILE\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Options:\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -o FILE, --output FILE      set output file name\0" as *const u8
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
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"'-' can be given as FILE for the standard input/output\0" as *const u8
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
