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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn Parse_Ma_File(
        file_name_in_0: *mut libc::c_char,
        comment_0: libc::c_int,
    ) -> libc::c_int;
    fn Init_Mapper();
    fn Asm_Start();
    fn Asm_Stop();
    fn Label(label: *mut libc::c_char);
    fn Pl_Fail();
    fn Call_C_Start(
        fct_name: *mut libc::c_char,
        fc: Bool,
        nb_args: libc::c_int,
        nb_args_in_words: libc::c_int,
    );
    fn Call_C_Arg_Int(offset: libc::c_int, int_val: PlLong) -> libc::c_int;
    fn Call_C_Arg_Double(offset: libc::c_int, d: *mut DoubleInf) -> libc::c_int;
    fn Call_C_Arg_String(offset: libc::c_int, s: *mut StringInf) -> libc::c_int;
    fn Call_C_Arg_Mem_L(
        offset: libc::c_int,
        adr_of: Bool,
        name: *mut libc::c_char,
        index: libc::c_int,
    ) -> libc::c_int;
    fn Call_C_Arg_Reg_X(
        offset: libc::c_int,
        adr_of: Bool,
        index: libc::c_int,
    ) -> libc::c_int;
    fn Call_C_Arg_Reg_Y(
        offset: libc::c_int,
        adr_of: Bool,
        index: libc::c_int,
    ) -> libc::c_int;
    fn Call_C_Arg_Foreign_L(
        offset: libc::c_int,
        adr_of: Bool,
        index: libc::c_int,
    ) -> libc::c_int;
    fn Call_C_Arg_Foreign_D(
        offset: libc::c_int,
        adr_of: Bool,
        index: libc::c_int,
    ) -> libc::c_int;
    fn Call_C_Invoke(
        fct_name: *mut libc::c_char,
        fc: Bool,
        nb_args: libc::c_int,
        nb_args_in_words: libc::c_int,
    );
    fn Call_C_Stop(fct_name: *mut libc::c_char, nb_args: libc::c_int);
    fn Cmp_Ret_And_Int(int_val: PlLong);
    fn Jump_If_Equal(label: *mut libc::c_char);
    fn Jump_If_Greater(label: *mut libc::c_char);
    fn Dico_String_Start(nb: libc::c_int);
    fn Dico_String(s: *mut StringInf);
    fn Dico_String_Stop(nb: libc::c_int);
    fn Dico_Double_Start(nb: libc::c_int);
    fn Dico_Double(d: *mut DoubleInf);
    fn Dico_Double_Stop(nb: libc::c_int);
    fn Dico_Long_Start(nb: libc::c_int);
    fn Dico_Long(l: *mut LongInf);
    fn Dico_Long_Stop(nb: libc::c_int);
    fn Data_Start(initializer_fct_0: *mut libc::c_char);
    fn Data_Stop(initializer_fct_0: *mut libc::c_char);
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
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
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
pub type BTNode = btnode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BTString {
    pub tree: *mut BTNode,
    pub nb_elem: libc::c_int,
}
pub type BTStrLstFct = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char, *mut libc::c_void) -> (),
>;
pub type Bool = libc::c_int;
pub type ArgTyp = libc::c_uint;
pub const STRING: ArgTyp = 264;
pub const FD_ARRAY: ArgTyp = 263;
pub const FL_ARRAY: ArgTyp = 262;
pub const Y_REG: ArgTyp = 261;
pub const X_REG: ArgTyp = 260;
pub const MEM: ArgTyp = 259;
pub const FLOAT: ArgTyp = 258;
pub const INTEGER: ArgTyp = 257;
pub const IDENTIFIER: ArgTyp = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArgInf {
    pub type_0: ArgTyp,
    pub adr_of: Bool,
    pub str_val: *mut libc::c_char,
    pub int_val: PlLong,
    pub dbl_val: libc::c_double,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwtInf {
    pub int_val: PlLong,
    pub label: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapperInf {
    pub can_produce_pic_code: Bool,
    pub needs_pre_pass: Bool,
    pub comment_prefix: *mut libc::c_char,
    pub local_symb_prefix: *mut libc::c_char,
    pub string_symb_prefix: *mut libc::c_char,
    pub double_symb_prefix: *mut libc::c_char,
    pub strings_need_null: Bool,
    pub call_c_reverse_args: Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodeInf {
    pub name: *mut libc::c_char,
    pub prolog: Bool,
    pub initializer: Bool,
    pub global: Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringInf {
    pub no: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub symb: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DoubleInf {
    pub no: libc::c_int,
    pub ma_str: *mut libc::c_char,
    pub is_ma_str_human: Bool,
    pub cmt_str: *mut libc::c_char,
    pub symb: *mut libc::c_char,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dbl: libc::c_double,
    pub i64_0: int64_t,
    pub i32_0: [int32_t; 2],
}
pub type VType = libc::c_uint;
pub const ARRAY_SIZE: VType = 2;
pub const INITIAL_VALUE: VType = 1;
pub const NONE: VType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongInf {
    pub name: *mut libc::c_char,
    pub global: Bool,
    pub vtype: VType,
    pub value: PlLong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LabelGen {
    pub prefix: *mut libc::c_char,
    pub no: libc::c_int,
    pub label: [libc::c_char; 256],
}
pub unsafe extern "C" fn BT_String_Init(mut bt_str: *mut BTString) {
    (*bt_str).tree = 0 as *mut BTNode;
    (*bt_str).nb_elem = 0 as libc::c_int;
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
pub static mut file_name_in: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut file_name_out: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut comment: Bool = 0;
pub static mut pic_code: Bool = 0;
pub static mut ignore_fc: Bool = 0;
pub static mut mi: MapperInf = MapperInf {
    can_produce_pic_code: 0,
    needs_pre_pass: 0,
    comment_prefix: 0 as *const libc::c_char as *mut libc::c_char,
    local_symb_prefix: 0 as *const libc::c_char as *mut libc::c_char,
    string_symb_prefix: 0 as *const libc::c_char as *mut libc::c_char,
    double_symb_prefix: 0 as *const libc::c_char as *mut libc::c_char,
    strings_need_null: 0,
    call_c_reverse_args: 0,
};
pub static mut lg_cont: LabelGen = LabelGen {
    prefix: 0 as *const libc::c_char as *mut libc::c_char,
    no: 0,
    label: [0; 256],
};
pub static mut file_out: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut bt_code: BTString = BTString {
    tree: 0 as *const BTNode as *mut BTNode,
    nb_elem: 0,
};
pub static mut bt_long: BTString = BTString {
    tree: 0 as *const BTNode as *mut BTNode,
    nb_elem: 0,
};
pub static mut bt_string: BTString = BTString {
    tree: 0 as *const BTNode as *mut BTNode,
    nb_elem: 0,
};
pub static mut bt_double: BTString = BTString {
    tree: 0 as *const BTNode as *mut BTNode,
    nb_elem: 0,
};
pub static mut initializer_fct: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut local_label: [libc::c_char; 64] = [0; 64];
pub static mut local_label_count: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    mi.can_produce_pic_code = 0 as libc::c_int;
    mi.needs_pre_pass = 0 as libc::c_int;
    mi.comment_prefix = b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    mi
        .local_symb_prefix = b"L\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    mi
        .string_symb_prefix = b"LC\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    mi
        .double_symb_prefix = b"LD\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    mi.strings_need_null = 0 as libc::c_int;
    mi.call_c_reverse_args = 0 as libc::c_int;
    Init_Mapper();
    Parse_Arguments(argc, argv);
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
    BT_String_Init(&mut bt_code);
    BT_String_Init(&mut bt_long);
    BT_String_Init(&mut bt_string);
    BT_String_Init(&mut bt_double);
    Label_Gen_Init(
        &mut lg_cont,
        b"cont\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Asm_Start();
    if Parse_Ma_File(file_name_in, comment) == 0 {
        fprintf(stderr, b"Translation aborted\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    Data_Start(initializer_fct);
    n = bt_string.nb_elem;
    if n != 0 {
        Dico_String_Start(n);
        BT_String_List(
            &mut bt_string,
            Some(
                Invoke_Dico_String
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> (),
            ),
        );
        Dico_String_Stop(n);
    }
    n = bt_double.nb_elem;
    if n != 0 {
        Dico_Double_Start(n);
        BT_String_List(
            &mut bt_double,
            Some(
                Invoke_Dico_Double
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> (),
            ),
        );
        Dico_Double_Stop(n);
    }
    n = bt_long.nb_elem;
    if n != 0 {
        Dico_Long_Start(n);
        BT_String_List(
            &mut bt_long,
            Some(
                Invoke_Dico_Long
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut libc::c_char,
                        *mut libc::c_void,
                    ) -> (),
            ),
        );
        Dico_Long_Stop(n);
    }
    Data_Stop(initializer_fct);
    Asm_Stop();
    if file_out != stdout {
        fclose(file_out);
    }
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn Invoke_Dico_String(
    mut no: libc::c_int,
    mut str: *mut libc::c_char,
    mut info: *mut libc::c_void,
) {
    let mut d: *mut StringInf = info as *mut StringInf;
    Dico_String(d);
}
pub unsafe extern "C" fn Invoke_Dico_Double(
    mut no: libc::c_int,
    mut str: *mut libc::c_char,
    mut info: *mut libc::c_void,
) {
    let mut d: *mut DoubleInf = info as *mut DoubleInf;
    if comment != 0 {
        Inst_Printf(
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            mi.comment_prefix,
            (*d).cmt_str,
        );
    }
    Dico_Double(d);
}
pub unsafe extern "C" fn Invoke_Dico_Long(
    mut no: libc::c_int,
    mut name: *mut libc::c_char,
    mut info: *mut libc::c_void,
) {
    let mut l: *mut LongInf = info as *mut LongInf;
    Dico_Long(l);
}
pub unsafe extern "C" fn Record_String(mut str: *mut libc::c_char) -> *mut StringInf {
    let mut label: [libc::c_char; 32] = [0; 32];
    let mut b: *mut BTNode = BT_String_Add(&mut bt_string, str);
    let mut s: *mut StringInf = &mut (*b).info as *mut [libc::c_char; 128]
        as *mut StringInf;
    (*s).no = (*b).no;
    (*s).str_0 = str;
    sprintf(
        label.as_mut_ptr(),
        b"%s%d\0" as *const u8 as *const libc::c_char,
        mi.string_symb_prefix,
        (*s).no,
    );
    (*s).symb = strdup(label.as_mut_ptr());
    return s;
}
pub unsafe extern "C" fn Record_Double(
    mut ma_str: *mut libc::c_char,
    mut dbl_val: libc::c_double,
) -> *mut DoubleInf {
    let mut label: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut cmt: [libc::c_char; 64] = [0; 64];
    let mut b: *mut BTNode = BT_String_Add(&mut bt_double, ma_str);
    let mut d: *mut DoubleInf = &mut (*b).info as *mut [libc::c_char; 128]
        as *mut DoubleInf;
    (*d).no = (*b).no;
    (*d).ma_str = ma_str;
    sprintf(
        label.as_mut_ptr(),
        b"%s%d\0" as *const u8 as *const libc::c_char,
        mi.double_symb_prefix,
        (*d).no,
    );
    (*d).symb = strdup(label.as_mut_ptr());
    (*d).v.dbl = dbl_val;
    p = ma_str;
    while *p as libc::c_int != 0
        && !(strchr(
            b"0123456789.-eE\0" as *const u8 as *const libc::c_char,
            *p as libc::c_int,
        ))
            .is_null()
    {
        p = p.offset(1);
        p;
    }
    (*d).is_ma_str_human = (*p as libc::c_int == '\0' as i32) as libc::c_int;
    if (*d).is_ma_str_human != 0 {
        (*d).cmt_str = (*d).ma_str;
    } else {
        sprintf(
            cmt.as_mut_ptr(),
            b"%s = %1.17g\0" as *const u8 as *const libc::c_char,
            ma_str,
            dbl_val,
        );
        (*d).cmt_str = strdup(cmt.as_mut_ptr());
    }
    return d;
}
pub unsafe extern "C" fn Declare_Initializer(mut init_fct: *mut libc::c_char) {
    initializer_fct = init_fct;
}
pub unsafe extern "C" fn Decl_Code(mut c: *mut CodeInf) {
    let mut c1: *mut CodeInf = 0 as *mut CodeInf;
    c1 = &mut (*(BT_String_Add
        as unsafe extern "C" fn(
            *mut BTString,
            *mut libc::c_char,
        ) -> *mut BTNode)(&mut bt_code, (*c).name))
        .info as *mut [libc::c_char; 128] as *mut CodeInf;
    *c1 = *c;
}
pub unsafe extern "C" fn Decl_Long(mut l: *mut LongInf) {
    let mut l1: *mut LongInf = 0 as *mut LongInf;
    l1 = &mut (*(BT_String_Add
        as unsafe extern "C" fn(
            *mut BTString,
            *mut libc::c_char,
        ) -> *mut BTNode)(&mut bt_long, (*l).name))
        .info as *mut [libc::c_char; 128] as *mut LongInf;
    *l1 = *l;
}
pub unsafe extern "C" fn Is_Code_Defined(mut name: *mut libc::c_char) -> libc::c_int {
    return (BT_String_Lookup(&mut bt_code, name)
        != 0 as *mut libc::c_void as *mut BTNode) as libc::c_int;
}
pub unsafe extern "C" fn Get_Code_Infos(mut name: *mut libc::c_char) -> *mut CodeInf {
    let mut b: *mut BTNode = BT_String_Lookup(&mut bt_code, name);
    return if b.is_null() {
        0 as *mut CodeInf
    } else {
        &mut (*b).info as *mut [libc::c_char; 128] as *mut CodeInf
    };
}
pub unsafe extern "C" fn Get_Long_Infos(mut name: *mut libc::c_char) -> *mut LongInf {
    let mut b: *mut BTNode = BT_String_Lookup(&mut bt_long, name);
    return if b.is_null() {
        0 as *mut LongInf
    } else {
        &mut (*b).info as *mut [libc::c_char; 128] as *mut LongInf
    };
}
pub unsafe extern "C" fn Scope_Of_Symbol(mut name: *mut libc::c_char) -> libc::c_int {
    let mut c: *mut CodeInf = 0 as *mut CodeInf;
    let mut l: *mut LongInf = 0 as *mut LongInf;
    if *name as libc::c_int == '.' as i32
        || strncmp(name, mi.local_symb_prefix, strlen(mi.local_symb_prefix))
            == 0 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    c = Get_Code_Infos(name);
    if !c.is_null() {
        return if (*c).global != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
    }
    l = Get_Long_Infos(name);
    if !l.is_null() {
        return if (*l).global != 0 { 0 as libc::c_int } else { 2 as libc::c_int };
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Call_C(
    mut fct_name: *mut libc::c_char,
    mut fc: Bool,
    mut nb_args: libc::c_int,
    mut nb_args_in_words: libc::c_int,
    mut arg: *mut ArgInf,
) {
    let mut i: libc::c_uint = 0;
    let mut inc: libc::c_int = 0;
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut s: *mut StringInf = 0 as *mut StringInf;
    let mut d: *mut DoubleInf = 0 as *mut DoubleInf;
    if ignore_fc != 0 {
        fc = 0 as libc::c_int;
    }
    Call_C_Start(fct_name, fc, nb_args, nb_args_in_words);
    if mi.call_c_reverse_args == 0 {
        i = 0 as libc::c_int as libc::c_uint;
        inc = 1 as libc::c_int;
    } else {
        i = (nb_args - 1 as libc::c_int) as libc::c_uint;
        inc = -(1 as libc::c_int);
    }
    while i < nb_args as libc::c_uint {
        match (*arg.offset(i as isize)).type_0 as libc::c_uint {
            257 => {
                offset += Call_C_Arg_Int(offset, (*arg.offset(i as isize)).int_val);
            }
            258 => {
                d = Record_Double(
                    (*arg.offset(i as isize)).str_val,
                    (*arg.offset(i as isize)).dbl_val,
                );
                if comment != 0 {
                    Inst_Printf(
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b"%s %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        mi.comment_prefix,
                        (*d).cmt_str,
                    );
                }
                offset += Call_C_Arg_Double(offset, d);
            }
            264 => {
                s = Record_String((*arg.offset(i as isize)).str_val);
                if comment != 0 {
                    Inst_Printf(
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        b"%s %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        mi.comment_prefix,
                        (*s).str_0,
                    );
                }
                offset += Call_C_Arg_String(offset, s);
            }
            259 => {
                offset
                    += Call_C_Arg_Mem_L(
                        offset,
                        (*arg.offset(i as isize)).adr_of,
                        (*arg.offset(i as isize)).str_val,
                        (*arg.offset(i as isize)).index,
                    );
            }
            260 => {
                offset
                    += Call_C_Arg_Reg_X(
                        offset,
                        (*arg.offset(i as isize)).adr_of,
                        (*arg.offset(i as isize)).index,
                    );
            }
            261 => {
                offset
                    += Call_C_Arg_Reg_Y(
                        offset,
                        (*arg.offset(i as isize)).adr_of,
                        (*arg.offset(i as isize)).index,
                    );
            }
            262 => {
                offset
                    += Call_C_Arg_Foreign_L(
                        offset,
                        (*arg.offset(i as isize)).adr_of,
                        (*arg.offset(i as isize)).index,
                    );
            }
            263 => {
                offset
                    += Call_C_Arg_Foreign_D(
                        offset,
                        (*arg.offset(i as isize)).adr_of,
                        (*arg.offset(i as isize)).index,
                    );
            }
            _ => {}
        }
        i = i.wrapping_add(inc as libc::c_uint);
    }
    Call_C_Invoke(fct_name, fc, nb_args, nb_args_in_words);
    Call_C_Stop(fct_name, nb_args);
}
pub unsafe extern "C" fn Switch_Ret(mut nb_swt: libc::c_int, mut swt: *mut SwtInf) {
    qsort(
        swt as *mut libc::c_void,
        nb_swt as size_t,
        ::std::mem::size_of::<SwtInf>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut SwtInf, *mut SwtInf) -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                Switch_Cmp_Int
                    as unsafe extern "C" fn(*mut SwtInf, *mut SwtInf) -> libc::c_int,
            ),
        ),
    );
    Switch_Rec(0 as libc::c_int, nb_swt - 1 as libc::c_int, swt);
}
pub unsafe extern "C" fn Switch_Rec(
    mut start: libc::c_int,
    mut stop: libc::c_int,
    mut swt: *mut SwtInf,
) {
    let mut mid: libc::c_int = 0;
    let mut lab_cont: [libc::c_char; 32] = [0; 32];
    match stop - start + 1 as libc::c_int {
        1 => {
            Switch_Equal(swt.offset(start as isize));
            Pl_Fail();
        }
        2 => {
            Switch_Equal(swt.offset(start as isize));
            Switch_Equal(swt.offset(stop as isize));
            Pl_Fail();
        }
        3 => {
            Switch_Equal(swt.offset(start as isize));
            Switch_Equal(swt.offset(start as isize).offset(1 as libc::c_int as isize));
            Switch_Equal(swt.offset(stop as isize));
            Pl_Fail();
        }
        _ => {
            mid = (start + stop) / 2 as libc::c_int;
            Switch_Equal(swt.offset(mid as isize));
            strcpy(lab_cont.as_mut_ptr(), Label_Gen_New(&mut lg_cont));
            Jump_If_Greater(lab_cont.as_mut_ptr());
            Switch_Rec(start, mid - 1 as libc::c_int, swt);
            Label(lab_cont.as_mut_ptr());
            Switch_Rec(mid + 1 as libc::c_int, stop, swt);
        }
    };
}
pub unsafe extern "C" fn Switch_Equal(mut c: *mut SwtInf) {
    Cmp_Ret_And_Int((*c).int_val);
    Jump_If_Equal((*c).label);
}
pub unsafe extern "C" fn Switch_Cmp_Int(
    mut c1: *mut SwtInf,
    mut c2: *mut SwtInf,
) -> libc::c_int {
    return ((*c1).int_val - (*c2).int_val) as libc::c_int;
}
pub unsafe extern "C" fn Label_Gen_Init(
    mut g: *mut LabelGen,
    mut prefix: *mut libc::c_char,
) {
    (*g).prefix = prefix;
    (*g).no = 0 as libc::c_int;
    (*g).label[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn Label_Gen_New(mut g: *mut LabelGen) -> *mut libc::c_char {
    (*g).no += 1;
    sprintf(
        ((*g).label).as_mut_ptr(),
        b"%s%s%d\0" as *const u8 as *const libc::c_char,
        mi.local_symb_prefix,
        (*g).prefix,
        (*g).no,
    );
    return ((*g).label).as_mut_ptr();
}
pub unsafe extern "C" fn Label_Gen_Get(mut g: *mut LabelGen) -> *mut libc::c_char {
    return ((*g).label).as_mut_ptr();
}
pub unsafe extern "C" fn Label_Gen_No(mut g: *mut LabelGen) -> libc::c_int {
    return (*g).no;
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
    fprintf(file_out, b"\t%s\t\0" as *const u8 as *const libc::c_char, op);
    vfprintf(file_out, operands, arg_ptr.as_va_list());
    fputc('\n' as i32, file_out);
}
pub unsafe extern "C" fn Inst_Out(
    mut op: *mut libc::c_char,
    mut operands: *mut libc::c_char,
) {
    fprintf(file_out, b"\t%s\t%s\n\0" as *const u8 as *const libc::c_char, op, operands);
}
pub unsafe extern "C" fn Char_Out(mut c: libc::c_char) {
    fprintf(file_out, b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
}
pub unsafe extern "C" fn String_Out(mut s: *mut libc::c_char) {
    fprintf(file_out, b"%s\0" as *const u8 as *const libc::c_char, s);
}
pub unsafe extern "C" fn Int_Out(mut d: libc::c_int) {
    fprintf(file_out, b"%d\0" as *const u8 as *const libc::c_char, d);
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
    pic_code = 0 as libc::c_int;
    ignore_fc = 0 as libc::c_int;
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
                b"--pic\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
                || strncmp(
                    *argv.offset(i as isize),
                    b"-fPIC\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
            {
                if mi.can_produce_pic_code != 0 {
                    pic_code = 1 as libc::c_int;
                } else {
                    fprintf(
                        stderr,
                        b"ignored option %s - cannot produce PIC code for this architecture\n\0"
                            as *const u8 as *const libc::c_char,
                        *argv.offset(i as isize),
                    );
                }
            } else if strncmp(
                *argv.offset(i as isize),
                b"--ignore-fast\0" as *const u8 as *const libc::c_char,
                strlen(*argv.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                ignore_fc = 1 as libc::c_int;
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
                        b"Mini-Assembly to Assembly Compiler\0" as *const u8
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
                    b"unknown option %s - try ma2asm --help\n\0" as *const u8
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
            str.as_mut_ptr().offset(i as isize).offset(-(3 as libc::c_int as isize)),
            b".ma\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            strcpy(
                str.as_mut_ptr().offset(i as isize).offset(-(3 as libc::c_int as isize)),
                b".s\0" as *const u8 as *const libc::c_char,
            );
        } else {
            strcpy(
                str.as_mut_ptr().offset(i as isize),
                b".s\0" as *const u8 as *const libc::c_char,
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
        b"Usage: ma2asm [option...] file\0" as *const u8 as *const libc::c_char,
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
        b"  --pic                       produce position independent code (PIC)\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --ignore-fast               ignore fast call (FC) declarations\0"
            as *const u8 as *const libc::c_char,
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
        b"'-' can be given as <file> for the standard input/output\0" as *const u8
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
