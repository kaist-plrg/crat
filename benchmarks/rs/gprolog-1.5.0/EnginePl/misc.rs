use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn Pl_Hash_Realloc_Table(
        tbl: *mut libc::c_char,
        new_tbl_size: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Hash_Table_Size(tbl: *mut libc::c_char) -> libc::c_int;
    fn Pl_Hash_Nb_Elements(tbl: *mut libc::c_char) -> libc::c_int;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type Bool = libc::c_int;
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
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
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
pub unsafe extern "C" fn Pl_Malloc_Check(
    mut size: libc::c_uint,
    mut src_file: *mut libc::c_char,
    mut src_line: libc::c_int,
) -> *mut libc::c_char {
    let mut m: *mut libc::c_char = malloc(size as libc::c_ulong) as *mut libc::c_char;
    if m.is_null() {
        Pl_Fatal_Error(
            b"Memory allocation fault (%s) in %s:%d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            b"malloc\0" as *const u8 as *const libc::c_char,
            src_file,
            src_line,
        );
    }
    return m;
}
pub unsafe extern "C" fn Pl_Calloc_Check(
    mut nb: libc::c_uint,
    mut size: libc::c_uint,
    mut src_file: *mut libc::c_char,
    mut src_line: libc::c_int,
) -> *mut libc::c_char {
    let mut m: *mut libc::c_char = calloc(nb as libc::c_ulong, size as libc::c_ulong)
        as *mut libc::c_char;
    if m.is_null() {
        Pl_Fatal_Error(
            b"Memory allocation fault (%s) in %s:%d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            b"calloc\0" as *const u8 as *const libc::c_char,
            src_file,
            src_line,
        );
    }
    return m;
}
pub unsafe extern "C" fn Pl_Realloc_Check(
    mut ptr: *mut libc::c_char,
    mut size: libc::c_uint,
    mut src_file: *mut libc::c_char,
    mut src_line: libc::c_int,
) -> *mut libc::c_char {
    let mut m: *mut libc::c_char = realloc(
        ptr as *mut libc::c_void,
        size as libc::c_ulong,
    ) as *mut libc::c_char;
    if m.is_null() {
        Pl_Fatal_Error(
            b"Memory allocation fault (%s) in %s:%d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            b"realloc\0" as *const u8 as *const libc::c_char,
            src_file,
            src_line,
        );
    }
    return m;
}
pub unsafe extern "C" fn Pl_Strdup_Check(
    mut str: *mut libc::c_char,
    mut src_file: *mut libc::c_char,
    mut src_line: libc::c_int,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = strdup(str);
    if s.is_null() {
        Pl_Fatal_Error(
            b"Memory allocation fault (%s) in %s:%d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            b"strdup\0" as *const u8 as *const libc::c_char,
            src_file,
            src_line,
        );
    }
    return s;
}
pub unsafe extern "C" fn Pl_Extend_Table_If_Needed(
    mut hash_tbl: *mut *mut libc::c_char,
) {
    let mut size: libc::c_int = Pl_Hash_Table_Size(*hash_tbl);
    if Pl_Hash_Nb_Elements(*hash_tbl) >= size {
        *hash_tbl = Pl_Hash_Realloc_Table(*hash_tbl, size * 2 as libc::c_int);
    }
}
pub unsafe extern "C" fn Pl_Extend_Array(
    mut ptbl: *mut *mut libc::c_char,
    mut nb_elem: *mut libc::c_int,
    mut elem_size: libc::c_int,
    mut bzero: Bool,
) {
    let mut old_nb_elem: libc::c_int = *nb_elem;
    let mut new_nb_elem: libc::c_int = old_nb_elem * 2 as libc::c_int;
    let mut new_tbl: *mut libc::c_char = 0 as *mut libc::c_char;
    new_tbl = Pl_Realloc_Check(
        *ptbl,
        (new_nb_elem * elem_size) as libc::c_uint,
        b"misc.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        185 as libc::c_int,
    );
    if bzero != 0 {
        memset(
            new_tbl.offset((old_nb_elem * elem_size) as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((new_nb_elem - old_nb_elem) * elem_size) as libc::c_ulong,
        );
    }
    *ptbl = new_tbl;
    *nb_elem = new_nb_elem;
}
pub unsafe extern "C" fn Pl_Exit_With_Value(mut ret_val: libc::c_int) {
    if pl_le_hook_exit_process.is_some() {
        ::std::mem::transmute::<
            _,
            fn(),
        >((Some(pl_le_hook_exit_process.unwrap())).unwrap())();
    }
    exit(ret_val);
}
pub unsafe extern "C" fn Pl_Fatal_Error(mut format: *mut libc::c_char, mut args: ...) {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    let mut buff: [libc::c_char; 1024] = [0; 1024];
    arg_ptr = args.clone();
    vsprintf(buff.as_mut_ptr(), format, arg_ptr.as_va_list());
    if pl_le_hook_message_box.is_some() {
        ::std::mem::transmute::<
            _,
            fn(_, _, _),
        >(
            (Some(pl_le_hook_message_box.unwrap())).unwrap(),
        )(
            b"Fatal Error\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
            0 as libc::c_int,
        );
    } else {
        fprintf(
            stderr,
            b"\nFatal Error: %s\n\0" as *const u8 as *const libc::c_char,
            buff.as_mut_ptr(),
        );
    }
    Pl_Exit_With_Value(1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_LSB(mut x: PlLong) -> libc::c_int {
    let mut bit: libc::c_int = 0 as libc::c_int;
    if x << 32 as libc::c_int == 0 as libc::c_int as libc::c_long {
        bit += 32 as libc::c_int;
        x >>= 32 as libc::c_int;
    }
    if x << 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
        == 0 as libc::c_int as libc::c_long
    {
        bit += 16 as libc::c_int;
        x >>= 16 as libc::c_int;
    }
    if x
        << 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int == 0 as libc::c_int as libc::c_long
    {
        bit += 8 as libc::c_int;
        x >>= 8 as libc::c_int;
    }
    if x
        << 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int + 4 as libc::c_int == 0 as libc::c_int as libc::c_long
    {
        bit += 4 as libc::c_int;
        x >>= 4 as libc::c_int;
    }
    if x
        << 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int
        == 0 as libc::c_int as libc::c_long
    {
        bit += 2 as libc::c_int;
        x >>= 2 as libc::c_int;
    }
    if x
        << 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int
        == 0 as libc::c_int as libc::c_long
    {
        bit += 1 as libc::c_int;
    }
    return bit;
}
pub unsafe extern "C" fn Pl_MSB(mut x: PlLong) -> libc::c_int {
    let mut bit: libc::c_int = 8 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int;
    if x >> 32 as libc::c_int == 0 as libc::c_int as libc::c_long {
        bit -= 32 as libc::c_int;
        x <<= 32 as libc::c_int;
    }
    if x >> 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
        == 0 as libc::c_int as libc::c_long
    {
        bit -= 16 as libc::c_int;
        x <<= 16 as libc::c_int;
    }
    if x
        >> 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int == 0 as libc::c_int as libc::c_long
    {
        bit -= 8 as libc::c_int;
        x <<= 8 as libc::c_int;
    }
    if x
        >> 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int + 4 as libc::c_int == 0 as libc::c_int as libc::c_long
    {
        bit -= 4 as libc::c_int;
        x <<= 4 as libc::c_int;
    }
    if x
        >> 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int
        == 0 as libc::c_int as libc::c_long
    {
        bit -= 2 as libc::c_int;
        x <<= 2 as libc::c_int;
    }
    if x
        >> 8 as libc::c_int * 8 as libc::c_int - 32 as libc::c_int + 16 as libc::c_int
            + 8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int
        == 0 as libc::c_int as libc::c_long
    {
        bit -= 1 as libc::c_int;
    }
    return bit;
}
pub unsafe extern "C" fn Pl_Popcount(mut x: PlLong) -> libc::c_int {
    static mut nb_bits_in_byte: [libc::c_int; 256] = [
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
    ];
    let mut n: libc::c_int = 0 as libc::c_int;
    n += nb_bits_in_byte[(x & 0xff as libc::c_int as libc::c_long) as usize];
    n
        += nb_bits_in_byte[(x >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as usize];
    n
        += nb_bits_in_byte[(x >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as usize];
    n
        += nb_bits_in_byte[(x >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as usize];
    n
        += nb_bits_in_byte[(x >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as usize];
    n
        += nb_bits_in_byte[(x >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as usize];
    n
        += nb_bits_in_byte[(x >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as usize];
    n
        += nb_bits_in_byte[(x >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as usize];
    return n;
}
pub unsafe extern "C" fn Pl_Dummy_Ptr(mut p: *mut libc::c_void) -> *mut libc::c_void {
    return p;
}
