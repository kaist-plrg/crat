use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn Pl_Hash_Buffer(data: *const libc::c_void, len: libc::c_int) -> uint32_t;
    static mut pl_fixed_sizes: PlLong;
    fn Pl_Strdup_Check(
        str: *mut libc::c_char,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_Calloc_Check(
        nb: libc::c_uint,
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_LE_Compl_Add_Word(
        word: *mut libc::c_char,
        word_length: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uint32_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AtomProp {
    #[bitfield(name = "length", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "op_mask", ty = "libc::c_uint", bits = "16..=19")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "20..=21")]
    #[bitfield(name = "needs_quote", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "needs_scan", ty = "libc::c_uint", bits = "23..=23")]
    pub length_op_mask_type_0_needs_quote_needs_scan: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AtomInf {
    pub name: *mut libc::c_char,
    pub hash: libc::c_uint,
    pub prop: AtomProp,
    pub info: *mut libc::c_void,
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_atom_tbl: *mut AtomInf = 0 as *const AtomInf as *mut AtomInf;
pub static mut pl_max_atom: PlULong = 0;
pub static mut pl_nb_atom: PlULong = 0;
pub static mut pl_atom_void: libc::c_int = 0;
pub static mut pl_atom_curly_brackets: libc::c_int = 0;
pub static mut pl_atom_false: libc::c_int = 0;
pub static mut pl_atom_true: libc::c_int = 0;
pub static mut pl_atom_end_of_file: libc::c_int = 0;
pub static mut pl_char_conv: [libc::c_char; 256] = [0; 256];
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_le_hook_start: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_put_char: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_char0: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_emit_beep: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_ins_mode: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_screen_size: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_kbd_is_not_empty: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_backd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_forwd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ_str: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_erase: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_set_line_buffering: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_line_buffering: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_flush: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_confirm_box: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_message_box: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_exit_process: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_initialize: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_char_type: [libc::c_int; 256] = [
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    8 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    2048 as libc::c_int,
    32 as libc::c_int,
    4 as libc::c_int,
    64 as libc::c_int,
    64 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    2 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    128 as libc::c_int,
    32 as libc::c_int,
    2 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    32 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    512 as libc::c_int,
    64 as libc::c_int,
    32 as libc::c_int,
    64 as libc::c_int,
    32 as libc::c_int,
    256 as libc::c_int,
    16 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    1024 as libc::c_int,
    64 as libc::c_int,
    64 as libc::c_int,
    64 as libc::c_int,
    32 as libc::c_int,
    1 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub static mut pl_escape_symbol: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"abfnrtv\0")
};
pub static mut pl_escape_char: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<
        &[u8; 8],
        &mut [libc::c_char; 8],
    >(b"\x07\x08\x0C\n\r\t\x0B\0")
};
static mut str_char: [[libc::c_char; 2]; 256] = [[0; 2]; 256];
pub unsafe extern "C" fn Pl_Init_Atom() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if pl_max_atom < 256 as libc::c_int as libc::c_ulong {
        pl_max_atom = 256 as libc::c_int as PlULong;
    }
    if pl_max_atom <= 256 as libc::c_int as libc::c_ulong {
        pl_max_atom = (256 as libc::c_int + 1 as libc::c_int) as PlULong;
    }
    if pl_max_atom
        > (1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
    {
        pl_max_atom = (1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong);
    }
    pl_atom_tbl = Pl_Calloc_Check(
        pl_max_atom as libc::c_uint,
        ::std::mem::size_of::<AtomInf>() as libc::c_ulong as libc::c_uint,
        b"atom.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        156 as libc::c_int,
    ) as *mut AtomInf;
    pl_nb_atom = 0 as libc::c_int as PlULong;
    c = 128 as libc::c_int;
    while c < 256 as libc::c_int {
        pl_char_type[c
            as usize] = if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            1024 as libc::c_int
        } else if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            512 as libc::c_int
        } else {
            4096 as libc::c_int
        };
        c += 1;
        c;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        pl_char_conv[i as usize] = i as libc::c_char;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        str_char[i as usize][0 as libc::c_int as usize] = i as libc::c_char;
        str_char[i as usize][1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        Pl_Create_Atom((str_char[i as usize]).as_mut_ptr());
        i += 1;
        i;
    }
    i = Pl_Create_Atom(b"[]\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if i != 256 as libc::c_int {
        Pl_Fatal_Error(
            b"atom: invalid ATOM_NIL (should be %d)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            i,
        );
    }
    pl_atom_void = Pl_Create_Atom(
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_curly_brackets = Pl_Create_Atom(
        b"{}\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_false = Pl_Create_Atom(
        b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_true = Pl_Create_Atom(
        b"true\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_end_of_file = Pl_Create_Atom(
        b"end_of_file\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Pl_Create_Allocate_Atom(
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = strlen(name) as libc::c_int;
    let mut hash: libc::c_uint = Hash_String(name, len);
    let mut patom: *mut AtomInf = Locate_Atom(name, hash);
    return Add_Atom(name, len, hash, patom, 1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Create_Atom(mut name: *mut libc::c_char) -> libc::c_int {
    let mut len: libc::c_int = strlen(name) as libc::c_int;
    let mut hash: libc::c_uint = Hash_String(name, len);
    let mut patom: *mut AtomInf = Locate_Atom(name, hash);
    return Add_Atom(name, len, hash, patom, 0 as libc::c_int);
}
unsafe extern "C" fn Add_Atom(
    mut name: *mut libc::c_char,
    mut len: libc::c_int,
    mut hash: libc::c_uint,
    mut patom: *mut AtomInf,
    mut allocate: Bool,
) -> libc::c_int {
    let mut prop: AtomProp = AtomProp {
        length_op_mask_type_0_needs_quote_needs_scan: [0; 3],
        c2rust_padding: [0; 1],
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c_type: libc::c_int = 0;
    let mut identifier: Bool = 0;
    let mut graphic: Bool = 0;
    if patom.is_null() {
        Error_Table_Full();
    }
    if !((*patom).name).is_null() {
        return patom.offset_from(pl_atom_tbl) as libc::c_long as libc::c_int;
    }
    if allocate != 0 {
        name = Pl_Strdup_Check(
            name,
            b"atom.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            251 as libc::c_int,
        );
    }
    pl_nb_atom = pl_nb_atom.wrapping_add(1);
    pl_nb_atom;
    (*patom).name = name;
    (*patom).hash = hash;
    prop.set_needs_scan(0 as libc::c_int as libc::c_uint);
    graphic = (*name as libc::c_int != '\0' as i32) as libc::c_int;
    identifier = graphic;
    p = name;
    while *p != 0 {
        c_type = pl_char_type[*p as libc::c_uchar as usize];
        if c_type
            & (256 as libc::c_int | 512 as libc::c_int | 1024 as libc::c_int
                | 128 as libc::c_int) == 0 as libc::c_int
        {
            identifier = 0 as libc::c_int;
        }
        if c_type != 32 as libc::c_int {
            graphic = 0 as libc::c_int;
        }
        if *p as libc::c_int != ' ' as i32
            && c_type & (4 as libc::c_int | 4096 as libc::c_int | 1 as libc::c_int) != 0
            || *p as libc::c_int == '\\' as i32
        {
            prop.set_needs_scan(1 as libc::c_int as libc::c_uint);
        }
        p = p.offset(1);
        p;
    }
    prop.set_length(len as libc::c_uint);
    if len > 1 as libc::c_int && identifier != 0 {
        Pl_LE_Compl_Add_Word(name, len);
    }
    if pl_char_type[*name as libc::c_uchar as usize] != 1024 as libc::c_int {
        identifier = 0 as libc::c_int;
    }
    if identifier != 0 {
        prop.set_type_0(0 as libc::c_int as libc::c_uint);
        prop.set_needs_quote(0 as libc::c_int as libc::c_uint);
    } else if graphic != 0 {
        prop.set_type_0(1 as libc::c_int as libc::c_uint);
        prop.set_needs_quote(
            (len == 1 as libc::c_int && *name as libc::c_int == '.' as i32
                || len == 1 as libc::c_int && *name as libc::c_int == '%' as i32
                || len >= 2 as libc::c_int
                    && *name.offset(0 as libc::c_int as isize) as libc::c_int
                        == '/' as i32
                    && *name.offset(1 as libc::c_int as isize) as libc::c_int
                        == '*' as i32) as libc::c_int as libc::c_uint,
        );
    } else if len == 1 as libc::c_int
        && pl_char_type[*name as libc::c_uchar as usize] == 2 as libc::c_int
    {
        prop.set_type_0(2 as libc::c_int as libc::c_uint);
        prop.set_needs_quote(
            (*name as libc::c_int == ',' as i32) as libc::c_int as libc::c_uint,
        );
    } else {
        prop.set_type_0(3 as libc::c_int as libc::c_uint);
        prop.set_needs_quote(
            (prop.needs_scan() as libc::c_int != 0
                || !(len == 2 as libc::c_int
                    && (*name.offset(0 as libc::c_int as isize) as libc::c_int
                        == '[' as i32
                        && *name.offset(1 as libc::c_int as isize) as libc::c_int
                            == ']' as i32
                        || *name.offset(0 as libc::c_int as isize) as libc::c_int
                            == '{' as i32
                            && *name.offset(1 as libc::c_int as isize) as libc::c_int
                                == '}' as i32))) as libc::c_int as libc::c_uint,
        );
    }
    prop.set_op_mask(0 as libc::c_int as libc::c_uint);
    (*patom).prop = prop;
    return patom.offset_from(pl_atom_tbl) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn Pl_Create_Atom_Tagged(mut name: *mut libc::c_char) -> WamWord {
    return (((Pl_Create_Atom(name) as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Find_Atom(mut name: *mut libc::c_char) -> libc::c_int {
    let mut len: libc::c_int = strlen(name) as libc::c_int;
    let mut hash: libc::c_uint = Hash_String(name, len);
    let mut patom: *mut AtomInf = 0 as *mut AtomInf;
    patom = Locate_Atom(name, hash);
    return (if patom.is_null() || ((*patom).name).is_null() {
        -(1 as libc::c_int) as libc::c_long
    } else {
        patom.offset_from(pl_atom_tbl) as libc::c_long
    }) as libc::c_int;
}
unsafe extern "C" fn Locate_Atom(
    mut name: *mut libc::c_char,
    mut hash: libc::c_uint,
) -> *mut AtomInf {
    let mut index: libc::c_int = 0;
    let mut patom0: *mut AtomInf = 0 as *mut AtomInf;
    let mut patom: *mut AtomInf = 0 as *mut AtomInf;
    let mut endt: *mut AtomInf = 0 as *mut AtomInf;
    index = (hash as libc::c_ulong).wrapping_rem(pl_max_atom) as libc::c_int;
    patom0 = pl_atom_tbl.offset(index as isize);
    patom = patom0;
    endt = pl_atom_tbl.offset(pl_max_atom as isize);
    while !((*patom).name).is_null()
        && ((*patom).hash != hash || strcmp((*patom).name, name) != 0 as libc::c_int)
    {
        patom = patom.offset(1);
        patom;
        if patom == endt {
            patom = pl_atom_tbl;
        }
        if patom == patom0 {
            return 0 as *mut AtomInf;
        }
    }
    return patom;
}
unsafe extern "C" fn Hash_String(
    mut str: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_uint {
    if len == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    if len == 1 as libc::c_int {
        return *str as libc::c_uchar as libc::c_uint;
    }
    if len == 2 as libc::c_int
        && *str.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32
        && *str.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
    {
        return 256 as libc::c_int as libc::c_uint;
    }
    return Pl_Hash_Buffer(str as *const libc::c_void, len);
}
pub unsafe extern "C" fn Pl_Gen_New_Atom(mut prefix: *mut libc::c_char) -> libc::c_int {
    static mut gen_sym_chars: [libc::c_char; 63] = unsafe {
        *::std::mem::transmute::<
            &[u8; 63],
            &mut [libc::c_char; 63],
        >(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0")
    };
    static mut gen_sym_buff: [libc::c_char; 1024] = [0; 1024];
    static mut gen_sym_rand_next: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    let mut try_no: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut hash: libc::c_uint = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut patom: *mut AtomInf = 0 as *mut AtomInf;
    let mut atom: libc::c_int = 0;
    if pl_nb_atom >= pl_max_atom {
        Error_Table_Full();
    }
    strcpy(gen_sym_buff.as_mut_ptr(), prefix);
    str = gen_sym_buff.as_mut_ptr().offset(strlen(prefix) as isize);
    loop {
        gen_sym_rand_next = gen_sym_rand_next
            .wrapping_mul(1103515245 as libc::c_int as libc::c_uint)
            .wrapping_add(12345 as libc::c_int as libc::c_uint);
        c = (gen_sym_rand_next
            .wrapping_div(65536 as libc::c_int as libc::c_uint)
            .wrapping_rem(32768 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_rem(
                (::std::mem::size_of::<[libc::c_char; 63]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int;
        *str = gen_sym_chars[c as usize];
        *str.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        len = (str.offset_from(gen_sym_buff.as_mut_ptr()) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_int;
        hash = Hash_String(gen_sym_buff.as_mut_ptr(), len);
        patom = Locate_Atom(gen_sym_buff.as_mut_ptr(), hash);
        if ((*patom).name).is_null() {
            break;
        }
        try_no += 1;
        if try_no == 1 as libc::c_int {
            str = str.offset(1);
            str;
            try_no = 0 as libc::c_int;
        }
    }
    atom = Add_Atom(gen_sym_buff.as_mut_ptr(), len, hash, patom, 1 as libc::c_int);
    return atom;
}
pub unsafe extern "C" fn Pl_Find_Next_Atom(mut last_atom: libc::c_int) -> libc::c_int {
    loop {
        last_atom += 1;
        if !((last_atom as PlULong) < pl_max_atom) {
            break;
        }
        if !((*pl_atom_tbl.offset(last_atom as isize)).name).is_null() {
            return last_atom;
        }
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn Error_Table_Full() {
    if pl_fixed_sizes != 0 {
        Pl_Fatal_Error(
            b"Atom table full (max atom: %d - fixed size)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            pl_max_atom,
        );
    } else {
        Pl_Fatal_Error(
            b"Atom table full (max atom: %d, environment variable used: %s)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            pl_max_atom,
            b"MAX_ATOM\0" as *const u8 as *const libc::c_char,
        );
    };
}
